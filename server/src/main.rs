mod ws_discovery_responder;
mod web_services;
mod rpi;

use get_if_addrs::{get_if_addrs, IfAddr, Ifv4Addr, Ifv6Addr};
use std::error::Error;
use tracing::info;
use ws_discovery_responder::bind_ws_discovery_responder;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use std::convert::Infallible;

//TODO: should this be constant, or ephemeral?
const WEB_PORT: u16 = 8088;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    tracing_subscriber::fmt::init();

    let xaddrs = get_urls(WEB_PORT, "picam/device-management")?.join(" ");

    // Create and start the 'Hyper' web server
    let web_server = {
        let make_service = make_service_fn(move |conn: &AddrStream|{
            let addr = conn.remote_addr();
            tracing::debug!("Making service for {}", &addr);
            async move {
                tracing::debug!("Service closure running for {}", &addr);
                Ok::<_, Infallible>(service_fn(move |req| web_services::handle_http_request(req, addr)))
            }
        });
        let addr = ([0, 0, 0, 0], WEB_PORT).into();
        Server::bind(&addr).serve(make_service)
    };
    info!("Started HTTP server bound at {:?}", xaddrs);

    // Start the UDP listener
    let ssdp_responder  = tokio::spawn(async move {
        bind_ws_discovery_responder(&xaddrs).await.unwrap();
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
