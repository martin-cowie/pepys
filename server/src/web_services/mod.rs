pub(crate) mod devicemgmt;
use hyper::{Method, StatusCode, body::Buf};

#[derive(Debug)]
pub struct ServiceErrorDetail {
    status: StatusCode,
    detail: Option<String>
}

impl ServiceErrorDetail {
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

pub async fn handle_http_request (req: hyper::Request<hyper::Body>, origin: std::net::SocketAddr) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {

        (&Method::POST, "/picam/device-management") => {
            let uri_path = req.uri().path().to_string();
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            tracing::info!("Responding to {} bytes of request for URI {} from {}", whole_body.len(), uri_path, origin);

            let result = self::devicemgmt::process_request(whole_body.reader()).await;
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
