use soap_fault::SoapFaultCode as Ter;
use imaging::{request, response};


pub struct ImagingService {

}

impl ImagingService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_request(&self, payload: impl std::io::Read) -> Result<String, Ter> {
        let request: request::Envelope = yaserde::de::from_reader(payload)
            .map_err(|_| Ter::WellFormed)?;

        // Check username/password
        super::authenticate(&request.header)?;

        let response  = match request.body {
            request::Body::GetMoveOptions(_) => self.get_move_options()?,
            request::Body::GetOptions(_) => self.get_options()?,

            _ => {
                return Err(Ter::ActionNotSupported);
            }

        };

        let result = yaserde::ser::to_string(&response)
            .map_err(|_| Ter::Action)?;
        Ok(result)
    }

    fn get_options(&self) -> Result<imaging::response::Envelope, Ter> {
        Ok(response::Envelope{
            body: imaging::response::Body::GetOptionsResponse(imaging::GetOptionsResponse {
                imaging_options: onvif::ImagingOptions20::example()
            })
        })
    }

    fn get_move_options(&self) -> Result<imaging::response::Envelope, Ter> {
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