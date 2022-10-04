use super::*;

#[test]
fn test_get_capabilities() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">Yf+M2y5TUvLR5NvOpoVUO9LKYoU=</wsse:Password>
                        <wsse:Nonce>oCX3bZ2AdjxHxMrmJHSqsg==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:38Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <GetCapabilities xmlns="http://www.onvif.org/ver10/device/wsdl">
                    <Category>All</Category>
                </GetCapabilities>
            </s:Body>
        </s:Envelope>"#;

    let envelope: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(envelope.body, request::Body::GetCapabilities(_)));

    let security = envelope.header.unwrap().security.unwrap();
    assert_eq!(security.username_token.username, "admin");
    assert!(security.is_password_authentic("password123") );

}

#[test]
fn test_get_device_information() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">dQ6B1K6649bi4hSMMCWqDcXeMek=</wsse:Password>
                        <wsse:Nonce>80bFI0rnQlukEG1YJFAnnw==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:38Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <GetDeviceInformation xmlns="http://www.onvif.org/ver10/device/wsdl"/>
            </s:Body>
        </s:Envelope>"#;

    let envelope: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(envelope.body, request::Body::GetDeviceInformation(_)));

    let security = envelope.header.unwrap().security.unwrap();
    assert_eq!(security.username_token.username, "admin");
    assert!(security.is_password_authentic("password123") );
}

#[test]
fn test_get_network_interfaces() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">Ccoa9aha4twxul9oHORPKv4HBhI=</wsse:Password>
                        <wsse:Nonce>qatvkRqPThU8rNZQZfd/6A==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:38Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <GetNetworkInterfaces xmlns="http://www.onvif.org/ver10/device/wsdl"/>
            </s:Body>
        </s:Envelope>"#;

    let envelope: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(envelope.body, request::Body::GetNetworkInterfaces(_)));

    let security = envelope.header.unwrap().security.unwrap();
    assert_eq!(security.username_token.username, "admin");
    assert!(security.is_password_authentic("password123") );
}

#[test]
fn test_get_ntp() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">oIj/36xv/1C0q0fMIaGMLNd7j7A=</wsse:Password>
                        <wsse:Nonce>CbD5EGD9SyVT+mcqO1lBGw==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:39Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <GetNTP xmlns="http://www.onvif.org/ver10/device/wsdl"/>
            </s:Body>
        </s:Envelope>"#;

    let envelope: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(envelope.body, request::Body::GetNTP(_)));

    let security = envelope.header.unwrap().security.unwrap();
    assert_eq!(security.username_token.username, "admin");
    assert!(security.is_password_authentic("password123") );
}

#[test]
fn test_get_relay_outputs() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">J1fc9juLVQbFQppgAgztkwgpw+Q=</wsse:Password>
                        <wsse:Nonce>JaOIIn6rjnsSZUdBwOFtaw==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:39Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <GetRelayOutputs xmlns="http://www.onvif.org/ver10/device/wsdl"/>
            </s:Body>
        </s:Envelope>"#;

    let envelope: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(envelope.body, request::Body::GetRelayOutputs(_)));

    let security = envelope.header.unwrap().security.unwrap();
    assert_eq!(security.username_token.username, "admin");
    assert!(security.is_password_authentic("password123") );
}

#[test]
fn test_get_services() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">jNVDvrHgQWoia/3GDRt79Ba5fKY=</wsse:Password>
                        <wsse:Nonce>uY9+9uYgE7cz5knujgr5Vg==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:38Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <GetServices xmlns="http://www.onvif.org/ver10/device/wsdl">
                    <IncludeCapability>false</IncludeCapability>
                </GetServices>
            </s:Body>
        </s:Envelope>"#;

    let envelope: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(envelope.body, request::Body::GetServices(_)));

    let security = envelope.header.unwrap().security.unwrap();
    assert_eq!(security.username_token.username, "admin");
    assert!(security.is_password_authentic("password123") );
}

#[test]
fn test_set_relay_output_settings() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">2+sviBT2SaB6xBdj3Epu3UftAkM=</wsse:Password>
                        <wsse:Nonce>haNPFChvN21yh9AH2Z3LWg==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:39Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <SetRelayOutputSettings xmlns="http://www.onvif.org/ver10/device/wsdl">
                    <RelayOutputToken>relay1</RelayOutputToken>
                    <Properties>
                        <Mode xmlns="http://www.onvif.org/ver10/schema">Bistable</Mode>
                        <DelayTime xmlns="http://www.onvif.org/ver10/schema"/>
                        <IdleState xmlns="http://www.onvif.org/ver10/schema">closed</IdleState>
                    </Properties>
                </SetRelayOutputSettings>
            </s:Body>
        </s:Envelope>"#;

    let envelope: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(envelope.body, request::Body::SetRelayOutputSettings(_)));

    let security = envelope.header.unwrap().security.unwrap();
    assert_eq!(security.username_token.username, "admin");
    assert!(security.is_password_authentic("password123") );

}


#[test]
fn test_get_profile_request() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
            <s:Header>
                <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                    <wsse:UsernameToken>
                        <wsse:Username>admin</wsse:Username>
                        <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">2+sviBT2SaB6xBdj3Epu3UftAkM=</wsse:Password>
                        <wsse:Nonce>haNPFChvN21yh9AH2Z3LWg==</wsse:Nonce>
                        <wsu:Created>2022-03-07T13:19:39Z</wsu:Created>
                    </wsse:UsernameToken>
                </wsse:Security>
            </s:Header>
            <s:Body xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <SetRelayOutputSettings xmlns="http://www.onvif.org/ver10/device/wsdl">
                    <RelayOutputToken>relay1</RelayOutputToken>
                    <Properties>
                        <Mode xmlns="http://www.onvif.org/ver10/schema">Bistable</Mode>
                        <DelayTime xmlns="http://www.onvif.org/ver10/schema">P20M</DelayTime>
                        <IdleState xmlns="http://www.onvif.org/ver10/schema">closed</IdleState>
                    </Properties>
                </SetRelayOutputSettings>
            </s:Body>
        </s:Envelope>"#;

    let envelope: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(envelope.body, request::Body::SetRelayOutputSettings(_)));

    let security = envelope.header.unwrap().security.unwrap();
    assert_eq!(security.username_token.username, "admin");
    assert!(security.is_password_authentic("password123") );

}

#[test]
fn test_get_system_date_and_time() {
    let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
            <s:Body>
                <tds:GetSystemDateAndTime/>
            </s:Body>
        </s:Envelope>"#;

    let env: request::Envelope = yaserde::de::from_str(xml_str).unwrap();
    assert!(matches!(env.body, request::Body::GetSystemDateAndTime(_)));

    assert!(env.header.is_none());
}

#[test]
fn test_de_ok() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <tds:GetSystemDateAndTimeResponse xmlns="http://www.onvif.org/ver10/device/wsdl" xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
            <tds:SystemDateAndTime>
                <tt:DateTimeType xmlns:tt="http://www.onvif.org/ver10/schema">NTP</tt:DateTimeType>
                <tt:DaylightSavings xmlns:tt="http://www.onvif.org/ver10/schema">true</tt:DaylightSavings>
                <tt:TimeZone xmlns:tt="http://www.onvif.org/ver10/schema">
                    <tt:TZ>UTC-1</tt:TZ>
                </tt:TimeZone>
                <tt:UTCDateTime xmlns:tt="http://www.onvif.org/ver10/schema">
                    <tt:Time>
                        <tt:Hour>12</tt:Hour>
                        <tt:Minute>7</tt:Minute>
                        <tt:Second>18</tt:Second>
                    </tt:Time>
                    <tt:Date>
                        <tt:Year>2022</tt:Year>
                        <tt:Month>4</tt:Month>
                        <tt:Day>2</tt:Day>
                    </tt:Date>
                </tt:UTCDateTime>
                <tt:LocalDateTime xmlns:tt="http://www.onvif.org/ver10/schema">
                    <tt:Time>
                        <tt:Hour>13</tt:Hour>
                        <tt:Minute>7</tt:Minute>
                        <tt:Second>18</tt:Second>
                    </tt:Time>
                    <tt:Date>
                        <tt:Year>2022</tt:Year>
                        <tt:Month>4</tt:Month>
                        <tt:Day>2</tt:Day>
                    </tt:Date>
                </tt:LocalDateTime>
                <tt:Extension xmlns:tt="http://www.onvif.org/ver10/schema"/>
            </tds:SystemDateAndTime>
        </tds:GetSystemDateAndTimeResponse>"#;

    let response: GetSystemDateAndTimeResponse = yaserde::de::from_str(xml_str).unwrap();
    assert_eq!(response.system_date_and_time.utc_date_time.unwrap().time.second, 18);
}