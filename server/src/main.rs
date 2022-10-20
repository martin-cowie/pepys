mod ws_discovery_responder;
mod web_services;
mod rpi;
mod camera;

use get_if_addrs::{get_if_addrs, IfAddr, Ifv4Addr, Ifv6Addr};
use std::error::Error;
use std::sync::Arc;
use tracing::{info, error};
use ws_discovery_responder::bind_ws_discovery_responder;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper::Uri;

use std::convert::Infallible;

//TODO: Take this from configuration
const WEB_PORT: u16 = 8088;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    tracing_subscriber::fmt::init();

    let xaddrs = get_urls(WEB_PORT, "onvif/device_service")?; //TODO: duplicate string

    // The inner layer of request handling machinery
    let web_services = {
        let service_root: Uri = xaddrs[0].parse().expect("Cannot parse root URI");
        Arc::new(web_services::WebServices::new(&service_root))
    };

    // Create and start the 'Hyper' web server
    let web_server = {

        // Build the outer layer of request handling machinery
        let service_maker = make_service_fn(move |conn: &AddrStream|{
            let addr = conn.remote_addr();
            let web_services = Arc::clone(&web_services);
            tracing::debug!("Making service for {}", &addr);

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    tracing::debug!("Service closure running for {}", &addr);
                    let web_services = Arc::clone(&web_services);

                    async move {
                        let uri = req.uri().clone();
                        let response = web_services.handle_http_request(req).await;

                        // Log request URIs next to response status code
                        if let Ok(ref response) = response {
                            let status = response.status();
                            let log_entry = format!("Responding to request for URI {} from {} => {}", uri, addr, status);

                            if status.is_success() || status.is_informational() {
                                info!("{}", log_entry);
                            } else {
                                error!("{}", log_entry);
                            }
                        }
                        response
                    }
                }))
            }
        });
        let addr = ([0, 0, 0, 0], WEB_PORT).into();
        Server::bind(&addr).serve(service_maker)
    };
    info!("Started HTTP server bound at {:?}", xaddrs);

    // Start the UDP listener
    let ssdp_responder  = tokio::spawn(async move {
        bind_ws_discovery_responder(&xaddrs.join(" ")).await.expect("Cannot start WS-Discovery listener");
    });

    let (_, _) = (web_server.await?, ssdp_responder.await?);

    Ok(())
}

/// Construct a set of URLS from the argument and
/// the set of IP addresses associated with non-loopback NICs
fn get_urls(port_number: u16, suffix: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let urls: Vec<String> = get_if_addrs()?
        .into_iter()
        .filter(|nic| !nic.is_loopback())
        .map(|nic|
            match nic.addr {
                IfAddr::V4(Ifv4Addr{ip, ..}) => format!("http://{}:{}/{}", ip, port_number, suffix),
                IfAddr::V6(Ifv6Addr{ip, ..}) => format!("http://{}:{}/{}", ip, port_number, suffix),
            }
        )
        .collect();

    Ok(urls)
}
