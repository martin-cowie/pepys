pub(crate) mod devicemgmt;
pub(crate) mod imaging;
pub(crate) mod media;

use soap_fault::SoapFaultCode as Ter;
use hyper::{Method, StatusCode, body::Buf, Uri};

use self::devicemgmt::DeviceManagmentService;
use self::imaging::ImagingService;
use self::media::MediaService;

#[cfg(debug_assertions)]
use hyper::header::CONTENT_TYPE;
#[cfg(debug_assertions)]
use std::include_bytes;

const DEVICE_MANAGEMENT_PATH: &str = "/onvif/device_service";
const IMAGING_MANAGEMENT_PATH: &str = "/onvif/imaging_service";
const MEDIA_MANAGEMENT_PATH: &str = "/onvif/media_service";
const TEST_PREVIEW_PATH: &str = "/test/preview.jpeg";

//============================================================

/// Get example instance of this object.
/// Intentionally different from Default.
pub(crate) trait ExampleData<T> {
    fn example() -> T;
}

//===| Authentication |=============================================

const PASSWORD: &str = "password123"; //FIXME: obvs

pub(crate) fn authenticate(header: &Option<soapenv::Header>) -> Result<(), Ter> {

    let header = match header {
        Some(header) => header,
        None => return Err(Ter::NotAuthorized)
    };

    let security = match header.security {
        Some(ref security) => security,
        None => return Err(Ter::NotAuthorized)
    };

    if !security.is_password_authentic(PASSWORD) {
        Err(Ter::NotAuthorized)
    } else {
        Ok(())
    }

    // FIXME: more compact edition, but was didn't like header being a reference
    //
    // if !header.ok_or_else(auth_lacking)?
    //     .security.ok_or_else(auth_lacking)?
    //     .is_password_authentic(PASSWORD) {
    //         return Err(ServiceErrorDetail::new(
    //             StatusCode::FORBIDDEN,
    //             Some("Incorrect password".to_string())
    //         ));
    // }

}

//===| Web Services Controller |====================================

pub struct WebServices {
    device_management_service: DeviceManagmentService,
    imaging_service: ImagingService,
    media_service: MediaService
}

impl WebServices {

    pub fn new(service_root: &Uri) -> Self {

        let snapshot_uri = build_address(service_root, TEST_PREVIEW_PATH);
        let stream_uri = "http://example.com/stream".parse().expect("Cannot parse URL");

        tracing::info!("Preview URL: {}", &snapshot_uri);

        Self {
            device_management_service: DeviceManagmentService::new(
                build_address(service_root, DEVICE_MANAGEMENT_PATH),
                build_address(service_root, IMAGING_MANAGEMENT_PATH),
                build_address(service_root, MEDIA_MANAGEMENT_PATH)
            ),
            imaging_service: ImagingService::new(),
            media_service: MediaService::new(snapshot_uri, stream_uri)
        }
    }

    pub async fn handle_http_request(&self, req: hyper::Request<hyper::Body>, origin: std::net::SocketAddr) -> Result<hyper::Response<hyper::Body>, hyper::Error> {

        let uri_path = req.uri().path().to_string();

        // TODO: refactor to reduce
        match (req.method(), req.uri().path()) {
            #[cfg(debug_assertions)]
            (&Method::GET, TEST_PREVIEW_PATH) => {
                let file_bytes = include_bytes!("preview.jpeg").to_vec();
                let response = hyper::Response::builder()
                    .header(CONTENT_TYPE, "image/jpeg")
                    .body(hyper::Body::from(file_bytes))
                    .unwrap_or_default();
                Ok(response)
            }

            (&Method::POST, IMAGING_MANAGEMENT_PATH) => {
                let whole_body = hyper::body::to_bytes(req.into_body()).await?;
                tracing::info!("Responding to {} bytes of request for URI {} from {}", whole_body.len(), uri_path, origin);

                let result = self.imaging_service.process_request(whole_body.reader());
                    match result {
                    Ok(string) => Ok(build_response(string)),
                    Err(detail) => {
                        tracing::error!("Cannot handle request: {:?}", &detail);
                        Ok(detail.into())
                    }
                }
            }

            (&Method::POST, DEVICE_MANAGEMENT_PATH) => {
                let whole_body = hyper::body::to_bytes(req.into_body()).await?;
                tracing::info!("Responding to {} bytes of request for URI {} from {}", whole_body.len(), uri_path, origin);

                let result = self.device_management_service.process_request(whole_body.reader());
                match result {
                    Ok(string) => Ok(build_response(string)),
                    Err(detail) => {
                        tracing::error!("Cannot handle request: {:?}", &detail);
                        Ok(detail.into())
                    }
                }
            },

            (&Method::POST, MEDIA_MANAGEMENT_PATH) => {
                let whole_body = hyper::body::to_bytes(req.into_body()).await?;
                tracing::info!("Responding to {} bytes of request for URI {} from {}", whole_body.len(), uri_path, origin);

                let result = self.media_service.process_request(whole_body.reader());
                    match result {
                    Ok(string) => Ok(build_response(string)),
                    Err(detail) => {
                        tracing::error!("Cannot handle request: {:?}", &detail);
                        Ok(detail.into())
                    }
                }

            },

            // Return 404 Not Found for all other methods & URIs.
            _ => {
                let response_content = format!("Unexpected method {} and URI: {}\n", req.method(), req.uri());

                let response = hyper::Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(hyper::Body::from(response_content))
                    .unwrap_or_default();

                Ok(response)
            }
        }
    }

}

//===| Support functions |=======

/// Build a Uri comprising the root of `root` and a path of `path`.
fn build_address(root: &Uri, path: &str) -> Uri {
    let parts = root.clone().into_parts();

    // TODO: better means of constructing one URI from another
    Uri::builder()
        .scheme(parts.scheme.expect("Cannot handle missing scheme"))
        .authority(parts.authority.expect("Cannot handle missing authority"))
        .path_and_query(path)
        .build()
        .expect("Cannot deconstruct service root")
}


/// Compose and set headers for an XML response
fn build_response(xml_str: String) -> hyper::Response<hyper::Body> {
    hyper::Response::builder()
        .header(CONTENT_TYPE, "application/soap+xml")
        .body(hyper::Body::from(xml_str))
        .unwrap_or_default()
}