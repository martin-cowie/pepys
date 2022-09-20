use yaserde_derive::*;

#[derive(Debug, YaSerialize)]
#[yaserde(
    prefix = "d",
    namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
)]
pub struct ProbeMatch {
    #[yaserde(prefix = "d", rename = "Types")]
    pub types: String,

    #[yaserde(prefix = "d", rename = "Scopes")]
    pub scopes: String,

    #[yaserde(prefix = "d", rename = "XAddrs")]
    pub x_addrs: String
}

#[derive(Debug, YaSerialize)]
#[yaserde(
    prefix = "d",
    namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
)]
pub struct ProbeMatches {
    #[yaserde(prefix = "d", rename = "ProbeMatch")]
    pub probe_match: Vec<ProbeMatch>,
}

#[derive(Debug, YaSerialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "w: http://schemas.xmlsoap.org/ws/2004/08/addressing"
)]
pub struct Header {
    #[yaserde(prefix = "w", rename = "MessageID")]
    pub message_id: String,

    #[yaserde(prefix = "w", rename = "RelatesTo")]
    pub relates_to: String,

    #[yaserde(prefix = "w", rename = "To")]
    pub to: String,

    #[yaserde(prefix = "w", rename = "Action")]
    pub action: String
}

#[derive(Debug, YaSerialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
)]
pub struct Body {
    #[yaserde(prefix = "d", rename = "ProbeMatches")]
    pub probe_matches: ProbeMatches,
}

#[derive(Debug, YaSerialize)]
#[yaserde(prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "dn: http://www.onvif.org/ver10/network/wsdl"
)]
pub struct Envelope {
    #[yaserde(prefix = "s", rename = "Header")]
    pub header: Header,

    #[yaserde(prefix = "s", rename = "Body")]
    pub body: Body,
}
