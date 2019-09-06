#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskEventType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "maliciousIPAddressValidCredentialsBlockedIP")]
    MaliciousIPAddressValidCredentialsBlockedIP,
    #[serde(rename = "investigationsThreatIntelligenceSigninLinked")]
    InvestigationsThreatIntelligenceSigninLinked,
    #[serde(rename = "mcasSuspiciousInboxManipulationRules")]
    McasSuspiciousInboxManipulationRules,
    #[serde(rename = "mcasImpossibleTravel")]
    McasImpossibleTravel,
    #[serde(rename = "adminConfirmedUserCompromised")]
    AdminConfirmedUserCompromised,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "investigationsThreatIntelligence")]
    InvestigationsThreatIntelligence,
    #[serde(rename = "leakedCredentials")]
    LeakedCredentials,
    #[serde(rename = "suspiciousIPAddress")]
    SuspiciousIPAddress,
    #[serde(rename = "malwareInfectedIPAddress")]
    MalwareInfectedIPAddress,
    #[serde(rename = "unfamiliarFeatures")]
    UnfamiliarFeatures,
    #[serde(rename = "maliciousIPAddress")]
    MaliciousIPAddress,
    #[serde(rename = "anonymizedIPAddress")]
    AnonymizedIPAddress,
    #[serde(rename = "unlikelyTravel")]
    UnlikelyTravel,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskDetail {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "adminConfirmedUserCompromised")]
    AdminConfirmedUserCompromised,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "adminConfirmedSigninCompromised")]
    AdminConfirmedSigninCompromised,
    #[serde(rename = "adminDismissedAllRiskForUser")]
    AdminDismissedAllRiskForUser,
    #[serde(rename = "userPassedMFADrivenByRiskBasedPolicy")]
    UserPassedMFADrivenByRiskBasedPolicy,
    #[serde(rename = "aiConfirmedSigninSafe")]
    AiConfirmedSigninSafe,
    #[serde(rename = "adminConfirmedSigninSafe")]
    AdminConfirmedSigninSafe,
    #[serde(rename = "userPerformedSecuredPasswordReset")]
    UserPerformedSecuredPasswordReset,
    #[serde(rename = "userPerformedSecuredPasswordChange")]
    UserPerformedSecuredPasswordChange,
    #[serde(rename = "adminGeneratedTemporaryPassword")]
    AdminGeneratedTemporaryPassword,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskState {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "confirmedCompromised")]
    ConfirmedCompromised,
    #[serde(rename = "atRisk")]
    AtRisk,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "remediated")]
    Remediated,
    #[serde(rename = "confirmedSafe")]
    ConfirmedSafe,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OperationResult {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "success")]
    Success,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "azureAD")]
    AzureAD,
    #[serde(rename = "unifiedGroups")]
    UnifiedGroups,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConditionalAccessStatus {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "notApplied")]
    NotApplied,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "success")]
    Success,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppliedConditionalAccessPolicyResult {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "notEnabled")]
    NotEnabled,
    #[serde(rename = "notApplied")]
    NotApplied,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "success")]
    Success,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataPolicyOperationStatus {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "notStarted")]
    NotStarted,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamsAppDistributionMethod {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "sideloaded")]
    Sideloaded,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "store")]
    Store,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamsAsyncOperationStatus {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "notStarted")]
    NotStarted,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamsAsyncOperationType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "createTeam")]
    CreateTeam,
    #[serde(rename = "unarchiveTeam")]
    UnarchiveTeam,
    #[serde(rename = "archiveTeam")]
    ArchiveTeam,
    #[serde(rename = "cloneTeam")]
    CloneTeam,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GiphyRatingType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "moderate")]
    Moderate,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClonableTeamParts {
    #[serde(rename = "members")]
    Members,
    #[serde(rename = "channels")]
    Channels,
    #[serde(rename = "settings")]
    Settings,
    #[serde(rename = "tabs")]
    Tabs,
    #[serde(rename = "apps")]
    Apps,
    #[serde(rename = "true")]
    True,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamVisibilityType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "hiddenMembership")]
    HiddenMembership,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserAccountSecurityType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "administrator")]
    Administrator,
    #[serde(rename = "power")]
    Power,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityNetworkProtocol {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "spxII")]
    SpxII,
    #[serde(rename = "spx")]
    Spx,
    #[serde(rename = "ipx")]
    Ipx,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "nd")]
    Nd,
    #[serde(rename = "ipv6DestinationOptions")]
    Ipv6DestinationOptions,
    #[serde(rename = "ipv6NoNextHeader")]
    Ipv6NoNextHeader,
    #[serde(rename = "icmpV6")]
    IcmpV6,
    #[serde(rename = "ipSecAuthenticationHeader")]
    IpSecAuthenticationHeader,
    #[serde(rename = "ipSecEncapsulatingSecurityPayload")]
    IpSecEncapsulatingSecurityPayload,
    #[serde(rename = "ipv6FragmentHeader")]
    Ipv6FragmentHeader,
    #[serde(rename = "ipv6RoutingHeader")]
    Ipv6RoutingHeader,
    #[serde(rename = "ipv6")]
    Ipv6,
    #[serde(rename = "idp")]
    Idp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "pup")]
    Pup,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ggp")]
    Ggp,
    #[serde(rename = "igmp")]
    Igmp,
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "ip")]
    Ip,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegistryValueType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "sz")]
    Sz,
    #[serde(rename = "qwordlittleEndian")]
    QwordlittleEndian,
    #[serde(rename = "qword")]
    Qword,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "multiSz")]
    MultiSz,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "expandSz")]
    ExpandSz,
    #[serde(rename = "dwordBigEndian")]
    DwordBigEndian,
    #[serde(rename = "dwordLittleEndian")]
    DwordLittleEndian,
    #[serde(rename = "dword")]
    Dword,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegistryOperation {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "modify")]
    Modify,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegistryHive {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "usersDefault")]
    UsersDefault,
    #[serde(rename = "localMachineSystem")]
    LocalMachineSystem,
    #[serde(rename = "localMachineSoftware")]
    LocalMachineSoftware,
    #[serde(rename = "localMachineSecurity")]
    LocalMachineSecurity,
    #[serde(rename = "localMachineSam")]
    LocalMachineSam,
    #[serde(rename = "currentUser")]
    CurrentUser,
    #[serde(rename = "currentConfig")]
    CurrentConfig,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProcessIntegrityLevel {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "untrusted")]
    Untrusted,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogonType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "remoteInteractive")]
    RemoteInteractive,
    #[serde(rename = "interactive")]
    Interactive,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileHashType {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "ctph")]
    Ctph,
    #[serde(rename = "lsHash")]
    LsHash,
    #[serde(rename = "authenticodeHash256")]
    AuthenticodeHash256,
    #[serde(rename = "md5")]
    Md5,
    #[serde(rename = "sha256")]
    Sha256,
    #[serde(rename = "sha1")]
    Sha1,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmailRole {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "recipient")]
    Recipient,
    #[serde(rename = "sender")]
    Sender,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectionStatus {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "attempted")]
    Attempted,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectionDirection {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertStatus {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "newAlert")]
    NewAlert,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertSeverity {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "informational")]
    Informational,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertFeedback {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "benignPositive")]
    BenignPositive,
    #[serde(rename = "falsePositive")]
    FalsePositive,
    #[serde(rename = "truePositive")]
    TruePositive,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "active")]
    Active,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceEnrollmentFailureReason {
    #[serde(rename = "userAbandonment")]
    UserAbandonment,
    #[serde(rename = "clientDisconnected")]
    ClientDisconnected,
    #[serde(rename = "enrollmentRestrictionsEnforced")]
    EnrollmentRestrictionsEnforced,
    #[serde(rename = "featureNotSupported")]
    FeatureNotSupported,
    #[serde(rename = "badRequest")]
    BadRequest,
    #[serde(rename = "inMaintenance")]
    InMaintenance,
    #[serde(rename = "deviceNotSupported")]
    DeviceNotSupported,
    #[serde(rename = "userValidation")]
    UserValidation,
    #[serde(rename = "accountValidation")]
    AccountValidation,
    #[serde(rename = "authorization")]
    Authorization,
    #[serde(rename = "authentication")]
    Authentication,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApplicationType {
    #[serde(rename = "desktop")]
    Desktop,
    #[serde(rename = "universal")]
    Universal,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RemoteAssistanceOnboardingStatus {
    #[serde(rename = "onboarded")]
    Onboarded,
    #[serde(rename = "onboarding")]
    Onboarding,
    #[serde(rename = "notOnboarded")]
    NotOnboarded,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstallState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "uninstallFailed")]
    UninstallFailed,
    #[serde(rename = "notInstalled")]
    NotInstalled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "notApplicable")]
    NotApplicable,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationTemplateBrandingOptions {
    #[serde(rename = "includeContactInformation")]
    IncludeContactInformation,
    #[serde(rename = "includeCompanyName")]
    IncludeCompanyName,
    #[serde(rename = "includeCompanyLogo")]
    IncludeCompanyLogo,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "true")]
    True,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedAppFlaggedReason {
    #[serde(rename = "rootedDevice")]
    RootedDevice,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsInformationProtectionPinCharacterRequirements {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "requireAtLeastOne")]
    RequireAtLeastOne,
    #[serde(rename = "notAllow")]
    NotAllow,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsInformationProtectionEnforcementLevel {
    #[serde(rename = "encryptAuditAndBlock")]
    EncryptAuditAndBlock,
    #[serde(rename = "encryptAuditAndPrompt")]
    EncryptAuditAndPrompt,
    #[serde(rename = "encryptAndAuditOnly")]
    EncryptAndAuditOnly,
    #[serde(rename = "noProtection")]
    NoProtection,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedAppDataEncryptionType {
    #[serde(rename = "whenDeviceLocked")]
    WhenDeviceLocked,
    #[serde(rename = "whenDeviceLockedExceptOpenFiles")]
    WhenDeviceLockedExceptOpenFiles,
    #[serde(rename = "afterDeviceRestart")]
    AfterDeviceRestart,
    #[serde(rename = "useDeviceSettings")]
    UseDeviceSettings,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedAppPinCharacterSet {
    #[serde(rename = "alphanumericAndSymbol")]
    AlphanumericAndSymbol,
    #[serde(rename = "numeric")]
    Numeric,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedAppClipboardSharingLevel {
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "managedApps")]
    ManagedApps,
    #[serde(rename = "managedAppsWithPasteIn")]
    ManagedAppsWithPasteIn,
    #[serde(rename = "allApps")]
    AllApps,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedAppDataTransferLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "managedApps")]
    ManagedApps,
    #[serde(rename = "allApps")]
    AllApps,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedAppDataStorageLocation {
    #[serde(rename = "localStorage")]
    LocalStorage,
    #[serde(rename = "sharePoint")]
    SharePoint,
    #[serde(rename = "oneDriveForBusiness")]
    OneDriveForBusiness,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceManagementPartnerAppType {
    #[serde(rename = "multiTenantApp")]
    MultiTenantApp,
    #[serde(rename = "singleTenantApp")]
    SingleTenantApp,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceManagementPartnerTenantState {
    #[serde(rename = "unresponsive")]
    Unresponsive,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MobileThreatPartnerTenantState {
    #[serde(rename = "unresponsive")]
    Unresponsive,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceManagementExchangeConnectorType {
    #[serde(rename = "dedicated")]
    Dedicated,
    #[serde(rename = "serviceToService")]
    ServiceToService,
    #[serde(rename = "hosted")]
    Hosted,
    #[serde(rename = "onPremises")]
    OnPremises,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceManagementExchangeConnectorStatus {
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "connectionPending")]
    ConnectionPending,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VppTokenSyncStatus {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VppTokenState {
    #[serde(rename = "assignedToExternalMDM")]
    AssignedToExternalMDM,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Enablement {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "notConfigured")]
    NotConfigured,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsHelloForBusinessPinUsage {
    #[serde(rename = "disallowed")]
    Disallowed,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "allowed")]
    Allowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MdmAuthority {
    #[serde(rename = "office365")]
    Office365,
    #[serde(rename = "sccm")]
    Sccm,
    #[serde(rename = "intune")]
    Intune,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceManagementExchangeConnectorSyncType {
    #[serde(rename = "deltaSync")]
    DeltaSync,
    #[serde(rename = "fullSync")]
    FullSync,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IosUpdatesInstallStatus {
    #[serde(rename = "sharedDeviceUserLoggedInError")]
    SharedDeviceUserLoggedInError,
    #[serde(rename = "notSupportedOperation")]
    NotSupportedOperation,
    #[serde(rename = "installFailed")]
    InstallFailed,
    #[serde(rename = "installPhoneCallInProgress")]
    InstallPhoneCallInProgress,
    #[serde(rename = "installInsufficientPower")]
    InstallInsufficientPower,
    #[serde(rename = "installInsufficientSpace")]
    InstallInsufficientSpace,
    #[serde(rename = "installing")]
    Installing,
    #[serde(rename = "downloadInsufficientNetwork")]
    DownloadInsufficientNetwork,
    #[serde(rename = "downloadInsufficientPower")]
    DownloadInsufficientPower,
    #[serde(rename = "downloadInsufficientSpace")]
    DownloadInsufficientSpace,
    #[serde(rename = "downloadRequiresComputer")]
    DownloadRequiresComputer,
    #[serde(rename = "downloadFailed")]
    DownloadFailed,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "success")]
    Success,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PolicyPlatformType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "androidWorkProfile")]
    AndroidWorkProfile,
    #[serde(rename = "windows10AndLater")]
    Windows10AndLater,
    #[serde(rename = "windows81AndLater")]
    Windows81AndLater,
    #[serde(rename = "windowsPhone81")]
    WindowsPhone81,
    #[serde(rename = "macOS")]
    MacOS,
    #[serde(rename = "iOS")]
    IOS,
    #[serde(rename = "android")]
    Android,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceThreatProtectionLevel {
    #[serde(rename = "notSet")]
    NotSet,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "secured")]
    Secured,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceComplianceActionType {
    #[serde(rename = "pushNotification")]
    PushNotification,
    #[serde(rename = "removeResourceAccessProfiles")]
    RemoveResourceAccessProfiles,
    #[serde(rename = "wipe")]
    Wipe,
    #[serde(rename = "retire")]
    Retire,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "notification")]
    Notification,
    #[serde(rename = "noAction")]
    NoAction,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WelcomeScreenMeetingInformation {
    #[serde(rename = "showOrganizerAndTimeAndSubject")]
    ShowOrganizerAndTimeAndSubject,
    #[serde(rename = "showOrganizerAndTimeOnly")]
    ShowOrganizerAndTimeOnly,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MiracastChannel {
    #[serde(rename = "oneHundredSixtyFive")]
    OneHundredSixtyFive,
    #[serde(rename = "oneHundredSixtyOne")]
    OneHundredSixtyOne,
    #[serde(rename = "oneHundredFiftySeven")]
    OneHundredFiftySeven,
    #[serde(rename = "oneHundredFiftyThree")]
    OneHundredFiftyThree,
    #[serde(rename = "oneHundredFortyNine")]
    OneHundredFortyNine,
    #[serde(rename = "fortyEight")]
    FortyEight,
    #[serde(rename = "fortyFour")]
    FortyFour,
    #[serde(rename = "forty")]
    Forty,
    #[serde(rename = "thirtySix")]
    ThirtySix,
    #[serde(rename = "eleven")]
    Eleven,
    #[serde(rename = "ten")]
    Ten,
    #[serde(rename = "nine")]
    Nine,
    #[serde(rename = "eight")]
    Eight,
    #[serde(rename = "seven")]
    Seven,
    #[serde(rename = "six")]
    Six,
    #[serde(rename = "five")]
    Five,
    #[serde(rename = "four")]
    Four,
    #[serde(rename = "three")]
    Three,
    #[serde(rename = "two")]
    Two,
    #[serde(rename = "one")]
    One,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsUserAccountControlSettings {
    #[serde(rename = "neverNotify")]
    NeverNotify,
    #[serde(rename = "notifyOnAppChangesWithoutDimming")]
    NotifyOnAppChangesWithoutDimming,
    #[serde(rename = "notifyOnAppChanges")]
    NotifyOnAppChanges,
    #[serde(rename = "alwaysNotify")]
    AlwaysNotify,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SiteSecurityLevel {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "mediumHigh")]
    MediumHigh,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "mediumLow")]
    MediumLow,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InternetSiteSecurityLevel {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "mediumHigh")]
    MediumHigh,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsUpdateType {
    #[serde(rename = "windowsInsiderBuildRelease")]
    WindowsInsiderBuildRelease,
    #[serde(rename = "windowsInsiderBuildSlow")]
    WindowsInsiderBuildSlow,
    #[serde(rename = "windowsInsiderBuildFast")]
    WindowsInsiderBuildFast,
    #[serde(rename = "businessReadyOnly")]
    BusinessReadyOnly,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SharedPCAllowedAccountType {
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "guest")]
    Guest,
    #[serde(rename = "true")]
    True,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SharedPCAccountDeletionPolicyType {
    #[serde(rename = "diskSpaceThresholdOrInactiveThreshold")]
    DiskSpaceThresholdOrInactiveThreshold,
    #[serde(rename = "diskSpaceThreshold")]
    DiskSpaceThreshold,
    #[serde(rename = "immediate")]
    Immediate,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsDeliveryOptimizationMode {
    #[serde(rename = "bypassMode")]
    BypassMode,
    #[serde(rename = "simpleDownload")]
    SimpleDownload,
    #[serde(rename = "httpWithInternetPeering")]
    HttpWithInternetPeering,
    #[serde(rename = "httpWithPeeringPrivateGroup")]
    HttpWithPeeringPrivateGroup,
    #[serde(rename = "httpWithPeeringNat")]
    HttpWithPeeringNat,
    #[serde(rename = "httpOnly")]
    HttpOnly,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EditionUpgradeLicenseType {
    #[serde(rename = "licenseFile")]
    LicenseFile,
    #[serde(rename = "productKey")]
    ProductKey,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PrereleaseFeatures {
    #[serde(rename = "notAllowed")]
    NotAllowed,
    #[serde(rename = "settingsAndExperimentations")]
    SettingsAndExperimentations,
    #[serde(rename = "settingsOnly")]
    SettingsOnly,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EdgeSearchEngineType {
    #[serde(rename = "bing")]
    Bing,
    #[serde(rename = "default")]
    Default,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SafeSearchFilterType {
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutomaticUpdateMode {
    #[serde(rename = "autoInstallAndRebootWithoutEndUserControl")]
    AutoInstallAndRebootWithoutEndUserControl,
    #[serde(rename = "autoInstallAndRebootAtScheduledTime")]
    AutoInstallAndRebootAtScheduledTime,
    #[serde(rename = "autoInstallAndRebootAtMaintenanceTime")]
    AutoInstallAndRebootAtMaintenanceTime,
    #[serde(rename = "autoInstallAtMaintenanceTime")]
    AutoInstallAtMaintenanceTime,
    #[serde(rename = "notifyDownload")]
    NotifyDownload,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsSpotlightEnablementSettings {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "notConfigured")]
    NotConfigured,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsStartMenuModeType {
    #[serde(rename = "nonFullScreen")]
    NonFullScreen,
    #[serde(rename = "fullScreen")]
    FullScreen,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsStartMenuAppListVisibilityType {
    #[serde(rename = "disableSettingsApp")]
    DisableSettingsApp,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "collapse")]
    Collapse,
    #[serde(rename = "userDefined")]
    UserDefined,
    #[serde(rename = "true")]
    True,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefenderCloudBlockLevelType {
    #[serde(rename = "zeroTolerance")]
    ZeroTolerance,
    #[serde(rename = "highPlus")]
    HighPlus,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "notConfigured")]
    NotConfigured,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefenderScanType {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "quick")]
    Quick,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefenderPromptForSampleSubmission {
    #[serde(rename = "sendAllDataWithoutPrompting")]
    SendAllDataWithoutPrompting,
    #[serde(rename = "neverSendData")]
    NeverSendData,
    #[serde(rename = "promptBeforeSendingPersonalData")]
    PromptBeforeSendingPersonalData,
    #[serde(rename = "alwaysPrompt")]
    AlwaysPrompt,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefenderMonitorFileActivity {
    #[serde(rename = "monitorOutgoingFilesOnly")]
    MonitorOutgoingFilesOnly,
    #[serde(rename = "monitorIncomingFilesOnly")]
    MonitorIncomingFilesOnly,
    #[serde(rename = "monitorAllFiles")]
    MonitorAllFiles,
    #[serde(rename = "disable")]
    Disable,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WeeklySchedule {
    #[serde(rename = "saturday")]
    Saturday,
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "thursday")]
    Thursday,
    #[serde(rename = "wednesday")]
    Wednesday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "sunday")]
    Sunday,
    #[serde(rename = "everyday")]
    Everyday,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefenderThreatAction {
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "userDefined")]
    UserDefined,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "quarantine")]
    Quarantine,
    #[serde(rename = "clean")]
    Clean,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VisibilitySetting {
    #[serde(rename = "show")]
    Show,
    #[serde(rename = "hide")]
    Hide,
    #[serde(rename = "notConfigured")]
    NotConfigured,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EdgeCookiePolicy {
    #[serde(rename = "blockAll")]
    BlockAll,
    #[serde(rename = "blockThirdParty")]
    BlockThirdParty,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DiagnosticDataSubmissionMode {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "enhanced")]
    Enhanced,
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "userDefined")]
    UserDefined,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BitLockerEncryptionMethod {
    #[serde(rename = "xtsAes256")]
    XtsAes256,
    #[serde(rename = "xtsAes128")]
    XtsAes128,
    #[serde(rename = "aesCbc256")]
    AesCbc256,
    #[serde(rename = "aesCbc128")]
    AesCbc128,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApplicationGuardBlockClipboardSharingType {
    #[serde(rename = "blockNone")]
    BlockNone,
    #[serde(rename = "blockContainerToHost")]
    BlockContainerToHost,
    #[serde(rename = "blockHostToContainer")]
    BlockHostToContainer,
    #[serde(rename = "blockBoth")]
    BlockBoth,
    #[serde(rename = "notConfigured")]
    NotConfigured,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApplicationGuardBlockFileTransferType {
    #[serde(rename = "blockTextFile")]
    BlockTextFile,
    #[serde(rename = "blockNone")]
    BlockNone,
    #[serde(rename = "blockImageFile")]
    BlockImageFile,
    #[serde(rename = "blockImageAndTextFile")]
    BlockImageAndTextFile,
    #[serde(rename = "notConfigured")]
    NotConfigured,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppLockerApplicationControlType {
    #[serde(rename = "auditComponentsStoreAppsAndSmartlocker")]
    AuditComponentsStoreAppsAndSmartlocker,
    #[serde(rename = "enforceComponentsStoreAppsAndSmartlocker")]
    EnforceComponentsStoreAppsAndSmartlocker,
    #[serde(rename = "auditComponentsAndStoreApps")]
    AuditComponentsAndStoreApps,
    #[serde(rename = "enforceComponentsAndStoreApps")]
    EnforceComponentsAndStoreApps,
    #[serde(rename = "notConfigured")]
    NotConfigured,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FirewallPacketQueueingMethodType {
    #[serde(rename = "queueBoth")]
    QueueBoth,
    #[serde(rename = "queueOutbound")]
    QueueOutbound,
    #[serde(rename = "queueInbound")]
    QueueInbound,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FirewallCertificateRevocationListCheckMethodType {
    #[serde(rename = "require")]
    Require,
    #[serde(rename = "attempt")]
    Attempt,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FirewallPreSharedKeyEncodingMethodType {
    #[serde(rename = "utF8")]
    UtF8,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateManagementSetting {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "notConfigured")]
    NotConfigured,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IosNotificationAlertType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "modal")]
    Modal,
    #[serde(rename = "banner")]
    Banner,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequiredPasswordType {
    #[serde(rename = "numeric")]
    Numeric,
    #[serde(rename = "alphanumeric")]
    Alphanumeric,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingAppsType {
    #[serde(rename = "agesAbove17")]
    AgesAbove17,
    #[serde(rename = "agesAbove12")]
    AgesAbove12,
    #[serde(rename = "agesAbove9")]
    AgesAbove9,
    #[serde(rename = "agesAbove4")]
    AgesAbove4,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingUnitedStatesTelevisionType {
    #[serde(rename = "adults")]
    Adults,
    #[serde(rename = "childrenAbove14")]
    ChildrenAbove14,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "childrenAbove7")]
    ChildrenAbove7,
    #[serde(rename = "childrenAll")]
    ChildrenAll,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingUnitedStatesMoviesType {
    #[serde(rename = "adults")]
    Adults,
    #[serde(rename = "restricted")]
    Restricted,
    #[serde(rename = "parentalGuidance13")]
    ParentalGuidance13,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingUnitedKingdomTelevisionType {
    #[serde(rename = "caution")]
    Caution,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingUnitedKingdomMoviesType {
    #[serde(rename = "adults")]
    Adults,
    #[serde(rename = "agesAbove15")]
    AgesAbove15,
    #[serde(rename = "agesAbove12Cinema")]
    AgesAbove12Cinema,
    #[serde(rename = "agesAbove12Video")]
    AgesAbove12Video,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "universalChildren")]
    UniversalChildren,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingNewZealandTelevisionType {
    #[serde(rename = "adults")]
    Adults,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingNewZealandMoviesType {
    #[serde(rename = "agesAbove16Restricted")]
    AgesAbove16Restricted,
    #[serde(rename = "restricted")]
    Restricted,
    #[serde(rename = "agesAbove18")]
    AgesAbove18,
    #[serde(rename = "agesAbove16")]
    AgesAbove16,
    #[serde(rename = "agesAbove15")]
    AgesAbove15,
    #[serde(rename = "agesAbove13")]
    AgesAbove13,
    #[serde(rename = "mature")]
    Mature,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingJapanTelevisionType {
    #[serde(rename = "explicitAllowed")]
    ExplicitAllowed,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingJapanMoviesType {
    #[serde(rename = "agesAbove18")]
    AgesAbove18,
    #[serde(rename = "agesAbove15")]
    AgesAbove15,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingIrelandTelevisionType {
    #[serde(rename = "mature")]
    Mature,
    #[serde(rename = "parentalSupervision")]
    ParentalSupervision,
    #[serde(rename = "youngAdults")]
    YoungAdults,
    #[serde(rename = "children")]
    Children,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingIrelandMoviesType {
    #[serde(rename = "adults")]
    Adults,
    #[serde(rename = "agesAbove16")]
    AgesAbove16,
    #[serde(rename = "agesAbove15")]
    AgesAbove15,
    #[serde(rename = "agesAbove12")]
    AgesAbove12,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingGermanyTelevisionType {
    #[serde(rename = "adults")]
    Adults,
    #[serde(rename = "agesAbove16")]
    AgesAbove16,
    #[serde(rename = "agesAbove12")]
    AgesAbove12,
    #[serde(rename = "agesAbove6")]
    AgesAbove6,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingGermanyMoviesType {
    #[serde(rename = "adults")]
    Adults,
    #[serde(rename = "agesAbove16")]
    AgesAbove16,
    #[serde(rename = "agesAbove12")]
    AgesAbove12,
    #[serde(rename = "agesAbove6")]
    AgesAbove6,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingFranceTelevisionType {
    #[serde(rename = "agesAbove18")]
    AgesAbove18,
    #[serde(rename = "agesAbove16")]
    AgesAbove16,
    #[serde(rename = "agesAbove12")]
    AgesAbove12,
    #[serde(rename = "agesAbove10")]
    AgesAbove10,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingFranceMoviesType {
    #[serde(rename = "agesAbove18")]
    AgesAbove18,
    #[serde(rename = "agesAbove16")]
    AgesAbove16,
    #[serde(rename = "agesAbove12")]
    AgesAbove12,
    #[serde(rename = "agesAbove10")]
    AgesAbove10,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingCanadaTelevisionType {
    #[serde(rename = "agesAbove18")]
    AgesAbove18,
    #[serde(rename = "agesAbove14")]
    AgesAbove14,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "childrenAbove8")]
    ChildrenAbove8,
    #[serde(rename = "children")]
    Children,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingCanadaMoviesType {
    #[serde(rename = "restricted")]
    Restricted,
    #[serde(rename = "agesAbove18")]
    AgesAbove18,
    #[serde(rename = "agesAbove14")]
    AgesAbove14,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingAustraliaTelevisionType {
    #[serde(rename = "agesAbove15AdultViolence")]
    AgesAbove15AdultViolence,
    #[serde(rename = "agesAbove15")]
    AgesAbove15,
    #[serde(rename = "mature")]
    Mature,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "children")]
    Children,
    #[serde(rename = "preschoolers")]
    Preschoolers,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingAustraliaMoviesType {
    #[serde(rename = "agesAbove18")]
    AgesAbove18,
    #[serde(rename = "agesAbove15")]
    AgesAbove15,
    #[serde(rename = "mature")]
    Mature,
    #[serde(rename = "parentalGuidance")]
    ParentalGuidance,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "allBlocked")]
    AllBlocked,
    #[serde(rename = "allAllowed")]
    AllAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AndroidWorkProfileDefaultAppPermissionPolicyType {
    #[serde(rename = "autoDeny")]
    AutoDeny,
    #[serde(rename = "autoGrant")]
    AutoGrant,
    #[serde(rename = "prompt")]
    Prompt,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AndroidWorkProfileCrossProfileDataSharingType {
    #[serde(rename = "noRestrictions")]
    NoRestrictions,
    #[serde(rename = "allowPersonalToWork")]
    AllowPersonalToWork,
    #[serde(rename = "preventAny")]
    PreventAny,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AndroidWorkProfileRequiredPasswordType {
    #[serde(rename = "alphanumericWithSymbols")]
    AlphanumericWithSymbols,
    #[serde(rename = "atLeastAlphanumeric")]
    AtLeastAlphanumeric,
    #[serde(rename = "atLeastAlphabetic")]
    AtLeastAlphabetic,
    #[serde(rename = "numericComplex")]
    NumericComplex,
    #[serde(rename = "atLeastNumeric")]
    AtLeastNumeric,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "lowSecurityBiometric")]
    LowSecurityBiometric,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebBrowserCookieSettings {
    #[serde(rename = "allowAlways")]
    AllowAlways,
    #[serde(rename = "allowFromWebsitesVisited")]
    AllowFromWebsitesVisited,
    #[serde(rename = "allowCurrentWebSite")]
    AllowCurrentWebSite,
    #[serde(rename = "blockAlways")]
    BlockAlways,
    #[serde(rename = "browserDefault")]
    BrowserDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AndroidRequiredPasswordType {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "numericComplex")]
    NumericComplex,
    #[serde(rename = "numeric")]
    Numeric,
    #[serde(rename = "lowSecurityBiometric")]
    LowSecurityBiometric,
    #[serde(rename = "alphanumericWithSymbols")]
    AlphanumericWithSymbols,
    #[serde(rename = "alphanumeric")]
    Alphanumeric,
    #[serde(rename = "alphabetic")]
    Alphabetic,
    #[serde(rename = "deviceDefault")]
    DeviceDefault,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppListType {
    #[serde(rename = "appsNotInListCompliant")]
    AppsNotInListCompliant,
    #[serde(rename = "appsInListCompliant")]
    AppsInListCompliant,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Windows10EditionType {
    #[serde(rename = "windows10ProfessionalWorkstationN")]
    Windows10ProfessionalWorkstationN,
    #[serde(rename = "windows10ProfessionalWorkstation")]
    Windows10ProfessionalWorkstation,
    #[serde(rename = "windows10ProfessionalEducationN")]
    Windows10ProfessionalEducationN,
    #[serde(rename = "windows10ProfessionalEducation")]
    Windows10ProfessionalEducation,
    #[serde(rename = "windows10ProfessionalN")]
    Windows10ProfessionalN,
    #[serde(rename = "windows10Professional")]
    Windows10Professional,
    #[serde(rename = "windows10HolographicEnterprise")]
    Windows10HolographicEnterprise,
    #[serde(rename = "windows10MobileEnterprise")]
    Windows10MobileEnterprise,
    #[serde(rename = "windows10EducationN")]
    Windows10EducationN,
    #[serde(rename = "windows10Education")]
    Windows10Education,
    #[serde(rename = "windows10EnterpriseN")]
    Windows10EnterpriseN,
    #[serde(rename = "windows10Enterprise")]
    Windows10Enterprise,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceManagementSubscriptionState {
    #[serde(rename = "lockedOut")]
    LockedOut,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedDevicePartnerReportedHealthState {
    #[serde(rename = "misconfigured")]
    Misconfigured,
    #[serde(rename = "compromised")]
    Compromised,
    #[serde(rename = "unresponsive")]
    Unresponsive,
    #[serde(rename = "highSeverity")]
    HighSeverity,
    #[serde(rename = "mediumSeverity")]
    MediumSeverity,
    #[serde(rename = "lowSeverity")]
    LowSeverity,
    #[serde(rename = "secured")]
    Secured,
    #[serde(rename = "deactivated")]
    Deactivated,
    #[serde(rename = "activated")]
    Activated,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceManagementExchangeAccessStateReason {
    #[serde(rename = "deviceNotKnownWithManagedApp")]
    DeviceNotKnownWithManagedApp,
    #[serde(rename = "compromisedPassword")]
    CompromisedPassword,
    #[serde(rename = "azureADBlockDueToAccessPolicy")]
    AzureADBlockDueToAccessPolicy,
    #[serde(rename = "mfaRequired")]
    MfaRequired,
    #[serde(rename = "unknownLocation")]
    UnknownLocation,
    #[serde(rename = "notEnrolled")]
    NotEnrolled,
    #[serde(rename = "notCompliant")]
    NotCompliant,
    #[serde(rename = "compliant")]
    Compliant,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "exchangeMailboxPolicy")]
    ExchangeMailboxPolicy,
    #[serde(rename = "exchangeUpgrade")]
    ExchangeUpgrade,
    #[serde(rename = "exchangeDeviceRule")]
    ExchangeDeviceRule,
    #[serde(rename = "exchangeIndividualRule")]
    ExchangeIndividualRule,
    #[serde(rename = "exchangeGlobalRule")]
    ExchangeGlobalRule,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceManagementExchangeAccessState {
    #[serde(rename = "quarantined")]
    Quarantined,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceRegistrationState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "notRegisteredPendingEnrollment")]
    NotRegisteredPendingEnrollment,
    #[serde(rename = "certificateReset")]
    CertificateReset,
    #[serde(rename = "approvalPending")]
    ApprovalPending,
    #[serde(rename = "keyConflict")]
    KeyConflict,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "notRegistered")]
    NotRegistered,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceEnrollmentType {
    #[serde(rename = "windowsCoManagement")]
    WindowsCoManagement,
    #[serde(rename = "windowsBulkAzureDomainJoin")]
    WindowsBulkAzureDomainJoin,
    #[serde(rename = "windowsAutoEnrollment")]
    WindowsAutoEnrollment,
    #[serde(rename = "windowsBulkUserless")]
    WindowsBulkUserless,
    #[serde(rename = "windowsAzureADJoin")]
    WindowsAzureADJoin,
    #[serde(rename = "appleBulkWithoutUser")]
    AppleBulkWithoutUser,
    #[serde(rename = "appleBulkWithUser")]
    AppleBulkWithUser,
    #[serde(rename = "deviceEnrollmentManager")]
    DeviceEnrollmentManager,
    #[serde(rename = "userEnrollment")]
    UserEnrollment,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagementAgentType {
    #[serde(rename = "googleCloudDevicePolicyController")]
    GoogleCloudDevicePolicyController,
    #[serde(rename = "jamf")]
    Jamf,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "configurationManagerClientMdmEas")]
    ConfigurationManagerClientMdmEas,
    #[serde(rename = "configurationManagerClientMdm")]
    ConfigurationManagerClientMdm,
    #[serde(rename = "configurationManagerClient")]
    ConfigurationManagerClient,
    #[serde(rename = "easIntuneClient")]
    EasIntuneClient,
    #[serde(rename = "intuneClient")]
    IntuneClient,
    #[serde(rename = "easMdm")]
    EasMdm,
    #[serde(rename = "mdm")]
    Mdm,
    #[serde(rename = "eas")]
    Eas,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComplianceState {
    #[serde(rename = "configManager")]
    ConfigManager,
    #[serde(rename = "inGracePeriod")]
    InGracePeriod,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "conflict")]
    Conflict,
    #[serde(rename = "noncompliant")]
    Noncompliant,
    #[serde(rename = "compliant")]
    Compliant,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedDeviceOwnerType {
    #[serde(rename = "personal")]
    Personal,
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionState {
    #[serde(rename = "notSupported")]
    NotSupported,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MdmAppConfigKeyType {
    #[serde(rename = "tokenType")]
    TokenType,
    #[serde(rename = "booleanType")]
    BooleanType,
    #[serde(rename = "realType")]
    RealType,
    #[serde(rename = "integerType")]
    IntegerType,
    #[serde(rename = "stringType")]
    StringType,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComplianceStatus {
    #[serde(rename = "notAssigned")]
    NotAssigned,
    #[serde(rename = "conflict")]
    Conflict,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "nonCompliant")]
    NonCompliant,
    #[serde(rename = "remediated")]
    Remediated,
    #[serde(rename = "compliant")]
    Compliant,
    #[serde(rename = "notApplicable")]
    NotApplicable,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VppTokenAccountType {
    #[serde(rename = "education")]
    Education,
    #[serde(rename = "business")]
    Business,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MicrosoftStoreForBusinessLicenseType {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "offline")]
    Offline,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsDeviceType {
    #[serde(rename = "team")]
    Team,
    #[serde(rename = "holographic")]
    Holographic,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "desktop")]
    Desktop,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "true")]
    True,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MobileAppContentFileUploadState {
    #[serde(rename = "commitFileTimedOut")]
    CommitFileTimedOut,
    #[serde(rename = "commitFileFailed")]
    CommitFileFailed,
    #[serde(rename = "commitFilePending")]
    CommitFilePending,
    #[serde(rename = "commitFileSuccess")]
    CommitFileSuccess,
    #[serde(rename = "azureStorageUriRenewalTimedOut")]
    AzureStorageUriRenewalTimedOut,
    #[serde(rename = "azureStorageUriRenewalFailed")]
    AzureStorageUriRenewalFailed,
    #[serde(rename = "azureStorageUriRenewalPending")]
    AzureStorageUriRenewalPending,
    #[serde(rename = "azureStorageUriRenewalSuccess")]
    AzureStorageUriRenewalSuccess,
    #[serde(rename = "azureStorageUriRequestTimedOut")]
    AzureStorageUriRequestTimedOut,
    #[serde(rename = "azureStorageUriRequestFailed")]
    AzureStorageUriRequestFailed,
    #[serde(rename = "azureStorageUriRequestPending")]
    AzureStorageUriRequestPending,
    #[serde(rename = "azureStorageUriRequestSuccess")]
    AzureStorageUriRequestSuccess,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "transientError")]
    TransientError,
    #[serde(rename = "success")]
    Success,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ManagedAppAvailability {
    #[serde(rename = "lineOfBusiness")]
    LineOfBusiness,
    #[serde(rename = "global")]
    Global,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowsArchitecture {
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "arm")]
    Arm,
    #[serde(rename = "x64")]
    X64,
    #[serde(rename = "x86")]
    X86,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "true")]
    True,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MobileAppPublishingState {
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "notPublished")]
    NotPublished,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstallIntent {
    #[serde(rename = "availableWithoutEnrollment")]
    AvailableWithoutEnrollment,
    #[serde(rename = "uninstall")]
    Uninstall,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "available")]
    Available,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EducationGender {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EducationExternalSource {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "sis")]
    Sis,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EducationUserRole {
    #[serde(rename = "unknownFutureValue")]
    UnknownFutureValue,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "teacher")]
    Teacher,
    #[serde(rename = "student")]
    Student,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnenoteUserRole {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Reader")]
    Reader,
    #[serde(rename = "Contributor")]
    Contributor,
    #[serde(rename = "Owner")]
    Owner,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnenoteSourceService {
    #[serde(rename = "OnPremOneDriveForBusiness")]
    OnPremOneDriveForBusiness,
    #[serde(rename = "OneDriveForBusiness")]
    OneDriveForBusiness,
    #[serde(rename = "OneDrive")]
    OneDrive,
    #[serde(rename = "Unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnenotePatchActionType {
    #[serde(rename = "Prepend")]
    Prepend,
    #[serde(rename = "Insert")]
    Insert,
    #[serde(rename = "Delete")]
    Delete,
    #[serde(rename = "Append")]
    Append,
    #[serde(rename = "Replace")]
    Replace,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnenotePatchInsertPosition {
    #[serde(rename = "Before")]
    Before,
    #[serde(rename = "After")]
    After,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OperationStatus {
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "NotStarted")]
    NotStarted,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlannerPreviewType {
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "checklist")]
    Checklist,
    #[serde(rename = "noPreview")]
    NoPreview,
    #[serde(rename = "automatic")]
    Automatic,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PhysicalAddressType {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActivityDomain {
    #[serde(rename = "unrestricted")]
    Unrestricted,
    #[serde(rename = "personal")]
    Personal,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebsiteType {
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "blog")]
    Blog,
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "other")]
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PhoneType {
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "pager")]
    Pager,
    #[serde(rename = "otherFax")]
    OtherFax,
    #[serde(rename = "businessFax")]
    BusinessFax,
    #[serde(rename = "homeFax")]
    HomeFax,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "home")]
    Home,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SelectionLikelihoodInfo {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "notSpecified")]
    NotSpecified,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CategoryColor {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "preset24")]
    Preset24,
    #[serde(rename = "preset23")]
    Preset23,
    #[serde(rename = "preset22")]
    Preset22,
    #[serde(rename = "preset21")]
    Preset21,
    #[serde(rename = "preset20")]
    Preset20,
    #[serde(rename = "preset19")]
    Preset19,
    #[serde(rename = "preset18")]
    Preset18,
    #[serde(rename = "preset17")]
    Preset17,
    #[serde(rename = "preset16")]
    Preset16,
    #[serde(rename = "preset15")]
    Preset15,
    #[serde(rename = "preset14")]
    Preset14,
    #[serde(rename = "preset13")]
    Preset13,
    #[serde(rename = "preset12")]
    Preset12,
    #[serde(rename = "preset11")]
    Preset11,
    #[serde(rename = "preset10")]
    Preset10,
    #[serde(rename = "preset9")]
    Preset9,
    #[serde(rename = "preset8")]
    Preset8,
    #[serde(rename = "preset7")]
    Preset7,
    #[serde(rename = "preset6")]
    Preset6,
    #[serde(rename = "preset5")]
    Preset5,
    #[serde(rename = "preset4")]
    Preset4,
    #[serde(rename = "preset3")]
    Preset3,
    #[serde(rename = "preset2")]
    Preset2,
    #[serde(rename = "preset1")]
    Preset1,
    #[serde(rename = "preset0")]
    Preset0,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageActionFlag {
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "replyToAll")]
    ReplyToAll,
    #[serde(rename = "reply")]
    Reply,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "noResponseNecessary")]
    NoResponseNecessary,
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "fyi")]
    Fyi,
    #[serde(rename = "followUp")]
    FollowUp,
    #[serde(rename = "doNotForward")]
    DoNotForward,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "any")]
    Any,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MeetingMessageType {
    #[serde(rename = "meetingDeclined")]
    MeetingDeclined,
    #[serde(rename = "meetingTenativelyAccepted")]
    MeetingTenativelyAccepted,
    #[serde(rename = "meetingAccepted")]
    MeetingAccepted,
    #[serde(rename = "meetingCancelled")]
    MeetingCancelled,
    #[serde(rename = "meetingRequest")]
    MeetingRequest,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "seriesMaster")]
    SeriesMaster,
    #[serde(rename = "exception")]
    Exception,
    #[serde(rename = "occurrence")]
    Occurrence,
    #[serde(rename = "singleInstance")]
    SingleInstance,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecurrenceRangeType {
    #[serde(rename = "numbered")]
    Numbered,
    #[serde(rename = "noEnd")]
    NoEnd,
    #[serde(rename = "endDate")]
    EndDate,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WeekIndex {
    #[serde(rename = "last")]
    Last,
    #[serde(rename = "fourth")]
    Fourth,
    #[serde(rename = "third")]
    Third,
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "first")]
    First,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecurrencePatternType {
    #[serde(rename = "relativeYearly")]
    RelativeYearly,
    #[serde(rename = "absoluteYearly")]
    AbsoluteYearly,
    #[serde(rename = "relativeMonthly")]
    RelativeMonthly,
    #[serde(rename = "absoluteMonthly")]
    AbsoluteMonthly,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "daily")]
    Daily,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Sensitivity {
    #[serde(rename = "confidential")]
    Confidential,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "personal")]
    Personal,
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "notResponded")]
    NotResponded,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "tentativelyAccepted")]
    TentativelyAccepted,
    #[serde(rename = "organizer")]
    Organizer,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CalendarColor {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "maxColor")]
    MaxColor,
    #[serde(rename = "lightRed")]
    LightRed,
    #[serde(rename = "lightBrown")]
    LightBrown,
    #[serde(rename = "lightPink")]
    LightPink,
    #[serde(rename = "lightTeal")]
    LightTeal,
    #[serde(rename = "lightYellow")]
    LightYellow,
    #[serde(rename = "lightGray")]
    LightGray,
    #[serde(rename = "lightOrange")]
    LightOrange,
    #[serde(rename = "lightGreen")]
    LightGreen,
    #[serde(rename = "lightBlue")]
    LightBlue,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FollowupFlagStatus {
    #[serde(rename = "flagged")]
    Flagged,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "notFlagged")]
    NotFlagged,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InferenceClassificationType {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "focused")]
    Focused,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Importance {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "low")]
    Low,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BodyType {
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "text")]
    Text,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeZoneStandard {
    #[serde(rename = "iana")]
    Iana,
    #[serde(rename = "windows")]
    Windows,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecipientScopeType {
    #[serde(rename = "externalNonPartner")]
    ExternalNonPartner,
    #[serde(rename = "externalPartner")]
    ExternalPartner,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "true")]
    True,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MailTipsType {
    #[serde(rename = "recipientSuggestions")]
    RecipientSuggestions,
    #[serde(rename = "recipientScope")]
    RecipientScope,
    #[serde(rename = "moderationStatus")]
    ModerationStatus,
    #[serde(rename = "deliveryRestriction")]
    DeliveryRestriction,
    #[serde(rename = "maxMessageSize")]
    MaxMessageSize,
    #[serde(rename = "totalMemberCount")]
    TotalMemberCount,
    #[serde(rename = "externalMemberCount")]
    ExternalMemberCount,
    #[serde(rename = "customMailTip")]
    CustomMailTip,
    #[serde(rename = "mailboxFullStatus")]
    MailboxFullStatus,
    #[serde(rename = "automaticReplies")]
    AutomaticReplies,
    #[serde(rename = "true")]
    True,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationUniqueIdType {
    #[serde(rename = "bing")]
    Bing,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "directory")]
    Directory,
    #[serde(rename = "locationStore")]
    LocationStore,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "postalAddress")]
    PostalAddress,
    #[serde(rename = "localBusiness")]
    LocalBusiness,
    #[serde(rename = "restaurant")]
    Restaurant,
    #[serde(rename = "hotel")]
    Hotel,
    #[serde(rename = "streetAddress")]
    StreetAddress,
    #[serde(rename = "geoCoordinates")]
    GeoCoordinates,
    #[serde(rename = "businessAddress")]
    BusinessAddress,
    #[serde(rename = "homeAddress")]
    HomeAddress,
    #[serde(rename = "conferenceRoom")]
    ConferenceRoom,
    #[serde(rename = "default")]
    Default,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FreeBusyStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "workingElsewhere")]
    WorkingElsewhere,
    #[serde(rename = "oof")]
    Oof,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "tentative")]
    Tentative,
    #[serde(rename = "free")]
    Free,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttendeeType {
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "required")]
    Required,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExternalAudienceScope {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "contactsOnly")]
    ContactsOnly,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutomaticRepliesStatus {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "alwaysEnabled")]
    AlwaysEnabled,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DayOfWeek {
    #[serde(rename = "saturday")]
    Saturday,
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "thursday")]
    Thursday,
    #[serde(rename = "wednesday")]
    Wednesday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "sunday")]
    Sunday,
}
