#![allow(clippy::derive_partial_eq_without_eq)]

// External dependencies
use yaserde_derive::{YaDeserialize, YaSerialize};
// use xsd_macro_utils::*;
use xsd_types::types as xs;

// Local dependencies
use validate::Validate;
use b2 as wsnt;
use ws_addr as wsa;
use t1 as wstop;

pub mod request;
pub mod response;


// Generated from event.xsd hereon ---------------------------------


//use http://www.w3.org/2005/08/addressing/ws-addr.xsd  http://www.w3.org/2005/08/addressing;
//use http://docs.oasis-open.org/wsn/t-1.xsd  http://docs.oasis-open.org/wsn/t-1;
//use http://docs.oasis-open.org/wsn/b-2.xsd  http://docs.oasis-open.org/wsn/b-2;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the event service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "tev", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct Capabilities {
    // Indicates that the WS Subscription policy is supported.
    #[yaserde(attribute, rename = "WSSubscriptionPolicySupport")]
    pub ws_subscription_policy_support: Option<bool>,

    // Indicates that the WS Pull Point is supported.
    #[yaserde(attribute, rename = "WSPullPointSupport")]
    pub ws_pull_point_support: Option<bool>,

    // Indicates that the WS Pausable Subscription Manager Interface is
    // supported.
    #[yaserde(attribute, rename = "WSPausableSubscriptionManagerInterfaceSupport")]
    pub ws_pausable_subscription_manager_interface_support: Option<bool>,

    // Maximum number of supported notification producers as defined by
    // WS-BaseNotification.
    #[yaserde(attribute, rename = "MaxNotificationProducers")]
    pub max_notification_producers: Option<i32>,

    // Maximum supported number of notification pull points.
    #[yaserde(attribute, rename = "MaxPullPoints")]
    pub max_pull_points: Option<i32>,

    // Indication if the device supports persistent notification storage.
    #[yaserde(attribute, rename = "PersistentNotificationStorage")]
    pub persistent_notification_storage: Option<bool>,

    // A space separated list of supported event broker protocols as defined by
    // the tev:EventBrokerProtocol datatype.
    #[yaserde(attribute, rename = "EventBrokerProtocols")]
    pub event_broker_protocols: Option<String>,

    // Maxiumum number of event broker configurations that can be added to the
    // device.
    #[yaserde(attribute, rename = "MaxEventBrokers")]
    pub max_event_brokers: Option<i32>,

    // Indicates that metadata streaming over MQTT is supported
    #[yaserde(attribute, rename = "MetadataOverMQTT")]
    pub metadata_over_mqtt: Option<bool>,
}

impl Validate for Capabilities {}


// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct CreatePullPointSubscription {
    // Optional XPATH expression to select specific topics.
    #[yaserde(prefix = "tev", rename = "Filter")]
    pub filter: Option<wsnt::FilterType>,

    // Initial termination time.
    #[yaserde(prefix = "tev", rename = "InitialTerminationTime")]
    pub initial_termination_time: Option<wsnt::AbsoluteOrRelativeTimeType>,

    // Refer to
    #[yaserde(prefix = "tev", rename = "SubscriptionPolicy")]
    pub subscription_policy: Option<create_pull_point_subscription::SubscriptionPolicyType>,
}

impl Validate for CreatePullPointSubscription {}

pub mod create_pull_point_subscription {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
    pub struct SubscriptionPolicyType {}

    impl Validate for SubscriptionPolicyType {}

}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct CreatePullPointSubscriptionResponse {
    // Endpoint reference of the subscription to be used for pulling the
    // messages.
    #[yaserde(prefix = "tev", rename = "SubscriptionReference")]
    pub subscription_reference: wsa::EndpointReferenceType,

    // Current time of the server for synchronization purposes.
    #[yaserde(prefix = "wsnt", rename = "CurrentTime")]
    pub current_time: wsnt::CurrentTime,

    // Date time when the PullPoint will be shut down without further pull
    // requests.
    #[yaserde(prefix = "wsnt", rename = "TerminationTime")]
    pub termination_time: wsnt::TerminationTime,
}

impl Validate for CreatePullPointSubscriptionResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct PullMessages {
    // Maximum time to block until this method returns.
    #[yaserde(prefix = "tev", rename = "Timeout")]
    pub timeout: xs::Duration,

    // Upper limit for the number of messages to return at once. A server
    // implementation may decide to return less messages.
    #[yaserde(prefix = "tev", rename = "MessageLimit")]
    pub message_limit: i32,
}

impl Validate for PullMessages {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct PullMessagesResponse {
    // The date and time when the messages have been delivered by the web server
    // to the client.
    #[yaserde(prefix = "tev", rename = "CurrentTime")]
    pub current_time: xs::DateTime,

    // Date time when the PullPoint will be shut down without further pull
    // requests.
    #[yaserde(prefix = "tev", rename = "TerminationTime")]
    pub termination_time: xs::DateTime,

    // List of messages. This list shall be empty in case of a timeout.
    #[yaserde(prefix = "wsnt", rename = "NotificationMessage")]
    pub notification_message: Vec<wsnt::NotificationMessage>,
}

impl Validate for PullMessagesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct PullMessagesFaultResponse {
    // Maximum timeout supported by the device.
    #[yaserde(prefix = "tev", rename = "MaxTimeout")]
    pub max_timeout: xs::Duration,

    // Maximum message limit supported by the device.
    #[yaserde(prefix = "tev", rename = "MaxMessageLimit")]
    pub max_message_limit: i32,
}

impl Validate for PullMessagesFaultResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct Seek {
    // The date and time to match against stored messages.
    #[yaserde(prefix = "tev", rename = "UtcTime")]
    pub utc_time: xs::DateTime,

    // Reverse the pull direction of PullMessages.
    #[yaserde(prefix = "tev", rename = "Reverse")]
    pub reverse: bool,
}

impl Validate for Seek {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct SeekResponse {}

impl Validate for SeekResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct SetSynchronizationPoint {}

impl Validate for SetSynchronizationPoint {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct SetSynchronizationPointResponse {}

impl Validate for SetSynchronizationPointResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct GetEventProperties {}

impl Validate for GetEventProperties {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct GetEventPropertiesResponse {
    // List of topic namespaces supported.
    #[yaserde(prefix = "tev", rename = "TopicNamespaceLocation")]
    pub topic_namespace_location: Vec<String>,

    // True when topicset is fixed for all times.
    #[yaserde(prefix = "wsnt", rename = "FixedTopicSet")]
    pub fixed_topic_set: wsnt::FixedTopicSet,

    // Set of topics supported.
    #[yaserde(prefix = "wstop", rename = "TopicSet")]
    pub topic_set: wstop::TopicSet,

    // Defines the XPath expression syntax supported for matching topic
    // expressions.
    #[yaserde(prefix = "wsnt", rename = "TopicExpressionDialect")]
    pub topic_expression_dialect: Vec<wsnt::TopicExpressionDialect>,

    // Defines the XPath function set supported for message content filtering.
    #[yaserde(prefix = "tev", rename = "MessageContentFilterDialect")]
    pub message_content_filter_dialect: Vec<String>,

    // Optional ProducerPropertiesDialects. Refer to
    #[yaserde(prefix = "tev", rename = "ProducerPropertiesFilterDialect")]
    pub producer_properties_filter_dialect: Vec<String>,

    // The Message Content Description Language allows referencing
    // of vendor-specific types. In order to ease the integration of such types
    // into a client application,
    // the GetEventPropertiesResponse shall list all URI locations to schema
    // files whose types are
    // used in the description of notifications, with
    // MessageContentSchemaLocation elements.
    #[yaserde(prefix = "tev", rename = "MessageContentSchemaLocation")]
    pub message_content_schema_location: Vec<String>,
}

impl Validate for GetEventPropertiesResponse {}


// When this element is included in the SubscriptionPolicy, the pullpoint shall
// not provide Initialized or Deleted events for Properties.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct ChangedOnly {}

impl Validate for ChangedOnly {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct AddEventBroker {
    #[yaserde(prefix = "tev", rename = "EventBroker")]
    pub event_broker: EventBrokerConfig,
}

impl Validate for AddEventBroker {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct AddEventBrokerResponse {}

impl Validate for AddEventBrokerResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct DeleteEventBroker {
    #[yaserde(prefix = "tev", rename = "Address")]
    pub address: String,
}

impl Validate for DeleteEventBroker {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct DeleteEventBrokerResponse {}

impl Validate for DeleteEventBrokerResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct GetEventBrokers {
    #[yaserde(prefix = "tev", rename = "Address")]
    pub address: String,
}

impl Validate for GetEventBrokers {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct GetEventBrokersResponse {
    #[yaserde(prefix = "tev", rename = "EventBroker")]
    pub event_broker: Vec<EventBrokerConfig>,
}

impl Validate for GetEventBrokersResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]
pub struct EventBrokerConfig {
    // Event broker address in the format "scheme://host:port[/resource]". The
    // supported schemes shall be returned by the EventBrokerProtocols
    // capability. The resource part of the URL is only valid when using
    // websocket. The Address must be unique.
    #[yaserde(prefix = "tev", rename = "Address")]
    pub address: String,

    // Prefix that will be prepended to all topics before they are published.
    // This is used to make published topics unique for each device. TopicPrefix
    // is not allowed to be empty.
    #[yaserde(prefix = "tev", rename = "TopicPrefix")]
    pub topic_prefix: String,

    // User name for the event broker.
    #[yaserde(prefix = "tev", rename = "UserName")]
    pub user_name: String,

    // Password for the event broker. Password shall not be included when
    // returned with GetEventBrokers.
    #[yaserde(prefix = "tev", rename = "Password")]
    pub password: String,

    // Optional certificate ID in the key store pointing to a client certificate
    // to be used for authenticating the device at the message broker.
    #[yaserde(prefix = "tev", rename = "CertificateID")]
    pub certificate_id: String,

    // Concrete Topic Expression to select specific event topics to publish.
    #[yaserde(prefix = "tev", rename = "PublishFilter")]
    pub publish_filter: wsnt::FilterType,

    // Quality of service level to use when publishing. This defines the
    // guarantee of delivery for a specific message: 0 = At most once, 1 = At
    // least once, 2 = Exactly once.
    #[yaserde(prefix = "tev", rename = "QoS")]
    pub qo_s: i32,

    // Current connection status (see tev:ConnectionStatus for possible values).
    #[yaserde(prefix = "tev", rename = "Status")]
    pub status: String,

    // The ID of the certification path validation policy used to validate the
    // broker certificate. In case encryption is used but no validation policy
    // is specified, the device shall not validate the broker certificate.
    #[yaserde(prefix = "tev", rename = "CertPathValidationPolicyID")]
    pub cert_path_validation_policy_id: Option<String>,

    // Concrete Topic Expression to select specific metadata topics to publish.
    #[yaserde(prefix = "tev", rename = "MetadataFilter")]
    pub metadata_filter: wsnt::FilterType,
}

impl Validate for EventBrokerConfig {}


#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]

pub enum EventBrokerProtocol {
    #[yaserde(rename = "mqtt")]
    Mqtt,
    #[yaserde(rename = "mqtts")]
    Mqtts,
    #[yaserde(rename = "ws")]
    Ws,
    #[yaserde(rename = "wss")]
    Wss,
    __Unknown__(String),
}

impl Default for EventBrokerProtocol {
    fn default() -> EventBrokerProtocol {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for EventBrokerProtocol {}



#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]#[yaserde(prefix = "tev", namespace = "tev: http://www.onvif.org/ver10/events/wsdl")]

pub enum ConnectionStatus {
    Offline,
    Connecting,
    Connected,
    __Unknown__(String),
}

impl Default for ConnectionStatus {
    fn default() -> ConnectionStatus {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ConnectionStatus {}
