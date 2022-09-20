pub(crate) mod devicemgmt;
use hyper::{Method, StatusCode};

pub async fn handle_http_request (req: hyper::Request<hyper::Body>, origin: std::net::SocketAddr) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {

        (&Method::POST, "/picam/device-management") => {
            tracing::info!("Responding to request for URI {} from {}", req.uri().path(), origin);

            // let result = handle_device_management_request(req).await;
            // match result {
            //     Ok(string) => Ok(Response::new(Body::from(string))),
            //     Err(err) => {
            //         tracing::error!("Cannot handle request: {:?}", err);

            //         let mut response = Response::new(Body::default());
            //         *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            //         Ok(response)
            //     }
            // }
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let payload = std::str::from_utf8(&whole_body).unwrap(); //Map this error
            let payload = String::from(payload);

            let result = self::devicemgmt::process_request(payload).await;
            match result {
                Ok(string) => Ok(hyper::Response::new(hyper::Body::from(string))),
                Err(err) => {
                    tracing::error!("Cannot handle request: {:?}", err);

                    let mut response = hyper::Response::new(hyper::Body::default());
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    Ok(response)
                }
            }
        },

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = hyper::Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
