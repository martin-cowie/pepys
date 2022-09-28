// Generated by build_scripts/gen_code.pl from ./schemata/imaging.wsdl.xml ---------------------
use yaserde_derive::*;
use soapenv::Header;

#[derive(Debug, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct Envelope {
    #[yaserde(prefix = "s", rename = "Header")]
    pub header: Option<Header>,

    #[yaserde(prefix = "s", rename = "Body")]
    pub body: Body,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
)]
pub enum Body {
    Unknown, // Requirement of `Default` impl, required by YaDeserialize

        #[yaserde(prefix = "tds")]
    GetImagingSettings(super::GetImagingSettings),

    #[yaserde(prefix = "tds")]
    GetMoveOptions(super::GetMoveOptions),

    #[yaserde(prefix = "tds")]
    GetOptions(super::GetOptions),

    #[yaserde(prefix = "tds")]
    GetStatus(super::GetStatus),

    #[yaserde(prefix = "tds")]
    Move(super::Move),

    #[yaserde(prefix = "tds")]
    SetImagingSettings(super::SetImagingSettings),

    #[yaserde(prefix = "tds")]
    Stop(super::Stop)
}

impl Default for Body {
    fn default() -> Self {
        Body::Unknown
    }
}
