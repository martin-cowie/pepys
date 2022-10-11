mod probe;
mod probe_response;

use static_init::dynamic;
use std::{error::Error, net::{SocketAddr, Ipv4Addr, IpAddr}};
use tokio::net::UdpSocket;
use tracing::{info, debug, error};
use uuid::Uuid;

use crate::ws_discovery_responder::probe::Envelope;

/**
 * Respond to WS-Discovery probes
 * https://specs.xmlsoap.org/ws/2005/04/discovery/ws-discovery.pdf
 */

const LOCAL_IPV4_ADDR: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

// Web Services Discovery multicast address
const MULTI_IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const MULTI_PORT: u16 = 3702; // Web Services Discovery (WSD) broadcast port

const WS_DISCOVERY_PROBE: &str = "http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe";
const WS_DISCOVERY_MATCHES: &str = "http://schemas.xmlsoap.org/ws/2005/04/discovery/ProbeMatches";
const ADDRESSING_ROLE_ANON: &str = "http://schemas.xmlsoap.org/ws/2004/08/addressing/role/anonymous";

#[dynamic]
static SCOPES: Vec<&'static str> = vec![
    "onvif://www.onvif.org/type/video_encoder",
    "onvif://www.onvif.org/hardware/Pepys",
    "onvif://www.onvif.org/name/Pepys",
    "onvif://www.onvif.org/location/"
];

pub async fn bind_ws_discovery_responder(xaddr: &str) -> Result<(), Box<dyn Error>> {
    let listening_socket = {
        let local_socket_addr = SocketAddr::new(IpAddr::V4(LOCAL_IPV4_ADDR), MULTI_PORT);
        let socket = UdpSocket::bind(local_socket_addr).await?;
        socket.join_multicast_v4(MULTI_IPV4_ADDR, LOCAL_IPV4_ADDR)?;

        socket
    };

    info!("Process {} listening for WS-Discovery traffic at {:?}", std::process::id(), listening_socket.local_addr()?);

    let mut buf = vec![0; 16 * 1024];
    loop {
        let (len, sender_addr) = listening_socket.recv_from(&mut buf).await?;
        let received_message = String::from_utf8_lossy(&buf[..len]).to_string();

        //TODO: handle parse failures - "IP Cams" sends XML with unbound prefixes.

        match yaserde::de::from_str::<Envelope>(&received_message) {
            Err(detail) => {
                error!("Cannot parse XML probe from {}: {}", sender_addr, detail);
            }

            Ok(probe) => {
                if probe.header.action == WS_DISCOVERY_PROBE {
                    debug!("Received probe from {}: {:#?}", sender_addr, probe);
                    let response = build_response(&probe, xaddr);
                    let response_str = yaserde::ser::to_string(&build_response(&probe, xaddr))?;
                    let response_bytes = response_str.as_bytes();
                    listening_socket.send_to(response_bytes, sender_addr).await?;
                    debug!("Sent {} bytes to {}: {:#?}", response_bytes.len(), sender_addr, response);
                }
            }
        }

     }
}

/**
 * Build a response or 'match' to the given probe object.
 */
fn build_response(probe: &probe::Envelope, xaddr: &str) -> probe_response::Envelope {

    let probe_uuid = &probe.header.message_id;
    let uuid = Uuid::new_v4();

    probe_response::Envelope{
        header: probe_response::Header{
            relates_to: probe_uuid.clone(),
            message_id: uuid.to_string(),
            to: ADDRESSING_ROLE_ANON.to_string(),
            action: WS_DISCOVERY_MATCHES.to_string(),
        },
        body: probe_response::Body{
            probe_matches: probe_response::ProbeMatches { probe_match: vec![
                probe_response::ProbeMatch {
                    scopes: SCOPES.join(" "),
                    types: "dn:NetworkVideoTransmitter".to_string(),
                    x_addrs: xaddr.to_string()
                }
            ] }
        }
    }
}