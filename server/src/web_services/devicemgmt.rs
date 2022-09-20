// use chrono::{Timelike, Datelike};
// use onvif::{Time, Date, DateTime, TimeZone, SystemDateTime};
use chrono::prelude::*;
use devicemgmt::request;
use devicemgmt::response;
use std::error::Error; //TODO: understand this

pub async fn process_request(payload: String) -> Result<String, Box<dyn Error>> {

    let request: request::Envelope = yaserde::de::from_str(&payload).unwrap();//TODO: map errors
    //TODO: map parse failures to 501s

    match request.body {
        request::Body::GetCapabilities(_) => todo!(),
        request::Body::GetDeviceInformation(_) => todo!(),
        request::Body::GetNetworkInterfaces(_) => todo!(),
        request::Body::GetNTP(_) => todo!(),
        request::Body::GetRelayOutputs(_) => todo!(),
        request::Body::GetServices(_) => todo!(),
        request::Body::GetSystemDateAndTime(_) => Ok(get_system_date_and_time()),
        request::Body::SetRelayOutputSettings(_) => todo!(),

        _ => {
            // Feed back: the request is valid, but unimplemented
            todo!()
        }
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

fn get_system_date_and_time() -> String {
    let utc: DateTime<Utc> = Utc::now();
    let local_time = Local::now();

    // Timezone information via another 3rd party crate
    let time_now = libc_strftime::epoch();
    let timezone_name = libc_strftime::strftime_local("%Z", time_now);
    let dst = {
        let tm = libc_strftime::get_local_tm_from_epoch(time_now);
        tm.tm_isdst > 0
    };

    let result: response::Envelope = response::Envelope {
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
    };

    yaserde::ser::to_string(&result).unwrap() //TODO: relay errors as 500
}

#[cfg(test)]
mod test {


}
