#[cfg(feature = "option")]
use crate::complexop::*;
#[cfg(feature = "option")]
use crate::enumtypes::*;

#[cfg(feature = "option")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity(#[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>);

#[cfg(feature = "option")]
impl Entity {
    pub fn new(value: Option<serde_json::Value>) -> Entity {
        Entity(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DirectoryObject {
    #[serde(rename = "deletedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AdministrativeUnit(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl AdministrativeUnit {
    pub fn new(value: Option<serde_json::Value>) -> AdministrativeUnit {
        AdministrativeUnit(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Directory {
    #[serde(rename = "deletedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_items: Option<Vec<DirectoryObject>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "accountEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[serde(rename = "alternativeSecurityIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_security_ids: Option<Vec<AlternativeSecurityId>>,
    #[serde(rename = "approximateLastSignInDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_last_sign_in_date_time: Option<String>,
    #[serde(rename = "complianceExpirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_expiration_date_time: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "deviceMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_metadata: Option<String>,
    #[serde(rename = "deviceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_version: Option<i32>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "isCompliant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[serde(rename = "isManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
    #[serde(rename = "onPremisesLastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_last_sync_date_time: Option<String>,
    #[serde(rename = "onPremisesSyncEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_sync_enabled: Option<bool>,
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "operatingSystemVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    #[serde(rename = "physicalIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_ids: Option<Vec<String>>,
    #[serde(rename = "profileType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_type: Option<String>,
    #[serde(rename = "systemLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_labels: Option<Vec<String>>,
    #[serde(rename = "trustType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<String>,
    #[serde(rename = "memberOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of: Option<Vec<DirectoryObject>>,
    #[serde(rename = "registeredOwners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_owners: Option<Vec<DirectoryObject>>,
    #[serde(rename = "registeredUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_users: Option<Vec<DirectoryObject>>,
    #[serde(rename = "transitiveMemberOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitive_member_of: Option<Vec<DirectoryObject>>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DirectoryObjectPartnerReference {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "externalPartnerTenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_partner_tenant_id: Option<String>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DirectoryRole {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "roleTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_template_id: Option<String>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<DirectoryObject>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DirectoryRoleTemplate {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(rename = "authenticationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "availabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
    #[serde(rename = "isAdminManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin_managed: Option<bool>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "isInitial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_initial: Option<bool>,
    #[serde(rename = "isRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    #[serde(rename = "isVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(rename = "passwordNotificationWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_notification_window_in_days: Option<i32>,
    #[serde(rename = "passwordValidityPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_validity_period_in_days: Option<i32>,
    #[serde(rename = "supportedServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_services: Option<Vec<String>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<DomainState>,
    #[serde(rename = "serviceConfigurationRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_configuration_records: Option<Vec<DomainDnsRecord>>,
    #[serde(rename = "verificationDnsRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_dns_records: Option<Vec<DomainDnsRecord>>,
    #[serde(rename = "domainNameReferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_references: Option<Vec<DirectoryObject>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsRecord {
    #[serde(rename = "isOptional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "recordType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    #[serde(rename = "supportedService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_service: Option<String>,
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsCnameRecord {
    #[serde(rename = "canonicalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsMxRecord {
    #[serde(rename = "mailExchange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_exchange: Option<String>,
    #[serde(rename = "preference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsSrvRecord {
    #[serde(rename = "nameTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_target: Option<String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsTxtRecord {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainDnsUnavailableRecord {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LicenseDetails {
    #[serde(rename = "servicePlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_plans: Option<Vec<ServicePlanInfo>>,
    #[serde(rename = "skuId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "skuPartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_part_number: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "assignedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_licenses: Option<Vec<AssignedLicense>>,
    #[serde(rename = "classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "hasMembersWithLicenseErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_members_with_license_errors: Option<bool>,
    #[serde(rename = "groupTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_types: Option<Vec<String>>,
    #[serde(rename = "licenseProcessingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_processing_state: Option<LicenseProcessingState>,
    #[serde(rename = "mail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[serde(rename = "mailEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_enabled: Option<bool>,
    #[serde(rename = "mailNickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[serde(rename = "onPremisesLastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_last_sync_date_time: Option<String>,
    #[serde(rename = "onPremisesProvisioningErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_provisioning_errors: Option<Vec<OnPremisesProvisioningError>>,
    #[serde(rename = "onPremisesSecurityIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier: Option<String>,
    #[serde(rename = "onPremisesSyncEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_sync_enabled: Option<bool>,
    #[serde(rename = "preferredDataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_data_location: Option<String>,
    #[serde(rename = "proxyAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_addresses: Option<Vec<String>>,
    #[serde(rename = "renewedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewed_date_time: Option<String>,
    #[serde(rename = "securityEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_enabled: Option<bool>,
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "allowExternalSenders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_senders: Option<bool>,
    #[serde(rename = "autoSubscribeNewMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_subscribe_new_members: Option<bool>,
    #[serde(rename = "isSubscribedByMail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribed_by_mail: Option<bool>,
    #[serde(rename = "unseenCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unseen_count: Option<i32>,
    #[serde(rename = "isArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<DirectoryObject>>,
    #[serde(rename = "memberOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of: Option<Vec<DirectoryObject>>,
    #[serde(rename = "membersWithLicenseErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_with_license_errors: Option<Vec<DirectoryObject>>,
    #[serde(rename = "transitiveMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitive_members: Option<Vec<DirectoryObject>>,
    #[serde(rename = "transitiveMemberOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitive_member_of: Option<Vec<DirectoryObject>>,
    #[serde(rename = "createdOnBehalfOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on_behalf_of: Option<DirectoryObject>,
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<DirectoryObject>>,
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<GroupSetting>>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<ConversationThread>>,
    #[serde(rename = "calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar: Option<Calendar>,
    #[serde(rename = "calendarView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_view: Option<Vec<Event>>,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "conversations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<Vec<Conversation>>,
    #[serde(rename = "photo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ProfilePhoto>,
    #[serde(rename = "photos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<ProfilePhoto>>,
    #[serde(rename = "acceptedSenders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_senders: Option<Vec<DirectoryObject>>,
    #[serde(rename = "rejectedSenders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_senders: Option<Vec<DirectoryObject>>,
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive: Option<Drive>,
    #[serde(rename = "drives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<Drive>>,
    #[serde(rename = "sites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<Site>>,
    #[serde(rename = "planner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planner: Option<PlannerGroup>,
    #[serde(rename = "onenote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onenote: Option<Onenote>,
    #[serde(rename = "groupLifecyclePolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_lifecycle_policies: Option<Vec<GroupLifecyclePolicy>>,
    #[serde(rename = "team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupSetting {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<SettingValue>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConversationThread {
    #[serde(rename = "toRecipients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_recipients: Option<Vec<Recipient>>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "hasAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(rename = "lastDeliveredDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivered_date_time: Option<String>,
    #[serde(rename = "uniqueSenders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_senders: Option<Vec<String>>,
    #[serde(rename = "ccRecipients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_recipients: Option<Vec<Recipient>>,
    #[serde(rename = "preview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(rename = "isLocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "posts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<Post>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Calendar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<CalendarColor>,
    #[serde(rename = "changeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_key: Option<String>,
    #[serde(rename = "canShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    #[serde(rename = "canViewPrivateItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_private_items: Option<bool>,
    #[serde(rename = "canEdit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<EmailAddress>,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "calendarView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_view: Option<Vec<Event>>,
    #[serde(rename = "singleValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_value_extended_properties: Option<String>,
    #[serde(rename = "multiValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_extended_properties: Option<Vec<MultiValueLegacyExtendedProperty>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutlookItem {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "changeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_key: Option<String>,
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "changeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_key: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "originalStartTimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_start_time_zone: Option<String>,
    #[serde(rename = "originalEndTimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_end_time_zone: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "responseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<ResponseStatus>,
    #[serde(rename = "iCalUId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cal_u_id: Option<String>,
    #[serde(rename = "reminderMinutesBeforeStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_minutes_before_start: Option<i32>,
    #[serde(rename = "isReminderOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reminder_on: Option<bool>,
    #[serde(rename = "hasAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ItemBody>,
    #[serde(rename = "bodyPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_preview: Option<String>,
    #[serde(rename = "importance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<Importance>,
    #[serde(rename = "sensitivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<Sensitivity>,
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "originalStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_start: Option<String>,
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
    #[serde(rename = "isAllDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
    #[serde(rename = "isCancelled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cancelled: Option<bool>,
    #[serde(rename = "isOrganizer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organizer: Option<bool>,
    #[serde(rename = "recurrence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<PatternedRecurrence>,
    #[serde(rename = "responseRequested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_requested: Option<bool>,
    #[serde(rename = "seriesMasterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_master_id: Option<String>,
    #[serde(rename = "showAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_as: Option<FreeBusyStatus>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<EventType>,
    #[serde(rename = "attendees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    #[serde(rename = "organizer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<Recipient>,
    #[serde(rename = "webLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_link: Option<String>,
    #[serde(rename = "onlineMeetingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_meeting_url: Option<String>,
    #[serde(rename = "calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar: Option<Calendar>,
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Event>>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "singleValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_value_extended_properties: Option<String>,
    #[serde(rename = "multiValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_extended_properties: Option<Vec<MultiValueLegacyExtendedProperty>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Conversation {
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "hasAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(rename = "lastDeliveredDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivered_date_time: Option<String>,
    #[serde(rename = "uniqueSenders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_senders: Option<Vec<String>>,
    #[serde(rename = "preview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<ConversationThread>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProfilePhoto {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BaseItem {
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ItemReference>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(rename = "createdByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<User>,
    #[serde(rename = "lastModifiedByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_user: Option<User>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "driveType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_type: Option<String>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentitySet>,
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<Quota>,
    #[serde(rename = "sharePointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_point_ids: Option<SharepointIds>,
    #[serde(rename = "system")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemFacet>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DriveItem>>,
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Box<List>>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Box<DriveItem>>,
    #[serde(rename = "special")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<Vec<DriveItem>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Site {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharepoint_ids: Option<SharepointIds>,
    #[serde(rename = "siteCollection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_collection: Option<SiteCollection>,
    #[serde(rename = "analytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<ItemAnalytics>,
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<ColumnDefinition>>,
    #[serde(rename = "contentTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<ContentType>>,
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive: Option<Drive>,
    #[serde(rename = "drives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<Drive>>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BaseItem>>,
    #[serde(rename = "lists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<List>>,
    #[serde(rename = "sites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<Site>>,
    #[serde(rename = "onenote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onenote: Option<Onenote>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerGroup {
    #[serde(rename = "plans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Vec<PlannerPlan>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Onenote {
    #[serde(rename = "notebooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebooks: Option<Vec<Notebook>>,
    #[serde(rename = "sections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<OnenoteSection>>,
    #[serde(rename = "sectionGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_groups: Option<Vec<SectionGroup>>,
    #[serde(rename = "pages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<OnenotePage>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<OnenoteResource>>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<OnenoteOperation>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupLifecyclePolicy {
    #[serde(rename = "groupLifetimeInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_lifetime_in_days: Option<i32>,
    #[serde(rename = "managedGroupTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_group_types: Option<String>,
    #[serde(rename = "alternateNotificationEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_notification_emails: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(rename = "memberSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_settings: Option<TeamMemberSettings>,
    #[serde(rename = "guestSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_settings: Option<TeamGuestSettings>,
    #[serde(rename = "messagingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messaging_settings: Option<TeamMessagingSettings>,
    #[serde(rename = "funSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fun_settings: Option<TeamFunSettings>,
    #[serde(rename = "isArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
    #[serde(rename = "installedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_apps: Option<Vec<TeamsAppInstallation>>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<TeamsAsyncOperation>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    #[serde(rename = "contractType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "defaultDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_domain_name: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SubscribedSku {
    #[serde(rename = "capabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_status: Option<String>,
    #[serde(rename = "consumedUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_units: Option<i32>,
    #[serde(rename = "prepaidUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_units: Option<LicenseUnitsDetail>,
    #[serde(rename = "servicePlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_plans: Option<Vec<ServicePlanInfo>>,
    #[serde(rename = "skuId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "skuPartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_part_number: Option<String>,
    #[serde(rename = "appliesTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "assignedPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_plans: Option<Vec<AssignedPlan>>,
    #[serde(rename = "businessPhones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_phones: Option<Vec<String>>,
    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "countryLetterCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_letter_code: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "marketingNotificationEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_notification_emails: Option<Vec<String>>,
    #[serde(rename = "onPremisesLastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_last_sync_date_time: Option<String>,
    #[serde(rename = "onPremisesSyncEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_sync_enabled: Option<bool>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "preferredLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "privacyProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_profile: Option<PrivacyProfile>,
    #[serde(rename = "provisionedPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_plans: Option<Vec<ProvisionedPlan>>,
    #[serde(rename = "securityComplianceNotificationMails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_compliance_notification_mails: Option<Vec<String>>,
    #[serde(rename = "securityComplianceNotificationPhones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_compliance_notification_phones: Option<Vec<String>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "street")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "technicalNotificationMails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_notification_mails: Option<Vec<String>>,
    #[serde(rename = "verifiedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_domains: Option<Vec<VerifiedDomain>>,
    #[serde(rename = "mobileDeviceManagementAuthority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_device_management_authority: Option<MdmAuthority>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "accountEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[serde(rename = "ageGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_group: Option<String>,
    #[serde(rename = "assignedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_licenses: Option<Vec<AssignedLicense>>,
    #[serde(rename = "assignedPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_plans: Option<Vec<AssignedPlan>>,
    #[serde(rename = "businessPhones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_phones: Option<Vec<String>>,
    #[serde(rename = "city")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "consentProvidedForMinor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_provided_for_minor: Option<String>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "department")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(rename = "faxNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_number: Option<String>,
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(rename = "imAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub im_addresses: Option<Vec<String>>,
    #[serde(rename = "isResourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resource_account: Option<bool>,
    #[serde(rename = "jobTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(rename = "legalAgeGroupClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_age_group_classification: Option<String>,
    #[serde(rename = "licenseAssignmentStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_assignment_states: Option<Vec<LicenseAssignmentState>>,
    #[serde(rename = "mail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[serde(rename = "mailNickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[serde(rename = "mobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "onPremisesDistinguishedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_distinguished_name: Option<String>,
    #[serde(rename = "onPremisesExtensionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_extension_attributes: Option<OnPremisesExtensionAttributes>,
    #[serde(rename = "onPremisesImmutableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_immutable_id: Option<String>,
    #[serde(rename = "onPremisesLastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_last_sync_date_time: Option<String>,
    #[serde(rename = "onPremisesProvisioningErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_provisioning_errors: Option<Vec<OnPremisesProvisioningError>>,
    #[serde(rename = "onPremisesSecurityIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier: Option<String>,
    #[serde(rename = "onPremisesSyncEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_sync_enabled: Option<bool>,
    #[serde(rename = "onPremisesDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_domain_name: Option<String>,
    #[serde(rename = "onPremisesSamAccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_sam_account_name: Option<String>,
    #[serde(rename = "onPremisesUserPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_user_principal_name: Option<String>,
    #[serde(rename = "otherMails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_mails: Option<Vec<String>>,
    #[serde(rename = "passwordPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policies: Option<String>,
    #[serde(rename = "passwordProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_profile: Option<PasswordProfile>,
    #[serde(rename = "officeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office_location: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "preferredLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "provisionedPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_plans: Option<Vec<ProvisionedPlan>>,
    #[serde(rename = "proxyAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_addresses: Option<Vec<String>>,
    #[serde(rename = "showInAddressList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_address_list: Option<bool>,
    #[serde(rename = "signInSessionsValidFromDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_sessions_valid_from_date_time: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "streetAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(rename = "surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(rename = "usageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_location: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "userType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(rename = "mailboxSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_settings: Option<MailboxSettings>,
    #[serde(rename = "aboutMe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about_me: Option<String>,
    #[serde(rename = "birthday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(rename = "hireDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    #[serde(rename = "interests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<String>>,
    #[serde(rename = "mySite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_site: Option<String>,
    #[serde(rename = "pastProjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_projects: Option<Vec<String>>,
    #[serde(rename = "preferredName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    #[serde(rename = "responsibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibilities: Option<Vec<String>>,
    #[serde(rename = "schools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schools: Option<Vec<String>>,
    #[serde(rename = "skills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    #[serde(rename = "deviceEnrollmentLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_enrollment_limit: Option<i32>,
    #[serde(rename = "ownedDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_devices: Option<Vec<DirectoryObject>>,
    #[serde(rename = "registeredDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_devices: Option<Vec<DirectoryObject>>,
    #[serde(rename = "manager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<DirectoryObject>,
    #[serde(rename = "directReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_reports: Option<Vec<DirectoryObject>>,
    #[serde(rename = "memberOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of: Option<Vec<DirectoryObject>>,
    #[serde(rename = "createdObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_objects: Option<Vec<DirectoryObject>>,
    #[serde(rename = "ownedObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_objects: Option<Vec<DirectoryObject>>,
    #[serde(rename = "licenseDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_details: Option<Vec<LicenseDetails>>,
    #[serde(rename = "transitiveMemberOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitive_member_of: Option<Vec<DirectoryObject>>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
    #[serde(rename = "outlook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlook: Option<OutlookUser>,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    #[serde(rename = "mailFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_folders: Option<Vec<MailFolder>>,
    #[serde(rename = "calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar: Option<Calendar>,
    #[serde(rename = "calendars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendars: Option<Vec<Calendar>>,
    #[serde(rename = "calendarGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_groups: Option<Vec<CalendarGroup>>,
    #[serde(rename = "calendarView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_view: Option<Vec<Event>>,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "people")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<Person>>,
    #[serde(rename = "contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<Contact>>,
    #[serde(rename = "contactFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_folders: Option<Vec<ContactFolder>>,
    #[serde(rename = "inferenceClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_classification: Option<InferenceClassification>,
    #[serde(rename = "photo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ProfilePhoto>,
    #[serde(rename = "photos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<ProfilePhoto>>,
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive: Option<Box<Drive>>,
    #[serde(rename = "drives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drives: Option<Vec<Drive>>,
    #[serde(rename = "planner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planner: Option<PlannerUser>,
    #[serde(rename = "onenote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onenote: Option<Onenote>,
    #[serde(rename = "managedDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_devices: Option<Vec<ManagedDevice>>,
    #[serde(rename = "managedAppRegistrations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_app_registrations: Option<Vec<ManagedAppRegistration>>,
    #[serde(rename = "deviceManagementTroubleshootingEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_management_troubleshooting_events: Option<Vec<DeviceManagementTroubleshootingEvent>>,
    #[serde(rename = "activities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<UserActivity>>,
    #[serde(rename = "insights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<OfficeGraphInsights>,
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UserSettings>,
    #[serde(rename = "joinedTeams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_teams: Option<Vec<Group>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutlookUser {
    #[serde(rename = "masterCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_categories: Option<Vec<OutlookCategory>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@odata.etag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odata_etag: Option<String>,
    #[serde(rename = "receivedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_date_time: Option<String>,
    #[serde(rename = "sentDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_date_time: Option<String>,
    #[serde(rename = "hasAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(rename = "internetMessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_message_id: Option<String>,
    #[serde(rename = "internetMessageHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_message_headers: Option<Vec<InternetMessageHeader>>,
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ItemBody>,
    #[serde(rename = "bodyPreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "importance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<Importance>,
    #[serde(rename = "parentFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    #[serde(rename = "sender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Recipient>,
    #[serde(rename = "from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Recipient>,
    #[serde(rename = "toRecipients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_recipients: Option<Vec<Recipient>>,
    #[serde(rename = "changeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_key: Option<String>,
    #[serde(rename = "ccRecipients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_recipients: Option<Vec<Recipient>>,
    #[serde(rename = "bccRecipients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc_recipients: Option<Vec<Recipient>>,
    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Vec<Recipient>>,
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(rename = "uniqueBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_body: Option<ItemBody>,
    #[serde(rename = "isDeliveryReceiptRequested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_delivery_receipt_requested: Option<bool>,
    #[serde(rename = "isReadReceiptRequested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_receipt_requested: Option<bool>,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "isRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read: Option<bool>,
    #[serde(rename = "isDraft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    #[serde(rename = "webLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_link: Option<String>,
    #[serde(rename = "inferenceClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_classification: Option<InferenceClassificationType>,
    #[serde(rename = "flag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<FollowupFlag>,
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
    #[serde(rename = "singleValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_value_extended_properties: Option<String>,
    #[serde(rename = "multiValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_extended_properties: Option<Vec<MultiValueLegacyExtendedProperty>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MailFolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "parentFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    #[serde(rename = "childFolderCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_folder_count: Option<i32>,
    #[serde(rename = "unreadItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_item_count: Option<i32>,
    #[serde(rename = "totalItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_item_count: Option<i32>,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    #[serde(rename = "messageRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_rules: Option<Vec<MessageRule>>,
    #[serde(rename = "childFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_folders: Option<Vec<MailFolder>>,
    #[serde(rename = "singleValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_value_extended_properties: Option<String>,
    #[serde(rename = "multiValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_extended_properties: Option<Vec<MultiValueLegacyExtendedProperty>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CalendarGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "classId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<String>,
    #[serde(rename = "changeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_key: Option<String>,
    #[serde(rename = "calendars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendars: Option<Vec<Calendar>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(rename = "surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(rename = "birthday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(rename = "personNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_notes: Option<String>,
    #[serde(rename = "isFavorite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "scoredEmailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scored_email_addresses: Option<Vec<ScoredEmailAddress>>,
    #[serde(rename = "phones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<Phone>>,
    #[serde(rename = "postalAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_addresses: Option<Vec<Location>>,
    #[serde(rename = "websites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websites: Option<Vec<Website>>,
    #[serde(rename = "jobTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "yomiCompany")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yomi_company: Option<String>,
    #[serde(rename = "department")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "officeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office_location: Option<String>,
    #[serde(rename = "profession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profession: Option<String>,
    #[serde(rename = "personType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_type: Option<PersonType>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "imAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub im_address: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "parentFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    #[serde(rename = "birthday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(rename = "fileAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_as: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(rename = "initials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "nickName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(rename = "surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "yomiGivenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yomi_given_name: Option<String>,
    #[serde(rename = "yomiSurname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yomi_surname: Option<String>,
    #[serde(rename = "yomiCompanyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yomi_company_name: Option<String>,
    #[serde(rename = "generation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[serde(rename = "emailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<EmailAddress>>,
    #[serde(rename = "imAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub im_addresses: Option<Vec<String>>,
    #[serde(rename = "jobTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "department")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "officeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office_location: Option<String>,
    #[serde(rename = "profession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profession: Option<String>,
    #[serde(rename = "businessHomePage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_home_page: Option<String>,
    #[serde(rename = "assistantName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_name: Option<String>,
    #[serde(rename = "manager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[serde(rename = "homePhones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_phones: Option<Vec<String>>,
    #[serde(rename = "mobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "businessPhones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_phones: Option<Vec<String>>,
    #[serde(rename = "homeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_address: Option<PhysicalAddress>,
    #[serde(rename = "businessAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_address: Option<PhysicalAddress>,
    #[serde(rename = "otherAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_address: Option<PhysicalAddress>,
    #[serde(rename = "spouseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spouse_name: Option<String>,
    #[serde(rename = "personalNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_notes: Option<String>,
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
    #[serde(rename = "singleValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_value_extended_properties: Option<String>,
    #[serde(rename = "multiValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_extended_properties: Option<Vec<MultiValueLegacyExtendedProperty>>,
    #[serde(rename = "photo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ProfilePhoto>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContactFolder {
    #[serde(rename = "parentFolderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<Contact>>,
    #[serde(rename = "childFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_folders: Option<Vec<ContactFolder>>,
    #[serde(rename = "singleValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_value_extended_properties: Option<String>,
    #[serde(rename = "multiValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_extended_properties: Option<Vec<MultiValueLegacyExtendedProperty>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InferenceClassification {
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<InferenceClassificationOverride>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerUser {
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<PlannerTask>>,
    #[serde(rename = "plans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Vec<PlannerPlan>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedDevice {
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "managedDeviceOwnerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_owner_type: Option<ManagedDeviceOwnerType>,
    #[serde(rename = "deviceActionResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_action_results: Option<Vec<DeviceActionResult>>,
    #[serde(rename = "enrolledDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrolled_date_time: Option<String>,
    #[serde(rename = "lastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_date_time: Option<String>,
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "complianceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<ComplianceState>,
    #[serde(rename = "jailBroken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jail_broken: Option<String>,
    #[serde(rename = "managementAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_agent: Option<ManagementAgentType>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "easActivated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eas_activated: Option<bool>,
    #[serde(rename = "easDeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eas_device_id: Option<String>,
    #[serde(rename = "easActivationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eas_activation_date_time: Option<String>,
    #[serde(rename = "azureADRegistered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_a_d_registered: Option<bool>,
    #[serde(rename = "deviceEnrollmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_enrollment_type: Option<DeviceEnrollmentType>,
    #[serde(rename = "activationLockBypassCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_lock_bypass_code: Option<String>,
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "azureADDeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_a_d_device_id: Option<String>,
    #[serde(rename = "deviceRegistrationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_registration_state: Option<DeviceRegistrationState>,
    #[serde(rename = "deviceCategoryDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_category_display_name: Option<String>,
    #[serde(rename = "isSupervised")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supervised: Option<bool>,
    #[serde(rename = "exchangeLastSuccessfulSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_last_successful_sync_date_time: Option<String>,
    #[serde(rename = "exchangeAccessState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_access_state: Option<DeviceManagementExchangeAccessState>,
    #[serde(rename = "exchangeAccessStateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_access_state_reason: Option<DeviceManagementExchangeAccessStateReason>,
    #[serde(rename = "remoteAssistanceSessionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_assistance_session_url: Option<String>,
    #[serde(rename = "remoteAssistanceSessionErrorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_assistance_session_error_details: Option<String>,
    #[serde(rename = "isEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "imei")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_grace_period_expiration_date_time: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "androidSecurityPatchLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_security_patch_level: Option<String>,
    #[serde(rename = "userDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "configurationManagerClientEnabledFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_manager_client_enabled_features:
        Option<ConfigurationManagerClientEnabledFeatures>,
    #[serde(rename = "wiFiMacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi_mac_address: Option<String>,
    #[serde(rename = "deviceHealthAttestationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_health_attestation_state: Option<DeviceHealthAttestationState>,
    #[serde(rename = "subscriberCarrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_carrier: Option<String>,
    #[serde(rename = "meid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meid: Option<String>,
    #[serde(rename = "totalStorageSpaceInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_storage_space_in_bytes: Option<i64>,
    #[serde(rename = "freeStorageSpaceInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_storage_space_in_bytes: Option<i64>,
    #[serde(rename = "managedDeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_name: Option<String>,
    #[serde(rename = "partnerReportedThreatState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_reported_threat_state: Option<ManagedDevicePartnerReportedHealthState>,
    #[serde(rename = "deviceConfigurationStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration_states: Option<Vec<DeviceConfigurationState>>,
    #[serde(rename = "deviceCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_category: Option<DeviceCategory>,
    #[serde(rename = "deviceCompliancePolicyStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_compliance_policy_states: Option<Vec<DeviceCompliancePolicyState>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppRegistration {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_date_time: Option<String>,
    #[serde(rename = "applicationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[serde(rename = "managementSdkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_sdk_version: Option<String>,
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "deviceTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_tag: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "flaggedReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flagged_reasons: Option<Vec<ManagedAppFlaggedReason>>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "appIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_identifier: Option<MobileAppIdentifier>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "appliedPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_policies: Option<Vec<ManagedAppPolicy>>,
    #[serde(rename = "intendedPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_policies: Option<Vec<ManagedAppPolicy>>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<ManagedAppOperation>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagementTroubleshootingEvent {
    #[serde(rename = "eventDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date_time: Option<String>,
    #[serde(rename = "correlationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserActivity {
    #[serde(rename = "visualElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_elements: Option<VisualInfo>,
    #[serde(rename = "activitySourceHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_source_host: Option<String>,
    #[serde(rename = "activationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_url: Option<String>,
    #[serde(rename = "appActivityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_activity_id: Option<String>,
    #[serde(rename = "appDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_display_name: Option<String>,
    #[serde(rename = "contentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "fallbackUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_url: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "userTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_timezone: Option<String>,
    #[serde(rename = "contentInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_info: Option<Json>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "historyItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_items: Option<Vec<ActivityHistoryItem>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OfficeGraphInsights {
    #[serde(rename = "trending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trending: Option<Vec<Trending>>,
    #[serde(rename = "shared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<Vec<SharedInsight>>,
    #[serde(rename = "used")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<Vec<UsedInsight>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    #[serde(rename = "contributionToContentDiscoveryDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_to_content_discovery_disabled: Option<bool>,
    #[serde(rename = "contributionToContentDiscoveryAsOrganizationDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_to_content_discovery_as_organization_disabled: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupSettingTemplate {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<SettingTemplateValue>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SchemaExtension {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "targetTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_types: Option<Vec<String>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ExtensionSchemaProperty>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "isInline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inline: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutlookCategory {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<CategoryColor>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "sequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<MessageRulePredicates>,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<MessageRuleActions>,
    #[serde(rename = "exceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<MessageRulePredicates>,
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "hasError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(rename = "isReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SingleValueLegacyExtendedProperty {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MultiValueLegacyExtendedProperty {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MailSearchFolder {
    #[serde(rename = "isSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supported: Option<bool>,
    #[serde(rename = "includeNestedFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_nested_folders: Option<bool>,
    #[serde(rename = "sourceFolderIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_folder_ids: Option<Vec<String>>,
    #[serde(rename = "filterQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_query: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileAttachment {
    #[serde(rename = "contentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "contentLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_location: Option<String>,
    #[serde(rename = "contentBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_bytes: Option<Vec<u8>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemAttachment {
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<OutlookItem>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    #[serde(rename = "meetingMessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_message_type: Option<MeetingMessageType>,
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReferenceAttachment(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl ReferenceAttachment {
    pub fn new(value: Option<serde_json::Value>) -> ReferenceAttachment {
        ReferenceAttachment(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpenTypeExtension {
    #[serde(rename = "extensionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ItemBody>,
    #[serde(rename = "receivedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_date_time: Option<String>,
    #[serde(rename = "hasAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(rename = "from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Recipient>,
    #[serde(rename = "sender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Recipient>,
    #[serde(rename = "conversationThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_thread_id: Option<String>,
    #[serde(rename = "newParticipants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_participants: Option<Vec<Recipient>>,
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,
    #[serde(rename = "inReplyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<Box<Post>>,
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "singleValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_value_extended_properties: Option<String>,
    #[serde(rename = "multiValueExtendedProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_extended_properties: Option<Vec<MultiValueLegacyExtendedProperty>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InferenceClassificationOverride {
    #[serde(rename = "classifyAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classify_as: Option<InferenceClassificationType>,
    #[serde(rename = "senderEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_email_address: Option<EmailAddress>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BaseItemVersion {
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "publication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication: Option<PublicationFacet>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ColumnDefinition {
    #[serde(rename = "boolean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<BooleanColumn>,
    #[serde(rename = "calculated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated: Option<CalculatedColumn>,
    #[serde(rename = "choice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice: Option<ChoiceColumn>,
    #[serde(rename = "columnGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_group: Option<String>,
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyColumn>,
    #[serde(rename = "dateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<DefaultColumnValue>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "enforceUniqueValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_unique_values: Option<bool>,
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "indexed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    #[serde(rename = "lookup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup: Option<LookupColumn>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberColumn>,
    #[serde(rename = "personOrGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_or_group: Option<PersonOrGroupColumn>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextColumn>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ColumnLink {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContentType {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "inheritedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<ItemReference>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ContentTypeOrder>,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "sealed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed: Option<bool>,
    #[serde(rename = "columnLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_links: Option<Vec<ColumnLink>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DriveItem {
    #[serde(rename = "@odata.context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_context: Option<String>,
    #[serde(rename = "@odata.type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_type: Option<String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ItemReference>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(rename = "createdByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<Box<User>>,
    #[serde(rename = "lastModifiedByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_user: Option<Box<User>>,
    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<u8>>,
    #[serde(rename = "cTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_tag: Option<String>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Deleted>,
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_info: Option<FileSystemInfo>,
    #[serde(rename = "folder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<Folder>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeoCoordinates>,
    #[serde(rename = "package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<Package>,
    #[serde(rename = "photo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Photo>,
    #[serde(rename = "publication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication: Option<PublicationFacet>,
    #[serde(rename = "remoteItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_item: Option<RemoteItem>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
    #[serde(rename = "searchResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_result: Option<SearchResult>,
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
    #[serde(rename = "video")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "webDavUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_dav_url: Option<String>,
    #[serde(rename = "analytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<ItemAnalytics>,
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DriveItem>>,
    #[serde(rename = "listItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_item: Option<ListItem>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
    #[serde(rename = "subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
    #[serde(rename = "thumbnails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnails: Option<Vec<ThumbnailSet>>,
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<DriveItemVersion>>,
    #[serde(rename = "workbook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workbook: Option<Workbook>,
    #[serde(rename = "@microsoft.graph.conflictBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_behavior: Option<String>,
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "@microsoft.graph.sourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct List {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<ListInfo>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharepoint_ids: Option<SharepointIds>,
    #[serde(rename = "system")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemFacet>,
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<ColumnDefinition>>,
    #[serde(rename = "contentTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<ContentType>>,
    #[serde(rename = "drive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive: Option<Drive>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ListItem>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemAnalytics {
    #[serde(rename = "itemActivityStats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_activity_stats: Option<Vec<ItemActivityStat>>,
    #[serde(rename = "allTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_time: Option<ItemActivityStat>,
    #[serde(rename = "lastSevenDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seven_days: Option<ItemActivityStat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListItem {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentTypeInfo>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharepoint_ids: Option<SharepointIds>,
    #[serde(rename = "analytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<ItemAnalytics>,
    #[serde(rename = "driveItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_item: Option<Box<DriveItem>>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<FieldValueSet>,
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<ListItemVersion>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "grantedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted_to: Option<IdentitySet>,
    #[serde(rename = "inheritedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<ItemReference>,
    #[serde(rename = "invitation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation: Option<SharingInvitation>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SharingLink>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(rename = "shareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "changeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(rename = "clientState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_state: Option<String>,
    #[serde(rename = "notificationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "applicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "creatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ThumbnailSet {
    #[serde(rename = "large")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large: Option<Thumbnail>,
    #[serde(rename = "medium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<Thumbnail>,
    #[serde(rename = "small")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small: Option<Thumbnail>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Thumbnail>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DriveItemVersion {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<u8>>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Workbook {
    #[serde(rename = "application")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<WorkbookApplication>,
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<WorkbookNamedItem>>,
    #[serde(rename = "tables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<WorkbookTable>>,
    #[serde(rename = "worksheets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worksheets: Option<Vec<WorkbookWorksheet>>,
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<WorkbookComment>>,
    #[serde(rename = "functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<WorkbookFunctions>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldValueSet(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>
);

#[cfg(feature = "option")]
impl FieldValueSet {
    pub fn new(value: Option<serde_json::Value>) -> FieldValueSet {
        FieldValueSet(value)
    }

    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemActivity {
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessAction>,
    #[serde(rename = "activityDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_date_time: Option<String>,
    #[serde(rename = "actor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<IdentitySet>,
    #[serde(rename = "driveItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_item: Option<DriveItem>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemActivityStat {
    #[serde(rename = "startDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "endDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<ItemActionStat>,
    #[serde(rename = "create")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<ItemActionStat>,
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<ItemActionStat>,
    #[serde(rename = "edit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<ItemActionStat>,
    #[serde(rename = "move")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _move: Option<ItemActionStat>,
    #[serde(rename = "isTrending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trending: Option<bool>,
    #[serde(rename = "incompleteData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_data: Option<IncompleteData>,
    #[serde(rename = "activities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<ItemActivity>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListItemVersion {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<FieldValueSet>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SharedDriveItem {
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<IdentitySet>,
    #[serde(rename = "driveItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_item: Option<DriveItem>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DriveItem>>,
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<List>,
    #[serde(rename = "listItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_item: Option<ListItem>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<DriveItem>,
    #[serde(rename = "site")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookApplication {
    #[serde(rename = "calculationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_mode: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookNamedItem {
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Json>,
    #[serde(rename = "visible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename = "worksheet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worksheet: Option<WorkbookWorksheet>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTable {
    #[serde(rename = "highlightFirstColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_first_column: Option<bool>,
    #[serde(rename = "highlightLastColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_last_column: Option<bool>,
    #[serde(rename = "legacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_id: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "showBandedColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_banded_columns: Option<bool>,
    #[serde(rename = "showBandedRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_banded_rows: Option<bool>,
    #[serde(rename = "showFilterButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_filter_button: Option<bool>,
    #[serde(rename = "showHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_headers: Option<bool>,
    #[serde(rename = "showTotals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_totals: Option<bool>,
    #[serde(rename = "style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<WorkbookTableColumn>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<WorkbookTableRow>>,
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<WorkbookTableSort>,
    #[serde(rename = "worksheet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worksheet: Option<WorkbookWorksheet>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookWorksheet {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "charts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charts: Option<Vec<WorkbookChart>>,
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<WorkbookNamedItem>>,
    #[serde(rename = "pivotTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_tables: Option<Vec<WorkbookPivotTable>>,
    #[serde(rename = "protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<WorkbookWorksheetProtection>,
    #[serde(rename = "tables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<WorkbookTable>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookComment {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "replies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<WorkbookCommentReply>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFunctions(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChart {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(rename = "axes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axes: Option<WorkbookChartAxes>,
    #[serde(rename = "dataLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<WorkbookChartDataLabels>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartAreaFormat>,
    #[serde(rename = "legend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<WorkbookChartLegend>,
    #[serde(rename = "series")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<WorkbookChartSeries>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<WorkbookChartTitle>,
    #[serde(rename = "worksheet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worksheet: Option<WorkbookWorksheet>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxes {
    #[serde(rename = "categoryAxis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_axis: Option<WorkbookChartAxis>,
    #[serde(rename = "seriesAxis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_axis: Option<WorkbookChartAxis>,
    #[serde(rename = "valueAxis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_axis: Option<WorkbookChartAxis>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartDataLabels {
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "separator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    #[serde(rename = "showBubbleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_bubble_size: Option<bool>,
    #[serde(rename = "showCategoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_category_name: Option<bool>,
    #[serde(rename = "showLegendKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_legend_key: Option<bool>,
    #[serde(rename = "showPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_percentage: Option<bool>,
    #[serde(rename = "showSeriesName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_series_name: Option<bool>,
    #[serde(rename = "showValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_value: Option<bool>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartDataLabelFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAreaFormat {
    #[serde(rename = "fill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<WorkbookChartFill>,
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<WorkbookChartFont>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartLegend {
    #[serde(rename = "overlay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<bool>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "visible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartLegendFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartSeries {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartSeriesFormat>,
    #[serde(rename = "points")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<WorkbookChartPoint>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartTitle {
    #[serde(rename = "overlay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<bool>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "visible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartTitleFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartFill(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl WorkbookChartFill {
    pub fn new(value: Option<serde_json::Value>) -> WorkbookChartFill {
        WorkbookChartFill(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartFont {
    #[serde(rename = "bold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "italic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "underline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxis {
    #[serde(rename = "majorUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_unit: Option<Json>,
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Json>,
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<Json>,
    #[serde(rename = "minorUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_unit: Option<Json>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartAxisFormat>,
    #[serde(rename = "majorGridlines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_gridlines: Option<WorkbookChartGridlines>,
    #[serde(rename = "minorGridlines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_gridlines: Option<WorkbookChartGridlines>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<WorkbookChartAxisTitle>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxisFormat {
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<WorkbookChartFont>,
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<WorkbookChartLineFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartGridlines {
    #[serde(rename = "visible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartGridlinesFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxisTitle {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "visible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartAxisTitleFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartLineFormat {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartAxisTitleFormat {
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<WorkbookChartFont>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartDataLabelFormat {
    #[serde(rename = "fill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<WorkbookChartFill>,
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<WorkbookChartFont>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartGridlinesFormat {
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<WorkbookChartLineFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartLegendFormat {
    #[serde(rename = "fill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<WorkbookChartFill>,
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<WorkbookChartFont>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartPoint {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Json>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookChartPointFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartPointFormat {
    #[serde(rename = "fill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<WorkbookChartFill>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartSeriesFormat {
    #[serde(rename = "fill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<WorkbookChartFill>,
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<WorkbookChartLineFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookChartTitleFormat {
    #[serde(rename = "fill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<WorkbookChartFill>,
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<WorkbookChartFont>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookCommentReply {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFilter {
    #[serde(rename = "criteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<WorkbookFilterCriteria>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFormatProtection {
    #[serde(rename = "formulaHidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula_hidden: Option<bool>,
    #[serde(rename = "locked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookFunctionResult {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Json>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookPivotTable {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "worksheet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worksheet: Option<WorkbookWorksheet>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRange {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "addressLocal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_local: Option<String>,
    #[serde(rename = "cellCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_count: Option<i32>,
    #[serde(rename = "columnCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    #[serde(rename = "columnHidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hidden: Option<bool>,
    #[serde(rename = "columnIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_index: Option<i32>,
    #[serde(rename = "formulas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulas: Option<Json>,
    #[serde(rename = "formulasLocal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulas_local: Option<Json>,
    #[serde(rename = "formulasR1C1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulas_r1_c1: Option<Json>,
    #[serde(rename = "hidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "numberFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_format: Option<Json>,
    #[serde(rename = "rowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    #[serde(rename = "rowHidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_hidden: Option<bool>,
    #[serde(rename = "rowIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_index: Option<i32>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Json>,
    #[serde(rename = "valueTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_types: Option<Json>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Json>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WorkbookRangeFormat>,
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<WorkbookRangeSort>,
    #[serde(rename = "worksheet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worksheet: Option<WorkbookWorksheet>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeFormat {
    #[serde(rename = "columnWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_width: Option<i64>,
    #[serde(rename = "horizontalAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<String>,
    #[serde(rename = "rowHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_height: Option<i64>,
    #[serde(rename = "verticalAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_alignment: Option<String>,
    #[serde(rename = "wrapText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_text: Option<bool>,
    #[serde(rename = "borders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<Vec<WorkbookRangeBorder>>,
    #[serde(rename = "fill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<WorkbookRangeFill>,
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<WorkbookRangeFont>,
    #[serde(rename = "protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<WorkbookFormatProtection>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeSort(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl WorkbookRangeSort {
    pub fn new(value: Option<serde_json::Value>) -> WorkbookRangeSort {
        WorkbookRangeSort(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeBorder {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "sideIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_index: Option<String>,
    #[serde(rename = "style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeFill {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeFont {
    #[serde(rename = "bold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "italic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "underline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookRangeView {
    #[serde(rename = "cellAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_addresses: Option<Json>,
    #[serde(rename = "columnCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    #[serde(rename = "formulas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulas: Option<Json>,
    #[serde(rename = "formulasLocal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulas_local: Option<Json>,
    #[serde(rename = "formulasR1C1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulas_r1_c1: Option<Json>,
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "numberFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_format: Option<Json>,
    #[serde(rename = "rowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Json>,
    #[serde(rename = "valueTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_types: Option<Json>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Json>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<WorkbookRangeView>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTableColumn {
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Json>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<WorkbookFilter>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTableRow {
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Json>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookTableSort {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<WorkbookSortField>>,
    #[serde(rename = "matchCase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_case: Option<bool>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkbookWorksheetProtection {
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<WorkbookWorksheetProtectionOptions>,
    #[serde(rename = "protected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    #[serde(rename = "invitedUserDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_user_display_name: Option<String>,
    #[serde(rename = "invitedUserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_user_type: Option<String>,
    #[serde(rename = "invitedUserEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_user_email_address: Option<String>,
    #[serde(rename = "invitedUserMessageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_user_message_info: Option<InvitedUserMessageInfo>,
    #[serde(rename = "sendInvitationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_invitation_message: Option<bool>,
    #[serde(rename = "inviteRedirectUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_redirect_url: Option<String>,
    #[serde(rename = "inviteRedeemUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_redeem_url: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "invitedUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_user: Option<User>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerTask {
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "planId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "bucketId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "orderHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hint: Option<String>,
    #[serde(rename = "assigneePriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_priority: Option<String>,
    #[serde(rename = "percentComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[serde(rename = "startDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "dueDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date_time: Option<String>,
    #[serde(rename = "hasDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_description: Option<bool>,
    #[serde(rename = "previewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_type: Option<PlannerPreviewType>,
    #[serde(rename = "completedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_date_time: Option<String>,
    #[serde(rename = "completedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by: Option<IdentitySet>,
    #[serde(rename = "referenceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_count: Option<i32>,
    #[serde(rename = "checklistItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_item_count: Option<i32>,
    #[serde(rename = "activeChecklistItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_checklist_item_count: Option<i32>,
    #[serde(rename = "appliedCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_categories: Option<PlannerAppliedCategories>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<PlannerAssignments>,
    #[serde(rename = "conversationThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_thread_id: Option<String>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<PlannerTaskDetails>,
    #[serde(rename = "assignedToTaskBoardFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_task_board_format: Option<PlannerAssignedToTaskBoardTaskFormat>,
    #[serde(rename = "progressTaskBoardFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_task_board_format: Option<PlannerProgressTaskBoardTaskFormat>,
    #[serde(rename = "bucketTaskBoardFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_task_board_format: Option<PlannerBucketTaskBoardTaskFormat>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerPlan {
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<PlannerTask>>,
    #[serde(rename = "buckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<PlannerBucket>>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<PlannerPlanDetails>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Planner {
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<PlannerTask>>,
    #[serde(rename = "plans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Vec<PlannerPlan>>,
    #[serde(rename = "buckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<PlannerBucket>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerBucket {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "planId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "orderHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hint: Option<String>,
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<PlannerTask>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerTaskDetails {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "previewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_type: Option<PlannerPreviewType>,
    #[serde(rename = "references")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<PlannerExternalReferences>,
    #[serde(rename = "checklist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist: Option<PlannerChecklistItems>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerAssignedToTaskBoardTaskFormat {
    #[serde(rename = "unassignedOrderHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unassigned_order_hint: Option<String>,
    #[serde(rename = "orderHintsByAssignee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hints_by_assignee: Option<PlannerOrderHintsByAssignee>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerProgressTaskBoardTaskFormat {
    #[serde(rename = "orderHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hint: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerBucketTaskBoardTaskFormat {
    #[serde(rename = "orderHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hint: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlannerPlanDetails {
    #[serde(rename = "sharedWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_with: Option<PlannerUserIds>,
    #[serde(rename = "categoryDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_descriptions: Option<PlannerCategoryDescriptions>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenoteEntityBaseModel {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenoteEntitySchemaObjectModel {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenoteEntityHierarchyModel {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
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
    #[serde(rename = "sections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<OnenoteSection>>,
    #[serde(rename = "sectionGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_groups: Option<Vec<SectionGroup>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenoteSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<SectionLinks>,
    #[serde(rename = "pagesUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_url: Option<String>,
    #[serde(rename = "parentNotebook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_notebook: Option<Notebook>,
    #[serde(rename = "parentSectionGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_section_group: Option<SectionGroup>,
    #[serde(rename = "pages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<OnenotePage>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SectionGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "sectionsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections_url: Option<String>,
    #[serde(rename = "sectionGroupsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_groups_url: Option<String>,
    #[serde(rename = "parentNotebook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_notebook: Option<Notebook>,
    #[serde(rename = "parentSectionGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_section_group: Option<Box<SectionGroup>>,
    #[serde(rename = "sections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<OnenoteSection>>,
    #[serde(rename = "sectionGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_groups: Option<Vec<SectionGroup>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenotePage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "createdByAppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_app_id: Option<String>,
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<PageLinks>,
    #[serde(rename = "contentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<u8>>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "userTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_tags: Option<Vec<String>>,
    #[serde(rename = "parentSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_section: Option<OnenoteSection>,
    #[serde(rename = "parentNotebook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_notebook: Option<Notebook>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenoteResource {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<u8>>,
    #[serde(rename = "contentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatus>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastActionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_action_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnenoteOperation {
    #[serde(rename = "resourceLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<OnenoteOperationError>,
    #[serde(rename = "percentComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReportRoot(#[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>);

#[cfg(feature = "option")]
impl ReportRoot {
    pub fn new(value: Option<serde_json::Value>) -> ReportRoot {
        ReportRoot(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EducationRoot {
    #[serde(rename = "classes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<EducationClass>>,
    #[serde(rename = "schools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schools: Option<Vec<EducationSchool>>,
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<EducationUser>>,
    #[serde(rename = "me")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub me: Option<EducationUser>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EducationClass {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "mailNickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "classCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_code: Option<String>,
    #[serde(rename = "externalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_name: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "externalSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source: Option<EducationExternalSource>,
    #[serde(rename = "term")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<EducationTerm>,
    #[serde(rename = "schools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schools: Option<Vec<EducationSchool>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<EducationUser>>,
    #[serde(rename = "teachers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teachers: Option<Vec<EducationUser>>,
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EducationOrganization {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source: Option<EducationExternalSource>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EducationSchool {
    #[serde(rename = "principalEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_email: Option<String>,
    #[serde(rename = "principalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[serde(rename = "externalPrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_principal_id: Option<String>,
    #[serde(rename = "lowestGrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_grade: Option<String>,
    #[serde(rename = "highestGrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_grade: Option<String>,
    #[serde(rename = "schoolNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school_number: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "phone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "fax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PhysicalAddress>,
    #[serde(rename = "classes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<EducationClass>>,
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<EducationUser>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EducationUser {
    #[serde(rename = "primaryRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_role: Option<EducationUserRole>,
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "externalSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source: Option<EducationExternalSource>,
    #[serde(rename = "residenceAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_address: Option<PhysicalAddress>,
    #[serde(rename = "mailingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<PhysicalAddress>,
    #[serde(rename = "student")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub student: Option<EducationStudent>,
    #[serde(rename = "teacher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher: Option<EducationTeacher>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<IdentitySet>,
    #[serde(rename = "accountEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[serde(rename = "assignedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_licenses: Option<Vec<AssignedLicense>>,
    #[serde(rename = "assignedPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_plans: Option<Vec<AssignedPlan>>,
    #[serde(rename = "businessPhones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_phones: Option<Vec<String>>,
    #[serde(rename = "department")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(rename = "mail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[serde(rename = "mailNickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[serde(rename = "mobilePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "passwordPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policies: Option<String>,
    #[serde(rename = "passwordProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_profile: Option<PasswordProfile>,
    #[serde(rename = "officeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office_location: Option<String>,
    #[serde(rename = "preferredLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "provisionedPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_plans: Option<Vec<ProvisionedPlan>>,
    #[serde(rename = "refreshTokensValidFromDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_tokens_valid_from_date_time: Option<String>,
    #[serde(rename = "showInAddressList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_address_list: Option<bool>,
    #[serde(rename = "surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(rename = "usageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_location: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "userType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(rename = "schools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schools: Option<Vec<EducationSchool>>,
    #[serde(rename = "classes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<EducationClass>>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceAppManagement {
    #[serde(rename = "microsoftStoreForBusinessLastSuccessfulSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_store_for_business_last_successful_sync_date_time: Option<String>,
    #[serde(rename = "isEnabledForMicrosoftStoreForBusiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled_for_microsoft_store_for_business: Option<bool>,
    #[serde(rename = "microsoftStoreForBusinessLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_store_for_business_language: Option<String>,
    #[serde(rename = "microsoftStoreForBusinessLastCompletedApplicationSyncTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_store_for_business_last_completed_application_sync_time: Option<String>,
    #[serde(rename = "mobileApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_apps: Option<Vec<MobileApp>>,
    #[serde(rename = "mobileAppCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_app_categories: Option<Vec<MobileAppCategory>>,
    #[serde(rename = "mobileAppConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_app_configurations: Option<Vec<ManagedDeviceMobileAppConfiguration>>,
    #[serde(rename = "vppTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpp_tokens: Option<Vec<VppToken>>,
    #[serde(rename = "managedAppPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_app_policies: Option<Vec<ManagedAppPolicy>>,
    #[serde(rename = "iosManagedAppProtections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_managed_app_protections: Option<Vec<IosManagedAppProtection>>,
    #[serde(rename = "androidManagedAppProtections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_managed_app_protections: Option<Vec<AndroidManagedAppProtection>>,
    #[serde(rename = "defaultManagedAppProtections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_managed_app_protections: Option<Vec<DefaultManagedAppProtection>>,
    #[serde(rename = "targetedManagedAppConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeted_managed_app_configurations: Option<Vec<TargetedManagedAppConfiguration>>,
    #[serde(rename = "mdmWindowsInformationProtectionPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdm_windows_information_protection_policies:
        Option<Vec<MdmWindowsInformationProtectionPolicy>>,
    #[serde(rename = "windowsInformationProtectionPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_information_protection_policies: Option<Vec<WindowsInformationProtectionPolicy>>,
    #[serde(rename = "managedAppRegistrations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_app_registrations: Option<Vec<ManagedAppRegistration>>,
    #[serde(rename = "managedAppStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_app_statuses: Option<Vec<ManagedAppStatus>>,
    #[serde(rename = "managedEBooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_e_books: Option<Vec<ManagedEBook>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileApp {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "publisher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "largeIcon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_icon: Option<MimeContent>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "isFeatured")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_featured: Option<bool>,
    #[serde(rename = "privacyInformationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_information_url: Option<String>,
    #[serde(rename = "informationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information_url: Option<String>,
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "developer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer: Option<String>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "publishingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_state: Option<MobileAppPublishingState>,
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<MobileAppCategory>>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<MobileAppAssignment>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileAppCategory {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfiguration {
    #[serde(rename = "targetedMobileApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeted_mobile_apps: Option<Vec<String>>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<ManagedDeviceMobileAppConfigurationAssignment>>,
    #[serde(rename = "deviceStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_statuses: Option<Vec<ManagedDeviceMobileAppConfigurationDeviceStatus>>,
    #[serde(rename = "userStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_statuses: Option<Vec<ManagedDeviceMobileAppConfigurationUserStatus>>,
    #[serde(rename = "deviceStatusSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status_summary: Option<ManagedDeviceMobileAppConfigurationDeviceSummary>,
    #[serde(rename = "userStatusSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status_summary: Option<ManagedDeviceMobileAppConfigurationUserSummary>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VppToken {
    #[serde(rename = "organizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(rename = "vppTokenAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpp_token_account_type: Option<VppTokenAccountType>,
    #[serde(rename = "appleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_id: Option<String>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "lastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_date_time: Option<String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<VppTokenState>,
    #[serde(rename = "lastSyncStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_status: Option<VppTokenSyncStatus>,
    #[serde(rename = "automaticallyUpdateApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_update_apps: Option<bool>,
    #[serde(rename = "countryOrRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppPolicy {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppProtection {
    #[serde(rename = "periodOfflineBeforeAccessCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_offline_before_access_check: Option<String>,
    #[serde(rename = "periodOnlineBeforeAccessCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_online_before_access_check: Option<String>,
    #[serde(rename = "allowedInboundDataTransferSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_inbound_data_transfer_sources: Option<ManagedAppDataTransferLevel>,
    #[serde(rename = "allowedOutboundDataTransferDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_outbound_data_transfer_destinations: Option<ManagedAppDataTransferLevel>,
    #[serde(rename = "organizationalCredentialsRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_credentials_required: Option<bool>,
    #[serde(rename = "allowedOutboundClipboardSharingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_outbound_clipboard_sharing_level: Option<ManagedAppClipboardSharingLevel>,
    #[serde(rename = "dataBackupBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_backup_blocked: Option<bool>,
    #[serde(rename = "deviceComplianceRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_compliance_required: Option<bool>,
    #[serde(rename = "managedBrowserToOpenLinksRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_browser_to_open_links_required: Option<bool>,
    #[serde(rename = "saveAsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_as_blocked: Option<bool>,
    #[serde(rename = "periodOfflineBeforeWipeIsEnforced")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_offline_before_wipe_is_enforced: Option<String>,
    #[serde(rename = "pinRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_required: Option<bool>,
    #[serde(rename = "maximumPinRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_pin_retries: Option<i32>,
    #[serde(rename = "simplePinBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_pin_blocked: Option<bool>,
    #[serde(rename = "minimumPinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_pin_length: Option<i32>,
    #[serde(rename = "pinCharacterSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_character_set: Option<ManagedAppPinCharacterSet>,
    #[serde(rename = "periodBeforePinReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_before_pin_reset: Option<String>,
    #[serde(rename = "allowedDataStorageLocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_data_storage_locations: Option<Vec<ManagedAppDataStorageLocation>>,
    #[serde(rename = "contactSyncBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_sync_blocked: Option<bool>,
    #[serde(rename = "printBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_blocked: Option<bool>,
    #[serde(rename = "fingerprintBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint_blocked: Option<bool>,
    #[serde(rename = "disableAppPinIfDevicePinIsSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_app_pin_if_device_pin_is_set: Option<bool>,
    #[serde(rename = "minimumRequiredOsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_required_os_version: Option<String>,
    #[serde(rename = "minimumWarningOsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_warning_os_version: Option<String>,
    #[serde(rename = "minimumRequiredAppVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_required_app_version: Option<String>,
    #[serde(rename = "minimumWarningAppVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_warning_app_version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TargetedManagedAppProtection {
    #[serde(rename = "isAssigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_assigned: Option<bool>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<TargetedManagedAppPolicyAssignment>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosManagedAppProtection {
    #[serde(rename = "appDataEncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_data_encryption_type: Option<ManagedAppDataEncryptionType>,
    #[serde(rename = "minimumRequiredSdkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_required_sdk_version: Option<String>,
    #[serde(rename = "deployedAppCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_app_count: Option<i32>,
    #[serde(rename = "faceIdBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id_blocked: Option<bool>,
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<ManagedMobileApp>>,
    #[serde(rename = "deploymentSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_summary: Option<ManagedAppPolicyDeploymentSummary>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidManagedAppProtection {
    #[serde(rename = "screenCaptureBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_capture_blocked: Option<bool>,
    #[serde(rename = "disableAppEncryptionIfDeviceEncryptionIsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_app_encryption_if_device_encryption_is_enabled: Option<bool>,
    #[serde(rename = "encryptAppData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_app_data: Option<bool>,
    #[serde(rename = "deployedAppCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_app_count: Option<i32>,
    #[serde(rename = "minimumRequiredPatchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_required_patch_version: Option<String>,
    #[serde(rename = "minimumWarningPatchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_warning_patch_version: Option<String>,
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<ManagedMobileApp>>,
    #[serde(rename = "deploymentSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_summary: Option<ManagedAppPolicyDeploymentSummary>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DefaultManagedAppProtection {
    #[serde(rename = "appDataEncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_data_encryption_type: Option<ManagedAppDataEncryptionType>,
    #[serde(rename = "screenCaptureBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_capture_blocked: Option<bool>,
    #[serde(rename = "encryptAppData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_app_data: Option<bool>,
    #[serde(rename = "disableAppEncryptionIfDeviceEncryptionIsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_app_encryption_if_device_encryption_is_enabled: Option<bool>,
    #[serde(rename = "minimumRequiredSdkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_required_sdk_version: Option<String>,
    #[serde(rename = "customSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_settings: Option<Vec<KeyValuePair>>,
    #[serde(rename = "deployedAppCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_app_count: Option<i32>,
    #[serde(rename = "minimumRequiredPatchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_required_patch_version: Option<String>,
    #[serde(rename = "minimumWarningPatchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_warning_patch_version: Option<String>,
    #[serde(rename = "faceIdBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id_blocked: Option<bool>,
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<ManagedMobileApp>>,
    #[serde(rename = "deploymentSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_summary: Option<ManagedAppPolicyDeploymentSummary>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppConfiguration {
    #[serde(rename = "customSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_settings: Option<Vec<KeyValuePair>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TargetedManagedAppConfiguration {
    #[serde(rename = "deployedAppCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_app_count: Option<i32>,
    #[serde(rename = "isAssigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_assigned: Option<bool>,
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<ManagedMobileApp>>,
    #[serde(rename = "deploymentSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_summary: Option<ManagedAppPolicyDeploymentSummary>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<TargetedManagedAppPolicyAssignment>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtection {
    #[serde(rename = "enforcementLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_level: Option<WindowsInformationProtectionEnforcementLevel>,
    #[serde(rename = "enterpriseDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_domain: Option<String>,
    #[serde(rename = "enterpriseProtectedDomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_protected_domain_names:
        Option<Vec<WindowsInformationProtectionResourceCollection>>,
    #[serde(rename = "protectionUnderLockConfigRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_under_lock_config_required: Option<bool>,
    #[serde(rename = "dataRecoveryCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_recovery_certificate: Option<WindowsInformationProtectionDataRecoveryCertificate>,
    #[serde(rename = "revokeOnUnenrollDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_on_unenroll_disabled: Option<bool>,
    #[serde(rename = "rightsManagementServicesTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights_management_services_template_id: Option<String>,
    #[serde(rename = "azureRightsManagementServicesAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_rights_management_services_allowed: Option<bool>,
    #[serde(rename = "iconsVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons_visible: Option<bool>,
    #[serde(rename = "protectedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_apps: Option<Vec<WindowsInformationProtectionApp>>,
    #[serde(rename = "exemptApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_apps: Option<Vec<WindowsInformationProtectionApp>>,
    #[serde(rename = "enterpriseNetworkDomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_network_domain_names:
        Option<Vec<WindowsInformationProtectionResourceCollection>>,
    #[serde(rename = "enterpriseProxiedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_proxied_domains:
        Option<Vec<WindowsInformationProtectionProxiedDomainCollection>>,
    #[serde(rename = "enterpriseIPRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_i_p_ranges: Option<Vec<WindowsInformationProtectionIPRangeCollection>>,
    #[serde(rename = "enterpriseIPRangesAreAuthoritative")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_i_p_ranges_are_authoritative: Option<bool>,
    #[serde(rename = "enterpriseProxyServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_proxy_servers: Option<Vec<WindowsInformationProtectionResourceCollection>>,
    #[serde(rename = "enterpriseInternalProxyServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_internal_proxy_servers:
        Option<Vec<WindowsInformationProtectionResourceCollection>>,
    #[serde(rename = "enterpriseProxyServersAreAuthoritative")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_proxy_servers_are_authoritative: Option<bool>,
    #[serde(rename = "neutralDomainResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral_domain_resources: Option<Vec<WindowsInformationProtectionResourceCollection>>,
    #[serde(rename = "indexingEncryptedStoresOrItemsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_encrypted_stores_or_items_blocked: Option<bool>,
    #[serde(rename = "smbAutoEncryptedFileExtensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_auto_encrypted_file_extensions:
        Option<Vec<WindowsInformationProtectionResourceCollection>>,
    #[serde(rename = "isAssigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_assigned: Option<bool>,
    #[serde(rename = "protectedAppLockerFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_app_locker_files: Option<Vec<WindowsInformationProtectionAppLockerFile>>,
    #[serde(rename = "exemptAppLockerFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_app_locker_files: Option<Vec<WindowsInformationProtectionAppLockerFile>>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<TargetedManagedAppPolicyAssignment>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MdmWindowsInformationProtectionPolicy(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl MdmWindowsInformationProtectionPolicy {
    pub fn new(value: Option<serde_json::Value>) -> MdmWindowsInformationProtectionPolicy {
        MdmWindowsInformationProtectionPolicy(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionPolicy {
    #[serde(rename = "revokeOnMdmHandoffDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_on_mdm_handoff_disabled: Option<bool>,
    #[serde(rename = "mdmEnrollmentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdm_enrollment_url: Option<String>,
    #[serde(rename = "windowsHelloForBusinessBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_hello_for_business_blocked: Option<bool>,
    #[serde(rename = "pinMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_minimum_length: Option<i32>,
    #[serde(rename = "pinUppercaseLetters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_uppercase_letters: Option<WindowsInformationProtectionPinCharacterRequirements>,
    #[serde(rename = "pinLowercaseLetters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_lowercase_letters: Option<WindowsInformationProtectionPinCharacterRequirements>,
    #[serde(rename = "pinSpecialCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_special_characters: Option<WindowsInformationProtectionPinCharacterRequirements>,
    #[serde(rename = "pinExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_expiration_days: Option<i32>,
    #[serde(rename = "numberOfPastPinsRemembered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_past_pins_remembered: Option<i32>,
    #[serde(rename = "passwordMaximumAttemptCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_maximum_attempt_count: Option<i32>,
    #[serde(rename = "minutesOfInactivityBeforeDeviceLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_of_inactivity_before_device_lock: Option<i32>,
    #[serde(rename = "daysWithoutContactBeforeUnenroll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_without_contact_before_unenroll: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppStatus {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedEBook {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "publisher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "publishedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date_time: Option<String>,
    #[serde(rename = "largeCover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_cover: Option<MimeContent>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "informationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information_url: Option<String>,
    #[serde(rename = "privacyInformationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_information_url: Option<String>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<ManagedEBookAssignment>>,
    #[serde(rename = "installSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_summary: Option<EBookInstallSummary>,
    #[serde(rename = "deviceStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_states: Option<Vec<DeviceInstallState>>,
    #[serde(rename = "userStateSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_state_summary: Option<Vec<UserInstallStateSummary>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileAppAssignment {
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<InstallIntent>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeviceAndAppManagementAssignmentTarget>,
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MobileAppAssignmentSettings>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileAppContentFile {
    #[serde(rename = "azureStorageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_storage_uri: Option<String>,
    #[serde(rename = "isCommitted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_committed: Option<bool>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "sizeEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_encrypted: Option<i64>,
    #[serde(rename = "azureStorageUriExpirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_storage_uri_expiration_date_time: Option<String>,
    #[serde(rename = "manifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<Vec<u8>>,
    #[serde(rename = "uploadState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_state: Option<MobileAppContentFileUploadState>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationAssignment {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeviceAndAppManagementAssignmentTarget>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationDeviceStatus {
    #[serde(rename = "deviceDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_display_name: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "deviceModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_grace_period_expiration_date_time: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComplianceStatus>,
    #[serde(rename = "lastReportedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reported_date_time: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationUserStatus {
    #[serde(rename = "userDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "devicesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices_count: Option<i32>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComplianceStatus>,
    #[serde(rename = "lastReportedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reported_date_time: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationDeviceSummary {
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "notApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i32>,
    #[serde(rename = "successCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "failedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "lastUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<String>,
    #[serde(rename = "configurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceMobileAppConfigurationUserSummary {
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "notApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i32>,
    #[serde(rename = "successCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "failedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "lastUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<String>,
    #[serde(rename = "configurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MacOSOfficeSuiteApp(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl MacOSOfficeSuiteApp {
    pub fn new(value: Option<serde_json::Value>) -> MacOSOfficeSuiteApp {
        MacOSOfficeSuiteApp(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedApp {
    #[serde(rename = "appAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_availability: Option<ManagedAppAvailability>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAndroidStoreApp {
    #[serde(rename = "packageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "appStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<String>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<AndroidMinimumOperatingSystem>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedIOSStoreApp {
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "appStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<String>,
    #[serde(rename = "applicableDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_device_type: Option<IosDeviceType>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<IosMinimumOperatingSystem>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedMobileLobApp {
    #[serde(rename = "committedContentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed_content_version: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "contentVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_versions: Option<Vec<MobileAppContent>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileAppContent {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<MobileAppContentFile>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAndroidLobApp {
    #[serde(rename = "packageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<AndroidMinimumOperatingSystem>,
    #[serde(rename = "versionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_code: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedIOSLobApp {
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "applicableDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_device_type: Option<IosDeviceType>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<IosMinimumOperatingSystem>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "versionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<String>,
    #[serde(rename = "buildNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileLobApp {
    #[serde(rename = "committedContentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed_content_version: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "contentVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_versions: Option<Vec<MobileAppContent>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsMobileMSI {
    #[serde(rename = "commandLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[serde(rename = "productVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_version: Option<String>,
    #[serde(rename = "ignoreVersionDetection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_version_detection: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsUniversalAppX {
    #[serde(rename = "applicableArchitectures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_architectures: Option<WindowsArchitecture>,
    #[serde(rename = "applicableDeviceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_device_types: Option<WindowsDeviceType>,
    #[serde(rename = "identityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(rename = "identityPublisherHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_publisher_hash: Option<String>,
    #[serde(rename = "identityResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_resource_identifier: Option<String>,
    #[serde(rename = "isBundle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<WindowsMinimumOperatingSystem>,
    #[serde(rename = "identityVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidLobApp {
    #[serde(rename = "packageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<AndroidMinimumOperatingSystem>,
    #[serde(rename = "versionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_code: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosLobApp {
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "applicableDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_device_type: Option<IosDeviceType>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<IosMinimumOperatingSystem>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "versionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<String>,
    #[serde(rename = "buildNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftStoreForBusinessApp {
    #[serde(rename = "usedLicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_license_count: Option<i32>,
    #[serde(rename = "totalLicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_license_count: Option<i32>,
    #[serde(rename = "productKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_key: Option<String>,
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<MicrosoftStoreForBusinessLicenseType>,
    #[serde(rename = "packageIdentityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_identity_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WebApp {
    #[serde(rename = "appUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    #[serde(rename = "useManagedBrowser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_managed_browser: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidStoreApp {
    #[serde(rename = "packageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "appStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<String>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<AndroidMinimumOperatingSystem>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosVppApp {
    #[serde(rename = "usedLicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_license_count: Option<i32>,
    #[serde(rename = "totalLicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_license_count: Option<i32>,
    #[serde(rename = "releaseDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_time: Option<String>,
    #[serde(rename = "appStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<String>,
    #[serde(rename = "licensingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licensing_type: Option<VppLicensingType>,
    #[serde(rename = "applicableDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_device_type: Option<IosDeviceType>,
    #[serde(rename = "vppTokenOrganizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpp_token_organization_name: Option<String>,
    #[serde(rename = "vppTokenAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpp_token_account_type: Option<VppTokenAccountType>,
    #[serde(rename = "vppTokenAppleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpp_token_apple_id: Option<String>,
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosStoreApp {
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "appStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<String>,
    #[serde(rename = "applicableDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_device_type: Option<IosDeviceType>,
    #[serde(rename = "minimumSupportedOperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_supported_operating_system: Option<IosMinimumOperatingSystem>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosMobileAppConfiguration {
    #[serde(rename = "encodedSettingXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_setting_xml: Option<Vec<u8>>,
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<AppConfigurationSettingItem>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagement {
    #[serde(rename = "subscriptionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_state: Option<DeviceManagementSubscriptionState>,
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DeviceManagementSettings>,
    #[serde(rename = "intuneBrand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intune_brand: Option<IntuneBrand>,
    #[serde(rename = "termsAndConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_and_conditions: Option<Vec<TermsAndConditions>>,
    #[serde(rename = "applePushNotificationCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_push_notification_certificate: Option<ApplePushNotificationCertificate>,
    #[serde(rename = "managedDeviceOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_overview: Option<ManagedDeviceOverview>,
    #[serde(rename = "detectedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_apps: Option<Vec<DetectedApp>>,
    #[serde(rename = "managedDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_devices: Option<Vec<ManagedDevice>>,
    #[serde(rename = "deviceConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configurations: Option<Vec<DeviceConfiguration>>,
    #[serde(rename = "deviceCompliancePolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_compliance_policies: Option<Vec<DeviceCompliancePolicy>>,
    #[serde(rename = "softwareUpdateStatusSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_update_status_summary: Option<SoftwareUpdateStatusSummary>,
    #[serde(rename = "deviceCompliancePolicyDeviceStateSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_compliance_policy_device_state_summary:
        Option<DeviceCompliancePolicyDeviceStateSummary>,
    #[serde(rename = "deviceCompliancePolicySettingStateSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_compliance_policy_setting_state_summaries:
        Option<Vec<DeviceCompliancePolicySettingStateSummary>>,
    #[serde(rename = "deviceConfigurationDeviceStateSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration_device_state_summaries: Option<DeviceConfigurationDeviceStateSummary>,
    #[serde(rename = "iosUpdateStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_update_statuses: Option<Vec<IosUpdateDeviceStatus>>,
    #[serde(rename = "deviceCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_categories: Option<Vec<DeviceCategory>>,
    #[serde(rename = "exchangeConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_connectors: Option<Vec<DeviceManagementExchangeConnector>>,
    #[serde(rename = "deviceEnrollmentConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_enrollment_configurations: Option<Vec<DeviceEnrollmentConfiguration>>,
    #[serde(rename = "conditionalAccessSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_access_settings: Option<OnPremisesConditionalAccessSettings>,
    #[serde(rename = "mobileThreatDefenseConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_threat_defense_connectors: Option<Vec<MobileThreatDefenseConnector>>,
    #[serde(rename = "deviceManagementPartners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_management_partners: Option<Vec<DeviceManagementPartner>>,
    #[serde(rename = "notificationMessageTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_message_templates: Option<Vec<NotificationMessageTemplate>>,
    #[serde(rename = "roleDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_definitions: Option<Vec<RoleDefinition>>,
    #[serde(rename = "roleAssignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<Vec<DeviceAndAppManagementRoleAssignment>>,
    #[serde(rename = "resourceOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_operations: Option<Vec<ResourceOperation>>,
    #[serde(rename = "telecomExpenseManagementPartners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecom_expense_management_partners: Option<Vec<TelecomExpenseManagementPartner>>,
    #[serde(rename = "remoteAssistancePartners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_assistance_partners: Option<Vec<RemoteAssistancePartner>>,
    #[serde(rename = "windowsInformationProtectionAppLearningSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_information_protection_app_learning_summaries:
        Option<Vec<WindowsInformationProtectionAppLearningSummary>>,
    #[serde(rename = "windowsInformationProtectionNetworkLearningSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_information_protection_network_learning_summaries:
        Option<Vec<WindowsInformationProtectionNetworkLearningSummary>>,
    #[serde(rename = "troubleshootingEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub troubleshooting_events: Option<Vec<DeviceManagementTroubleshootingEvent>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TermsAndConditions {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "bodyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(rename = "acceptanceStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_statement: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<TermsAndConditionsAssignment>>,
    #[serde(rename = "acceptanceStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_statuses: Option<Vec<TermsAndConditionsAcceptanceStatus>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ApplePushNotificationCertificate {
    #[serde(rename = "appleIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_identifier: Option<String>,
    #[serde(rename = "topicIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_identifier: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedDeviceOverview {
    #[serde(rename = "enrolledDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrolled_device_count: Option<i32>,
    #[serde(rename = "mdmEnrolledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdm_enrolled_count: Option<i32>,
    #[serde(rename = "dualEnrolledDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_enrolled_device_count: Option<i32>,
    #[serde(rename = "deviceOperatingSystemSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_operating_system_summary: Option<DeviceOperatingSystemSummary>,
    #[serde(rename = "deviceExchangeAccessStateSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_exchange_access_state_summary: Option<DeviceExchangeAccessStateSummary>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DetectedApp {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "sizeInByte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_byte: Option<i64>,
    #[serde(rename = "deviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(rename = "managedDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_devices: Option<Vec<ManagedDevice>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfiguration {
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<DeviceConfigurationAssignment>>,
    #[serde(rename = "deviceStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_statuses: Option<Vec<DeviceConfigurationDeviceStatus>>,
    #[serde(rename = "userStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_statuses: Option<Vec<DeviceConfigurationUserStatus>>,
    #[serde(rename = "deviceStatusOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status_overview: Option<DeviceConfigurationDeviceOverview>,
    #[serde(rename = "userStatusOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status_overview: Option<DeviceConfigurationUserOverview>,
    #[serde(rename = "deviceSettingStateSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_setting_state_summaries: Option<Vec<SettingStateDeviceSummary>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicy {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "scheduledActionsForRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_actions_for_rule: Option<Vec<DeviceComplianceScheduledActionForRule>>,
    #[serde(rename = "deviceStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_statuses: Option<Vec<DeviceComplianceDeviceStatus>>,
    #[serde(rename = "userStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_statuses: Option<Vec<DeviceComplianceUserStatus>>,
    #[serde(rename = "deviceStatusOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status_overview: Option<DeviceComplianceDeviceOverview>,
    #[serde(rename = "userStatusOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status_overview: Option<DeviceComplianceUserOverview>,
    #[serde(rename = "deviceSettingStateSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_setting_state_summaries: Option<Vec<SettingStateDeviceSummary>>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<DeviceCompliancePolicyAssignment>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SoftwareUpdateStatusSummary {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "compliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_device_count: Option<i32>,
    #[serde(rename = "nonCompliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_device_count: Option<i32>,
    #[serde(rename = "remediatedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediated_device_count: Option<i32>,
    #[serde(rename = "errorDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_device_count: Option<i32>,
    #[serde(rename = "unknownDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_device_count: Option<i32>,
    #[serde(rename = "conflictDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_device_count: Option<i32>,
    #[serde(rename = "notApplicableDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_device_count: Option<i32>,
    #[serde(rename = "compliantUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_user_count: Option<i32>,
    #[serde(rename = "nonCompliantUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_user_count: Option<i32>,
    #[serde(rename = "remediatedUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediated_user_count: Option<i32>,
    #[serde(rename = "errorUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_user_count: Option<i32>,
    #[serde(rename = "unknownUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_user_count: Option<i32>,
    #[serde(rename = "conflictUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_user_count: Option<i32>,
    #[serde(rename = "notApplicableUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_user_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicyDeviceStateSummary {
    #[serde(rename = "inGracePeriodCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_grace_period_count: Option<i32>,
    #[serde(rename = "configManagerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_manager_count: Option<i32>,
    #[serde(rename = "unknownDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_device_count: Option<i32>,
    #[serde(rename = "notApplicableDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_device_count: Option<i32>,
    #[serde(rename = "compliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_device_count: Option<i32>,
    #[serde(rename = "remediatedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediated_device_count: Option<i32>,
    #[serde(rename = "nonCompliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_device_count: Option<i32>,
    #[serde(rename = "errorDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_device_count: Option<i32>,
    #[serde(rename = "conflictDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_device_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicySettingStateSummary {
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<String>,
    #[serde(rename = "settingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_name: Option<String>,
    #[serde(rename = "platformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<PolicyPlatformType>,
    #[serde(rename = "unknownDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_device_count: Option<i32>,
    #[serde(rename = "notApplicableDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_device_count: Option<i32>,
    #[serde(rename = "compliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_device_count: Option<i32>,
    #[serde(rename = "remediatedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediated_device_count: Option<i32>,
    #[serde(rename = "nonCompliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_device_count: Option<i32>,
    #[serde(rename = "errorDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_device_count: Option<i32>,
    #[serde(rename = "conflictDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_device_count: Option<i32>,
    #[serde(rename = "deviceComplianceSettingStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_compliance_setting_states: Option<Vec<DeviceComplianceSettingState>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationDeviceStateSummary {
    #[serde(rename = "unknownDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_device_count: Option<i32>,
    #[serde(rename = "notApplicableDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_device_count: Option<i32>,
    #[serde(rename = "compliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_device_count: Option<i32>,
    #[serde(rename = "remediatedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediated_device_count: Option<i32>,
    #[serde(rename = "nonCompliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_device_count: Option<i32>,
    #[serde(rename = "errorDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_device_count: Option<i32>,
    #[serde(rename = "conflictDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_device_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosUpdateDeviceStatus {
    #[serde(rename = "installStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_status: Option<IosUpdatesInstallStatus>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "deviceDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_display_name: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "deviceModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_grace_period_expiration_date_time: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComplianceStatus>,
    #[serde(rename = "lastReportedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reported_date_time: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceCategory {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagementExchangeConnector {
    #[serde(rename = "lastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_date_time: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeviceManagementExchangeConnectorStatus>,
    #[serde(rename = "primarySmtpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_smtp_address: Option<String>,
    #[serde(rename = "serverName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "connectorServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_server_name: Option<String>,
    #[serde(rename = "exchangeConnectorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_connector_type: Option<DeviceManagementExchangeConnectorType>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "exchangeAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_alias: Option<String>,
    #[serde(rename = "exchangeOrganization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_organization: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentConfiguration {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<EnrollmentConfigurationAssignment>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OnPremisesConditionalAccessSettings {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "includedGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_groups: Option<Vec<String>>,
    #[serde(rename = "excludedGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_groups: Option<Vec<String>>,
    #[serde(rename = "overrideDefaultRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_default_rule: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MobileThreatDefenseConnector {
    #[serde(rename = "lastHeartbeatDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_date_time: Option<String>,
    #[serde(rename = "partnerState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_state: Option<MobileThreatPartnerTenantState>,
    #[serde(rename = "androidEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_enabled: Option<bool>,
    #[serde(rename = "iosEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_enabled: Option<bool>,
    #[serde(rename = "androidDeviceBlockedOnMissingPartnerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_device_blocked_on_missing_partner_data: Option<bool>,
    #[serde(rename = "iosDeviceBlockedOnMissingPartnerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_device_blocked_on_missing_partner_data: Option<bool>,
    #[serde(rename = "partnerUnsupportedOsVersionBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_unsupported_os_version_blocked: Option<bool>,
    #[serde(rename = "partnerUnresponsivenessThresholdInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_unresponsiveness_threshold_in_days: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceManagementPartner {
    #[serde(rename = "lastHeartbeatDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_date_time: Option<String>,
    #[serde(rename = "partnerState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_state: Option<DeviceManagementPartnerTenantState>,
    #[serde(rename = "partnerAppType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_app_type: Option<DeviceManagementPartnerAppType>,
    #[serde(rename = "singleTenantAppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_tenant_app_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "isConfigured")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_configured: Option<bool>,
    #[serde(rename = "whenPartnerDevicesWillBeRemovedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_partner_devices_will_be_removed_date_time: Option<String>,
    #[serde(rename = "whenPartnerDevicesWillBeMarkedAsNonCompliantDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_partner_devices_will_be_marked_as_non_compliant_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NotificationMessageTemplate {
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "defaultLocale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    #[serde(rename = "brandingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding_options: Option<NotificationTemplateBrandingOptions>,
    #[serde(rename = "localizedNotificationMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_notification_messages: Option<Vec<LocalizedNotificationMessage>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RoleDefinition {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "rolePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_permissions: Option<Vec<RolePermission>>,
    #[serde(rename = "isBuiltIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_built_in: Option<bool>,
    #[serde(rename = "roleAssignments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<Vec<RoleAssignment>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resourceScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_scopes: Option<Vec<String>>,
    #[serde(rename = "roleDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_definition: Option<RoleDefinition>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceAndAppManagementRoleAssignment {
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourceOperation {
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TelecomExpenseManagementPartner {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "appAuthorized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_authorized: Option<bool>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lastConnectionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RemoteAssistancePartner {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "onboardingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_url: Option<String>,
    #[serde(rename = "onboardingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<RemoteAssistanceOnboardingStatus>,
    #[serde(rename = "lastConnectionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionAppLearningSummary {
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "applicationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<ApplicationType>,
    #[serde(rename = "deviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionNetworkLearningSummary {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "deviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TermsAndConditionsAssignment {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeviceAndAppManagementAssignmentTarget>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TermsAndConditionsAcceptanceStatus {
    #[serde(rename = "userDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "acceptedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_version: Option<i32>,
    #[serde(rename = "acceptedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_date_time: Option<String>,
    #[serde(rename = "termsAndConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_and_conditions: Option<TermsAndConditions>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationState {
    #[serde(rename = "settingStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_states: Option<Vec<DeviceConfigurationSettingState>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "platformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<PolicyPlatformType>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ComplianceStatus>,
    #[serde(rename = "settingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicyState {
    #[serde(rename = "settingStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_states: Option<Vec<DeviceCompliancePolicySettingState>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "platformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<PolicyPlatformType>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ComplianceStatus>,
    #[serde(rename = "settingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationAssignment {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeviceAndAppManagementAssignmentTarget>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationDeviceStatus {
    #[serde(rename = "deviceDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_display_name: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "deviceModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_grace_period_expiration_date_time: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComplianceStatus>,
    #[serde(rename = "lastReportedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reported_date_time: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationUserStatus {
    #[serde(rename = "userDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "devicesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices_count: Option<i32>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComplianceStatus>,
    #[serde(rename = "lastReportedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reported_date_time: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationDeviceOverview {
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "notApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i32>,
    #[serde(rename = "successCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "failedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "lastUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<String>,
    #[serde(rename = "configurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceConfigurationUserOverview {
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "notApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i32>,
    #[serde(rename = "successCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "failedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "lastUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<String>,
    #[serde(rename = "configurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SettingStateDeviceSummary {
    #[serde(rename = "settingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_name: Option<String>,
    #[serde(rename = "instancePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_path: Option<String>,
    #[serde(rename = "unknownDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_device_count: Option<i32>,
    #[serde(rename = "notApplicableDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_device_count: Option<i32>,
    #[serde(rename = "compliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_device_count: Option<i32>,
    #[serde(rename = "remediatedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediated_device_count: Option<i32>,
    #[serde(rename = "nonCompliantDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_device_count: Option<i32>,
    #[serde(rename = "errorDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_device_count: Option<i32>,
    #[serde(rename = "conflictDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_device_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceCompliancePolicyAssignment {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeviceAndAppManagementAssignmentTarget>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceScheduledActionForRule {
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "scheduledActionConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_configurations: Option<Vec<DeviceComplianceActionItem>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceDeviceStatus {
    #[serde(rename = "deviceDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_display_name: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "deviceModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_grace_period_expiration_date_time: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComplianceStatus>,
    #[serde(rename = "lastReportedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reported_date_time: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceUserStatus {
    #[serde(rename = "userDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "devicesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices_count: Option<i32>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComplianceStatus>,
    #[serde(rename = "lastReportedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reported_date_time: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceDeviceOverview {
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "notApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i32>,
    #[serde(rename = "successCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "failedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "lastUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<String>,
    #[serde(rename = "configurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceUserOverview {
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i32>,
    #[serde(rename = "notApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i32>,
    #[serde(rename = "successCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "failedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "lastUpdateDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<String>,
    #[serde(rename = "configurationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceActionItem {
    #[serde(rename = "gracePeriodHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_period_hours: Option<i32>,
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<DeviceComplianceActionType>,
    #[serde(rename = "notificationTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_template_id: Option<String>,
    #[serde(rename = "notificationMessageCCList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_message_c_c_list: Option<Vec<String>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidCustomConfiguration {
    #[serde(rename = "omaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oma_settings: Option<Vec<OmaSetting>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidGeneralDeviceConfiguration {
    #[serde(rename = "appsBlockClipboardSharing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_block_clipboard_sharing: Option<bool>,
    #[serde(rename = "appsBlockCopyPaste")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_block_copy_paste: Option<bool>,
    #[serde(rename = "appsBlockYouTube")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_block_you_tube: Option<bool>,
    #[serde(rename = "bluetoothBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_blocked: Option<bool>,
    #[serde(rename = "cameraBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_blocked: Option<bool>,
    #[serde(rename = "cellularBlockDataRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_data_roaming: Option<bool>,
    #[serde(rename = "cellularBlockMessaging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_messaging: Option<bool>,
    #[serde(rename = "cellularBlockVoiceRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_voice_roaming: Option<bool>,
    #[serde(rename = "cellularBlockWiFiTethering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_wi_fi_tethering: Option<bool>,
    #[serde(rename = "compliantAppsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_apps_list: Option<Vec<AppListItem>>,
    #[serde(rename = "compliantAppListType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_app_list_type: Option<AppListType>,
    #[serde(rename = "diagnosticDataBlockSubmission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_data_block_submission: Option<bool>,
    #[serde(rename = "locationServicesBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_services_blocked: Option<bool>,
    #[serde(rename = "googleAccountBlockAutoSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_account_block_auto_sync: Option<bool>,
    #[serde(rename = "googlePlayStoreBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_play_store_blocked: Option<bool>,
    #[serde(rename = "kioskModeBlockSleepButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_block_sleep_button: Option<bool>,
    #[serde(rename = "kioskModeBlockVolumeButtons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_block_volume_buttons: Option<bool>,
    #[serde(rename = "kioskModeApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_apps: Option<Vec<AppListItem>>,
    #[serde(rename = "nfcBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfc_blocked: Option<bool>,
    #[serde(rename = "passwordBlockFingerprintUnlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_fingerprint_unlock: Option<bool>,
    #[serde(rename = "passwordBlockTrustAgents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_trust_agents: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_screen_timeout: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_sign_in_failure_count_before_factory_reset: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<AndroidRequiredPasswordType>,
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "powerOffBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_off_blocked: Option<bool>,
    #[serde(rename = "factoryResetBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factory_reset_blocked: Option<bool>,
    #[serde(rename = "screenCaptureBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_capture_blocked: Option<bool>,
    #[serde(rename = "deviceSharingAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_sharing_allowed: Option<bool>,
    #[serde(rename = "storageBlockGoogleBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_block_google_backup: Option<bool>,
    #[serde(rename = "storageBlockRemovableStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_block_removable_storage: Option<bool>,
    #[serde(rename = "storageRequireDeviceEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_device_encryption: Option<bool>,
    #[serde(rename = "storageRequireRemovableStorageEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_removable_storage_encryption: Option<bool>,
    #[serde(rename = "voiceAssistantBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_assistant_blocked: Option<bool>,
    #[serde(rename = "voiceDialingBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_dialing_blocked: Option<bool>,
    #[serde(rename = "webBrowserBlockPopups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_browser_block_popups: Option<bool>,
    #[serde(rename = "webBrowserBlockAutofill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_browser_block_autofill: Option<bool>,
    #[serde(rename = "webBrowserBlockJavaScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_browser_block_java_script: Option<bool>,
    #[serde(rename = "webBrowserBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_browser_blocked: Option<bool>,
    #[serde(rename = "webBrowserCookieSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_browser_cookie_settings: Option<WebBrowserCookieSettings>,
    #[serde(rename = "wiFiBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi_blocked: Option<bool>,
    #[serde(rename = "appsInstallAllowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_install_allow_list: Option<Vec<AppListItem>>,
    #[serde(rename = "appsLaunchBlockList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_launch_block_list: Option<Vec<AppListItem>>,
    #[serde(rename = "appsHideList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_hide_list: Option<Vec<AppListItem>>,
    #[serde(rename = "securityRequireVerifyApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_verify_apps: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidWorkProfileCustomConfiguration {
    #[serde(rename = "omaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oma_settings: Option<Vec<OmaSetting>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidWorkProfileGeneralDeviceConfiguration {
    #[serde(rename = "passwordBlockFingerprintUnlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_fingerprint_unlock: Option<bool>,
    #[serde(rename = "passwordBlockTrustAgents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_trust_agents: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_screen_timeout: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_sign_in_failure_count_before_factory_reset: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<AndroidWorkProfileRequiredPasswordType>,
    #[serde(rename = "workProfileDataSharingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_data_sharing_type: Option<AndroidWorkProfileCrossProfileDataSharingType>,
    #[serde(rename = "workProfileBlockNotificationsWhileDeviceLocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_block_notifications_while_device_locked: Option<bool>,
    #[serde(rename = "workProfileBlockAddingAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_block_adding_accounts: Option<bool>,
    #[serde(rename = "workProfileBluetoothEnableContactSharing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_bluetooth_enable_contact_sharing: Option<bool>,
    #[serde(rename = "workProfileBlockScreenCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_block_screen_capture: Option<bool>,
    #[serde(rename = "workProfileBlockCrossProfileCallerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_block_cross_profile_caller_id: Option<bool>,
    #[serde(rename = "workProfileBlockCamera")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_block_camera: Option<bool>,
    #[serde(rename = "workProfileBlockCrossProfileContactsSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_block_cross_profile_contacts_search: Option<bool>,
    #[serde(rename = "workProfileBlockCrossProfileCopyPaste")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_block_cross_profile_copy_paste: Option<bool>,
    #[serde(rename = "workProfileDefaultAppPermissionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_default_app_permission_policy:
        Option<AndroidWorkProfileDefaultAppPermissionPolicyType>,
    #[serde(rename = "workProfilePasswordBlockFingerprintUnlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_block_fingerprint_unlock: Option<bool>,
    #[serde(rename = "workProfilePasswordBlockTrustAgents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_block_trust_agents: Option<bool>,
    #[serde(rename = "workProfilePasswordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_expiration_days: Option<i32>,
    #[serde(rename = "workProfilePasswordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_minimum_length: Option<i32>,
    #[serde(rename = "workProfilePasswordMinNumericCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_min_numeric_characters: Option<i32>,
    #[serde(rename = "workProfilePasswordMinNonLetterCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_min_non_letter_characters: Option<i32>,
    #[serde(rename = "workProfilePasswordMinLetterCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_min_letter_characters: Option<i32>,
    #[serde(rename = "workProfilePasswordMinLowerCaseCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_min_lower_case_characters: Option<i32>,
    #[serde(rename = "workProfilePasswordMinUpperCaseCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_min_upper_case_characters: Option<i32>,
    #[serde(rename = "workProfilePasswordMinSymbolCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_min_symbol_characters: Option<i32>,
    #[serde(rename = "workProfilePasswordMinutesOfInactivityBeforeScreenTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_minutes_of_inactivity_before_screen_timeout: Option<i32>,
    #[serde(rename = "workProfilePasswordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_previous_password_block_count: Option<i32>,
    #[serde(rename = "workProfilePasswordSignInFailureCountBeforeFactoryReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_sign_in_failure_count_before_factory_reset: Option<i32>,
    #[serde(rename = "workProfilePasswordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_password_required_type: Option<AndroidWorkProfileRequiredPasswordType>,
    #[serde(rename = "workProfileRequirePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_profile_require_password: Option<bool>,
    #[serde(rename = "securityRequireVerifyApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_verify_apps: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosCertificateProfile(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl IosCertificateProfile {
    pub fn new(value: Option<serde_json::Value>) -> IosCertificateProfile {
        IosCertificateProfile(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosCustomConfiguration {
    #[serde(rename = "payloadName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_name: Option<String>,
    #[serde(rename = "payloadFileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_file_name: Option<String>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<u8>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosGeneralDeviceConfiguration {
    #[serde(rename = "accountBlockModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_block_modification: Option<bool>,
    #[serde(rename = "activationLockAllowWhenSupervised")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_lock_allow_when_supervised: Option<bool>,
    #[serde(rename = "airDropBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_drop_blocked: Option<bool>,
    #[serde(rename = "airDropForceUnmanagedDropTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_drop_force_unmanaged_drop_target: Option<bool>,
    #[serde(rename = "airPlayForcePairingPasswordForOutgoingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_play_force_pairing_password_for_outgoing_requests: Option<bool>,
    #[serde(rename = "appleWatchBlockPairing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_watch_block_pairing: Option<bool>,
    #[serde(rename = "appleWatchForceWristDetection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_watch_force_wrist_detection: Option<bool>,
    #[serde(rename = "appleNewsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_news_blocked: Option<bool>,
    #[serde(rename = "appsSingleAppModeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_single_app_mode_list: Option<Vec<AppListItem>>,
    #[serde(rename = "appsVisibilityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_visibility_list: Option<Vec<AppListItem>>,
    #[serde(rename = "appsVisibilityListType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_visibility_list_type: Option<AppListType>,
    #[serde(rename = "appStoreBlockAutomaticDownloads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_block_automatic_downloads: Option<bool>,
    #[serde(rename = "appStoreBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_blocked: Option<bool>,
    #[serde(rename = "appStoreBlockInAppPurchases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_block_in_app_purchases: Option<bool>,
    #[serde(rename = "appStoreBlockUIAppInstallation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_block_u_i_app_installation: Option<bool>,
    #[serde(rename = "appStoreRequirePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_require_password: Option<bool>,
    #[serde(rename = "bluetoothBlockModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_block_modification: Option<bool>,
    #[serde(rename = "cameraBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_blocked: Option<bool>,
    #[serde(rename = "cellularBlockDataRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_data_roaming: Option<bool>,
    #[serde(rename = "cellularBlockGlobalBackgroundFetchWhileRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_global_background_fetch_while_roaming: Option<bool>,
    #[serde(rename = "cellularBlockPerAppDataModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_per_app_data_modification: Option<bool>,
    #[serde(rename = "cellularBlockPersonalHotspot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_personal_hotspot: Option<bool>,
    #[serde(rename = "cellularBlockVoiceRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_voice_roaming: Option<bool>,
    #[serde(rename = "certificatesBlockUntrustedTlsCertificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates_block_untrusted_tls_certificates: Option<bool>,
    #[serde(rename = "classroomAppBlockRemoteScreenObservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classroom_app_block_remote_screen_observation: Option<bool>,
    #[serde(rename = "classroomAppForceUnpromptedScreenObservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classroom_app_force_unprompted_screen_observation: Option<bool>,
    #[serde(rename = "compliantAppsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_apps_list: Option<Vec<AppListItem>>,
    #[serde(rename = "compliantAppListType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_app_list_type: Option<AppListType>,
    #[serde(rename = "configurationProfileBlockChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_profile_block_changes: Option<bool>,
    #[serde(rename = "definitionLookupBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_lookup_blocked: Option<bool>,
    #[serde(rename = "deviceBlockEnableRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_block_enable_restrictions: Option<bool>,
    #[serde(rename = "deviceBlockEraseContentAndSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_block_erase_content_and_settings: Option<bool>,
    #[serde(rename = "deviceBlockNameModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_block_name_modification: Option<bool>,
    #[serde(rename = "diagnosticDataBlockSubmission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_data_block_submission: Option<bool>,
    #[serde(rename = "diagnosticDataBlockSubmissionModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_data_block_submission_modification: Option<bool>,
    #[serde(rename = "documentsBlockManagedDocumentsInUnmanagedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_block_managed_documents_in_unmanaged_apps: Option<bool>,
    #[serde(rename = "documentsBlockUnmanagedDocumentsInManagedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_block_unmanaged_documents_in_managed_apps: Option<bool>,
    #[serde(rename = "emailInDomainSuffixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_in_domain_suffixes: Option<Vec<String>>,
    #[serde(rename = "enterpriseAppBlockTrust")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_app_block_trust: Option<bool>,
    #[serde(rename = "enterpriseAppBlockTrustModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_app_block_trust_modification: Option<bool>,
    #[serde(rename = "faceTimeBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_time_blocked: Option<bool>,
    #[serde(rename = "findMyFriendsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_my_friends_blocked: Option<bool>,
    #[serde(rename = "gamingBlockGameCenterFriends")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gaming_block_game_center_friends: Option<bool>,
    #[serde(rename = "gamingBlockMultiplayer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gaming_block_multiplayer: Option<bool>,
    #[serde(rename = "gameCenterBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_center_blocked: Option<bool>,
    #[serde(rename = "hostPairingBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_pairing_blocked: Option<bool>,
    #[serde(rename = "iBooksStoreBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_books_store_blocked: Option<bool>,
    #[serde(rename = "iBooksStoreBlockErotica")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_books_store_block_erotica: Option<bool>,
    #[serde(rename = "iCloudBlockActivityContinuation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cloud_block_activity_continuation: Option<bool>,
    #[serde(rename = "iCloudBlockBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cloud_block_backup: Option<bool>,
    #[serde(rename = "iCloudBlockDocumentSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cloud_block_document_sync: Option<bool>,
    #[serde(rename = "iCloudBlockManagedAppsSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cloud_block_managed_apps_sync: Option<bool>,
    #[serde(rename = "iCloudBlockPhotoLibrary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cloud_block_photo_library: Option<bool>,
    #[serde(rename = "iCloudBlockPhotoStreamSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cloud_block_photo_stream_sync: Option<bool>,
    #[serde(rename = "iCloudBlockSharedPhotoStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cloud_block_shared_photo_stream: Option<bool>,
    #[serde(rename = "iCloudRequireEncryptedBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_cloud_require_encrypted_backup: Option<bool>,
    #[serde(rename = "iTunesBlockExplicitContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_tunes_block_explicit_content: Option<bool>,
    #[serde(rename = "iTunesBlockMusicService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_tunes_block_music_service: Option<bool>,
    #[serde(rename = "iTunesBlockRadio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_tunes_block_radio: Option<bool>,
    #[serde(rename = "keyboardBlockAutoCorrect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_block_auto_correct: Option<bool>,
    #[serde(rename = "keyboardBlockDictation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_block_dictation: Option<bool>,
    #[serde(rename = "keyboardBlockPredictive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_block_predictive: Option<bool>,
    #[serde(rename = "keyboardBlockShortcuts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_block_shortcuts: Option<bool>,
    #[serde(rename = "keyboardBlockSpellCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_block_spell_check: Option<bool>,
    #[serde(rename = "kioskModeAllowAssistiveSpeak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_assistive_speak: Option<bool>,
    #[serde(rename = "kioskModeAllowAssistiveTouchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_assistive_touch_settings: Option<bool>,
    #[serde(rename = "kioskModeAllowAutoLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_auto_lock: Option<bool>,
    #[serde(rename = "kioskModeAllowColorInversionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_color_inversion_settings: Option<bool>,
    #[serde(rename = "kioskModeAllowRingerSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_ringer_switch: Option<bool>,
    #[serde(rename = "kioskModeAllowScreenRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_screen_rotation: Option<bool>,
    #[serde(rename = "kioskModeAllowSleepButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_sleep_button: Option<bool>,
    #[serde(rename = "kioskModeAllowTouchscreen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_touchscreen: Option<bool>,
    #[serde(rename = "kioskModeAllowVoiceOverSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_voice_over_settings: Option<bool>,
    #[serde(rename = "kioskModeAllowVolumeButtons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_volume_buttons: Option<bool>,
    #[serde(rename = "kioskModeAllowZoomSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_allow_zoom_settings: Option<bool>,
    #[serde(rename = "kioskModeAppStoreUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_app_store_url: Option<String>,
    #[serde(rename = "kioskModeBuiltInAppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_built_in_app_id: Option<String>,
    #[serde(rename = "kioskModeRequireAssistiveTouch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_require_assistive_touch: Option<bool>,
    #[serde(rename = "kioskModeRequireColorInversion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_require_color_inversion: Option<bool>,
    #[serde(rename = "kioskModeRequireMonoAudio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_require_mono_audio: Option<bool>,
    #[serde(rename = "kioskModeRequireVoiceOver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_require_voice_over: Option<bool>,
    #[serde(rename = "kioskModeRequireZoom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_require_zoom: Option<bool>,
    #[serde(rename = "kioskModeManagedAppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_mode_managed_app_id: Option<String>,
    #[serde(rename = "lockScreenBlockControlCenter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_block_control_center: Option<bool>,
    #[serde(rename = "lockScreenBlockNotificationView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_block_notification_view: Option<bool>,
    #[serde(rename = "lockScreenBlockPassbook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_block_passbook: Option<bool>,
    #[serde(rename = "lockScreenBlockTodayView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_block_today_view: Option<bool>,
    #[serde(rename = "mediaContentRatingAustralia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_australia: Option<MediaContentRatingAustralia>,
    #[serde(rename = "mediaContentRatingCanada")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_canada: Option<MediaContentRatingCanada>,
    #[serde(rename = "mediaContentRatingFrance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_france: Option<MediaContentRatingFrance>,
    #[serde(rename = "mediaContentRatingGermany")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_germany: Option<MediaContentRatingGermany>,
    #[serde(rename = "mediaContentRatingIreland")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_ireland: Option<MediaContentRatingIreland>,
    #[serde(rename = "mediaContentRatingJapan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_japan: Option<MediaContentRatingJapan>,
    #[serde(rename = "mediaContentRatingNewZealand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_new_zealand: Option<MediaContentRatingNewZealand>,
    #[serde(rename = "mediaContentRatingUnitedKingdom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_united_kingdom: Option<MediaContentRatingUnitedKingdom>,
    #[serde(rename = "mediaContentRatingUnitedStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_united_states: Option<MediaContentRatingUnitedStates>,
    #[serde(rename = "networkUsageRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_usage_rules: Option<Vec<IosNetworkUsageRule>>,
    #[serde(rename = "mediaContentRatingApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_content_rating_apps: Option<RatingAppsType>,
    #[serde(rename = "messagesBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_blocked: Option<bool>,
    #[serde(rename = "notificationsBlockSettingsModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_block_settings_modification: Option<bool>,
    #[serde(rename = "passcodeBlockFingerprintUnlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_block_fingerprint_unlock: Option<bool>,
    #[serde(rename = "passcodeBlockFingerprintModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_block_fingerprint_modification: Option<bool>,
    #[serde(rename = "passcodeBlockModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_block_modification: Option<bool>,
    #[serde(rename = "passcodeBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_block_simple: Option<bool>,
    #[serde(rename = "passcodeExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_expiration_days: Option<i32>,
    #[serde(rename = "passcodeMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_minimum_length: Option<i32>,
    #[serde(rename = "passcodeMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passcodeMinutesOfInactivityBeforeScreenTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_minutes_of_inactivity_before_screen_timeout: Option<i32>,
    #[serde(rename = "passcodeMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passcodePreviousPasscodeBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_previous_passcode_block_count: Option<i32>,
    #[serde(rename = "passcodeSignInFailureCountBeforeWipe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_sign_in_failure_count_before_wipe: Option<i32>,
    #[serde(rename = "passcodeRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passcodeRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_required: Option<bool>,
    #[serde(rename = "podcastsBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub podcasts_blocked: Option<bool>,
    #[serde(rename = "safariBlockAutofill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safari_block_autofill: Option<bool>,
    #[serde(rename = "safariBlockJavaScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safari_block_java_script: Option<bool>,
    #[serde(rename = "safariBlockPopups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safari_block_popups: Option<bool>,
    #[serde(rename = "safariBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safari_blocked: Option<bool>,
    #[serde(rename = "safariCookieSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safari_cookie_settings: Option<WebBrowserCookieSettings>,
    #[serde(rename = "safariManagedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safari_managed_domains: Option<Vec<String>>,
    #[serde(rename = "safariPasswordAutoFillDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safari_password_auto_fill_domains: Option<Vec<String>>,
    #[serde(rename = "safariRequireFraudWarning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safari_require_fraud_warning: Option<bool>,
    #[serde(rename = "screenCaptureBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_capture_blocked: Option<bool>,
    #[serde(rename = "siriBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub siri_blocked: Option<bool>,
    #[serde(rename = "siriBlockedWhenLocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub siri_blocked_when_locked: Option<bool>,
    #[serde(rename = "siriBlockUserGeneratedContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub siri_block_user_generated_content: Option<bool>,
    #[serde(rename = "siriRequireProfanityFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub siri_require_profanity_filter: Option<bool>,
    #[serde(rename = "spotlightBlockInternetResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotlight_block_internet_results: Option<bool>,
    #[serde(rename = "voiceDialingBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_dialing_blocked: Option<bool>,
    #[serde(rename = "wallpaperBlockModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallpaper_block_modification: Option<bool>,
    #[serde(rename = "wiFiConnectOnlyToConfiguredNetworks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi_connect_only_to_configured_networks: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosUpdateConfiguration {
    #[serde(rename = "activeHoursStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_hours_start: Option<String>,
    #[serde(rename = "activeHoursEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_hours_end: Option<String>,
    #[serde(rename = "scheduledInstallDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_install_days: Option<Vec<DayOfWeek>>,
    #[serde(rename = "utcTimeOffsetInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_time_offset_in_minutes: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MacOSCustomConfiguration {
    #[serde(rename = "payloadName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_name: Option<String>,
    #[serde(rename = "payloadFileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_file_name: Option<String>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<u8>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MacOSGeneralDeviceConfiguration {
    #[serde(rename = "compliantAppsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_apps_list: Option<Vec<AppListItem>>,
    #[serde(rename = "compliantAppListType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_app_list_type: Option<AppListType>,
    #[serde(rename = "emailInDomainSuffixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_in_domain_suffixes: Option<Vec<String>>,
    #[serde(rename = "passwordBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_simple: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_screen_timeout: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AppleDeviceFeaturesConfigurationBase(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl AppleDeviceFeaturesConfigurationBase {
    pub fn new(value: Option<serde_json::Value>) -> AppleDeviceFeaturesConfigurationBase {
        AppleDeviceFeaturesConfigurationBase(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosDeviceFeaturesConfiguration {
    #[serde(rename = "assetTagTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_tag_template: Option<String>,
    #[serde(rename = "lockScreenFootnote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_footnote: Option<String>,
    #[serde(rename = "homeScreenDockIcons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_screen_dock_icons: Option<Vec<IosHomeScreenItem>>,
    #[serde(rename = "homeScreenPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_screen_pages: Option<Vec<IosHomeScreenPage>>,
    #[serde(rename = "notificationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<Vec<IosNotificationSettings>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MacOSDeviceFeaturesConfiguration(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl MacOSDeviceFeaturesConfiguration {
    pub fn new(value: Option<serde_json::Value>) -> MacOSDeviceFeaturesConfiguration {
        MacOSDeviceFeaturesConfiguration(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10EndpointProtectionConfiguration {
    #[serde(rename = "firewallBlockStatefulFTP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_block_stateful_f_t_p: Option<bool>,
    #[serde(rename = "firewallIdleTimeoutForSecurityAssociationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_idle_timeout_for_security_association_in_seconds: Option<i32>,
    #[serde(rename = "firewallPreSharedKeyEncodingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_pre_shared_key_encoding_method: Option<FirewallPreSharedKeyEncodingMethodType>,
    #[serde(rename = "firewallIPSecExemptionsAllowNeighborDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_i_p_sec_exemptions_allow_neighbor_discovery: Option<bool>,
    #[serde(rename = "firewallIPSecExemptionsAllowICMP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_i_p_sec_exemptions_allow_i_c_m_p: Option<bool>,
    #[serde(rename = "firewallIPSecExemptionsAllowRouterDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_i_p_sec_exemptions_allow_router_discovery: Option<bool>,
    #[serde(rename = "firewallIPSecExemptionsAllowDHCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_i_p_sec_exemptions_allow_d_h_c_p: Option<bool>,
    #[serde(rename = "firewallCertificateRevocationListCheckMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_certificate_revocation_list_check_method:
        Option<FirewallCertificateRevocationListCheckMethodType>,
    #[serde(rename = "firewallMergeKeyingModuleSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_merge_keying_module_settings: Option<bool>,
    #[serde(rename = "firewallPacketQueueingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_packet_queueing_method: Option<FirewallPacketQueueingMethodType>,
    #[serde(rename = "firewallProfileDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_profile_domain: Option<WindowsFirewallNetworkProfile>,
    #[serde(rename = "firewallProfilePublic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_profile_public: Option<WindowsFirewallNetworkProfile>,
    #[serde(rename = "firewallProfilePrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_profile_private: Option<WindowsFirewallNetworkProfile>,
    #[serde(rename = "defenderAttackSurfaceReductionExcludedPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_attack_surface_reduction_excluded_paths: Option<Vec<String>>,
    #[serde(rename = "defenderGuardedFoldersAllowedAppPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_guarded_folders_allowed_app_paths: Option<Vec<String>>,
    #[serde(rename = "defenderAdditionalGuardedFolders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_additional_guarded_folders: Option<Vec<String>>,
    #[serde(rename = "defenderExploitProtectionXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_exploit_protection_xml: Option<Vec<u8>>,
    #[serde(rename = "defenderExploitProtectionXmlFileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_exploit_protection_xml_file_name: Option<String>,
    #[serde(rename = "defenderSecurityCenterBlockExploitProtectionOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_security_center_block_exploit_protection_override: Option<bool>,
    #[serde(rename = "appLockerApplicationControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_locker_application_control: Option<AppLockerApplicationControlType>,
    #[serde(rename = "smartScreenEnableInShell")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_screen_enable_in_shell: Option<bool>,
    #[serde(rename = "smartScreenBlockOverrideForFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_screen_block_override_for_files: Option<bool>,
    #[serde(rename = "applicationGuardEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_enabled: Option<bool>,
    #[serde(rename = "applicationGuardBlockFileTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_block_file_transfer: Option<ApplicationGuardBlockFileTransferType>,
    #[serde(rename = "applicationGuardBlockNonEnterpriseContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_block_non_enterprise_content: Option<bool>,
    #[serde(rename = "applicationGuardAllowPersistence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_allow_persistence: Option<bool>,
    #[serde(rename = "applicationGuardForceAuditing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_force_auditing: Option<bool>,
    #[serde(rename = "applicationGuardBlockClipboardSharing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_block_clipboard_sharing:
        Option<ApplicationGuardBlockClipboardSharingType>,
    #[serde(rename = "applicationGuardAllowPrintToPDF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_allow_print_to_p_d_f: Option<bool>,
    #[serde(rename = "applicationGuardAllowPrintToXPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_allow_print_to_x_p_s: Option<bool>,
    #[serde(rename = "applicationGuardAllowPrintToLocalPrinters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_allow_print_to_local_printers: Option<bool>,
    #[serde(rename = "applicationGuardAllowPrintToNetworkPrinters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_guard_allow_print_to_network_printers: Option<bool>,
    #[serde(rename = "bitLockerDisableWarningForOtherDiskEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_locker_disable_warning_for_other_disk_encryption: Option<bool>,
    #[serde(rename = "bitLockerEnableStorageCardEncryptionOnMobile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_locker_enable_storage_card_encryption_on_mobile: Option<bool>,
    #[serde(rename = "bitLockerEncryptDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_locker_encrypt_device: Option<bool>,
    #[serde(rename = "bitLockerRemovableDrivePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_locker_removable_drive_policy: Option<BitLockerRemovableDrivePolicy>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10GeneralConfiguration {
    #[serde(rename = "enterpriseCloudPrintDiscoveryEndPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_cloud_print_discovery_end_point: Option<String>,
    #[serde(rename = "enterpriseCloudPrintOAuthAuthority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_cloud_print_o_auth_authority: Option<String>,
    #[serde(rename = "enterpriseCloudPrintOAuthClientIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_cloud_print_o_auth_client_identifier: Option<String>,
    #[serde(rename = "enterpriseCloudPrintResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_cloud_print_resource_identifier: Option<String>,
    #[serde(rename = "enterpriseCloudPrintDiscoveryMaxLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_cloud_print_discovery_max_limit: Option<i32>,
    #[serde(rename = "enterpriseCloudPrintMopriaDiscoveryResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_cloud_print_mopria_discovery_resource_identifier: Option<String>,
    #[serde(rename = "searchBlockDiacritics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_block_diacritics: Option<bool>,
    #[serde(rename = "searchDisableAutoLanguageDetection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_disable_auto_language_detection: Option<bool>,
    #[serde(rename = "searchDisableIndexingEncryptedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_disable_indexing_encrypted_items: Option<bool>,
    #[serde(rename = "searchEnableRemoteQueries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_enable_remote_queries: Option<bool>,
    #[serde(rename = "searchDisableIndexerBackoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_disable_indexer_backoff: Option<bool>,
    #[serde(rename = "searchDisableIndexingRemovableDrive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_disable_indexing_removable_drive: Option<bool>,
    #[serde(rename = "searchEnableAutomaticIndexSizeManangement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_enable_automatic_index_size_manangement: Option<bool>,
    #[serde(rename = "diagnosticsDataSubmissionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_data_submission_mode: Option<DiagnosticDataSubmissionMode>,
    #[serde(rename = "oneDriveDisableFileSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_drive_disable_file_sync: Option<bool>,
    #[serde(rename = "smartScreenEnableAppInstallControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_screen_enable_app_install_control: Option<bool>,
    #[serde(rename = "personalizationDesktopImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_desktop_image_url: Option<String>,
    #[serde(rename = "personalizationLockScreenImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_lock_screen_image_url: Option<String>,
    #[serde(rename = "bluetoothAllowedServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_allowed_services: Option<Vec<String>>,
    #[serde(rename = "bluetoothBlockAdvertising")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_block_advertising: Option<bool>,
    #[serde(rename = "bluetoothBlockDiscoverableMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_block_discoverable_mode: Option<bool>,
    #[serde(rename = "bluetoothBlockPrePairing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_block_pre_pairing: Option<bool>,
    #[serde(rename = "edgeBlockAutofill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_autofill: Option<bool>,
    #[serde(rename = "edgeBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_blocked: Option<bool>,
    #[serde(rename = "edgeCookiePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_cookie_policy: Option<EdgeCookiePolicy>,
    #[serde(rename = "edgeBlockDeveloperTools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_developer_tools: Option<bool>,
    #[serde(rename = "edgeBlockSendingDoNotTrackHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_sending_do_not_track_header: Option<bool>,
    #[serde(rename = "edgeBlockExtensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_extensions: Option<bool>,
    #[serde(rename = "edgeBlockInPrivateBrowsing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_in_private_browsing: Option<bool>,
    #[serde(rename = "edgeBlockJavaScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_java_script: Option<bool>,
    #[serde(rename = "edgeBlockPasswordManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_password_manager: Option<bool>,
    #[serde(rename = "edgeBlockAddressBarDropdown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_address_bar_dropdown: Option<bool>,
    #[serde(rename = "edgeBlockCompatibilityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_compatibility_list: Option<bool>,
    #[serde(rename = "edgeClearBrowsingDataOnExit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_clear_browsing_data_on_exit: Option<bool>,
    #[serde(rename = "edgeAllowStartPagesModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_allow_start_pages_modification: Option<bool>,
    #[serde(rename = "edgeDisableFirstRunPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_disable_first_run_page: Option<bool>,
    #[serde(rename = "edgeBlockLiveTileDataCollection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_live_tile_data_collection: Option<bool>,
    #[serde(rename = "edgeSyncFavoritesWithInternetExplorer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_sync_favorites_with_internet_explorer: Option<bool>,
    #[serde(rename = "cellularBlockDataWhenRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_data_when_roaming: Option<bool>,
    #[serde(rename = "cellularBlockVpn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_vpn: Option<bool>,
    #[serde(rename = "cellularBlockVpnWhenRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_vpn_when_roaming: Option<bool>,
    #[serde(rename = "defenderBlockEndUserAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_block_end_user_access: Option<bool>,
    #[serde(rename = "defenderDaysBeforeDeletingQuarantinedMalware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_days_before_deleting_quarantined_malware: Option<i32>,
    #[serde(rename = "defenderDetectedMalwareActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_detected_malware_actions: Option<DefenderDetectedMalwareActions>,
    #[serde(rename = "defenderSystemScanSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_system_scan_schedule: Option<WeeklySchedule>,
    #[serde(rename = "defenderFilesAndFoldersToExclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_files_and_folders_to_exclude: Option<Vec<String>>,
    #[serde(rename = "defenderFileExtensionsToExclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_file_extensions_to_exclude: Option<Vec<String>>,
    #[serde(rename = "defenderScanMaxCpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_max_cpu: Option<i32>,
    #[serde(rename = "defenderMonitorFileActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_monitor_file_activity: Option<DefenderMonitorFileActivity>,
    #[serde(rename = "defenderProcessesToExclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_processes_to_exclude: Option<Vec<String>>,
    #[serde(rename = "defenderPromptForSampleSubmission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_prompt_for_sample_submission: Option<DefenderPromptForSampleSubmission>,
    #[serde(rename = "defenderRequireBehaviorMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_require_behavior_monitoring: Option<bool>,
    #[serde(rename = "defenderRequireCloudProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_require_cloud_protection: Option<bool>,
    #[serde(rename = "defenderRequireNetworkInspectionSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_require_network_inspection_system: Option<bool>,
    #[serde(rename = "defenderRequireRealTimeMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_require_real_time_monitoring: Option<bool>,
    #[serde(rename = "defenderScanArchiveFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_archive_files: Option<bool>,
    #[serde(rename = "defenderScanDownloads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_downloads: Option<bool>,
    #[serde(rename = "defenderScanNetworkFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_network_files: Option<bool>,
    #[serde(rename = "defenderScanIncomingMail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_incoming_mail: Option<bool>,
    #[serde(rename = "defenderScanMappedNetworkDrivesDuringFullScan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_mapped_network_drives_during_full_scan: Option<bool>,
    #[serde(rename = "defenderScanRemovableDrivesDuringFullScan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_removable_drives_during_full_scan: Option<bool>,
    #[serde(rename = "defenderScanScriptsLoadedInInternetExplorer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_scripts_loaded_in_internet_explorer: Option<bool>,
    #[serde(rename = "defenderSignatureUpdateIntervalInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_signature_update_interval_in_hours: Option<i32>,
    #[serde(rename = "defenderScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scan_type: Option<DefenderScanType>,
    #[serde(rename = "defenderScheduledScanTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scheduled_scan_time: Option<String>,
    #[serde(rename = "defenderScheduledQuickScanTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_scheduled_quick_scan_time: Option<String>,
    #[serde(rename = "defenderCloudBlockLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defender_cloud_block_level: Option<DefenderCloudBlockLevelType>,
    #[serde(rename = "lockScreenAllowTimeoutConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_allow_timeout_configuration: Option<bool>,
    #[serde(rename = "lockScreenBlockActionCenterNotifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_block_action_center_notifications: Option<bool>,
    #[serde(rename = "lockScreenBlockCortana")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_block_cortana: Option<bool>,
    #[serde(rename = "lockScreenBlockToastNotifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_block_toast_notifications: Option<bool>,
    #[serde(rename = "lockScreenTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_screen_timeout_in_seconds: Option<i32>,
    #[serde(rename = "passwordBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_simple: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_screen_timeout: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "passwordRequireWhenResumeFromIdleState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_require_when_resume_from_idle_state: Option<bool>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_sign_in_failure_count_before_factory_reset: Option<i32>,
    #[serde(rename = "privacyAdvertisingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_advertising_id: Option<StateManagementSetting>,
    #[serde(rename = "privacyAutoAcceptPairingAndConsentPrompts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_auto_accept_pairing_and_consent_prompts: Option<bool>,
    #[serde(rename = "privacyBlockInputPersonalization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_block_input_personalization: Option<bool>,
    #[serde(rename = "startBlockUnpinningAppsFromTaskbar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_block_unpinning_apps_from_taskbar: Option<bool>,
    #[serde(rename = "startMenuAppListVisibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_app_list_visibility: Option<WindowsStartMenuAppListVisibilityType>,
    #[serde(rename = "startMenuHideChangeAccountSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_change_account_settings: Option<bool>,
    #[serde(rename = "startMenuHideFrequentlyUsedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_frequently_used_apps: Option<bool>,
    #[serde(rename = "startMenuHideHibernate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_hibernate: Option<bool>,
    #[serde(rename = "startMenuHideLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_lock: Option<bool>,
    #[serde(rename = "startMenuHidePowerButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_power_button: Option<bool>,
    #[serde(rename = "startMenuHideRecentJumpLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_recent_jump_lists: Option<bool>,
    #[serde(rename = "startMenuHideRecentlyAddedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_recently_added_apps: Option<bool>,
    #[serde(rename = "startMenuHideRestartOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_restart_options: Option<bool>,
    #[serde(rename = "startMenuHideShutDown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_shut_down: Option<bool>,
    #[serde(rename = "startMenuHideSignOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_sign_out: Option<bool>,
    #[serde(rename = "startMenuHideSleep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_sleep: Option<bool>,
    #[serde(rename = "startMenuHideSwitchAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_switch_account: Option<bool>,
    #[serde(rename = "startMenuHideUserTile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_hide_user_tile: Option<bool>,
    #[serde(rename = "startMenuLayoutEdgeAssetsXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_layout_edge_assets_xml: Option<Vec<u8>>,
    #[serde(rename = "startMenuLayoutXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_layout_xml: Option<Vec<u8>>,
    #[serde(rename = "startMenuMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_mode: Option<WindowsStartMenuModeType>,
    #[serde(rename = "startMenuPinnedFolderDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_documents: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderDownloads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_downloads: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderFileExplorer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_file_explorer: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderHomeGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_home_group: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderMusic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_music: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_network: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderPersonalFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_personal_folder: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderPictures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_pictures: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_settings: Option<VisibilitySetting>,
    #[serde(rename = "startMenuPinnedFolderVideos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_menu_pinned_folder_videos: Option<VisibilitySetting>,
    #[serde(rename = "settingsBlockSettingsApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_settings_app: Option<bool>,
    #[serde(rename = "settingsBlockSystemPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_system_page: Option<bool>,
    #[serde(rename = "settingsBlockDevicesPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_devices_page: Option<bool>,
    #[serde(rename = "settingsBlockNetworkInternetPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_network_internet_page: Option<bool>,
    #[serde(rename = "settingsBlockPersonalizationPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_personalization_page: Option<bool>,
    #[serde(rename = "settingsBlockAccountsPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_accounts_page: Option<bool>,
    #[serde(rename = "settingsBlockTimeLanguagePage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_time_language_page: Option<bool>,
    #[serde(rename = "settingsBlockEaseOfAccessPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_ease_of_access_page: Option<bool>,
    #[serde(rename = "settingsBlockPrivacyPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_privacy_page: Option<bool>,
    #[serde(rename = "settingsBlockUpdateSecurityPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_update_security_page: Option<bool>,
    #[serde(rename = "settingsBlockAppsPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_apps_page: Option<bool>,
    #[serde(rename = "settingsBlockGamingPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_gaming_page: Option<bool>,
    #[serde(rename = "windowsSpotlightBlockConsumerSpecificFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_spotlight_block_consumer_specific_features: Option<bool>,
    #[serde(rename = "windowsSpotlightBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_spotlight_blocked: Option<bool>,
    #[serde(rename = "windowsSpotlightBlockOnActionCenter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_spotlight_block_on_action_center: Option<bool>,
    #[serde(rename = "windowsSpotlightBlockTailoredExperiences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_spotlight_block_tailored_experiences: Option<bool>,
    #[serde(rename = "windowsSpotlightBlockThirdPartyNotifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_spotlight_block_third_party_notifications: Option<bool>,
    #[serde(rename = "windowsSpotlightBlockWelcomeExperience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_spotlight_block_welcome_experience: Option<bool>,
    #[serde(rename = "windowsSpotlightBlockWindowsTips")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_spotlight_block_windows_tips: Option<bool>,
    #[serde(rename = "windowsSpotlightConfigureOnLockScreen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_spotlight_configure_on_lock_screen: Option<WindowsSpotlightEnablementSettings>,
    #[serde(rename = "networkProxyApplySettingsDeviceWide")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy_apply_settings_device_wide: Option<bool>,
    #[serde(rename = "networkProxyDisableAutoDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy_disable_auto_detect: Option<bool>,
    #[serde(rename = "networkProxyAutomaticConfigurationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy_automatic_configuration_url: Option<String>,
    #[serde(rename = "networkProxyServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy_server: Option<Windows10NetworkProxyServer>,
    #[serde(rename = "accountsBlockAddingNonMicrosoftAccountEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_block_adding_non_microsoft_account_email: Option<bool>,
    #[serde(rename = "antiTheftModeBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_theft_mode_blocked: Option<bool>,
    #[serde(rename = "bluetoothBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_blocked: Option<bool>,
    #[serde(rename = "cameraBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_blocked: Option<bool>,
    #[serde(rename = "connectedDevicesServiceBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_devices_service_blocked: Option<bool>,
    #[serde(rename = "certificatesBlockManualRootCertificateInstallation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates_block_manual_root_certificate_installation: Option<bool>,
    #[serde(rename = "copyPasteBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_paste_blocked: Option<bool>,
    #[serde(rename = "cortanaBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cortana_blocked: Option<bool>,
    #[serde(rename = "deviceManagementBlockFactoryResetOnMobile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_management_block_factory_reset_on_mobile: Option<bool>,
    #[serde(rename = "deviceManagementBlockManualUnenroll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_management_block_manual_unenroll: Option<bool>,
    #[serde(rename = "safeSearchFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_search_filter: Option<SafeSearchFilterType>,
    #[serde(rename = "edgeBlockPopups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_popups: Option<bool>,
    #[serde(rename = "edgeBlockSearchSuggestions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_search_suggestions: Option<bool>,
    #[serde(rename = "edgeBlockSendingIntranetTrafficToInternetExplorer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_sending_intranet_traffic_to_internet_explorer: Option<bool>,
    #[serde(rename = "edgeSendIntranetTrafficToInternetExplorer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_send_intranet_traffic_to_internet_explorer: Option<bool>,
    #[serde(rename = "edgeRequireSmartScreen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_require_smart_screen: Option<bool>,
    #[serde(rename = "edgeEnterpriseModeSiteListLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_enterprise_mode_site_list_location: Option<String>,
    #[serde(rename = "edgeFirstRunUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_first_run_url: Option<String>,
    #[serde(rename = "edgeSearchEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_search_engine: Option<EdgeSearchEngineBase>,
    #[serde(rename = "edgeHomepageUrls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_homepage_urls: Option<Vec<String>>,
    #[serde(rename = "edgeBlockAccessToAboutFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_block_access_to_about_flags: Option<bool>,
    #[serde(rename = "smartScreenBlockPromptOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_screen_block_prompt_override: Option<bool>,
    #[serde(rename = "smartScreenBlockPromptOverrideForFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_screen_block_prompt_override_for_files: Option<bool>,
    #[serde(rename = "webRtcBlockLocalhostIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_rtc_block_localhost_ip_address: Option<bool>,
    #[serde(rename = "internetSharingBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_sharing_blocked: Option<bool>,
    #[serde(rename = "settingsBlockAddProvisioningPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_add_provisioning_package: Option<bool>,
    #[serde(rename = "settingsBlockRemoveProvisioningPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_remove_provisioning_package: Option<bool>,
    #[serde(rename = "settingsBlockChangeSystemTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_change_system_time: Option<bool>,
    #[serde(rename = "settingsBlockEditDeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_edit_device_name: Option<bool>,
    #[serde(rename = "settingsBlockChangeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_change_region: Option<bool>,
    #[serde(rename = "settingsBlockChangeLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_change_language: Option<bool>,
    #[serde(rename = "settingsBlockChangePowerSleep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_change_power_sleep: Option<bool>,
    #[serde(rename = "locationServicesBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_services_blocked: Option<bool>,
    #[serde(rename = "microsoftAccountBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_account_blocked: Option<bool>,
    #[serde(rename = "microsoftAccountBlockSettingsSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_account_block_settings_sync: Option<bool>,
    #[serde(rename = "nfcBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfc_blocked: Option<bool>,
    #[serde(rename = "resetProtectionModeBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_protection_mode_blocked: Option<bool>,
    #[serde(rename = "screenCaptureBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_capture_blocked: Option<bool>,
    #[serde(rename = "storageBlockRemovableStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_block_removable_storage: Option<bool>,
    #[serde(rename = "storageRequireMobileDeviceEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_mobile_device_encryption: Option<bool>,
    #[serde(rename = "usbBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usb_blocked: Option<bool>,
    #[serde(rename = "voiceRecordingBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_recording_blocked: Option<bool>,
    #[serde(rename = "wiFiBlockAutomaticConnectHotspots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi_block_automatic_connect_hotspots: Option<bool>,
    #[serde(rename = "wiFiBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi_blocked: Option<bool>,
    #[serde(rename = "wiFiBlockManualConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi_block_manual_configuration: Option<bool>,
    #[serde(rename = "wiFiScanInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi_scan_interval: Option<i32>,
    #[serde(rename = "wirelessDisplayBlockProjectionToThisDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wireless_display_block_projection_to_this_device: Option<bool>,
    #[serde(rename = "wirelessDisplayBlockUserInputFromReceiver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wireless_display_block_user_input_from_receiver: Option<bool>,
    #[serde(rename = "wirelessDisplayRequirePinForPairing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wireless_display_require_pin_for_pairing: Option<bool>,
    #[serde(rename = "windowsStoreBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_store_blocked: Option<bool>,
    #[serde(rename = "appsAllowTrustedAppsSideloading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_allow_trusted_apps_sideloading: Option<StateManagementSetting>,
    #[serde(rename = "windowsStoreBlockAutoUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_store_block_auto_update: Option<bool>,
    #[serde(rename = "developerUnlockSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_unlock_setting: Option<StateManagementSetting>,
    #[serde(rename = "sharedUserAppDataAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_user_app_data_allowed: Option<bool>,
    #[serde(rename = "appsBlockWindowsStoreOriginatedApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_block_windows_store_originated_apps: Option<bool>,
    #[serde(rename = "windowsStoreEnablePrivateStoreOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_store_enable_private_store_only: Option<bool>,
    #[serde(rename = "storageRestrictAppDataToSystemVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_restrict_app_data_to_system_volume: Option<bool>,
    #[serde(rename = "storageRestrictAppInstallToSystemVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_restrict_app_install_to_system_volume: Option<bool>,
    #[serde(rename = "gameDvrBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_dvr_blocked: Option<bool>,
    #[serde(rename = "experienceBlockDeviceDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_block_device_discovery: Option<bool>,
    #[serde(rename = "experienceBlockErrorDialogWhenNoSIM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_block_error_dialog_when_no_s_i_m: Option<bool>,
    #[serde(rename = "experienceBlockTaskSwitcher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_block_task_switcher: Option<bool>,
    #[serde(rename = "logonBlockFastUserSwitching")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_block_fast_user_switching: Option<bool>,
    #[serde(rename = "tenantLockdownRequireNetworkDuringOutOfBoxExperience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_lockdown_require_network_during_out_of_box_experience: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsDefenderAdvancedThreatProtectionConfiguration {
    #[serde(rename = "allowSampleSharing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sample_sharing: Option<bool>,
    #[serde(rename = "enableExpeditedTelemetryReporting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_expedited_telemetry_reporting: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EditionUpgradeConfiguration {
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<EditionUpgradeLicenseType>,
    #[serde(rename = "targetEdition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_edition: Option<Windows10EditionType>,
    #[serde(rename = "license")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(rename = "productKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_key: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10CustomConfiguration {
    #[serde(rename = "omaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oma_settings: Option<Vec<OmaSetting>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10EnterpriseModernAppManagementConfiguration {
    #[serde(rename = "uninstallBuiltInApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninstall_built_in_apps: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SharedPCConfiguration {
    #[serde(rename = "accountManagerPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_manager_policy: Option<SharedPCAccountManagerPolicy>,
    #[serde(rename = "allowedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_accounts: Option<SharedPCAllowedAccountType>,
    #[serde(rename = "allowLocalStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_local_storage: Option<bool>,
    #[serde(rename = "disableAccountManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_account_manager: Option<bool>,
    #[serde(rename = "disableEduPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edu_policies: Option<bool>,
    #[serde(rename = "disablePowerPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_power_policies: Option<bool>,
    #[serde(rename = "disableSignInOnResume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sign_in_on_resume: Option<bool>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "idleTimeBeforeSleepInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_time_before_sleep_in_seconds: Option<i32>,
    #[serde(rename = "kioskAppDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_app_display_name: Option<String>,
    #[serde(rename = "kioskAppUserModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk_app_user_model_id: Option<String>,
    #[serde(rename = "maintenanceStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10SecureAssessmentConfiguration {
    #[serde(rename = "launchUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_uri: Option<String>,
    #[serde(rename = "configurationAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_account: Option<String>,
    #[serde(rename = "allowPrinting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_printing: Option<bool>,
    #[serde(rename = "allowScreenCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_screen_capture: Option<bool>,
    #[serde(rename = "allowTextSuggestion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_text_suggestion: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsPhone81CustomConfiguration {
    #[serde(rename = "omaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oma_settings: Option<Vec<OmaSetting>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsUpdateForBusinessConfiguration {
    #[serde(rename = "deliveryOptimizationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_optimization_mode: Option<WindowsDeliveryOptimizationMode>,
    #[serde(rename = "prereleaseFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerelease_features: Option<PrereleaseFeatures>,
    #[serde(rename = "automaticUpdateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_update_mode: Option<AutomaticUpdateMode>,
    #[serde(rename = "microsoftUpdateServiceAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_update_service_allowed: Option<bool>,
    #[serde(rename = "driversExcluded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_excluded: Option<bool>,
    #[serde(rename = "installationSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_schedule: Option<WindowsUpdateInstallScheduleType>,
    #[serde(rename = "qualityUpdatesDeferralPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_updates_deferral_period_in_days: Option<i32>,
    #[serde(rename = "featureUpdatesDeferralPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_updates_deferral_period_in_days: Option<i32>,
    #[serde(rename = "qualityUpdatesPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_updates_paused: Option<bool>,
    #[serde(rename = "featureUpdatesPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_updates_paused: Option<bool>,
    #[serde(rename = "qualityUpdatesPauseExpiryDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_updates_pause_expiry_date_time: Option<String>,
    #[serde(rename = "featureUpdatesPauseExpiryDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_updates_pause_expiry_date_time: Option<String>,
    #[serde(rename = "businessReadyUpdatesOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_ready_updates_only: Option<WindowsUpdateType>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows81GeneralConfiguration {
    #[serde(rename = "accountsBlockAddingNonMicrosoftAccountEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_block_adding_non_microsoft_account_email: Option<bool>,
    #[serde(rename = "applyOnlyToWindows81")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_only_to_windows81: Option<bool>,
    #[serde(rename = "browserBlockAutofill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_block_autofill: Option<bool>,
    #[serde(rename = "browserBlockAutomaticDetectionOfIntranetSites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_block_automatic_detection_of_intranet_sites: Option<bool>,
    #[serde(rename = "browserBlockEnterpriseModeAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_block_enterprise_mode_access: Option<bool>,
    #[serde(rename = "browserBlockJavaScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_block_java_script: Option<bool>,
    #[serde(rename = "browserBlockPlugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_block_plugins: Option<bool>,
    #[serde(rename = "browserBlockPopups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_block_popups: Option<bool>,
    #[serde(rename = "browserBlockSendingDoNotTrackHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_block_sending_do_not_track_header: Option<bool>,
    #[serde(rename = "browserBlockSingleWordEntryOnIntranetSites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_block_single_word_entry_on_intranet_sites: Option<bool>,
    #[serde(rename = "browserRequireSmartScreen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_require_smart_screen: Option<bool>,
    #[serde(rename = "browserEnterpriseModeSiteListLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_enterprise_mode_site_list_location: Option<String>,
    #[serde(rename = "browserInternetSecurityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_internet_security_level: Option<InternetSiteSecurityLevel>,
    #[serde(rename = "browserIntranetSecurityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_intranet_security_level: Option<SiteSecurityLevel>,
    #[serde(rename = "browserLoggingReportLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_logging_report_location: Option<String>,
    #[serde(rename = "browserRequireHighSecurityForRestrictedSites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_require_high_security_for_restricted_sites: Option<bool>,
    #[serde(rename = "browserRequireFirewall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_require_firewall: Option<bool>,
    #[serde(rename = "browserRequireFraudWarning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_require_fraud_warning: Option<bool>,
    #[serde(rename = "browserTrustedSitesSecurityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_trusted_sites_security_level: Option<SiteSecurityLevel>,
    #[serde(rename = "cellularBlockDataRoaming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_data_roaming: Option<bool>,
    #[serde(rename = "diagnosticsBlockDataSubmission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_block_data_submission: Option<bool>,
    #[serde(rename = "passwordBlockPicturePasswordAndPin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_picture_password_and_pin: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_screen_timeout: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_sign_in_failure_count_before_factory_reset: Option<i32>,
    #[serde(rename = "storageRequireDeviceEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_device_encryption: Option<bool>,
    #[serde(rename = "updatesRequireAutomaticUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates_require_automatic_updates: Option<bool>,
    #[serde(rename = "userAccountControlSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_account_control_settings: Option<WindowsUserAccountControlSettings>,
    #[serde(rename = "workFoldersUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_folders_url: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsPhone81GeneralConfiguration {
    #[serde(rename = "applyOnlyToWindowsPhone81")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_only_to_windows_phone81: Option<bool>,
    #[serde(rename = "appsBlockCopyPaste")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_block_copy_paste: Option<bool>,
    #[serde(rename = "bluetoothBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth_blocked: Option<bool>,
    #[serde(rename = "cameraBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_blocked: Option<bool>,
    #[serde(rename = "cellularBlockWifiTethering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_block_wifi_tethering: Option<bool>,
    #[serde(rename = "compliantAppsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_apps_list: Option<Vec<AppListItem>>,
    #[serde(rename = "compliantAppListType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_app_list_type: Option<AppListType>,
    #[serde(rename = "diagnosticDataBlockSubmission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_data_block_submission: Option<bool>,
    #[serde(rename = "emailBlockAddingAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_block_adding_accounts: Option<bool>,
    #[serde(rename = "locationServicesBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_services_blocked: Option<bool>,
    #[serde(rename = "microsoftAccountBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_account_blocked: Option<bool>,
    #[serde(rename = "nfcBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfc_blocked: Option<bool>,
    #[serde(rename = "passwordBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_simple: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeScreenTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_screen_timeout: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordSignInFailureCountBeforeFactoryReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_sign_in_failure_count_before_factory_reset: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "screenCaptureBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_capture_blocked: Option<bool>,
    #[serde(rename = "storageBlockRemovableStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_block_removable_storage: Option<bool>,
    #[serde(rename = "storageRequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_encryption: Option<bool>,
    #[serde(rename = "webBrowserBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_browser_blocked: Option<bool>,
    #[serde(rename = "wifiBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_blocked: Option<bool>,
    #[serde(rename = "wifiBlockAutomaticConnectHotspots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_block_automatic_connect_hotspots: Option<bool>,
    #[serde(rename = "wifiBlockHotspotReporting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_block_hotspot_reporting: Option<bool>,
    #[serde(rename = "windowsStoreBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_store_blocked: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10TeamGeneralConfiguration {
    #[serde(rename = "azureOperationalInsightsBlockTelemetry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_operational_insights_block_telemetry: Option<bool>,
    #[serde(rename = "azureOperationalInsightsWorkspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_operational_insights_workspace_id: Option<String>,
    #[serde(rename = "azureOperationalInsightsWorkspaceKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_operational_insights_workspace_key: Option<String>,
    #[serde(rename = "connectAppBlockAutoLaunch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_app_block_auto_launch: Option<bool>,
    #[serde(rename = "maintenanceWindowBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_blocked: Option<bool>,
    #[serde(rename = "maintenanceWindowDurationInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_duration_in_hours: Option<i32>,
    #[serde(rename = "maintenanceWindowStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_start_time: Option<String>,
    #[serde(rename = "miracastChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miracast_channel: Option<MiracastChannel>,
    #[serde(rename = "miracastBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miracast_blocked: Option<bool>,
    #[serde(rename = "miracastRequirePin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miracast_require_pin: Option<bool>,
    #[serde(rename = "settingsBlockMyMeetingsAndFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_my_meetings_and_files: Option<bool>,
    #[serde(rename = "settingsBlockSessionResume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_session_resume: Option<bool>,
    #[serde(rename = "settingsBlockSigninSuggestions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_block_signin_suggestions: Option<bool>,
    #[serde(rename = "settingsDefaultVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_default_volume: Option<i32>,
    #[serde(rename = "settingsScreenTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_screen_timeout_in_minutes: Option<i32>,
    #[serde(rename = "settingsSessionTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_session_timeout_in_minutes: Option<i32>,
    #[serde(rename = "settingsSleepTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_sleep_timeout_in_minutes: Option<i32>,
    #[serde(rename = "welcomeScreenBlockAutomaticWakeUp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_screen_block_automatic_wake_up: Option<bool>,
    #[serde(rename = "welcomeScreenBackgroundImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_screen_background_image_url: Option<String>,
    #[serde(rename = "welcomeScreenMeetingInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_screen_meeting_information: Option<WelcomeScreenMeetingInformation>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidCompliancePolicy {
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<AndroidRequiredPasswordType>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "securityPreventInstallAppsFromUnknownSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_prevent_install_apps_from_unknown_sources: Option<bool>,
    #[serde(rename = "securityDisableUsbDebugging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_disable_usb_debugging: Option<bool>,
    #[serde(rename = "securityRequireVerifyApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_verify_apps: Option<bool>,
    #[serde(rename = "deviceThreatProtectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_threat_protection_enabled: Option<bool>,
    #[serde(rename = "deviceThreatProtectionRequiredSecurityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_threat_protection_required_security_level: Option<DeviceThreatProtectionLevel>,
    #[serde(rename = "securityBlockJailbrokenDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_block_jailbroken_devices: Option<bool>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
    #[serde(rename = "minAndroidSecurityPatchLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_android_security_patch_level: Option<String>,
    #[serde(rename = "storageRequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_encryption: Option<bool>,
    #[serde(rename = "securityRequireSafetyNetAttestationBasicIntegrity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_safety_net_attestation_basic_integrity: Option<bool>,
    #[serde(rename = "securityRequireSafetyNetAttestationCertifiedDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_safety_net_attestation_certified_device: Option<bool>,
    #[serde(rename = "securityRequireGooglePlayServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_google_play_services: Option<bool>,
    #[serde(rename = "securityRequireUpToDateSecurityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_up_to_date_security_providers: Option<bool>,
    #[serde(rename = "securityRequireCompanyPortalAppIntegrity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_company_portal_app_integrity: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidWorkProfileCompliancePolicy {
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<AndroidRequiredPasswordType>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "securityPreventInstallAppsFromUnknownSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_prevent_install_apps_from_unknown_sources: Option<bool>,
    #[serde(rename = "securityDisableUsbDebugging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_disable_usb_debugging: Option<bool>,
    #[serde(rename = "securityRequireVerifyApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_verify_apps: Option<bool>,
    #[serde(rename = "deviceThreatProtectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_threat_protection_enabled: Option<bool>,
    #[serde(rename = "deviceThreatProtectionRequiredSecurityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_threat_protection_required_security_level: Option<DeviceThreatProtectionLevel>,
    #[serde(rename = "securityBlockJailbrokenDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_block_jailbroken_devices: Option<bool>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
    #[serde(rename = "minAndroidSecurityPatchLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_android_security_patch_level: Option<String>,
    #[serde(rename = "storageRequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_encryption: Option<bool>,
    #[serde(rename = "securityRequireSafetyNetAttestationBasicIntegrity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_safety_net_attestation_basic_integrity: Option<bool>,
    #[serde(rename = "securityRequireSafetyNetAttestationCertifiedDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_safety_net_attestation_certified_device: Option<bool>,
    #[serde(rename = "securityRequireGooglePlayServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_google_play_services: Option<bool>,
    #[serde(rename = "securityRequireUpToDateSecurityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_up_to_date_security_providers: Option<bool>,
    #[serde(rename = "securityRequireCompanyPortalAppIntegrity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_require_company_portal_app_integrity: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosCompliancePolicy {
    #[serde(rename = "passcodeBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_block_simple: Option<bool>,
    #[serde(rename = "passcodeExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_expiration_days: Option<i32>,
    #[serde(rename = "passcodeMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_minimum_length: Option<i32>,
    #[serde(rename = "passcodeMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passcodePreviousPasscodeBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_previous_passcode_block_count: Option<i32>,
    #[serde(rename = "passcodeMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passcodeRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passcodeRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode_required: Option<bool>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
    #[serde(rename = "securityBlockJailbrokenDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_block_jailbroken_devices: Option<bool>,
    #[serde(rename = "deviceThreatProtectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_threat_protection_enabled: Option<bool>,
    #[serde(rename = "deviceThreatProtectionRequiredSecurityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_threat_protection_required_security_level: Option<DeviceThreatProtectionLevel>,
    #[serde(rename = "managedEmailProfileRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_email_profile_required: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MacOSCompliancePolicy {
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "passwordBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_simple: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
    #[serde(rename = "systemIntegrityProtectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_integrity_protection_enabled: Option<bool>,
    #[serde(rename = "deviceThreatProtectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_threat_protection_enabled: Option<bool>,
    #[serde(rename = "deviceThreatProtectionRequiredSecurityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_threat_protection_required_security_level: Option<DeviceThreatProtectionLevel>,
    #[serde(rename = "storageRequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_encryption: Option<bool>,
    #[serde(rename = "firewallEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_enabled: Option<bool>,
    #[serde(rename = "firewallBlockAllIncoming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_block_all_incoming: Option<bool>,
    #[serde(rename = "firewallEnableStealthMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_enable_stealth_mode: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10CompliancePolicy {
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "passwordBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_simple: Option<bool>,
    #[serde(rename = "passwordRequiredToUnlockFromIdle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_to_unlock_from_idle: Option<bool>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "requireHealthyDeviceReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_healthy_device_report: Option<bool>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
    #[serde(rename = "mobileOsMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_os_minimum_version: Option<String>,
    #[serde(rename = "mobileOsMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_os_maximum_version: Option<String>,
    #[serde(rename = "earlyLaunchAntiMalwareDriverEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_launch_anti_malware_driver_enabled: Option<bool>,
    #[serde(rename = "bitLockerEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_locker_enabled: Option<bool>,
    #[serde(rename = "secureBootEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<bool>,
    #[serde(rename = "codeIntegrityEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_integrity_enabled: Option<bool>,
    #[serde(rename = "storageRequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_encryption: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows10MobileCompliancePolicy {
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "passwordBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_simple: Option<bool>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passwordRequireToUnlockFromIdle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_require_to_unlock_from_idle: Option<bool>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
    #[serde(rename = "earlyLaunchAntiMalwareDriverEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_launch_anti_malware_driver_enabled: Option<bool>,
    #[serde(rename = "bitLockerEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_locker_enabled: Option<bool>,
    #[serde(rename = "secureBootEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<bool>,
    #[serde(rename = "codeIntegrityEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_integrity_enabled: Option<bool>,
    #[serde(rename = "storageRequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_encryption: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Windows81CompliancePolicy {
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "passwordBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_simple: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
    #[serde(rename = "storageRequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_encryption: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsPhone81CompliancePolicy {
    #[serde(rename = "passwordBlockSimple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_block_simple: Option<bool>,
    #[serde(rename = "passwordExpirationDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expiration_days: Option<i32>,
    #[serde(rename = "passwordMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_length: Option<i32>,
    #[serde(rename = "passwordMinutesOfInactivityBeforeLock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minutes_of_inactivity_before_lock: Option<i32>,
    #[serde(rename = "passwordMinimumCharacterSetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_minimum_character_set_count: Option<i32>,
    #[serde(rename = "passwordRequiredType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required_type: Option<RequiredPasswordType>,
    #[serde(rename = "passwordPreviousPasswordBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_previous_password_block_count: Option<i32>,
    #[serde(rename = "passwordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "osMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_minimum_version: Option<String>,
    #[serde(rename = "osMaximumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_maximum_version: Option<String>,
    #[serde(rename = "storageRequireEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_require_encryption: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceComplianceSettingState {
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<String>,
    #[serde(rename = "settingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_name: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "deviceModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ComplianceStatus>,
    #[serde(rename = "complianceGracePeriodExpirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_grace_period_expiration_date_time: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentConfigurationAssignment {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeviceAndAppManagementAssignmentTarget>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentLimitConfiguration {
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentPlatformRestrictionsConfiguration {
    #[serde(rename = "iosRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_restriction: Option<DeviceEnrollmentPlatformRestriction>,
    #[serde(rename = "windowsRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_restriction: Option<DeviceEnrollmentPlatformRestriction>,
    #[serde(rename = "windowsMobileRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_mobile_restriction: Option<DeviceEnrollmentPlatformRestriction>,
    #[serde(rename = "androidRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_restriction: Option<DeviceEnrollmentPlatformRestriction>,
    #[serde(rename = "macOSRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_o_s_restriction: Option<DeviceEnrollmentPlatformRestriction>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceEnrollmentWindowsHelloForBusinessConfiguration {
    #[serde(rename = "pinMinimumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_minimum_length: Option<i32>,
    #[serde(rename = "pinMaximumLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_maximum_length: Option<i32>,
    #[serde(rename = "pinUppercaseCharactersUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_uppercase_characters_usage: Option<WindowsHelloForBusinessPinUsage>,
    #[serde(rename = "pinLowercaseCharactersUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_lowercase_characters_usage: Option<WindowsHelloForBusinessPinUsage>,
    #[serde(rename = "pinSpecialCharactersUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_special_characters_usage: Option<WindowsHelloForBusinessPinUsage>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Enablement>,
    #[serde(rename = "securityDeviceRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_device_required: Option<bool>,
    #[serde(rename = "unlockWithBiometricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_with_biometrics_enabled: Option<bool>,
    #[serde(rename = "remotePassportEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_passport_enabled: Option<bool>,
    #[serde(rename = "pinPreviousBlockCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_previous_block_count: Option<i32>,
    #[serde(rename = "pinExpirationInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_expiration_in_days: Option<i32>,
    #[serde(rename = "enhancedBiometricsState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_biometrics_state: Option<Enablement>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedMobileApp {
    #[serde(rename = "mobileAppIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_app_identifier: Option<MobileAppIdentifier>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TargetedManagedAppPolicyAssignment {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeviceAndAppManagementAssignmentTarget>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppOperation {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppPolicyDeploymentSummary {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "configurationDeployedUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_deployed_user_count: Option<i32>,
    #[serde(rename = "lastRefreshTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_time: Option<String>,
    #[serde(rename = "configurationDeploymentSummaryPerApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_deployment_summary_per_app:
        Option<Vec<ManagedAppPolicyDeploymentSummaryPerApp>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WindowsInformationProtectionAppLockerFile {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "fileHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_hash: Option<String>,
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<u8>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosManagedAppRegistration(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl IosManagedAppRegistration {
    pub fn new(value: Option<serde_json::Value>) -> IosManagedAppRegistration {
        IosManagedAppRegistration(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AndroidManagedAppRegistration(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl AndroidManagedAppRegistration {
    pub fn new(value: Option<serde_json::Value>) -> AndroidManagedAppRegistration {
        AndroidManagedAppRegistration(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedAppStatusRaw {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Json>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LocalizedNotificationMessage {
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "messageTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_template: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceAndAppManagementRoleDefinition(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl DeviceAndAppManagementRoleDefinition {
    pub fn new(value: Option<serde_json::Value>) -> DeviceAndAppManagementRoleDefinition {
        DeviceAndAppManagementRoleDefinition(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ManagedEBookAssignment {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DeviceAndAppManagementAssignmentTarget>,
    #[serde(rename = "installIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_intent: Option<InstallIntent>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EBookInstallSummary {
    #[serde(rename = "installedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_device_count: Option<i32>,
    #[serde(rename = "failedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_device_count: Option<i32>,
    #[serde(rename = "notInstalledDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_installed_device_count: Option<i32>,
    #[serde(rename = "installedUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_user_count: Option<i32>,
    #[serde(rename = "failedUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_user_count: Option<i32>,
    #[serde(rename = "notInstalledUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_installed_user_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceInstallState {
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "lastSyncDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_date_time: Option<String>,
    #[serde(rename = "installState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_state: Option<InstallState>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "osDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_description: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserInstallStateSummary {
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "installedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_device_count: Option<i32>,
    #[serde(rename = "failedDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_device_count: Option<i32>,
    #[serde(rename = "notInstalledDeviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_installed_device_count: Option<i32>,
    #[serde(rename = "deviceStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_states: Option<Vec<DeviceInstallState>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosVppEBookAssignment(
    #[serde(skip_serializing_if = "Option::is_none")] Option<serde_json::Value>,
);

#[cfg(feature = "option")]
impl IosVppEBookAssignment {
    pub fn new(value: Option<serde_json::Value>) -> IosVppEBookAssignment {
        IosVppEBookAssignment(value)
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.0.as_ref()
    }
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IosVppEBook {
    #[serde(rename = "vppTokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpp_token_id: Option<String>,
    #[serde(rename = "appleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_id: Option<String>,
    #[serde(rename = "vppOrganizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpp_organization_name: Option<String>,
    #[serde(rename = "genres")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "seller")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller: Option<String>,
    #[serde(rename = "totalLicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_license_count: Option<i32>,
    #[serde(rename = "usedLicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_license_count: Option<i32>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentTroubleshootingEvent {
    #[serde(rename = "managedDeviceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_identifier: Option<String>,
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "enrollmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_type: Option<DeviceEnrollmentType>,
    #[serde(rename = "failureCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_category: Option<DeviceEnrollmentFailureReason>,
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ActivityHistoryItem {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "activeDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_duration_seconds: Option<i32>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "lastActiveDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(rename = "startedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_date_time: Option<String>,
    #[serde(rename = "userTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_timezone: Option<String>,
    #[serde(rename = "activity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<UserActivity>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Security {
    #[serde(rename = "alerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<Alert>>,
    #[serde(rename = "secureScoreControlProfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_score_control_profiles: Option<Vec<SecureScoreControlProfile>>,
    #[serde(rename = "secureScores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_scores: Option<Vec<SecureScore>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(rename = "activityGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_group_name: Option<String>,
    #[serde(rename = "assignedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(rename = "azureSubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_subscription_id: Option<String>,
    #[serde(rename = "azureTenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_tenant_id: Option<String>,
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "closedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_date_time: Option<String>,
    #[serde(rename = "cloudAppStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_app_states: Option<Vec<CloudAppSecurityState>>,
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<String>>,
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "detectionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_ids: Option<Vec<String>>,
    #[serde(rename = "eventDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date_time: Option<String>,
    #[serde(rename = "feedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<AlertFeedback>,
    #[serde(rename = "fileStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_states: Option<Vec<FileSecurityState>>,
    #[serde(rename = "historyStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_states: Option<Vec<AlertHistoryState>>,
    #[serde(rename = "hostStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_states: Option<Vec<HostSecurityState>>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "malwareStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_states: Option<Vec<MalwareState>>,
    #[serde(rename = "networkConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connections: Option<Vec<NetworkConnection>>,
    #[serde(rename = "processes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processes: Option<Vec<Process>>,
    #[serde(rename = "recommendedActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<Vec<String>>,
    #[serde(rename = "registryKeyStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_key_states: Option<Vec<RegistryKeyState>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(rename = "sourceMaterials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_materials: Option<Vec<String>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertStatus>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<AlertTrigger>>,
    #[serde(rename = "userStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_states: Option<Vec<UserSecurityState>>,
    #[serde(rename = "vendorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_information: Option<SecurityVendorInformation>,
    #[serde(rename = "vulnerabilityStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_states: Option<Vec<VulnerabilityState>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SecureScoreControlProfile {
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "actionUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(rename = "azureTenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_tenant_id: Option<String>,
    #[serde(rename = "complianceInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_information: Option<Vec<ComplianceInformation>>,
    #[serde(rename = "controlCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_category: Option<String>,
    #[serde(rename = "controlStateUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_state_updates: Option<Vec<SecureScoreControlStateUpdate>>,
    #[serde(rename = "deprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "implementationCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_cost: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "maxScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<i64>,
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(rename = "remediation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
    #[serde(rename = "remediationImpact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_impact: Option<String>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "threats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threats: Option<Vec<String>>,
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "userImpact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_impact: Option<String>,
    #[serde(rename = "vendorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_information: Option<SecurityVendorInformation>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SecureScore {
    #[serde(rename = "activeUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_user_count: Option<i32>,
    #[serde(rename = "averageComparativeScores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_comparative_scores: Option<Vec<AverageComparativeScore>>,
    #[serde(rename = "azureTenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_tenant_id: Option<String>,
    #[serde(rename = "controlScores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_scores: Option<Vec<ControlScore>>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "currentScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_score: Option<i64>,
    #[serde(rename = "enabledServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_services: Option<Vec<String>>,
    #[serde(rename = "licensedUserCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licensed_user_count: Option<i32>,
    #[serde(rename = "maxScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<i64>,
    #[serde(rename = "vendorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_information: Option<SecurityVendorInformation>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Trending {
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[serde(rename = "resourceVisualization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_visualization: Option<ResourceVisualization>,
    #[serde(rename = "resourceReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_reference: Option<ResourceReference>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Entity>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SharedInsight {
    #[serde(rename = "lastShared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_shared: Option<SharingDetail>,
    #[serde(rename = "sharingHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_history: Option<Vec<SharingDetail>>,
    #[serde(rename = "resourceVisualization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_visualization: Option<ResourceVisualization>,
    #[serde(rename = "resourceReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_reference: Option<ResourceReference>,
    #[serde(rename = "lastSharedMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_shared_method: Option<Entity>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Entity>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsedInsight {
    #[serde(rename = "lastUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<UsageDetails>,
    #[serde(rename = "resourceVisualization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_visualization: Option<ResourceVisualization>,
    #[serde(rename = "resourceReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_reference: Option<ResourceReference>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Entity>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AppCatalogs {
    #[serde(rename = "teamsApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_apps: Option<Vec<TeamsApp>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamsApp {
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "distributionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_method: Option<TeamsAppDistributionMethod>,
    #[serde(rename = "appDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_definitions: Option<Vec<TeamsAppDefinition>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(rename = "tabs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabs: Option<Vec<TeamsTab>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamsAppInstallation {
    #[serde(rename = "teamsApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_app: Option<TeamsApp>,
    #[serde(rename = "teamsAppDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_app_definition: Option<TeamsAppDefinition>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamsAsyncOperation {
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<TeamsAsyncOperationType>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TeamsAsyncOperationStatus>,
    #[serde(rename = "lastActionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_action_date_time: Option<String>,
    #[serde(rename = "attemptsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts_count: Option<i32>,
    #[serde(rename = "targetResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[serde(rename = "targetResourceLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_location: Option<String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationError>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamsAppDefinition {
    #[serde(rename = "teamsAppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_app_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TeamsTab {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TeamsTabConfiguration>,
    #[serde(rename = "teamsApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_app: Option<TeamsApp>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DataPolicyOperation {
    #[serde(rename = "completedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_date_time: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DataPolicyOperationStatus>,
    #[serde(rename = "storageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "submittedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_date_time: Option<String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IdentityProvider {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DirectoryAudit {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "correlationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<OperationResult>,
    #[serde(rename = "resultReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_reason: Option<String>,
    #[serde(rename = "activityDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_display_name: Option<String>,
    #[serde(rename = "activityDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_date_time: Option<String>,
    #[serde(rename = "loggedByService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logged_by_service: Option<String>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(rename = "initiatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<AuditActivityInitiator>,
    #[serde(rename = "targetResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resources: Option<Vec<TargetResource>>,
    #[serde(rename = "additionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<Vec<KeyValue>>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SignIn {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "userDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "userPrincipalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "appDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_display_name: Option<String>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SignInStatus>,
    #[serde(rename = "clientAppUsed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_app_used: Option<String>,
    #[serde(rename = "deviceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_detail: Option<DeviceDetail>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SignInLocation>,
    #[serde(rename = "correlationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(rename = "conditionalAccessStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_access_status: Option<ConditionalAccessStatus>,
    #[serde(rename = "appliedConditionalAccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_conditional_access_policies: Option<Vec<AppliedConditionalAccessPolicy>>,
    #[serde(rename = "isInteractive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_interactive: Option<bool>,
    #[serde(rename = "riskDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_detail: Option<RiskDetail>,
    #[serde(rename = "riskLevelAggregated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level_aggregated: Option<RiskLevel>,
    #[serde(rename = "riskLevelDuringSignIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level_during_sign_in: Option<RiskLevel>,
    #[serde(rename = "riskState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_state: Option<RiskState>,
    #[serde(rename = "riskEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_event_types: Option<Vec<RiskEventType>>,
    #[serde(rename = "resourceDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_display_name: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RestrictedSignIn {
    #[serde(rename = "targetTenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tenant_id: Option<String>,
}

#[cfg(feature = "option")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditLogRoot {
    #[serde(rename = "signIns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_ins: Option<Vec<SignIn>>,
    #[serde(rename = "directoryAudits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_audits: Option<Vec<DirectoryAudit>>,
    #[serde(rename = "restrictedSignIns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_sign_ins: Option<Vec<RestrictedSignIn>>,
}
