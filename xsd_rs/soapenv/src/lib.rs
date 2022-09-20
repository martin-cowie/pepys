#![allow(clippy::derive_partial_eq_without_eq)]

// External dependencies
use yaserde_derive::{YaDeserialize, YaSerialize};
use xsd_macro_utils::*;

// Local dependencies
use validate::Validate;

// Runtime dependencies
use std::str::FromStr;

// Generated from soapenv.xsd hereon ---------------------------------


//use http://www.w3.org/2001/xml.xsd  http://www.w3.org/XML/1998/namespace;
// pub type Envelope = Envelope;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Envelope {
    #[yaserde(prefix = "tns", rename = "Header")]
    pub header: Option<Header>,

    #[yaserde(prefix = "tns", rename = "Body")]
    pub body: Body,
}

impl Validate for Envelope {}


// pub type Header = Header;
// Elements replacing the wildcard MUST be namespace qualified, but can be in
// the targetNamespace
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Header {}

impl Validate for Header {}


// pub type Body = Body;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Body {}

impl Validate for Body {}


// pub type MustUnderstand = bool;
// pub type Relay = bool;
// pub type Role = String;
// pub type EncodingStyle = String;
// pub type Fault = Fault;
// Fault reporting structure
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Fault {
    #[yaserde(prefix = "tns", rename = "Code")]
    pub code: Faultcode,

    #[yaserde(prefix = "tns", rename = "Reason")]
    pub reason: Faultreason,

    #[yaserde(prefix = "tns", rename = "Node")]
    pub node: Option<String>,

    #[yaserde(prefix = "tns", rename = "Role")]
    pub role: Option<String>,

    #[yaserde(prefix = "tns", rename = "Detail")]
    pub detail: Option<Detail>,
}

impl Validate for Fault {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Faultreason {
    #[yaserde(prefix = "tns", rename = "Text")]
    pub text: Vec<Reasontext>,
}

impl Validate for Faultreason {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Reasontext {
    #[yaserde(attribute, prefix = "xml", rename = "lang")]
    // pub lang: xml::Lang,
    pub lang: String, //NB: amended by-hand to solve conflict with xml crate
}

impl Validate for Reasontext {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Faultcode {
    #[yaserde(prefix = "tns", rename = "Value")]
    pub value: FaultcodeEnum,

    #[yaserde(prefix = "tns", rename = "Subcode")]
    pub subcode: Option<Subcode>,
}

impl Validate for Faultcode {}


#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FaultcodeEnum (pub String);

impl Validate for FaultcodeEnum {}
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Subcode {
    #[yaserde(prefix = "tns", rename = "Value")]
    pub value: String,

    #[yaserde(prefix = "tns", rename = "Subcode")]
    pub subcode: Vec<Subcode>,
}

impl Validate for Subcode {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct Detail {}

impl Validate for Detail {}


// pub type NotUnderstood = NotUnderstoodType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct NotUnderstoodType {
    #[yaserde(attribute, rename = "qname")]
    pub qname: String,
}

impl Validate for NotUnderstoodType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct SupportedEnvType {
    #[yaserde(attribute, rename = "qname")]
    pub qname: String,
}

impl Validate for SupportedEnvType {}


// pub type Upgrade = UpgradeType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://www.w3.org/2003/05/soap-envelope")]
pub struct UpgradeType {
    #[yaserde(prefix = "tns", rename = "SupportedEnvelope")]
    pub supported_envelope: Vec<SupportedEnvType>,
}

impl Validate for UpgradeType {}
