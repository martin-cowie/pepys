use super::ServiceErrorDetail;
use hyper::StatusCode;

use media::request;
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

            media::request::Body::GetAudioEncoderConfigurationOptions(_) => self.get_audio_encoder_configuration_options(&request)?,
            media::request::Body::GetGuaranteedNumberOfVideoEncoderInstances(_) => self.get_guaranteed_number_of_video_encoder_instances(&request)?,
            media::request::Body::GetServiceCapabilities(_) => self.get_service_capabilities(&request)?,

            // media::request::Body::GetSnapshotUri(_) => todo!(),
            // media::request::Body::GetStreamUri(_) => todo!(),

            media::request::Body::GetVideoEncoderConfiguration(_) => self.get_video_encoder_configuration(&request)?,
            media::request::Body::GetVideoEncoderConfigurations(_) => self.get_video_encoder_configurations(&request)?,
            media::request::Body::GetVideoEncoderConfigurationOptions(_) => self.get_video_encoder_configuration_options(&request)?,
            media::request::Body::SetVideoEncoderConfiguration(_) => self.set_video_encoder_configuration(&request)?,


            media::request::Body::GetVideoSourceConfiguration(_) => self.get_video_source_configuration(&request)?,
            media::request::Body::GetVideoSourceConfigurations(_) => self.get_video_source_configurations(&request)?,


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

    //====| Misc |=============================================================

    fn get_audio_encoder_configuration_options(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetAudioEncoderConfigurationOptionsResponse(media::GetAudioEncoderConfigurationOptionsResponse {
                options: AudioEncoderConfigurationOptions {
                    options: vec![],
                }
            })
        })
    }

    fn get_guaranteed_number_of_video_encoder_instances(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetGuaranteedNumberOfVideoEncoderInstancesResponse(media::GetGuaranteedNumberOfVideoEncoderInstancesResponse {
                total_number: 1,
                jpeg: None,
                h264: Some(1),
                mpeg4: None
            })
        })
    }

    fn get_service_capabilities(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetServiceCapabilitiesResponse(media::GetServiceCapabilitiesResponse {
                capabilities: Capabilities::example()
             })
        })
    }

    //====| Video Encoder Configurations |=====================================

    fn get_video_encoder_configuration(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetVideoEncoderConfigurationResponse(media::GetVideoEncoderConfigurationResponse {
                configuration: VideoEncoderConfiguration::example()
            })
        })
    }

    fn get_video_encoder_configurations(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetVideoEncoderConfigurationsResponse(media::GetVideoEncoderConfigurationsResponse {
                configurations: vec![VideoEncoderConfiguration::example()]
            })
        })
    }

    fn get_video_encoder_configuration_options(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetVideoEncoderConfigurationOptionsResponse(media::GetVideoEncoderConfigurationOptionsResponse {
                options: VideoEncoderConfigurationOptions::example()
            })
        })
    }

    fn set_video_encoder_configuration(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::SetVideoEncoderConfigurationResponse(media::SetVideoEncoderConfigurationResponse {
                // Deliberately empty
            })
        })
    }

    //====| Profiles |=========================================================

    fn get_profile(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {

        Ok(media::response::Envelope{
            body: media::response::Body::GetProfileResponse(media::GetProfileResponse {
                profile: Profile::example()
            })
        })
    }

    fn get_profiles(&self) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetProfilesResponse(media::GetProfilesResponse {
                profiles: vec![Profile::example()],
            })
        })
    }

    fn create_profile(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::CreateProfileResponse(media::CreateProfileResponse {
                profile: Profile::example()
            })
        })
    }

    fn delete_profile(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::DeleteProfileResponse(media::DeleteProfileResponse {
                // Deliberately empty
            })
        })
    }

    //====| Video Source Configuration |=========================================================

    fn get_video_source_configuration(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetVideoSourceConfigurationResponse(media::GetVideoSourceConfigurationResponse {
                configuration: VideoSourceConfiguration::example(),
            })
        })
    }


    fn get_video_source_configurations(&self, _request: &media::request::Envelope) -> Result<media::response::Envelope, ServiceErrorDetail> {
        Ok(media::response::Envelope{
            body: media::response::Body::GetVideoSourceConfigurationsResponse(media::GetVideoSourceConfigurationsResponse {
                configurations: vec![VideoSourceConfiguration::example()]
            })
        })
    }

}

//====| Example Data Implementations |=========================================================

use super::ExampleData;

impl ExampleData<media::Capabilities> for Capabilities {
    fn example() -> media::Capabilities {

        media::Capabilities {
            profile_capabilities: vec![
                media::ProfileCapabilities {
                    maximum_number_of_profiles: Some(
                        1,
                    ),
                },
            ],
            streaming_capabilities: vec![
                media::StreamingCapabilities {
                    rtp_multicast: Some(
                        false,
                    ),
                    rtp_tcp: Some(
                        true,
                    ),
                    rtp_rtsp_tcp: Some(
                        true,
                    ),
                    non_aggregate_control: Some(
                        false,
                    ),
                    no_rtsp_streaming: Some(
                        false,
                    ),
                },
            ],
            snapshot_uri: Some(
                true,
            ),
            rotation: Some(
                false,
            ),
            video_source_mode: Some(
                true,
            ),
            osd: Some(
                false,
            ),
            temporary_osd_text: None,
            exi_compression: None,
        }

    }
}

impl ExampleData<VideoSourceConfiguration> for VideoSourceConfiguration {
    fn example() -> VideoSourceConfiguration {
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
            )
        }
    }
}

impl ExampleData<VideoEncoderConfigurationOptions> for VideoEncoderConfigurationOptions {
    fn example() -> VideoEncoderConfigurationOptions {
        VideoEncoderConfigurationOptions {
            quality_range: IntRange {
                min: 1,
                max: 1,
            },
            jpeg: None,
            mpeg4: None,
            h264: Some(
                H264Options {
                    resolutions_available: vec![
                        VideoResolution {
                            width: 640,
                            height: 480,
                        },
                        VideoResolution {
                            width: 800,
                            height: 600,
                        },
                        VideoResolution {
                            width: 1024,
                            height: 768,
                        },
                        VideoResolution {
                            width: 1280,
                            height: 1024,
                        },
                        VideoResolution {
                            width: 1280,
                            height: 720,
                        },
                        VideoResolution {
                            width: 1640,
                            height: 1232,
                        },
                        VideoResolution {
                            width: 1920,
                            height: 1080,
                        },
                    ],
                    gov_length_range: IntRange {
                        min: 0,
                        max: 2147483647,
                    },
                    frame_rate_range: IntRange {
                        min: 2,
                        max: 30,
                    },
                    encoding_interval_range: IntRange {
                        min: 1,
                        max: 1,
                    },
                    h264_profiles_supported: vec![
                        H264Profile::Baseline,
                        H264Profile::Main,
                        H264Profile::High,
                    ],
                },
            ),
            extension: Some(
                VideoEncoderOptionsExtension {
                    jpeg: None,
                    mpeg4: None,
                    h264: None,
                    extension: None,
                },
            ),
            guaranteed_frame_rate_supported: None,
        }
    }
}

impl ExampleData<VideoEncoderConfiguration> for VideoEncoderConfiguration {
    fn example() -> VideoEncoderConfiguration {
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
        }
    }
}

impl ExampleData<Profile> for Profile {
    fn example() -> Profile {
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
}
