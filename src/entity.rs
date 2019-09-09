#[cfg(not(feature = "option"))]
use crate::complex::*;
#[cfg(not(feature = "option"))]
use crate::enumtypes::*;

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryObject {
    #[serde(rename = "deletedDateTime")]
    pub deleted_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdministrativeUnit {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Directory {
    #[serde(rename = "deletedItems")]
    pub deleted_items: Vec<DirectoryObject>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "accountEnabled")]
    pub account_enabled: bool,
    #[serde(rename = "alternativeSecurityIds")]
    pub alternative_security_ids: Vec<AlternativeSecurityId>,
    #[serde(rename = "approximateLastSignInDateTime")]
    pub approximate_last_sign_in_date_time: String,
    #[serde(rename = "complianceExpirationDateTime")]
    pub compliance_expiration_date_time: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "deviceMetadata")]
    pub device_metadata: String,
    #[serde(rename = "deviceVersion")]
    pub device_version: i32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isCompliant")]
    pub is_compliant: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "onPremisesLastSyncDateTime")]
    pub on_premises_last_sync_date_time: String,
    #[serde(rename = "onPremisesSyncEnabled")]
    pub on_premises_sync_enabled: bool,
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,
    #[serde(rename = "operatingSystemVersion")]
    pub operating_system_version: String,
    #[serde(rename = "physicalIds")]
    pub physical_ids: Vec<String>,
    #[serde(rename = "profileType")]
    pub profile_type: String,
    #[serde(rename = "systemLabels")]
    pub system_labels: Vec<String>,
    #[serde(rename = "trustType")]
    pub trust_type: String,
    #[serde(rename = "memberOf")]
    pub member_of: Vec<DirectoryObject>,
    #[serde(rename = "registeredOwners")]
    pub registered_owners: Vec<DirectoryObject>,
    #[serde(rename = "registeredUsers")]
    pub registered_users: Vec<DirectoryObject>,
    #[serde(rename = "transitiveMemberOf")]
    pub transitive_member_of: Vec<DirectoryObject>,
    #[serde(rename = "extensions")]
    pub extensions: Vec<Extension>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Extension {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryObjectPartnerReference {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "externalPartnerTenantId")]
    pub external_partner_tenant_id: String,
    #[serde(rename = "objectType")]
    pub object_type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryRole {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "roleTemplateId")]
    pub role_template_id: String,
    #[serde(rename = "members")]
    pub members: Vec<DirectoryObject>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryRoleTemplate {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(rename = "authenticationType")]
    pub authentication_type: String,
    #[serde(rename = "availabilityStatus")]
    pub availability_status: String,
    #[serde(rename = "isAdminManaged")]
    pub is_admin_managed: bool,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isInitial")]
    pub is_initial: bool,
    #[serde(rename = "isRoot")]
    pub is_root: bool,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
    #[serde(rename = "passwordNotificationWindowInDays")]
    pub password_notification_window_in_days: i32,
    #[serde(rename = "passwordValidityPeriodInDays")]
    pub password_validity_period_in_days: i32,
    #[serde(rename = "supportedServices")]
    pub supported_services: Vec<String>,
    #[serde(rename = "state")]
    pub state: DomainState,
    #[serde(rename = "serviceConfigurationRecords")]
    pub service_configuration_records: Vec<DomainDnsRecord>,
    #[serde(rename = "verificationDnsRecords")]
    pub verification_dns_records: Vec<DomainDnsRecord>,
    #[serde(rename = "domainNameReferences")]
    pub domain_name_references: Vec<DirectoryObject>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsRecord {
    #[serde(rename = "isOptional")]
    pub is_optional: bool,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "recordType")]
    pub record_type: String,
    #[serde(rename = "supportedService")]
    pub supported_service: String,
    #[serde(rename = "ttl")]
    pub ttl: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsCnameRecord {
    #[serde(rename = "canonicalName")]
    pub canonical_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsMxRecord {
    #[serde(rename = "mailExchange")]
    pub mail_exchange: String,
    #[serde(rename = "preference")]
    pub preference: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsSrvRecord {
    #[serde(rename = "nameTarget")]
    pub name_target: String,
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "weight")]
    pub weight: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsTxtRecord {
    #[serde(rename = "text")]
    pub text: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsUnavailableRecord {
    #[serde(rename = "description")]
    pub description: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseDetails {
    #[serde(rename = "servicePlans")]
    pub service_plans: Vec<ServicePlanInfo>,
    #[serde(rename = "skuId")]
    pub sku_id: String,
    #[serde(rename = "skuPartNumber")]
    pub sku_part_number: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "assignedLicenses")]
    pub assigned_licenses: Vec<AssignedLicense>,
    #[serde(rename = "classification")]
    pub classification: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "hasMembersWithLicenseErrors")]
    pub has_members_with_license_errors: bool,
    #[serde(rename = "groupTypes")]
    pub group_types: Vec<String>,
    #[serde(rename = "licenseProcessingState")]
    pub license_processing_state: LicenseProcessingState,
    #[serde(rename = "mail")]
    pub mail: String,
    #[serde(rename = "mailEnabled")]
    pub mail_enabled: bool,
    #[serde(rename = "mailNickname")]
    pub mail_nickname: String,
    #[serde(rename = "onPremisesLastSyncDateTime")]
    pub on_premises_last_sync_date_time: String,
    #[serde(rename = "onPremisesProvisioningErrors")]
    pub on_premises_provisioning_errors: Vec<OnPremisesProvisioningError>,
    #[serde(rename = "onPremisesSecurityIdentifier")]
    pub on_premises_security_identifier: String,
    #[serde(rename = "onPremisesSyncEnabled")]
    pub on_premises_sync_enabled: bool,
    #[serde(rename = "preferredDataLocation")]
    pub preferred_data_location: String,
    #[serde(rename = "proxyAddresses")]
    pub proxy_addresses: Vec<String>,
    #[serde(rename = "renewedDateTime")]
    pub renewed_date_time: String,
    #[serde(rename = "securityEnabled")]
    pub security_enabled: bool,
    #[serde(rename = "visibility")]
    pub visibility: String,
    #[serde(rename = "allowExternalSenders")]
    pub allow_external_senders: bool,
    #[serde(rename = "autoSubscribeNewMembers")]
    pub auto_subscribe_new_members: bool,
    #[serde(rename = "isSubscribedByMail")]
    pub is_subscribed_by_mail: bool,
    #[serde(rename = "unseenCount")]
    pub unseen_count: i32,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "members")]
    pub members: Vec<DirectoryObject>,
    #[serde(rename = "memberOf")]
    pub member_of: Vec<DirectoryObject>,
    #[serde(rename = "membersWithLicenseErrors")]
    pub members_with_license_errors: Vec<DirectoryObject>,
    #[serde(rename = "transitiveMembers")]
    pub transitive_members: Vec<DirectoryObject>,
    #[serde(rename = "transitiveMemberOf")]
    pub transitive_member_of: Vec<DirectoryObject>,
    #[serde(rename = "createdOnBehalfOf")]
    pub created_on_behalf_of: DirectoryObject,
    #[serde(rename = "owners")]
    pub owners: Vec<DirectoryObject>,
    #[serde(rename = "settings")]
    pub settings: Vec<GroupSetting>,
    #[serde(rename = "extensions")]
    pub extensions: Vec<Extension>,
    #[serde(rename = "threads")]
    pub threads: Vec<ConversationThread>,
    #[serde(rename = "calendar")]
    pub calendar: Calendar,
    #[serde(rename = "calendarView")]
    pub calendar_view: Vec<Event>,
    #[serde(rename = "events")]
    pub events: Vec<Event>,
    #[serde(rename = "conversations")]
    pub conversations: Vec<Conversation>,
    #[serde(rename = "photo")]
    pub photo: ProfilePhoto,
    #[serde(rename = "photos")]
    pub photos: Vec<ProfilePhoto>,
    #[serde(rename = "acceptedSenders")]
    pub accepted_senders: Vec<DirectoryObject>,
    #[serde(rename = "rejectedSenders")]
    pub rejected_senders: Vec<DirectoryObject>,
    #[serde(rename = "drive")]
    pub drive: Drive,
    #[serde(rename = "drives")]
    pub drives: Vec<Drive>,
    #[serde(rename = "sites")]
    pub sites: Vec<Site>,
    #[serde(rename = "planner")]
    pub planner: PlannerGroup,
    #[serde(rename = "onenote")]
    pub onenote: Onenote,
    #[serde(rename = "groupLifecyclePolicies")]
    pub group_lifecycle_policies: Vec<GroupLifecyclePolicy>,
    #[serde(rename = "team")]
    pub team: Team,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupSetting {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "values")]
    pub values: Vec<SettingValue>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationThread {
    #[serde(rename = "toRecipients")]
    pub to_recipients: Vec<Recipient>,
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "hasAttachments")]
    pub has_attachments: bool,
    #[serde(rename = "lastDeliveredDateTime")]
    pub last_delivered_date_time: String,
    #[serde(rename = "uniqueSenders")]
    pub unique_senders: Vec<String>,
    #[serde(rename = "ccRecipients")]
    pub cc_recipients: Vec<Recipient>,
    #[serde(rename = "preview")]
    pub preview: String,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "posts")]
    pub posts: Vec<Post>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Calendar {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "color")]
    pub color: CalendarColor,
    #[serde(rename = "changeKey")]
    pub change_key: String,
    #[serde(rename = "canShare")]
    pub can_share: bool,
    #[serde(rename = "canViewPrivateItems")]
    pub can_view_private_items: bool,
    #[serde(rename = "canEdit")]
    pub can_edit: bool,
    #[serde(rename = "owner")]
    pub owner: EmailAddress,
    #[serde(rename = "events")]
    pub events: Vec<Event>,
    #[serde(rename = "calendarView")]
    pub calendar_view: Vec<Event>,
    #[serde(rename = "singleValueExtendedProperties")]
    pub single_value_extended_properties: String,
    #[serde(rename = "multiValueExtendedProperties")]
    pub multi_value_extended_properties: Vec<MultiValueLegacyExtendedProperty>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutlookItem {
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "changeKey")]
    pub change_key: String,
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "originalStartTimeZone")]
    pub original_start_time_zone: String,
    #[serde(rename = "originalEndTimeZone")]
    pub original_end_time_zone: String,
    #[serde(rename = "responseStatus")]
    pub response_status: ResponseStatus,
    #[serde(rename = "iCalUId")]
    pub i_cal_u_id: String,
    #[serde(rename = "reminderMinutesBeforeStart")]
    pub reminder_minutes_before_start: i32,
    #[serde(rename = "isReminderOn")]
    pub is_reminder_on: bool,
    #[serde(rename = "hasAttachments")]
    pub has_attachments: bool,
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "body")]
    pub body: ItemBody,
    #[serde(rename = "bodyPreview")]
    pub body_preview: String,
    #[serde(rename = "importance")]
    pub importance: Importance,
    #[serde(rename = "sensitivity")]
    pub sensitivity: Sensitivity,
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "originalStart")]
    pub original_start: String,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "location")]
    pub location: Location,
    #[serde(rename = "locations")]
    pub locations: Vec<Location>,
    #[serde(rename = "isAllDay")]
    pub is_all_day: bool,
    #[serde(rename = "isCancelled")]
    pub is_cancelled: bool,
    #[serde(rename = "isOrganizer")]
    pub is_organizer: bool,
    #[serde(rename = "recurrence")]
    pub recurrence: PatternedRecurrence,
    #[serde(rename = "responseRequested")]
    pub response_requested: bool,
    #[serde(rename = "seriesMasterId")]
    pub series_master_id: String,
    #[serde(rename = "showAs")]
    pub show_as: FreeBusyStatus,
    #[serde(rename = "type")]
    pub _type: EventType,
    #[serde(rename = "attendees")]
    pub attendees: Vec<Attendee>,
    #[serde(rename = "organizer")]
    pub organizer: Recipient,
    #[serde(rename = "webLink")]
    pub web_link: String,
    #[serde(rename = "onlineMeetingUrl")]
    pub online_meeting_url: String,
    #[serde(rename = "calendar")]
    pub calendar: Calendar,
    #[serde(rename = "instances")]
    pub instances: Vec<Event>,
    #[serde(rename = "extensions")]
    pub extensions: Vec<Extension>,
    #[serde(rename = "attachments")]
    pub attachments: Vec<Attachment>,
    #[serde(rename = "singleValueExtendedProperties")]
    pub single_value_extended_properties: String,
    #[serde(rename = "multiValueExtendedProperties")]
    pub multi_value_extended_properties: Vec<MultiValueLegacyExtendedProperty>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Conversation {
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "hasAttachments")]
    pub has_attachments: bool,
    #[serde(rename = "lastDeliveredDateTime")]
    pub last_delivered_date_time: String,
    #[serde(rename = "uniqueSenders")]
    pub unique_senders: Vec<String>,
    #[serde(rename = "preview")]
    pub preview: String,
    #[serde(rename = "threads")]
    pub threads: Vec<ConversationThread>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfilePhoto {
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "width")]
    pub width: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItem {
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "eTag")]
    pub e_tag: String,
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: IdentitySet,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentReference")]
    pub parent_reference: ItemReference,
    #[serde(rename = "webUrl")]
    pub web_url: String,
    #[serde(rename = "createdByUser")]
    pub created_by_user: User,
    #[serde(rename = "lastModifiedByUser")]
    pub last_modified_by_user: User,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "driveType")]
    pub drive_type: String,
    #[serde(rename = "owner")]
    pub owner: IdentitySet,
    #[serde(rename = "quota")]
    pub quota: Quota,
    #[serde(rename = "sharePointIds")]
    pub share_point_ids: SharepointIds,
    #[serde(rename = "system")]
    pub system: SystemFacet,
    #[serde(rename = "items")]
    pub items: Vec<DriveItem>,
    #[serde(rename = "list")]
    pub list: Box<List>,
    #[serde(rename = "root")]
    pub root: Box<DriveItem>,
    #[serde(rename = "special")]
    pub special: Vec<DriveItem>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Site {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "root")]
    pub root: Root,
    #[serde(rename = "sharepointIds")]
    pub sharepoint_ids: SharepointIds,
    #[serde(rename = "siteCollection")]
    pub site_collection: SiteCollection,
    #[serde(rename = "analytics")]
    pub analytics: ItemAnalytics,
    #[serde(rename = "columns")]
    pub columns: Vec<ColumnDefinition>,
    #[serde(rename = "contentTypes")]
    pub content_types: Vec<ContentType>,
    #[serde(rename = "drive")]
    pub drive: Drive,
    #[serde(rename = "drives")]
    pub drives: Vec<Drive>,
    #[serde(rename = "items")]
    pub items: Vec<BaseItem>,
    #[serde(rename = "lists")]
    pub lists: Vec<List>,
    #[serde(rename = "sites")]
    pub sites: Vec<Site>,
    #[serde(rename = "onenote")]
    pub onenote: Onenote,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerGroup {
    #[serde(rename = "plans")]
    pub plans: Vec<PlannerPlan>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Onenote {
    #[serde(rename = "notebooks")]
    pub notebooks: Vec<Notebook>,
    #[serde(rename = "sections")]
    pub sections: Vec<OnenoteSection>,
    #[serde(rename = "sectionGroups")]
    pub section_groups: Vec<SectionGroup>,
    #[serde(rename = "pages")]
    pub pages: Vec<OnenotePage>,
    #[serde(rename = "resources")]
    pub resources: Vec<OnenoteResource>,
    #[serde(rename = "operations")]
    pub operations: Vec<OnenoteOperation>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupLifecyclePolicy {
    #[serde(rename = "groupLifetimeInDays")]
    pub group_lifetime_in_days: i32,
    #[serde(rename = "managedGroupTypes")]
    pub managed_group_types: String,
    #[serde(rename = "alternateNotificationEmails")]
    pub alternate_notification_emails: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "webUrl")]
    pub web_url: String,
    #[serde(rename = "memberSettings")]
    pub member_settings: TeamMemberSettings,
    #[serde(rename = "guestSettings")]
    pub guest_settings: TeamGuestSettings,
    #[serde(rename = "messagingSettings")]
    pub messaging_settings: TeamMessagingSettings,
    #[serde(rename = "funSettings")]
    pub fun_settings: TeamFunSettings,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "channels")]
    pub channels: Vec<Channel>,
    #[serde(rename = "installedApps")]
    pub installed_apps: Vec<TeamsAppInstallation>,
    #[serde(rename = "operations")]
    pub operations: Vec<TeamsAsyncOperation>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    #[serde(rename = "contractType")]
    pub contract_type: String,
    #[serde(rename = "customerId")]
    pub customer_id: String,
    #[serde(rename = "defaultDomainName")]
    pub default_domain_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscribedSku {
    #[serde(rename = "capabilityStatus")]
    pub capability_status: String,
    #[serde(rename = "consumedUnits")]
    pub consumed_units: i32,
    #[serde(rename = "prepaidUnits")]
    pub prepaid_units: LicenseUnitsDetail,
    #[serde(rename = "servicePlans")]
    pub service_plans: Vec<ServicePlanInfo>,
    #[serde(rename = "skuId")]
    pub sku_id: String,
    #[serde(rename = "skuPartNumber")]
    pub sku_part_number: String,
    #[serde(rename = "appliesTo")]
    pub applies_to: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "assignedPlans")]
    pub assigned_plans: Vec<AssignedPlan>,
    #[serde(rename = "businessPhones")]
    pub business_phones: Vec<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "countryLetterCode")]
    pub country_letter_code: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "marketingNotificationEmails")]
    pub marketing_notification_emails: Vec<String>,
    #[serde(rename = "onPremisesLastSyncDateTime")]
    pub on_premises_last_sync_date_time: String,
    #[serde(rename = "onPremisesSyncEnabled")]
    pub on_premises_sync_enabled: bool,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[serde(rename = "preferredLanguage")]
    pub preferred_language: String,
    #[serde(rename = "privacyProfile")]
    pub privacy_profile: PrivacyProfile,
    #[serde(rename = "provisionedPlans")]
    pub provisioned_plans: Vec<ProvisionedPlan>,
    #[serde(rename = "securityComplianceNotificationMails")]
    pub security_compliance_notification_mails: Vec<String>,
    #[serde(rename = "securityComplianceNotificationPhones")]
    pub security_compliance_notification_phones: Vec<String>,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "street")]
    pub street: String,
    #[serde(rename = "technicalNotificationMails")]
    pub technical_notification_mails: Vec<String>,
    #[serde(rename = "verifiedDomains")]
    pub verified_domains: Vec<VerifiedDomain>,
    #[serde(rename = "mobileDeviceManagementAuthority")]
    pub mobile_device_management_authority: MdmAuthority,
    #[serde(rename = "extensions")]
    pub extensions: Vec<Extension>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "accountEnabled")]
    pub account_enabled: bool,
    #[serde(rename = "ageGroup")]
    pub age_group: String,
    #[serde(rename = "assignedLicenses")]
    pub assigned_licenses: Vec<AssignedLicense>,
    #[serde(rename = "assignedPlans")]
    pub assigned_plans: Vec<AssignedPlan>,
    #[serde(rename = "businessPhones")]
    pub business_phones: Vec<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "consentProvidedForMinor")]
    pub consent_provided_for_minor: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "department")]
    pub department: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "employeeId")]
    pub employee_id: String,
    #[serde(rename = "faxNumber")]
    pub fax_number: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "imAddresses")]
    pub im_addresses: Vec<String>,
    #[serde(rename = "isResourceAccount")]
    pub is_resource_account: bool,
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    #[serde(rename = "legalAgeGroupClassification")]
    pub legal_age_group_classification: String,
    #[serde(rename = "licenseAssignmentStates")]
    pub license_assignment_states: Vec<LicenseAssignmentState>,
    #[serde(rename = "mail")]
    pub mail: String,
    #[serde(rename = "mailNickname")]
    pub mail_nickname: String,
    #[serde(rename = "mobilePhone")]
    pub mobile_phone: String,
    #[serde(rename = "onPremisesDistinguishedName")]
    pub on_premises_distinguished_name: String,
    #[serde(rename = "onPremisesExtensionAttributes")]
    pub on_premises_extension_attributes: OnPremisesExtensionAttributes,
    #[serde(rename = "onPremisesImmutableId")]
    pub on_premises_immutable_id: String,
    #[serde(rename = "onPremisesLastSyncDateTime")]
    pub on_premises_last_sync_date_time: String,
    #[serde(rename = "onPremisesProvisioningErrors")]
    pub on_premises_provisioning_errors: Vec<OnPremisesProvisioningError>,
    #[serde(rename = "onPremisesSecurityIdentifier")]
    pub on_premises_security_identifier: String,
    #[serde(rename = "onPremisesSyncEnabled")]
    pub on_premises_sync_enabled: bool,
    #[serde(rename = "onPremisesDomainName")]
    pub on_premises_domain_name: String,
    #[serde(rename = "onPremisesSamAccountName")]
    pub on_premises_sam_account_name: String,
    #[serde(rename = "onPremisesUserPrincipalName")]
    pub on_premises_user_principal_name: String,
    #[serde(rename = "otherMails")]
    pub other_mails: Vec<String>,
    #[serde(rename = "passwordPolicies")]
    pub password_policies: String,
    #[serde(rename = "passwordProfile")]
    pub password_profile: PasswordProfile,
    #[serde(rename = "officeLocation")]
    pub office_location: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[serde(rename = "preferredLanguage")]
    pub preferred_language: String,
    #[serde(rename = "provisionedPlans")]
    pub provisioned_plans: Vec<ProvisionedPlan>,
    #[serde(rename = "proxyAddresses")]
    pub proxy_addresses: Vec<String>,
    #[serde(rename = "showInAddressList")]
    pub show_in_address_list: bool,
    #[serde(rename = "signInSessionsValidFromDateTime")]
    pub sign_in_sessions_valid_from_date_time: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "streetAddress")]
    pub street_address: String,
    #[serde(rename = "surname")]
    pub surname: String,
    #[serde(rename = "usageLocation")]
    pub usage_location: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "userType")]
    pub user_type: String,
    #[serde(rename = "mailboxSettings")]
    pub mailbox_settings: MailboxSettings,
    #[serde(rename = "aboutMe")]
    pub about_me: String,
    #[serde(rename = "birthday")]
    pub birthday: String,
    #[serde(rename = "hireDate")]
    pub hire_date: String,
    #[serde(rename = "interests")]
    pub interests: Vec<String>,
    #[serde(rename = "mySite")]
    pub my_site: String,
    #[serde(rename = "pastProjects")]
    pub past_projects: Vec<String>,
    #[serde(rename = "preferredName")]
    pub preferred_name: String,
    #[serde(rename = "responsibilities")]
    pub responsibilities: Vec<String>,
    #[serde(rename = "schools")]
    pub schools: Vec<String>,
    #[serde(rename = "skills")]
    pub skills: Vec<String>,
    #[serde(rename = "deviceEnrollmentLimit")]
    pub device_enrollment_limit: i32,
    #[serde(rename = "ownedDevices")]
    pub owned_devices: Vec<DirectoryObject>,
    #[serde(rename = "registeredDevices")]
    pub registered_devices: Vec<DirectoryObject>,
    #[serde(rename = "manager")]
    pub manager: DirectoryObject,
    #[serde(rename = "directReports")]
    pub direct_reports: Vec<DirectoryObject>,
    #[serde(rename = "memberOf")]
    pub member_of: Vec<DirectoryObject>,
    #[serde(rename = "createdObjects")]
    pub created_objects: Vec<DirectoryObject>,
    #[serde(rename = "ownedObjects")]
    pub owned_objects: Vec<DirectoryObject>,
    #[serde(rename = "licenseDetails")]
    pub license_details: Vec<LicenseDetails>,
    #[serde(rename = "transitiveMemberOf")]
    pub transitive_member_of: Vec<DirectoryObject>,
    #[serde(rename = "extensions")]
    pub extensions: Vec<Extension>,
    #[serde(rename = "outlook")]
    pub outlook: OutlookUser,
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
    #[serde(rename = "mailFolders")]
    pub mail_folders: Vec<MailFolder>,
    #[serde(rename = "calendar")]
    pub calendar: Calendar,
    #[serde(rename = "calendars")]
    pub calendars: Vec<Calendar>,
    #[serde(rename = "calendarGroups")]
    pub calendar_groups: Vec<CalendarGroup>,
    #[serde(rename = "calendarView")]
    pub calendar_view: Vec<Event>,
    #[serde(rename = "events")]
    pub events: Vec<Event>,
    #[serde(rename = "people")]
    pub people: Vec<Person>,
    #[serde(rename = "contacts")]
    pub contacts: Vec<Contact>,
    #[serde(rename = "contactFolders")]
    pub contact_folders: Vec<ContactFolder>,
    #[serde(rename = "inferenceClassification")]
    pub inference_classification: InferenceClassification,
    #[serde(rename = "photo")]
    pub photo: ProfilePhoto,
    #[serde(rename = "photos")]
    pub photos: Vec<ProfilePhoto>,
    #[serde(rename = "drive")]
    pub drive: Box<Drive>,
    #[serde(rename = "drives")]
    pub drives: Vec<Drive>,
    #[serde(rename = "planner")]
    pub planner: PlannerUser,
    #[serde(rename = "onenote")]
    pub onenote: Onenote,
    #[serde(rename = "managedDevices")]
    pub managed_devices: Vec<ManagedDevice>,
    #[serde(rename = "managedAppRegistrations")]
    pub managed_app_registrations: Vec<ManagedAppRegistration>,
    #[serde(rename = "deviceManagementTroubleshootingEvents")]
    pub device_management_troubleshooting_events: Vec<DeviceManagementTroubleshootingEvent>,
    #[serde(rename = "activities")]
    pub activities: Vec<UserActivity>,
    #[serde(rename = "insights")]
    pub insights: OfficeGraphInsights,
    #[serde(rename = "settings")]
    pub settings: UserSettings,
    #[serde(rename = "joinedTeams")]
    pub joined_teams: Vec<Group>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutlookUser {
    #[serde(rename = "masterCategories")]
    pub master_categories: Vec<OutlookCategory>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "receivedDateTime")]
    pub received_date_time: String,
    #[serde(rename = "sentDateTime")]
    pub sent_date_time: String,
    #[serde(rename = "hasAttachments")]
    pub has_attachments: bool,
    #[serde(rename = "internetMessageId")]
    pub internet_message_id: String,
    #[serde(rename = "internetMessageHeaders")]
    pub internet_message_headers: Vec<InternetMessageHeader>,
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "body")]
    pub body: ItemBody,
    #[serde(rename = "bodyPreview")]
    pub body_preview: String,
    #[serde(rename = "importance")]
    pub importance: Importance,
    #[serde(rename = "parentFolderId")]
    pub parent_folder_id: String,
    #[serde(rename = "sender")]
    pub sender: Recipient,
    #[serde(rename = "from")]
    pub from: Recipient,
    #[serde(rename = "toRecipients")]
    pub to_recipients: Vec<Recipient>,
    #[serde(rename = "ccRecipients")]
    pub cc_recipients: Vec<Recipient>,
    #[serde(rename = "bccRecipients")]
    pub bcc_recipients: Vec<Recipient>,
    #[serde(rename = "replyTo")]
    pub reply_to: Vec<Recipient>,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    #[serde(rename = "uniqueBody")]
    pub unique_body: ItemBody,
    #[serde(rename = "isDeliveryReceiptRequested")]
    pub is_delivery_receipt_requested: bool,
    #[serde(rename = "isReadReceiptRequested")]
    pub is_read_receipt_requested: bool,
    #[serde(rename = "isRead")]
    pub is_read: bool,
    #[serde(rename = "isDraft")]
    pub is_draft: bool,
    #[serde(rename = "webLink")]
    pub web_link: String,
    #[serde(rename = "inferenceClassification")]
    pub inference_classification: InferenceClassificationType,
    #[serde(rename = "flag")]
    pub flag: FollowupFlag,
    #[serde(rename = "attachments")]
    pub attachments: Vec<Attachment>,
    #[serde(rename = "extensions")]
    pub extensions: Vec<Extension>,
    #[serde(rename = "singleValueExtendedProperties")]
    pub single_value_extended_properties: String,
    #[serde(rename = "multiValueExtendedProperties")]
    pub multi_value_extended_properties: Vec<MultiValueLegacyExtendedProperty>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MailFolder {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "parentFolderId")]
    pub parent_folder_id: String,
    #[serde(rename = "childFolderCount")]
    pub child_folder_count: i32,
    #[serde(rename = "unreadItemCount")]
    pub unread_item_count: i32,
    #[serde(rename = "totalItemCount")]
    pub total_item_count: i32,
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
    #[serde(rename = "messageRules")]
    pub message_rules: Vec<MessageRule>,
    #[serde(rename = "childFolders")]
    pub child_folders: Vec<MailFolder>,
    #[serde(rename = "singleValueExtendedProperties")]
    pub single_value_extended_properties: String,
    #[serde(rename = "multiValueExtendedProperties")]
    pub multi_value_extended_properties: Vec<MultiValueLegacyExtendedProperty>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarGroup {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "classId")]
    pub class_id: String,
    #[serde(rename = "changeKey")]
    pub change_key: String,
    #[serde(rename = "calendars")]
    pub calendars: Vec<Calendar>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "surname")]
    pub surname: String,
    #[serde(rename = "birthday")]
    pub birthday: String,
    #[serde(rename = "personNotes")]
    pub person_notes: String,
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    #[serde(rename = "scoredEmailAddresses")]
    pub scored_email_addresses: Vec<ScoredEmailAddress>,
    #[serde(rename = "phones")]
    pub phones: Vec<Phone>,
    #[serde(rename = "postalAddresses")]
    pub postal_addresses: Vec<Location>,
    #[serde(rename = "websites")]
    pub websites: Vec<Website>,
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "yomiCompany")]
    pub yomi_company: String,
    #[serde(rename = "department")]
    pub department: String,
    #[serde(rename = "officeLocation")]
    pub office_location: String,
    #[serde(rename = "profession")]
    pub profession: String,
    #[serde(rename = "personType")]
    pub person_type: PersonType,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "imAddress")]
    pub im_address: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "parentFolderId")]
    pub parent_folder_id: String,
    #[serde(rename = "birthday")]
    pub birthday: String,
    #[serde(rename = "fileAs")]
    pub file_as: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "initials")]
    pub initials: String,
    #[serde(rename = "middleName")]
    pub middle_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "surname")]
    pub surname: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "yomiGivenName")]
    pub yomi_given_name: String,
    #[serde(rename = "yomiSurname")]
    pub yomi_surname: String,
    #[serde(rename = "yomiCompanyName")]
    pub yomi_company_name: String,
    #[serde(rename = "generation")]
    pub generation: String,
    #[serde(rename = "emailAddresses")]
    pub email_addresses: Vec<EmailAddress>,
    #[serde(rename = "imAddresses")]
    pub im_addresses: Vec<String>,
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "department")]
    pub department: String,
    #[serde(rename = "officeLocation")]
    pub office_location: String,
    #[serde(rename = "profession")]
    pub profession: String,
    #[serde(rename = "businessHomePage")]
    pub business_home_page: String,
    #[serde(rename = "assistantName")]
    pub assistant_name: String,
    #[serde(rename = "manager")]
    pub manager: String,
    #[serde(rename = "homePhones")]
    pub home_phones: Vec<String>,
    #[serde(rename = "mobilePhone")]
    pub mobile_phone: String,
    #[serde(rename = "businessPhones")]
    pub business_phones: Vec<String>,
    #[serde(rename = "homeAddress")]
    pub home_address: PhysicalAddress,
    #[serde(rename = "businessAddress")]
    pub business_address: PhysicalAddress,
    #[serde(rename = "otherAddress")]
    pub other_address: PhysicalAddress,
    #[serde(rename = "spouseName")]
    pub spouse_name: String,
    #[serde(rename = "personalNotes")]
    pub personal_notes: String,
    #[serde(rename = "children")]
    pub children: Vec<String>,
    #[serde(rename = "extensions")]
    pub extensions: Vec<Extension>,
    #[serde(rename = "singleValueExtendedProperties")]
    pub single_value_extended_properties: String,
    #[serde(rename = "multiValueExtendedProperties")]
    pub multi_value_extended_properties: Vec<MultiValueLegacyExtendedProperty>,
    #[serde(rename = "photo")]
    pub photo: ProfilePhoto,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactFolder {
    #[serde(rename = "parentFolderId")]
    pub parent_folder_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "contacts")]
    pub contacts: Vec<Contact>,
    #[serde(rename = "childFolders")]
    pub child_folders: Vec<ContactFolder>,
    #[serde(rename = "singleValueExtendedProperties")]
    pub single_value_extended_properties: String,
    #[serde(rename = "multiValueExtendedProperties")]
    pub multi_value_extended_properties: Vec<MultiValueLegacyExtendedProperty>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InferenceClassification {
    #[serde(rename = "overrides")]
    pub overrides: Vec<InferenceClassificationOverride>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerUser {
    #[serde(rename = "tasks")]
    pub tasks: Vec<PlannerTask>,
    #[serde(rename = "plans")]
    pub plans: Vec<PlannerPlan>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDevice {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "deviceName")]
    pub device_name: String,
    #[serde(rename = "managedDeviceOwnerType")]
    pub managed_device_owner_type: ManagedDeviceOwnerType,
    #[serde(rename = "deviceActionResults")]
    pub device_action_results: Vec<DeviceActionResult>,
    #[serde(rename = "enrolledDateTime")]
    pub enrolled_date_time: String,
    #[serde(rename = "lastSyncDateTime")]
    pub last_sync_date_time: String,
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,
    #[serde(rename = "complianceState")]
    pub compliance_state: ComplianceState,
    #[serde(rename = "jailBroken")]
    pub jail_broken: String,
    #[serde(rename = "managementAgent")]
    pub management_agent: ManagementAgentType,
    #[serde(rename = "osVersion")]
    pub os_version: String,
    #[serde(rename = "easActivated")]
    pub eas_activated: bool,
    #[serde(rename = "easDeviceId")]
    pub eas_device_id: String,
    #[serde(rename = "easActivationDateTime")]
    pub eas_activation_date_time: String,
    #[serde(rename = "azureADRegistered")]
    pub azure_a_d_registered: bool,
    #[serde(rename = "deviceEnrollmentType")]
    pub device_enrollment_type: DeviceEnrollmentType,
    #[serde(rename = "activationLockBypassCode")]
    pub activation_lock_bypass_code: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "azureADDeviceId")]
    pub azure_a_d_device_id: String,
    #[serde(rename = "deviceRegistrationState")]
    pub device_registration_state: DeviceRegistrationState,
    #[serde(rename = "deviceCategoryDisplayName")]
    pub device_category_display_name: String,
    #[serde(rename = "isSupervised")]
    pub is_supervised: bool,
    #[serde(rename = "exchangeLastSuccessfulSyncDateTime")]
    pub exchange_last_successful_sync_date_time: String,
    #[serde(rename = "exchangeAccessState")]
    pub exchange_access_state: DeviceManagementExchangeAccessState,
    #[serde(rename = "exchangeAccessStateReason")]
    pub exchange_access_state_reason: DeviceManagementExchangeAccessStateReason,
    #[serde(rename = "remoteAssistanceSessionUrl")]
    pub remote_assistance_session_url: String,
    #[serde(rename = "remoteAssistanceSessionErrorDetails")]
    pub remote_assistance_session_error_details: String,
    #[serde(rename = "isEncrypted")]
    pub is_encrypted: bool,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "imei")]
    pub imei: String,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    pub compliance_grace_period_expiration_date_time: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(rename = "androidSecurityPatchLevel")]
    pub android_security_patch_level: String,
    #[serde(rename = "userDisplayName")]
    pub user_display_name: String,
    #[serde(rename = "configurationManagerClientEnabledFeatures")]
    pub configuration_manager_client_enabled_features: ConfigurationManagerClientEnabledFeatures,
    #[serde(rename = "wiFiMacAddress")]
    pub wi_fi_mac_address: String,
    #[serde(rename = "deviceHealthAttestationState")]
    pub device_health_attestation_state: DeviceHealthAttestationState,
    #[serde(rename = "subscriberCarrier")]
    pub subscriber_carrier: String,
    #[serde(rename = "meid")]
    pub meid: String,
    #[serde(rename = "totalStorageSpaceInBytes")]
    pub total_storage_space_in_bytes: i64,
    #[serde(rename = "freeStorageSpaceInBytes")]
    pub free_storage_space_in_bytes: i64,
    #[serde(rename = "managedDeviceName")]
    pub managed_device_name: String,
    #[serde(rename = "partnerReportedThreatState")]
    pub partner_reported_threat_state: ManagedDevicePartnerReportedHealthState,
    #[serde(rename = "deviceConfigurationStates")]
    pub device_configuration_states: Vec<DeviceConfigurationState>,
    #[serde(rename = "deviceCategory")]
    pub device_category: DeviceCategory,
    #[serde(rename = "deviceCompliancePolicyStates")]
    pub device_compliance_policy_states: Vec<DeviceCompliancePolicyState>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppRegistration {
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastSyncDateTime")]
    pub last_sync_date_time: String,
    #[serde(rename = "applicationVersion")]
    pub application_version: String,
    #[serde(rename = "managementSdkVersion")]
    pub management_sdk_version: String,
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
    #[serde(rename = "deviceType")]
    pub device_type: String,
    #[serde(rename = "deviceTag")]
    pub device_tag: String,
    #[serde(rename = "deviceName")]
    pub device_name: String,
    #[serde(rename = "flaggedReasons")]
    pub flagged_reasons: Vec<ManagedAppFlaggedReason>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appIdentifier")]
    pub app_identifier: MobileAppIdentifier,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "appliedPolicies")]
    pub applied_policies: Vec<ManagedAppPolicy>,
    #[serde(rename = "intendedPolicies")]
    pub intended_policies: Vec<ManagedAppPolicy>,
    #[serde(rename = "operations")]
    pub operations: Vec<ManagedAppOperation>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagementTroubleshootingEvent {
    #[serde(rename = "eventDateTime")]
    pub event_date_time: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserActivity {
    #[serde(rename = "visualElements")]
    pub visual_elements: VisualInfo,
    #[serde(rename = "activitySourceHost")]
    pub activity_source_host: String,
    #[serde(rename = "activationUrl")]
    pub activation_url: String,
    #[serde(rename = "appActivityId")]
    pub app_activity_id: String,
    #[serde(rename = "appDisplayName")]
    pub app_display_name: String,
    #[serde(rename = "contentUrl")]
    pub content_url: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "fallbackUrl")]
    pub fallback_url: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "userTimezone")]
    pub user_timezone: String,
    #[serde(rename = "contentInfo")]
    pub content_info: Json,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "historyItems")]
    pub history_items: Vec<ActivityHistoryItem>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeGraphInsights {
    #[serde(rename = "trending")]
    pub trending: Vec<Trending>,
    #[serde(rename = "shared")]
    pub shared: Vec<SharedInsight>,
    #[serde(rename = "used")]
    pub used: Vec<UsedInsight>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    #[serde(rename = "contributionToContentDiscoveryDisabled")]
    pub contribution_to_content_discovery_disabled: bool,
    #[serde(rename = "contributionToContentDiscoveryAsOrganizationDisabled")]
    pub contribution_to_content_discovery_as_organization_disabled: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupSettingTemplate {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "values")]
    pub values: Vec<SettingTemplateValue>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaExtension {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "targetTypes")]
    pub target_types: Vec<String>,
    #[serde(rename = "properties")]
    pub properties: Vec<ExtensionSchemaProperty>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "owner")]
    pub owner: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "isInline")]
    pub is_inline: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutlookCategory {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "color")]
    pub color: CategoryColor,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageRule {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "sequence")]
    pub sequence: i32,
    #[serde(rename = "conditions")]
    pub conditions: MessageRulePredicates,
    #[serde(rename = "actions")]
    pub actions: MessageRuleActions,
    #[serde(rename = "exceptions")]
    pub exceptions: MessageRulePredicates,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "hasError")]
    pub has_error: bool,
    #[serde(rename = "isReadOnly")]
    pub is_read_only: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleValueLegacyExtendedProperty {
    #[serde(rename = "value")]
    pub value: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiValueLegacyExtendedProperty {
    #[serde(rename = "value")]
    pub value: Vec<String>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MailSearchFolder {
    #[serde(rename = "isSupported")]
    pub is_supported: bool,
    #[serde(rename = "includeNestedFolders")]
    pub include_nested_folders: bool,
    #[serde(rename = "sourceFolderIds")]
    pub source_folder_ids: Vec<String>,
    #[serde(rename = "filterQuery")]
    pub filter_query: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileAttachment {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "contentLocation")]
    pub content_location: String,
    #[serde(rename = "contentBytes")]
    pub content_bytes: Vec<u8>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemAttachment {
    #[serde(rename = "item")]
    pub item: OutlookItem,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    #[serde(rename = "meetingMessageType")]
    pub meeting_message_type: MeetingMessageType,
    #[serde(rename = "event")]
    pub event: Event,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceAttachment {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenTypeExtension {
    #[serde(rename = "extensionName")]
    pub extension_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "body")]
    pub body: ItemBody,
    #[serde(rename = "receivedDateTime")]
    pub received_date_time: String,
    #[serde(rename = "hasAttachments")]
    pub has_attachments: bool,
    #[serde(rename = "from")]
    pub from: Recipient,
    #[serde(rename = "sender")]
    pub sender: Recipient,
    #[serde(rename = "conversationThreadId")]
    pub conversation_thread_id: String,
    #[serde(rename = "newParticipants")]
    pub new_participants: Vec<Recipient>,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    #[serde(rename = "extensions")]
    pub extensions: Vec<Extension>,
    #[serde(rename = "inReplyTo")]
    pub in_reply_to: Box<Post>,
    #[serde(rename = "attachments")]
    pub attachments: Vec<Attachment>,
    #[serde(rename = "singleValueExtendedProperties")]
    pub single_value_extended_properties: String,
    #[serde(rename = "multiValueExtendedProperties")]
    pub multi_value_extended_properties: Vec<MultiValueLegacyExtendedProperty>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InferenceClassificationOverride {
    #[serde(rename = "classifyAs")]
    pub classify_as: InferenceClassificationType,
    #[serde(rename = "senderEmailAddress")]
    pub sender_email_address: EmailAddress,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemVersion {
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: IdentitySet,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "publication")]
    pub publication: PublicationFacet,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnDefinition {
    #[serde(rename = "boolean")]
    pub boolean: BooleanColumn,
    #[serde(rename = "calculated")]
    pub calculated: CalculatedColumn,
    #[serde(rename = "choice")]
    pub choice: ChoiceColumn,
    #[serde(rename = "columnGroup")]
    pub column_group: String,
    #[serde(rename = "currency")]
    pub currency: CurrencyColumn,
    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "defaultValue")]
    pub default_value: DefaultColumnValue,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "enforceUniqueValues")]
    pub enforce_unique_values: bool,
    #[serde(rename = "hidden")]
    pub hidden: bool,
    #[serde(rename = "indexed")]
    pub indexed: bool,
    #[serde(rename = "lookup")]
    pub lookup: LookupColumn,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "number")]
    pub number: NumberColumn,
    #[serde(rename = "personOrGroup")]
    pub person_or_group: PersonOrGroupColumn,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    #[serde(rename = "required")]
    pub required: bool,
    #[serde(rename = "text")]
    pub text: TextColumn,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnLink {
    #[serde(rename = "name")]
    pub name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentType {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "group")]
    pub group: String,
    #[serde(rename = "hidden")]
    pub hidden: bool,
    #[serde(rename = "inheritedFrom")]
    pub inherited_from: ItemReference,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "order")]
    pub order: ContentTypeOrder,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    #[serde(rename = "sealed")]
    pub sealed: bool,
    #[serde(rename = "columnLinks")]
    pub column_links: Vec<ColumnLink>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriveItem {
    #[serde(rename = "@odata.context")]
    odata_context: String,
    #[serde(rename = "@odata.type")]
    odata_type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "eTag")]
    pub e_tag: String,
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: IdentitySet,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentReference")]
    pub parent_reference: ItemReference,
    #[serde(rename = "webUrl")]
    pub web_url: String,
    #[serde(rename = "createdByUser")]
    pub created_by_user: Box<User>,
    #[serde(rename = "lastModifiedByUser")]
    pub last_modified_by_user: Box<User>,
    #[serde(rename = "audio")]
    pub audio: Audio,
    #[serde(rename = "content")]
    pub content: Vec<u8>,
    #[serde(rename = "cTag")]
    pub c_tag: String,
    #[serde(rename = "deleted")]
    pub deleted: Deleted,
    #[serde(rename = "file")]
    pub file: File,
    #[serde(rename = "fileSystemInfo")]
    pub file_system_info: FileSystemInfo,
    #[serde(rename = "folder")]
    pub folder: Folder,
    #[serde(rename = "image")]
    pub image: Image,
    #[serde(rename = "location")]
    pub location: GeoCoordinates,
    #[serde(rename = "package")]
    pub package: Package,
    #[serde(rename = "photo")]
    pub photo: Photo,
    #[serde(rename = "publication")]
    pub publication: PublicationFacet,
    #[serde(rename = "remoteItem")]
    pub remote_item: RemoteItem,
    #[serde(rename = "root")]
    pub root: Root,
    #[serde(rename = "searchResult")]
    pub search_result: SearchResult,
    #[serde(rename = "shared")]
    pub shared: Shared,
    #[serde(rename = "sharepointIds")]
    pub sharepoint_ids: SharepointIds,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "specialFolder")]
    pub special_folder: SpecialFolder,
    #[serde(rename = "video")]
    pub video: Video,
    #[serde(rename = "webDavUrl")]
    pub web_dav_url: String,
    #[serde(rename = "analytics")]
    pub analytics: ItemAnalytics,
    #[serde(rename = "children")]
    pub children: Vec<DriveItem>,
    #[serde(rename = "listItem")]
    pub list_item: Box<ListItem>,
    #[serde(rename = "permissions")]
    pub permissions: Vec<Permission>,
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<Subscription>,
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<ThumbnailSet>,
    #[serde(rename = "versions")]
    pub versions: Vec<DriveItemVersion>,
    #[serde(rename = "workbook")]
    pub workbook: Workbook,
    #[serde(rename = "@microsoft.graph.conflictBehavior")]
    conflict_behavior: String,
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    pub download_url: String,
    #[serde(rename = "@microsoft.graph.sourceUrl")]
    pub source_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct List {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "list")]
    pub list: ListInfo,
    #[serde(rename = "sharepointIds")]
    pub sharepoint_ids: SharepointIds,
    #[serde(rename = "system")]
    pub system: SystemFacet,
    #[serde(rename = "columns")]
    pub columns: Vec<ColumnDefinition>,
    #[serde(rename = "contentTypes")]
    pub content_types: Vec<ContentType>,
    #[serde(rename = "drive")]
    pub drive: Drive,
    #[serde(rename = "items")]
    pub items: Vec<ListItem>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemAnalytics {
    #[serde(rename = "itemActivityStats")]
    pub item_activity_stats: Vec<ItemActivityStat>,
    #[serde(rename = "allTime")]
    pub all_time: ItemActivityStat,
    #[serde(rename = "lastSevenDays")]
    pub last_seven_days: ItemActivityStat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListItem {
    #[serde(rename = "contentType")]
    pub content_type: ContentTypeInfo,
    #[serde(rename = "sharepointIds")]
    pub sharepoint_ids: SharepointIds,
    #[serde(rename = "analytics")]
    pub analytics: ItemAnalytics,
    #[serde(rename = "driveItem")]
    pub drive_item: Box<DriveItem>,
    #[serde(rename = "fields")]
    pub fields: FieldValueSet,
    #[serde(rename = "versions")]
    pub versions: Vec<ListItemVersion>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "grantedTo")]
    pub granted_to: IdentitySet,
    #[serde(rename = "inheritedFrom")]
    pub inherited_from: ItemReference,
    #[serde(rename = "invitation")]
    pub invitation: SharingInvitation,
    #[serde(rename = "link")]
    pub link: SharingLink,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
    #[serde(rename = "shareId")]
    pub share_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "changeType")]
    pub change_type: String,
    #[serde(rename = "clientState")]
    pub client_state: String,
    #[serde(rename = "notificationUrl")]
    pub notification_url: String,
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThumbnailSet {
    #[serde(rename = "large")]
    pub large: Thumbnail,
    #[serde(rename = "medium")]
    pub medium: Thumbnail,
    #[serde(rename = "small")]
    pub small: Thumbnail,
    #[serde(rename = "source")]
    pub source: Thumbnail,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriveItemVersion {
    #[serde(rename = "content")]
    pub content: Vec<u8>,
    #[serde(rename = "size")]
    pub size: i64,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workbook {
    #[serde(rename = "application")]
    pub application: WorkbookApplication,
    #[serde(rename = "names")]
    pub names: Vec<WorkbookNamedItem>,
    #[serde(rename = "tables")]
    pub tables: Vec<WorkbookTable>,
    #[serde(rename = "worksheets")]
    pub worksheets: Vec<WorkbookWorksheet>,
    #[serde(rename = "comments")]
    pub comments: Vec<WorkbookComment>,
    #[serde(rename = "functions")]
    pub functions: WorkbookFunctions,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldValueSet {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemActivity {
    #[serde(rename = "access")]
    pub access: AccessAction,
    #[serde(rename = "activityDateTime")]
    pub activity_date_time: String,
    #[serde(rename = "actor")]
    pub actor: IdentitySet,
    #[serde(rename = "driveItem")]
    pub drive_item: DriveItem,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemActivityStat {
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,
    #[serde(rename = "endDateTime")]
    pub end_date_time: String,
    #[serde(rename = "access")]
    pub access: ItemActionStat,
    #[serde(rename = "create")]
    pub create: ItemActionStat,
    #[serde(rename = "delete")]
    pub delete: ItemActionStat,
    #[serde(rename = "edit")]
    pub edit: ItemActionStat,
    #[serde(rename = "move")]
    pub _move: ItemActionStat,
    #[serde(rename = "isTrending")]
    pub is_trending: bool,
    #[serde(rename = "incompleteData")]
    pub incomplete_data: IncompleteData,
    #[serde(rename = "activities")]
    pub activities: Vec<ItemActivity>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListItemVersion {
    #[serde(rename = "fields")]
    pub fields: FieldValueSet,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDriveItem {
    #[serde(rename = "owner")]
    pub owner: IdentitySet,
    #[serde(rename = "driveItem")]
    pub drive_item: DriveItem,
    #[serde(rename = "items")]
    pub items: Vec<DriveItem>,
    #[serde(rename = "list")]
    pub list: List,
    #[serde(rename = "listItem")]
    pub list_item: ListItem,
    #[serde(rename = "root")]
    pub root: DriveItem,
    #[serde(rename = "site")]
    pub site: Site,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookApplication {
    #[serde(rename = "calculationMode")]
    pub calculation_mode: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookNamedItem {
    #[serde(rename = "comment")]
    pub comment: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "value")]
    pub value: Json,
    #[serde(rename = "visible")]
    pub visible: bool,
    #[serde(rename = "worksheet")]
    pub worksheet: WorkbookWorksheet,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTable {
    #[serde(rename = "highlightFirstColumn")]
    pub highlight_first_column: bool,
    #[serde(rename = "highlightLastColumn")]
    pub highlight_last_column: bool,
    #[serde(rename = "legacyId")]
    pub legacy_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "showBandedColumns")]
    pub show_banded_columns: bool,
    #[serde(rename = "showBandedRows")]
    pub show_banded_rows: bool,
    #[serde(rename = "showFilterButton")]
    pub show_filter_button: bool,
    #[serde(rename = "showHeaders")]
    pub show_headers: bool,
    #[serde(rename = "showTotals")]
    pub show_totals: bool,
    #[serde(rename = "style")]
    pub style: String,
    #[serde(rename = "columns")]
    pub columns: Vec<WorkbookTableColumn>,
    #[serde(rename = "rows")]
    pub rows: Vec<WorkbookTableRow>,
    #[serde(rename = "sort")]
    pub sort: WorkbookTableSort,
    #[serde(rename = "worksheet")]
    pub worksheet: WorkbookWorksheet,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookWorksheet {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "position")]
    pub position: i32,
    #[serde(rename = "visibility")]
    pub visibility: String,
    #[serde(rename = "charts")]
    pub charts: Vec<WorkbookChart>,
    #[serde(rename = "names")]
    pub names: Vec<WorkbookNamedItem>,
    #[serde(rename = "pivotTables")]
    pub pivot_tables: Vec<WorkbookPivotTable>,
    #[serde(rename = "protection")]
    pub protection: WorkbookWorksheetProtection,
    #[serde(rename = "tables")]
    pub tables: Vec<WorkbookTable>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookComment {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "replies")]
    pub replies: Vec<WorkbookCommentReply>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFunctions {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChart {
    #[serde(rename = "height")]
    pub height: i64,
    #[serde(rename = "left")]
    pub left: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "top")]
    pub top: i64,
    #[serde(rename = "width")]
    pub width: i64,
    #[serde(rename = "axes")]
    pub axes: WorkbookChartAxes,
    #[serde(rename = "dataLabels")]
    pub data_labels: WorkbookChartDataLabels,
    #[serde(rename = "format")]
    pub format: WorkbookChartAreaFormat,
    #[serde(rename = "legend")]
    pub legend: WorkbookChartLegend,
    #[serde(rename = "series")]
    pub series: Vec<WorkbookChartSeries>,
    #[serde(rename = "title")]
    pub title: WorkbookChartTitle,
    #[serde(rename = "worksheet")]
    pub worksheet: WorkbookWorksheet,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxes {
    #[serde(rename = "categoryAxis")]
    pub category_axis: WorkbookChartAxis,
    #[serde(rename = "seriesAxis")]
    pub series_axis: WorkbookChartAxis,
    #[serde(rename = "valueAxis")]
    pub value_axis: WorkbookChartAxis,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartDataLabels {
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "separator")]
    pub separator: String,
    #[serde(rename = "showBubbleSize")]
    pub show_bubble_size: bool,
    #[serde(rename = "showCategoryName")]
    pub show_category_name: bool,
    #[serde(rename = "showLegendKey")]
    pub show_legend_key: bool,
    #[serde(rename = "showPercentage")]
    pub show_percentage: bool,
    #[serde(rename = "showSeriesName")]
    pub show_series_name: bool,
    #[serde(rename = "showValue")]
    pub show_value: bool,
    #[serde(rename = "format")]
    pub format: WorkbookChartDataLabelFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAreaFormat {
    #[serde(rename = "fill")]
    pub fill: WorkbookChartFill,
    #[serde(rename = "font")]
    pub font: WorkbookChartFont,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartLegend {
    #[serde(rename = "overlay")]
    pub overlay: bool,
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "visible")]
    pub visible: bool,
    #[serde(rename = "format")]
    pub format: WorkbookChartLegendFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartSeries {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "format")]
    pub format: WorkbookChartSeriesFormat,
    #[serde(rename = "points")]
    pub points: Vec<WorkbookChartPoint>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartTitle {
    #[serde(rename = "overlay")]
    pub overlay: bool,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "visible")]
    pub visible: bool,
    #[serde(rename = "format")]
    pub format: WorkbookChartTitleFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartFill {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartFont {
    #[serde(rename = "bold")]
    pub bold: bool,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "italic")]
    pub italic: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "underline")]
    pub underline: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxis {
    #[serde(rename = "majorUnit")]
    pub major_unit: Json,
    #[serde(rename = "maximum")]
    pub maximum: Json,
    #[serde(rename = "minimum")]
    pub minimum: Json,
    #[serde(rename = "minorUnit")]
    pub minor_unit: Json,
    #[serde(rename = "format")]
    pub format: WorkbookChartAxisFormat,
    #[serde(rename = "majorGridlines")]
    pub major_gridlines: WorkbookChartGridlines,
    #[serde(rename = "minorGridlines")]
    pub minor_gridlines: WorkbookChartGridlines,
    #[serde(rename = "title")]
    pub title: WorkbookChartAxisTitle,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxisFormat {
    #[serde(rename = "font")]
    pub font: WorkbookChartFont,
    #[serde(rename = "line")]
    pub line: WorkbookChartLineFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartGridlines {
    #[serde(rename = "visible")]
    pub visible: bool,
    #[serde(rename = "format")]
    pub format: WorkbookChartGridlinesFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxisTitle {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "visible")]
    pub visible: bool,
    #[serde(rename = "format")]
    pub format: WorkbookChartAxisTitleFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartLineFormat {
    #[serde(rename = "color")]
    pub color: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxisTitleFormat {
    #[serde(rename = "font")]
    pub font: WorkbookChartFont,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartDataLabelFormat {
    #[serde(rename = "fill")]
    pub fill: WorkbookChartFill,
    #[serde(rename = "font")]
    pub font: WorkbookChartFont,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartGridlinesFormat {
    #[serde(rename = "line")]
    pub line: WorkbookChartLineFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartLegendFormat {
    #[serde(rename = "fill")]
    pub fill: WorkbookChartFill,
    #[serde(rename = "font")]
    pub font: WorkbookChartFont,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartPoint {
    #[serde(rename = "value")]
    pub value: Json,
    #[serde(rename = "format")]
    pub format: WorkbookChartPointFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartPointFormat {
    #[serde(rename = "fill")]
    pub fill: WorkbookChartFill,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartSeriesFormat {
    #[serde(rename = "fill")]
    pub fill: WorkbookChartFill,
    #[serde(rename = "line")]
    pub line: WorkbookChartLineFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartTitleFormat {
    #[serde(rename = "fill")]
    pub fill: WorkbookChartFill,
    #[serde(rename = "font")]
    pub font: WorkbookChartFont,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookCommentReply {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFilter {
    #[serde(rename = "criteria")]
    pub criteria: WorkbookFilterCriteria,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFormatProtection {
    #[serde(rename = "formulaHidden")]
    pub formula_hidden: bool,
    #[serde(rename = "locked")]
    pub locked: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFunctionResult {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "value")]
    pub value: Json,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookPivotTable {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "worksheet")]
    pub worksheet: WorkbookWorksheet,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRange {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "addressLocal")]
    pub address_local: String,
    #[serde(rename = "cellCount")]
    pub cell_count: i32,
    #[serde(rename = "columnCount")]
    pub column_count: i32,
    #[serde(rename = "columnHidden")]
    pub column_hidden: bool,
    #[serde(rename = "columnIndex")]
    pub column_index: i32,
    #[serde(rename = "formulas")]
    pub formulas: Json,
    #[serde(rename = "formulasLocal")]
    pub formulas_local: Json,
    #[serde(rename = "formulasR1C1")]
    pub formulas_r1_c1: Json,
    #[serde(rename = "hidden")]
    pub hidden: bool,
    #[serde(rename = "numberFormat")]
    pub number_format: Json,
    #[serde(rename = "rowCount")]
    pub row_count: i32,
    #[serde(rename = "rowHidden")]
    pub row_hidden: bool,
    #[serde(rename = "rowIndex")]
    pub row_index: i32,
    #[serde(rename = "text")]
    pub text: Json,
    #[serde(rename = "valueTypes")]
    pub value_types: Json,
    #[serde(rename = "values")]
    pub values: Json,
    #[serde(rename = "format")]
    pub format: WorkbookRangeFormat,
    #[serde(rename = "sort")]
    pub sort: WorkbookRangeSort,
    #[serde(rename = "worksheet")]
    pub worksheet: WorkbookWorksheet,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeFormat {
    #[serde(rename = "columnWidth")]
    pub column_width: i64,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: String,
    #[serde(rename = "rowHeight")]
    pub row_height: i64,
    #[serde(rename = "verticalAlignment")]
    pub vertical_alignment: String,
    #[serde(rename = "wrapText")]
    pub wrap_text: bool,
    #[serde(rename = "borders")]
    pub borders: Vec<WorkbookRangeBorder>,
    #[serde(rename = "fill")]
    pub fill: WorkbookRangeFill,
    #[serde(rename = "font")]
    pub font: WorkbookRangeFont,
    #[serde(rename = "protection")]
    pub protection: WorkbookFormatProtection,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeSort {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeBorder {
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "sideIndex")]
    pub side_index: String,
    #[serde(rename = "style")]
    pub style: String,
    #[serde(rename = "weight")]
    pub weight: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeFill {
    #[serde(rename = "color")]
    pub color: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeFont {
    #[serde(rename = "bold")]
    pub bold: bool,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "italic")]
    pub italic: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "underline")]
    pub underline: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeView {
    #[serde(rename = "cellAddresses")]
    pub cell_addresses: Json,
    #[serde(rename = "columnCount")]
    pub column_count: i32,
    #[serde(rename = "formulas")]
    pub formulas: Json,
    #[serde(rename = "formulasLocal")]
    pub formulas_local: Json,
    #[serde(rename = "formulasR1C1")]
    pub formulas_r1_c1: Json,
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "numberFormat")]
    pub number_format: Json,
    #[serde(rename = "rowCount")]
    pub row_count: i32,
    #[serde(rename = "text")]
    pub text: Json,
    #[serde(rename = "valueTypes")]
    pub value_types: Json,
    #[serde(rename = "values")]
    pub values: Json,
    #[serde(rename = "rows")]
    pub rows: Vec<WorkbookRangeView>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTableColumn {
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "values")]
    pub values: Json,
    #[serde(rename = "filter")]
    pub filter: WorkbookFilter,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTableRow {
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "values")]
    pub values: Json,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTableSort {
    #[serde(rename = "fields")]
    pub fields: Vec<WorkbookSortField>,
    #[serde(rename = "matchCase")]
    pub match_case: bool,
    #[serde(rename = "method")]
    pub method: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookWorksheetProtection {
    #[serde(rename = "options")]
    pub options: WorkbookWorksheetProtectionOptions,
    #[serde(rename = "protected")]
    pub protected: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    #[serde(rename = "invitedUserDisplayName")]
    pub invited_user_display_name: String,
    #[serde(rename = "invitedUserType")]
    pub invited_user_type: String,
    #[serde(rename = "invitedUserEmailAddress")]
    pub invited_user_email_address: String,
    #[serde(rename = "invitedUserMessageInfo")]
    pub invited_user_message_info: InvitedUserMessageInfo,
    #[serde(rename = "sendInvitationMessage")]
    pub send_invitation_message: bool,
    #[serde(rename = "inviteRedirectUrl")]
    pub invite_redirect_url: String,
    #[serde(rename = "inviteRedeemUrl")]
    pub invite_redeem_url: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "invitedUser")]
    pub invited_user: User,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerTask {
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "orderHint")]
    pub order_hint: String,
    #[serde(rename = "assigneePriority")]
    pub assignee_priority: String,
    #[serde(rename = "percentComplete")]
    pub percent_complete: i32,
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "dueDateTime")]
    pub due_date_time: String,
    #[serde(rename = "hasDescription")]
    pub has_description: bool,
    #[serde(rename = "previewType")]
    pub preview_type: PlannerPreviewType,
    #[serde(rename = "completedDateTime")]
    pub completed_date_time: String,
    #[serde(rename = "completedBy")]
    pub completed_by: IdentitySet,
    #[serde(rename = "referenceCount")]
    pub reference_count: i32,
    #[serde(rename = "checklistItemCount")]
    pub checklist_item_count: i32,
    #[serde(rename = "activeChecklistItemCount")]
    pub active_checklist_item_count: i32,
    #[serde(rename = "appliedCategories")]
    pub applied_categories: PlannerAppliedCategories,
    #[serde(rename = "assignments")]
    pub assignments: PlannerAssignments,
    #[serde(rename = "conversationThreadId")]
    pub conversation_thread_id: String,
    #[serde(rename = "details")]
    pub details: PlannerTaskDetails,
    #[serde(rename = "assignedToTaskBoardFormat")]
    pub assigned_to_task_board_format: PlannerAssignedToTaskBoardTaskFormat,
    #[serde(rename = "progressTaskBoardFormat")]
    pub progress_task_board_format: PlannerProgressTaskBoardTaskFormat,
    #[serde(rename = "bucketTaskBoardFormat")]
    pub bucket_task_board_format: PlannerBucketTaskBoardTaskFormat,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerPlan {
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "tasks")]
    pub tasks: Vec<PlannerTask>,
    #[serde(rename = "buckets")]
    pub buckets: Vec<PlannerBucket>,
    #[serde(rename = "details")]
    pub details: PlannerPlanDetails,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Planner {
    #[serde(rename = "tasks")]
    pub tasks: Vec<PlannerTask>,
    #[serde(rename = "plans")]
    pub plans: Vec<PlannerPlan>,
    #[serde(rename = "buckets")]
    pub buckets: Vec<PlannerBucket>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerBucket {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "orderHint")]
    pub order_hint: String,
    #[serde(rename = "tasks")]
    pub tasks: Vec<PlannerTask>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerTaskDetails {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "previewType")]
    pub preview_type: PlannerPreviewType,
    #[serde(rename = "references")]
    pub references: PlannerExternalReferences,
    #[serde(rename = "checklist")]
    pub checklist: PlannerChecklistItems,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerAssignedToTaskBoardTaskFormat {
    #[serde(rename = "unassignedOrderHint")]
    pub unassigned_order_hint: String,
    #[serde(rename = "orderHintsByAssignee")]
    pub order_hints_by_assignee: PlannerOrderHintsByAssignee,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerProgressTaskBoardTaskFormat {
    #[serde(rename = "orderHint")]
    pub order_hint: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerBucketTaskBoardTaskFormat {
    #[serde(rename = "orderHint")]
    pub order_hint: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannerPlanDetails {
    #[serde(rename = "sharedWith")]
    pub shared_with: PlannerUserIds,
    #[serde(rename = "categoryDescriptions")]
    pub category_descriptions: PlannerCategoryDescriptions,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenoteEntityBaseModel {
    #[serde(rename = "self")]
    pub _self: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenoteEntitySchemaObjectModel {
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenoteEntityHierarchyModel {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: IdentitySet,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
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
    #[serde(rename = "sections")]
    pub sections: Vec<OnenoteSection>,
    #[serde(rename = "sectionGroups")]
    pub section_groups: Vec<SectionGroup>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenoteSection {
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "links")]
    pub links: SectionLinks,
    #[serde(rename = "pagesUrl")]
    pub pages_url: String,
    #[serde(rename = "parentNotebook")]
    pub parent_notebook: Notebook,
    #[serde(rename = "parentSectionGroup")]
    pub parent_section_group: SectionGroup,
    #[serde(rename = "pages")]
    pub pages: Vec<OnenotePage>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SectionGroup {
    #[serde(rename = "sectionsUrl")]
    pub sections_url: String,
    #[serde(rename = "sectionGroupsUrl")]
    pub section_groups_url: String,
    #[serde(rename = "parentNotebook")]
    pub parent_notebook: Notebook,
    #[serde(rename = "parentSectionGroup")]
    pub parent_section_group: Box<SectionGroup>,
    #[serde(rename = "sections")]
    pub sections: Vec<OnenoteSection>,
    #[serde(rename = "sectionGroups")]
    pub section_groups: Vec<SectionGroup>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenotePage {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "createdByAppId")]
    pub created_by_app_id: String,
    #[serde(rename = "links")]
    pub links: PageLinks,
    #[serde(rename = "contentUrl")]
    pub content_url: String,
    #[serde(rename = "content")]
    pub content: Vec<u8>,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "userTags")]
    pub user_tags: Vec<String>,
    #[serde(rename = "parentSection")]
    pub parent_section: OnenoteSection,
    #[serde(rename = "parentNotebook")]
    pub parent_notebook: Notebook,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenoteResource {
    #[serde(rename = "content")]
    pub content: Vec<u8>,
    #[serde(rename = "contentUrl")]
    pub content_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(rename = "status")]
    pub status: OperationStatus,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastActionDateTime")]
    pub last_action_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnenoteOperation {
    #[serde(rename = "resourceLocation")]
    pub resource_location: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "error")]
    pub error: OnenoteOperationError,
    #[serde(rename = "percentComplete")]
    pub percent_complete: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportRoot {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EducationRoot {
    #[serde(rename = "classes")]
    pub classes: Vec<EducationClass>,
    #[serde(rename = "schools")]
    pub schools: Vec<EducationSchool>,
    #[serde(rename = "users")]
    pub users: Vec<EducationUser>,
    #[serde(rename = "me")]
    pub me: EducationUser,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EducationClass {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "mailNickname")]
    pub mail_nickname: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "classCode")]
    pub class_code: String,
    #[serde(rename = "externalName")]
    pub external_name: String,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "externalSource")]
    pub external_source: EducationExternalSource,
    #[serde(rename = "term")]
    pub term: EducationTerm,
    #[serde(rename = "schools")]
    pub schools: Vec<EducationSchool>,
    #[serde(rename = "members")]
    pub members: Vec<EducationUser>,
    #[serde(rename = "teachers")]
    pub teachers: Vec<EducationUser>,
    #[serde(rename = "group")]
    pub group: Group,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EducationOrganization {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "externalSource")]
    pub external_source: EducationExternalSource,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EducationSchool {
    #[serde(rename = "principalEmail")]
    pub principal_email: String,
    #[serde(rename = "principalName")]
    pub principal_name: String,
    #[serde(rename = "externalPrincipalId")]
    pub external_principal_id: String,
    #[serde(rename = "lowestGrade")]
    pub lowest_grade: String,
    #[serde(rename = "highestGrade")]
    pub highest_grade: String,
    #[serde(rename = "schoolNumber")]
    pub school_number: String,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "phone")]
    pub phone: String,
    #[serde(rename = "fax")]
    pub fax: String,
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "address")]
    pub address: PhysicalAddress,
    #[serde(rename = "classes")]
    pub classes: Vec<EducationClass>,
    #[serde(rename = "users")]
    pub users: Vec<EducationUser>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EducationUser {
    #[serde(rename = "primaryRole")]
    pub primary_role: EducationUserRole,
    #[serde(rename = "middleName")]
    pub middle_name: String,
    #[serde(rename = "externalSource")]
    pub external_source: EducationExternalSource,
    #[serde(rename = "residenceAddress")]
    pub residence_address: PhysicalAddress,
    #[serde(rename = "mailingAddress")]
    pub mailing_address: PhysicalAddress,
    #[serde(rename = "student")]
    pub student: EducationStudent,
    #[serde(rename = "teacher")]
    pub teacher: EducationTeacher,
    #[serde(rename = "createdBy")]
    pub created_by: IdentitySet,
    #[serde(rename = "accountEnabled")]
    pub account_enabled: bool,
    #[serde(rename = "assignedLicenses")]
    pub assigned_licenses: Vec<AssignedLicense>,
    #[serde(rename = "assignedPlans")]
    pub assigned_plans: Vec<AssignedPlan>,
    #[serde(rename = "businessPhones")]
    pub business_phones: Vec<String>,
    #[serde(rename = "department")]
    pub department: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "mail")]
    pub mail: String,
    #[serde(rename = "mailNickname")]
    pub mail_nickname: String,
    #[serde(rename = "mobilePhone")]
    pub mobile_phone: String,
    #[serde(rename = "passwordPolicies")]
    pub password_policies: String,
    #[serde(rename = "passwordProfile")]
    pub password_profile: PasswordProfile,
    #[serde(rename = "officeLocation")]
    pub office_location: String,
    #[serde(rename = "preferredLanguage")]
    pub preferred_language: String,
    #[serde(rename = "provisionedPlans")]
    pub provisioned_plans: Vec<ProvisionedPlan>,
    #[serde(rename = "refreshTokensValidFromDateTime")]
    pub refresh_tokens_valid_from_date_time: String,
    #[serde(rename = "showInAddressList")]
    pub show_in_address_list: bool,
    #[serde(rename = "surname")]
    pub surname: String,
    #[serde(rename = "usageLocation")]
    pub usage_location: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "userType")]
    pub user_type: String,
    #[serde(rename = "schools")]
    pub schools: Vec<EducationSchool>,
    #[serde(rename = "classes")]
    pub classes: Vec<EducationClass>,
    #[serde(rename = "user")]
    pub user: User,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAppManagement {
    #[serde(rename = "microsoftStoreForBusinessLastSuccessfulSyncDateTime")]
    pub microsoft_store_for_business_last_successful_sync_date_time: String,
    #[serde(rename = "isEnabledForMicrosoftStoreForBusiness")]
    pub is_enabled_for_microsoft_store_for_business: bool,
    #[serde(rename = "microsoftStoreForBusinessLanguage")]
    pub microsoft_store_for_business_language: String,
    #[serde(rename = "microsoftStoreForBusinessLastCompletedApplicationSyncTime")]
    pub microsoft_store_for_business_last_completed_application_sync_time: String,
    #[serde(rename = "mobileApps")]
    pub mobile_apps: Vec<MobileApp>,
    #[serde(rename = "mobileAppCategories")]
    pub mobile_app_categories: Vec<MobileAppCategory>,
    #[serde(rename = "mobileAppConfigurations")]
    pub mobile_app_configurations: Vec<ManagedDeviceMobileAppConfiguration>,
    #[serde(rename = "vppTokens")]
    pub vpp_tokens: Vec<VppToken>,
    #[serde(rename = "managedAppPolicies")]
    pub managed_app_policies: Vec<ManagedAppPolicy>,
    #[serde(rename = "iosManagedAppProtections")]
    pub ios_managed_app_protections: Vec<IosManagedAppProtection>,
    #[serde(rename = "androidManagedAppProtections")]
    pub android_managed_app_protections: Vec<AndroidManagedAppProtection>,
    #[serde(rename = "defaultManagedAppProtections")]
    pub default_managed_app_protections: Vec<DefaultManagedAppProtection>,
    #[serde(rename = "targetedManagedAppConfigurations")]
    pub targeted_managed_app_configurations: Vec<TargetedManagedAppConfiguration>,
    #[serde(rename = "mdmWindowsInformationProtectionPolicies")]
    pub mdm_windows_information_protection_policies: Vec<MdmWindowsInformationProtectionPolicy>,
    #[serde(rename = "windowsInformationProtectionPolicies")]
    pub windows_information_protection_policies: Vec<WindowsInformationProtectionPolicy>,
    #[serde(rename = "managedAppRegistrations")]
    pub managed_app_registrations: Vec<ManagedAppRegistration>,
    #[serde(rename = "managedAppStatuses")]
    pub managed_app_statuses: Vec<ManagedAppStatus>,
    #[serde(rename = "managedEBooks")]
    pub managed_e_books: Vec<ManagedEBook>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileApp {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "publisher")]
    pub publisher: String,
    #[serde(rename = "largeIcon")]
    pub large_icon: MimeContent,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "isFeatured")]
    pub is_featured: bool,
    #[serde(rename = "privacyInformationUrl")]
    pub privacy_information_url: String,
    #[serde(rename = "informationUrl")]
    pub information_url: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "developer")]
    pub developer: String,
    #[serde(rename = "notes")]
    pub notes: String,
    #[serde(rename = "publishingState")]
    pub publishing_state: MobileAppPublishingState,
    #[serde(rename = "categories")]
    pub categories: Vec<MobileAppCategory>,
    #[serde(rename = "assignments")]
    pub assignments: Vec<MobileAppAssignment>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileAppCategory {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfiguration {
    #[serde(rename = "targetedMobileApps")]
    pub targeted_mobile_apps: Vec<String>,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "assignments")]
    pub assignments: Vec<ManagedDeviceMobileAppConfigurationAssignment>,
    #[serde(rename = "deviceStatuses")]
    pub device_statuses: Vec<ManagedDeviceMobileAppConfigurationDeviceStatus>,
    #[serde(rename = "userStatuses")]
    pub user_statuses: Vec<ManagedDeviceMobileAppConfigurationUserStatus>,
    #[serde(rename = "deviceStatusSummary")]
    pub device_status_summary: ManagedDeviceMobileAppConfigurationDeviceSummary,
    #[serde(rename = "userStatusSummary")]
    pub user_status_summary: ManagedDeviceMobileAppConfigurationUserSummary,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VppToken {
    #[serde(rename = "organizationName")]
    pub organization_name: String,
    #[serde(rename = "vppTokenAccountType")]
    pub vpp_token_account_type: VppTokenAccountType,
    #[serde(rename = "appleId")]
    pub apple_id: String,
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "lastSyncDateTime")]
    pub last_sync_date_time: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "state")]
    pub state: VppTokenState,
    #[serde(rename = "lastSyncStatus")]
    pub last_sync_status: VppTokenSyncStatus,
    #[serde(rename = "automaticallyUpdateApps")]
    pub automatically_update_apps: bool,
    #[serde(rename = "countryOrRegion")]
    pub country_or_region: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppPolicy {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "version")]
    pub version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppProtection {
    #[serde(rename = "periodOfflineBeforeAccessCheck")]
    pub period_offline_before_access_check: String,
    #[serde(rename = "periodOnlineBeforeAccessCheck")]
    pub period_online_before_access_check: String,
    #[serde(rename = "allowedInboundDataTransferSources")]
    pub allowed_inbound_data_transfer_sources: ManagedAppDataTransferLevel,
    #[serde(rename = "allowedOutboundDataTransferDestinations")]
    pub allowed_outbound_data_transfer_destinations: ManagedAppDataTransferLevel,
    #[serde(rename = "organizationalCredentialsRequired")]
    pub organizational_credentials_required: bool,
    #[serde(rename = "allowedOutboundClipboardSharingLevel")]
    pub allowed_outbound_clipboard_sharing_level: ManagedAppClipboardSharingLevel,
    #[serde(rename = "dataBackupBlocked")]
    pub data_backup_blocked: bool,
    #[serde(rename = "deviceComplianceRequired")]
    pub device_compliance_required: bool,
    #[serde(rename = "managedBrowserToOpenLinksRequired")]
    pub managed_browser_to_open_links_required: bool,
    #[serde(rename = "saveAsBlocked")]
    pub save_as_blocked: bool,
    #[serde(rename = "periodOfflineBeforeWipeIsEnforced")]
    pub period_offline_before_wipe_is_enforced: String,
    #[serde(rename = "pinRequired")]
    pub pin_required: bool,
    #[serde(rename = "maximumPinRetries")]
    pub maximum_pin_retries: i32,
    #[serde(rename = "simplePinBlocked")]
    pub simple_pin_blocked: bool,
    #[serde(rename = "minimumPinLength")]
    pub minimum_pin_length: i32,
    #[serde(rename = "pinCharacterSet")]
    pub pin_character_set: ManagedAppPinCharacterSet,
    #[serde(rename = "periodBeforePinReset")]
    pub period_before_pin_reset: String,
    #[serde(rename = "allowedDataStorageLocations")]
    pub allowed_data_storage_locations: Vec<ManagedAppDataStorageLocation>,
    #[serde(rename = "contactSyncBlocked")]
    pub contact_sync_blocked: bool,
    #[serde(rename = "printBlocked")]
    pub print_blocked: bool,
    #[serde(rename = "fingerprintBlocked")]
    pub fingerprint_blocked: bool,
    #[serde(rename = "disableAppPinIfDevicePinIsSet")]
    pub disable_app_pin_if_device_pin_is_set: bool,
    #[serde(rename = "minimumRequiredOsVersion")]
    pub minimum_required_os_version: String,
    #[serde(rename = "minimumWarningOsVersion")]
    pub minimum_warning_os_version: String,
    #[serde(rename = "minimumRequiredAppVersion")]
    pub minimum_required_app_version: String,
    #[serde(rename = "minimumWarningAppVersion")]
    pub minimum_warning_app_version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetedManagedAppProtection {
    #[serde(rename = "isAssigned")]
    pub is_assigned: bool,
    #[serde(rename = "assignments")]
    pub assignments: Vec<TargetedManagedAppPolicyAssignment>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosManagedAppProtection {
    #[serde(rename = "appDataEncryptionType")]
    pub app_data_encryption_type: ManagedAppDataEncryptionType,
    #[serde(rename = "minimumRequiredSdkVersion")]
    pub minimum_required_sdk_version: String,
    #[serde(rename = "deployedAppCount")]
    pub deployed_app_count: i32,
    #[serde(rename = "faceIdBlocked")]
    pub face_id_blocked: bool,
    #[serde(rename = "apps")]
    pub apps: Vec<ManagedMobileApp>,
    #[serde(rename = "deploymentSummary")]
    pub deployment_summary: ManagedAppPolicyDeploymentSummary,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidManagedAppProtection {
    #[serde(rename = "screenCaptureBlocked")]
    pub screen_capture_blocked: bool,
    #[serde(rename = "disableAppEncryptionIfDeviceEncryptionIsEnabled")]
    pub disable_app_encryption_if_device_encryption_is_enabled: bool,
    #[serde(rename = "encryptAppData")]
    pub encrypt_app_data: bool,
    #[serde(rename = "deployedAppCount")]
    pub deployed_app_count: i32,
    #[serde(rename = "minimumRequiredPatchVersion")]
    pub minimum_required_patch_version: String,
    #[serde(rename = "minimumWarningPatchVersion")]
    pub minimum_warning_patch_version: String,
    #[serde(rename = "apps")]
    pub apps: Vec<ManagedMobileApp>,
    #[serde(rename = "deploymentSummary")]
    pub deployment_summary: ManagedAppPolicyDeploymentSummary,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultManagedAppProtection {
    #[serde(rename = "appDataEncryptionType")]
    pub app_data_encryption_type: ManagedAppDataEncryptionType,
    #[serde(rename = "screenCaptureBlocked")]
    pub screen_capture_blocked: bool,
    #[serde(rename = "encryptAppData")]
    pub encrypt_app_data: bool,
    #[serde(rename = "disableAppEncryptionIfDeviceEncryptionIsEnabled")]
    pub disable_app_encryption_if_device_encryption_is_enabled: bool,
    #[serde(rename = "minimumRequiredSdkVersion")]
    pub minimum_required_sdk_version: String,
    #[serde(rename = "customSettings")]
    pub custom_settings: Vec<KeyValuePair>,
    #[serde(rename = "deployedAppCount")]
    pub deployed_app_count: i32,
    #[serde(rename = "minimumRequiredPatchVersion")]
    pub minimum_required_patch_version: String,
    #[serde(rename = "minimumWarningPatchVersion")]
    pub minimum_warning_patch_version: String,
    #[serde(rename = "faceIdBlocked")]
    pub face_id_blocked: bool,
    #[serde(rename = "apps")]
    pub apps: Vec<ManagedMobileApp>,
    #[serde(rename = "deploymentSummary")]
    pub deployment_summary: ManagedAppPolicyDeploymentSummary,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppConfiguration {
    #[serde(rename = "customSettings")]
    pub custom_settings: Vec<KeyValuePair>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetedManagedAppConfiguration {
    #[serde(rename = "deployedAppCount")]
    pub deployed_app_count: i32,
    #[serde(rename = "isAssigned")]
    pub is_assigned: bool,
    #[serde(rename = "apps")]
    pub apps: Vec<ManagedMobileApp>,
    #[serde(rename = "deploymentSummary")]
    pub deployment_summary: ManagedAppPolicyDeploymentSummary,
    #[serde(rename = "assignments")]
    pub assignments: Vec<TargetedManagedAppPolicyAssignment>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtection {
    #[serde(rename = "enforcementLevel")]
    pub enforcement_level: WindowsInformationProtectionEnforcementLevel,
    #[serde(rename = "enterpriseDomain")]
    pub enterprise_domain: String,
    #[serde(rename = "enterpriseProtectedDomainNames")]
    pub enterprise_protected_domain_names: Vec<WindowsInformationProtectionResourceCollection>,
    #[serde(rename = "protectionUnderLockConfigRequired")]
    pub protection_under_lock_config_required: bool,
    #[serde(rename = "dataRecoveryCertificate")]
    pub data_recovery_certificate: WindowsInformationProtectionDataRecoveryCertificate,
    #[serde(rename = "revokeOnUnenrollDisabled")]
    pub revoke_on_unenroll_disabled: bool,
    #[serde(rename = "rightsManagementServicesTemplateId")]
    pub rights_management_services_template_id: String,
    #[serde(rename = "azureRightsManagementServicesAllowed")]
    pub azure_rights_management_services_allowed: bool,
    #[serde(rename = "iconsVisible")]
    pub icons_visible: bool,
    #[serde(rename = "protectedApps")]
    pub protected_apps: Vec<WindowsInformationProtectionApp>,
    #[serde(rename = "exemptApps")]
    pub exempt_apps: Vec<WindowsInformationProtectionApp>,
    #[serde(rename = "enterpriseNetworkDomainNames")]
    pub enterprise_network_domain_names: Vec<WindowsInformationProtectionResourceCollection>,
    #[serde(rename = "enterpriseProxiedDomains")]
    pub enterprise_proxied_domains: Vec<WindowsInformationProtectionProxiedDomainCollection>,
    #[serde(rename = "enterpriseIPRanges")]
    pub enterprise_i_p_ranges: Vec<WindowsInformationProtectionIPRangeCollection>,
    #[serde(rename = "enterpriseIPRangesAreAuthoritative")]
    pub enterprise_i_p_ranges_are_authoritative: bool,
    #[serde(rename = "enterpriseProxyServers")]
    pub enterprise_proxy_servers: Vec<WindowsInformationProtectionResourceCollection>,
    #[serde(rename = "enterpriseInternalProxyServers")]
    pub enterprise_internal_proxy_servers: Vec<WindowsInformationProtectionResourceCollection>,
    #[serde(rename = "enterpriseProxyServersAreAuthoritative")]
    pub enterprise_proxy_servers_are_authoritative: bool,
    #[serde(rename = "neutralDomainResources")]
    pub neutral_domain_resources: Vec<WindowsInformationProtectionResourceCollection>,
    #[serde(rename = "indexingEncryptedStoresOrItemsBlocked")]
    pub indexing_encrypted_stores_or_items_blocked: bool,
    #[serde(rename = "smbAutoEncryptedFileExtensions")]
    pub smb_auto_encrypted_file_extensions: Vec<WindowsInformationProtectionResourceCollection>,
    #[serde(rename = "isAssigned")]
    pub is_assigned: bool,
    #[serde(rename = "protectedAppLockerFiles")]
    pub protected_app_locker_files: Vec<WindowsInformationProtectionAppLockerFile>,
    #[serde(rename = "exemptAppLockerFiles")]
    pub exempt_app_locker_files: Vec<WindowsInformationProtectionAppLockerFile>,
    #[serde(rename = "assignments")]
    pub assignments: Vec<TargetedManagedAppPolicyAssignment>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdmWindowsInformationProtectionPolicy {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionPolicy {
    #[serde(rename = "revokeOnMdmHandoffDisabled")]
    pub revoke_on_mdm_handoff_disabled: bool,
    #[serde(rename = "mdmEnrollmentUrl")]
    pub mdm_enrollment_url: String,
    #[serde(rename = "windowsHelloForBusinessBlocked")]
    pub windows_hello_for_business_blocked: bool,
    #[serde(rename = "pinMinimumLength")]
    pub pin_minimum_length: i32,
    #[serde(rename = "pinUppercaseLetters")]
    pub pin_uppercase_letters: WindowsInformationProtectionPinCharacterRequirements,
    #[serde(rename = "pinLowercaseLetters")]
    pub pin_lowercase_letters: WindowsInformationProtectionPinCharacterRequirements,
    #[serde(rename = "pinSpecialCharacters")]
    pub pin_special_characters: WindowsInformationProtectionPinCharacterRequirements,
    #[serde(rename = "pinExpirationDays")]
    pub pin_expiration_days: i32,
    #[serde(rename = "numberOfPastPinsRemembered")]
    pub number_of_past_pins_remembered: i32,
    #[serde(rename = "passwordMaximumAttemptCount")]
    pub password_maximum_attempt_count: i32,
    #[serde(rename = "minutesOfInactivityBeforeDeviceLock")]
    pub minutes_of_inactivity_before_device_lock: i32,
    #[serde(rename = "daysWithoutContactBeforeUnenroll")]
    pub days_without_contact_before_unenroll: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppStatus {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "version")]
    pub version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedEBook {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "publisher")]
    pub publisher: String,
    #[serde(rename = "publishedDateTime")]
    pub published_date_time: String,
    #[serde(rename = "largeCover")]
    pub large_cover: MimeContent,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "informationUrl")]
    pub information_url: String,
    #[serde(rename = "privacyInformationUrl")]
    pub privacy_information_url: String,
    #[serde(rename = "assignments")]
    pub assignments: Vec<ManagedEBookAssignment>,
    #[serde(rename = "installSummary")]
    pub install_summary: EBookInstallSummary,
    #[serde(rename = "deviceStates")]
    pub device_states: Vec<DeviceInstallState>,
    #[serde(rename = "userStateSummary")]
    pub user_state_summary: Vec<UserInstallStateSummary>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileAppAssignment {
    #[serde(rename = "intent")]
    pub intent: InstallIntent,
    #[serde(rename = "target")]
    pub target: DeviceAndAppManagementAssignmentTarget,
    #[serde(rename = "settings")]
    pub settings: MobileAppAssignmentSettings,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileAppContentFile {
    #[serde(rename = "azureStorageUri")]
    pub azure_storage_uri: String,
    #[serde(rename = "isCommitted")]
    pub is_committed: bool,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "sizeEncrypted")]
    pub size_encrypted: i64,
    #[serde(rename = "azureStorageUriExpirationDateTime")]
    pub azure_storage_uri_expiration_date_time: String,
    #[serde(rename = "manifest")]
    pub manifest: Vec<u8>,
    #[serde(rename = "uploadState")]
    pub upload_state: MobileAppContentFileUploadState,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationAssignment {
    #[serde(rename = "target")]
    pub target: DeviceAndAppManagementAssignmentTarget,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationDeviceStatus {
    #[serde(rename = "deviceDisplayName")]
    pub device_display_name: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "deviceModel")]
    pub device_model: String,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    pub compliance_grace_period_expiration_date_time: String,
    #[serde(rename = "status")]
    pub status: ComplianceStatus,
    #[serde(rename = "lastReportedDateTime")]
    pub last_reported_date_time: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationUserStatus {
    #[serde(rename = "userDisplayName")]
    pub user_display_name: String,
    #[serde(rename = "devicesCount")]
    pub devices_count: i32,
    #[serde(rename = "status")]
    pub status: ComplianceStatus,
    #[serde(rename = "lastReportedDateTime")]
    pub last_reported_date_time: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationDeviceSummary {
    #[serde(rename = "pendingCount")]
    pub pending_count: i32,
    #[serde(rename = "notApplicableCount")]
    pub not_applicable_count: i32,
    #[serde(rename = "successCount")]
    pub success_count: i32,
    #[serde(rename = "errorCount")]
    pub error_count: i32,
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_date_time: String,
    #[serde(rename = "configurationVersion")]
    pub configuration_version: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationUserSummary {
    #[serde(rename = "pendingCount")]
    pub pending_count: i32,
    #[serde(rename = "notApplicableCount")]
    pub not_applicable_count: i32,
    #[serde(rename = "successCount")]
    pub success_count: i32,
    #[serde(rename = "errorCount")]
    pub error_count: i32,
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_date_time: String,
    #[serde(rename = "configurationVersion")]
    pub configuration_version: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MacOSOfficeSuiteApp {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedApp {
    #[serde(rename = "appAvailability")]
    pub app_availability: ManagedAppAvailability,
    #[serde(rename = "version")]
    pub version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAndroidStoreApp {
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "appStoreUrl")]
    pub app_store_url: String,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: AndroidMinimumOperatingSystem,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIOSStoreApp {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "appStoreUrl")]
    pub app_store_url: String,
    #[serde(rename = "applicableDeviceType")]
    pub applicable_device_type: IosDeviceType,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: IosMinimumOperatingSystem,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedMobileLobApp {
    #[serde(rename = "committedContentVersion")]
    pub committed_content_version: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "contentVersions")]
    pub content_versions: Vec<MobileAppContent>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileAppContent {
    #[serde(rename = "files")]
    pub files: Vec<MobileAppContentFile>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAndroidLobApp {
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: AndroidMinimumOperatingSystem,
    #[serde(rename = "versionName")]
    pub version_name: String,
    #[serde(rename = "versionCode")]
    pub version_code: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIOSLobApp {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "applicableDeviceType")]
    pub applicable_device_type: IosDeviceType,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: IosMinimumOperatingSystem,
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "versionNumber")]
    pub version_number: String,
    #[serde(rename = "buildNumber")]
    pub build_number: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileLobApp {
    #[serde(rename = "committedContentVersion")]
    pub committed_content_version: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "contentVersions")]
    pub content_versions: Vec<MobileAppContent>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsMobileMSI {
    #[serde(rename = "commandLine")]
    pub command_line: String,
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[serde(rename = "productVersion")]
    pub product_version: String,
    #[serde(rename = "ignoreVersionDetection")]
    pub ignore_version_detection: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsUniversalAppX {
    #[serde(rename = "applicableArchitectures")]
    pub applicable_architectures: WindowsArchitecture,
    #[serde(rename = "applicableDeviceTypes")]
    pub applicable_device_types: WindowsDeviceType,
    #[serde(rename = "identityName")]
    pub identity_name: String,
    #[serde(rename = "identityPublisherHash")]
    pub identity_publisher_hash: String,
    #[serde(rename = "identityResourceIdentifier")]
    pub identity_resource_identifier: String,
    #[serde(rename = "isBundle")]
    pub is_bundle: bool,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: WindowsMinimumOperatingSystem,
    #[serde(rename = "identityVersion")]
    pub identity_version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidLobApp {
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: AndroidMinimumOperatingSystem,
    #[serde(rename = "versionName")]
    pub version_name: String,
    #[serde(rename = "versionCode")]
    pub version_code: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosLobApp {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "applicableDeviceType")]
    pub applicable_device_type: IosDeviceType,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: IosMinimumOperatingSystem,
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "versionNumber")]
    pub version_number: String,
    #[serde(rename = "buildNumber")]
    pub build_number: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftStoreForBusinessApp {
    #[serde(rename = "usedLicenseCount")]
    pub used_license_count: i32,
    #[serde(rename = "totalLicenseCount")]
    pub total_license_count: i32,
    #[serde(rename = "productKey")]
    pub product_key: String,
    #[serde(rename = "licenseType")]
    pub license_type: MicrosoftStoreForBusinessLicenseType,
    #[serde(rename = "packageIdentityName")]
    pub package_identity_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebApp {
    #[serde(rename = "appUrl")]
    pub app_url: String,
    #[serde(rename = "useManagedBrowser")]
    pub use_managed_browser: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidStoreApp {
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "appStoreUrl")]
    pub app_store_url: String,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: AndroidMinimumOperatingSystem,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosVppApp {
    #[serde(rename = "usedLicenseCount")]
    pub used_license_count: i32,
    #[serde(rename = "totalLicenseCount")]
    pub total_license_count: i32,
    #[serde(rename = "releaseDateTime")]
    pub release_date_time: String,
    #[serde(rename = "appStoreUrl")]
    pub app_store_url: String,
    #[serde(rename = "licensingType")]
    pub licensing_type: VppLicensingType,
    #[serde(rename = "applicableDeviceType")]
    pub applicable_device_type: IosDeviceType,
    #[serde(rename = "vppTokenOrganizationName")]
    pub vpp_token_organization_name: String,
    #[serde(rename = "vppTokenAccountType")]
    pub vpp_token_account_type: VppTokenAccountType,
    #[serde(rename = "vppTokenAppleId")]
    pub vpp_token_apple_id: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosStoreApp {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "appStoreUrl")]
    pub app_store_url: String,
    #[serde(rename = "applicableDeviceType")]
    pub applicable_device_type: IosDeviceType,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    pub minimum_supported_operating_system: IosMinimumOperatingSystem,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosMobileAppConfiguration {
    #[serde(rename = "encodedSettingXml")]
    pub encoded_setting_xml: Vec<u8>,
    #[serde(rename = "settings")]
    pub settings: Vec<AppConfigurationSettingItem>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagement {
    #[serde(rename = "subscriptionState")]
    pub subscription_state: DeviceManagementSubscriptionState,
    #[serde(rename = "settings")]
    pub settings: DeviceManagementSettings,
    #[serde(rename = "intuneBrand")]
    pub intune_brand: IntuneBrand,
    #[serde(rename = "termsAndConditions")]
    pub terms_and_conditions: Vec<TermsAndConditions>,
    #[serde(rename = "applePushNotificationCertificate")]
    pub apple_push_notification_certificate: ApplePushNotificationCertificate,
    #[serde(rename = "managedDeviceOverview")]
    pub managed_device_overview: ManagedDeviceOverview,
    #[serde(rename = "detectedApps")]
    pub detected_apps: Vec<DetectedApp>,
    #[serde(rename = "managedDevices")]
    pub managed_devices: Vec<ManagedDevice>,
    #[serde(rename = "deviceConfigurations")]
    pub device_configurations: Vec<DeviceConfiguration>,
    #[serde(rename = "deviceCompliancePolicies")]
    pub device_compliance_policies: Vec<DeviceCompliancePolicy>,
    #[serde(rename = "softwareUpdateStatusSummary")]
    pub software_update_status_summary: SoftwareUpdateStatusSummary,
    #[serde(rename = "deviceCompliancePolicyDeviceStateSummary")]
    pub device_compliance_policy_device_state_summary: DeviceCompliancePolicyDeviceStateSummary,
    #[serde(rename = "deviceCompliancePolicySettingStateSummaries")]
    pub device_compliance_policy_setting_state_summaries:
        Vec<DeviceCompliancePolicySettingStateSummary>,
    #[serde(rename = "deviceConfigurationDeviceStateSummaries")]
    pub device_configuration_device_state_summaries: DeviceConfigurationDeviceStateSummary,
    #[serde(rename = "iosUpdateStatuses")]
    pub ios_update_statuses: Vec<IosUpdateDeviceStatus>,
    #[serde(rename = "deviceCategories")]
    pub device_categories: Vec<DeviceCategory>,
    #[serde(rename = "exchangeConnectors")]
    pub exchange_connectors: Vec<DeviceManagementExchangeConnector>,
    #[serde(rename = "deviceEnrollmentConfigurations")]
    pub device_enrollment_configurations: Vec<DeviceEnrollmentConfiguration>,
    #[serde(rename = "conditionalAccessSettings")]
    pub conditional_access_settings: OnPremisesConditionalAccessSettings,
    #[serde(rename = "mobileThreatDefenseConnectors")]
    pub mobile_threat_defense_connectors: Vec<MobileThreatDefenseConnector>,
    #[serde(rename = "deviceManagementPartners")]
    pub device_management_partners: Vec<DeviceManagementPartner>,
    #[serde(rename = "notificationMessageTemplates")]
    pub notification_message_templates: Vec<NotificationMessageTemplate>,
    #[serde(rename = "roleDefinitions")]
    pub role_definitions: Vec<RoleDefinition>,
    #[serde(rename = "roleAssignments")]
    pub role_assignments: Vec<DeviceAndAppManagementRoleAssignment>,
    #[serde(rename = "resourceOperations")]
    pub resource_operations: Vec<ResourceOperation>,
    #[serde(rename = "telecomExpenseManagementPartners")]
    pub telecom_expense_management_partners: Vec<TelecomExpenseManagementPartner>,
    #[serde(rename = "remoteAssistancePartners")]
    pub remote_assistance_partners: Vec<RemoteAssistancePartner>,
    #[serde(rename = "windowsInformationProtectionAppLearningSummaries")]
    pub windows_information_protection_app_learning_summaries:
        Vec<WindowsInformationProtectionAppLearningSummary>,
    #[serde(rename = "windowsInformationProtectionNetworkLearningSummaries")]
    pub windows_information_protection_network_learning_summaries:
        Vec<WindowsInformationProtectionNetworkLearningSummary>,
    #[serde(rename = "troubleshootingEvents")]
    pub troubleshooting_events: Vec<DeviceManagementTroubleshootingEvent>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TermsAndConditions {
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    #[serde(rename = "acceptanceStatement")]
    pub acceptance_statement: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "assignments")]
    pub assignments: Vec<TermsAndConditionsAssignment>,
    #[serde(rename = "acceptanceStatuses")]
    pub acceptance_statuses: Vec<TermsAndConditionsAcceptanceStatus>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplePushNotificationCertificate {
    #[serde(rename = "appleIdentifier")]
    pub apple_identifier: String,
    #[serde(rename = "topicIdentifier")]
    pub topic_identifier: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "certificate")]
    pub certificate: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceOverview {
    #[serde(rename = "enrolledDeviceCount")]
    pub enrolled_device_count: i32,
    #[serde(rename = "mdmEnrolledCount")]
    pub mdm_enrolled_count: i32,
    #[serde(rename = "dualEnrolledDeviceCount")]
    pub dual_enrolled_device_count: i32,
    #[serde(rename = "deviceOperatingSystemSummary")]
    pub device_operating_system_summary: DeviceOperatingSystemSummary,
    #[serde(rename = "deviceExchangeAccessStateSummary")]
    pub device_exchange_access_state_summary: DeviceExchangeAccessStateSummary,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetectedApp {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "sizeInByte")]
    pub size_in_byte: i64,
    #[serde(rename = "deviceCount")]
    pub device_count: i32,
    #[serde(rename = "managedDevices")]
    pub managed_devices: Vec<ManagedDevice>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfiguration {
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "assignments")]
    pub assignments: Vec<DeviceConfigurationAssignment>,
    #[serde(rename = "deviceStatuses")]
    pub device_statuses: Vec<DeviceConfigurationDeviceStatus>,
    #[serde(rename = "userStatuses")]
    pub user_statuses: Vec<DeviceConfigurationUserStatus>,
    #[serde(rename = "deviceStatusOverview")]
    pub device_status_overview: DeviceConfigurationDeviceOverview,
    #[serde(rename = "userStatusOverview")]
    pub user_status_overview: DeviceConfigurationUserOverview,
    #[serde(rename = "deviceSettingStateSummaries")]
    pub device_setting_state_summaries: Vec<SettingStateDeviceSummary>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicy {
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "scheduledActionsForRule")]
    pub scheduled_actions_for_rule: Vec<DeviceComplianceScheduledActionForRule>,
    #[serde(rename = "deviceStatuses")]
    pub device_statuses: Vec<DeviceComplianceDeviceStatus>,
    #[serde(rename = "userStatuses")]
    pub user_statuses: Vec<DeviceComplianceUserStatus>,
    #[serde(rename = "deviceStatusOverview")]
    pub device_status_overview: DeviceComplianceDeviceOverview,
    #[serde(rename = "userStatusOverview")]
    pub user_status_overview: DeviceComplianceUserOverview,
    #[serde(rename = "deviceSettingStateSummaries")]
    pub device_setting_state_summaries: Vec<SettingStateDeviceSummary>,
    #[serde(rename = "assignments")]
    pub assignments: Vec<DeviceCompliancePolicyAssignment>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareUpdateStatusSummary {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "compliantDeviceCount")]
    pub compliant_device_count: i32,
    #[serde(rename = "nonCompliantDeviceCount")]
    pub non_compliant_device_count: i32,
    #[serde(rename = "remediatedDeviceCount")]
    pub remediated_device_count: i32,
    #[serde(rename = "errorDeviceCount")]
    pub error_device_count: i32,
    #[serde(rename = "unknownDeviceCount")]
    pub unknown_device_count: i32,
    #[serde(rename = "conflictDeviceCount")]
    pub conflict_device_count: i32,
    #[serde(rename = "notApplicableDeviceCount")]
    pub not_applicable_device_count: i32,
    #[serde(rename = "compliantUserCount")]
    pub compliant_user_count: i32,
    #[serde(rename = "nonCompliantUserCount")]
    pub non_compliant_user_count: i32,
    #[serde(rename = "remediatedUserCount")]
    pub remediated_user_count: i32,
    #[serde(rename = "errorUserCount")]
    pub error_user_count: i32,
    #[serde(rename = "unknownUserCount")]
    pub unknown_user_count: i32,
    #[serde(rename = "conflictUserCount")]
    pub conflict_user_count: i32,
    #[serde(rename = "notApplicableUserCount")]
    pub not_applicable_user_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicyDeviceStateSummary {
    #[serde(rename = "inGracePeriodCount")]
    pub in_grace_period_count: i32,
    #[serde(rename = "configManagerCount")]
    pub config_manager_count: i32,
    #[serde(rename = "unknownDeviceCount")]
    pub unknown_device_count: i32,
    #[serde(rename = "notApplicableDeviceCount")]
    pub not_applicable_device_count: i32,
    #[serde(rename = "compliantDeviceCount")]
    pub compliant_device_count: i32,
    #[serde(rename = "remediatedDeviceCount")]
    pub remediated_device_count: i32,
    #[serde(rename = "nonCompliantDeviceCount")]
    pub non_compliant_device_count: i32,
    #[serde(rename = "errorDeviceCount")]
    pub error_device_count: i32,
    #[serde(rename = "conflictDeviceCount")]
    pub conflict_device_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicySettingStateSummary {
    #[serde(rename = "setting")]
    pub setting: String,
    #[serde(rename = "settingName")]
    pub setting_name: String,
    #[serde(rename = "platformType")]
    pub platform_type: PolicyPlatformType,
    #[serde(rename = "unknownDeviceCount")]
    pub unknown_device_count: i32,
    #[serde(rename = "notApplicableDeviceCount")]
    pub not_applicable_device_count: i32,
    #[serde(rename = "compliantDeviceCount")]
    pub compliant_device_count: i32,
    #[serde(rename = "remediatedDeviceCount")]
    pub remediated_device_count: i32,
    #[serde(rename = "nonCompliantDeviceCount")]
    pub non_compliant_device_count: i32,
    #[serde(rename = "errorDeviceCount")]
    pub error_device_count: i32,
    #[serde(rename = "conflictDeviceCount")]
    pub conflict_device_count: i32,
    #[serde(rename = "deviceComplianceSettingStates")]
    pub device_compliance_setting_states: Vec<DeviceComplianceSettingState>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationDeviceStateSummary {
    #[serde(rename = "unknownDeviceCount")]
    pub unknown_device_count: i32,
    #[serde(rename = "notApplicableDeviceCount")]
    pub not_applicable_device_count: i32,
    #[serde(rename = "compliantDeviceCount")]
    pub compliant_device_count: i32,
    #[serde(rename = "remediatedDeviceCount")]
    pub remediated_device_count: i32,
    #[serde(rename = "nonCompliantDeviceCount")]
    pub non_compliant_device_count: i32,
    #[serde(rename = "errorDeviceCount")]
    pub error_device_count: i32,
    #[serde(rename = "conflictDeviceCount")]
    pub conflict_device_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosUpdateDeviceStatus {
    #[serde(rename = "installStatus")]
    pub install_status: IosUpdatesInstallStatus,
    #[serde(rename = "osVersion")]
    pub os_version: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "deviceDisplayName")]
    pub device_display_name: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "deviceModel")]
    pub device_model: String,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    pub compliance_grace_period_expiration_date_time: String,
    #[serde(rename = "status")]
    pub status: ComplianceStatus,
    #[serde(rename = "lastReportedDateTime")]
    pub last_reported_date_time: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCategory {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagementExchangeConnector {
    #[serde(rename = "lastSyncDateTime")]
    pub last_sync_date_time: String,
    #[serde(rename = "status")]
    pub status: DeviceManagementExchangeConnectorStatus,
    #[serde(rename = "primarySmtpAddress")]
    pub primary_smtp_address: String,
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[serde(rename = "connectorServerName")]
    pub connector_server_name: String,
    #[serde(rename = "exchangeConnectorType")]
    pub exchange_connector_type: DeviceManagementExchangeConnectorType,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "exchangeAlias")]
    pub exchange_alias: String,
    #[serde(rename = "exchangeOrganization")]
    pub exchange_organization: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentConfiguration {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "assignments")]
    pub assignments: Vec<EnrollmentConfigurationAssignment>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPremisesConditionalAccessSettings {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "includedGroups")]
    pub included_groups: Vec<String>,
    #[serde(rename = "excludedGroups")]
    pub excluded_groups: Vec<String>,
    #[serde(rename = "overrideDefaultRule")]
    pub override_default_rule: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MobileThreatDefenseConnector {
    #[serde(rename = "lastHeartbeatDateTime")]
    pub last_heartbeat_date_time: String,
    #[serde(rename = "partnerState")]
    pub partner_state: MobileThreatPartnerTenantState,
    #[serde(rename = "androidEnabled")]
    pub android_enabled: bool,
    #[serde(rename = "iosEnabled")]
    pub ios_enabled: bool,
    #[serde(rename = "androidDeviceBlockedOnMissingPartnerData")]
    pub android_device_blocked_on_missing_partner_data: bool,
    #[serde(rename = "iosDeviceBlockedOnMissingPartnerData")]
    pub ios_device_blocked_on_missing_partner_data: bool,
    #[serde(rename = "partnerUnsupportedOsVersionBlocked")]
    pub partner_unsupported_os_version_blocked: bool,
    #[serde(rename = "partnerUnresponsivenessThresholdInDays")]
    pub partner_unresponsiveness_threshold_in_days: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagementPartner {
    #[serde(rename = "lastHeartbeatDateTime")]
    pub last_heartbeat_date_time: String,
    #[serde(rename = "partnerState")]
    pub partner_state: DeviceManagementPartnerTenantState,
    #[serde(rename = "partnerAppType")]
    pub partner_app_type: DeviceManagementPartnerAppType,
    #[serde(rename = "singleTenantAppId")]
    pub single_tenant_app_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isConfigured")]
    pub is_configured: bool,
    #[serde(rename = "whenPartnerDevicesWillBeRemovedDateTime")]
    pub when_partner_devices_will_be_removed_date_time: String,
    #[serde(rename = "whenPartnerDevicesWillBeMarkedAsNonCompliantDateTime")]
    pub when_partner_devices_will_be_marked_as_non_compliant_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationMessageTemplate {
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "defaultLocale")]
    pub default_locale: String,
    #[serde(rename = "brandingOptions")]
    pub branding_options: NotificationTemplateBrandingOptions,
    #[serde(rename = "localizedNotificationMessages")]
    pub localized_notification_messages: Vec<LocalizedNotificationMessage>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleDefinition {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "rolePermissions")]
    pub role_permissions: Vec<RolePermission>,
    #[serde(rename = "isBuiltIn")]
    pub is_built_in: bool,
    #[serde(rename = "roleAssignments")]
    pub role_assignments: Vec<RoleAssignment>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "resourceScopes")]
    pub resource_scopes: Vec<String>,
    #[serde(rename = "roleDefinition")]
    pub role_definition: RoleDefinition,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAndAppManagementRoleAssignment {
    #[serde(rename = "members")]
    pub members: Vec<String>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceOperation {
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[serde(rename = "actionName")]
    pub action_name: String,
    #[serde(rename = "description")]
    pub description: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelecomExpenseManagementPartner {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "appAuthorized")]
    pub app_authorized: bool,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "lastConnectionDateTime")]
    pub last_connection_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteAssistancePartner {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "onboardingUrl")]
    pub onboarding_url: String,
    #[serde(rename = "onboardingStatus")]
    pub onboarding_status: RemoteAssistanceOnboardingStatus,
    #[serde(rename = "lastConnectionDateTime")]
    pub last_connection_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionAppLearningSummary {
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[serde(rename = "applicationType")]
    pub application_type: ApplicationType,
    #[serde(rename = "deviceCount")]
    pub device_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionNetworkLearningSummary {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "deviceCount")]
    pub device_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TermsAndConditionsAssignment {
    #[serde(rename = "target")]
    pub target: DeviceAndAppManagementAssignmentTarget,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TermsAndConditionsAcceptanceStatus {
    #[serde(rename = "userDisplayName")]
    pub user_display_name: String,
    #[serde(rename = "acceptedVersion")]
    pub accepted_version: i32,
    #[serde(rename = "acceptedDateTime")]
    pub accepted_date_time: String,
    #[serde(rename = "termsAndConditions")]
    pub terms_and_conditions: TermsAndConditions,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationState {
    #[serde(rename = "settingStates")]
    pub setting_states: Vec<DeviceConfigurationSettingState>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "platformType")]
    pub platform_type: PolicyPlatformType,
    #[serde(rename = "state")]
    pub state: ComplianceStatus,
    #[serde(rename = "settingCount")]
    pub setting_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicyState {
    #[serde(rename = "settingStates")]
    pub setting_states: Vec<DeviceCompliancePolicySettingState>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "platformType")]
    pub platform_type: PolicyPlatformType,
    #[serde(rename = "state")]
    pub state: ComplianceStatus,
    #[serde(rename = "settingCount")]
    pub setting_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationAssignment {
    #[serde(rename = "target")]
    pub target: DeviceAndAppManagementAssignmentTarget,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationDeviceStatus {
    #[serde(rename = "deviceDisplayName")]
    pub device_display_name: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "deviceModel")]
    pub device_model: String,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    pub compliance_grace_period_expiration_date_time: String,
    #[serde(rename = "status")]
    pub status: ComplianceStatus,
    #[serde(rename = "lastReportedDateTime")]
    pub last_reported_date_time: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationUserStatus {
    #[serde(rename = "userDisplayName")]
    pub user_display_name: String,
    #[serde(rename = "devicesCount")]
    pub devices_count: i32,
    #[serde(rename = "status")]
    pub status: ComplianceStatus,
    #[serde(rename = "lastReportedDateTime")]
    pub last_reported_date_time: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationDeviceOverview {
    #[serde(rename = "pendingCount")]
    pub pending_count: i32,
    #[serde(rename = "notApplicableCount")]
    pub not_applicable_count: i32,
    #[serde(rename = "successCount")]
    pub success_count: i32,
    #[serde(rename = "errorCount")]
    pub error_count: i32,
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_date_time: String,
    #[serde(rename = "configurationVersion")]
    pub configuration_version: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationUserOverview {
    #[serde(rename = "pendingCount")]
    pub pending_count: i32,
    #[serde(rename = "notApplicableCount")]
    pub not_applicable_count: i32,
    #[serde(rename = "successCount")]
    pub success_count: i32,
    #[serde(rename = "errorCount")]
    pub error_count: i32,
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_date_time: String,
    #[serde(rename = "configurationVersion")]
    pub configuration_version: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingStateDeviceSummary {
    #[serde(rename = "settingName")]
    pub setting_name: String,
    #[serde(rename = "instancePath")]
    pub instance_path: String,
    #[serde(rename = "unknownDeviceCount")]
    pub unknown_device_count: i32,
    #[serde(rename = "notApplicableDeviceCount")]
    pub not_applicable_device_count: i32,
    #[serde(rename = "compliantDeviceCount")]
    pub compliant_device_count: i32,
    #[serde(rename = "remediatedDeviceCount")]
    pub remediated_device_count: i32,
    #[serde(rename = "nonCompliantDeviceCount")]
    pub non_compliant_device_count: i32,
    #[serde(rename = "errorDeviceCount")]
    pub error_device_count: i32,
    #[serde(rename = "conflictDeviceCount")]
    pub conflict_device_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicyAssignment {
    #[serde(rename = "target")]
    pub target: DeviceAndAppManagementAssignmentTarget,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceScheduledActionForRule {
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    #[serde(rename = "scheduledActionConfigurations")]
    pub scheduled_action_configurations: Vec<DeviceComplianceActionItem>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceDeviceStatus {
    #[serde(rename = "deviceDisplayName")]
    pub device_display_name: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "deviceModel")]
    pub device_model: String,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    pub compliance_grace_period_expiration_date_time: String,
    #[serde(rename = "status")]
    pub status: ComplianceStatus,
    #[serde(rename = "lastReportedDateTime")]
    pub last_reported_date_time: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceUserStatus {
    #[serde(rename = "userDisplayName")]
    pub user_display_name: String,
    #[serde(rename = "devicesCount")]
    pub devices_count: i32,
    #[serde(rename = "status")]
    pub status: ComplianceStatus,
    #[serde(rename = "lastReportedDateTime")]
    pub last_reported_date_time: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceDeviceOverview {
    #[serde(rename = "pendingCount")]
    pub pending_count: i32,
    #[serde(rename = "notApplicableCount")]
    pub not_applicable_count: i32,
    #[serde(rename = "successCount")]
    pub success_count: i32,
    #[serde(rename = "errorCount")]
    pub error_count: i32,
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_date_time: String,
    #[serde(rename = "configurationVersion")]
    pub configuration_version: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceUserOverview {
    #[serde(rename = "pendingCount")]
    pub pending_count: i32,
    #[serde(rename = "notApplicableCount")]
    pub not_applicable_count: i32,
    #[serde(rename = "successCount")]
    pub success_count: i32,
    #[serde(rename = "errorCount")]
    pub error_count: i32,
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_date_time: String,
    #[serde(rename = "configurationVersion")]
    pub configuration_version: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceActionItem {
    #[serde(rename = "gracePeriodHours")]
    pub grace_period_hours: i32,
    #[serde(rename = "actionType")]
    pub action_type: DeviceComplianceActionType,
    #[serde(rename = "notificationTemplateId")]
    pub notification_template_id: String,
    #[serde(rename = "notificationMessageCCList")]
    pub notification_message_c_c_list: Vec<String>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidCustomConfiguration {
    #[serde(rename = "omaSettings")]
    pub oma_settings: Vec<OmaSetting>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidGeneralDeviceConfiguration {
    #[serde(rename = "appsBlockClipboardSharing")]
    pub apps_block_clipboard_sharing: bool,
    #[serde(rename = "appsBlockCopyPaste")]
    pub apps_block_copy_paste: bool,
    #[serde(rename = "appsBlockYouTube")]
    pub apps_block_you_tube: bool,
    #[serde(rename = "bluetoothBlocked")]
    pub bluetooth_blocked: bool,
    #[serde(rename = "cameraBlocked")]
    pub camera_blocked: bool,
    #[serde(rename = "cellularBlockDataRoaming")]
    pub cellular_block_data_roaming: bool,
    #[serde(rename = "cellularBlockMessaging")]
    pub cellular_block_messaging: bool,
    #[serde(rename = "cellularBlockVoiceRoaming")]
    pub cellular_block_voice_roaming: bool,
    #[serde(rename = "cellularBlockWiFiTethering")]
    pub cellular_block_wi_fi_tethering: bool,
    #[serde(rename = "compliantAppsList")]
    pub compliant_apps_list: Vec<AppListItem>,
    #[serde(rename = "compliantAppListType")]
    pub compliant_app_list_type: AppListType,
    #[serde(rename = "diagnosticDataBlockSubmission")]
    pub diagnostic_data_block_submission: bool,
    #[serde(rename = "locationServicesBlocked")]
    pub location_services_blocked: bool,
    #[serde(rename = "googleAccountBlockAutoSync")]
    pub google_account_block_auto_sync: bool,
    #[serde(rename = "googlePlayStoreBlocked")]
    pub google_play_store_blocked: bool,
    #[serde(rename = "kioskModeBlockSleepButton")]
    pub kiosk_mode_block_sleep_button: bool,
    #[serde(rename = "kioskModeBlockVolumeButtons")]
    pub kiosk_mode_block_volume_buttons: bool,
    #[serde(rename = "kioskModeApps")]
    pub kiosk_mode_apps: Vec<AppListItem>,
    #[serde(rename = "nfcBlocked")]
    pub nfc_blocked: bool,
    #[serde(rename = "passwordBlockFingerprintUnlock")]
    pub password_block_fingerprint_unlock: bool,
    #[serde(rename = "passwordBlockTrustAgents")]
    pub password_block_trust_agents: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    pub password_minutes_of_inactivity_before_screen_timeout: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    pub password_sign_in_failure_count_before_factory_reset: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: AndroidRequiredPasswordType,
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "powerOffBlocked")]
    pub power_off_blocked: bool,
    #[serde(rename = "factoryResetBlocked")]
    pub factory_reset_blocked: bool,
    #[serde(rename = "screenCaptureBlocked")]
    pub screen_capture_blocked: bool,
    #[serde(rename = "deviceSharingAllowed")]
    pub device_sharing_allowed: bool,
    #[serde(rename = "storageBlockGoogleBackup")]
    pub storage_block_google_backup: bool,
    #[serde(rename = "storageBlockRemovableStorage")]
    pub storage_block_removable_storage: bool,
    #[serde(rename = "storageRequireDeviceEncryption")]
    pub storage_require_device_encryption: bool,
    #[serde(rename = "storageRequireRemovableStorageEncryption")]
    pub storage_require_removable_storage_encryption: bool,
    #[serde(rename = "voiceAssistantBlocked")]
    pub voice_assistant_blocked: bool,
    #[serde(rename = "voiceDialingBlocked")]
    pub voice_dialing_blocked: bool,
    #[serde(rename = "webBrowserBlockPopups")]
    pub web_browser_block_popups: bool,
    #[serde(rename = "webBrowserBlockAutofill")]
    pub web_browser_block_autofill: bool,
    #[serde(rename = "webBrowserBlockJavaScript")]
    pub web_browser_block_java_script: bool,
    #[serde(rename = "webBrowserBlocked")]
    pub web_browser_blocked: bool,
    #[serde(rename = "webBrowserCookieSettings")]
    pub web_browser_cookie_settings: WebBrowserCookieSettings,
    #[serde(rename = "wiFiBlocked")]
    pub wi_fi_blocked: bool,
    #[serde(rename = "appsInstallAllowList")]
    pub apps_install_allow_list: Vec<AppListItem>,
    #[serde(rename = "appsLaunchBlockList")]
    pub apps_launch_block_list: Vec<AppListItem>,
    #[serde(rename = "appsHideList")]
    pub apps_hide_list: Vec<AppListItem>,
    #[serde(rename = "securityRequireVerifyApps")]
    pub security_require_verify_apps: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidWorkProfileCustomConfiguration {
    #[serde(rename = "omaSettings")]
    pub oma_settings: Vec<OmaSetting>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidWorkProfileGeneralDeviceConfiguration {
    #[serde(rename = "passwordBlockFingerprintUnlock")]
    pub password_block_fingerprint_unlock: bool,
    #[serde(rename = "passwordBlockTrustAgents")]
    pub password_block_trust_agents: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    pub password_minutes_of_inactivity_before_screen_timeout: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    pub password_sign_in_failure_count_before_factory_reset: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: AndroidWorkProfileRequiredPasswordType,
    #[serde(rename = "workProfileDataSharingType")]
    pub work_profile_data_sharing_type: AndroidWorkProfileCrossProfileDataSharingType,
    #[serde(rename = "workProfileBlockNotificationsWhileDeviceLocked")]
    pub work_profile_block_notifications_while_device_locked: bool,
    #[serde(rename = "workProfileBlockAddingAccounts")]
    pub work_profile_block_adding_accounts: bool,
    #[serde(rename = "workProfileBluetoothEnableContactSharing")]
    pub work_profile_bluetooth_enable_contact_sharing: bool,
    #[serde(rename = "workProfileBlockScreenCapture")]
    pub work_profile_block_screen_capture: bool,
    #[serde(rename = "workProfileBlockCrossProfileCallerId")]
    pub work_profile_block_cross_profile_caller_id: bool,
    #[serde(rename = "workProfileBlockCamera")]
    pub work_profile_block_camera: bool,
    #[serde(rename = "workProfileBlockCrossProfileContactsSearch")]
    pub work_profile_block_cross_profile_contacts_search: bool,
    #[serde(rename = "workProfileBlockCrossProfileCopyPaste")]
    pub work_profile_block_cross_profile_copy_paste: bool,
    #[serde(rename = "workProfileDefaultAppPermissionPolicy")]
    pub work_profile_default_app_permission_policy:
        AndroidWorkProfileDefaultAppPermissionPolicyType,
    #[serde(rename = "workProfilePasswordBlockFingerprintUnlock")]
    pub work_profile_password_block_fingerprint_unlock: bool,
    #[serde(rename = "workProfilePasswordBlockTrustAgents")]
    pub work_profile_password_block_trust_agents: bool,
    #[serde(rename = "workProfilePasswordExpirationDays")]
    pub work_profile_password_expiration_days: i32,
    #[serde(rename = "workProfilePasswordMinimumLength")]
    pub work_profile_password_minimum_length: i32,
    #[serde(rename = "workProfilePasswordMinNumericCharacters")]
    pub work_profile_password_min_numeric_characters: i32,
    #[serde(rename = "workProfilePasswordMinNonLetterCharacters")]
    pub work_profile_password_min_non_letter_characters: i32,
    #[serde(rename = "workProfilePasswordMinLetterCharacters")]
    pub work_profile_password_min_letter_characters: i32,
    #[serde(rename = "workProfilePasswordMinLowerCaseCharacters")]
    pub work_profile_password_min_lower_case_characters: i32,
    #[serde(rename = "workProfilePasswordMinUpperCaseCharacters")]
    pub work_profile_password_min_upper_case_characters: i32,
    #[serde(rename = "workProfilePasswordMinSymbolCharacters")]
    pub work_profile_password_min_symbol_characters: i32,
    #[serde(rename = "workProfilePasswordMinutesOfInactivityBeforeScreenTimeout")]
    pub work_profile_password_minutes_of_inactivity_before_screen_timeout: i32,
    #[serde(rename = "workProfilePasswordPreviousPasswordBlockCount")]
    pub work_profile_password_previous_password_block_count: i32,
    #[serde(rename = "workProfilePasswordSignInFailureCountBeforeFactoryReset")]
    pub work_profile_password_sign_in_failure_count_before_factory_reset: i32,
    #[serde(rename = "workProfilePasswordRequiredType")]
    pub work_profile_password_required_type: AndroidWorkProfileRequiredPasswordType,
    #[serde(rename = "workProfileRequirePassword")]
    pub work_profile_require_password: bool,
    #[serde(rename = "securityRequireVerifyApps")]
    pub security_require_verify_apps: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosCertificateProfile {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosCustomConfiguration {
    #[serde(rename = "payloadName")]
    pub payload_name: String,
    #[serde(rename = "payloadFileName")]
    pub payload_file_name: String,
    #[serde(rename = "payload")]
    pub payload: Vec<u8>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosGeneralDeviceConfiguration {
    #[serde(rename = "accountBlockModification")]
    pub account_block_modification: bool,
    #[serde(rename = "activationLockAllowWhenSupervised")]
    pub activation_lock_allow_when_supervised: bool,
    #[serde(rename = "airDropBlocked")]
    pub air_drop_blocked: bool,
    #[serde(rename = "airDropForceUnmanagedDropTarget")]
    pub air_drop_force_unmanaged_drop_target: bool,
    #[serde(rename = "airPlayForcePairingPasswordForOutgoingRequests")]
    pub air_play_force_pairing_password_for_outgoing_requests: bool,
    #[serde(rename = "appleWatchBlockPairing")]
    pub apple_watch_block_pairing: bool,
    #[serde(rename = "appleWatchForceWristDetection")]
    pub apple_watch_force_wrist_detection: bool,
    #[serde(rename = "appleNewsBlocked")]
    pub apple_news_blocked: bool,
    #[serde(rename = "appsSingleAppModeList")]
    pub apps_single_app_mode_list: Vec<AppListItem>,
    #[serde(rename = "appsVisibilityList")]
    pub apps_visibility_list: Vec<AppListItem>,
    #[serde(rename = "appsVisibilityListType")]
    pub apps_visibility_list_type: AppListType,
    #[serde(rename = "appStoreBlockAutomaticDownloads")]
    pub app_store_block_automatic_downloads: bool,
    #[serde(rename = "appStoreBlocked")]
    pub app_store_blocked: bool,
    #[serde(rename = "appStoreBlockInAppPurchases")]
    pub app_store_block_in_app_purchases: bool,
    #[serde(rename = "appStoreBlockUIAppInstallation")]
    pub app_store_block_u_i_app_installation: bool,
    #[serde(rename = "appStoreRequirePassword")]
    pub app_store_require_password: bool,
    #[serde(rename = "bluetoothBlockModification")]
    pub bluetooth_block_modification: bool,
    #[serde(rename = "cameraBlocked")]
    pub camera_blocked: bool,
    #[serde(rename = "cellularBlockDataRoaming")]
    pub cellular_block_data_roaming: bool,
    #[serde(rename = "cellularBlockGlobalBackgroundFetchWhileRoaming")]
    pub cellular_block_global_background_fetch_while_roaming: bool,
    #[serde(rename = "cellularBlockPerAppDataModification")]
    pub cellular_block_per_app_data_modification: bool,
    #[serde(rename = "cellularBlockPersonalHotspot")]
    pub cellular_block_personal_hotspot: bool,
    #[serde(rename = "cellularBlockVoiceRoaming")]
    pub cellular_block_voice_roaming: bool,
    #[serde(rename = "certificatesBlockUntrustedTlsCertificates")]
    pub certificates_block_untrusted_tls_certificates: bool,
    #[serde(rename = "classroomAppBlockRemoteScreenObservation")]
    pub classroom_app_block_remote_screen_observation: bool,
    #[serde(rename = "classroomAppForceUnpromptedScreenObservation")]
    pub classroom_app_force_unprompted_screen_observation: bool,
    #[serde(rename = "compliantAppsList")]
    pub compliant_apps_list: Vec<AppListItem>,
    #[serde(rename = "compliantAppListType")]
    pub compliant_app_list_type: AppListType,
    #[serde(rename = "configurationProfileBlockChanges")]
    pub configuration_profile_block_changes: bool,
    #[serde(rename = "definitionLookupBlocked")]
    pub definition_lookup_blocked: bool,
    #[serde(rename = "deviceBlockEnableRestrictions")]
    pub device_block_enable_restrictions: bool,
    #[serde(rename = "deviceBlockEraseContentAndSettings")]
    pub device_block_erase_content_and_settings: bool,
    #[serde(rename = "deviceBlockNameModification")]
    pub device_block_name_modification: bool,
    #[serde(rename = "diagnosticDataBlockSubmission")]
    pub diagnostic_data_block_submission: bool,
    #[serde(rename = "diagnosticDataBlockSubmissionModification")]
    pub diagnostic_data_block_submission_modification: bool,
    #[serde(rename = "documentsBlockManagedDocumentsInUnmanagedApps")]
    pub documents_block_managed_documents_in_unmanaged_apps: bool,
    #[serde(rename = "documentsBlockUnmanagedDocumentsInManagedApps")]
    pub documents_block_unmanaged_documents_in_managed_apps: bool,
    #[serde(rename = "emailInDomainSuffixes")]
    pub email_in_domain_suffixes: Vec<String>,
    #[serde(rename = "enterpriseAppBlockTrust")]
    pub enterprise_app_block_trust: bool,
    #[serde(rename = "enterpriseAppBlockTrustModification")]
    pub enterprise_app_block_trust_modification: bool,
    #[serde(rename = "faceTimeBlocked")]
    pub face_time_blocked: bool,
    #[serde(rename = "findMyFriendsBlocked")]
    pub find_my_friends_blocked: bool,
    #[serde(rename = "gamingBlockGameCenterFriends")]
    pub gaming_block_game_center_friends: bool,
    #[serde(rename = "gamingBlockMultiplayer")]
    pub gaming_block_multiplayer: bool,
    #[serde(rename = "gameCenterBlocked")]
    pub game_center_blocked: bool,
    #[serde(rename = "hostPairingBlocked")]
    pub host_pairing_blocked: bool,
    #[serde(rename = "iBooksStoreBlocked")]
    pub i_books_store_blocked: bool,
    #[serde(rename = "iBooksStoreBlockErotica")]
    pub i_books_store_block_erotica: bool,
    #[serde(rename = "iCloudBlockActivityContinuation")]
    pub i_cloud_block_activity_continuation: bool,
    #[serde(rename = "iCloudBlockBackup")]
    pub i_cloud_block_backup: bool,
    #[serde(rename = "iCloudBlockDocumentSync")]
    pub i_cloud_block_document_sync: bool,
    #[serde(rename = "iCloudBlockManagedAppsSync")]
    pub i_cloud_block_managed_apps_sync: bool,
    #[serde(rename = "iCloudBlockPhotoLibrary")]
    pub i_cloud_block_photo_library: bool,
    #[serde(rename = "iCloudBlockPhotoStreamSync")]
    pub i_cloud_block_photo_stream_sync: bool,
    #[serde(rename = "iCloudBlockSharedPhotoStream")]
    pub i_cloud_block_shared_photo_stream: bool,
    #[serde(rename = "iCloudRequireEncryptedBackup")]
    pub i_cloud_require_encrypted_backup: bool,
    #[serde(rename = "iTunesBlockExplicitContent")]
    pub i_tunes_block_explicit_content: bool,
    #[serde(rename = "iTunesBlockMusicService")]
    pub i_tunes_block_music_service: bool,
    #[serde(rename = "iTunesBlockRadio")]
    pub i_tunes_block_radio: bool,
    #[serde(rename = "keyboardBlockAutoCorrect")]
    pub keyboard_block_auto_correct: bool,
    #[serde(rename = "keyboardBlockDictation")]
    pub keyboard_block_dictation: bool,
    #[serde(rename = "keyboardBlockPredictive")]
    pub keyboard_block_predictive: bool,
    #[serde(rename = "keyboardBlockShortcuts")]
    pub keyboard_block_shortcuts: bool,
    #[serde(rename = "keyboardBlockSpellCheck")]
    pub keyboard_block_spell_check: bool,
    #[serde(rename = "kioskModeAllowAssistiveSpeak")]
    pub kiosk_mode_allow_assistive_speak: bool,
    #[serde(rename = "kioskModeAllowAssistiveTouchSettings")]
    pub kiosk_mode_allow_assistive_touch_settings: bool,
    #[serde(rename = "kioskModeAllowAutoLock")]
    pub kiosk_mode_allow_auto_lock: bool,
    #[serde(rename = "kioskModeAllowColorInversionSettings")]
    pub kiosk_mode_allow_color_inversion_settings: bool,
    #[serde(rename = "kioskModeAllowRingerSwitch")]
    pub kiosk_mode_allow_ringer_switch: bool,
    #[serde(rename = "kioskModeAllowScreenRotation")]
    pub kiosk_mode_allow_screen_rotation: bool,
    #[serde(rename = "kioskModeAllowSleepButton")]
    pub kiosk_mode_allow_sleep_button: bool,
    #[serde(rename = "kioskModeAllowTouchscreen")]
    pub kiosk_mode_allow_touchscreen: bool,
    #[serde(rename = "kioskModeAllowVoiceOverSettings")]
    pub kiosk_mode_allow_voice_over_settings: bool,
    #[serde(rename = "kioskModeAllowVolumeButtons")]
    pub kiosk_mode_allow_volume_buttons: bool,
    #[serde(rename = "kioskModeAllowZoomSettings")]
    pub kiosk_mode_allow_zoom_settings: bool,
    #[serde(rename = "kioskModeAppStoreUrl")]
    pub kiosk_mode_app_store_url: String,
    #[serde(rename = "kioskModeBuiltInAppId")]
    pub kiosk_mode_built_in_app_id: String,
    #[serde(rename = "kioskModeRequireAssistiveTouch")]
    pub kiosk_mode_require_assistive_touch: bool,
    #[serde(rename = "kioskModeRequireColorInversion")]
    pub kiosk_mode_require_color_inversion: bool,
    #[serde(rename = "kioskModeRequireMonoAudio")]
    pub kiosk_mode_require_mono_audio: bool,
    #[serde(rename = "kioskModeRequireVoiceOver")]
    pub kiosk_mode_require_voice_over: bool,
    #[serde(rename = "kioskModeRequireZoom")]
    pub kiosk_mode_require_zoom: bool,
    #[serde(rename = "kioskModeManagedAppId")]
    pub kiosk_mode_managed_app_id: String,
    #[serde(rename = "lockScreenBlockControlCenter")]
    pub lock_screen_block_control_center: bool,
    #[serde(rename = "lockScreenBlockNotificationView")]
    pub lock_screen_block_notification_view: bool,
    #[serde(rename = "lockScreenBlockPassbook")]
    pub lock_screen_block_passbook: bool,
    #[serde(rename = "lockScreenBlockTodayView")]
    pub lock_screen_block_today_view: bool,
    #[serde(rename = "mediaContentRatingAustralia")]
    pub media_content_rating_australia: MediaContentRatingAustralia,
    #[serde(rename = "mediaContentRatingCanada")]
    pub media_content_rating_canada: MediaContentRatingCanada,
    #[serde(rename = "mediaContentRatingFrance")]
    pub media_content_rating_france: MediaContentRatingFrance,
    #[serde(rename = "mediaContentRatingGermany")]
    pub media_content_rating_germany: MediaContentRatingGermany,
    #[serde(rename = "mediaContentRatingIreland")]
    pub media_content_rating_ireland: MediaContentRatingIreland,
    #[serde(rename = "mediaContentRatingJapan")]
    pub media_content_rating_japan: MediaContentRatingJapan,
    #[serde(rename = "mediaContentRatingNewZealand")]
    pub media_content_rating_new_zealand: MediaContentRatingNewZealand,
    #[serde(rename = "mediaContentRatingUnitedKingdom")]
    pub media_content_rating_united_kingdom: MediaContentRatingUnitedKingdom,
    #[serde(rename = "mediaContentRatingUnitedStates")]
    pub media_content_rating_united_states: MediaContentRatingUnitedStates,
    #[serde(rename = "networkUsageRules")]
    pub network_usage_rules: Vec<IosNetworkUsageRule>,
    #[serde(rename = "mediaContentRatingApps")]
    pub media_content_rating_apps: RatingAppsType,
    #[serde(rename = "messagesBlocked")]
    pub messages_blocked: bool,
    #[serde(rename = "notificationsBlockSettingsModification")]
    pub notifications_block_settings_modification: bool,
    #[serde(rename = "passcodeBlockFingerprintUnlock")]
    pub passcode_block_fingerprint_unlock: bool,
    #[serde(rename = "passcodeBlockFingerprintModification")]
    pub passcode_block_fingerprint_modification: bool,
    #[serde(rename = "passcodeBlockModification")]
    pub passcode_block_modification: bool,
    #[serde(rename = "passcodeBlockSimple")]
    pub passcode_block_simple: bool,
    #[serde(rename = "passcodeExpirationDays")]
    pub passcode_expiration_days: i32,
    #[serde(rename = "passcodeMinimumLength")]
    pub passcode_minimum_length: i32,
    #[serde(rename = "passcodeMinutesOfInactivityBeforeLock")]
    pub passcode_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passcodeMinutesOfInactivityBeforeScreenTimeout")]
    pub passcode_minutes_of_inactivity_before_screen_timeout: i32,
    #[serde(rename = "passcodeMinimumCharacterSetCount")]
    pub passcode_minimum_character_set_count: i32,
    #[serde(rename = "passcodePreviousPasscodeBlockCount")]
    pub passcode_previous_passcode_block_count: i32,
    #[serde(rename = "passcodeSignInFailureCountBeforeWipe")]
    pub passcode_sign_in_failure_count_before_wipe: i32,
    #[serde(rename = "passcodeRequiredType")]
    pub passcode_required_type: RequiredPasswordType,
    #[serde(rename = "passcodeRequired")]
    pub passcode_required: bool,
    #[serde(rename = "podcastsBlocked")]
    pub podcasts_blocked: bool,
    #[serde(rename = "safariBlockAutofill")]
    pub safari_block_autofill: bool,
    #[serde(rename = "safariBlockJavaScript")]
    pub safari_block_java_script: bool,
    #[serde(rename = "safariBlockPopups")]
    pub safari_block_popups: bool,
    #[serde(rename = "safariBlocked")]
    pub safari_blocked: bool,
    #[serde(rename = "safariCookieSettings")]
    pub safari_cookie_settings: WebBrowserCookieSettings,
    #[serde(rename = "safariManagedDomains")]
    pub safari_managed_domains: Vec<String>,
    #[serde(rename = "safariPasswordAutoFillDomains")]
    pub safari_password_auto_fill_domains: Vec<String>,
    #[serde(rename = "safariRequireFraudWarning")]
    pub safari_require_fraud_warning: bool,
    #[serde(rename = "screenCaptureBlocked")]
    pub screen_capture_blocked: bool,
    #[serde(rename = "siriBlocked")]
    pub siri_blocked: bool,
    #[serde(rename = "siriBlockedWhenLocked")]
    pub siri_blocked_when_locked: bool,
    #[serde(rename = "siriBlockUserGeneratedContent")]
    pub siri_block_user_generated_content: bool,
    #[serde(rename = "siriRequireProfanityFilter")]
    pub siri_require_profanity_filter: bool,
    #[serde(rename = "spotlightBlockInternetResults")]
    pub spotlight_block_internet_results: bool,
    #[serde(rename = "voiceDialingBlocked")]
    pub voice_dialing_blocked: bool,
    #[serde(rename = "wallpaperBlockModification")]
    pub wallpaper_block_modification: bool,
    #[serde(rename = "wiFiConnectOnlyToConfiguredNetworks")]
    pub wi_fi_connect_only_to_configured_networks: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosUpdateConfiguration {
    #[serde(rename = "activeHoursStart")]
    pub active_hours_start: String,
    #[serde(rename = "activeHoursEnd")]
    pub active_hours_end: String,
    #[serde(rename = "scheduledInstallDays")]
    pub scheduled_install_days: Vec<DayOfWeek>,
    #[serde(rename = "utcTimeOffsetInMinutes")]
    pub utc_time_offset_in_minutes: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MacOSCustomConfiguration {
    #[serde(rename = "payloadName")]
    pub payload_name: String,
    #[serde(rename = "payloadFileName")]
    pub payload_file_name: String,
    #[serde(rename = "payload")]
    pub payload: Vec<u8>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MacOSGeneralDeviceConfiguration {
    #[serde(rename = "compliantAppsList")]
    pub compliant_apps_list: Vec<AppListItem>,
    #[serde(rename = "compliantAppListType")]
    pub compliant_app_list_type: AppListType,
    #[serde(rename = "emailInDomainSuffixes")]
    pub email_in_domain_suffixes: Vec<String>,
    #[serde(rename = "passwordBlockSimple")]
    pub password_block_simple: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    pub password_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    pub password_minutes_of_inactivity_before_screen_timeout: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppleDeviceFeaturesConfigurationBase {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosDeviceFeaturesConfiguration {
    #[serde(rename = "assetTagTemplate")]
    pub asset_tag_template: String,
    #[serde(rename = "lockScreenFootnote")]
    pub lock_screen_footnote: String,
    #[serde(rename = "homeScreenDockIcons")]
    pub home_screen_dock_icons: Vec<IosHomeScreenItem>,
    #[serde(rename = "homeScreenPages")]
    pub home_screen_pages: Vec<IosHomeScreenPage>,
    #[serde(rename = "notificationSettings")]
    pub notification_settings: Vec<IosNotificationSettings>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MacOSDeviceFeaturesConfiguration {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10EndpointProtectionConfiguration {
    #[serde(rename = "firewallBlockStatefulFTP")]
    pub firewall_block_stateful_f_t_p: bool,
    #[serde(rename = "firewallIdleTimeoutForSecurityAssociationInSeconds")]
    pub firewall_idle_timeout_for_security_association_in_seconds: i32,
    #[serde(rename = "firewallPreSharedKeyEncodingMethod")]
    pub firewall_pre_shared_key_encoding_method: FirewallPreSharedKeyEncodingMethodType,
    #[serde(rename = "firewallIPSecExemptionsAllowNeighborDiscovery")]
    pub firewall_i_p_sec_exemptions_allow_neighbor_discovery: bool,
    #[serde(rename = "firewallIPSecExemptionsAllowICMP")]
    pub firewall_i_p_sec_exemptions_allow_i_c_m_p: bool,
    #[serde(rename = "firewallIPSecExemptionsAllowRouterDiscovery")]
    pub firewall_i_p_sec_exemptions_allow_router_discovery: bool,
    #[serde(rename = "firewallIPSecExemptionsAllowDHCP")]
    pub firewall_i_p_sec_exemptions_allow_d_h_c_p: bool,
    #[serde(rename = "firewallCertificateRevocationListCheckMethod")]
    pub firewall_certificate_revocation_list_check_method:
        FirewallCertificateRevocationListCheckMethodType,
    #[serde(rename = "firewallMergeKeyingModuleSettings")]
    pub firewall_merge_keying_module_settings: bool,
    #[serde(rename = "firewallPacketQueueingMethod")]
    pub firewall_packet_queueing_method: FirewallPacketQueueingMethodType,
    #[serde(rename = "firewallProfileDomain")]
    pub firewall_profile_domain: WindowsFirewallNetworkProfile,
    #[serde(rename = "firewallProfilePublic")]
    pub firewall_profile_public: WindowsFirewallNetworkProfile,
    #[serde(rename = "firewallProfilePrivate")]
    pub firewall_profile_private: WindowsFirewallNetworkProfile,
    #[serde(rename = "defenderAttackSurfaceReductionExcludedPaths")]
    pub defender_attack_surface_reduction_excluded_paths: Vec<String>,
    #[serde(rename = "defenderGuardedFoldersAllowedAppPaths")]
    pub defender_guarded_folders_allowed_app_paths: Vec<String>,
    #[serde(rename = "defenderAdditionalGuardedFolders")]
    pub defender_additional_guarded_folders: Vec<String>,
    #[serde(rename = "defenderExploitProtectionXml")]
    pub defender_exploit_protection_xml: Vec<u8>,
    #[serde(rename = "defenderExploitProtectionXmlFileName")]
    pub defender_exploit_protection_xml_file_name: String,
    #[serde(rename = "defenderSecurityCenterBlockExploitProtectionOverride")]
    pub defender_security_center_block_exploit_protection_override: bool,
    #[serde(rename = "appLockerApplicationControl")]
    pub app_locker_application_control: AppLockerApplicationControlType,
    #[serde(rename = "smartScreenEnableInShell")]
    pub smart_screen_enable_in_shell: bool,
    #[serde(rename = "smartScreenBlockOverrideForFiles")]
    pub smart_screen_block_override_for_files: bool,
    #[serde(rename = "applicationGuardEnabled")]
    pub application_guard_enabled: bool,
    #[serde(rename = "applicationGuardBlockFileTransfer")]
    pub application_guard_block_file_transfer: ApplicationGuardBlockFileTransferType,
    #[serde(rename = "applicationGuardBlockNonEnterpriseContent")]
    pub application_guard_block_non_enterprise_content: bool,
    #[serde(rename = "applicationGuardAllowPersistence")]
    pub application_guard_allow_persistence: bool,
    #[serde(rename = "applicationGuardForceAuditing")]
    pub application_guard_force_auditing: bool,
    #[serde(rename = "applicationGuardBlockClipboardSharing")]
    pub application_guard_block_clipboard_sharing: ApplicationGuardBlockClipboardSharingType,
    #[serde(rename = "applicationGuardAllowPrintToPDF")]
    pub application_guard_allow_print_to_p_d_f: bool,
    #[serde(rename = "applicationGuardAllowPrintToXPS")]
    pub application_guard_allow_print_to_x_p_s: bool,
    #[serde(rename = "applicationGuardAllowPrintToLocalPrinters")]
    pub application_guard_allow_print_to_local_printers: bool,
    #[serde(rename = "applicationGuardAllowPrintToNetworkPrinters")]
    pub application_guard_allow_print_to_network_printers: bool,
    #[serde(rename = "bitLockerDisableWarningForOtherDiskEncryption")]
    pub bit_locker_disable_warning_for_other_disk_encryption: bool,
    #[serde(rename = "bitLockerEnableStorageCardEncryptionOnMobile")]
    pub bit_locker_enable_storage_card_encryption_on_mobile: bool,
    #[serde(rename = "bitLockerEncryptDevice")]
    pub bit_locker_encrypt_device: bool,
    #[serde(rename = "bitLockerRemovableDrivePolicy")]
    pub bit_locker_removable_drive_policy: BitLockerRemovableDrivePolicy,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10GeneralConfiguration {
    #[serde(rename = "enterpriseCloudPrintDiscoveryEndPoint")]
    pub enterprise_cloud_print_discovery_end_point: String,
    #[serde(rename = "enterpriseCloudPrintOAuthAuthority")]
    pub enterprise_cloud_print_o_auth_authority: String,
    #[serde(rename = "enterpriseCloudPrintOAuthClientIdentifier")]
    pub enterprise_cloud_print_o_auth_client_identifier: String,
    #[serde(rename = "enterpriseCloudPrintResourceIdentifier")]
    pub enterprise_cloud_print_resource_identifier: String,
    #[serde(rename = "enterpriseCloudPrintDiscoveryMaxLimit")]
    pub enterprise_cloud_print_discovery_max_limit: i32,
    #[serde(rename = "enterpriseCloudPrintMopriaDiscoveryResourceIdentifier")]
    pub enterprise_cloud_print_mopria_discovery_resource_identifier: String,
    #[serde(rename = "searchBlockDiacritics")]
    pub search_block_diacritics: bool,
    #[serde(rename = "searchDisableAutoLanguageDetection")]
    pub search_disable_auto_language_detection: bool,
    #[serde(rename = "searchDisableIndexingEncryptedItems")]
    pub search_disable_indexing_encrypted_items: bool,
    #[serde(rename = "searchEnableRemoteQueries")]
    pub search_enable_remote_queries: bool,
    #[serde(rename = "searchDisableIndexerBackoff")]
    pub search_disable_indexer_backoff: bool,
    #[serde(rename = "searchDisableIndexingRemovableDrive")]
    pub search_disable_indexing_removable_drive: bool,
    #[serde(rename = "searchEnableAutomaticIndexSizeManangement")]
    pub search_enable_automatic_index_size_manangement: bool,
    #[serde(rename = "diagnosticsDataSubmissionMode")]
    pub diagnostics_data_submission_mode: DiagnosticDataSubmissionMode,
    #[serde(rename = "oneDriveDisableFileSync")]
    pub one_drive_disable_file_sync: bool,
    #[serde(rename = "smartScreenEnableAppInstallControl")]
    pub smart_screen_enable_app_install_control: bool,
    #[serde(rename = "personalizationDesktopImageUrl")]
    pub personalization_desktop_image_url: String,
    #[serde(rename = "personalizationLockScreenImageUrl")]
    pub personalization_lock_screen_image_url: String,
    #[serde(rename = "bluetoothAllowedServices")]
    pub bluetooth_allowed_services: Vec<String>,
    #[serde(rename = "bluetoothBlockAdvertising")]
    pub bluetooth_block_advertising: bool,
    #[serde(rename = "bluetoothBlockDiscoverableMode")]
    pub bluetooth_block_discoverable_mode: bool,
    #[serde(rename = "bluetoothBlockPrePairing")]
    pub bluetooth_block_pre_pairing: bool,
    #[serde(rename = "edgeBlockAutofill")]
    pub edge_block_autofill: bool,
    #[serde(rename = "edgeBlocked")]
    pub edge_blocked: bool,
    #[serde(rename = "edgeCookiePolicy")]
    pub edge_cookie_policy: EdgeCookiePolicy,
    #[serde(rename = "edgeBlockDeveloperTools")]
    pub edge_block_developer_tools: bool,
    #[serde(rename = "edgeBlockSendingDoNotTrackHeader")]
    pub edge_block_sending_do_not_track_header: bool,
    #[serde(rename = "edgeBlockExtensions")]
    pub edge_block_extensions: bool,
    #[serde(rename = "edgeBlockInPrivateBrowsing")]
    pub edge_block_in_private_browsing: bool,
    #[serde(rename = "edgeBlockJavaScript")]
    pub edge_block_java_script: bool,
    #[serde(rename = "edgeBlockPasswordManager")]
    pub edge_block_password_manager: bool,
    #[serde(rename = "edgeBlockAddressBarDropdown")]
    pub edge_block_address_bar_dropdown: bool,
    #[serde(rename = "edgeBlockCompatibilityList")]
    pub edge_block_compatibility_list: bool,
    #[serde(rename = "edgeClearBrowsingDataOnExit")]
    pub edge_clear_browsing_data_on_exit: bool,
    #[serde(rename = "edgeAllowStartPagesModification")]
    pub edge_allow_start_pages_modification: bool,
    #[serde(rename = "edgeDisableFirstRunPage")]
    pub edge_disable_first_run_page: bool,
    #[serde(rename = "edgeBlockLiveTileDataCollection")]
    pub edge_block_live_tile_data_collection: bool,
    #[serde(rename = "edgeSyncFavoritesWithInternetExplorer")]
    pub edge_sync_favorites_with_internet_explorer: bool,
    #[serde(rename = "cellularBlockDataWhenRoaming")]
    pub cellular_block_data_when_roaming: bool,
    #[serde(rename = "cellularBlockVpn")]
    pub cellular_block_vpn: bool,
    #[serde(rename = "cellularBlockVpnWhenRoaming")]
    pub cellular_block_vpn_when_roaming: bool,
    #[serde(rename = "defenderBlockEndUserAccess")]
    pub defender_block_end_user_access: bool,
    #[serde(rename = "defenderDaysBeforeDeletingQuarantinedMalware")]
    pub defender_days_before_deleting_quarantined_malware: i32,
    #[serde(rename = "defenderDetectedMalwareActions")]
    pub defender_detected_malware_actions: DefenderDetectedMalwareActions,
    #[serde(rename = "defenderSystemScanSchedule")]
    pub defender_system_scan_schedule: WeeklySchedule,
    #[serde(rename = "defenderFilesAndFoldersToExclude")]
    pub defender_files_and_folders_to_exclude: Vec<String>,
    #[serde(rename = "defenderFileExtensionsToExclude")]
    pub defender_file_extensions_to_exclude: Vec<String>,
    #[serde(rename = "defenderScanMaxCpu")]
    pub defender_scan_max_cpu: i32,
    #[serde(rename = "defenderMonitorFileActivity")]
    pub defender_monitor_file_activity: DefenderMonitorFileActivity,
    #[serde(rename = "defenderProcessesToExclude")]
    pub defender_processes_to_exclude: Vec<String>,
    #[serde(rename = "defenderPromptForSampleSubmission")]
    pub defender_prompt_for_sample_submission: DefenderPromptForSampleSubmission,
    #[serde(rename = "defenderRequireBehaviorMonitoring")]
    pub defender_require_behavior_monitoring: bool,
    #[serde(rename = "defenderRequireCloudProtection")]
    pub defender_require_cloud_protection: bool,
    #[serde(rename = "defenderRequireNetworkInspectionSystem")]
    pub defender_require_network_inspection_system: bool,
    #[serde(rename = "defenderRequireRealTimeMonitoring")]
    pub defender_require_real_time_monitoring: bool,
    #[serde(rename = "defenderScanArchiveFiles")]
    pub defender_scan_archive_files: bool,
    #[serde(rename = "defenderScanDownloads")]
    pub defender_scan_downloads: bool,
    #[serde(rename = "defenderScanNetworkFiles")]
    pub defender_scan_network_files: bool,
    #[serde(rename = "defenderScanIncomingMail")]
    pub defender_scan_incoming_mail: bool,
    #[serde(rename = "defenderScanMappedNetworkDrivesDuringFullScan")]
    pub defender_scan_mapped_network_drives_during_full_scan: bool,
    #[serde(rename = "defenderScanRemovableDrivesDuringFullScan")]
    pub defender_scan_removable_drives_during_full_scan: bool,
    #[serde(rename = "defenderScanScriptsLoadedInInternetExplorer")]
    pub defender_scan_scripts_loaded_in_internet_explorer: bool,
    #[serde(rename = "defenderSignatureUpdateIntervalInHours")]
    pub defender_signature_update_interval_in_hours: i32,
    #[serde(rename = "defenderScanType")]
    pub defender_scan_type: DefenderScanType,
    #[serde(rename = "defenderScheduledScanTime")]
    pub defender_scheduled_scan_time: String,
    #[serde(rename = "defenderScheduledQuickScanTime")]
    pub defender_scheduled_quick_scan_time: String,
    #[serde(rename = "defenderCloudBlockLevel")]
    pub defender_cloud_block_level: DefenderCloudBlockLevelType,
    #[serde(rename = "lockScreenAllowTimeoutConfiguration")]
    pub lock_screen_allow_timeout_configuration: bool,
    #[serde(rename = "lockScreenBlockActionCenterNotifications")]
    pub lock_screen_block_action_center_notifications: bool,
    #[serde(rename = "lockScreenBlockCortana")]
    pub lock_screen_block_cortana: bool,
    #[serde(rename = "lockScreenBlockToastNotifications")]
    pub lock_screen_block_toast_notifications: bool,
    #[serde(rename = "lockScreenTimeoutInSeconds")]
    pub lock_screen_timeout_in_seconds: i32,
    #[serde(rename = "passwordBlockSimple")]
    pub password_block_simple: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    pub password_minutes_of_inactivity_before_screen_timeout: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "passwordRequireWhenResumeFromIdleState")]
    pub password_require_when_resume_from_idle_state: bool,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    pub password_sign_in_failure_count_before_factory_reset: i32,
    #[serde(rename = "privacyAdvertisingId")]
    pub privacy_advertising_id: StateManagementSetting,
    #[serde(rename = "privacyAutoAcceptPairingAndConsentPrompts")]
    pub privacy_auto_accept_pairing_and_consent_prompts: bool,
    #[serde(rename = "privacyBlockInputPersonalization")]
    pub privacy_block_input_personalization: bool,
    #[serde(rename = "startBlockUnpinningAppsFromTaskbar")]
    pub start_block_unpinning_apps_from_taskbar: bool,
    #[serde(rename = "startMenuAppListVisibility")]
    pub start_menu_app_list_visibility: WindowsStartMenuAppListVisibilityType,
    #[serde(rename = "startMenuHideChangeAccountSettings")]
    pub start_menu_hide_change_account_settings: bool,
    #[serde(rename = "startMenuHideFrequentlyUsedApps")]
    pub start_menu_hide_frequently_used_apps: bool,
    #[serde(rename = "startMenuHideHibernate")]
    pub start_menu_hide_hibernate: bool,
    #[serde(rename = "startMenuHideLock")]
    pub start_menu_hide_lock: bool,
    #[serde(rename = "startMenuHidePowerButton")]
    pub start_menu_hide_power_button: bool,
    #[serde(rename = "startMenuHideRecentJumpLists")]
    pub start_menu_hide_recent_jump_lists: bool,
    #[serde(rename = "startMenuHideRecentlyAddedApps")]
    pub start_menu_hide_recently_added_apps: bool,
    #[serde(rename = "startMenuHideRestartOptions")]
    pub start_menu_hide_restart_options: bool,
    #[serde(rename = "startMenuHideShutDown")]
    pub start_menu_hide_shut_down: bool,
    #[serde(rename = "startMenuHideSignOut")]
    pub start_menu_hide_sign_out: bool,
    #[serde(rename = "startMenuHideSleep")]
    pub start_menu_hide_sleep: bool,
    #[serde(rename = "startMenuHideSwitchAccount")]
    pub start_menu_hide_switch_account: bool,
    #[serde(rename = "startMenuHideUserTile")]
    pub start_menu_hide_user_tile: bool,
    #[serde(rename = "startMenuLayoutEdgeAssetsXml")]
    pub start_menu_layout_edge_assets_xml: Vec<u8>,
    #[serde(rename = "startMenuLayoutXml")]
    pub start_menu_layout_xml: Vec<u8>,
    #[serde(rename = "startMenuMode")]
    pub start_menu_mode: WindowsStartMenuModeType,
    #[serde(rename = "startMenuPinnedFolderDocuments")]
    pub start_menu_pinned_folder_documents: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderDownloads")]
    pub start_menu_pinned_folder_downloads: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderFileExplorer")]
    pub start_menu_pinned_folder_file_explorer: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderHomeGroup")]
    pub start_menu_pinned_folder_home_group: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderMusic")]
    pub start_menu_pinned_folder_music: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderNetwork")]
    pub start_menu_pinned_folder_network: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderPersonalFolder")]
    pub start_menu_pinned_folder_personal_folder: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderPictures")]
    pub start_menu_pinned_folder_pictures: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderSettings")]
    pub start_menu_pinned_folder_settings: VisibilitySetting,
    #[serde(rename = "startMenuPinnedFolderVideos")]
    pub start_menu_pinned_folder_videos: VisibilitySetting,
    #[serde(rename = "settingsBlockSettingsApp")]
    pub settings_block_settings_app: bool,
    #[serde(rename = "settingsBlockSystemPage")]
    pub settings_block_system_page: bool,
    #[serde(rename = "settingsBlockDevicesPage")]
    pub settings_block_devices_page: bool,
    #[serde(rename = "settingsBlockNetworkInternetPage")]
    pub settings_block_network_internet_page: bool,
    #[serde(rename = "settingsBlockPersonalizationPage")]
    pub settings_block_personalization_page: bool,
    #[serde(rename = "settingsBlockAccountsPage")]
    pub settings_block_accounts_page: bool,
    #[serde(rename = "settingsBlockTimeLanguagePage")]
    pub settings_block_time_language_page: bool,
    #[serde(rename = "settingsBlockEaseOfAccessPage")]
    pub settings_block_ease_of_access_page: bool,
    #[serde(rename = "settingsBlockPrivacyPage")]
    pub settings_block_privacy_page: bool,
    #[serde(rename = "settingsBlockUpdateSecurityPage")]
    pub settings_block_update_security_page: bool,
    #[serde(rename = "settingsBlockAppsPage")]
    pub settings_block_apps_page: bool,
    #[serde(rename = "settingsBlockGamingPage")]
    pub settings_block_gaming_page: bool,
    #[serde(rename = "windowsSpotlightBlockConsumerSpecificFeatures")]
    pub windows_spotlight_block_consumer_specific_features: bool,
    #[serde(rename = "windowsSpotlightBlocked")]
    pub windows_spotlight_blocked: bool,
    #[serde(rename = "windowsSpotlightBlockOnActionCenter")]
    pub windows_spotlight_block_on_action_center: bool,
    #[serde(rename = "windowsSpotlightBlockTailoredExperiences")]
    pub windows_spotlight_block_tailored_experiences: bool,
    #[serde(rename = "windowsSpotlightBlockThirdPartyNotifications")]
    pub windows_spotlight_block_third_party_notifications: bool,
    #[serde(rename = "windowsSpotlightBlockWelcomeExperience")]
    pub windows_spotlight_block_welcome_experience: bool,
    #[serde(rename = "windowsSpotlightBlockWindowsTips")]
    pub windows_spotlight_block_windows_tips: bool,
    #[serde(rename = "windowsSpotlightConfigureOnLockScreen")]
    pub windows_spotlight_configure_on_lock_screen: WindowsSpotlightEnablementSettings,
    #[serde(rename = "networkProxyApplySettingsDeviceWide")]
    pub network_proxy_apply_settings_device_wide: bool,
    #[serde(rename = "networkProxyDisableAutoDetect")]
    pub network_proxy_disable_auto_detect: bool,
    #[serde(rename = "networkProxyAutomaticConfigurationUrl")]
    pub network_proxy_automatic_configuration_url: String,
    #[serde(rename = "networkProxyServer")]
    pub network_proxy_server: Windows10NetworkProxyServer,
    #[serde(rename = "accountsBlockAddingNonMicrosoftAccountEmail")]
    pub accounts_block_adding_non_microsoft_account_email: bool,
    #[serde(rename = "antiTheftModeBlocked")]
    pub anti_theft_mode_blocked: bool,
    #[serde(rename = "bluetoothBlocked")]
    pub bluetooth_blocked: bool,
    #[serde(rename = "cameraBlocked")]
    pub camera_blocked: bool,
    #[serde(rename = "connectedDevicesServiceBlocked")]
    pub connected_devices_service_blocked: bool,
    #[serde(rename = "certificatesBlockManualRootCertificateInstallation")]
    pub certificates_block_manual_root_certificate_installation: bool,
    #[serde(rename = "copyPasteBlocked")]
    pub copy_paste_blocked: bool,
    #[serde(rename = "cortanaBlocked")]
    pub cortana_blocked: bool,
    #[serde(rename = "deviceManagementBlockFactoryResetOnMobile")]
    pub device_management_block_factory_reset_on_mobile: bool,
    #[serde(rename = "deviceManagementBlockManualUnenroll")]
    pub device_management_block_manual_unenroll: bool,
    #[serde(rename = "safeSearchFilter")]
    pub safe_search_filter: SafeSearchFilterType,
    #[serde(rename = "edgeBlockPopups")]
    pub edge_block_popups: bool,
    #[serde(rename = "edgeBlockSearchSuggestions")]
    pub edge_block_search_suggestions: bool,
    #[serde(rename = "edgeBlockSendingIntranetTrafficToInternetExplorer")]
    pub edge_block_sending_intranet_traffic_to_internet_explorer: bool,
    #[serde(rename = "edgeSendIntranetTrafficToInternetExplorer")]
    pub edge_send_intranet_traffic_to_internet_explorer: bool,
    #[serde(rename = "edgeRequireSmartScreen")]
    pub edge_require_smart_screen: bool,
    #[serde(rename = "edgeEnterpriseModeSiteListLocation")]
    pub edge_enterprise_mode_site_list_location: String,
    #[serde(rename = "edgeFirstRunUrl")]
    pub edge_first_run_url: String,
    #[serde(rename = "edgeSearchEngine")]
    pub edge_search_engine: EdgeSearchEngineBase,
    #[serde(rename = "edgeHomepageUrls")]
    pub edge_homepage_urls: Vec<String>,
    #[serde(rename = "edgeBlockAccessToAboutFlags")]
    pub edge_block_access_to_about_flags: bool,
    #[serde(rename = "smartScreenBlockPromptOverride")]
    pub smart_screen_block_prompt_override: bool,
    #[serde(rename = "smartScreenBlockPromptOverrideForFiles")]
    pub smart_screen_block_prompt_override_for_files: bool,
    #[serde(rename = "webRtcBlockLocalhostIpAddress")]
    pub web_rtc_block_localhost_ip_address: bool,
    #[serde(rename = "internetSharingBlocked")]
    pub internet_sharing_blocked: bool,
    #[serde(rename = "settingsBlockAddProvisioningPackage")]
    pub settings_block_add_provisioning_package: bool,
    #[serde(rename = "settingsBlockRemoveProvisioningPackage")]
    pub settings_block_remove_provisioning_package: bool,
    #[serde(rename = "settingsBlockChangeSystemTime")]
    pub settings_block_change_system_time: bool,
    #[serde(rename = "settingsBlockEditDeviceName")]
    pub settings_block_edit_device_name: bool,
    #[serde(rename = "settingsBlockChangeRegion")]
    pub settings_block_change_region: bool,
    #[serde(rename = "settingsBlockChangeLanguage")]
    pub settings_block_change_language: bool,
    #[serde(rename = "settingsBlockChangePowerSleep")]
    pub settings_block_change_power_sleep: bool,
    #[serde(rename = "locationServicesBlocked")]
    pub location_services_blocked: bool,
    #[serde(rename = "microsoftAccountBlocked")]
    pub microsoft_account_blocked: bool,
    #[serde(rename = "microsoftAccountBlockSettingsSync")]
    pub microsoft_account_block_settings_sync: bool,
    #[serde(rename = "nfcBlocked")]
    pub nfc_blocked: bool,
    #[serde(rename = "resetProtectionModeBlocked")]
    pub reset_protection_mode_blocked: bool,
    #[serde(rename = "screenCaptureBlocked")]
    pub screen_capture_blocked: bool,
    #[serde(rename = "storageBlockRemovableStorage")]
    pub storage_block_removable_storage: bool,
    #[serde(rename = "storageRequireMobileDeviceEncryption")]
    pub storage_require_mobile_device_encryption: bool,
    #[serde(rename = "usbBlocked")]
    pub usb_blocked: bool,
    #[serde(rename = "voiceRecordingBlocked")]
    pub voice_recording_blocked: bool,
    #[serde(rename = "wiFiBlockAutomaticConnectHotspots")]
    pub wi_fi_block_automatic_connect_hotspots: bool,
    #[serde(rename = "wiFiBlocked")]
    pub wi_fi_blocked: bool,
    #[serde(rename = "wiFiBlockManualConfiguration")]
    pub wi_fi_block_manual_configuration: bool,
    #[serde(rename = "wiFiScanInterval")]
    pub wi_fi_scan_interval: i32,
    #[serde(rename = "wirelessDisplayBlockProjectionToThisDevice")]
    pub wireless_display_block_projection_to_this_device: bool,
    #[serde(rename = "wirelessDisplayBlockUserInputFromReceiver")]
    pub wireless_display_block_user_input_from_receiver: bool,
    #[serde(rename = "wirelessDisplayRequirePinForPairing")]
    pub wireless_display_require_pin_for_pairing: bool,
    #[serde(rename = "windowsStoreBlocked")]
    pub windows_store_blocked: bool,
    #[serde(rename = "appsAllowTrustedAppsSideloading")]
    pub apps_allow_trusted_apps_sideloading: StateManagementSetting,
    #[serde(rename = "windowsStoreBlockAutoUpdate")]
    pub windows_store_block_auto_update: bool,
    #[serde(rename = "developerUnlockSetting")]
    pub developer_unlock_setting: StateManagementSetting,
    #[serde(rename = "sharedUserAppDataAllowed")]
    pub shared_user_app_data_allowed: bool,
    #[serde(rename = "appsBlockWindowsStoreOriginatedApps")]
    pub apps_block_windows_store_originated_apps: bool,
    #[serde(rename = "windowsStoreEnablePrivateStoreOnly")]
    pub windows_store_enable_private_store_only: bool,
    #[serde(rename = "storageRestrictAppDataToSystemVolume")]
    pub storage_restrict_app_data_to_system_volume: bool,
    #[serde(rename = "storageRestrictAppInstallToSystemVolume")]
    pub storage_restrict_app_install_to_system_volume: bool,
    #[serde(rename = "gameDvrBlocked")]
    pub game_dvr_blocked: bool,
    #[serde(rename = "experienceBlockDeviceDiscovery")]
    pub experience_block_device_discovery: bool,
    #[serde(rename = "experienceBlockErrorDialogWhenNoSIM")]
    pub experience_block_error_dialog_when_no_s_i_m: bool,
    #[serde(rename = "experienceBlockTaskSwitcher")]
    pub experience_block_task_switcher: bool,
    #[serde(rename = "logonBlockFastUserSwitching")]
    pub logon_block_fast_user_switching: bool,
    #[serde(rename = "tenantLockdownRequireNetworkDuringOutOfBoxExperience")]
    pub tenant_lockdown_require_network_during_out_of_box_experience: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsDefenderAdvancedThreatProtectionConfiguration {
    #[serde(rename = "allowSampleSharing")]
    pub allow_sample_sharing: bool,
    #[serde(rename = "enableExpeditedTelemetryReporting")]
    pub enable_expedited_telemetry_reporting: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditionUpgradeConfiguration {
    #[serde(rename = "licenseType")]
    pub license_type: EditionUpgradeLicenseType,
    #[serde(rename = "targetEdition")]
    pub target_edition: Windows10EditionType,
    #[serde(rename = "license")]
    pub license: String,
    #[serde(rename = "productKey")]
    pub product_key: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10CustomConfiguration {
    #[serde(rename = "omaSettings")]
    pub oma_settings: Vec<OmaSetting>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10EnterpriseModernAppManagementConfiguration {
    #[serde(rename = "uninstallBuiltInApps")]
    pub uninstall_built_in_apps: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedPCConfiguration {
    #[serde(rename = "accountManagerPolicy")]
    pub account_manager_policy: SharedPCAccountManagerPolicy,
    #[serde(rename = "allowedAccounts")]
    pub allowed_accounts: SharedPCAllowedAccountType,
    #[serde(rename = "allowLocalStorage")]
    pub allow_local_storage: bool,
    #[serde(rename = "disableAccountManager")]
    pub disable_account_manager: bool,
    #[serde(rename = "disableEduPolicies")]
    pub disable_edu_policies: bool,
    #[serde(rename = "disablePowerPolicies")]
    pub disable_power_policies: bool,
    #[serde(rename = "disableSignInOnResume")]
    pub disable_sign_in_on_resume: bool,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "idleTimeBeforeSleepInSeconds")]
    pub idle_time_before_sleep_in_seconds: i32,
    #[serde(rename = "kioskAppDisplayName")]
    pub kiosk_app_display_name: String,
    #[serde(rename = "kioskAppUserModelId")]
    pub kiosk_app_user_model_id: String,
    #[serde(rename = "maintenanceStartTime")]
    pub maintenance_start_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10SecureAssessmentConfiguration {
    #[serde(rename = "launchUri")]
    pub launch_uri: String,
    #[serde(rename = "configurationAccount")]
    pub configuration_account: String,
    #[serde(rename = "allowPrinting")]
    pub allow_printing: bool,
    #[serde(rename = "allowScreenCapture")]
    pub allow_screen_capture: bool,
    #[serde(rename = "allowTextSuggestion")]
    pub allow_text_suggestion: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsPhone81CustomConfiguration {
    #[serde(rename = "omaSettings")]
    pub oma_settings: Vec<OmaSetting>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsUpdateForBusinessConfiguration {
    #[serde(rename = "deliveryOptimizationMode")]
    pub delivery_optimization_mode: WindowsDeliveryOptimizationMode,
    #[serde(rename = "prereleaseFeatures")]
    pub prerelease_features: PrereleaseFeatures,
    #[serde(rename = "automaticUpdateMode")]
    pub automatic_update_mode: AutomaticUpdateMode,
    #[serde(rename = "microsoftUpdateServiceAllowed")]
    pub microsoft_update_service_allowed: bool,
    #[serde(rename = "driversExcluded")]
    pub drivers_excluded: bool,
    #[serde(rename = "installationSchedule")]
    pub installation_schedule: WindowsUpdateInstallScheduleType,
    #[serde(rename = "qualityUpdatesDeferralPeriodInDays")]
    pub quality_updates_deferral_period_in_days: i32,
    #[serde(rename = "featureUpdatesDeferralPeriodInDays")]
    pub feature_updates_deferral_period_in_days: i32,
    #[serde(rename = "qualityUpdatesPaused")]
    pub quality_updates_paused: bool,
    #[serde(rename = "featureUpdatesPaused")]
    pub feature_updates_paused: bool,
    #[serde(rename = "qualityUpdatesPauseExpiryDateTime")]
    pub quality_updates_pause_expiry_date_time: String,
    #[serde(rename = "featureUpdatesPauseExpiryDateTime")]
    pub feature_updates_pause_expiry_date_time: String,
    #[serde(rename = "businessReadyUpdatesOnly")]
    pub business_ready_updates_only: WindowsUpdateType,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows81GeneralConfiguration {
    #[serde(rename = "accountsBlockAddingNonMicrosoftAccountEmail")]
    pub accounts_block_adding_non_microsoft_account_email: bool,
    #[serde(rename = "applyOnlyToWindows81")]
    pub apply_only_to_windows81: bool,
    #[serde(rename = "browserBlockAutofill")]
    pub browser_block_autofill: bool,
    #[serde(rename = "browserBlockAutomaticDetectionOfIntranetSites")]
    pub browser_block_automatic_detection_of_intranet_sites: bool,
    #[serde(rename = "browserBlockEnterpriseModeAccess")]
    pub browser_block_enterprise_mode_access: bool,
    #[serde(rename = "browserBlockJavaScript")]
    pub browser_block_java_script: bool,
    #[serde(rename = "browserBlockPlugins")]
    pub browser_block_plugins: bool,
    #[serde(rename = "browserBlockPopups")]
    pub browser_block_popups: bool,
    #[serde(rename = "browserBlockSendingDoNotTrackHeader")]
    pub browser_block_sending_do_not_track_header: bool,
    #[serde(rename = "browserBlockSingleWordEntryOnIntranetSites")]
    pub browser_block_single_word_entry_on_intranet_sites: bool,
    #[serde(rename = "browserRequireSmartScreen")]
    pub browser_require_smart_screen: bool,
    #[serde(rename = "browserEnterpriseModeSiteListLocation")]
    pub browser_enterprise_mode_site_list_location: String,
    #[serde(rename = "browserInternetSecurityLevel")]
    pub browser_internet_security_level: InternetSiteSecurityLevel,
    #[serde(rename = "browserIntranetSecurityLevel")]
    pub browser_intranet_security_level: SiteSecurityLevel,
    #[serde(rename = "browserLoggingReportLocation")]
    pub browser_logging_report_location: String,
    #[serde(rename = "browserRequireHighSecurityForRestrictedSites")]
    pub browser_require_high_security_for_restricted_sites: bool,
    #[serde(rename = "browserRequireFirewall")]
    pub browser_require_firewall: bool,
    #[serde(rename = "browserRequireFraudWarning")]
    pub browser_require_fraud_warning: bool,
    #[serde(rename = "browserTrustedSitesSecurityLevel")]
    pub browser_trusted_sites_security_level: SiteSecurityLevel,
    #[serde(rename = "cellularBlockDataRoaming")]
    pub cellular_block_data_roaming: bool,
    #[serde(rename = "diagnosticsBlockDataSubmission")]
    pub diagnostics_block_data_submission: bool,
    #[serde(rename = "passwordBlockPicturePasswordAndPin")]
    pub password_block_picture_password_and_pin: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    pub password_minutes_of_inactivity_before_screen_timeout: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    pub password_sign_in_failure_count_before_factory_reset: i32,
    #[serde(rename = "storageRequireDeviceEncryption")]
    pub storage_require_device_encryption: bool,
    #[serde(rename = "updatesRequireAutomaticUpdates")]
    pub updates_require_automatic_updates: bool,
    #[serde(rename = "userAccountControlSettings")]
    pub user_account_control_settings: WindowsUserAccountControlSettings,
    #[serde(rename = "workFoldersUrl")]
    pub work_folders_url: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsPhone81GeneralConfiguration {
    #[serde(rename = "applyOnlyToWindowsPhone81")]
    pub apply_only_to_windows_phone81: bool,
    #[serde(rename = "appsBlockCopyPaste")]
    pub apps_block_copy_paste: bool,
    #[serde(rename = "bluetoothBlocked")]
    pub bluetooth_blocked: bool,
    #[serde(rename = "cameraBlocked")]
    pub camera_blocked: bool,
    #[serde(rename = "cellularBlockWifiTethering")]
    pub cellular_block_wifi_tethering: bool,
    #[serde(rename = "compliantAppsList")]
    pub compliant_apps_list: Vec<AppListItem>,
    #[serde(rename = "compliantAppListType")]
    pub compliant_app_list_type: AppListType,
    #[serde(rename = "diagnosticDataBlockSubmission")]
    pub diagnostic_data_block_submission: bool,
    #[serde(rename = "emailBlockAddingAccounts")]
    pub email_block_adding_accounts: bool,
    #[serde(rename = "locationServicesBlocked")]
    pub location_services_blocked: bool,
    #[serde(rename = "microsoftAccountBlocked")]
    pub microsoft_account_blocked: bool,
    #[serde(rename = "nfcBlocked")]
    pub nfc_blocked: bool,
    #[serde(rename = "passwordBlockSimple")]
    pub password_block_simple: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    pub password_minutes_of_inactivity_before_screen_timeout: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    pub password_sign_in_failure_count_before_factory_reset: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "screenCaptureBlocked")]
    pub screen_capture_blocked: bool,
    #[serde(rename = "storageBlockRemovableStorage")]
    pub storage_block_removable_storage: bool,
    #[serde(rename = "storageRequireEncryption")]
    pub storage_require_encryption: bool,
    #[serde(rename = "webBrowserBlocked")]
    pub web_browser_blocked: bool,
    #[serde(rename = "wifiBlocked")]
    pub wifi_blocked: bool,
    #[serde(rename = "wifiBlockAutomaticConnectHotspots")]
    pub wifi_block_automatic_connect_hotspots: bool,
    #[serde(rename = "wifiBlockHotspotReporting")]
    pub wifi_block_hotspot_reporting: bool,
    #[serde(rename = "windowsStoreBlocked")]
    pub windows_store_blocked: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10TeamGeneralConfiguration {
    #[serde(rename = "azureOperationalInsightsBlockTelemetry")]
    pub azure_operational_insights_block_telemetry: bool,
    #[serde(rename = "azureOperationalInsightsWorkspaceId")]
    pub azure_operational_insights_workspace_id: String,
    #[serde(rename = "azureOperationalInsightsWorkspaceKey")]
    pub azure_operational_insights_workspace_key: String,
    #[serde(rename = "connectAppBlockAutoLaunch")]
    pub connect_app_block_auto_launch: bool,
    #[serde(rename = "maintenanceWindowBlocked")]
    pub maintenance_window_blocked: bool,
    #[serde(rename = "maintenanceWindowDurationInHours")]
    pub maintenance_window_duration_in_hours: i32,
    #[serde(rename = "maintenanceWindowStartTime")]
    pub maintenance_window_start_time: String,
    #[serde(rename = "miracastChannel")]
    pub miracast_channel: MiracastChannel,
    #[serde(rename = "miracastBlocked")]
    pub miracast_blocked: bool,
    #[serde(rename = "miracastRequirePin")]
    pub miracast_require_pin: bool,
    #[serde(rename = "settingsBlockMyMeetingsAndFiles")]
    pub settings_block_my_meetings_and_files: bool,
    #[serde(rename = "settingsBlockSessionResume")]
    pub settings_block_session_resume: bool,
    #[serde(rename = "settingsBlockSigninSuggestions")]
    pub settings_block_signin_suggestions: bool,
    #[serde(rename = "settingsDefaultVolume")]
    pub settings_default_volume: i32,
    #[serde(rename = "settingsScreenTimeoutInMinutes")]
    pub settings_screen_timeout_in_minutes: i32,
    #[serde(rename = "settingsSessionTimeoutInMinutes")]
    pub settings_session_timeout_in_minutes: i32,
    #[serde(rename = "settingsSleepTimeoutInMinutes")]
    pub settings_sleep_timeout_in_minutes: i32,
    #[serde(rename = "welcomeScreenBlockAutomaticWakeUp")]
    pub welcome_screen_block_automatic_wake_up: bool,
    #[serde(rename = "welcomeScreenBackgroundImageUrl")]
    pub welcome_screen_background_image_url: String,
    #[serde(rename = "welcomeScreenMeetingInformation")]
    pub welcome_screen_meeting_information: WelcomeScreenMeetingInformation,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidCompliancePolicy {
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: AndroidRequiredPasswordType,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    pub password_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "securityPreventInstallAppsFromUnknownSources")]
    pub security_prevent_install_apps_from_unknown_sources: bool,
    #[serde(rename = "securityDisableUsbDebugging")]
    pub security_disable_usb_debugging: bool,
    #[serde(rename = "securityRequireVerifyApps")]
    pub security_require_verify_apps: bool,
    #[serde(rename = "deviceThreatProtectionEnabled")]
    pub device_threat_protection_enabled: bool,
    #[serde(rename = "deviceThreatProtectionRequiredSecurityLevel")]
    pub device_threat_protection_required_security_level: DeviceThreatProtectionLevel,
    #[serde(rename = "securityBlockJailbrokenDevices")]
    pub security_block_jailbroken_devices: bool,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
    #[serde(rename = "minAndroidSecurityPatchLevel")]
    pub min_android_security_patch_level: String,
    #[serde(rename = "storageRequireEncryption")]
    pub storage_require_encryption: bool,
    #[serde(rename = "securityRequireSafetyNetAttestationBasicIntegrity")]
    pub security_require_safety_net_attestation_basic_integrity: bool,
    #[serde(rename = "securityRequireSafetyNetAttestationCertifiedDevice")]
    pub security_require_safety_net_attestation_certified_device: bool,
    #[serde(rename = "securityRequireGooglePlayServices")]
    pub security_require_google_play_services: bool,
    #[serde(rename = "securityRequireUpToDateSecurityProviders")]
    pub security_require_up_to_date_security_providers: bool,
    #[serde(rename = "securityRequireCompanyPortalAppIntegrity")]
    pub security_require_company_portal_app_integrity: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidWorkProfileCompliancePolicy {
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: AndroidRequiredPasswordType,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    pub password_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "securityPreventInstallAppsFromUnknownSources")]
    pub security_prevent_install_apps_from_unknown_sources: bool,
    #[serde(rename = "securityDisableUsbDebugging")]
    pub security_disable_usb_debugging: bool,
    #[serde(rename = "securityRequireVerifyApps")]
    pub security_require_verify_apps: bool,
    #[serde(rename = "deviceThreatProtectionEnabled")]
    pub device_threat_protection_enabled: bool,
    #[serde(rename = "deviceThreatProtectionRequiredSecurityLevel")]
    pub device_threat_protection_required_security_level: DeviceThreatProtectionLevel,
    #[serde(rename = "securityBlockJailbrokenDevices")]
    pub security_block_jailbroken_devices: bool,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
    #[serde(rename = "minAndroidSecurityPatchLevel")]
    pub min_android_security_patch_level: String,
    #[serde(rename = "storageRequireEncryption")]
    pub storage_require_encryption: bool,
    #[serde(rename = "securityRequireSafetyNetAttestationBasicIntegrity")]
    pub security_require_safety_net_attestation_basic_integrity: bool,
    #[serde(rename = "securityRequireSafetyNetAttestationCertifiedDevice")]
    pub security_require_safety_net_attestation_certified_device: bool,
    #[serde(rename = "securityRequireGooglePlayServices")]
    pub security_require_google_play_services: bool,
    #[serde(rename = "securityRequireUpToDateSecurityProviders")]
    pub security_require_up_to_date_security_providers: bool,
    #[serde(rename = "securityRequireCompanyPortalAppIntegrity")]
    pub security_require_company_portal_app_integrity: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosCompliancePolicy {
    #[serde(rename = "passcodeBlockSimple")]
    pub passcode_block_simple: bool,
    #[serde(rename = "passcodeExpirationDays")]
    pub passcode_expiration_days: i32,
    #[serde(rename = "passcodeMinimumLength")]
    pub passcode_minimum_length: i32,
    #[serde(rename = "passcodeMinutesOfInactivityBeforeLock")]
    pub passcode_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passcodePreviousPasscodeBlockCount")]
    pub passcode_previous_passcode_block_count: i32,
    #[serde(rename = "passcodeMinimumCharacterSetCount")]
    pub passcode_minimum_character_set_count: i32,
    #[serde(rename = "passcodeRequiredType")]
    pub passcode_required_type: RequiredPasswordType,
    #[serde(rename = "passcodeRequired")]
    pub passcode_required: bool,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
    #[serde(rename = "securityBlockJailbrokenDevices")]
    pub security_block_jailbroken_devices: bool,
    #[serde(rename = "deviceThreatProtectionEnabled")]
    pub device_threat_protection_enabled: bool,
    #[serde(rename = "deviceThreatProtectionRequiredSecurityLevel")]
    pub device_threat_protection_required_security_level: DeviceThreatProtectionLevel,
    #[serde(rename = "managedEmailProfileRequired")]
    pub managed_email_profile_required: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MacOSCompliancePolicy {
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "passwordBlockSimple")]
    pub password_block_simple: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    pub password_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
    #[serde(rename = "systemIntegrityProtectionEnabled")]
    pub system_integrity_protection_enabled: bool,
    #[serde(rename = "deviceThreatProtectionEnabled")]
    pub device_threat_protection_enabled: bool,
    #[serde(rename = "deviceThreatProtectionRequiredSecurityLevel")]
    pub device_threat_protection_required_security_level: DeviceThreatProtectionLevel,
    #[serde(rename = "storageRequireEncryption")]
    pub storage_require_encryption: bool,
    #[serde(rename = "firewallEnabled")]
    pub firewall_enabled: bool,
    #[serde(rename = "firewallBlockAllIncoming")]
    pub firewall_block_all_incoming: bool,
    #[serde(rename = "firewallEnableStealthMode")]
    pub firewall_enable_stealth_mode: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10CompliancePolicy {
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "passwordBlockSimple")]
    pub password_block_simple: bool,
    #[serde(rename = "passwordRequiredToUnlockFromIdle")]
    pub password_required_to_unlock_from_idle: bool,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    pub password_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "requireHealthyDeviceReport")]
    pub require_healthy_device_report: bool,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
    #[serde(rename = "mobileOsMinimumVersion")]
    pub mobile_os_minimum_version: String,
    #[serde(rename = "mobileOsMaximumVersion")]
    pub mobile_os_maximum_version: String,
    #[serde(rename = "earlyLaunchAntiMalwareDriverEnabled")]
    pub early_launch_anti_malware_driver_enabled: bool,
    #[serde(rename = "bitLockerEnabled")]
    pub bit_locker_enabled: bool,
    #[serde(rename = "secureBootEnabled")]
    pub secure_boot_enabled: bool,
    #[serde(rename = "codeIntegrityEnabled")]
    pub code_integrity_enabled: bool,
    #[serde(rename = "storageRequireEncryption")]
    pub storage_require_encryption: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows10MobileCompliancePolicy {
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "passwordBlockSimple")]
    pub password_block_simple: bool,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    pub password_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passwordRequireToUnlockFromIdle")]
    pub password_require_to_unlock_from_idle: bool,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
    #[serde(rename = "earlyLaunchAntiMalwareDriverEnabled")]
    pub early_launch_anti_malware_driver_enabled: bool,
    #[serde(rename = "bitLockerEnabled")]
    pub bit_locker_enabled: bool,
    #[serde(rename = "secureBootEnabled")]
    pub secure_boot_enabled: bool,
    #[serde(rename = "codeIntegrityEnabled")]
    pub code_integrity_enabled: bool,
    #[serde(rename = "storageRequireEncryption")]
    pub storage_require_encryption: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Windows81CompliancePolicy {
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "passwordBlockSimple")]
    pub password_block_simple: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    pub password_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
    #[serde(rename = "storageRequireEncryption")]
    pub storage_require_encryption: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsPhone81CompliancePolicy {
    #[serde(rename = "passwordBlockSimple")]
    pub password_block_simple: bool,
    #[serde(rename = "passwordExpirationDays")]
    pub password_expiration_days: i32,
    #[serde(rename = "passwordMinimumLength")]
    pub password_minimum_length: i32,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    pub password_minutes_of_inactivity_before_lock: i32,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    pub password_minimum_character_set_count: i32,
    #[serde(rename = "passwordRequiredType")]
    pub password_required_type: RequiredPasswordType,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    pub password_previous_password_block_count: i32,
    #[serde(rename = "passwordRequired")]
    pub password_required: bool,
    #[serde(rename = "osMinimumVersion")]
    pub os_minimum_version: String,
    #[serde(rename = "osMaximumVersion")]
    pub os_maximum_version: String,
    #[serde(rename = "storageRequireEncryption")]
    pub storage_require_encryption: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceSettingState {
    #[serde(rename = "setting")]
    pub setting: String,
    #[serde(rename = "settingName")]
    pub setting_name: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "deviceName")]
    pub device_name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "deviceModel")]
    pub device_model: String,
    #[serde(rename = "state")]
    pub state: ComplianceStatus,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    pub compliance_grace_period_expiration_date_time: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentConfigurationAssignment {
    #[serde(rename = "target")]
    pub target: DeviceAndAppManagementAssignmentTarget,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentLimitConfiguration {
    #[serde(rename = "limit")]
    pub limit: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentPlatformRestrictionsConfiguration {
    #[serde(rename = "iosRestriction")]
    pub ios_restriction: DeviceEnrollmentPlatformRestriction,
    #[serde(rename = "windowsRestriction")]
    pub windows_restriction: DeviceEnrollmentPlatformRestriction,
    #[serde(rename = "windowsMobileRestriction")]
    pub windows_mobile_restriction: DeviceEnrollmentPlatformRestriction,
    #[serde(rename = "androidRestriction")]
    pub android_restriction: DeviceEnrollmentPlatformRestriction,
    #[serde(rename = "macOSRestriction")]
    pub mac_o_s_restriction: DeviceEnrollmentPlatformRestriction,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentWindowsHelloForBusinessConfiguration {
    #[serde(rename = "pinMinimumLength")]
    pub pin_minimum_length: i32,
    #[serde(rename = "pinMaximumLength")]
    pub pin_maximum_length: i32,
    #[serde(rename = "pinUppercaseCharactersUsage")]
    pub pin_uppercase_characters_usage: WindowsHelloForBusinessPinUsage,
    #[serde(rename = "pinLowercaseCharactersUsage")]
    pub pin_lowercase_characters_usage: WindowsHelloForBusinessPinUsage,
    #[serde(rename = "pinSpecialCharactersUsage")]
    pub pin_special_characters_usage: WindowsHelloForBusinessPinUsage,
    #[serde(rename = "state")]
    pub state: Enablement,
    #[serde(rename = "securityDeviceRequired")]
    pub security_device_required: bool,
    #[serde(rename = "unlockWithBiometricsEnabled")]
    pub unlock_with_biometrics_enabled: bool,
    #[serde(rename = "remotePassportEnabled")]
    pub remote_passport_enabled: bool,
    #[serde(rename = "pinPreviousBlockCount")]
    pub pin_previous_block_count: i32,
    #[serde(rename = "pinExpirationInDays")]
    pub pin_expiration_in_days: i32,
    #[serde(rename = "enhancedBiometricsState")]
    pub enhanced_biometrics_state: Enablement,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedMobileApp {
    #[serde(rename = "mobileAppIdentifier")]
    pub mobile_app_identifier: MobileAppIdentifier,
    #[serde(rename = "version")]
    pub version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetedManagedAppPolicyAssignment {
    #[serde(rename = "target")]
    pub target: DeviceAndAppManagementAssignmentTarget,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppOperation {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "version")]
    pub version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppPolicyDeploymentSummary {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "configurationDeployedUserCount")]
    pub configuration_deployed_user_count: i32,
    #[serde(rename = "lastRefreshTime")]
    pub last_refresh_time: String,
    #[serde(rename = "configurationDeploymentSummaryPerApp")]
    pub configuration_deployment_summary_per_app: Vec<ManagedAppPolicyDeploymentSummaryPerApp>,
    #[serde(rename = "version")]
    pub version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionAppLockerFile {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fileHash")]
    pub file_hash: String,
    #[serde(rename = "file")]
    pub file: Vec<u8>,
    #[serde(rename = "version")]
    pub version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosManagedAppRegistration {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AndroidManagedAppRegistration {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppStatusRaw {
    #[serde(rename = "content")]
    pub content: Json,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizedNotificationMessage {
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "locale")]
    pub locale: String,
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "messageTemplate")]
    pub message_template: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceAndAppManagementRoleDefinition {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedEBookAssignment {
    #[serde(rename = "target")]
    pub target: DeviceAndAppManagementAssignmentTarget,
    #[serde(rename = "installIntent")]
    pub install_intent: InstallIntent,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EBookInstallSummary {
    #[serde(rename = "installedDeviceCount")]
    pub installed_device_count: i32,
    #[serde(rename = "failedDeviceCount")]
    pub failed_device_count: i32,
    #[serde(rename = "notInstalledDeviceCount")]
    pub not_installed_device_count: i32,
    #[serde(rename = "installedUserCount")]
    pub installed_user_count: i32,
    #[serde(rename = "failedUserCount")]
    pub failed_user_count: i32,
    #[serde(rename = "notInstalledUserCount")]
    pub not_installed_user_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceInstallState {
    #[serde(rename = "deviceName")]
    pub device_name: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "lastSyncDateTime")]
    pub last_sync_date_time: String,
    #[serde(rename = "installState")]
    pub install_state: InstallState,
    #[serde(rename = "errorCode")]
    pub error_code: String,
    #[serde(rename = "osVersion")]
    pub os_version: String,
    #[serde(rename = "osDescription")]
    pub os_description: String,
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInstallStateSummary {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "installedDeviceCount")]
    pub installed_device_count: i32,
    #[serde(rename = "failedDeviceCount")]
    pub failed_device_count: i32,
    #[serde(rename = "notInstalledDeviceCount")]
    pub not_installed_device_count: i32,
    #[serde(rename = "deviceStates")]
    pub device_states: Vec<DeviceInstallState>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosVppEBookAssignment {}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IosVppEBook {
    #[serde(rename = "vppTokenId")]
    pub vpp_token_id: String,
    #[serde(rename = "appleId")]
    pub apple_id: String,
    #[serde(rename = "vppOrganizationName")]
    pub vpp_organization_name: String,
    #[serde(rename = "genres")]
    pub genres: Vec<String>,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "seller")]
    pub seller: String,
    #[serde(rename = "totalLicenseCount")]
    pub total_license_count: i32,
    #[serde(rename = "usedLicenseCount")]
    pub used_license_count: i32,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentTroubleshootingEvent {
    #[serde(rename = "managedDeviceIdentifier")]
    pub managed_device_identifier: String,
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,
    #[serde(rename = "osVersion")]
    pub os_version: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "enrollmentType")]
    pub enrollment_type: DeviceEnrollmentType,
    #[serde(rename = "failureCategory")]
    pub failure_category: DeviceEnrollmentFailureReason,
    #[serde(rename = "failureReason")]
    pub failure_reason: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityHistoryItem {
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "activeDurationSeconds")]
    pub active_duration_seconds: i32,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "lastActiveDateTime")]
    pub last_active_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "expirationDateTime")]
    pub expiration_date_time: String,
    #[serde(rename = "startedDateTime")]
    pub started_date_time: String,
    #[serde(rename = "userTimezone")]
    pub user_timezone: String,
    #[serde(rename = "activity")]
    pub activity: UserActivity,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Security {
    #[serde(rename = "alerts")]
    pub alerts: Vec<Alert>,
    #[serde(rename = "secureScoreControlProfiles")]
    pub secure_score_control_profiles: Vec<SecureScoreControlProfile>,
    #[serde(rename = "secureScores")]
    pub secure_scores: Vec<SecureScore>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(rename = "activityGroupName")]
    pub activity_group_name: String,
    #[serde(rename = "assignedTo")]
    pub assigned_to: String,
    #[serde(rename = "azureSubscriptionId")]
    pub azure_subscription_id: String,
    #[serde(rename = "azureTenantId")]
    pub azure_tenant_id: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "closedDateTime")]
    pub closed_date_time: String,
    #[serde(rename = "cloudAppStates")]
    pub cloud_app_states: Vec<CloudAppSecurityState>,
    #[serde(rename = "comments")]
    pub comments: Vec<String>,
    #[serde(rename = "confidence")]
    pub confidence: i32,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "detectionIds")]
    pub detection_ids: Vec<String>,
    #[serde(rename = "eventDateTime")]
    pub event_date_time: String,
    #[serde(rename = "feedback")]
    pub feedback: AlertFeedback,
    #[serde(rename = "fileStates")]
    pub file_states: Vec<FileSecurityState>,
    #[serde(rename = "historyStates")]
    pub history_states: Vec<AlertHistoryState>,
    #[serde(rename = "hostStates")]
    pub host_states: Vec<HostSecurityState>,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "malwareStates")]
    pub malware_states: Vec<MalwareState>,
    #[serde(rename = "networkConnections")]
    pub network_connections: Vec<NetworkConnection>,
    #[serde(rename = "processes")]
    pub processes: Vec<Process>,
    #[serde(rename = "recommendedActions")]
    pub recommended_actions: Vec<String>,
    #[serde(rename = "registryKeyStates")]
    pub registry_key_states: Vec<RegistryKeyState>,
    #[serde(rename = "severity")]
    pub severity: AlertSeverity,
    #[serde(rename = "sourceMaterials")]
    pub source_materials: Vec<String>,
    #[serde(rename = "status")]
    pub status: AlertStatus,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "triggers")]
    pub triggers: Vec<AlertTrigger>,
    #[serde(rename = "userStates")]
    pub user_states: Vec<UserSecurityState>,
    #[serde(rename = "vendorInformation")]
    pub vendor_information: SecurityVendorInformation,
    #[serde(rename = "vulnerabilityStates")]
    pub vulnerability_states: Vec<VulnerabilityState>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecureScoreControlProfile {
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(rename = "actionUrl")]
    pub action_url: String,
    #[serde(rename = "azureTenantId")]
    pub azure_tenant_id: String,
    #[serde(rename = "complianceInformation")]
    pub compliance_information: Vec<ComplianceInformation>,
    #[serde(rename = "controlCategory")]
    pub control_category: String,
    #[serde(rename = "controlStateUpdates")]
    pub control_state_updates: Vec<SecureScoreControlStateUpdate>,
    #[serde(rename = "deprecated")]
    pub deprecated: bool,
    #[serde(rename = "implementationCost")]
    pub implementation_cost: String,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "maxScore")]
    pub max_score: i64,
    #[serde(rename = "rank")]
    pub rank: i32,
    #[serde(rename = "remediation")]
    pub remediation: String,
    #[serde(rename = "remediationImpact")]
    pub remediation_impact: String,
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "threats")]
    pub threats: Vec<String>,
    #[serde(rename = "tier")]
    pub tier: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "userImpact")]
    pub user_impact: String,
    #[serde(rename = "vendorInformation")]
    pub vendor_information: SecurityVendorInformation,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecureScore {
    #[serde(rename = "activeUserCount")]
    pub active_user_count: i32,
    #[serde(rename = "averageComparativeScores")]
    pub average_comparative_scores: Vec<AverageComparativeScore>,
    #[serde(rename = "azureTenantId")]
    pub azure_tenant_id: String,
    #[serde(rename = "controlScores")]
    pub control_scores: Vec<ControlScore>,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "currentScore")]
    pub current_score: i64,
    #[serde(rename = "enabledServices")]
    pub enabled_services: Vec<String>,
    #[serde(rename = "licensedUserCount")]
    pub licensed_user_count: i32,
    #[serde(rename = "maxScore")]
    pub max_score: i64,
    #[serde(rename = "vendorInformation")]
    pub vendor_information: SecurityVendorInformation,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trending {
    #[serde(rename = "weight")]
    pub weight: i64,
    #[serde(rename = "resourceVisualization")]
    pub resource_visualization: ResourceVisualization,
    #[serde(rename = "resourceReference")]
    pub resource_reference: ResourceReference,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "resource")]
    pub resource: Entity,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedInsight {
    #[serde(rename = "lastShared")]
    pub last_shared: SharingDetail,
    #[serde(rename = "sharingHistory")]
    pub sharing_history: Vec<SharingDetail>,
    #[serde(rename = "resourceVisualization")]
    pub resource_visualization: ResourceVisualization,
    #[serde(rename = "resourceReference")]
    pub resource_reference: ResourceReference,
    #[serde(rename = "lastSharedMethod")]
    pub last_shared_method: Entity,
    #[serde(rename = "resource")]
    pub resource: Entity,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsedInsight {
    #[serde(rename = "lastUsed")]
    pub last_used: UsageDetails,
    #[serde(rename = "resourceVisualization")]
    pub resource_visualization: ResourceVisualization,
    #[serde(rename = "resourceReference")]
    pub resource_reference: ResourceReference,
    #[serde(rename = "resource")]
    pub resource: Entity,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppCatalogs {
    #[serde(rename = "teamsApps")]
    pub teams_apps: Vec<TeamsApp>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsApp {
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "distributionMethod")]
    pub distribution_method: TeamsAppDistributionMethod,
    #[serde(rename = "appDefinitions")]
    pub app_definitions: Vec<TeamsAppDefinition>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "webUrl")]
    pub web_url: String,
    #[serde(rename = "tabs")]
    pub tabs: Vec<TeamsTab>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsAppInstallation {
    #[serde(rename = "teamsApp")]
    pub teams_app: TeamsApp,
    #[serde(rename = "teamsAppDefinition")]
    pub teams_app_definition: TeamsAppDefinition,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsAsyncOperation {
    #[serde(rename = "operationType")]
    pub operation_type: TeamsAsyncOperationType,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "status")]
    pub status: TeamsAsyncOperationStatus,
    #[serde(rename = "lastActionDateTime")]
    pub last_action_date_time: String,
    #[serde(rename = "attemptsCount")]
    pub attempts_count: i32,
    #[serde(rename = "targetResourceId")]
    pub target_resource_id: String,
    #[serde(rename = "targetResourceLocation")]
    pub target_resource_location: String,
    #[serde(rename = "error")]
    pub error: OperationError,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsAppDefinition {
    #[serde(rename = "teamsAppId")]
    pub teams_app_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "version")]
    pub version: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsTab {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "webUrl")]
    pub web_url: String,
    #[serde(rename = "configuration")]
    pub configuration: TeamsTabConfiguration,
    #[serde(rename = "teamsApp")]
    pub teams_app: TeamsApp,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPolicyOperation {
    #[serde(rename = "completedDateTime")]
    pub completed_date_time: String,
    #[serde(rename = "status")]
    pub status: DataPolicyOperationStatus,
    #[serde(rename = "storageLocation")]
    pub storage_location: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "submittedDateTime")]
    pub submitted_date_time: String,
    #[serde(rename = "progress")]
    pub progress: i64,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProvider {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryAudit {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[serde(rename = "result")]
    pub result: OperationResult,
    #[serde(rename = "resultReason")]
    pub result_reason: String,
    #[serde(rename = "activityDisplayName")]
    pub activity_display_name: String,
    #[serde(rename = "activityDateTime")]
    pub activity_date_time: String,
    #[serde(rename = "loggedByService")]
    pub logged_by_service: String,
    #[serde(rename = "operationType")]
    pub operation_type: String,
    #[serde(rename = "initiatedBy")]
    pub initiated_by: AuditActivityInitiator,
    #[serde(rename = "targetResources")]
    pub target_resources: Vec<TargetResource>,
    #[serde(rename = "additionalDetails")]
    pub additional_details: Vec<KeyValue>,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignIn {
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "userDisplayName")]
    pub user_display_name: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "appDisplayName")]
    pub app_display_name: String,
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(rename = "status")]
    pub status: SignInStatus,
    #[serde(rename = "clientAppUsed")]
    pub client_app_used: String,
    #[serde(rename = "deviceDetail")]
    pub device_detail: DeviceDetail,
    #[serde(rename = "location")]
    pub location: SignInLocation,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[serde(rename = "conditionalAccessStatus")]
    pub conditional_access_status: ConditionalAccessStatus,
    #[serde(rename = "appliedConditionalAccessPolicies")]
    pub applied_conditional_access_policies: Vec<AppliedConditionalAccessPolicy>,
    #[serde(rename = "isInteractive")]
    pub is_interactive: bool,
    #[serde(rename = "riskDetail")]
    pub risk_detail: RiskDetail,
    #[serde(rename = "riskLevelAggregated")]
    pub risk_level_aggregated: RiskLevel,
    #[serde(rename = "riskLevelDuringSignIn")]
    pub risk_level_during_sign_in: RiskLevel,
    #[serde(rename = "riskState")]
    pub risk_state: RiskState,
    #[serde(rename = "riskEventTypes")]
    pub risk_event_types: Vec<RiskEventType>,
    #[serde(rename = "resourceDisplayName")]
    pub resource_display_name: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestrictedSignIn {
    #[serde(rename = "targetTenantId")]
    pub target_tenant_id: String,
}

#[cfg(not(feature = "option"))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogRoot {
    #[serde(rename = "signIns")]
    pub sign_ins: Vec<SignIn>,
    #[serde(rename = "directoryAudits")]
    pub directory_audits: Vec<DirectoryAudit>,
    #[serde(rename = "restrictedSignIns")]
    pub restricted_sign_ins: Vec<RestrictedSignIn>,
}
