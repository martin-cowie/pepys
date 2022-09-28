#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

pub mod request;
pub mod response;

// Generated from media.wsdl.xml hereon ---------------------------------

//use ../../../ver10/schema/onvif.xsd  http://www.onvif.org/ver10/schema;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the media service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "trt", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct Capabilities {
    // Media profile capabilities.
    #[yaserde(prefix = "trt", rename = "ProfileCapabilities")]
    pub profile_capabilities: Vec<ProfileCapabilities>,

    // Streaming capabilities.
    #[yaserde(prefix = "trt", rename = "StreamingCapabilities")]
    pub streaming_capabilities: Vec<StreamingCapabilities>,

    // Indicates if GetSnapshotUri is supported.
    #[yaserde(attribute, rename = "SnapshotUri")]
    pub snapshot_uri: Option<bool>,

    // Indicates whether or not Rotation feature is supported.
    #[yaserde(attribute, rename = "Rotation")]
    pub rotation: Option<bool>,

    // Indicates the support for changing video source mode.
    #[yaserde(attribute, rename = "VideoSourceMode")]
    pub video_source_mode: Option<bool>,

    // Indicates if OSD is supported.
    #[yaserde(attribute, rename = "OSD")]
    pub osd: Option<bool>,

    // Indicates the support for temporary osd text configuration.
    #[yaserde(attribute, rename = "TemporaryOSDText")]
    pub temporary_osd_text: Option<bool>,

    // Indicates the support for the Efficient XML Interchange (EXI) binary XML
    // format.
    #[yaserde(attribute, rename = "EXICompression")]
    pub exi_compression: Option<bool>,
}

impl Validate for Capabilities {}


// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct ProfileCapabilities {
    // Maximum number of profiles supported.
    #[yaserde(attribute, rename = "MaximumNumberOfProfiles")]
    pub maximum_number_of_profiles: Option<i32>,
}

impl Validate for ProfileCapabilities {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct StreamingCapabilities {
    // Indicates support for RTP multicast.
    #[yaserde(attribute, rename = "RTPMulticast")]
    pub rtp_multicast: Option<bool>,

    // Indicates support for RTP over TCP.
    #[yaserde(attribute, rename = "RTP_TCP")]
    pub rtp_tcp: Option<bool>,

    // Indicates support for RTP/RTSP/TCP.
    #[yaserde(attribute, rename = "RTP_RTSP_TCP")]
    pub rtp_rtsp_tcp: Option<bool>,

    // Indicates support for non aggregate RTSP control.
    #[yaserde(attribute, rename = "NonAggregateControl")]
    pub non_aggregate_control: Option<bool>,

    // Indicates the device does not support live media streaming via RTSP.
    #[yaserde(attribute, rename = "NoRTSPStreaming")]
    pub no_rtsp_streaming: Option<bool>,
}

impl Validate for StreamingCapabilities {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSources {}

impl Validate for GetVideoSources {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourcesResponse {
    // List of existing Video Sources
    #[yaserde(prefix = "trt", rename = "VideoSources")]
    pub video_sources: Vec<tt::VideoSource>,
}

impl Validate for GetVideoSourcesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioSources {}

impl Validate for GetAudioSources {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioSourcesResponse {
    // List of existing Audio Sources
    #[yaserde(prefix = "trt", rename = "AudioSources")]
    pub audio_sources: Vec<tt::AudioSource>,
}

impl Validate for GetAudioSourcesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioOutputs {}

impl Validate for GetAudioOutputs {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioOutputsResponse {
    // List of existing Audio Outputs
    #[yaserde(prefix = "trt", rename = "AudioOutputs")]
    pub audio_outputs: Vec<tt::AudioOutput>,
}

impl Validate for GetAudioOutputsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct CreateProfile {
    // friendly name of the profile to be created
    #[yaserde(prefix = "trt", rename = "Name")]
    pub name: tt::Name,

    // Optional token, specifying the unique identifier of the new profile.
    #[yaserde(prefix = "trt", rename = "Token")]
    pub token: Option<tt::ReferenceToken>,
}

impl Validate for CreateProfile {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct CreateProfileResponse {
    // returns the new created profile
    #[yaserde(prefix = "trt", rename = "Profile")]
    pub profile: tt::Profile,
}

impl Validate for CreateProfileResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetProfile {
    // this command requests a specific profile
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetProfile {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetProfileResponse {
    // returns the requested media profile
    #[yaserde(prefix = "trt", rename = "Profile")]
    pub profile: tt::Profile,
}

impl Validate for GetProfileResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetProfiles {}

impl Validate for GetProfiles {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetProfilesResponse {
    // lists all profiles that exist in the media service
    #[yaserde(prefix = "trt", rename = "Profiles")]
    pub profiles: Vec<tt::Profile>,
}

impl Validate for GetProfilesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddVideoEncoderConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the VideoEncoderConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddVideoEncoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddVideoEncoderConfigurationResponse {}

impl Validate for AddVideoEncoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveVideoEncoderConfiguration {
    // Contains a reference to the media profile from which the
    // VideoEncoderConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveVideoEncoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveVideoEncoderConfigurationResponse {}

impl Validate for RemoveVideoEncoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddVideoSourceConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the VideoSourceConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddVideoSourceConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddVideoSourceConfigurationResponse {}

impl Validate for AddVideoSourceConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveVideoSourceConfiguration {
    // Contains a reference to the media profile from which the
    // VideoSourceConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveVideoSourceConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveVideoSourceConfigurationResponse {}

impl Validate for RemoveVideoSourceConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddAudioEncoderConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the AudioEncoderConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddAudioEncoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddAudioEncoderConfigurationResponse {}

impl Validate for AddAudioEncoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveAudioEncoderConfiguration {
    // Contains a reference to the media profile from which the
    // AudioEncoderConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveAudioEncoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveAudioEncoderConfigurationResponse {}

impl Validate for RemoveAudioEncoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddAudioSourceConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the AudioSourceConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddAudioSourceConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddAudioSourceConfigurationResponse {}

impl Validate for AddAudioSourceConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveAudioSourceConfiguration {
    // Contains a reference to the media profile from which the
    // AudioSourceConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveAudioSourceConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveAudioSourceConfigurationResponse {}

impl Validate for RemoveAudioSourceConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddPTZConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the PTZConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddPTZConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddPTZConfigurationResponse {}

impl Validate for AddPTZConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemovePTZConfiguration {
    // Contains a reference to the media profile from which the
    // PTZConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemovePTZConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemovePTZConfigurationResponse {}

impl Validate for RemovePTZConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddVideoAnalyticsConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the VideoAnalyticsConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddVideoAnalyticsConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddVideoAnalyticsConfigurationResponse {}

impl Validate for AddVideoAnalyticsConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveVideoAnalyticsConfiguration {
    // Contains a reference to the media profile from which the
    // VideoAnalyticsConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveVideoAnalyticsConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveVideoAnalyticsConfigurationResponse {}

impl Validate for RemoveVideoAnalyticsConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddMetadataConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the MetadataConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddMetadataConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddMetadataConfigurationResponse {}

impl Validate for AddMetadataConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveMetadataConfiguration {
    // Contains a reference to the media profile from which the
    // MetadataConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveMetadataConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveMetadataConfigurationResponse {}

impl Validate for RemoveMetadataConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddAudioOutputConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the AudioOutputConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddAudioOutputConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddAudioOutputConfigurationResponse {}

impl Validate for AddAudioOutputConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveAudioOutputConfiguration {
    // Contains a reference to the media profile from which the
    // AudioOutputConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveAudioOutputConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveAudioOutputConfigurationResponse {}

impl Validate for RemoveAudioOutputConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddAudioDecoderConfiguration {
    // This element contains a reference to the profile where the configuration
    // should be added.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // This element contains a reference to the AudioDecoderConfiguration to
    // add.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddAudioDecoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct AddAudioDecoderConfigurationResponse {}

impl Validate for AddAudioDecoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveAudioDecoderConfiguration {
    // This element contains a reference to the media profile from which the
    // AudioDecoderConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveAudioDecoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct RemoveAudioDecoderConfigurationResponse {}

impl Validate for RemoveAudioDecoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct DeleteProfile {
    // This element contains a reference to the profile that should be deleted.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for DeleteProfile {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct DeleteProfileResponse {}

impl Validate for DeleteProfileResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoEncoderConfigurations {}

impl Validate for GetVideoEncoderConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoEncoderConfigurationsResponse {
    // This element contains a list of video encoder configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoEncoderConfiguration>,
}

impl Validate for GetVideoEncoderConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourceConfigurations {}

impl Validate for GetVideoSourceConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourceConfigurationsResponse {
    // This element contains a list of video source configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoSourceConfiguration>,
}

impl Validate for GetVideoSourceConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioEncoderConfigurations {}

impl Validate for GetAudioEncoderConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioEncoderConfigurationsResponse {
    // This element contains a list of audio encoder configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioEncoderConfiguration>,
}

impl Validate for GetAudioEncoderConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioSourceConfigurations {}

impl Validate for GetAudioSourceConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioSourceConfigurationsResponse {
    // This element contains a list of audio source configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioSourceConfiguration>,
}

impl Validate for GetAudioSourceConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoAnalyticsConfigurations {}

impl Validate for GetVideoAnalyticsConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoAnalyticsConfigurationsResponse {
    // This element contains a list of VideoAnalytics configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoAnalyticsConfiguration>,
}

impl Validate for GetVideoAnalyticsConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetMetadataConfigurations {}

impl Validate for GetMetadataConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetMetadataConfigurationsResponse {
    // This element contains a list of metadata configurations
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::MetadataConfiguration>,
}

impl Validate for GetMetadataConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioOutputConfigurations {}

impl Validate for GetAudioOutputConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioOutputConfigurationsResponse {
    // This element contains a list of audio output configurations
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioOutputConfiguration>,
}

impl Validate for GetAudioOutputConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioDecoderConfigurations {}

impl Validate for GetAudioDecoderConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioDecoderConfigurationsResponse {
    // This element contains a list of audio decoder configurations
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioDecoderConfiguration>,
}

impl Validate for GetAudioDecoderConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourceConfiguration {
    // Token of the requested video source configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetVideoSourceConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourceConfigurationResponse {
    // The requested video source configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoSourceConfiguration,
}

impl Validate for GetVideoSourceConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoEncoderConfiguration {
    // Token of the requested video encoder configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetVideoEncoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoEncoderConfigurationResponse {
    // The requested video encoder configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoEncoderConfiguration,
}

impl Validate for GetVideoEncoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioSourceConfiguration {
    // Token of the requested audio source configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAudioSourceConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioSourceConfigurationResponse {
    // The requested audio source configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioSourceConfiguration,
}

impl Validate for GetAudioSourceConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioEncoderConfiguration {
    // Token of the requested audio encoder configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAudioEncoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioEncoderConfigurationResponse {
    // The requested audio encoder configuration
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioEncoderConfiguration,
}

impl Validate for GetAudioEncoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoAnalyticsConfiguration {
    // Token of the requested video analytics configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetVideoAnalyticsConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoAnalyticsConfigurationResponse {
    // The requested video analytics configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoAnalyticsConfiguration,
}

impl Validate for GetVideoAnalyticsConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetMetadataConfiguration {
    // Token of the requested metadata configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetMetadataConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetMetadataConfigurationResponse {
    // The requested metadata configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::MetadataConfiguration,
}

impl Validate for GetMetadataConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioOutputConfiguration {
    // Token of the requested audio output configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAudioOutputConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioOutputConfigurationResponse {
    // The requested audio output configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioOutputConfiguration,
}

impl Validate for GetAudioOutputConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioDecoderConfiguration {
    // Token of the requested audio decoder configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAudioDecoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioDecoderConfigurationResponse {
    // The requested audio decoder configuration
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioDecoderConfiguration,
}

impl Validate for GetAudioDecoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleVideoEncoderConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleVideoEncoderConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleVideoEncoderConfigurationsResponse {
    // Contains a list of video encoder configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoEncoderConfiguration>,
}

impl Validate for GetCompatibleVideoEncoderConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleVideoSourceConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleVideoSourceConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleVideoSourceConfigurationsResponse {
    // Contains a list of video source configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoSourceConfiguration>,
}

impl Validate for GetCompatibleVideoSourceConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleAudioEncoderConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleAudioEncoderConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleAudioEncoderConfigurationsResponse {
    // Contains a list of audio encoder configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioEncoderConfiguration>,
}

impl Validate for GetCompatibleAudioEncoderConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleAudioSourceConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleAudioSourceConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleAudioSourceConfigurationsResponse {
    // Contains a list of audio source configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioSourceConfiguration>,
}

impl Validate for GetCompatibleAudioSourceConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleVideoAnalyticsConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleVideoAnalyticsConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleVideoAnalyticsConfigurationsResponse {
    // Contains a list of video analytics configurations that are compatible
    // with the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoAnalyticsConfiguration>,
}

impl Validate for GetCompatibleVideoAnalyticsConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleMetadataConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleMetadataConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleMetadataConfigurationsResponse {
    // Contains a list of metadata configurations that are compatible with the
    // specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::MetadataConfiguration>,
}

impl Validate for GetCompatibleMetadataConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleAudioOutputConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleAudioOutputConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleAudioOutputConfigurationsResponse {
    // Contains a list of audio output configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioOutputConfiguration>,
}

impl Validate for GetCompatibleAudioOutputConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleAudioDecoderConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleAudioDecoderConfigurations {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetCompatibleAudioDecoderConfigurationsResponse {
    // Contains a list of audio decoder configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioDecoderConfiguration>,
}

impl Validate for GetCompatibleAudioDecoderConfigurationsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetVideoEncoderConfiguration {
    // Contains the modified video encoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoEncoderConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetVideoEncoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetVideoEncoderConfigurationResponse {}

impl Validate for SetVideoEncoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetVideoSourceConfiguration {
    // Contains the modified video source configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoSourceConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetVideoSourceConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetVideoSourceConfigurationResponse {}

impl Validate for SetVideoSourceConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetAudioEncoderConfiguration {
    // Contains the modified audio encoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioEncoderConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioEncoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetAudioEncoderConfigurationResponse {}

impl Validate for SetAudioEncoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetAudioSourceConfiguration {
    // Contains the modified audio source configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioSourceConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioSourceConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetAudioSourceConfigurationResponse {}

impl Validate for SetAudioSourceConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetVideoAnalyticsConfiguration {
    // Contains the modified video analytics configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoAnalyticsConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetVideoAnalyticsConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetVideoAnalyticsConfigurationResponse {}

impl Validate for SetVideoAnalyticsConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetMetadataConfiguration {
    // Contains the modified metadata configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::MetadataConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetMetadataConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetMetadataConfigurationResponse {}

impl Validate for SetMetadataConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetAudioOutputConfiguration {
    // Contains the modified audio output configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioOutputConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioOutputConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetAudioOutputConfigurationResponse {}

impl Validate for SetAudioOutputConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetAudioDecoderConfiguration {
    // Contains the modified audio decoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioDecoderConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioDecoderConfiguration {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetAudioDecoderConfigurationResponse {}

impl Validate for SetAudioDecoderConfigurationResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourceConfigurationOptions {
    // Optional video source configurationToken that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetVideoSourceConfigurationOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourceConfigurationOptionsResponse {
    // This message contains the video source configuration options. If a video
    // source configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::VideoSourceConfigurationOptions,
}

impl Validate for GetVideoSourceConfigurationOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoEncoderConfigurationOptions {
    // Optional video encoder configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetVideoEncoderConfigurationOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoEncoderConfigurationOptionsResponse {
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::VideoEncoderConfigurationOptions,
}

impl Validate for GetVideoEncoderConfigurationOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioSourceConfigurationOptions {
    // Optional audio source configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetAudioSourceConfigurationOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioSourceConfigurationOptionsResponse {
    // This message contains the audio source configuration options. If a audio
    // source configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::AudioSourceConfigurationOptions,
}

impl Validate for GetAudioSourceConfigurationOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioEncoderConfigurationOptions {
    // Optional audio encoder configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetAudioEncoderConfigurationOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioEncoderConfigurationOptionsResponse {
    // This message contains the audio encoder configuration options. If a audio
    // encoder configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::AudioEncoderConfigurationOptions,
}

impl Validate for GetAudioEncoderConfigurationOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetMetadataConfigurationOptions {
    // Optional metadata configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetMetadataConfigurationOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetMetadataConfigurationOptionsResponse {
    // This message contains the metadata configuration options. If a metadata
    // configuration is specified, the options shall concern that particular
    // configuration. If a media profile is specified, the options shall be
    // compatible with that media profile. If no tokens are specified, the
    // options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::MetadataConfigurationOptions,
}

impl Validate for GetMetadataConfigurationOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioOutputConfigurationOptions {
    // Optional audio output configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetAudioOutputConfigurationOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioOutputConfigurationOptionsResponse {
    // This message contains the audio output configuration options. If a audio
    // output configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::AudioOutputConfigurationOptions,
}

impl Validate for GetAudioOutputConfigurationOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioDecoderConfigurationOptions {
    // Optional audio decoder configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetAudioDecoderConfigurationOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetAudioDecoderConfigurationOptionsResponse {
    // This message contains the audio decoder configuration options. If a audio
    // decoder configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::AudioDecoderConfigurationOptions,
}

impl Validate for GetAudioDecoderConfigurationOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetGuaranteedNumberOfVideoEncoderInstances {
    // Token of the video source configuration
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetGuaranteedNumberOfVideoEncoderInstances {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetGuaranteedNumberOfVideoEncoderInstancesResponse {
    // The minimum guaranteed total number of encoder instances (applications)
    // per VideoSourceConfiguration. The device is able to deliver the
    // TotalNumber of streams
    #[yaserde(prefix = "trt", rename = "TotalNumber")]
    pub total_number: i32,

    // If a device limits the number of instances for respective Video Codecs
    // the response contains the information how many Jpeg streams can be set up
    // at the same time per VideoSource.
    #[yaserde(prefix = "trt", rename = "JPEG")]
    pub jpeg: Option<i32>,

    // If a device limits the number of instances for respective Video Codecs
    // the response contains the information how many H264 streams can be set up
    // at the same time per VideoSource.
    #[yaserde(prefix = "trt", rename = "H264")]
    pub h264: Option<i32>,

    // If a device limits the number of instances for respective Video Codecs
    // the response contains the information how many Mpeg4 streams can be set
    // up at the same time per VideoSource.
    #[yaserde(prefix = "trt", rename = "MPEG4")]
    pub mpeg4: Option<i32>,
}

impl Validate for GetGuaranteedNumberOfVideoEncoderInstancesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetStreamUri {
    // Stream Setup that should be used with the uri
    #[yaserde(prefix = "trt", rename = "StreamSetup")]
    pub stream_setup: tt::StreamSetup,

    // The ProfileToken element indicates the media profile to use and will
    // define the configuration of the content of the stream.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetStreamUri {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetStreamUriResponse {
    #[yaserde(prefix = "trt", rename = "MediaUri")]
    pub media_uri: tt::MediaUri,
}

impl Validate for GetStreamUriResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct StartMulticastStreaming {
    // Contains the token of the Profile that is used to define the multicast
    // stream.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for StartMulticastStreaming {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct StartMulticastStreamingResponse {}

impl Validate for StartMulticastStreamingResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct StopMulticastStreaming {
    // Contains the token of the Profile that is used to define the multicast
    // stream.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for StopMulticastStreaming {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct StopMulticastStreamingResponse {}

impl Validate for StopMulticastStreamingResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetSynchronizationPoint {
    // Contains a Profile reference for which a Synchronization Point is
    // requested.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for SetSynchronizationPoint {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetSynchronizationPointResponse {}

impl Validate for SetSynchronizationPointResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetSnapshotUri {
    // The ProfileToken element indicates the media profile to use and will
    // define the source and dimensions of the snapshot.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetSnapshotUri {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetSnapshotUriResponse {
    #[yaserde(prefix = "trt", rename = "MediaUri")]
    pub media_uri: tt::MediaUri,
}

impl Validate for GetSnapshotUriResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourceModes {
    // Contains a video source reference for which a video source mode is
    // requested.
    #[yaserde(prefix = "trt", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetVideoSourceModes {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetVideoSourceModesResponse {
    // Return the information for specified video source mode.
    #[yaserde(prefix = "trt", rename = "VideoSourceModes")]
    pub video_source_modes: Vec<VideoSourceMode>,
}

impl Validate for GetVideoSourceModesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetVideoSourceMode {
    // Contains a video source reference for which a video source mode is
    // requested.
    #[yaserde(prefix = "trt", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Indicate video source mode.
    #[yaserde(prefix = "trt", rename = "VideoSourceModeToken")]
    pub video_source_mode_token: tt::ReferenceToken,
}

impl Validate for SetVideoSourceMode {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetVideoSourceModeResponse {
    // The response contains information about rebooting after returning
    // response. When Reboot is set true, a device will reboot automatically
    // after setting mode.
    #[yaserde(prefix = "trt", rename = "Reboot")]
    pub reboot: bool,
}

impl Validate for SetVideoSourceModeResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct VideoSourceMode {
    // Max frame rate in frames per second for this video source mode.
    #[yaserde(prefix = "trt", rename = "MaxFramerate")]
    pub max_framerate: f64,

    // Max horizontal and vertical resolution for this video source mode.
    #[yaserde(prefix = "trt", rename = "MaxResolution")]
    pub max_resolution: tt::VideoResolution,

    // Indication which encodings are supported for this video source. The list
    // may contain one or more enumeration values of tt:VideoEncoding.
    #[yaserde(prefix = "trt", rename = "Encodings")]
    pub encodings: tt::StringList,

    // After setting the mode if a device starts to reboot this value is true.
    // If a device change the mode without rebooting this value is false. If
    // true, configured parameters may not be guaranteed by the device after
    // rebooting.
    #[yaserde(prefix = "trt", rename = "Reboot")]
    pub reboot: bool,

    // Informative description of this video source mode. This field should be
    // described in English.
    #[yaserde(prefix = "trt", rename = "Description")]
    pub description: Option<tt::Description>,

    #[yaserde(prefix = "trt", rename = "Extension")]
    pub extension: Option<VideoSourceModeExtension>,

    // Indicate token for video source mode.
    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,

    // Indication of whether this mode is active. If active this value is true.
    // In case of non-indication, it means as false. The value of true shall be
    // had by only one video source mode.
    #[yaserde(attribute, rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl Validate for VideoSourceMode {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct VideoSourceModeExtension {}

impl Validate for VideoSourceModeExtension {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetOSDs {
    // Token of the Video Source Configuration, which has OSDs associated with
    // are requested. If token not exist, request all available OSDs.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetOSDs {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetOSDsResponse {
    // This element contains a list of requested OSDs.
    #[yaserde(prefix = "trt", rename = "OSDs")]
    pub os_ds: Vec<tt::Osdconfiguration>,
}

impl Validate for GetOSDsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetOSD {
    // The GetOSD command fetches the OSD configuration if the OSD token is
    // known.
    #[yaserde(prefix = "trt", rename = "OSDToken")]
    pub osd_token: tt::ReferenceToken,
}

impl Validate for GetOSD {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetOSDResponse {
    // The requested OSD configuration.
    #[yaserde(prefix = "trt", rename = "OSD")]
    pub osd: tt::Osdconfiguration,
}

impl Validate for GetOSDResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetOSD {
    // Contains the modified OSD configuration.
    #[yaserde(prefix = "trt", rename = "OSD")]
    pub osd: tt::Osdconfiguration,
}

impl Validate for SetOSD {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct SetOSDResponse {}

impl Validate for SetOSDResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetOSDOptions {
    // Video Source Configuration Token that specifies an existing video source
    // configuration that the options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetOSDOptions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct GetOSDOptionsResponse {
    #[yaserde(prefix = "trt", rename = "OSDOptions")]
    pub osd_options: tt::OsdconfigurationOptions,
}

impl Validate for GetOSDOptionsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct CreateOSD {
    // Contain the initial OSD configuration for create.
    #[yaserde(prefix = "trt", rename = "OSD")]
    pub osd: tt::Osdconfiguration,
}

impl Validate for CreateOSD {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct CreateOSDResponse {
    // Returns Token of the newly created OSD
    #[yaserde(prefix = "trt", rename = "OSDToken")]
    pub osd_token: tt::ReferenceToken,
}

impl Validate for CreateOSDResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct DeleteOSD {
    // This element contains a reference to the OSD configuration that should be
    // deleted.
    #[yaserde(prefix = "trt", rename = "OSDToken")]
    pub osd_token: tt::ReferenceToken,
}

impl Validate for DeleteOSD {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "trt", namespace = "trt: http://www.onvif.org/ver10/media/wsdl")]
pub struct DeleteOSDResponse {}

impl Validate for DeleteOSDResponse {}

//NB: removed client-focused functions