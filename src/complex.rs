#[cfg(not(feature = "option"))]
use crate::enumtypes::*;

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlternativeSecurityId {
    #[serde(rename = "type")]
    pub _type: i32,
    #[serde(rename = "identityProvider")]
    pub identity_provider: String,
    #[serde(rename = "key")]
    pub key: Vec<u8>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainState {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "lastActionDateTime")]
    pub last_action_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePlanInfo {
    #[serde(rename = "servicePlanId")]
    pub service_plan_id: String,
    #[serde(rename = "servicePlanName")]
    pub service_plan_name: String,
    #[serde(rename = "provisioningStatus")]
    pub provisioning_status: String,
    #[serde(rename = "appliesTo")]
    pub applies_to: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignedLicense {
    #[serde(rename = "disabledPlans")]
    pub disabled_plans: Vec<String>,
    #[serde(rename = "skuId")]
    pub sku_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseProcessingState {
    #[serde(rename = "state")]
    pub state: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPremisesProvisioningError {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "propertyCausingError")]
    pub property_causing_error: String,
    #[serde(rename = "occurredDateTime")]
    pub occurred_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseUnitsDetail {
    #[serde(rename = "enabled")]
    pub enabled: i32,
    #[serde(rename = "suspended")]
    pub suspended: i32,
    #[serde(rename = "warning")]
    pub warning: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignedPlan {
    #[serde(rename = "assignedDateTime")]
    pub assigned_date_time: String,
    #[serde(rename = "capabilityStatus")]
    pub capability_status: String,
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "servicePlanId")]
    pub service_plan_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivacyProfile {
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
    #[serde(rename = "statementUrl")]
    pub statement_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedPlan {
    #[serde(rename = "capabilityStatus")]
    pub capability_status: String,
    #[serde(rename = "provisioningStatus")]
    pub provisioning_status: String,
    #[serde(rename = "service")]
    pub service: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifiedDomain {
    #[serde(rename = "capabilities")]
    pub capabilities: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isInitial")]
    pub is_initial: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseAssignmentState {
    #[serde(rename = "skuId")]
    pub sku_id: String,
    #[serde(rename = "disabledPlans")]
    pub disabled_plans: Vec<String>,
    #[serde(rename = "assignedByGroup")]
    pub assigned_by_group: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "error")]
    pub error: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPremisesExtensionAttributes {
    #[serde(rename = "extensionAttribute1")]
    pub extension_attribute1: String,
    #[serde(rename = "extensionAttribute2")]
    pub extension_attribute2: String,
    #[serde(rename = "extensionAttribute3")]
    pub extension_attribute3: String,
    #[serde(rename = "extensionAttribute4")]
    pub extension_attribute4: String,
    #[serde(rename = "extensionAttribute5")]
    pub extension_attribute5: String,
    #[serde(rename = "extensionAttribute6")]
    pub extension_attribute6: String,
    #[serde(rename = "extensionAttribute7")]
    pub extension_attribute7: String,
    #[serde(rename = "extensionAttribute8")]
    pub extension_attribute8: String,
    #[serde(rename = "extensionAttribute9")]
    pub extension_attribute9: String,
    #[serde(rename = "extensionAttribute10")]
    pub extension_attribute10: String,
    #[serde(rename = "extensionAttribute11")]
    pub extension_attribute11: String,
    #[serde(rename = "extensionAttribute12")]
    pub extension_attribute12: String,
    #[serde(rename = "extensionAttribute13")]
    pub extension_attribute13: String,
    #[serde(rename = "extensionAttribute14")]
    pub extension_attribute14: String,
    #[serde(rename = "extensionAttribute15")]
    pub extension_attribute15: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordProfile {
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "forceChangePasswordNextSignIn")]
    pub force_change_password_next_sign_in: bool,
    #[serde(rename = "forceChangePasswordNextSignInWithMfa")]
    pub force_change_password_next_sign_in_with_mfa: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MailboxSettings {
    #[serde(rename = "automaticRepliesSetting")]
    pub automatic_replies_setting: AutomaticRepliesSetting,
    #[serde(rename = "archiveFolder")]
    pub archive_folder: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "language")]
    pub language: LocaleInfo,
    #[serde(rename = "workingHours")]
    pub working_hours: WorkingHours,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomaticRepliesSetting {
    #[serde(rename = "status")]
    pub status: AutomaticRepliesStatus,
    #[serde(rename = "externalAudience")]
    pub external_audience: ExternalAudienceScope,
    #[serde(rename = "scheduledStartDateTime")]
    pub scheduled_start_date_time: String,
    #[serde(rename = "scheduledEndDateTime")]
    pub scheduled_end_date_time: String,
    #[serde(rename = "internalReplyMessage")]
    pub internal_reply_message: String,
    #[serde(rename = "externalReplyMessage")]
    pub external_reply_message: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateTimeTimeZone {
    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocaleInfo {
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkingHours {
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Vec<DayOfWeek>,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "timeZone")]
    pub time_zone: TimeZoneBase,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneBase {
    #[serde(rename = "name")]
    pub name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingValue {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingTemplateValue {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "defaultValue")]
    pub default_value: String,
    #[serde(rename = "description")]
    pub description: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplexExtensionValue {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionSchemaProperty {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomTimeZone {
    #[serde(rename = "bias")]
    pub bias: i32,
    #[serde(rename = "standardOffset")]
    pub standard_offset: StandardTimeZoneOffset,
    #[serde(rename = "daylightOffset")]
    pub daylight_offset: DaylightTimeZoneOffset,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandardTimeZoneOffset {
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "dayOccurrence")]
    pub day_occurrence: i32,
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: DayOfWeek,
    #[serde(rename = "month")]
    pub month: i32,
    #[serde(rename = "year")]
    pub year: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DaylightTimeZoneOffset {
    #[serde(rename = "daylightBias")]
    pub daylight_bias: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recipient {
    #[serde(rename = "emailAddress")]
    pub email_address: EmailAddress,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailAddress {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "address")]
    pub address: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttendeeBase {
    #[serde(rename = "type")]
    pub _type: AttendeeType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "locationEmailAddress")]
    pub location_email_address: String,
    #[serde(rename = "address")]
    pub address: PhysicalAddress,
    #[serde(rename = "coordinates")]
    pub coordinates: OutlookGeoCoordinates,
    #[serde(rename = "locationUri")]
    pub location_uri: String,
    #[serde(rename = "locationType")]
    pub location_type: LocationType,
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    #[serde(rename = "uniqueIdType")]
    pub unique_id_type: LocationUniqueIdType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicalAddress {
    #[serde(rename = "street")]
    pub street: String,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "countryOrRegion")]
    pub country_or_region: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutlookGeoCoordinates {
    #[serde(rename = "altitude")]
    pub altitude: i64,
    #[serde(rename = "latitude")]
    pub latitude: i64,
    #[serde(rename = "longitude")]
    pub longitude: i64,
    #[serde(rename = "accuracy")]
    pub accuracy: i64,
    #[serde(rename = "altitudeAccuracy")]
    pub altitude_accuracy: i64,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "eventStartTime")]
    pub event_start_time: String,
    #[serde(rename = "eventEndTime")]
    pub event_end_time: String,
    #[serde(rename = "changeKey")]
    pub change_key: String,
    #[serde(rename = "eventSubject")]
    pub event_subject: String,
    #[serde(rename = "eventLocation")]
    pub event_location: Location,
    #[serde(rename = "eventWebLink")]
    pub event_web_link: String,
    #[serde(rename = "reminderFireTime")]
    pub reminder_fire_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MailTips {
    #[serde(rename = "emailAddress")]
    pub email_address: EmailAddress,
    #[serde(rename = "automaticReplies")]
    pub automatic_replies: AutomaticRepliesMailTips,
    #[serde(rename = "mailboxFull")]
    pub mailbox_full: bool,
    #[serde(rename = "customMailTip")]
    pub custom_mail_tip: String,
    #[serde(rename = "externalMemberCount")]
    pub external_member_count: i32,
    #[serde(rename = "totalMemberCount")]
    pub total_member_count: i32,
    #[serde(rename = "deliveryRestricted")]
    pub delivery_restricted: bool,
    #[serde(rename = "isModerated")]
    pub is_moderated: bool,
    #[serde(rename = "recipientScope")]
    pub recipient_scope: RecipientScopeType,
    #[serde(rename = "recipientSuggestions")]
    pub recipient_suggestions: Vec<Recipient>,
    #[serde(rename = "maxMessageSize")]
    pub max_message_size: i32,
    #[serde(rename = "error")]
    pub error: MailTipsError,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomaticRepliesMailTips {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "messageLanguage")]
    pub message_language: LocaleInfo,
    #[serde(rename = "scheduledStartTime")]
    pub scheduled_start_time: String,
    #[serde(rename = "scheduledEndTime")]
    pub scheduled_end_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MailTipsError {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "code")]
    pub code: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneInformation {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternetMessageHeader {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemBody {
    #[serde(rename = "contentType")]
    pub content_type: BodyType,
    #[serde(rename = "content")]
    pub content: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FollowupFlag {
    #[serde(rename = "completedDateTime")]
    pub completed_date_time: String,
    #[serde(rename = "dueDateTime")]
    pub due_date_time: String,
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,
    #[serde(rename = "flagStatus")]
    pub flag_status: FollowupFlagStatus,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleInformation {
    #[serde(rename = "scheduleId")]
    pub schedule_id: String,
    #[serde(rename = "scheduleItems")]
    pub schedule_items: Vec<ScheduleItem>,
    #[serde(rename = "availabilityView")]
    pub availability_view: String,
    #[serde(rename = "error")]
    pub error: FreeBusyError,
    #[serde(rename = "workingHours")]
    pub working_hours: WorkingHours,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleItem {
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "status")]
    pub status: FreeBusyStatus,
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "location")]
    pub location: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FreeBusyError {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "responseCode")]
    pub response_code: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseStatus {
    #[serde(rename = "response")]
    pub response: ResponseType,
    #[serde(rename = "time")]
    pub time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternedRecurrence {
    #[serde(rename = "pattern")]
    pub pattern: RecurrencePattern,
    #[serde(rename = "range")]
    pub range: RecurrenceRange,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrencePattern {
    #[serde(rename = "type")]
    pub _type: RecurrencePatternType,
    #[serde(rename = "interval")]
    pub interval: i32,
    #[serde(rename = "month")]
    pub month: i32,
    #[serde(rename = "dayOfMonth")]
    pub day_of_month: i32,
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Vec<DayOfWeek>,
    #[serde(rename = "firstDayOfWeek")]
    pub first_day_of_week: DayOfWeek,
    #[serde(rename = "index")]
    pub index: WeekIndex,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrenceRange {
    #[serde(rename = "type")]
    pub _type: RecurrenceRangeType,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "recurrenceTimeZone")]
    pub recurrence_time_zone: String,
    #[serde(rename = "numberOfOccurrences")]
    pub number_of_occurrences: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attendee {
    #[serde(rename = "status")]
    pub status: ResponseStatus,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageRulePredicates {
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    #[serde(rename = "subjectContains")]
    pub subject_contains: Vec<String>,
    #[serde(rename = "bodyContains")]
    pub body_contains: Vec<String>,
    #[serde(rename = "bodyOrSubjectContains")]
    pub body_or_subject_contains: Vec<String>,
    #[serde(rename = "senderContains")]
    pub sender_contains: Vec<String>,
    #[serde(rename = "recipientContains")]
    pub recipient_contains: Vec<String>,
    #[serde(rename = "headerContains")]
    pub header_contains: Vec<String>,
    #[serde(rename = "messageActionFlag")]
    pub message_action_flag: MessageActionFlag,
    #[serde(rename = "importance")]
    pub importance: Importance,
    #[serde(rename = "sensitivity")]
    pub sensitivity: Sensitivity,
    #[serde(rename = "fromAddresses")]
    pub from_addresses: Vec<Recipient>,
    #[serde(rename = "sentToAddresses")]
    pub sent_to_addresses: Vec<Recipient>,
    #[serde(rename = "sentToMe")]
    pub sent_to_me: bool,
    #[serde(rename = "sentOnlyToMe")]
    pub sent_only_to_me: bool,
    #[serde(rename = "sentCcMe")]
    pub sent_cc_me: bool,
    #[serde(rename = "sentToOrCcMe")]
    pub sent_to_or_cc_me: bool,
    #[serde(rename = "notSentToMe")]
    pub not_sent_to_me: bool,
    #[serde(rename = "hasAttachments")]
    pub has_attachments: bool,
    #[serde(rename = "isApprovalRequest")]
    pub is_approval_request: bool,
    #[serde(rename = "isAutomaticForward")]
    pub is_automatic_forward: bool,
    #[serde(rename = "isAutomaticReply")]
    pub is_automatic_reply: bool,
    #[serde(rename = "isEncrypted")]
    pub is_encrypted: bool,
    #[serde(rename = "isMeetingRequest")]
    pub is_meeting_request: bool,
    #[serde(rename = "isMeetingResponse")]
    pub is_meeting_response: bool,
    #[serde(rename = "isNonDeliveryReport")]
    pub is_non_delivery_report: bool,
    #[serde(rename = "isPermissionControlled")]
    pub is_permission_controlled: bool,
    #[serde(rename = "isReadReceipt")]
    pub is_read_receipt: bool,
    #[serde(rename = "isSigned")]
    pub is_signed: bool,
    #[serde(rename = "isVoicemail")]
    pub is_voicemail: bool,
    #[serde(rename = "withinSizeRange")]
    pub within_size_range: SizeRange,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeRange {
    #[serde(rename = "minimumSize")]
    pub minimum_size: i32,
    #[serde(rename = "maximumSize")]
    pub maximum_size: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageRuleActions {
    #[serde(rename = "moveToFolder")]
    pub move_to_folder: String,
    #[serde(rename = "copyToFolder")]
    pub copy_to_folder: String,
    #[serde(rename = "delete")]
    pub delete: bool,
    #[serde(rename = "permanentDelete")]
    pub permanent_delete: bool,
    #[serde(rename = "markAsRead")]
    pub mark_as_read: bool,
    #[serde(rename = "markImportance")]
    pub mark_importance: Importance,
    #[serde(rename = "forwardTo")]
    pub forward_to: Vec<Recipient>,
    #[serde(rename = "forwardAsAttachmentTo")]
    pub forward_as_attachment_to: Vec<Recipient>,
    #[serde(rename = "redirectTo")]
    pub redirect_to: Vec<Recipient>,
    #[serde(rename = "assignCategories")]
    pub assign_categories: Vec<String>,
    #[serde(rename = "stopProcessingRules")]
    pub stop_processing_rules: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoredEmailAddress {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "relevanceScore")]
    pub relevance_score: i64,
    #[serde(rename = "selectionLikelihood")]
    pub selection_likelihood: SelectionLikelihoodInfo,
    #[serde(rename = "ItemId")]
    pub _item_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Phone {
    #[serde(rename = "type")]
    pub _type: PhoneType,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "language")]
    pub language: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Website {
    #[serde(rename = "type")]
    pub _type: WebsiteType,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonType {
    #[serde(rename = "class")]
    pub class: String,
    #[serde(rename = "subclass")]
    pub subclass: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationConstraint {
    #[serde(rename = "locations")]
    pub locations: Vec<LocationConstraintItem>,
    #[serde(rename = "isRequired")]
    pub is_required: bool,
    #[serde(rename = "suggestLocation")]
    pub suggest_location: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationConstraintItem {
    #[serde(rename = "resolveAvailability")]
    pub resolve_availability: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeetingTimeSuggestionsResult {
    #[serde(rename = "meetingTimeSuggestions")]
    pub meeting_time_suggestions: Vec<MeetingTimeSuggestion>,
    #[serde(rename = "emptySuggestionsReason")]
    pub empty_suggestions_reason: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeetingTimeSuggestion {
    #[serde(rename = "confidence")]
    pub confidence: i64,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "organizerAvailability")]
    pub organizer_availability: FreeBusyStatus,
    #[serde(rename = "attendeeAvailability")]
    pub attendee_availability: Vec<AttendeeAvailability>,
    #[serde(rename = "locations")]
    pub locations: Vec<Location>,
    #[serde(rename = "suggestionReason")]
    pub suggestion_reason: String,
    #[serde(rename = "meetingTimeSlot")]
    pub meeting_time_slot: TimeSlot,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttendeeAvailability {
    #[serde(rename = "attendee")]
    pub attendee: AttendeeBase,
    #[serde(rename = "availability")]
    pub availability: FreeBusyStatus,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSlot {
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end")]
    pub end: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeConstraint {
    #[serde(rename = "activityDomain")]
    pub activity_domain: ActivityDomain,
    #[serde(rename = "timeSlots")]
    pub time_slots: Vec<TimeSlot>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitySet {
    #[serde(rename = "application")]
    pub application: Identity,
    #[serde(rename = "device")]
    pub device: Identity,
    #[serde(rename = "user")]
    pub user: Identity,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "id")]
    pub id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemReference {
    #[serde(rename = "driveId")]
    pub drive_id: String,
    #[serde(rename = "driveType")]
    pub drive_type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "shareId")]
    pub share_id: String,
    #[serde(rename = "sharepointIds")]
    pub sharepoint_ids: SharepointIds,
    #[serde(rename = "siteId")]
    pub site_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharepointIds {
    #[serde(rename = "listId")]
    pub list_id: String,
    #[serde(rename = "listItemId")]
    pub list_item_id: String,
    #[serde(rename = "listItemUniqueId")]
    pub list_item_unique_id: String,
    #[serde(rename = "siteId")]
    pub site_id: String,
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    #[serde(rename = "webId")]
    pub web_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicationFacet {
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "versionId")]
    pub version_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanColumn {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculatedColumn {
    #[serde(rename = "format")]
    pub format: String,
    #[serde(rename = "formula")]
    pub formula: String,
    #[serde(rename = "outputType")]
    pub output_type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChoiceColumn {
    #[serde(rename = "allowTextEntry")]
    pub allow_text_entry: bool,
    #[serde(rename = "choices")]
    pub choices: Vec<String>,
    #[serde(rename = "displayAs")]
    pub display_as: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrencyColumn {
    #[serde(rename = "locale")]
    pub locale: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateTimeColumn {
    #[serde(rename = "displayAs")]
    pub display_as: String,
    #[serde(rename = "format")]
    pub format: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultColumnValue {
    #[serde(rename = "formula")]
    pub formula: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LookupColumn {
    #[serde(rename = "allowMultipleValues")]
    pub allow_multiple_values: bool,
    #[serde(rename = "allowUnlimitedLength")]
    pub allow_unlimited_length: bool,
    #[serde(rename = "columnName")]
    pub column_name: String,
    #[serde(rename = "listId")]
    pub list_id: String,
    #[serde(rename = "primaryLookupColumnId")]
    pub primary_lookup_column_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumberColumn {
    #[serde(rename = "decimalPlaces")]
    pub decimal_places: String,
    #[serde(rename = "displayAs")]
    pub display_as: String,
    #[serde(rename = "maximum")]
    pub maximum: i64,
    #[serde(rename = "minimum")]
    pub minimum: i64,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonOrGroupColumn {
    #[serde(rename = "allowMultipleSelection")]
    pub allow_multiple_selection: bool,
    #[serde(rename = "chooseFromType")]
    pub choose_from_type: String,
    #[serde(rename = "displayAs")]
    pub display_as: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextColumn {
    #[serde(rename = "allowMultipleLines")]
    pub allow_multiple_lines: bool,
    #[serde(rename = "appendChangesToExistingText")]
    pub append_changes_to_existing_text: bool,
    #[serde(rename = "linesForEditing")]
    pub lines_for_editing: i32,
    #[serde(rename = "maxLength")]
    pub max_length: i32,
    #[serde(rename = "textType")]
    pub text_type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentTypeOrder {
    #[serde(rename = "default")]
    pub default: bool,
    #[serde(rename = "position")]
    pub position: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quota {
    #[serde(rename = "deleted")]
    pub deleted: i64,
    #[serde(rename = "remaining")]
    pub remaining: i64,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "used")]
    pub used: i64,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemFacet {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    #[serde(rename = "album")]
    pub album: String,
    #[serde(rename = "albumArtist")]
    pub album_artist: String,
    #[serde(rename = "artist")]
    pub artist: String,
    #[serde(rename = "bitrate")]
    pub bitrate: i64,
    #[serde(rename = "composers")]
    pub composers: String,
    #[serde(rename = "copyright")]
    pub copyright: String,
    #[serde(rename = "disc")]
    pub disc: i16,
    #[serde(rename = "discCount")]
    pub disc_count: i16,
    #[serde(rename = "duration")]
    pub duration: i64,
    #[serde(rename = "genre")]
    pub genre: String,
    #[serde(rename = "hasDrm")]
    pub has_drm: bool,
    #[serde(rename = "isVariableBitrate")]
    pub is_variable_bitrate: bool,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "track")]
    pub track: i32,
    #[serde(rename = "trackCount")]
    pub track_count: i32,
    #[serde(rename = "year")]
    pub year: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deleted {
    #[serde(rename = "state")]
    pub state: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "hashes")]
    pub hashes: Hashes,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "processingMetadata")]
    pub processing_metadata: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hashes {
    #[serde(rename = "crc32Hash")]
    pub crc32_hash: String,
    #[serde(rename = "quickXorHash")]
    pub quick_xor_hash: String,
    #[serde(rename = "sha1Hash")]
    pub sha1_hash: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSystemInfo {
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastAccessedDateTime")]
    pub last_accessed_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Folder {
    #[serde(rename = "childCount")]
    pub child_count: i32,
    #[serde(rename = "view")]
    pub view: FolderView,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FolderView {
    #[serde(rename = "sortBy")]
    pub sort_by: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: String,
    #[serde(rename = "viewType")]
    pub view_type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "width")]
    pub width: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoCoordinates {
    #[serde(rename = "altitude")]
    pub altitude: i64,
    #[serde(rename = "latitude")]
    pub latitude: i64,
    #[serde(rename = "longitude")]
    pub longitude: i64,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "type")]
    pub _type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Photo {
    #[serde(rename = "cameraMake")]
    pub camera_make: String,
    #[serde(rename = "cameraModel")]
    pub camera_model: String,
    #[serde(rename = "exposureDenominator")]
    pub exposure_denominator: i64,
    #[serde(rename = "exposureNumerator")]
    pub exposure_numerator: i64,
    #[serde(rename = "fNumber")]
    pub f_number: i64,
    #[serde(rename = "focalLength")]
    pub focal_length: i64,
    #[serde(rename = "iso")]
    pub iso: i32,
    #[serde(rename = "takenDateTime")]
    pub taken_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteItem {
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "file")]
    pub file: File,
    #[serde(rename = "fileSystemInfo")]
    pub file_system_info: FileSystemInfo,
    #[serde(rename = "folder")]
    pub folder: Folder,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: IdentitySet,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "package")]
    pub package: Package,
    #[serde(rename = "parentReference")]
    pub parent_reference: ItemReference,
    #[serde(rename = "shared")]
    pub shared: Shared,
    #[serde(rename = "sharepointIds")]
    pub sharepoint_ids: SharepointIds,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "specialFolder")]
    pub special_folder: SpecialFolder,
    #[serde(rename = "webDavUrl")]
    pub web_dav_url: String,
    #[serde(rename = "webUrl")]
    pub web_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Shared {
    #[serde(rename = "owner")]
    pub owner: IdentitySet,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "sharedBy")]
    pub shared_by: IdentitySet,
    #[serde(rename = "sharedDateTime")]
    pub shared_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecialFolder {
    #[serde(rename = "name")]
    pub name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Root {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "onClickTelemetryUrl")]
    pub on_click_telemetry_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Video {
    #[serde(rename = "audioBitsPerSample")]
    pub audio_bits_per_sample: i32,
    #[serde(rename = "audioChannels")]
    pub audio_channels: i32,
    #[serde(rename = "audioFormat")]
    pub audio_format: String,
    #[serde(rename = "audioSamplesPerSecond")]
    pub audio_samples_per_second: i32,
    #[serde(rename = "bitrate")]
    pub bitrate: i32,
    #[serde(rename = "duration")]
    pub duration: i64,
    #[serde(rename = "fourCC")]
    pub four_c_c: String,
    #[serde(rename = "frameRate")]
    pub frame_rate: i64,
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "width")]
    pub width: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessAction {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemActionStat {
    #[serde(rename = "actionCount")]
    pub action_count: i32,
    #[serde(rename = "actorCount")]
    pub actor_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncompleteData {
    #[serde(rename = "missingDataBeforeDateTime")]
    pub missing_data_before_date_time: String,
    #[serde(rename = "wasThrottled")]
    pub was_throttled: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListInfo {
    #[serde(rename = "contentTypesEnabled")]
    pub content_types_enabled: bool,
    #[serde(rename = "hidden")]
    pub hidden: bool,
    #[serde(rename = "template")]
    pub template: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentTypeInfo {
    #[serde(rename = "id")]
    pub id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharingInvitation {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "invitedBy")]
    pub invited_by: IdentitySet,
    #[serde(rename = "redeemedBy")]
    pub redeemed_by: String,
    #[serde(rename = "signInRequired")]
    pub sign_in_required: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharingLink {
    #[serde(rename = "application")]
    pub application: Identity,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "webUrl")]
    pub web_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteCollection {
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "root")]
    pub root: Root,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Thumbnail {
    #[serde(rename = "content")]
    pub content: Vec<u8>,
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "sourceItemId")]
    pub source_item_id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "width")]
    pub width: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriveItemUploadableProperties {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "fileSystemInfo")]
    pub file_system_info: FileSystemInfo,
    #[serde(rename = "name")]
    pub name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriveRecipient {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "objectId")]
    pub object_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemPreviewInfo {
    #[serde(rename = "getUrl")]
    pub get_url: String,
    #[serde(rename = "postParameters")]
    pub post_parameters: String,
    #[serde(rename = "postUrl")]
    pub post_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadSession {
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "nextExpectedRanges")]
    pub next_expected_ranges: Vec<String>,
    #[serde(rename = "uploadUrl")]
    pub upload_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookSessionInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "persistChanges")]
    pub persist_changes: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Json {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFilterCriteria {
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "criterion1")]
    pub criterion1: String,
    #[serde(rename = "criterion2")]
    pub criterion2: String,
    #[serde(rename = "dynamicCriteria")]
    pub dynamic_criteria: String,
    #[serde(rename = "filterOn")]
    pub filter_on: String,
    #[serde(rename = "icon")]
    pub icon: WorkbookIcon,
    #[serde(rename = "operator")]
    pub operator: String,
    #[serde(rename = "values")]
    pub values: Json,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookIcon {
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "set")]
    pub set: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookSortField {
    #[serde(rename = "ascending")]
    pub ascending: bool,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "dataOption")]
    pub data_option: String,
    #[serde(rename = "icon")]
    pub icon: WorkbookIcon,
    #[serde(rename = "key")]
    pub key: i32,
    #[serde(rename = "sortOn")]
    pub sort_on: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookWorksheetProtectionOptions {
    #[serde(rename = "allowAutoFilter")]
    pub allow_auto_filter: bool,
    #[serde(rename = "allowDeleteColumns")]
    pub allow_delete_columns: bool,
    #[serde(rename = "allowDeleteRows")]
    pub allow_delete_rows: bool,
    #[serde(rename = "allowFormatCells")]
    pub allow_format_cells: bool,
    #[serde(rename = "allowFormatColumns")]
    pub allow_format_columns: bool,
    #[serde(rename = "allowFormatRows")]
    pub allow_format_rows: bool,
    #[serde(rename = "allowInsertColumns")]
    pub allow_insert_columns: bool,
    #[serde(rename = "allowInsertHyperlinks")]
    pub allow_insert_hyperlinks: bool,
    #[serde(rename = "allowInsertRows")]
    pub allow_insert_rows: bool,
    #[serde(rename = "allowPivotTables")]
    pub allow_pivot_tables: bool,
    #[serde(rename = "allowSort")]
    pub allow_sort: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFilterDatetime {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "specificity")]
    pub specificity: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeReference {
    #[serde(rename = "address")]
    pub address: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvitedUserMessageInfo {
    #[serde(rename = "ccRecipients")]
    pub cc_recipients: Vec<Recipient>,
    #[serde(rename = "messageLanguage")]
    pub message_language: String,
    #[serde(rename = "customizedMessageBody")]
    pub customized_message_body: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerAppliedCategories {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerAssignments {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerExternalReference {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "previewPriority")]
    pub preview_priority: String,
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: IdentitySet,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerChecklistItem {
    #[serde(rename = "isChecked")]
    pub is_checked: bool,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "orderHint")]
    pub order_hint: String,
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: IdentitySet,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerAssignment {
    #[serde(rename = "assignedBy")]
    pub assigned_by: IdentitySet,
    #[serde(rename = "assignedDateTime")]
    pub assigned_date_time: String,
    #[serde(rename = "orderHint")]
    pub order_hint: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerExternalReferences {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerChecklistItems {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerOrderHintsByAssignee {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerUserIds {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerCategoryDescriptions {
    #[serde(rename = "category1")]
    pub category1: String,
    #[serde(rename = "category2")]
    pub category2: String,
    #[serde(rename = "category3")]
    pub category3: String,
    #[serde(rename = "category4")]
    pub category4: String,
    #[serde(rename = "category5")]
    pub category5: String,
    #[serde(rename = "category6")]
    pub category6: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookLinks {
    #[serde(rename = "oneNoteClientUrl")]
    pub one_note_client_url: ExternalLink,
    #[serde(rename = "oneNoteWebUrl")]
    pub one_note_web_url: ExternalLink,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalLink {
    #[serde(rename = "href")]
    pub href: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SectionLinks {
    #[serde(rename = "oneNoteClientUrl")]
    pub one_note_client_url: ExternalLink,
    #[serde(rename = "oneNoteWebUrl")]
    pub one_note_web_url: ExternalLink,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageLinks {
    #[serde(rename = "oneNoteClientUrl")]
    pub one_note_client_url: ExternalLink,
    #[serde(rename = "oneNoteWebUrl")]
    pub one_note_web_url: ExternalLink,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenoteOperationError {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "url")]
    pub url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenotePatchContentCommand {
    #[serde(rename = "action")]
    pub action: OnenotePatchActionType,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "position")]
    pub position: OnenotePatchInsertPosition,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenotePagePreview {
    #[serde(rename = "previewText")]
    pub preview_text: String,
    #[serde(rename = "links")]
    pub links: OnenotePagePreviewLinks,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenotePagePreviewLinks {
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: ExternalLink,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecentNotebook {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "lastAccessedTime")]
    pub last_accessed_time: String,
    #[serde(rename = "links")]
    pub links: RecentNotebookLinks,
    #[serde(rename = "sourceService")]
    pub source_service: OnenoteSourceService,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecentNotebookLinks {
    #[serde(rename = "oneNoteClientUrl")]
    pub one_note_client_url: ExternalLink,
    #[serde(rename = "oneNoteWebUrl")]
    pub one_note_web_url: ExternalLink,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopyNotebookModel {
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "userRole")]
    pub user_role: OnenoteUserRole,
    #[serde(rename = "isShared")]
    pub is_shared: bool,
    #[serde(rename = "sectionsUrl")]
    pub sections_url: String,
    #[serde(rename = "sectionGroupsUrl")]
    pub section_groups_url: String,
    #[serde(rename = "links")]
    pub links: NotebookLinks,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "createdByIdentity")]
    pub created_by_identity: IdentitySet,
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: String,
    #[serde(rename = "lastModifiedByIdentity")]
    pub last_modified_by_identity: IdentitySet,
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "self")]
    pub _self: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "content")]
    pub content: Vec<u8>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EducationStudent {
    #[serde(rename = "graduationYear")]
    pub graduation_year: String,
    #[serde(rename = "grade")]
    pub grade: String,
    #[serde(rename = "birthDate")]
    pub birth_date: String,
    #[serde(rename = "gender")]
    pub gender: EducationGender,
    #[serde(rename = "studentNumber")]
    pub student_number: String,
    #[serde(rename = "externalId")]
    pub external_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EducationTeacher {
    #[serde(rename = "teacherNumber")]
    pub teacher_number: String,
    #[serde(rename = "externalId")]
    pub external_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EducationTerm {
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAndAppManagementAssignmentTarget {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileAppAssignmentSettings {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MimeContent {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "value")]
    pub value: Vec<u8>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileEncryptionInfo {
    #[serde(rename = "encryptionKey")]
    pub encryption_key: Vec<u8>,
    #[serde(rename = "initializationVector")]
    pub initialization_vector: Vec<u8>,
    #[serde(rename = "mac")]
    pub mac: Vec<u8>,
    #[serde(rename = "macKey")]
    pub mac_key: Vec<u8>,
    #[serde(rename = "profileIdentifier")]
    pub profile_identifier: String,
    #[serde(rename = "fileDigest")]
    pub file_digest: Vec<u8>,
    #[serde(rename = "fileDigestAlgorithm")]
    pub file_digest_algorithm: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllLicensedUsersAssignmentTarget {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAssignmentTarget {
    #[serde(rename = "groupId")]
    pub group_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExclusionGroupAssignmentTarget {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllDevicesAssignmentTarget {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosLobAppAssignmentSettings {
    #[serde(rename = "vpnConfigurationId")]
    pub vpn_configuration_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosStoreAppAssignmentSettings {
    #[serde(rename = "vpnConfigurationId")]
    pub vpn_configuration_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosVppAppAssignmentSettings {
    #[serde(rename = "useDeviceLicensing")]
    pub use_device_licensing: bool,
    #[serde(rename = "vpnConfigurationId")]
    pub vpn_configuration_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftStoreForBusinessAppAssignmentSettings {
    #[serde(rename = "useDeviceContext")]
    pub use_device_context: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidMinimumOperatingSystem {
    #[serde(rename = "v4_0")]
    pub v4_0: bool,
    #[serde(rename = "v4_0_3")]
    pub v4_0_3: bool,
    #[serde(rename = "v4_1")]
    pub v4_1: bool,
    #[serde(rename = "v4_2")]
    pub v4_2: bool,
    #[serde(rename = "v4_3")]
    pub v4_3: bool,
    #[serde(rename = "v4_4")]
    pub v4_4: bool,
    #[serde(rename = "v5_0")]
    pub v5_0: bool,
    #[serde(rename = "v5_1")]
    pub v5_1: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosDeviceType {
    #[serde(rename = "iPad")]
    pub i_pad: bool,
    #[serde(rename = "iPhoneAndIPod")]
    pub i_phone_and_i_pod: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosMinimumOperatingSystem {
    #[serde(rename = "v8_0")]
    pub v8_0: bool,
    #[serde(rename = "v9_0")]
    pub v9_0: bool,
    #[serde(rename = "v10_0")]
    pub v10_0: bool,
    #[serde(rename = "v11_0")]
    pub v11_0: bool,
    #[serde(rename = "v12_0")]
    pub v12_0: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsMinimumOperatingSystem {
    #[serde(rename = "v8_0")]
    pub v8_0: bool,
    #[serde(rename = "v8_1")]
    pub v8_1: bool,
    #[serde(rename = "v10_0")]
    pub v10_0: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VppLicensingType {
    #[serde(rename = "supportsUserLicensing")]
    pub supports_user_licensing: bool,
    #[serde(rename = "supportsDeviceLicensing")]
    pub supports_device_licensing: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfigurationSettingItem {
    #[serde(rename = "appConfigKey")]
    pub app_config_key: String,
    #[serde(rename = "appConfigKeyType")]
    pub app_config_key_type: MdmAppConfigKeyType,
    #[serde(rename = "appConfigKeyValue")]
    pub app_config_key_value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagementSettings {
    #[serde(rename = "deviceComplianceCheckinThresholdDays")]
    pub device_compliance_checkin_threshold_days: i32,
    #[serde(rename = "isScheduledActionEnabled")]
    pub is_scheduled_action_enabled: bool,
    #[serde(rename = "secureByDefault")]
    pub secure_by_default: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntuneBrand {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "contactITName")]
    pub contact_i_t_name: String,
    #[serde(rename = "contactITPhoneNumber")]
    pub contact_i_t_phone_number: String,
    #[serde(rename = "contactITEmailAddress")]
    pub contact_i_t_email_address: String,
    #[serde(rename = "contactITNotes")]
    pub contact_i_t_notes: String,
    #[serde(rename = "privacyUrl")]
    pub privacy_url: String,
    #[serde(rename = "onlineSupportSiteUrl")]
    pub online_support_site_url: String,
    #[serde(rename = "onlineSupportSiteName")]
    pub online_support_site_name: String,
    #[serde(rename = "themeColor")]
    pub theme_color: RgbColor,
    #[serde(rename = "showLogo")]
    pub show_logo: bool,
    #[serde(rename = "lightBackgroundLogo")]
    pub light_background_logo: MimeContent,
    #[serde(rename = "darkBackgroundLogo")]
    pub dark_background_logo: MimeContent,
    #[serde(rename = "showNameNextToLogo")]
    pub show_name_next_to_logo: bool,
    #[serde(rename = "showDisplayNameNextToLogo")]
    pub show_display_name_next_to_logo: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RgbColor {
    #[serde(rename = "r")]
    pub r: u8,
    #[serde(rename = "g")]
    pub g: u8,
    #[serde(rename = "b")]
    pub b: u8,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceActionResult {
    #[serde(rename = "actionName")]
    pub action_name: String,
    #[serde(rename = "actionState")]
    pub action_state: ActionState,
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,
    #[serde(rename = "lastUpdatedDateTime")]
    pub last_updated_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationManagerClientEnabledFeatures {
    #[serde(rename = "inventory")]
    pub inventory: bool,
    #[serde(rename = "modernApps")]
    pub modern_apps: bool,
    #[serde(rename = "resourceAccess")]
    pub resource_access: bool,
    #[serde(rename = "deviceConfiguration")]
    pub device_configuration: bool,
    #[serde(rename = "compliancePolicy")]
    pub compliance_policy: bool,
    #[serde(rename = "windowsUpdateForBusiness")]
    pub windows_update_for_business: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceHealthAttestationState {
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_date_time: String,
    #[serde(rename = "contentNamespaceUrl")]
    pub content_namespace_url: String,
    #[serde(rename = "deviceHealthAttestationStatus")]
    pub device_health_attestation_status: String,
    #[serde(rename = "contentVersion")]
    pub content_version: String,
    #[serde(rename = "issuedDateTime")]
    pub issued_date_time: String,
    #[serde(rename = "attestationIdentityKey")]
    pub attestation_identity_key: String,
    #[serde(rename = "resetCount")]
    pub reset_count: i64,
    #[serde(rename = "restartCount")]
    pub restart_count: i64,
    #[serde(rename = "dataExcutionPolicy")]
    pub data_excution_policy: String,
    #[serde(rename = "bitLockerStatus")]
    pub bit_locker_status: String,
    #[serde(rename = "bootManagerVersion")]
    pub boot_manager_version: String,
    #[serde(rename = "codeIntegrityCheckVersion")]
    pub code_integrity_check_version: String,
    #[serde(rename = "secureBoot")]
    pub secure_boot: String,
    #[serde(rename = "bootDebugging")]
    pub boot_debugging: String,
    #[serde(rename = "operatingSystemKernelDebugging")]
    pub operating_system_kernel_debugging: String,
    #[serde(rename = "codeIntegrity")]
    pub code_integrity: String,
    #[serde(rename = "testSigning")]
    pub test_signing: String,
    #[serde(rename = "safeMode")]
    pub safe_mode: String,
    #[serde(rename = "windowsPE")]
    pub windows_p_e: String,
    #[serde(rename = "earlyLaunchAntiMalwareDriverProtection")]
    pub early_launch_anti_malware_driver_protection: String,
    #[serde(rename = "virtualSecureMode")]
    pub virtual_secure_mode: String,
    #[serde(rename = "pcrHashAlgorithm")]
    pub pcr_hash_algorithm: String,
    #[serde(rename = "bootAppSecurityVersion")]
    pub boot_app_security_version: String,
    #[serde(rename = "bootManagerSecurityVersion")]
    pub boot_manager_security_version: String,
    #[serde(rename = "tpmVersion")]
    pub tpm_version: String,
    #[serde(rename = "pcr0")]
    pub pcr0: String,
    #[serde(rename = "secureBootConfigurationPolicyFingerPrint")]
    pub secure_boot_configuration_policy_finger_print: String,
    #[serde(rename = "codeIntegrityPolicy")]
    pub code_integrity_policy: String,
    #[serde(rename = "bootRevisionListInfo")]
    pub boot_revision_list_info: String,
    #[serde(rename = "operatingSystemRevListInfo")]
    pub operating_system_rev_list_info: String,
    #[serde(rename = "healthStatusMismatchInfo")]
    pub health_status_mismatch_info: String,
    #[serde(rename = "healthAttestationSupportedStatus")]
    pub health_attestation_supported_status: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWindowsDeviceAccountActionParameter {
    #[serde(rename = "deviceAccount")]
    pub device_account: WindowsDeviceAccount,
    #[serde(rename = "passwordRotationEnabled")]
    pub password_rotation_enabled: bool,
    #[serde(rename = "calendarSyncEnabled")]
    pub calendar_sync_enabled: bool,
    #[serde(rename = "deviceAccountEmail")]
    pub device_account_email: String,
    #[serde(rename = "exchangeServer")]
    pub exchange_server: String,
    #[serde(rename = "sessionInitiationProtocalAddress")]
    pub session_initiation_protocal_address: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsDeviceAccount {
    #[serde(rename = "password")]
    pub password: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsDefenderScanActionResult {
    #[serde(rename = "scanType")]
    pub scan_type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteUserFromSharedAppleDeviceActionResult {
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGeoLocation {
    #[serde(rename = "lastCollectedDateTime")]
    pub last_collected_date_time: String,
    #[serde(rename = "longitude")]
    pub longitude: i64,
    #[serde(rename = "latitude")]
    pub latitude: i64,
    #[serde(rename = "altitude")]
    pub altitude: i64,
    #[serde(rename = "horizontalAccuracy")]
    pub horizontal_accuracy: i64,
    #[serde(rename = "verticalAccuracy")]
    pub vertical_accuracy: i64,
    #[serde(rename = "heading")]
    pub heading: i64,
    #[serde(rename = "speed")]
    pub speed: i64,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocateDeviceActionResult {
    #[serde(rename = "deviceLocation")]
    pub device_location: DeviceGeoLocation,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteLockActionResult {
    #[serde(rename = "unlockPin")]
    pub unlock_pin: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetPasscodeActionResult {
    #[serde(rename = "passcode")]
    pub passcode: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceOperatingSystemSummary {
    #[serde(rename = "androidCount")]
    pub android_count: i32,
    #[serde(rename = "iosCount")]
    pub ios_count: i32,
    #[serde(rename = "macOSCount")]
    pub mac_o_s_count: i32,
    #[serde(rename = "windowsMobileCount")]
    pub windows_mobile_count: i32,
    #[serde(rename = "windowsCount")]
    pub windows_count: i32,
    #[serde(rename = "unknownCount")]
    pub unknown_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceExchangeAccessStateSummary {
    #[serde(rename = "allowedDeviceCount")]
    pub allowed_device_count: i32,
    #[serde(rename = "blockedDeviceCount")]
    pub blocked_device_count: i32,
    #[serde(rename = "quarantinedDeviceCount")]
    pub quarantined_device_count: i32,
    #[serde(rename = "unknownDeviceCount")]
    pub unknown_device_count: i32,
    #[serde(rename = "unavailableDeviceCount")]
    pub unavailable_device_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsDeviceADAccount {
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsDeviceAzureADAccount {
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppListItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "publisher")]
    pub publisher: String,
    #[serde(rename = "appStoreUrl")]
    pub app_store_url: String,
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OmaSetting {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "omaUri")]
    pub oma_uri: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingInteger {
    #[serde(rename = "value")]
    pub value: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingFloatingPoint {
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingString {
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingDateTime {
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingStringXml {
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "value")]
    pub value: Vec<u8>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingBoolean {
    #[serde(rename = "value")]
    pub value: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OmaSettingBase64 {
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingAustralia {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingAustraliaMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingAustraliaTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingCanada {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingCanadaMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingCanadaTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingFrance {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingFranceMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingFranceTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingGermany {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingGermanyMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingGermanyTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingIreland {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingIrelandMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingIrelandTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingJapan {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingJapanMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingJapanTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingNewZealand {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingNewZealandMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingNewZealandTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingUnitedKingdom {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingUnitedKingdomMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingUnitedKingdomTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContentRatingUnitedStates {
    #[serde(rename = "movieRating")]
    pub movie_rating: RatingUnitedStatesMoviesType,
    #[serde(rename = "tvRating")]
    pub tv_rating: RatingUnitedStatesTelevisionType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosNetworkUsageRule {
    #[serde(rename = "managedApps")]
    pub managed_apps: Vec<AppListItem>,
    #[serde(rename = "cellularDataBlockWhenRoaming")]
    pub cellular_data_block_when_roaming: bool,
    #[serde(rename = "cellularDataBlocked")]
    pub cellular_data_blocked: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenItem {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenPage {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "icons")]
    pub icons: Vec<IosHomeScreenItem>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosNotificationSettings {
    #[serde(rename = "bundleID")]
    pub bundle_i_d: String,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "publisher")]
    pub publisher: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "showInNotificationCenter")]
    pub show_in_notification_center: bool,
    #[serde(rename = "showOnLockScreen")]
    pub show_on_lock_screen: bool,
    #[serde(rename = "alertType")]
    pub alert_type: IosNotificationAlertType,
    #[serde(rename = "badgesEnabled")]
    pub badges_enabled: bool,
    #[serde(rename = "soundsEnabled")]
    pub sounds_enabled: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenFolder {
    #[serde(rename = "pages")]
    pub pages: Vec<IosHomeScreenFolderPage>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenFolderPage {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "apps")]
    pub apps: Vec<IosHomeScreenApp>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosHomeScreenApp {
    #[serde(rename = "bundleID")]
    pub bundle_i_d: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsFirewallNetworkProfile {
    #[serde(rename = "firewallEnabled")]
    pub firewall_enabled: StateManagementSetting,
    #[serde(rename = "stealthModeBlocked")]
    pub stealth_mode_blocked: bool,
    #[serde(rename = "incomingTrafficBlocked")]
    pub incoming_traffic_blocked: bool,
    #[serde(rename = "unicastResponsesToMulticastBroadcastsBlocked")]
    pub unicast_responses_to_multicast_broadcasts_blocked: bool,
    #[serde(rename = "inboundNotificationsBlocked")]
    pub inbound_notifications_blocked: bool,
    #[serde(rename = "authorizedApplicationRulesFromGroupPolicyMerged")]
    pub authorized_application_rules_from_group_policy_merged: bool,
    #[serde(rename = "globalPortRulesFromGroupPolicyMerged")]
    pub global_port_rules_from_group_policy_merged: bool,
    #[serde(rename = "connectionSecurityRulesFromGroupPolicyMerged")]
    pub connection_security_rules_from_group_policy_merged: bool,
    #[serde(rename = "outboundConnectionsBlocked")]
    pub outbound_connections_blocked: bool,
    #[serde(rename = "inboundConnectionsBlocked")]
    pub inbound_connections_blocked: bool,
    #[serde(rename = "securedPacketExemptionAllowed")]
    pub secured_packet_exemption_allowed: bool,
    #[serde(rename = "policyRulesFromGroupPolicyMerged")]
    pub policy_rules_from_group_policy_merged: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BitLockerRemovableDrivePolicy {
    #[serde(rename = "encryptionMethod")]
    pub encryption_method: BitLockerEncryptionMethod,
    #[serde(rename = "requireEncryptionForWriteAccess")]
    pub require_encryption_for_write_access: bool,
    #[serde(rename = "blockCrossOrganizationWriteAccess")]
    pub block_cross_organization_write_access: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderDetectedMalwareActions {
    #[serde(rename = "lowSeverity")]
    pub low_severity: DefenderThreatAction,
    #[serde(rename = "moderateSeverity")]
    pub moderate_severity: DefenderThreatAction,
    #[serde(rename = "highSeverity")]
    pub high_severity: DefenderThreatAction,
    #[serde(rename = "severeSeverity")]
    pub severe_severity: DefenderThreatAction,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10NetworkProxyServer {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "exceptions")]
    pub exceptions: Vec<String>,
    #[serde(rename = "useForLocalAddresses")]
    pub use_for_local_addresses: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeSearchEngineBase {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeSearchEngineCustom {
    #[serde(rename = "edgeSearchEngineOpenSearchXmlUrl")]
    pub edge_search_engine_open_search_xml_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeSearchEngine {
    #[serde(rename = "edgeSearchEngineType")]
    pub edge_search_engine_type: EdgeSearchEngineType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPCAccountManagerPolicy {
    #[serde(rename = "accountDeletionPolicy")]
    pub account_deletion_policy: SharedPCAccountDeletionPolicyType,
    #[serde(rename = "cacheAccountsAboveDiskFreePercentage")]
    pub cache_accounts_above_disk_free_percentage: i32,
    #[serde(rename = "inactiveThresholdDays")]
    pub inactive_threshold_days: i32,
    #[serde(rename = "removeAccountsBelowDiskFreePercentage")]
    pub remove_accounts_below_disk_free_percentage: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsUpdateInstallScheduleType {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsUpdateScheduledInstall {
    #[serde(rename = "scheduledInstallDay")]
    pub scheduled_install_day: WeeklySchedule,
    #[serde(rename = "scheduledInstallTime")]
    pub scheduled_install_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsUpdateActiveHoursInstall {
    #[serde(rename = "activeHoursStart")]
    pub active_hours_start: String,
    #[serde(rename = "activeHoursEnd")]
    pub active_hours_end: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationSettingState {
    #[serde(rename = "setting")]
    pub setting: String,
    #[serde(rename = "settingName")]
    pub setting_name: String,
    #[serde(rename = "instanceDisplayName")]
    pub instance_display_name: String,
    #[serde(rename = "state")]
    pub state: ComplianceStatus,
    #[serde(rename = "errorCode")]
    pub error_code: i64,
    #[serde(rename = "errorDescription")]
    pub error_description: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "sources")]
    pub sources: Vec<SettingSource>,
    #[serde(rename = "currentValue")]
    pub current_value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingSource {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicySettingState {
    #[serde(rename = "setting")]
    pub setting: String,
    #[serde(rename = "settingName")]
    pub setting_name: String,
    #[serde(rename = "instanceDisplayName")]
    pub instance_display_name: String,
    #[serde(rename = "state")]
    pub state: ComplianceStatus,
    #[serde(rename = "errorCode")]
    pub error_code: i64,
    #[serde(rename = "errorDescription")]
    pub error_description: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "sources")]
    pub sources: Vec<SettingSource>,
    #[serde(rename = "currentValue")]
    pub current_value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentPlatformRestriction {
    #[serde(rename = "platformBlocked")]
    pub platform_blocked: bool,
    #[serde(rename = "personalDeviceEnrollmentBlocked")]
    pub personal_device_enrollment_blocked: bool,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileAppIdentifier {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppDiagnosticStatus {
    #[serde(rename = "validationName")]
    pub validation_name: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "mitigationInstruction")]
    pub mitigation_instruction: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValuePair {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionResourceCollection {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionDataRecoveryCertificate {
    #[serde(rename = "subjectName")]
    pub subject_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "certificate")]
    pub certificate: Vec<u8>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionApp {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "publisherName")]
    pub publisher_name: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "denied")]
    pub denied: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionProxiedDomainCollection {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "proxiedDomains")]
    pub proxied_domains: Vec<ProxiedDomain>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxiedDomain {
    #[serde(rename = "ipAddressOrFQDN")]
    pub ip_address_or_f_q_d_n: String,
    #[serde(rename = "proxy")]
    pub proxy: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionIPRangeCollection {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "ranges")]
    pub ranges: Vec<IpRange>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRange {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidMobileAppIdentifier {
    #[serde(rename = "packageId")]
    pub package_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosMobileAppIdentifier {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppPolicyDeploymentSummaryPerApp {
    #[serde(rename = "mobileAppIdentifier")]
    pub mobile_app_identifier: MobileAppIdentifier,
    #[serde(rename = "configurationAppliedUserCount")]
    pub configuration_applied_user_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionStoreApp {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionDesktopApp {
    #[serde(rename = "binaryName")]
    pub binary_name: String,
    #[serde(rename = "binaryVersionLow")]
    pub binary_version_low: String,
    #[serde(rename = "binaryVersionHigh")]
    pub binary_version_high: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPv6Range {
    #[serde(rename = "lowerAddress")]
    pub lower_address: String,
    #[serde(rename = "upperAddress")]
    pub upper_address: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPv4Range {
    #[serde(rename = "lowerAddress")]
    pub lower_address: String,
    #[serde(rename = "upperAddress")]
    pub upper_address: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolePermission {
    #[serde(rename = "resourceActions")]
    pub resource_actions: Vec<ResourceAction>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceAction {
    #[serde(rename = "allowedResourceActions")]
    pub allowed_resource_actions: Vec<String>,
    #[serde(rename = "notAllowedResourceActions")]
    pub not_allowed_resource_actions: Vec<String>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "alternativeText")]
    pub alternative_text: String,
    #[serde(rename = "alternateText")]
    pub alternate_text: String,
    #[serde(rename = "addImageQuery")]
    pub add_image_query: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualInfo {
    #[serde(rename = "attribution")]
    pub attribution: ImageInfo,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayText")]
    pub display_text: String,
    #[serde(rename = "content")]
    pub content: Json,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudAppSecurityState {
    #[serde(rename = "destinationServiceIp")]
    pub destination_service_ip: String,
    #[serde(rename = "destinationServiceName")]
    pub destination_service_name: String,
    #[serde(rename = "riskScore")]
    pub risk_score: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSecurityState {
    #[serde(rename = "fileHash")]
    pub file_hash: FileHash,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "riskScore")]
    pub risk_score: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileHash {
    #[serde(rename = "hashType")]
    pub hash_type: FileHashType,
    #[serde(rename = "hashValue")]
    pub hash_value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertHistoryState {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "assignedTo")]
    pub assigned_to: String,
    #[serde(rename = "comments")]
    pub comments: Vec<String>,
    #[serde(rename = "feedback")]
    pub feedback: AlertFeedback,
    #[serde(rename = "status")]
    pub status: AlertStatus,
    #[serde(rename = "updatedDateTime")]
    pub updated_date_time: String,
    #[serde(rename = "user")]
    pub user: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostSecurityState {
    #[serde(rename = "fqdn")]
    pub fqdn: String,
    #[serde(rename = "isAzureAdJoined")]
    pub is_azure_ad_joined: bool,
    #[serde(rename = "isAzureAdRegistered")]
    pub is_azure_ad_registered: bool,
    #[serde(rename = "isHybridAzureDomainJoined")]
    pub is_hybrid_azure_domain_joined: bool,
    #[serde(rename = "netBiosName")]
    pub net_bios_name: String,
    #[serde(rename = "os")]
    pub os: String,
    #[serde(rename = "privateIpAddress")]
    pub private_ip_address: String,
    #[serde(rename = "publicIpAddress")]
    pub public_ip_address: String,
    #[serde(rename = "riskScore")]
    pub risk_score: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MalwareState {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "family")]
    pub family: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "wasRunning")]
    pub was_running: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConnection {
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[serde(rename = "destinationAddress")]
    pub destination_address: String,
    #[serde(rename = "destinationDomain")]
    pub destination_domain: String,
    #[serde(rename = "destinationPort")]
    pub destination_port: String,
    #[serde(rename = "destinationUrl")]
    pub destination_url: String,
    #[serde(rename = "direction")]
    pub direction: ConnectionDirection,
    #[serde(rename = "domainRegisteredDateTime")]
    pub domain_registered_date_time: String,
    #[serde(rename = "localDnsName")]
    pub local_dns_name: String,
    #[serde(rename = "natDestinationAddress")]
    pub nat_destination_address: String,
    #[serde(rename = "natDestinationPort")]
    pub nat_destination_port: String,
    #[serde(rename = "natSourceAddress")]
    pub nat_source_address: String,
    #[serde(rename = "natSourcePort")]
    pub nat_source_port: String,
    #[serde(rename = "protocol")]
    pub protocol: SecurityNetworkProtocol,
    #[serde(rename = "riskScore")]
    pub risk_score: String,
    #[serde(rename = "sourceAddress")]
    pub source_address: String,
    #[serde(rename = "sourcePort")]
    pub source_port: String,
    #[serde(rename = "status")]
    pub status: ConnectionStatus,
    #[serde(rename = "urlParameters")]
    pub url_parameters: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Process {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "commandLine")]
    pub command_line: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "fileHash")]
    pub file_hash: FileHash,
    #[serde(rename = "integrityLevel")]
    pub integrity_level: ProcessIntegrityLevel,
    #[serde(rename = "isElevated")]
    pub is_elevated: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentProcessCreatedDateTime")]
    pub parent_process_created_date_time: String,
    #[serde(rename = "parentProcessId")]
    pub parent_process_id: i32,
    #[serde(rename = "parentProcessName")]
    pub parent_process_name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "processId")]
    pub process_id: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryKeyState {
    #[serde(rename = "hive")]
    pub hive: RegistryHive,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "oldKey")]
    pub old_key: String,
    #[serde(rename = "oldValueData")]
    pub old_value_data: String,
    #[serde(rename = "oldValueName")]
    pub old_value_name: String,
    #[serde(rename = "operation")]
    pub operation: RegistryOperation,
    #[serde(rename = "processId")]
    pub process_id: i32,
    #[serde(rename = "valueData")]
    pub value_data: String,
    #[serde(rename = "valueName")]
    pub value_name: String,
    #[serde(rename = "valueType")]
    pub value_type: RegistryValueType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertTrigger {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSecurityState {
    #[serde(rename = "aadUserId")]
    pub aad_user_id: String,
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[serde(rename = "emailRole")]
    pub email_role: EmailRole,
    #[serde(rename = "isVpn")]
    pub is_vpn: bool,
    #[serde(rename = "logonDateTime")]
    pub logon_date_time: String,
    #[serde(rename = "logonId")]
    pub logon_id: String,
    #[serde(rename = "logonIp")]
    pub logon_ip: String,
    #[serde(rename = "logonLocation")]
    pub logon_location: String,
    #[serde(rename = "logonType")]
    pub logon_type: LogonType,
    #[serde(rename = "onPremisesSecurityIdentifier")]
    pub on_premises_security_identifier: String,
    #[serde(rename = "riskScore")]
    pub risk_score: String,
    #[serde(rename = "userAccountType")]
    pub user_account_type: UserAccountSecurityType,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityVendorInformation {
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "providerVersion")]
    pub provider_version: String,
    #[serde(rename = "subProvider")]
    pub sub_provider: String,
    #[serde(rename = "vendor")]
    pub vendor: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VulnerabilityState {
    #[serde(rename = "cve")]
    pub cve: String,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "wasRunning")]
    pub was_running: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AverageComparativeScore {
    #[serde(rename = "averageScore")]
    pub average_score: i64,
    #[serde(rename = "basis")]
    pub basis: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControlScore {
    #[serde(rename = "controlCategory")]
    pub control_category: String,
    #[serde(rename = "controlName")]
    pub control_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "score")]
    pub score: i64,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComplianceInformation {
    #[serde(rename = "certificationControls")]
    pub certification_controls: Vec<CertificationControl>,
    #[serde(rename = "certificationName")]
    pub certification_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificationControl {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecureScoreControlStateUpdate {
    #[serde(rename = "assignedTo")]
    pub assigned_to: String,
    #[serde(rename = "comment")]
    pub comment: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[serde(rename = "updatedDateTime")]
    pub updated_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceVisualization {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: String,
    #[serde(rename = "previewText")]
    pub preview_text: String,
    #[serde(rename = "containerWebUrl")]
    pub container_web_url: String,
    #[serde(rename = "containerDisplayName")]
    pub container_display_name: String,
    #[serde(rename = "containerType")]
    pub container_type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceReference {
    #[serde(rename = "webUrl")]
    pub web_url: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharingDetail {
    #[serde(rename = "sharedBy")]
    pub shared_by: InsightIdentity,
    #[serde(rename = "sharedDateTime")]
    pub shared_date_time: String,
    #[serde(rename = "sharingSubject")]
    pub sharing_subject: String,
    #[serde(rename = "sharingType")]
    pub sharing_type: String,
    #[serde(rename = "sharingReference")]
    pub sharing_reference: ResourceReference,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsightIdentity {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "address")]
    pub address: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDetails {
    #[serde(rename = "lastAccessedDateTime")]
    pub last_accessed_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamMemberSettings {
    #[serde(rename = "allowCreateUpdateChannels")]
    pub allow_create_update_channels: bool,
    #[serde(rename = "allowDeleteChannels")]
    pub allow_delete_channels: bool,
    #[serde(rename = "allowAddRemoveApps")]
    pub allow_add_remove_apps: bool,
    #[serde(rename = "allowCreateUpdateRemoveTabs")]
    pub allow_create_update_remove_tabs: bool,
    #[serde(rename = "allowCreateUpdateRemoveConnectors")]
    pub allow_create_update_remove_connectors: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamGuestSettings {
    #[serde(rename = "allowCreateUpdateChannels")]
    pub allow_create_update_channels: bool,
    #[serde(rename = "allowDeleteChannels")]
    pub allow_delete_channels: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamMessagingSettings {
    #[serde(rename = "allowUserEditMessages")]
    pub allow_user_edit_messages: bool,
    #[serde(rename = "allowUserDeleteMessages")]
    pub allow_user_delete_messages: bool,
    #[serde(rename = "allowOwnerDeleteMessages")]
    pub allow_owner_delete_messages: bool,
    #[serde(rename = "allowTeamMentions")]
    pub allow_team_mentions: bool,
    #[serde(rename = "allowChannelMentions")]
    pub allow_channel_mentions: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamFunSettings {
    #[serde(rename = "allowGiphy")]
    pub allow_giphy: bool,
    #[serde(rename = "giphyContentRating")]
    pub giphy_content_rating: GiphyRatingType,
    #[serde(rename = "allowStickersAndMemes")]
    pub allow_stickers_and_memes: bool,
    #[serde(rename = "allowCustomMemes")]
    pub allow_custom_memes: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamClassSettings {
    #[serde(rename = "notifyGuardiansAboutAssignments")]
    pub notify_guardians_about_assignments: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsTabConfiguration {
    #[serde(rename = "entityId")]
    pub entity_id: String,
    #[serde(rename = "contentUrl")]
    pub content_url: String,
    #[serde(rename = "removeUrl")]
    pub remove_url: String,
    #[serde(rename = "websiteUrl")]
    pub website_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationError {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditActivityInitiator {
    #[serde(rename = "user")]
    pub user: UserIdentity,
    #[serde(rename = "app")]
    pub app: AppIdentity,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentity {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppIdentity {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "servicePrincipalId")]
    pub service_principal_id: String,
    #[serde(rename = "servicePrincipalName")]
    pub service_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetResource {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
    #[serde(rename = "modifiedProperties")]
    pub modified_properties: Vec<ModifiedProperty>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifiedProperty {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "oldValue")]
    pub old_value: String,
    #[serde(rename = "newValue")]
    pub new_value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignInStatus {
    #[serde(rename = "errorCode")]
    pub error_code: i32,
    #[serde(rename = "failureReason")]
    pub failure_reason: String,
    #[serde(rename = "additionalDetails")]
    pub additional_details: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceDetail {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,
    #[serde(rename = "browser")]
    pub browser: String,
    #[serde(rename = "isCompliant")]
    pub is_compliant: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "trustType")]
    pub trust_type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignInLocation {
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "countryOrRegion")]
    pub country_or_region: String,
    #[serde(rename = "geoCoordinates")]
    pub geo_coordinates: GeoCoordinates,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedConditionalAccessPolicy {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "enforcedGrantControls")]
    pub enforced_grant_controls: Vec<String>,
    #[serde(rename = "enforcedSessionControls")]
    pub enforced_session_controls: Vec<String>,
    #[serde(rename = "result")]
    pub result: AppliedConditionalAccessPolicyResult,
}
