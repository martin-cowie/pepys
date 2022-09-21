// use chrono::{Timelike, Datelike};
// use onvif::{Time, Date, DateTime, TimeZone, SystemDateTime};
use chrono::prelude::*;
use devicemgmt::request;
use devicemgmt::response;
use onvif::Ntpinformation;
use std::error::Error;

use crate::rpi; //TODO: understand this

pub async fn process_request(payload: String) -> Result<String, Box<dyn Error>> {

    let request: request::Envelope = yaserde::de::from_str(&payload).unwrap();//TODO: map errors
    //TODO: map parse failures to 501s

    let response = match request.body {
        request::Body::GetCapabilities(_) => todo!(),
        request::Body::GetDeviceInformation(_) => get_device_information(),
        request::Body::GetNetworkInterfaces(_) => todo!(),
        request::Body::GetNTP(_) => get_ntp(),
        request::Body::GetRelayOutputs(_) => todo!(),
        request::Body::GetServices(_) => todo!(),
        request::Body::GetSystemDateAndTime(_) => get_system_date_and_time(),
        request::Body::SetRelayOutputSettings(_) => todo!(),

        _ => {
            // Feed back: the request is valid, but unimplemented
            todo!()
        }
    };

    Ok(yaserde::ser::to_string(&response).unwrap()) //TODO: relay errors as 500

}

fn get_ntp() -> devicemgmt::response::Envelope {
    response::Envelope{
        body: response::Body::GetNTPResponse(devicemgmt::GetNTPResponse{ ntp_information: Ntpinformation{
            from_dhcp: true, //TODO: get from configuration
            ntp_from_dhcp: vec![],
            ntp_manual: vec![],
            extension: None
        } })
    }
}

fn get_device_information() -> response::Envelope {

    let hardware_info = rpi::RpiProcInfo::new().unwrap_or_default();

    response::Envelope{
        body: response::Body::GetDeviceInformationResponse(devicemgmt::GetDeviceInformationResponse {
            manufacturer: hardware_info.manufacturer,
            model: hardware_info.model,
            firmware_version: hardware_info.revision,
            serial_number: hardware_info.serial,
            hardware_id: hardware_info.hardware
        })
    }

}

fn to_date_time<T: chrono::TimeZone>(time: &DateTime<T>) -> onvif::DateTime {
    onvif::DateTime{
        time: onvif::Time{
            hour: time.hour() as i32,
            minute: time.minute() as i32,
            second: time.second() as i32 },
        date: onvif::Date{
            year: time.year() as i32,
            month: time.month() as i32,
            day: time.day() as i32 }
    }
}

fn get_system_date_and_time() -> response::Envelope {
    let utc: DateTime<Utc> = Utc::now();
    let local_time = Local::now();

    // Timezone information via another 3rd party crate
    let time_now = libc_strftime::epoch();
    let timezone_name = libc_strftime::strftime_local("%Z", time_now);
    let dst = {
        let tm = libc_strftime::get_local_tm_from_epoch(time_now);
        tm.tm_isdst > 0
    };

    response::Envelope {
        body: response::Body::GetSystemDateAndTimeResponse( devicemgmt::GetSystemDateAndTimeResponse{
            system_date_and_time: onvif::SystemDateTime {
                date_time_type: onvif::SetDateTimeType::Ntp,
                daylight_savings: dst,
                time_zone: Some(onvif::TimeZone {
                    tz: timezone_name
                }),
                utc_date_time: Some(to_date_time(&utc)),
                local_date_time: Some(to_date_time(&local_time)),
                extension: None
            },
        })
    }
}

#[cfg(test)]
mod test {


}
