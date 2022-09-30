use super::ServiceErrorDetail;
use hyper::StatusCode;

use imaging::{request, response};


pub struct ImagingService {

}

impl ImagingService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_request(&self, payload: impl std::io::Read) -> Result<String, ServiceErrorDetail> {
        let request: request::Envelope = yaserde::de::from_reader(payload)
            .map_err(|parse_err| ServiceErrorDetail::new(StatusCode::UNPROCESSABLE_ENTITY, Some(parse_err)))?;

        let response  = match request.body {
            request::Body::GetMoveOptions(_) => self.get_move_options()?,
            request::Body::GetOptions(_) => self.get_options()?,

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

    fn get_options(&self) -> Result<imaging::response::Envelope, ServiceErrorDetail> {
        Ok(response::Envelope{
            body: imaging::response::Body::GetOptionsResponse(imaging::GetOptionsResponse {
                imaging_options: onvif::ImagingOptions20::example()
            })
        })
    }

    fn get_move_options(&self) -> Result<imaging::response::Envelope, ServiceErrorDetail> {
        // All 'None' as there is no PTZ implementation
        Ok(response::Envelope {
            body: imaging::response::Body::GetMoveOptionsResponse(imaging::GetMoveOptionsResponse {
                move_options: onvif::MoveOptions20::example()
            })
        })
    }

}

//====| Example Data Implementations |=========================================================
use super::ExampleData;

impl ExampleData<onvif::MoveOptions20> for onvif::MoveOptions20 {
    fn example() -> onvif::MoveOptions20 {
        onvif::MoveOptions20 {
            absolute: None,
            relative: None,
            continuous: None
        }
    }
}

impl ExampleData<onvif::ImagingOptions20> for onvif::ImagingOptions20 {
    fn example() -> onvif::ImagingOptions20 {
        onvif::ImagingOptions20 {
            backlight_compensation: None,
            brightness: Some(onvif::FloatRange { min: 0.0, max: 100.0 }),
            color_saturation: None,
            contrast: None,
            exposure: None,
            focus: Some(onvif::FocusOptions20 {
                auto_focus_modes: vec![onvif::AutoFocusMode::Auto, onvif::AutoFocusMode::Manual],
                default_speed: Some(onvif::FloatRange{min: 0.1, max: 1.0}),
                near_limit: Some(onvif::FloatRange{min: 0.1, max: 3.0}),
                far_limit: Some(onvif::FloatRange{min: 0.0, max: 0.0}),
                extension: None }),
            ir_cut_filter_modes: vec![],
            sharpness: None,
            wide_dynamic_range: None,
            white_balance: None,
            extension: None
        }
    }
}