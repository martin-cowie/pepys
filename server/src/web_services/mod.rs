pub(crate) mod devicemgmt;
pub(crate) mod imaging;
pub(crate) mod media;

use soap_fault::SoapFaultCode as Ter;
use hyper::{Method, StatusCode, body::Buf, Uri};
use hyper::header::CONTENT_TYPE;

use self::devicemgmt::DeviceManagmentService;
use self::imaging::ImagingService;
use self::media::MediaService;
use super::camera::{TestCameraAdapter, CameraAdapter};

const DEVICE_MANAGEMENT_PATH: &str = "/onvif/device_service";
const IMAGING_MANAGEMENT_PATH: &str = "/onvif/imaging_service";
const MEDIA_MANAGEMENT_PATH: &str = "/onvif/media_service";
const CAMERA_PREVIEW_PATH: &str = "/camera/preview";

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
    media_service: MediaService,
    camera_adapter: &'static dyn CameraAdapter //TODO: replace with reference to a trait object
}

impl WebServices {

    /// Services are made available at, or relative to `service_root`.
    ///
    pub fn new(service_root: &Uri) -> Self {

        let snapshot_uri = build_address(service_root, CAMERA_PREVIEW_PATH);
        let camera_adapter: &'static dyn CameraAdapter = Box::leak(Box::new(TestCameraAdapter::new()));

        Self {
            device_management_service: DeviceManagmentService::new(
                build_address(service_root, DEVICE_MANAGEMENT_PATH),
                build_address(service_root, IMAGING_MANAGEMENT_PATH),
                build_address(service_root, MEDIA_MANAGEMENT_PATH)
            ),
            imaging_service: ImagingService::new(),
            media_service: MediaService::new(snapshot_uri, camera_adapter.get_camera_streams()[0].clone()),
            camera_adapter
        }
    }

    pub async fn handle_http_request(&self, req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
        // TODO: refactor to reduce
        match (req.method(), req.uri().path()) {

            (&Method::GET, CAMERA_PREVIEW_PATH) => {
                let (file_bytes, mime_type) = self.camera_adapter.get_preview();

                let response = hyper::Response::builder()
                    .header(CONTENT_TYPE, mime_type)
                    .body(hyper::Body::from(file_bytes))
                    .unwrap_or_default();
                Ok(response)
            }

            (&Method::POST, IMAGING_MANAGEMENT_PATH) => {
                let whole_body = hyper::body::to_bytes(req.into_body()).await?;

                let result = self.imaging_service.process_request(whole_body.reader());
                match result {
                    Ok(string) => Ok(build_response(string)),
                    Err(detail) => Ok(detail.into())
                }
            }

            (&Method::POST, DEVICE_MANAGEMENT_PATH) => {
                let whole_body = hyper::body::to_bytes(req.into_body()).await?;

                let result = self.device_management_service.process_request(whole_body.reader());
                match result {
                    Ok(string) => Ok(build_response(string)),
                    Err(detail) => Ok(detail.into())
                }
            },

            (&Method::POST, MEDIA_MANAGEMENT_PATH) => {
                let whole_body = hyper::body::to_bytes(req.into_body()).await?;

                let result = self.media_service.process_request(whole_body.reader());
                match result {
                    Ok(string) => Ok(build_response(string)),
                    Err(detail) => Ok(detail.into())
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