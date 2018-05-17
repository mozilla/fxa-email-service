// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{
    de::{Deserialize, Deserializer, Error as DeserializeError, Unexpected},
    ser::{Error as SerializeError, Serialize, Serializer},
};

use auth_db::{BounceSubtype as AuthDbBounceSubtype, BounceType as AuthDbBounceType};

#[cfg(test)]
mod test;

// Warning, long vehicle! This module is a direct encoding
// of the SES notification format that's documented here:
//
// https://docs.aws.amazon.com/ses/latest/DeveloperGuide/notification-contents.html

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Notification {
    #[serde(rename = "notificationType")]
    pub notification_type: NotificationType,
    pub mail: Mail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce: Option<Bounce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint: Option<Complaint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Delivery>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotificationType {
    Bounce,
    Complaint,
    Delivery,
    Null,
}

impl From<NotificationType> for String {
    fn from(notification_type: NotificationType) -> String {
        String::from(match notification_type {
            NotificationType::Bounce => "Bounce",
            NotificationType::Complaint => "Complaint",
            NotificationType::Delivery => "Delivery",
            NotificationType::Null => "",
        })
    }
}

impl Display for NotificationType {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", From::from(*self): String)
    }
}

impl Default for NotificationType {
    fn default() -> NotificationType {
        NotificationType::Null
    }
}

impl<'d> Deserialize<'d> for NotificationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        match value.as_str() {
            "Bounce" => Ok(NotificationType::Bounce),
            "Complaint" => Ok(NotificationType::Complaint),
            "Delivery" => Ok(NotificationType::Delivery),
            _ => Err(D::Error::invalid_value(
                Unexpected::Str(&value),
                &"SES notificiation type",
            )),
        }
    }
}

impl Serialize for NotificationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string: String = From::from(*self);
        if string == "" {
            Err(S::Error::custom("notification type not set"))
        } else {
            serializer.serialize_str(&string)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mail {
    timestamp: DateTime<Utc>,
    #[serde(rename = "messageId")]
    message_id: String,
    source: String,
    #[serde(rename = "sourceArn")]
    source_arn: String,
    #[serde(rename = "sourceIp")]
    source_ip: String,
    #[serde(rename = "sendingAccountId")]
    sending_account_id: String,
    destination: Vec<String>,
    #[serde(rename = "headersTruncated")]
    headers_truncated: Option<String>,
    headers: Option<Vec<Header>>,
    #[serde(rename = "commonHeaders")]
    common_headers: Option<Vec<Header>>,
}

impl Default for Mail {
    fn default() -> Mail {
        Mail {
            timestamp: Utc::now(),
            message_id: String::from(""),
            source: String::from(""),
            source_arn: String::from(""),
            source_ip: String::from(""),
            sending_account_id: String::from(""),
            destination: Vec::new(),
            headers_truncated: None,
            headers: None,
            common_headers: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Header {
    name: String,
    value: HeaderValue,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum HeaderValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bounce {
    #[serde(rename = "bounceType")]
    pub bounce_type: BounceType,
    #[serde(rename = "bounceSubType")]
    pub bounce_subtype: BounceSubtype,
    pub bounced_recipients: Vec<BouncedRecipient>,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "feedbackId")]
    pub feedback_id: String,
    #[serde(rename = "remoteMtaIp")]
    pub remote_mta_ip: Option<String>,
    #[serde(rename = "reportingMTA")]
    pub reporting_mta: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum BounceType {
    Undetermined,
    Permanent,
    Transient,
}

impl From<BounceType> for AuthDbBounceType {
    fn from(bounce_type: BounceType) -> AuthDbBounceType {
        match bounce_type {
            BounceType::Undetermined => {
                println!("Mapped SesBounceType::Undetermined to AuthDbBounceType::Soft");
                AuthDbBounceType::Soft
            }
            BounceType::Permanent => AuthDbBounceType::Hard,
            BounceType::Transient => AuthDbBounceType::Soft,
        }
    }
}

impl<'d> Deserialize<'d> for BounceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        match value.as_str() {
            "Undetermined" => Ok(BounceType::Undetermined),
            "Permanent" => Ok(BounceType::Permanent),
            "Transient" => Ok(BounceType::Transient),
            _ => {
                println!(
                    "Mapped unrecognised SES bounceType `{}` to BounceType::Undetermined",
                    value.as_str()
                );
                Ok(BounceType::Undetermined)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum BounceSubtype {
    Undetermined,
    General,
    NoEmail,
    Suppressed,
    MailboxFull,
    MessageTooLarge,
    ContentRejected,
    AttachmentRejected,
}

impl From<BounceSubtype> for AuthDbBounceSubtype {
    fn from(bounce_subtype: BounceSubtype) -> AuthDbBounceSubtype {
        match bounce_subtype {
            BounceSubtype::Undetermined => AuthDbBounceSubtype::Undetermined,
            BounceSubtype::General => AuthDbBounceSubtype::General,
            BounceSubtype::NoEmail => AuthDbBounceSubtype::NoEmail,
            BounceSubtype::Suppressed => AuthDbBounceSubtype::Suppressed,
            BounceSubtype::MailboxFull => AuthDbBounceSubtype::MailboxFull,
            BounceSubtype::MessageTooLarge => AuthDbBounceSubtype::MessageTooLarge,
            BounceSubtype::ContentRejected => AuthDbBounceSubtype::ContentRejected,
            BounceSubtype::AttachmentRejected => AuthDbBounceSubtype::AttachmentRejected,
        }
    }
}

impl<'d> Deserialize<'d> for BounceSubtype {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        match value.as_str() {
            "Undetermined" => Ok(BounceSubtype::Undetermined),
            "General" => Ok(BounceSubtype::General),
            "NoEmail" => Ok(BounceSubtype::NoEmail),
            "Suppressed" => Ok(BounceSubtype::Suppressed),
            "MailboxFull" => Ok(BounceSubtype::MailboxFull),
            "MessageTooLarge" => Ok(BounceSubtype::MessageTooLarge),
            "ContentRejected" => Ok(BounceSubtype::ContentRejected),
            "AttachmentRejected" => Ok(BounceSubtype::AttachmentRejected),
            _ => {
                println!(
                    "Mapped unrecognised SES bounceSubType `{}` to BounceSubtype::Undetermined",
                    value.as_str()
                );
                Ok(BounceSubtype::Undetermined)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BouncedRecipient {
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    pub action: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "diagnosticCode")]
    pub diagnostic_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Complaint {
    #[serde(rename = "complainedRecipients")]
    pub complained_recipients: Vec<ComplainedRecipient>,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "feedbackId")]
    pub feedback_id: String,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(rename = "complaintFeedbackType")]
    pub complaint_feedback_type: Option<ComplaintFeedbackType>,
    #[serde(rename = "arrivalDate")]
    pub arrival_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComplainedRecipient {
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ComplaintFeedbackType {
    Abuse,
    AuthFailure,
    Fraud,
    NotSpam,
    Other,
    Virus,
}

impl From<ComplaintFeedbackType> for AuthDbBounceSubtype {
    fn from(complaint_feedback_type: ComplaintFeedbackType) -> AuthDbBounceSubtype {
        match complaint_feedback_type {
            ComplaintFeedbackType::Abuse => AuthDbBounceSubtype::Abuse,
            ComplaintFeedbackType::AuthFailure => AuthDbBounceSubtype::AuthFailure,
            ComplaintFeedbackType::Fraud => AuthDbBounceSubtype::Fraud,
            ComplaintFeedbackType::NotSpam => AuthDbBounceSubtype::NotSpam,
            ComplaintFeedbackType::Other => AuthDbBounceSubtype::Other,
            ComplaintFeedbackType::Virus => AuthDbBounceSubtype::Virus,
        }
    }
}

impl<'d> Deserialize<'d> for ComplaintFeedbackType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        match value.as_str() {
            "abuse" => Ok(ComplaintFeedbackType::Abuse),
            "auth-failure" => Ok(ComplaintFeedbackType::AuthFailure),
            "fraud" => Ok(ComplaintFeedbackType::Fraud),
            "not-spam" => Ok(ComplaintFeedbackType::NotSpam),
            "other" => Ok(ComplaintFeedbackType::Other),
            "virus" => Ok(ComplaintFeedbackType::Virus),
            _ => {
                println!("Mapped unrecognised SES complaintFeedbackType `{}` to ComplaintFeedbackType::Other", value.as_str());
                Ok(ComplaintFeedbackType::Other)
            }
        }
    }
}

impl Serialize for ComplaintFeedbackType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            ComplaintFeedbackType::Abuse => "abuse",
            ComplaintFeedbackType::AuthFailure => "auth-failure",
            ComplaintFeedbackType::Fraud => "fraud",
            ComplaintFeedbackType::NotSpam => "not-spam",
            ComplaintFeedbackType::Other => "other",
            ComplaintFeedbackType::Virus => "virus",
        };
        serializer.serialize_str(value)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Delivery {
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "processingTimeMillis")]
    pub processing_time_millis: u32,
    pub recipients: Vec<String>,
    #[serde(rename = "smtpResponse")]
    pub smtp_response: String,
    #[serde(rename = "remoteMtaIp")]
    pub remote_mta_ip: Option<String>,
    #[serde(rename = "reportingMTA")]
    pub reporting_mta: Option<String>,
}
