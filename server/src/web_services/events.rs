use soap_fault::SoapFaultCode as Ter;
use event::{request, response};
use super::ExampleData;
use ws_addr as wsa;
use super::Authenticator;

pub struct EventsService {
    authenticator: &'static Authenticator
}

impl EventsService {
    pub fn new(authenticator: &'static Authenticator) -> Self {
        Self {
            authenticator
        }
    }

    pub fn process_request(&self, payload: impl std::io::Read) -> Result<String, Ter> {
        let request: request::Envelope = yaserde::de::from_reader(payload)
            .map_err(|_| Ter::WellFormed)?;

        // Check username/password
        self.authenticator.authenticate(&request.header)?;

        let response = match request.body {

            request::Body::GetEventProperties(_) => self.get_event_properties(&request)?,
            request::Body::CreatePullPointSubscription(_) => self.create_pull_point_subscription(&request)?,
            request::Body::PullMessages(_) => self.pull_messages(&request)?,

            _ => {
                return Err(Ter::ActionNotSupported);
            }

        };

        let result = yaserde::ser::to_string(&response)
            .map_err(|_| Ter::Action)?;
        Ok(result)

    }

    pub fn get_event_properties(&self, _req: &request::Envelope) -> Result<event::response::Envelope, Ter> {
        Ok(response::Envelope{
            body: response::Body::GetEventPropertiesResponse(
                event::GetEventPropertiesResponse::example()
            )
        })
    }

    pub fn create_pull_point_subscription(&self, _req: &request::Envelope) -> Result<event::response::Envelope, Ter> {
        Ok(response::Envelope {
            body: response::Body::CreatePullPointSubscriptionResponse(
                event::CreatePullPointSubscriptionResponse::example()
            )
        })
    }

    pub fn pull_messages(&self, _req: &request::Envelope) -> Result<event::response::Envelope, Ter> {
        Ok(response::Envelope {
            body: response::Body::PullMessagesResponse(
                event::PullMessagesResponse::example()
            )
        })
    }

}

use xsd_types::types::DateTime;

impl ExampleData<event::CreatePullPointSubscriptionResponse> for event::CreatePullPointSubscriptionResponse {
    fn example() -> event::CreatePullPointSubscriptionResponse {
        event::CreatePullPointSubscriptionResponse {
            subscription_reference: wsa::EndpointReferenceType {
                address: wsa::AttributedURIType {},
                reference_parameters: None,
                metadata: None,
            },
            current_time: DateTime::default(), //TODO: something meaningful inside these values
            termination_time: DateTime::default(),
        }
    }
}

impl ExampleData<event::GetEventPropertiesResponse> for event::GetEventPropertiesResponse {
    fn example() -> event::GetEventPropertiesResponse {
        event::GetEventPropertiesResponse {
            topic_namespace_location: vec![
                "http://www.onvif.org/onvif/ver10/topics/topicns.xml".to_string(),
            ],
            fixed_topic_set: b2::FixedTopicSet {},
            topic_set: t1::TopicSetType {
                documentation: None,
            },
            topic_expression_dialect: vec![
                b2::TopicExpressionDialect {},
                b2::TopicExpressionDialect {},
            ],
            message_content_filter_dialect: vec![
                "http://www.onvif.org/ver10/tev/messageContentFilter/ItemFilter".to_string(),
            ],
            producer_properties_filter_dialect: vec![],
            message_content_schema_location: vec![
                "http://www.onvif.org/onvif/ver10/schema/onvif.xsd".to_string(),
            ],
        }
    }
}

impl ExampleData<event::PullMessagesResponse> for event::PullMessagesResponse {
    fn example() -> event::PullMessagesResponse {
        event::PullMessagesResponse {
            current_time: DateTime {
                value: chrono::DateTime::default(), //TODO: replace with something meaningful
            },
            termination_time: DateTime {
                value: chrono::DateTime::default(), //TODO: replace with something meaningful
            },
            notification_message: vec![]
        }
    }
}