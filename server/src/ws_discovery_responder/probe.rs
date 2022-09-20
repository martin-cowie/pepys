use yaserde_derive::*;

#[derive(Default, PartialEq, Eq, Debug, YaDeserialize)]
#[yaserde(
    prefix = "d",
    namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
)]
pub struct Probe {
    #[yaserde(prefix = "d", rename = "Types")]
    pub types: String,
}

#[derive(Default, PartialEq, Eq, Debug, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
)]
pub struct Body {
    #[yaserde(prefix = "d", rename = "Probe")]
    pub probe: Probe,
}


#[derive(Default, PartialEq, Eq, Debug, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "w: http://schemas.xmlsoap.org/ws/2004/08/addressing"
)]
pub struct Header {
    #[yaserde(prefix = "w", rename = "MessageID")]
    pub message_id: String,

    #[yaserde(prefix = "w", rename = "To")]
    pub to: String,

    #[yaserde(prefix = "w", rename = "Action")]
    pub action: String,
}

#[derive(Default, PartialEq, Eq, Debug, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery",
    namespace = "w: http://schemas.xmlsoap.org/ws/2004/08/addressing"
)]
pub struct Envelope {
    #[yaserde(prefix = "s", rename = "Header")]
    pub header: Header,

    #[yaserde(prefix = "s", rename = "Body")]
    pub body: Body,
}
