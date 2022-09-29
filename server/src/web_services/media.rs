use super::ServiceErrorDetail;
use hyper::StatusCode;

use media::{request, response};

use onvif::*;
use xsd_types::types::Duration;

pub struct MediaService {
}

impl MediaService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_request(&self, payload: impl std::io::Read) -> Result<String, ServiceErrorDetail> {
        let request: request::Envelope = yaserde::de::from_reader(payload)
            .map_err(|parse_err| ServiceErrorDetail::new(StatusCode::UNPROCESSABLE_ENTITY, Some(parse_err)))?;

        let response  = match request.body {
            media::request::Body::CreateProfile(_) => self.create_profile(&request)?,
            media::request::Body::DeleteProfile(_) => self.delete_profile(&request)?,
            media::request::Body::GetProfile(_) => self.get_profile(&request)?,
            media::request::Body::GetProfiles(_) => self.get_profiles()?,

            // media::request::Body::GetAudioEncoderConfigurationOptions(_) => todo!(),
            // media::request::Body::GetGuaranteedNumberOfVideoEncoderInstances(_) => todo!(),

            // media::request::Body::GetServiceCapabilities(_) => todo!(),
            // media::request::Body::GetSnapshotUri(_) => todo!(),
            // media::request::Body::GetStreamUri(_) => todo!(),

            // media::request::Body::GetVideoEncoderConfiguration(_) => todo!(),
            // media::request::Body::GetVideoEncoderConfigurationOptions(_) => todo!(),
            // media::request::Body::GetVideoEncoderConfigurations(_) => todo!(),

            // media::request::Body::GetVideoSourceConfigurationOptions(_) => todo!(),
            media::request::Body::GetVideoSourceConfigurations(_) => self.get_video_source_configurations()?,

            // media::request::Body::SetVideoEncoderConfiguration(_) => todo!(),

            _ => {
                return Err(ServiceErrorDetail::new(
                    StatusCode::NOT_IMPLEMENTED,
                    Some("Service not implemented.".to_string())
                ));
            }

        };

        let result = yaserde::ser::to_string(&response)
            .map_err(|ser_err| ServiceErrorDetail::new(StatusCode::INTERNAL_SERVER_ERROR, Some(ser_err)))?;
        Ok(result)
    }

    //====| Profiles |=========================================================

    fn get_profile(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {

        Ok(media::response::Envelope{
            body: media::response::Body::GetProfileResponse(media::GetProfileResponse {
                profile: make_example_profile()
            })
        })
    }

    fn get_profiles(&self) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetProfilesResponse(media::GetProfilesResponse {
                profiles: vec![make_example_profile()],
            })
        })
    }

    fn create_profile(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::CreateProfileResponse(media::CreateProfileResponse {
                profile: make_example_profile()
            })
        })
    }

    fn delete_profile(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::DeleteProfileResponse(media::DeleteProfileResponse {})
        })
    }

    fn get_video_source_configurations(&self) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Err(ServiceErrorDetail::new(
            StatusCode::NOT_IMPLEMENTED,
            Some("Service skeleton only.".to_string())
        ))
    }



}

/// Create an example Profile struct
fn make_example_profile() -> Profile {
    Profile {
        name: Name(
            "CurrentProfile".to_string(),
        ),
        video_source_configuration: Some(
            VideoSourceConfiguration {
                source_token: ReferenceToken(
                    "video_src_token".to_string(),
                ),
                bounds: IntRectangle {
                    x: 0,
                    y: 0,
                    width: 1920,
                    height: 1080,
                },
                extension: None,
                view_mode: None,
                name: Name(
                    "Primary Source".to_string(),
                ),
                use_count: 0,
                token: ReferenceToken(
                    "video_src_config_token".to_string(),
                ),
            },
        ),
        audio_source_configuration: None,
        video_encoder_configuration: Some(
            VideoEncoderConfiguration {
                encoding: VideoEncoding::H264,
                resolution: VideoResolution {
                    width: 1280,
                    height: 720,
                },
                quality: 1.0,
                rate_control: Some(
                    VideoRateControl {
                        frame_rate_limit: 25,
                        encoding_interval: 1,
                        bitrate_limit: 10000,
                    },
                ),
                mpeg4: None,
                h264: Some(
                    H264Configuration {
                        gov_length: 60,
                        h264_profile: H264Profile::High,
                    },
                ),
                multicast: MulticastConfiguration {
                    address: Ipaddress {
                        _type: Iptype::Ipv4,
                        i_pv_4_address: Some(
                            Ipv4Address(
                                "0.0.0.0".to_string(),
                            ),
                        ),
                        i_pv_6_address: None,
                    },
                    port: 0,
                    ttl: 1,
                    auto_start: false,
                },
                session_timeout: Duration {
                    is_negative: false,
                    years: 0,
                    months: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 1000.0,
                },
                guaranteed_frame_rate: None,
                name: Name(
                    "PiCameraConfiguration".to_string(),
                ),
                use_count: 0,
                token: ReferenceToken(
                    "encoder_config_token".to_string(),
                ),
            },
        ),
        audio_encoder_configuration: None,
        video_analytics_configuration: None,
        ptz_configuration: Some(
            Ptzconfiguration {
                node_token: ReferenceToken(
                    "ptz_node_token_0".to_string(),
                ),
                default_absolute_pant_tilt_position_space: Some(
                    "http://www.onvif.org/ver10/tptz/PanTiltSpaces/PositionGenericSpace".to_string(),
                ),
                default_absolute_zoom_position_space: Some(
                    "http://www.onvif.org/ver10/tptz/ZoomSpaces/PositionGenericSpace".to_string(),
                ),
                default_relative_pan_tilt_translation_space: Some(
                    "http://www.onvif.org/ver10/tptz/PanTiltSpaces/TranslationGenericSpace".to_string(),
                ),
                default_relative_zoom_translation_space: Some(
                    "http://www.onvif.org/ver10/tptz/ZoomSpaces/TranslationGenericSpace".to_string(),
                ),
                default_continuous_pan_tilt_velocity_space: Some(
                    "http://www.onvif.org/ver10/tptz/PanTiltSpaces/VelocityGenericSpace".to_string(),
                ),
                default_continuous_zoom_velocity_space: Some(
                    "http://www.onvif.org/ver10/tptz/ZoomSpaces/VelocityGenericSpace".to_string(),
                ),
                default_ptz_speed: Some(
                    Ptzspeed {
                        pan_tilt: Some(
                            Vector2D {
                                x: 1.0,
                                y: 1.0,
                                space: Some(
                                    "http://www.onvif.org/ver10/tptz/PanTiltSpaces/GenericSpeedSpace".to_string(),
                                ),
                            },
                        ),
                        zoom: Some(
                            Vector1D {
                                x: 1.0,
                                space: Some(
                                    "http://www.onvif.org/ver10/tptz/ZoomSpaces/ZoomGenericSpeedSpace".to_string(),
                                ),
                            },
                        ),
                    },
                ),
                default_ptz_timeout: Some(
                    Duration {
                        is_negative: false,
                        years: 0,
                        months: 0,
                        days: 0,
                        hours: 0,
                        minutes: 0,
                        seconds: 5.0,
                    },
                ),
                pan_tilt_limits: None,
                zoom_limits: None,
                extension: None,
                move_ramp: None,
                preset_ramp: None,
                preset_tour_ramp: None,
                name: Name(
                    "PTZ Configuration".to_string(),
                ),
                use_count: 1,
                token: ReferenceToken(
                    "ptz_config_token_0".to_string(),
                ),
            },
        ),
        metadata_configuration: None,
        extension: None,
        token: ReferenceToken(
            "profile_token".to_string(),
        ),
        fixed: None,
    }
}