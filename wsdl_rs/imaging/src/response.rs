// Generated by build_scripts/gen_code.pl from ./schemata/imaging.wsdl.xml ---------------------
use yaserde_derive::*;

#[derive(Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct Envelope {
    #[yaserde(prefix = "s", rename = "Body")]
    pub body: Body,
}

#[derive(Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub enum Body {
    Unknown, // Requirement of `Default` impl, required by YaDeserialize

    #[yaserde(prefix = "timg")]
    GetImagingSettingsResponse(super::GetImagingSettingsResponse),
    #[yaserde(prefix = "timg")]
	GetMoveOptionsResponse(super::GetMoveOptionsResponse),
    #[yaserde(prefix = "timg")]
	GetOptionsResponse(super::GetOptionsResponse),
    #[yaserde(prefix = "timg")]
	GetStatusResponse(super::GetStatusResponse),
    #[yaserde(prefix = "timg")]
	MoveResponse(super::MoveResponse),
    #[yaserde(prefix = "timg")]
	SetImagingSettingsResponse(super::SetImagingSettingsResponse),
    #[yaserde(prefix = "timg")]
	StopResponse(super::StopResponse)
}

impl Default for Body {
    fn default() -> Self {
        Body::Unknown
    }
}
