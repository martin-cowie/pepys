use chrono::prelude::*;
use devicemgmt::{request, response};
use get_if_addrs::{Interface, IfAddr, get_if_addrs};
use hyper::Uri;
use onvif::Ntpinformation;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

use crate::rpi;
use super::Authenticator;
use super::ExampleData;
use soap_fault::SoapFaultCode as Ter;

static VERSION_MAJOR: i32 = 2;
static VERSION_MINOR: i32 = 5;

/// Count the width of an Ipv4Addr or Ipv6Addr .. or anything with an `octects` method
macro_rules! netmask_width {
    ($netmask:expr) => {
        if let Some(netmask) = $netmask {
            netmask.octets().iter().map(|x| count_ones(*x) as i32).sum()
        } else {
            0
        }
    }
}


pub struct DeviceManagmentService {
    service_address: Uri,
    imaging_address: Uri,
    media_address: Uri,
    events_address: Uri,
    authenticator: &'static Authenticator,
    has_ip_v6: bool
}

impl DeviceManagmentService {

    pub fn new(service_address: Uri, imaging_address: Uri, media_address: Uri, events_address: Uri, authenticator: &'static Authenticator) -> Self {

        Self {
            service_address,
            imaging_address,
            media_address,
            events_address,
            authenticator,
            has_ip_v6: Self::get_has_ip_v6(get_if_addrs().expect("Cannot get NICs"))
        }
    }

    pub fn process_request(&self, payload: impl std::io::Read) -> Result<String, Ter> {
        let request: request::Envelope = yaserde::de::from_reader(payload)
            .map_err(|_| Ter::WellFormed)?;

        let response = if let request::Body::GetSystemDateAndTime(_) = request.body {
            // NB: get_system_date_and_time does NOT mandate a password header.
            self.get_system_date_and_time()?
        } else {

            // Check username/password
            self.authenticator.authenticate(&request.header)?;

            match request.body {
                request::Body::GetDeviceInformation(_) => self.get_device_information()?,
                request::Body::GetNetworkInterfaces(_) => self.get_network_interfaces()?,
                request::Body::GetNTP(_) => self.get_ntp()?,
                request::Body::GetSystemDateAndTime(_) => self.get_system_date_and_time()?,
                request::Body::GetCapabilities(_) => self.get_capabilities()?,
                request::Body::GetRelayOutputs(_) => self.get_relay_outputs()?,
                // request::Body::SetRelayOutputSettings(_) => self.get_relay_outputs()?, // Deliberately omitted until a references can be found
                request::Body::GetServices(_) => self.get_services()?,

                _ => {
                    return Err(Ter::ActionNotSupported);
                }
            }
        };


        let result = yaserde::ser::to_string(&response)
            .map_err(|_| Ter::Action)?;
        Ok(result)
    }

    fn get_has_ip_v6(nics: Vec<Interface>) -> bool {
        nics
            .into_iter()
            .any(|nic| matches!(nic.addr, IfAddr::V6(_)) )
    }

    //===| Request Handlers |=======

    fn get_capabilities(&self) -> Result<devicemgmt::response::Envelope, Ter> {
        let device = onvif::DeviceCapabilities{
            x_addr: self.service_address.to_string(),
            network: Some(onvif::NetworkCapabilities {
                ip_filter: Some(false),
                zero_configuration: Some(false),
                ip_version_6: Some(self.has_ip_v6),
                dyn_dns: Some(false),
                extension: None }),
            system: Some(onvif::SystemCapabilities{
                discovery_resolve: false,
                discovery_bye: false,
                remote_discovery: false,
                system_backup: false,
                system_logging: false,
                firmware_upgrade: false,
                supported_versions: vec![onvif::OnvifVersion{
                    major: VERSION_MAJOR,
                    minor: VERSION_MINOR }],
                extension: None }),
            io: Some(onvif::Iocapabilities{
                input_connectors: Some(0),
                relay_outputs: Some(1),
                extension: None }),
            security: Some(onvif::SecurityCapabilities{
                tls1_1: false,
                tls1_2: false,
                onboard_key_generation: false,
                access_policy_config: false,
                x_509_token: false,
                saml_token: false,
                kerberos_token: false,
                rel_token: false,
                extension: None,
            }),

            extension: None
        };

        let imaging = onvif::ImagingCapabilities {
            x_addr: self.imaging_address.to_string()
        };

        //TODO: get these details from the horses mouth
        let media = onvif::MediaCapabilities {
            x_addr: self.media_address.to_string(),
            streaming_capabilities: onvif::RealTimeStreamingCapabilities {
                rtp_multicast: Some(false),
                rtp_tcp: Some(true),
                rtp_rtsp_tcp: Some(true),
                extension: None,
            },
            extension: None,
        };

        let events = onvif::EventCapabilities {
            x_addr: self.events_address.to_string(),
            ws_subscription_policy_support: false,
            ws_pull_point_support: false,
            ws_pausable_subscription_manager_interface_support: false
        };

        Ok(response::Envelope{
            body: response::Body::GetCapabilitiesResponse(devicemgmt::GetCapabilitiesResponse {
                capabilities: onvif::Capabilities {
                    analytics: vec![],
                    device: vec![device],
                    events: vec![events],
                    imaging: vec![imaging],
                    media: vec![media],
                    ptz: vec![],
                    extension: None
                }
        })})

        // Ok(response::Envelope { body: response::Body::GetCapabilitiesResponse(devicemgmt::GetCapabilitiesResponse::example()) })

    }

    fn get_network_interfaces(&self) -> Result<devicemgmt::response::Envelope, Ter> {
        let network_interfaces = NetworkInterface::show()
            .map_err(|_| Ter::WellFormed)?;

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
                                prefix_length: netmask_width!(v4_addr.netmask)
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
                                prefix_length: netmask_width!(v6_addr.netmask)
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

    fn get_ntp(&self) -> Result<devicemgmt::response::Envelope, Ter> {
        Ok(response::Envelope{
            body: response::Body::GetNTPResponse(devicemgmt::GetNTPResponse{
                ntp_information: onvif::Ntpinformation::example()
            })
        })
    }

    fn get_device_information(&self) -> Result<devicemgmt::response::Envelope, Ter> {

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

    fn get_system_date_and_time(&self) -> Result<devicemgmt::response::Envelope, Ter> {
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

    fn get_relay_outputs(&self) -> Result<devicemgmt::response::Envelope, Ter> {
        Ok(response::Envelope {
            body: response::Body::GetRelayOutputsResponse( devicemgmt::GetRelayOutputsResponse {
                relay_outputs: vec![onvif::RelayOutput::example()]
            })
        })
    }

    fn get_services(&self) -> Result<devicemgmt::response::Envelope, Ter> {

        Ok(response::Envelope {
            body: response::Body::GetServicesResponse(devicemgmt::GetServicesResponse {
                service:vec![
                    devicemgmt::Service {
                        namespace: "http://www.onvif.org/ver10/device/wsdl".to_string(),
                        x_addr: self.service_address.to_string(),
                        capabilities: None,
                        version: onvif::OnvifVersion{
                            major: VERSION_MAJOR,
                            minor: VERSION_MINOR
                        }
                    },
                    devicemgmt::Service {
                        namespace: "http://www.onvif.org/ver20/imaging/wsdl".to_string(),
                        x_addr: self.imaging_address.to_string(),
                        capabilities: None,
                        version: onvif::OnvifVersion{
                            major: VERSION_MAJOR,
                            minor: VERSION_MINOR
                        }
                    },

                    devicemgmt::Service {
                        namespace: "http://www.onvif.org/ver10/media/wsdl".to_string(),
                        x_addr: self.media_address.to_string(),
                        capabilities: None,
                        version: onvif::OnvifVersion{
                            major: VERSION_MAJOR,
                            minor: VERSION_MINOR
                        }
                    },

                    devicemgmt::Service {
                        namespace: "http://www.onvif.org/ver10/events/wsdl".to_string(),
                        x_addr: self.events_address.to_string(),
                        capabilities: None,
                        version: onvif::OnvifVersion{
                            major: VERSION_MAJOR,
                            minor: VERSION_MINOR
                        }
                    }
                ]
            })
        })


    }

}

//====| Example Data Implementations |=========================================================

impl ExampleData<onvif::Ntpinformation> for onvif::Ntpinformation {
    fn example() -> onvif::Ntpinformation {
        Ntpinformation{
            from_dhcp: true,
            ntp_from_dhcp: vec![],
            ntp_manual: vec![],
            extension: None
        }
    }
}

impl ExampleData<onvif::RelayOutput> for onvif::RelayOutput {
    fn example() -> onvif::RelayOutput {
        onvif::RelayOutput{
            properties: onvif::RelayOutputSettings {
                mode: onvif::RelayMode::Bistable,
                delay_time: "5s".to_string(), //TODO: defer to docs for reasonable value
                idle_state: onvif::RelayIdleState::Open
            },
            token:  onvif::ReferenceToken("relay1".to_string())
        }
    }
}


//===| Support functions |=======

fn to_date_time<T: chrono::TimeZone>(time: &DateTime<T>) -> onvif::DateTime {
    onvif::DateTime{
        time: onvif::Time{
            hour: time.hour() as i32,
            minute: time.minute() as i32,
            second: time.second() as i32 },
        date: onvif::Date{
            year: time.year(),
            month: time.month() as i32,
            day: time.day() as i32 }
    }
}


/// Count the set bits in a byte using Brain Kerningham's algorithm.
fn count_ones(mut value: u8) -> u8 {
    let mut result = 0;
    while value > 0 {
        value &= value -1;
        result += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use get_if_addrs::Ifv6Addr;
    use network_interface::Netmask;
    use std::net::{Ipv4Addr, Ipv6Addr};


    #[test]
    pub fn test_netmask_width() {
        let mask: Netmask<Ipv4Addr> = None;

        assert_eq!(0, netmask_width!(mask));
        assert_eq!(8, netmask_width!(Some(Ipv4Addr::new(127, 0, 0, 1))));
        assert_eq!(5, netmask_width!(Some(Ipv4Addr::new(1, 2, 3, 4))));
        assert_eq!(24, netmask_width!(Some(Ipv4Addr::new(255, 255, 255, 0))));
        assert_eq!(16, netmask_width!(Some(Ipv4Addr::new(255, 255, 0, 0))));
        assert_eq!(8, netmask_width!(Some(Ipv4Addr::new(255, 0, 0, 0))));

        assert_eq!(1, netmask_width!(Some(Ipv6Addr::LOCALHOST)));
        assert_eq!(4, netmask_width!(Some(Ipv6Addr::new(0,0,0,0,0,3,2,1))));
        assert_eq!(4, netmask_width!(Some(Ipv6Addr::new(0,0,0,0,0,3,2,1))));
        assert_eq!(16, netmask_width!(Some(Ipv6Addr::new(0xFFFF,0,0,0,0,0,0,0))));
    }

    #[test]
    pub fn test_count_ones() {
        assert_eq!(count_ones(0), 0);
        assert_eq!(count_ones(0b11111111), 8);
        assert_eq!(count_ones(0b10001000), 2);
        assert_eq!(count_ones(0b10001000), 2);
    }

    #[test]
    pub fn test_has_ip_v6_true() {
        let ip_v6_nic: Interface = Interface{ //TODO: Mock this
            name: "test-nic".to_string(),
            addr: IfAddr::V6(Ifv6Addr{
                ip: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
                netmask: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
                broadcast: None })
        };

        let no_v6_nics = vec![];
        let with_v6_nics = vec![ip_v6_nic];

        assert_eq!(true, DeviceManagmentService::get_has_ip_v6(with_v6_nics));
        assert_eq!(false, DeviceManagmentService::get_has_ip_v6(no_v6_nics));
    }

}