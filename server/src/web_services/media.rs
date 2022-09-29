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
            media::request::Body::CreateProfile(_) => self.create_profiles(&request)?,
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

    fn get_profile(&self, request: &media::request::Envelope) -> Result<imaging::response::Envelope, ServiceErrorDetail> {
        Err(ServiceErrorDetail::new(
            StatusCode::NOT_IMPLEMENTED,
            Some("Service skeleton only.".to_string())
        ))
    }
    fn get_profiles(&self) -> Result<imaging::response::Envelope, ServiceErrorDetail> {
        Err(ServiceErrorDetail::new(
            StatusCode::NOT_IMPLEMENTED,
            Some("Service skeleton only.".to_string())
        ))
    }

    fn create_profiles(&self, request: &media::request::Envelope) -> Result<imaging::response::Envelope, ServiceErrorDetail> {
        Err(ServiceErrorDetail::new(
            StatusCode::NOT_IMPLEMENTED,
            Some("Service skeleton only.".to_string())
        ))
    }

    fn delete_profile(&self, request: &media::request::Envelope) -> Result<imaging::response::Envelope, ServiceErrorDetail> {
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