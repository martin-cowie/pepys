use super::ServiceErrorDetail;
use hyper::StatusCode;

use media::{request, response};

pub struct MediaService {}

impl MediaService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_request(&self, payload: impl std::io::Read) -> Result<String, ServiceErrorDetail> {
        let request: request::Envelope = yaserde::de::from_reader(payload)
            .map_err(|parse_err| ServiceErrorDetail::new(StatusCode::UNPROCESSABLE_ENTITY, Some(parse_err)))?;

        let response  = match request.body {
            // media::request::Body::AddAudioDecoderConfiguration(_) => todo!(),
            // media::request::Body::AddAudioEncoderConfiguration(_) => todo!(),
            // media::request::Body::AddAudioOutputConfiguration(_) => todo!(),
            // media::request::Body::AddAudioSourceConfiguration(_) => todo!(),
            // media::request::Body::AddMetadataConfiguration(_) => todo!(),
            // media::request::Body::AddPTZConfiguration(_) => todo!(),
            // media::request::Body::AddVideoAnalyticsConfiguration(_) => todo!(),
            // media::request::Body::AddVideoEncoderConfiguration(_) => todo!(),
            // media::request::Body::AddVideoSourceConfiguration(_) => todo!(),
            // media::request::Body::CreateOSD(_) => todo!(),
            // media::request::Body::CreateProfile(_) => todo!(),
            // media::request::Body::DeleteOSD(_) => todo!(),
            // media::request::Body::DeleteProfile(_) => todo!(),
            // media::request::Body::GetAudioDecoderConfiguration(_) => todo!(),
            // media::request::Body::GetAudioDecoderConfigurationOptions(_) => todo!(),
            // media::request::Body::GetAudioDecoderConfigurations(_) => todo!(),
            // media::request::Body::GetAudioEncoderConfiguration(_) => todo!(),
            // media::request::Body::GetAudioEncoderConfigurationOptions(_) => todo!(),
            // media::request::Body::GetAudioEncoderConfigurations(_) => todo!(),
            // media::request::Body::GetAudioOutputConfiguration(_) => todo!(),
            // media::request::Body::GetAudioOutputConfigurationOptions(_) => todo!(),
            // media::request::Body::GetAudioOutputConfigurations(_) => todo!(),
            // media::request::Body::GetAudioOutputs(_) => todo!(),
            // media::request::Body::GetAudioSourceConfiguration(_) => todo!(),
            // media::request::Body::GetAudioSourceConfigurationOptions(_) => todo!(),
            // media::request::Body::GetAudioSourceConfigurations(_) => todo!(),
            // media::request::Body::GetAudioSources(_) => todo!(),
            // media::request::Body::GetCompatibleAudioDecoderConfigurations(_) => todo!(),
            // media::request::Body::GetCompatibleAudioEncoderConfigurations(_) => todo!(),
            // media::request::Body::GetCompatibleAudioOutputConfigurations(_) => todo!(),
            // media::request::Body::GetCompatibleAudioSourceConfigurations(_) => todo!(),
            // media::request::Body::GetCompatibleMetadataConfigurations(_) => todo!(),
            // media::request::Body::GetCompatibleVideoAnalyticsConfigurations(_) => todo!(),
            // media::request::Body::GetCompatibleVideoEncoderConfigurations(_) => todo!(),
            // media::request::Body::GetCompatibleVideoSourceConfigurations(_) => todo!(),
            // media::request::Body::GetGuaranteedNumberOfVideoEncoderInstances(_) => todo!(),
            // media::request::Body::GetMetadataConfiguration(_) => todo!(),
            // media::request::Body::GetMetadataConfigurationOptions(_) => todo!(),
            // media::request::Body::GetMetadataConfigurations(_) => todo!(),
            // media::request::Body::GetOSD(_) => todo!(),
            // media::request::Body::GetOSDOptions(_) => todo!(),
            // media::request::Body::GetOSDs(_) => todo!(),
            // media::request::Body::GetProfile(_) => todo!(),
            media::request::Body::GetProfiles(_) => self.get_profiles()?,
            // media::request::Body::GetServiceCapabilities(_) => todo!(),
            // media::request::Body::GetSnapshotUri(_) => todo!(),
            // media::request::Body::GetStreamUri(_) => todo!(),
            // media::request::Body::GetVideoAnalyticsConfiguration(_) => todo!(),
            // media::request::Body::GetVideoAnalyticsConfigurations(_) => todo!(),
            // media::request::Body::GetVideoEncoderConfiguration(_) => todo!(),
            // media::request::Body::GetVideoEncoderConfigurationOptions(_) => todo!(),
            // media::request::Body::GetVideoEncoderConfigurations(_) => todo!(),
            // media::request::Body::GetVideoSourceConfiguration(_) => todo!(),
            // media::request::Body::GetVideoSourceConfigurationOptions(_) => todo!(),
            media::request::Body::GetVideoSourceConfigurations(_) => self.get_video_source_configurations()?,
            // media::request::Body::GetVideoSourceModes(_) => todo!(),
            // media::request::Body::GetVideoSources(_) => todo!(),
            // media::request::Body::RemoveAudioDecoderConfiguration(_) => todo!(),
            // media::request::Body::RemoveAudioEncoderConfiguration(_) => todo!(),
            // media::request::Body::RemoveAudioOutputConfiguration(_) => todo!(),
            // media::request::Body::RemoveAudioSourceConfiguration(_) => todo!(),
            // media::request::Body::RemoveMetadataConfiguration(_) => todo!(),
            // media::request::Body::RemovePTZConfiguration(_) => todo!(),
            // media::request::Body::RemoveVideoAnalyticsConfiguration(_) => todo!(),
            // media::request::Body::RemoveVideoEncoderConfiguration(_) => todo!(),
            // media::request::Body::RemoveVideoSourceConfiguration(_) => todo!(),
            // media::request::Body::SetAudioDecoderConfiguration(_) => todo!(),
            // media::request::Body::SetAudioEncoderConfiguration(_) => todo!(),
            // media::request::Body::SetAudioOutputConfiguration(_) => todo!(),
            // media::request::Body::SetAudioSourceConfiguration(_) => todo!(),
            // media::request::Body::SetMetadataConfiguration(_) => todo!(),
            // media::request::Body::SetOSD(_) => todo!(),
            // media::request::Body::SetSynchronizationPoint(_) => todo!(),
            // media::request::Body::SetVideoAnalyticsConfiguration(_) => todo!(),
            // media::request::Body::SetVideoEncoderConfiguration(_) => todo!(),
            // media::request::Body::SetVideoSourceConfiguration(_) => todo!(),
            // media::request::Body::SetVideoSourceMode(_) => todo!(),
            // media::request::Body::StartMulticastStreaming(_) => todo!(),
            // media::request::Body::StopMulticastStreaming(_) => todo!(),

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

    fn get_profiles(&self) -> Result<imaging::response::Envelope, ServiceErrorDetail> {
        Err(ServiceErrorDetail::new(
            StatusCode::NOT_IMPLEMENTED,
            Some("Service skeleton only.".to_string())
        ))
    }

    fn get_video_source_configurations(&self) -> Result<imaging::response::Envelope, ServiceErrorDetail> {
        Err(ServiceErrorDetail::new(
            StatusCode::NOT_IMPLEMENTED,
            Some("Service skeleton only.".to_string())
        ))
    }

}