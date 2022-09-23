use chrono::prelude::*;
use devicemgmt::request;
use devicemgmt::response;
use onvif::Ntpinformation;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use hyper::StatusCode;


use crate::rpi;
use super::ServiceErrorDetail;

pub async fn process_request(payload: impl std::io::Read) -> Result<String, ServiceErrorDetail> {

    let request: request::Envelope = yaserde::de::from_reader(payload)
        .map_err(|parse_err| ServiceErrorDetail::new(StatusCode::UNPROCESSABLE_ENTITY, Some(parse_err)))?;

    let response = match request.body {
        request::Body::GetDeviceInformation(_) => get_device_information()?,
        request::Body::GetNetworkInterfaces(_) => get_network_interfaces()?,
        request::Body::GetNTP(_) => get_ntp()?,
        request::Body::GetSystemDateAndTime(_) => get_system_date_and_time()?,
        // request::Body::GetCapabilities(_) => todo!(),
        // request::Body::GetRelayOutputs(_) => todo!(),
        // request::Body::GetServices(_) => todo!(),
        // request::Body::SetRelayOutputSettings(_) => todo!(),

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

//===| Request Handlers |=======

fn get_network_interfaces() -> Result<devicemgmt::response::Envelope, ServiceErrorDetail> {
    let network_interfaces = NetworkInterface::show()
        .map_err(|e| ServiceErrorDetail::new(StatusCode::INTERNAL_SERVER_ERROR, Some(e.to_string())))?;

    let nic_structs: Vec<onvif::NetworkInterface> = network_interfaces.iter()
        .map(|nic| {
        onvif::NetworkInterface{
            enabled: true, //TODO: this is a guess
            info: Some(onvif::NetworkInterfaceInfo{
                name: Some(nic.name.clone()),
                hw_address: onvif::HwAddress(nic.mac_addr.clone().unwrap_or_default()),
                mtu: None,
            }),
            link: None,
            i_pv_4: if let Some(network_interface::Addr::V4(v4_addr)) = nic.addr {vec![
                onvif::Ipv4NetworkInterface {
                    enabled: true, //TODO: this is a guess
                    config: onvif::Ipv4Configuration {
                        manual: vec![],
                        link_local: None,
                        from_dhcp: Some(onvif::PrefixedIPv4Address {
                            address: onvif::Ipv4Address(v4_addr.ip.to_string()),
                            prefix_length: v4_netmask_width(v4_addr.netmask)
                        }),
                        dhcp: true
                    }
                }
            ]} else {vec![]},
            i_pv_6: if let Some(network_interface::Addr::V6(v6_addr)) = nic.addr {vec![
                onvif::Ipv6NetworkInterface {
                    enabled:true, //TODO: this is a guess
                    config: Some(onvif::Ipv6Configuration{
                        accept_router_advert: None,
                        dhcp: onvif::Ipv6DHCPConfiguration::Auto,
                        manual: vec![],
                        link_local: vec![],
                        from_dhcp: vec![onvif::PrefixedIPv6Address{
                            address: onvif::Ipv6Address(v6_addr.ip.to_string()),
                            prefix_length: v6_netmask_width(v6_addr.netmask)
                        }],
                        from_ra: vec![],
                        extension: None }) }
            ]} else {vec![]},
            extension: None,
            token: onvif::ReferenceToken(nic.name.clone()),
        }
    }).collect();

    Ok(response::Envelope{
        body: response::Body::GetNetworkInterfacesResponse(devicemgmt::GetNetworkInterfacesResponse {
            network_interfaces: nic_structs
        })
    })
}


fn get_ntp() -> Result<devicemgmt::response::Envelope, ServiceErrorDetail> {
    Ok(response::Envelope{
        body: response::Body::GetNTPResponse(devicemgmt::GetNTPResponse{ ntp_information: Ntpinformation{
            from_dhcp: true, //TODO: get from configuration
            ntp_from_dhcp: vec![],
            ntp_manual: vec![],
            extension: None
        } })
    })
}

fn get_device_information() -> Result<devicemgmt::response::Envelope, ServiceErrorDetail> {

    let hardware_info = rpi::RpiProcInfo::new().unwrap_or_default();

    Ok(response::Envelope{
        body: response::Body::GetDeviceInformationResponse(devicemgmt::GetDeviceInformationResponse {
            manufacturer: hardware_info.manufacturer,
            model: hardware_info.model,
            firmware_version: hardware_info.revision,
            serial_number: hardware_info.serial,
            hardware_id: hardware_info.hardware
        })
    })

}

fn get_system_date_and_time() -> Result<devicemgmt::response::Envelope, ServiceErrorDetail> {
    let utc: DateTime<Utc> = Utc::now();
    let local_time = Local::now();

    // Timezone information via another 3rd party crate
    let time_now = libc_strftime::epoch();
    let timezone_name = libc_strftime::strftime_local("%Z", time_now);
    let dst = {
        let tm = libc_strftime::get_local_tm_from_epoch(time_now);
        tm.tm_isdst > 0
    };

    Ok(response::Envelope {
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
    })
}

//===| Support functions |=======

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

//TODO: these two function are identical

fn v6_netmask_width(netmask: network_interface::Netmask<Ipv6Addr>) -> i32 {
    if let Some(netmask) = netmask {
        netmask.octets().iter().map(|x| *x as i32).sum()
    } else {
        0
    }
}

fn v4_netmask_width(netmask: network_interface::Netmask<Ipv4Addr>) -> i32 {
    if let Some(netmask) = netmask {
        netmask.octets().iter().map(|x| *x as i32).sum()
    } else {
        0
    }
}


#[cfg(test)]
mod test {

    use network_interface::NetworkInterface;
    use network_interface::NetworkInterfaceConfig;

    #[test]
    fn test_crate() {
        let network_interfaces = NetworkInterface::show().unwrap();

        for itf in network_interfaces.iter() {


            println!("{:#?}", itf);

        }
    }

}
