use hyper::header::CONTENT_TYPE;
use soapenv::{Fault, Faultcode};
use yaserde_derive::*;

#[derive(Debug)]
pub struct FaultDetail {
    pub faultcode: String,
    pub subcode: String,
    pub reason: String
}

impl FaultDetail {
    pub fn new(faultcode: &str, subcode: &str, reason: &str) -> Self {
        Self {
            faultcode: faultcode.to_string(),
            subcode: subcode.to_string(),
            reason: reason.to_string()
        }
    }
}


/// From https://www.onvif.org/specs/core/ONVIF-Core-Specification-v261.pdf Table 5
#[derive(Debug)]
pub enum SoapFaultCode {
    VersionMismatch,
    MustUnderstand,
    DataEncodingUnknown,

    WellFormed,
    TagMismatch,
    Tag,
    Namespace,
    MissingAttr,
    ProhibAttr,
    InvalidArgs,
    InvalidArgVal,
    UnknownAction,
    OperationProhibited,
    NotAuthorized,

    ActionNotSupported,
    Action,
    OutofMemory,
    CriticalError
}

const SENDER: &str = "env:Sender";
const RECEIVER: &str = "env:Receiver";

impl From<SoapFaultCode> for FaultDetail {
    fn from(fault: SoapFaultCode) -> FaultDetail {

        match fault {
            SoapFaultCode::VersionMismatch => FaultDetail::new("", "env:VersionMismatch", "SOAP version mismatch"),
            SoapFaultCode::MustUnderstand => FaultDetail::new("", "env:MustUnderstand", "SOAP header blocks not understood"),
            SoapFaultCode::DataEncodingUnknown => FaultDetail::new("", "env:DataEncodingUnknown", "Unsupported SOAP data encoding"),

            SoapFaultCode::WellFormed => FaultDetail::new(SENDER, "ter:WellFormed", "Well-formed Error"),
            SoapFaultCode::TagMismatch => FaultDetail::new(SENDER, "ter:TagMismatch", "Tag Mismatch"),
            SoapFaultCode::Tag => FaultDetail::new(SENDER, "ter:Tag", "No Tag"),
            SoapFaultCode::Namespace => FaultDetail::new(SENDER, "ter:Namespace", "Namespace Error"),
            SoapFaultCode::MissingAttr => FaultDetail::new(SENDER, "ter:MissingAttr", "Required Attribute not present"),
            SoapFaultCode::ProhibAttr => FaultDetail::new(SENDER, "ter:ProhibAttr", "Prohibited Attribute"),
            SoapFaultCode::InvalidArgs => FaultDetail::new(SENDER, "ter:InvalidArgs", "Invalid Args"),
            SoapFaultCode::InvalidArgVal => FaultDetail::new(SENDER, "ter:InvalidArgVal", "Argument Value Invalid"),
            SoapFaultCode::UnknownAction => FaultDetail::new(SENDER, "ter:UnknownAction", "Unknown Action"),
            SoapFaultCode::OperationProhibited => FaultDetail::new(SENDER, "ter:OperationProhibited", "Operation not Permitted"),
            SoapFaultCode::NotAuthorized => FaultDetail::new(SENDER, "ter:NotAuthorized", "Sender not Authorized"),

            SoapFaultCode::ActionNotSupported => FaultDetail::new(RECEIVER, "ter:ActionNotSupported", "Optional Action Not Implemented"),
            SoapFaultCode::Action => FaultDetail::new(RECEIVER, "ter:Action", "Action Failed"),
            SoapFaultCode::OutofMemory => FaultDetail::new(RECEIVER, "ter:OutofMemory", "Out of Memory"),
            SoapFaultCode::CriticalError => FaultDetail::new(RECEIVER, "ter:CriticalError", "Critical Error")
        }
    }

}

impl From<FaultDetail> for Envelope {
    fn from(detail: FaultDetail) -> Self {
        Envelope {
            body: Body {
                fault: Fault {
                    code: Faultcode {
                        value: soapenv::FaultcodeEnum(detail.faultcode),
                        subcode: Some(soapenv::Subcode {
                            value: detail.subcode,
                        }),
                    },
                    reason: soapenv::Faultreason {
                        text: vec![soapenv::Reasontext {
                            lang: "en".to_string(),
                            content: detail.reason
                        }]
                    },
                    node: None,
                    role: None,
                    detail: None,
                    encoding_style: "http://www.w3.org/2003/05/soap-encoding".to_string(),
                }
            }
        }
    }
}

impl From<SoapFaultCode> for hyper::Response<hyper::Body> {
    fn from(ter: SoapFaultCode) -> Self {
        let detail: FaultDetail = ter.into();
        let envelope: Envelope = detail.into();
        let xml = yaserde::ser::to_string(&envelope).unwrap();

        hyper::Response::builder()
            .header(CONTENT_TYPE, "application/soap+xml; charset=utf-8")
            .status(hyper::StatusCode::BAD_REQUEST)
            .body(hyper::Body::from(xml))
            .unwrap_or_default()
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soap",
    namespace = "soap: http://www.w3.org/2003/05/soap-envelope",
    namespace = "ter: http://www.onvif.org/ver10/error",
    namespace = "env: http://www.w3.org/2003/05/soap-envelope"
)]
struct Envelope {
    #[yaserde(prefix = "soap", rename = "Body")]
    pub body: Body
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soap")]
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