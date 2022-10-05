use yaserde_derive::*;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soap", namespace = "soap: http://www.w3.org/2003/05/soap-envelope")]
struct Envelope {

    #[yaserde(prefix = "soap", rename = "Body")]
    pub body: Body
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soap")
]
struct Body {

    #[yaserde(prefix = "soap", rename = "Fault")]
    pub fault: soapenv::Fault
}

#[cfg(test)]
mod test {

    use super::*;

    const XML: &str = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <SOAP-ENV:Envelope xmlns:SOAP-ENC="http://www.w3.org/2003/05/soap-encoding" xmlns:SOAP-ENV="http://www.w3.org/2003/05/soap-envelope" xmlns:ns1="http://www.onvif.org/ver10/actionengine/wsdl" xmlns:ns10="http://www.onvif.org/ver10/events/wsdl/PullPointBinding" xmlns:ns11="http://www.onvif.org/ver10/events/wsdl/CreatePullPointBinding" xmlns:ns12="http://www.onvif.org/ver10/events/wsdl/PausableSubscriptionManagerBinding" xmlns:ns13="http://www.onvif.org/ver10/network/wsdl/RemoteDiscoveryBinding" xmlns:ns14="http://www.onvif.org/ver10/network/wsdl/DiscoveryLookupBinding" xmlns:ns3="http://www.onvif.org/ver20/analytics/wsdl/RuleEngineBinding" xmlns:ns4="http://www.onvif.org/ver20/analytics/wsdl/AnalyticsEngineBinding" xmlns:ns5="http://www.onvif.org/ver10/events/wsdl/PullPointSubscriptionBinding" xmlns:ns6="http://www.onvif.org/ver10/events/wsdl/EventBinding" xmlns:ns7="http://www.onvif.org/ver10/events/wsdl/SubscriptionManagerBinding" xmlns:ns8="http://www.onvif.org/ver10/events/wsdl/NotificationProducerBinding" xmlns:ns9="http://www.onvif.org/ver10/events/wsdl/NotificationConsumerBinding" xmlns:tad="http://www.onvif.org/ver10/analyticsdevice/wsdl" xmlns:tan="http://www.onvif.org/ver20/analytics/wsdl" xmlns:tdn="http://www.onvif.org/ver10/network/wsdl" xmlns:tds="http://www.onvif.org/ver10/device/wsdl" xmlns:ter="http://www.onvif.org/ver10/error" xmlns:tev="http://www.onvif.org/ver10/events/wsdl" xmlns:timg="http://www.onvif.org/ver20/imaging/wsdl" xmlns:tls="http://www.onvif.org/ver10/display/wsdl" xmlns:tmd="http://www.onvif.org/ver10/deviceIO/wsdl" xmlns:tns1="http://www.onvif.org/ver10/topics" xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl" xmlns:trc="http://www.onvif.org/ver10/recording/wsdl" xmlns:trp="http://www.onvif.org/ver10/replay/wsdl" xmlns:trt="http://www.onvif.org/ver10/media/wsdl" xmlns:trv="http://www.onvif.org/ver10/receiver/wsdl" xmlns:tse="http://www.onvif.org/ver10/search/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:wsa="http://schemas.xmlsoap.org/ws/2004/08/addressing" xmlns:wsa5="http://www.w3.org/2005/08/addressing" xmlns:wsdd="http://schemas.xmlsoap.org/ws/2005/04/discovery" xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2" xmlns:wsrfbf="http://docs.oasis-open.org/wsrf/bf-2" xmlns:wsrfr="http://docs.oasis-open.org/wsrf/r-2" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wstop="http://docs.oasis-open.org/wsn/t-1" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" xmlns:xmime="http://tempuri.org/xmime.xsd" xmlns:xmime5="http://www.w3.org/2005/05/xmlmime" xmlns:xop="http://www.w3.org/2004/08/xop/include" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <SOAP-ENV:Body>
            <SOAP-ENV:Fault SOAP-ENV:encodingStyle="http://www.w3.org/2003/05/soap-encoding">
                <SOAP-ENV:Code>
                    <SOAP-ENV:Value>SOAP-ENV:Sender</SOAP-ENV:Value>
                    <SOAP-ENV:Subcode>
                        <SOAP-ENV:Value>ter:NotAuthorized</SOAP-ENV:Value>
                    </SOAP-ENV:Subcode>
                </SOAP-ENV:Code>
                <SOAP-ENV:Reason>
                    <SOAP-ENV:Text xml:lang="en">Sender not Authorized</SOAP-ENV:Text>
                </SOAP-ENV:Reason>
            </SOAP-ENV:Fault>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#;

    #[test]
    fn test_parse() {
        let env: Envelope = yaserde::de::from_str(XML).unwrap();
        assert_eq!(env.body.fault.code.value.0, "SOAP-ENV:Sender");
    }

    #[test]
    fn test_parse_fault() {
        let xml = r#"
        <SOAP-ENV:Fault xmlns:SOAP-ENV="http://www.w3.org/2003/05/soap-envelope" SOAP-ENV:encodingStyle="http://www.w3.org/2003/05/soap-encoding">
            <SOAP-ENV:Code>
                <SOAP-ENV:Value>SOAP-ENV:Sender</SOAP-ENV:Value>
                <SOAP-ENV:Subcode>
                    <SOAP-ENV:Value>ter:NotAuthorized</SOAP-ENV:Value>
                </SOAP-ENV:Subcode>
            </SOAP-ENV:Code>
            <SOAP-ENV:Reason>
                <SOAP-ENV:Text xml:lang="en">Sender not Authorized</SOAP-ENV:Text>
            </SOAP-ENV:Reason>
        </SOAP-ENV:Fault>"#;

        let fault: soapenv::Fault = yaserde::de::from_str(xml).unwrap();

        assert_eq!(fault.code.subcode.unwrap().value, "ter:NotAuthorized");
        assert_eq!(fault.reason.text[0].content, "Sender not Authorized");
    }

}