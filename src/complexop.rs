#[cfg(feature = "option")]
use crate::enumtypes::*;

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AlternativeSecurityId {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<i32>,
    #[serde(rename = "identityProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<u8>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainState {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "lastActionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_action_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServicePlanInfo {
    #[serde(rename = "servicePlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_plan_id: Option<String>,
    #[serde(rename = "servicePlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_plan_name: Option<String>,
    #[serde(rename = "provisioningStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<String>,
    #[serde(rename = "appliesTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AssignedLicense {
    #[serde(rename = "disabledPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_plans: Option<Vec<String>>,
    #[serde(rename = "skuId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LicenseProcessingState {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnPremisesProvisioningError {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "propertyCausingError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_causing_error: Option<String>,
    #[serde(rename = "occurredDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LicenseUnitsDetail {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<i32>,
    #[serde(rename = "suspended")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<i32>,
    #[serde(rename = "warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AssignedPlan {
    #[serde(rename = "assignedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_date_time: Option<String>,
    #[serde(rename = "capabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_status: Option<String>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "servicePlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_plan_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PrivacyProfile {
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(rename = "statementUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedPlan {
    #[serde(rename = "capabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_status: Option<String>,
    #[serde(rename = "provisioningStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<String>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VerifiedDomain {
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "isInitial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_initial: Option<bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LicenseAssignmentState {
    #[serde(rename = "skuId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "disabledPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_plans: Option<Vec<String>>,
    #[serde(rename = "assignedByGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_by_group: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnPremisesExtensionAttributes {
    #[serde(rename = "extensionAttribute1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute1: Option<String>,
    #[serde(rename = "extensionAttribute2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute2: Option<String>,
    #[serde(rename = "extensionAttribute3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute3: Option<String>,
    #[serde(rename = "extensionAttribute4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute4: Option<String>,
    #[serde(rename = "extensionAttribute5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute5: Option<String>,
    #[serde(rename = "extensionAttribute6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute6: Option<String>,
    #[serde(rename = "extensionAttribute7")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute7: Option<String>,
    #[serde(rename = "extensionAttribute8")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute8: Option<String>,
    #[serde(rename = "extensionAttribute9")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute9: Option<String>,
    #[serde(rename = "extensionAttribute10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute10: Option<String>,
    #[serde(rename = "extensionAttribute11")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute11: Option<String>,
    #[serde(rename = "extensionAttribute12")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute12: Option<String>,
    #[serde(rename = "extensionAttribute13")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute13: Option<String>,
    #[serde(rename = "extensionAttribute14")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute14: Option<String>,
    #[serde(rename = "extensionAttribute15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_attribute15: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PasswordProfile {
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "forceChangePasswordNextSignIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_change_password_next_sign_in: Option<bool>,
    #[serde(rename = "forceChangePasswordNextSignInWithMfa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_change_password_next_sign_in_with_mfa: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MailboxSettings {
    #[serde(rename = "automaticRepliesSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_replies_setting: Option<AutomaticRepliesSetting>,
    #[serde(rename = "archiveFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_folder: Option<String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LocaleInfo>,
    #[serde(rename = "workingHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_hours: Option<WorkingHours>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AutomaticRepliesSetting {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AutomaticRepliesStatus>,
    #[serde(rename = "externalAudience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_audience: Option<ExternalAudienceScope>,
    #[serde(rename = "scheduledStartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_date_time: Option<String>,
    #[serde(rename = "scheduledEndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_date_time: Option<String>,
    #[serde(rename = "internalReplyMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reply_message: Option<String>,
    #[serde(rename = "externalReplyMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply_message: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DateTimeTimeZone {
    #[serde(rename = "dateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LocaleInfo {
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkingHours {
    #[serde(rename = "daysOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<DayOfWeek>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<TimeZoneBase>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneBase {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SettingValue {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SettingTemplateValue {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ComplexExtensionValue(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl ComplexExtensionValue {
    pub fn new(value: Option<serde_json::Value>) -> ComplexExtensionValue {
        ComplexExtensionValue(value)
    }

    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ExtensionSchemaProperty {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CustomTimeZone {
    #[serde(rename = "bias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bias: Option<i32>,
    #[serde(rename = "standardOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_offset: Option<StandardTimeZoneOffset>,
    #[serde(rename = "daylightOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daylight_offset: Option<DaylightTimeZoneOffset>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StandardTimeZoneOffset {
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "dayOccurrence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_occurrence: Option<i32>,
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DaylightTimeZoneOffset {
    #[serde(rename = "daylightBias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daylight_bias: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Recipient {
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<EmailAddress>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AttendeeBase {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<AttendeeType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "locationEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_email_address: Option<String>,
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PhysicalAddress>,
    #[serde(rename = "coordinates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<OutlookGeoCoordinates>,
    #[serde(rename = "locationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "locationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<LocationType>,
    #[serde(rename = "uniqueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(rename = "uniqueIdType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id_type: Option<LocationUniqueIdType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PhysicalAddress {
    #[serde(rename = "street")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "countryOrRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutlookGeoCoordinates {
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<i64>,
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<i64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<i64>,
    #[serde(rename = "accuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<i64>,
    #[serde(rename = "altitudeAccuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_accuracy: Option<i64>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "eventStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<String>,
    #[serde(rename = "eventEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_end_time: Option<String>,
    #[serde(rename = "changeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_key: Option<String>,
    #[serde(rename = "eventSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subject: Option<String>,
    #[serde(rename = "eventLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_location: Option<Location>,
    #[serde(rename = "eventWebLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_web_link: Option<String>,
    #[serde(rename = "reminderFireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_fire_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MailTips {
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<EmailAddress>,
    #[serde(rename = "automaticReplies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_replies: Option<AutomaticRepliesMailTips>,
    #[serde(rename = "mailboxFull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_full: Option<bool>,
    #[serde(rename = "customMailTip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mail_tip: Option<String>,
    #[serde(rename = "externalMemberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_member_count: Option<i32>,
    #[serde(rename = "totalMemberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_member_count: Option<i32>,
    #[serde(rename = "deliveryRestricted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_restricted: Option<bool>,
    #[serde(rename = "isModerated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moderated: Option<bool>,
    #[serde(rename = "recipientScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_scope: Option<RecipientScopeType>,
    #[serde(rename = "recipientSuggestions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_suggestions: Option<Vec<Recipient>>,
    #[serde(rename = "maxMessageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_message_size: Option<i32>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MailTipsError>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AutomaticRepliesMailTips {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "messageLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_language: Option<LocaleInfo>,
    #[serde(rename = "scheduledStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<String>,
    #[serde(rename = "scheduledEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MailTipsError {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneInformation {
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InternetMessageHeader {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemBody {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<BodyType>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FollowupFlag {
    #[serde(rename = "completedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_date_time: Option<String>,
    #[serde(rename = "dueDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date_time: Option<String>,
    #[serde(rename = "startDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "flagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_status: Option<FollowupFlagStatus>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScheduleInformation {
    #[serde(rename = "scheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(rename = "scheduleItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_items: Option<Vec<ScheduleItem>>,
    #[serde(rename = "availabilityView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_view: Option<String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<FreeBusyError>,
    #[serde(rename = "workingHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_hours: Option<WorkingHours>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScheduleItem {
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FreeBusyStatus>,
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FreeBusyError {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseStatus {
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseType>,
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PatternedRecurrence {
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<RecurrencePattern>,
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RecurrenceRange>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RecurrencePattern {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<RecurrencePatternType>,
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    #[serde(rename = "dayOfMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i32>,
    #[serde(rename = "daysOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<DayOfWeek>>,
    #[serde(rename = "firstDayOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_day_of_week: Option<DayOfWeek>,
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<WeekIndex>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RecurrenceRange {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<RecurrenceRangeType>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "recurrenceTimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_time_zone: Option<String>,
    #[serde(rename = "numberOfOccurrences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_occurrences: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Attendee {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponseStatus>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageRulePredicates {
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "subjectContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_contains: Option<Vec<String>>,
    #[serde(rename = "bodyContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_contains: Option<Vec<String>>,
    #[serde(rename = "bodyOrSubjectContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_or_subject_contains: Option<Vec<String>>,
    #[serde(rename = "senderContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_contains: Option<Vec<String>>,
    #[serde(rename = "recipientContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_contains: Option<Vec<String>>,
    #[serde(rename = "headerContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_contains: Option<Vec<String>>,
    #[serde(rename = "messageActionFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_action_flag: Option<MessageActionFlag>,
    #[serde(rename = "importance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<Importance>,
    #[serde(rename = "sensitivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<Sensitivity>,
    #[serde(rename = "fromAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_addresses: Option<Vec<Recipient>>,
    #[serde(rename = "sentToAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_to_addresses: Option<Vec<Recipient>>,
    #[serde(rename = "sentToMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_to_me: Option<bool>,
    #[serde(rename = "sentOnlyToMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_only_to_me: Option<bool>,
    #[serde(rename = "sentCcMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_cc_me: Option<bool>,
    #[serde(rename = "sentToOrCcMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_to_or_cc_me: Option<bool>,
    #[serde(rename = "notSentToMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_sent_to_me: Option<bool>,
    #[serde(rename = "hasAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(rename = "isApprovalRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_approval_request: Option<bool>,
    #[serde(rename = "isAutomaticForward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    #[serde(rename = "isAutomaticReply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_reply: Option<bool>,
    #[serde(rename = "isEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(rename = "isMeetingRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_meeting_request: Option<bool>,
    #[serde(rename = "isMeetingResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_meeting_response: Option<bool>,
    #[serde(rename = "isNonDeliveryReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_non_delivery_report: Option<bool>,
    #[serde(rename = "isPermissionControlled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_permission_controlled: Option<bool>,
    #[serde(rename = "isReadReceipt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_receipt: Option<bool>,
    #[serde(rename = "isSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_signed: Option<bool>,
    #[serde(rename = "isVoicemail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_voicemail: Option<bool>,
    #[serde(rename = "withinSizeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub within_size_range: Option<SizeRange>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SizeRange {
    #[serde(rename = "minimumSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_size: Option<i32>,
    #[serde(rename = "maximumSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_size: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageRuleActions {
    #[serde(rename = "moveToFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_folder: Option<String>,
    #[serde(rename = "copyToFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_to_folder: Option<String>,
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(rename = "permanentDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_delete: Option<bool>,
    #[serde(rename = "markAsRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_as_read: Option<bool>,
    #[serde(rename = "markImportance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_importance: Option<Importance>,
    #[serde(rename = "forwardTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_to: Option<Vec<Recipient>>,
    #[serde(rename = "forwardAsAttachmentTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_as_attachment_to: Option<Vec<Recipient>>,
    #[serde(rename = "redirectTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<Vec<Recipient>>,
    #[serde(rename = "assignCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_categories: Option<Vec<String>>,
    #[serde(rename = "stopProcessingRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_processing_rules: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScoredEmailAddress {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "relevanceScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance_score: Option<i64>,
    #[serde(rename = "selectionLikelihood")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_likelihood: Option<SelectionLikelihoodInfo>,
    #[serde(rename = "ItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _item_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Phone {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<PhoneType>,
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Website {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<WebsiteType>,
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PersonType {
    #[serde(rename = "class")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(rename = "subclass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclass: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LocationConstraint {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<LocationConstraintItem>>,
    #[serde(rename = "isRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(rename = "suggestLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggest_location: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LocationConstraintItem {
    #[serde(rename = "resolveAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_availability: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MeetingTimeSuggestionsResult {
    #[serde(rename = "meetingTimeSuggestions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_time_suggestions: Option<Vec<MeetingTimeSuggestion>>,
    #[serde(rename = "emptySuggestionsReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_suggestions_reason: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MeetingTimeSuggestion {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "organizerAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer_availability: Option<FreeBusyStatus>,
    #[serde(rename = "attendeeAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_availability: Option<Vec<AttendeeAvailability>>,
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
    #[serde(rename = "suggestionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_reason: Option<String>,
    #[serde(rename = "meetingTimeSlot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_time_slot: Option<TimeSlot>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AttendeeAvailability {
    #[serde(rename = "attendee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<AttendeeBase>,
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<FreeBusyStatus>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeSlot {
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeConstraint {
    #[serde(rename = "activityDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_domain: Option<ActivityDomain>,
    #[serde(rename = "timeSlots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_slots: Option<Vec<TimeSlot>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IdentitySet {
    #[serde(rename = "application")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Identity>,
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Identity>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Identity>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemReference {
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
    #[serde(rename = "driveType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_type: Option<String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "shareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharepoint_ids: Option<SharepointIds>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SharepointIds {
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    #[serde(rename = "listItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_item_id: Option<String>,
    #[serde(rename = "listItemUniqueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_item_unique_id: Option<String>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "siteUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "webId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PublicationFacet {
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BooleanColumn(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl BooleanColumn {
    pub fn new(value: Option<serde_json::Value>) -> BooleanColumn {
        BooleanColumn(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CalculatedColumn {
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "formula")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
    #[serde(rename = "outputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChoiceColumn {
    #[serde(rename = "allowTextEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_text_entry: Option<bool>,
    #[serde(rename = "choices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<String>>,
    #[serde(rename = "displayAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_as: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CurrencyColumn {
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DateTimeColumn {
    #[serde(rename = "displayAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_as: Option<String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DefaultColumnValue {
    #[serde(rename = "formula")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LookupColumn {
    #[serde(rename = "allowMultipleValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_values: Option<bool>,
    #[serde(rename = "allowUnlimitedLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unlimited_length: Option<bool>,
    #[serde(rename = "columnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    #[serde(rename = "primaryLookupColumnId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_lookup_column_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NumberColumn {
    #[serde(rename = "decimalPlaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<String>,
    #[serde(rename = "displayAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_as: Option<String>,
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PersonOrGroupColumn {
    #[serde(rename = "allowMultipleSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_selection: Option<bool>,
    #[serde(rename = "chooseFromType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose_from_type: Option<String>,
    #[serde(rename = "displayAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_as: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TextColumn {
    #[serde(rename = "allowMultipleLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_lines: Option<bool>,
    #[serde(rename = "appendChangesToExistingText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_changes_to_existing_text: Option<bool>,
    #[serde(rename = "linesForEditing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_for_editing: Option<i32>,
    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[serde(rename = "textType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContentTypeOrder {
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Quota {
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i64>,
    #[serde(rename = "remaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "used")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemFacet(#[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>);

#[cfg(feature = "option")]
impl SystemFacet {
    pub fn new(value: Option<serde_json::Value>) -> SystemFacet {
        SystemFacet(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    #[serde(rename = "album")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[serde(rename = "albumArtist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_artist: Option<String>,
    #[serde(rename = "artist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[serde(rename = "composers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composers: Option<String>,
    #[serde(rename = "copyright")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(rename = "disc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc: Option<i16>,
    #[serde(rename = "discCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_count: Option<i16>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "genre")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(rename = "hasDrm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_drm: Option<bool>,
    #[serde(rename = "isVariableBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_variable_bitrate: Option<bool>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "track")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<i32>,
    #[serde(rename = "trackCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_count: Option<i32>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Deleted {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "hashes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Hashes>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "processingMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_metadata: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Hashes {
    #[serde(rename = "crc32Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crc32_hash: Option<String>,
    #[serde(rename = "quickXorHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_xor_hash: Option<String>,
    #[serde(rename = "sha1Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1_hash: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileSystemInfo {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastAccessedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Folder {
    #[serde(rename = "childCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_count: Option<i32>,
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<FolderView>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FolderView {
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "viewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GeoCoordinates {
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<i64>,
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<i64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<i64>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Photo {
    #[serde(rename = "cameraMake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_make: Option<String>,
    #[serde(rename = "cameraModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_model: Option<String>,
    #[serde(rename = "exposureDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure_denominator: Option<i64>,
    #[serde(rename = "exposureNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure_numerator: Option<i64>,
    #[serde(rename = "fNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_number: Option<i64>,
    #[serde(rename = "focalLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focal_length: Option<i64>,
    #[serde(rename = "iso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso: Option<i32>,
    #[serde(rename = "takenDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taken_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RemoteItem {
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_info: Option<FileSystemInfo>,
    #[serde(rename = "folder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<Folder>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<Package>,
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ItemReference>,
    #[serde(rename = "shared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<Shared>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharepoint_ids: Option<SharepointIds>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "specialFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_folder: Option<SpecialFolder>,
    #[serde(rename = "webDavUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_dav_url: Option<String>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Shared {
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentitySet>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "sharedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_by: Option<IdentitySet>,
    #[serde(rename = "sharedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpecialFolder {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Root(#[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>);

#[cfg(feature = "option")]
impl Root {
    pub fn new(value: Option<serde_json::Value>) -> Root {
        Root(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "onClickTelemetryUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_click_telemetry_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Video {
    #[serde(rename = "audioBitsPerSample")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_bits_per_sample: Option<i32>,
    #[serde(rename = "audioChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<i32>,
    #[serde(rename = "audioFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_format: Option<String>,
    #[serde(rename = "audioSamplesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_samples_per_second: Option<i32>,
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "fourCC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub four_c_c: Option<String>,
    #[serde(rename = "frameRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<i64>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AccessAction(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl AccessAction {
    pub fn new(value: Option<serde_json::Value>) -> AccessAction {
        AccessAction(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemActionStat {
    #[serde(rename = "actionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_count: Option<i32>,
    #[serde(rename = "actorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IncompleteData {
    #[serde(rename = "missingDataBeforeDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_data_before_date_time: Option<String>,
    #[serde(rename = "wasThrottled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_throttled: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListInfo {
    #[serde(rename = "contentTypesEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_types_enabled: Option<bool>,
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContentTypeInfo {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SharingInvitation {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "invitedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<IdentitySet>,
    #[serde(rename = "redeemedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemed_by: Option<String>,
    #[serde(rename = "signInRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_required: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SharingLink {
    #[serde(rename = "application")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Identity>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SiteCollection {
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Thumbnail {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<u8>>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "sourceItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_item_id: Option<String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DriveItemUploadableProperties {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_info: Option<FileSystemInfo>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DriveRecipient {
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemPreviewInfo {
    #[serde(rename = "getUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_url: Option<String>,
    #[serde(rename = "postParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_parameters: Option<String>,
    #[serde(rename = "postUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UploadSession {
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "nextExpectedRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_expected_ranges: Option<Vec<String>>,
    #[serde(rename = "uploadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookSessionInfo {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "persistChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist_changes: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Json(#[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>);

#[cfg(feature = "option")]
impl Json {
    pub fn new(value: Option<serde_json::Value>) -> Json {
        Json(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFilterCriteria {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "criterion1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion1: Option<String>,
    #[serde(rename = "criterion2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion2: Option<String>,
    #[serde(rename = "dynamicCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_criteria: Option<String>,
    #[serde(rename = "filterOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_on: Option<String>,
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<WorkbookIcon>,
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Json>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookIcon {
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "set")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookSortField {
    #[serde(rename = "ascending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "dataOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_option: Option<String>,
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<WorkbookIcon>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
    #[serde(rename = "sortOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_on: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookWorksheetProtectionOptions {
    #[serde(rename = "allowAutoFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_auto_filter: Option<bool>,
    #[serde(rename = "allowDeleteColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_columns: Option<bool>,
    #[serde(rename = "allowDeleteRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_rows: Option<bool>,
    #[serde(rename = "allowFormatCells")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_format_cells: Option<bool>,
    #[serde(rename = "allowFormatColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_format_columns: Option<bool>,
    #[serde(rename = "allowFormatRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_format_rows: Option<bool>,
    #[serde(rename = "allowInsertColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_insert_columns: Option<bool>,
    #[serde(rename = "allowInsertHyperlinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_insert_hyperlinks: Option<bool>,
    #[serde(rename = "allowInsertRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_insert_rows: Option<bool>,
    #[serde(rename = "allowPivotTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_pivot_tables: Option<bool>,
    #[serde(rename = "allowSort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sort: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFilterDatetime {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "specificity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificity: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeReference {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InvitedUserMessageInfo {
    #[serde(rename = "ccRecipients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_recipients: Option<Vec<Recipient>>,
    #[serde(rename = "messageLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_language: Option<String>,
    #[serde(rename = "customizedMessageBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_message_body: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerAppliedCategories(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl PlannerAppliedCategories {
    pub fn new(value: Option<serde_json::Value>) -> PlannerAppliedCategories {
        PlannerAppliedCategories(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerAssignments(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl PlannerAssignments {
    pub fn new(value: Option<serde_json::Value>) -> PlannerAssignments {
        PlannerAssignments(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerExternalReference {
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "previewPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_priority: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerChecklistItem {
    #[serde(rename = "isChecked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_checked: Option<bool>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "orderHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hint: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerAssignment {
    #[serde(rename = "assignedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_by: Option<IdentitySet>,
    #[serde(rename = "assignedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_date_time: Option<String>,
    #[serde(rename = "orderHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hint: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerExternalReferences(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl PlannerExternalReferences {
    pub fn new(value: Option<serde_json::Value>) -> PlannerExternalReferences {
        PlannerExternalReferences(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerChecklistItems(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl PlannerChecklistItems {
    pub fn new(value: Option<serde_json::Value>) -> PlannerChecklistItems {
        PlannerChecklistItems(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerOrderHintsByAssignee(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl PlannerOrderHintsByAssignee {
    pub fn new(value: Option<serde_json::Value>) -> PlannerOrderHintsByAssignee {
        PlannerOrderHintsByAssignee(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerUserIds(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl PlannerUserIds {
    pub fn new(value: Option<serde_json::Value>) -> PlannerUserIds {
        PlannerUserIds(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerCategoryDescriptions {
    #[serde(rename = "category1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category1: Option<String>,
    #[serde(rename = "category2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category2: Option<String>,
    #[serde(rename = "category3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category3: Option<String>,
    #[serde(rename = "category4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category4: Option<String>,
    #[serde(rename = "category5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category5: Option<String>,
    #[serde(rename = "category6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category6: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NotebookLinks {
    #[serde(rename = "oneNoteClientUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_note_client_url: Option<ExternalLink>,
    #[serde(rename = "oneNoteWebUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_note_web_url: Option<ExternalLink>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ExternalLink {
    #[serde(rename = "href")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SectionLinks {
    #[serde(rename = "oneNoteClientUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_note_client_url: Option<ExternalLink>,
    #[serde(rename = "oneNoteWebUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_note_web_url: Option<ExternalLink>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PageLinks {
    #[serde(rename = "oneNoteClientUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_note_client_url: Option<ExternalLink>,
    #[serde(rename = "oneNoteWebUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_note_web_url: Option<ExternalLink>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenoteOperationError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenotePatchContentCommand {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<OnenotePatchActionType>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<OnenotePatchInsertPosition>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenotePagePreview {
    #[serde(rename = "previewText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<OnenotePagePreviewLinks>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenotePagePreviewLinks {
    #[serde(rename = "previewImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image_url: Option<ExternalLink>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RecentNotebook {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastAccessedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_time: Option<String>,
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<RecentNotebookLinks>,
    #[serde(rename = "sourceService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_service: Option<OnenoteSourceService>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RecentNotebookLinks {
    #[serde(rename = "oneNoteClientUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_note_client_url: Option<ExternalLink>,
    #[serde(rename = "oneNoteWebUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_note_web_url: Option<ExternalLink>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CopyNotebookModel {
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "userRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<OnenoteUserRole>,
    #[serde(rename = "isShared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(rename = "sectionsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections_url: Option<String>,
    #[serde(rename = "sectionGroupsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_groups_url: Option<String>,
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<NotebookLinks>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_identity: Option<IdentitySet>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_identity: Option<IdentitySet>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<u8>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EducationStudent {
    #[serde(rename = "graduationYear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduation_year: Option<String>,
    #[serde(rename = "grade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<String>,
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<EducationGender>,
    #[serde(rename = "studentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub student_number: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EducationTeacher {
    #[serde(rename = "teacherNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher_number: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EducationTerm {
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceAndAppManagementAssignmentTarget(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl DeviceAndAppManagementAssignmentTarget {
    pub fn new(value: Option<serde_json::Value>) -> DeviceAndAppManagementAssignmentTarget {
        DeviceAndAppManagementAssignmentTarget(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileAppAssignmentSettings(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl MobileAppAssignmentSettings {
    pub fn new(value: Option<serde_json::Value>) -> MobileAppAssignmentSettings {
        MobileAppAssignmentSettings(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MimeContent {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<u8>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileEncryptionInfo {
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<Vec<u8>>,
    #[serde(rename = "initializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector: Option<Vec<u8>>,
    #[serde(rename = "mac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<Vec<u8>>,
    #[serde(rename = "macKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_key: Option<Vec<u8>>,
    #[serde(rename = "profileIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_identifier: Option<String>,
    #[serde(rename = "fileDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_digest: Option<Vec<u8>>,
    #[serde(rename = "fileDigestAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_digest_algorithm: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AllLicensedUsersAssignmentTarget(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl AllLicensedUsersAssignmentTarget {
    pub fn new(value: Option<serde_json::Value>) -> AllLicensedUsersAssignmentTarget {
        AllLicensedUsersAssignmentTarget(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupAssignmentTarget {
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ExclusionGroupAssignmentTarget(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl ExclusionGroupAssignmentTarget {
    pub fn new(value: Option<serde_json::Value>) -> ExclusionGroupAssignmentTarget {
        ExclusionGroupAssignmentTarget(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AllDevicesAssignmentTarget(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl AllDevicesAssignmentTarget {
    pub fn new(value: Option<serde_json::Value>) -> AllDevicesAssignmentTarget {
        AllDevicesAssignmentTarget(value)
    }

    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosLobAppAssignmentSettings {
    #[serde(rename = "vpnConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_configuration_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosStoreAppAssignmentSettings {
    #[serde(rename = "vpnConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_configuration_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosVppAppAssignmentSettings {
    #[serde(rename = "useDeviceLicensing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_device_licensing: Option<bool>,
    #[serde(rename = "vpnConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_configuration_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftStoreForBusinessAppAssignmentSettings {
    #[serde(rename = "useDeviceContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_device_context: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidMinimumOperatingSystem {
    #[serde(rename = "v4_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_0: Option<bool>,
    #[serde(rename = "v4_0_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_0_3: Option<bool>,
    #[serde(rename = "v4_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_1: Option<bool>,
    #[serde(rename = "v4_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_2: Option<bool>,
    #[serde(rename = "v4_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_3: Option<bool>,
    #[serde(rename = "v4_4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_4: Option<bool>,
    #[serde(rename = "v5_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v5_0: Option<bool>,
    #[serde(rename = "v5_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v5_1: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosDeviceType {
    #[serde(rename = "iPad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pad: Option<bool>,
    #[serde(rename = "iPhoneAndIPod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_phone_and_i_pod: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosMinimumOperatingSystem {
    #[serde(rename = "v8_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v8_0: Option<bool>,
    #[serde(rename = "v9_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v9_0: Option<bool>,
    #[serde(rename = "v10_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v10_0: Option<bool>,
    #[serde(rename = "v11_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v11_0: Option<bool>,
    #[serde(rename = "v12_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v12_0: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsMinimumOperatingSystem {
    #[serde(rename = "v8_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v8_0: Option<bool>,
    #[serde(rename = "v8_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v8_1: Option<bool>,
    #[serde(rename = "v10_0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v10_0: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VppLicensingType {
    #[serde(rename = "supportsUserLicensing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_user_licensing: Option<bool>,
    #[serde(rename = "supportsDeviceLicensing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_device_licensing: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AppConfigurationSettingItem {
    #[serde(rename = "appConfigKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_config_key: Option<String>,
    #[serde(rename = "appConfigKeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_config_key_type: Option<MdmAppConfigKeyType>,
    #[serde(rename = "appConfigKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_config_key_value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagementSettings {
    #[serde(rename = "deviceComplianceCheckinThresholdDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_compliance_checkin_threshold_days: Option<i32>,
    #[serde(rename = "isScheduledActionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_scheduled_action_enabled: Option<bool>,
    #[serde(rename = "secureByDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_by_default: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IntuneBrand {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "contactITName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_i_t_name: Option<String>,
    #[serde(rename = "contactITPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_i_t_phone_number: Option<String>,
    #[serde(rename = "contactITEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_i_t_email_address: Option<String>,
    #[serde(rename = "contactITNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_i_t_notes: Option<String>,
    #[serde(rename = "privacyUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_url: Option<String>,
    #[serde(rename = "onlineSupportSiteUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_support_site_url: Option<String>,
    #[serde(rename = "onlineSupportSiteName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_support_site_name: Option<String>,
    #[serde(rename = "themeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_color: Option<RgbColor>,
    #[serde(rename = "showLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_logo: Option<bool>,
    #[serde(rename = "lightBackgroundLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_background_logo: Option<MimeContent>,
    #[serde(rename = "darkBackgroundLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_background_logo: Option<MimeContent>,
    #[serde(rename = "showNameNextToLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_name_next_to_logo: Option<bool>,
    #[serde(rename = "showDisplayNameNextToLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_display_name_next_to_logo: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RgbColor {
    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<u8>,
    #[serde(rename = "g")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g: Option<u8>,
    #[serde(rename = "b")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<u8>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceActionResult {
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "actionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_state: Option<ActionState>,
    #[serde(rename = "startDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationManagerClientEnabledFeatures {
    #[serde(rename = "inventory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<bool>,
    #[serde(rename = "modernApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modern_apps: Option<bool>,
    #[serde(rename = "resourceAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access: Option<bool>,
    #[serde(rename = "deviceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<bool>,
    #[serde(rename = "compliancePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_policy: Option<bool>,
    #[serde(rename = "windowsUpdateForBusiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_update_for_business: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceHealthAttestationState {
    #[serde(rename = "lastUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<String>,
    #[serde(rename = "contentNamespaceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_namespace_url: Option<String>,
    #[serde(rename = "deviceHealthAttestationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_health_attestation_status: Option<String>,
    #[serde(rename = "contentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
    #[serde(rename = "issuedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_date_time: Option<String>,
    #[serde(rename = "attestationIdentityKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_identity_key: Option<String>,
    #[serde(rename = "resetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_count: Option<i64>,
    #[serde(rename = "restartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i64>,
    #[serde(rename = "dataExcutionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_excution_policy: Option<String>,
    #[serde(rename = "bitLockerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_locker_status: Option<String>,
    #[serde(rename = "bootManagerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_manager_version: Option<String>,
    #[serde(rename = "codeIntegrityCheckVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_integrity_check_version: Option<String>,
    #[serde(rename = "secureBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_boot: Option<String>,
    #[serde(rename = "bootDebugging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_debugging: Option<String>,
    #[serde(rename = "operatingSystemKernelDebugging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_kernel_debugging: Option<String>,
    #[serde(rename = "codeIntegrity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_integrity: Option<String>,
    #[serde(rename = "testSigning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_signing: Option<String>,
    #[serde(rename = "safeMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_mode: Option<String>,
    #[serde(rename = "windowsPE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_p_e: Option<String>,
    #[serde(rename = "earlyLaunchAntiMalwareDriverProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_launch_anti_malware_driver_protection: Option<String>,
    #[serde(rename = "virtualSecureMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_secure_mode: Option<String>,
    #[serde(rename = "pcrHashAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_hash_algorithm: Option<String>,
    #[serde(rename = "bootAppSecurityVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_app_security_version: Option<String>,
    #[serde(rename = "bootManagerSecurityVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_manager_security_version: Option<String>,
    #[serde(rename = "tpmVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpm_version: Option<String>,
    #[serde(rename = "pcr0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr0: Option<String>,
    #[serde(rename = "secureBootConfigurationPolicyFingerPrint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_boot_configuration_policy_finger_print: Option<String>,
    #[serde(rename = "codeIntegrityPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_integrity_policy: Option<String>,
    #[serde(rename = "bootRevisionListInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_revision_list_info: Option<String>,
    #[serde(rename = "operatingSystemRevListInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_rev_list_info: Option<String>,
    #[serde(rename = "healthStatusMismatchInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status_mismatch_info: Option<String>,
    #[serde(rename = "healthAttestationSupportedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_attestation_supported_status: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateWindowsDeviceAccountActionParameter {
    #[serde(rename = "deviceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_account: Option<WindowsDeviceAccount>,
    #[serde(rename = "passwordRotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_rotation_enabled: Option<bool>,
    #[serde(rename = "calendarSyncEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_sync_enabled: Option<bool>,
    #[serde(rename = "deviceAccountEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_account_email: Option<String>,
    #[serde(rename = "exchangeServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_server: Option<String>,
    #[serde(rename = "sessionInitiationProtocalAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_initiation_protocal_address: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsDeviceAccount {
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsDefenderScanActionResult {
    #[serde(rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeleteUserFromSharedAppleDeviceActionResult {
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceGeoLocation {
    #[serde(rename = "lastCollectedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_collected_date_time: Option<String>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<i64>,
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<i64>,
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<i64>,
    #[serde(rename = "horizontalAccuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<i64>,
    #[serde(rename = "verticalAccuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_accuracy: Option<i64>,
    #[serde(rename = "heading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LocateDeviceActionResult {
    #[serde(rename = "deviceLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_location: Option<DeviceGeoLocation>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RemoteLockActionResult {
    #[serde(rename = "unlockPin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_pin: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResetPasscodeActionResult {
    #[serde(rename = "passcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceOperatingSystemSummary {
    #[serde(rename = "androidCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_count: Option<i32>,
    #[serde(rename = "iosCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_count: Option<i32>,
    #[serde(rename = "macOSCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_o_s_count: Option<i32>,
    #[serde(rename = "windowsMobileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_mobile_count: Option<i32>,
    #[serde(rename = "windowsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_count: Option<i32>,
    #[serde(rename = "unknownCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceExchangeAccessStateSummary {
    #[serde(rename = "allowedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_device_count: Option<i32>,
    #[serde(rename = "blockedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_device_count: Option<i32>,
    #[serde(rename = "quarantinedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarantined_device_count: Option<i32>,
    #[serde(rename = "unknownDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_device_count: Option<i32>,
    #[serde(rename = "unavailableDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_device_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsDeviceADAccount {
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsDeviceAzureADAccount {
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AppListItem {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publisher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "appStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<String>,
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OmaSetting {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "omaUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oma_uri: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingInteger {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingFloatingPoint {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingString {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingDateTime {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingStringXml {
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<u8>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingBoolean {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingBase64 {
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingAustralia {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingAustraliaMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingAustraliaTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingCanada {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingCanadaMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingCanadaTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingFrance {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingFranceMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingFranceTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingGermany {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingGermanyMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingGermanyTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingIreland {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingIrelandMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingIrelandTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingJapan {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingJapanMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingJapanTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingNewZealand {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingNewZealandMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingNewZealandTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingUnitedKingdom {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingUnitedKingdomMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingUnitedKingdomTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingUnitedStates {
    #[serde(rename = "movieRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_rating: Option<RatingUnitedStatesMoviesType>,
    #[serde(rename = "tvRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv_rating: Option<RatingUnitedStatesTelevisionType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosNetworkUsageRule {
    #[serde(rename = "managedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_apps: Option<Vec<AppListItem>>,
    #[serde(rename = "cellularDataBlockWhenRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_data_block_when_roaming: Option<bool>,
    #[serde(rename = "cellularDataBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_data_blocked: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenItem {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenPage {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "icons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IosHomeScreenItem>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosNotificationSettings {
    #[serde(rename = "bundleID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_i_d: Option<String>,
    #[serde(rename = "appName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(rename = "publisher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "showInNotificationCenter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_notification_center: Option<bool>,
    #[serde(rename = "showOnLockScreen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_on_lock_screen: Option<bool>,
    #[serde(rename = "alertType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<IosNotificationAlertType>,
    #[serde(rename = "badgesEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges_enabled: Option<bool>,
    #[serde(rename = "soundsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sounds_enabled: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenFolder {
    #[serde(rename = "pages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<IosHomeScreenFolderPage>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenFolderPage {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<IosHomeScreenApp>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenApp {
    #[serde(rename = "bundleID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_i_d: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsFirewallNetworkProfile {
    #[serde(rename = "firewallEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_enabled: Option<StateManagementSetting>,
    #[serde(rename = "stealthModeBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth_mode_blocked: Option<bool>,
    #[serde(rename = "incomingTrafficBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_traffic_blocked: Option<bool>,
    #[serde(rename = "unicastResponsesToMulticastBroadcastsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicast_responses_to_multicast_broadcasts_blocked: Option<bool>,
    #[serde(rename = "inboundNotificationsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_notifications_blocked: Option<bool>,
    #[serde(rename = "authorizedApplicationRulesFromGroupPolicyMerged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_application_rules_from_group_policy_merged: Option<bool>,
    #[serde(rename = "globalPortRulesFromGroupPolicyMerged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_port_rules_from_group_policy_merged: Option<bool>,
    #[serde(rename = "connectionSecurityRulesFromGroupPolicyMerged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_security_rules_from_group_policy_merged: Option<bool>,
    #[serde(rename = "outboundConnectionsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_connections_blocked: Option<bool>,
    #[serde(rename = "inboundConnectionsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_connections_blocked: Option<bool>,
    #[serde(rename = "securedPacketExemptionAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secured_packet_exemption_allowed: Option<bool>,
    #[serde(rename = "policyRulesFromGroupPolicyMerged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_rules_from_group_policy_merged: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BitLockerRemovableDrivePolicy {
    #[serde(rename = "encryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<BitLockerEncryptionMethod>,
    #[serde(rename = "requireEncryptionForWriteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_encryption_for_write_access: Option<bool>,
    #[serde(rename = "blockCrossOrganizationWriteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_cross_organization_write_access: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DefenderDetectedMalwareActions {
    #[serde(rename = "lowSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_severity: Option<DefenderThreatAction>,
    #[serde(rename = "moderateSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderate_severity: Option<DefenderThreatAction>,
    #[serde(rename = "highSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_severity: Option<DefenderThreatAction>,
    #[serde(rename = "severeSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severe_severity: Option<DefenderThreatAction>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10NetworkProxyServer {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "exceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<String>>,
    #[serde(rename = "useForLocalAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_for_local_addresses: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EdgeSearchEngineBase(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl EdgeSearchEngineBase {
    pub fn new(value: Option<serde_json::Value>) -> EdgeSearchEngineBase {
        EdgeSearchEngineBase(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EdgeSearchEngineCustom {
    #[serde(rename = "edgeSearchEngineOpenSearchXmlUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_search_engine_open_search_xml_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EdgeSearchEngine {
    #[serde(rename = "edgeSearchEngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_search_engine_type: Option<EdgeSearchEngineType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SharedPCAccountManagerPolicy {
    #[serde(rename = "accountDeletionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_deletion_policy: Option<SharedPCAccountDeletionPolicyType>,
    #[serde(rename = "cacheAccountsAboveDiskFreePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_accounts_above_disk_free_percentage: Option<i32>,
    #[serde(rename = "inactiveThresholdDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_threshold_days: Option<i32>,
    #[serde(rename = "removeAccountsBelowDiskFreePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_accounts_below_disk_free_percentage: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsUpdateInstallScheduleType(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);
#[cfg(feature = "option")]
impl WindowsUpdateInstallScheduleType {
    pub fn new(value: Option<serde_json::Value>) -> WindowsUpdateInstallScheduleType {
        WindowsUpdateInstallScheduleType(value)
    }

    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}
#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsUpdateScheduledInstall {
    #[serde(rename = "scheduledInstallDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_install_day: Option<WeeklySchedule>,
    #[serde(rename = "scheduledInstallTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_install_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsUpdateActiveHoursInstall {
    #[serde(rename = "activeHoursStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_hours_start: Option<String>,
    #[serde(rename = "activeHoursEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_hours_end: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationSettingState {
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<String>,
    #[serde(rename = "settingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_name: Option<String>,
    #[serde(rename = "instanceDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_display_name: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ComplianceStatus>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    #[serde(rename = "errorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SettingSource>>,
    #[serde(rename = "currentValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SettingSource {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicySettingState {
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<String>,
    #[serde(rename = "settingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_name: Option<String>,
    #[serde(rename = "instanceDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_display_name: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ComplianceStatus>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    #[serde(rename = "errorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SettingSource>>,
    #[serde(rename = "currentValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentPlatformRestriction {
    #[serde(rename = "platformBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_blocked: Option<bool>,
    #[serde(rename = "personalDeviceEnrollmentBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_device_enrollment_blocked: Option<bool>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileAppIdentifier(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl MobileAppIdentifier {
    pub fn new(value: Option<serde_json::Value>) -> MobileAppIdentifier {
        MobileAppIdentifier(value)
    }

    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppDiagnosticStatus {
    #[serde(rename = "validationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_name: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "mitigationInstruction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_instruction: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyValuePair {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionResourceCollection {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionDataRecoveryCertificate {
    #[serde(rename = "subjectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Vec<u8>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionApp {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "publisherName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(rename = "productName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "denied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionProxiedDomainCollection {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "proxiedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied_domains: Option<Vec<ProxiedDomain>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProxiedDomain {
    #[serde(rename = "ipAddressOrFQDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_or_f_q_d_n: Option<String>,
    #[serde(rename = "proxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionIPRangeCollection {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ranges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<IpRange>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IpRange(#[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>);

#[cfg(feature = "option")]
impl IpRange {
    pub fn new(value: Option<serde_json::Value>) -> IpRange {
        IpRange(value)
    }

    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidMobileAppIdentifier {
    #[serde(rename = "packageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosMobileAppIdentifier {
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppPolicyDeploymentSummaryPerApp {
    #[serde(rename = "mobileAppIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_app_identifier: Option<MobileAppIdentifier>,
    #[serde(rename = "configurationAppliedUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_applied_user_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionStoreApp(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl WindowsInformationProtectionStoreApp {
    pub fn new(value: Option<serde_json::Value>) -> WindowsInformationProtectionStoreApp {
        WindowsInformationProtectionStoreApp(value)
    }

    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionDesktopApp {
    #[serde(rename = "binaryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_name: Option<String>,
    #[serde(rename = "binaryVersionLow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_version_low: Option<String>,
    #[serde(rename = "binaryVersionHigh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_version_high: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IPv6Range {
    #[serde(rename = "lowerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_address: Option<String>,
    #[serde(rename = "upperAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_address: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IPv4Range {
    #[serde(rename = "lowerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_address: Option<String>,
    #[serde(rename = "upperAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_address: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RolePermission {
    #[serde(rename = "resourceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_actions: Option<Vec<ResourceAction>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourceAction {
    #[serde(rename = "allowedResourceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_resource_actions: Option<Vec<String>>,
    #[serde(rename = "notAllowedResourceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_allowed_resource_actions: Option<Vec<String>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "alternativeText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_text: Option<String>,
    #[serde(rename = "alternateText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_text: Option<String>,
    #[serde(rename = "addImageQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_image_query: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VisualInfo {
    #[serde(rename = "attribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<ImageInfo>,
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Json>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudAppSecurityState {
    #[serde(rename = "destinationServiceIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_service_ip: Option<String>,
    #[serde(rename = "destinationServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_service_name: Option<String>,
    #[serde(rename = "riskScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileSecurityState {
    #[serde(rename = "fileHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_hash: Option<FileHash>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "riskScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileHash {
    #[serde(rename = "hashType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<FileHashType>,
    #[serde(rename = "hashValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AlertHistoryState {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "assignedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<String>>,
    #[serde(rename = "feedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<AlertFeedback>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertStatus>,
    #[serde(rename = "updatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_time: Option<String>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HostSecurityState {
    #[serde(rename = "fqdn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "isAzureAdJoined")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_azure_ad_joined: Option<bool>,
    #[serde(rename = "isAzureAdRegistered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_azure_ad_registered: Option<bool>,
    #[serde(rename = "isHybridAzureDomainJoined")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hybrid_azure_domain_joined: Option<bool>,
    #[serde(rename = "netBiosName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_bios_name: Option<String>,
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "privateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "publicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[serde(rename = "riskScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MalwareState {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "wasRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_running: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NetworkConnection {
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "destinationAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,
    #[serde(rename = "destinationDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_domain: Option<String>,
    #[serde(rename = "destinationPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<String>,
    #[serde(rename = "destinationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_url: Option<String>,
    #[serde(rename = "direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ConnectionDirection>,
    #[serde(rename = "domainRegisteredDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registered_date_time: Option<String>,
    #[serde(rename = "localDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_dns_name: Option<String>,
    #[serde(rename = "natDestinationAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_destination_address: Option<String>,
    #[serde(rename = "natDestinationPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_destination_port: Option<String>,
    #[serde(rename = "natSourceAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_source_address: Option<String>,
    #[serde(rename = "natSourcePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_source_port: Option<String>,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<SecurityNetworkProtocol>,
    #[serde(rename = "riskScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<String>,
    #[serde(rename = "sourceAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    #[serde(rename = "sourcePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ConnectionStatus>,
    #[serde(rename = "urlParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_parameters: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Process {
    #[serde(rename = "accountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "commandLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "fileHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_hash: Option<FileHash>,
    #[serde(rename = "integrityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrity_level: Option<ProcessIntegrityLevel>,
    #[serde(rename = "isElevated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_elevated: Option<bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentProcessCreatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_process_created_date_time: Option<String>,
    #[serde(rename = "parentProcessId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_process_id: Option<i32>,
    #[serde(rename = "parentProcessName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_process_name: Option<String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "processId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RegistryKeyState {
    #[serde(rename = "hive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hive: Option<RegistryHive>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "oldKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_key: Option<String>,
    #[serde(rename = "oldValueData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value_data: Option<String>,
    #[serde(rename = "oldValueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value_name: Option<String>,
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<RegistryOperation>,
    #[serde(rename = "processId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i32>,
    #[serde(rename = "valueData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_data: Option<String>,
    #[serde(rename = "valueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_name: Option<String>,
    #[serde(rename = "valueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<RegistryValueType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AlertTrigger {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserSecurityState {
    #[serde(rename = "aadUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aad_user_id: Option<String>,
    #[serde(rename = "accountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "emailRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_role: Option<EmailRole>,
    #[serde(rename = "isVpn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vpn: Option<bool>,
    #[serde(rename = "logonDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_date_time: Option<String>,
    #[serde(rename = "logonId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_id: Option<String>,
    #[serde(rename = "logonIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_ip: Option<String>,
    #[serde(rename = "logonLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_location: Option<String>,
    #[serde(rename = "logonType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_type: Option<LogonType>,
    #[serde(rename = "onPremisesSecurityIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier: Option<String>,
    #[serde(rename = "riskScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<String>,
    #[serde(rename = "userAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_account_type: Option<UserAccountSecurityType>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SecurityVendorInformation {
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "providerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_version: Option<String>,
    #[serde(rename = "subProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_provider: Option<String>,
    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VulnerabilityState {
    #[serde(rename = "cve")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve: Option<String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "wasRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_running: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AverageComparativeScore {
    #[serde(rename = "averageScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_score: Option<i64>,
    #[serde(rename = "basis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ControlScore {
    #[serde(rename = "controlCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_category: Option<String>,
    #[serde(rename = "controlName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ComplianceInformation {
    #[serde(rename = "certificationControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certification_controls: Option<Vec<CertificationControl>>,
    #[serde(rename = "certificationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certification_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CertificationControl {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SecureScoreControlStateUpdate {
    #[serde(rename = "assignedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "updatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "updatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourceVisualization {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "previewImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image_url: Option<String>,
    #[serde(rename = "previewText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(rename = "containerWebUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_web_url: Option<String>,
    #[serde(rename = "containerDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_display_name: Option<String>,
    #[serde(rename = "containerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourceReference {
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SharingDetail {
    #[serde(rename = "sharedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_by: Option<InsightIdentity>,
    #[serde(rename = "sharedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_date_time: Option<String>,
    #[serde(rename = "sharingSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_subject: Option<String>,
    #[serde(rename = "sharingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_type: Option<String>,
    #[serde(rename = "sharingReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_reference: Option<ResourceReference>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InsightIdentity {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageDetails {
    #[serde(rename = "lastAccessedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamMemberSettings {
    #[serde(rename = "allowCreateUpdateChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_create_update_channels: Option<bool>,
    #[serde(rename = "allowDeleteChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_channels: Option<bool>,
    #[serde(rename = "allowAddRemoveApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_add_remove_apps: Option<bool>,
    #[serde(rename = "allowCreateUpdateRemoveTabs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_create_update_remove_tabs: Option<bool>,
    #[serde(rename = "allowCreateUpdateRemoveConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_create_update_remove_connectors: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamGuestSettings {
    #[serde(rename = "allowCreateUpdateChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_create_update_channels: Option<bool>,
    #[serde(rename = "allowDeleteChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_channels: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamMessagingSettings {
    #[serde(rename = "allowUserEditMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_edit_messages: Option<bool>,
    #[serde(rename = "allowUserDeleteMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_delete_messages: Option<bool>,
    #[serde(rename = "allowOwnerDeleteMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_owner_delete_messages: Option<bool>,
    #[serde(rename = "allowTeamMentions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_team_mentions: Option<bool>,
    #[serde(rename = "allowChannelMentions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_mentions: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamFunSettings {
    #[serde(rename = "allowGiphy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_giphy: Option<bool>,
    #[serde(rename = "giphyContentRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giphy_content_rating: Option<GiphyRatingType>,
    #[serde(rename = "allowStickersAndMemes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_stickers_and_memes: Option<bool>,
    #[serde(rename = "allowCustomMemes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_custom_memes: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamClassSettings {
    #[serde(rename = "notifyGuardiansAboutAssignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_guardians_about_assignments: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamsTabConfiguration {
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "contentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(rename = "removeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_url: Option<String>,
    #[serde(rename = "websiteUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OperationError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditActivityInitiator {
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserIdentity>,
    #[serde(rename = "app")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppIdentity>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserIdentity {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AppIdentity {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "servicePrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_id: Option<String>,
    #[serde(rename = "servicePrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TargetResource {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "groupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<GroupType>,
    #[serde(rename = "modifiedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_properties: Option<Vec<ModifiedProperty>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModifiedProperty {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "oldValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(rename = "newValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SignInStatus {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "additionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceDetail {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "browser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(rename = "isCompliant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[serde(rename = "isManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
    #[serde(rename = "trustType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SignInLocation {
    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "countryOrRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
    #[serde(rename = "geoCoordinates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_coordinates: Option<GeoCoordinates>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AppliedConditionalAccessPolicy {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "enforcedGrantControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforced_grant_controls: Option<Vec<String>>,
    #[serde(rename = "enforcedSessionControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforced_session_controls: Option<Vec<String>>,
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<AppliedConditionalAccessPolicyResult>,
}
