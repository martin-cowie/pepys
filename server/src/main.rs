mod ws_discovery_responder;
mod web_services;
mod rpi;
mod camera;
mod config;
mod nics;

use camera::{CameraAdapter, TestCameraAdapter, PiCameraAdapter};
use web_services::Authenticator;
use tracing::{info, trace, error};
use ws_discovery_responder::bind_ws_discovery_responder;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper::Uri;

use std::convert::Infallible;
use std::env;
use std::error::Error;
use std::time::Duration;

use tokio::time::sleep;

const LOGO: &str = r"
 ____
/\  _`\
\ \ \L\ \ __   _____   __  __    ____
 \ \ ,__/'__`\/\ '__`\/\ \/\ \  /',__\
  \ \ \/\  __/\ \ \L\ \ \ \_\ \/\__, `\
   \ \_\ \____\\ \ ,__/\/`____ \/\____/
    \/_/\/____/ \ \ \/  `/___/> \/___/
                 \ \_\     /\___/
                  \/_/     \/__/
";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    tracing_subscriber::fmt::init();

    let config: &'static config::Config = Box::leak(Box::new({
        let args: Vec<String> = env::args().collect();
        let config_name = args.get(1);
        if let Some(config_name) = config_name {
            let result = config::Config::load(config_name)
                .map_err(|err| format!("Cannot load {}: {}", &config_name, err))?;
            info!("Applied configuration from {}", config_name);
            result
        } else {
            let result = config::Config::default();
            info!("Applied default configuration");
            result
        }
    }));

    let authenticator: &'static Authenticator = Box::leak(Box::new(Authenticator::new(&config.username, &config.password)));

    let device_management_addrs = get_urls(config.port, web_services::DEVICE_MANAGEMENT_PATH)?;

    let web_services: &'static web_services::WebServices = {
        let service_root: Uri = device_management_addrs[0].parse().expect("Cannot parse root URI");
        let camera_adapter: &'static dyn CameraAdapter = match config.adapter_type {
            config::AdapterType::Test => Box::leak(Box::new(TestCameraAdapter::new())),
            config::AdapterType::Pi => Box::leak(Box::new(PiCameraAdapter::new(&config.pi_camera, &config.username, &config.password))),

        };

        Box::leak(Box::new(web_services::WebServices::new(&service_root, authenticator, camera_adapter)))
    };


    // Create and start the 'Hyper' web server
    let web_server = {

        // Build the outer layer of request handling machinery
        let service_maker = make_service_fn(move |conn: &AddrStream|{
            let addr = conn.remote_addr();
            tracing::debug!("Making service for {}", &addr);

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    tracing::debug!("Service closure running for {}", &addr);

                    async move {
                        let uri = req.uri().clone();
                        let response = web_services.handle_http_request(req).await;

                        // Log request URIs next to response status code
                        if let Ok(ref response) = response {
                            let status = response.status();
                            let log_entry = format!("Responding to request for URI {} from {} => {}", uri, addr, status);

                            if status.is_success() || status.is_informational() {
                                trace!("{}", log_entry);
                            } else {
                                error!("{}", log_entry);
                            }
                        }
                        response
                    }
                }))
            }
        });
        let addr = ([0, 0, 0, 0], config.port).into();
        Server::bind(&addr).serve(service_maker)
    };

    // Start the UDP listener
    let ssdp_responder  = tokio::spawn(async move {
        bind_ws_discovery_responder(&device_management_addrs.join(" "))
            .await
            .expect("Cannot start WS-Discovery listener");
    });

    // Log the greeting after 1s
    tokio::spawn(async move {
        let duration = Duration::from_secs(1);
        sleep(duration).await;

        let web_server_addresses = get_urls(config.port, "")
            .expect("Cannot get URLs")
            .join(", ");
        info!("Peyps begun. HTTP server bound at {}{}", web_server_addresses, LOGO);
    });

    let (_, _) = (web_server.await?, ssdp_responder.await?);

    Ok(())
}

/// Construct a set of URLS from the argument and
/// the set of IP addresses associated with non-loopback NICs
/// Argument `suffix` is affixed to the result with a delimiting `/` character if it does not start with one.
fn get_urls(port_number: u16, suffix: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let urls: Vec<String> = nics::get_v4_addresses()
        .map(|ip| {
            let separator = if !suffix.starts_with('/') {"/"} else {""};
            format!("http://{ip}:{port_number}{separator}{suffix}")
        })
        .collect();

    Ok(urls)
}

#[cfg(test)]
mod test {

    use super::*;
    use hyper::Uri;

    #[test]
    fn test_get_urls() {
        //NB: this test presumes there is at least 1 IPv4 non-loopback NIC :-(

        let test_urls = |item: String| {
            let uri: Uri = item.parse().unwrap();

            assert_eq!(uri.port_u16(), Some(1234));
            assert_eq!(uri.path(), "/foo/bar");
        };

        get_urls(1234, "/foo/bar")
            .unwrap()
            .into_iter()
            .for_each(test_urls);

        get_urls(1234, "foo/bar")
            .unwrap()
            .into_iter()
            .for_each(test_urls);

    }

}