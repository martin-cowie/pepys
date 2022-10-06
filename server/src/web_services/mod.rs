pub(crate) mod devicemgmt;
pub(crate) mod imaging;
pub(crate) mod media;

use soap_fault::SoapFaultCode as Ter;
use hyper::{Method, StatusCode, body::Buf, Uri};

use self::devicemgmt::DeviceManagmentService;
use self::imaging::ImagingService;
use self::media::MediaService;

const DEVICE_MANAGEMENT_PATH: &str = "/onvif/device_service";
const IMAGING_MANAGEMENT_PATH: &str = "/onvif/imaging_service";
const MEDIA_MANAGEMENT_PATH: &str = "/onvif/media_service";

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

        let snapshot_uri = "http://example.com/snapshot".parse().expect("Cannot parse URL");
        let stream_uri = "http://example.com/stream".parse().expect("Cannot parse URL");

        Self {
            device_management_service: DeviceManagmentService::new(service_root,
                DEVICE_MANAGEMENT_PATH,
                IMAGING_MANAGEMENT_PATH,
                MEDIA_MANAGEMENT_PATH
            ),
            imaging_service: ImagingService::new(),
            media_service: MediaService::new(snapshot_uri, stream_uri)
        }
    }

    pub async fn handle_http_request(&self, req: hyper::Request<hyper::Body>, origin: std::net::SocketAddr) -> Result<hyper::Response<hyper::Body>, hyper::Error> {

        let uri_path = req.uri().path().to_string();

        match (req.method(), req.uri().path()) {

            (&Method::POST, IMAGING_MANAGEMENT_PATH) => {
                let whole_body = hyper::body::to_bytes(req.into_body()).await?;
                tracing::info!("Responding to {} bytes of request for URI {} from {}", whole_body.len(), uri_path, origin);

                let result = self.imaging_service.process_request(whole_body.reader());
                    match result {
                    Ok(string) => Ok(hyper::Response::new(hyper::Body::from(string))),
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
                    Ok(string) => Ok(hyper::Response::new(hyper::Body::from(string))),
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
                    Ok(string) => Ok(hyper::Response::new(hyper::Body::from(string))),
                    Err(detail) => {
                        tracing::error!("Cannot handle request: {:?}", &detail);
                        Ok(detail.into())
                    }
                }

            },

            // Return 404 Not Found for all other methods & URIs.
            _ => {
                let response_content = format!("Unexpected method {} and URI: {}\n", req.method(), req.uri());
                // let mut response = hyper::Response::new(hyper::Body::from(response_content));
                // *response.status_mut() = StatusCode::NOT_FOUND;
                // Ok(response)

                let response = hyper::Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(hyper::Body::from(response_content))
                    .unwrap_or_default();

                Ok(response)
            }
        }
    }

}
