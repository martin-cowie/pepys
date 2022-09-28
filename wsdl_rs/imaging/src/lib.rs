#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

pub mod request;
pub mod response;

// Generated from imaging.xsd hereon ---------------------------------

//use ../../../ver10/schema/onvif.xsd  http://www.onvif.org/ver10/schema;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the imaging service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "timg", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct Capabilities {
    // Indicates whether or not Image Stabilization feature is supported.
    // The use of this capability is deprecated, a client should use GetOption
    // to find out if image stabilization is supported.
    #[yaserde(attribute, rename = "ImageStabilization")]
    pub image_stabilization: Option<bool>,

    // Indicates whether or not Imaging Presets feature is supported.
    #[yaserde(attribute, rename = "Presets")]
    pub presets: Option<bool>,

    // Indicates whether or not imaging preset settings can be updated.
    #[yaserde(attribute, rename = "AdaptablePreset")]
    pub adaptable_preset: Option<bool>,
}

impl Validate for Capabilities {}


// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetImagingSettings {
    // Reference token to the VideoSource for which the ImagingSettings.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetImagingSettings {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetImagingSettingsResponse {
    // ImagingSettings for the VideoSource that was requested.
    #[yaserde(prefix = "timg", rename = "ImagingSettings")]
    pub imaging_settings: tt::ImagingSettings20,
}

impl Validate for GetImagingSettingsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct SetImagingSettings {
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    #[yaserde(prefix = "timg", rename = "ImagingSettings")]
    pub imaging_settings: tt::ImagingSettings20,

    #[yaserde(prefix = "timg", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetImagingSettings {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct SetImagingSettingsResponse {}

impl Validate for SetImagingSettingsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetOptions {
    // Reference token to the VideoSource for which the imaging parameter
    // options are requested.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetOptionsResponse {
    // Valid ranges for the imaging parameters that are categorized as device
    // specific.
    #[yaserde(prefix = "timg", rename = "ImagingOptions")]
    pub imaging_options: tt::ImagingOptions20,
}

impl Validate for GetOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct Move {
    // Reference to the VideoSource for the requested move (focus) operation.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Content of the requested move (focus) operation.
    #[yaserde(prefix = "timg", rename = "Focus")]
    pub focus: Vec<tt::FocusMove>,
}

impl Validate for Move {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct MoveResponse {}

impl Validate for MoveResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetMoveOptions {
    // Reference token to the VideoSource for the requested move options.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetMoveOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetMoveOptionsResponse {
    // Valid ranges for the focus lens move options.
    #[yaserde(prefix = "timg", rename = "MoveOptions")]
    pub move_options: tt::MoveOptions20,
}

impl Validate for GetMoveOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct Stop {
    // Reference token to the VideoSource where the focus movement should be
    // stopped.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for Stop {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct StopResponse {}

impl Validate for StopResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetStatus {
    // Reference token to the VideoSource where the imaging status should be
    // requested.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetStatus {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "timg", namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl")]
pub struct GetStatusResponse {
    // Requested imaging status.
    #[yaserde(prefix = "timg", rename = "Status")]
    pub status: tt::ImagingStatus20,
}

impl Validate for GetStatusResponse {}

//NB: removed client-focused functions