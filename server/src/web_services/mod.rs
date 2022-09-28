pub(crate) mod devicemgmt;
pub(crate) mod imaging;

use std::error::Error;
use hyper::{Method, StatusCode, body::Buf, Uri};

use self::devicemgmt::DeviceManagmentService;
use self::imaging::ImagingService;

#[derive(Debug)]
pub struct ServiceErrorDetail {
    status: StatusCode,
    detail: Option<String>
}

const DEVICE_MANAGEMENT_PATH: &str = "/pepys/device_service";
const EVENT_MANAGEMENT_PATH: &str = "/pepys/events_service";
const IMAGING_MANAGEMENT_PATH: &str = "/pepys/imaging_service";
const MEDIA_MANAGEMENT_PATH: &str = "/pepys/media_service";

//===| Common means of communicating errors from service implementations |============

impl ServiceErrorDetail  {
    pub fn new(status: StatusCode, detail: Option<String>) -> Self {
        Self {
            status,
            detail
        }
    }

    pub fn into_response(self) -> hyper::Response<hyper::Body> {
        let mut response = hyper::Response::new(hyper::Body::from(self.detail.clone().unwrap_or_default()));
        *response.status_mut() = self.status;
        response
    }
}

impl std::fmt::Display for ServiceErrorDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(detail) = self.detail.as_ref() {
            write!(f, "{}", detail)
        } else {
            write!(f, "Empty")
        }
    }
}

impl Error for ServiceErrorDetail{}

//===| Web Services Controller |====================================

pub struct WebServices {
    device_management_service: DeviceManagmentService,
    imaging_service: ImagingService
}

impl WebServices {

    pub fn new(service_root: &Uri) -> Self {

        Self {
            device_management_service: DeviceManagmentService::new(service_root,
                DEVICE_MANAGEMENT_PATH,
                EVENT_MANAGEMENT_PATH,
                IMAGING_MANAGEMENT_PATH,
                MEDIA_MANAGEMENT_PATH
            ),
            imaging_service: ImagingService::new()
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
                        Ok(detail.into_response())
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
                        Ok(detail.into_response())
                    }
                }
            },



            // Return 404 Not Found for all other methods & URIs.
            _ => {
                let response_content = format!("Unexpected method {} and URI: {}\n", req.method(), req.uri());

                let mut response = hyper::Response::new(hyper::Body::from(response_content));
                *response.status_mut() = StatusCode::NOT_FOUND;
                Ok(response)
            }
        }
    }

}
