use inflector::Inflector;
use std::convert::AsRef;

#[derive(
    AsRefStr,
    Copy,
    Clone,
    Eq,
    PartialEq,
    EnumString,
    EnumIter,
    Debug,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "camelCase")]
pub enum ResourceIdentity {
    Activities,
    AppCatalogs,
    Application,
    Applications,
    Attachments,
    AuditLogs,
    Buckets,
    Calendar,
    CalendarGroup,
    CalendarGroups,
    CalendarView,
    CalendarViews,
    Calendars,
    CallRecord,
    CallRecords,
    Calls,
    CertificateBasedAuthConfiguration,
    ChildFolders,
    Communications,
    ContactFolders,
    Contacts,
    ContentTypes,
    Contracts,
    Conversations,
    DataPolicyOperations,
    DeviceAppManagement,
    DeviceManagement,
    Devices,
    Directory,
    DirectoryObjects,
    DirectoryRoleTemplates,
    DirectoryRoles,
    DomainDnsRecords,
    Domains,
    Drive,
    Drives,
    Education,
    Event,
    Events,
    ExtendedProperties,
    GroupLifecyclePolicies,
    GroupSettingTemplates,
    GroupSettings,
    Groups,
    HistoryItems,
    Identity,
    InferenceClassification,
    InformationProtection,
    Insights,
    Instances,
    Invitations,
    Items,
    List,
    Lists,
    MailFolders,
    ManagedDevices,
    Me,
    Messages,
    Notebooks,
    Oauth2PermissionGrants,
    Onenote,
    OnlineMeetings,
    OrgContact,
    Organization,
    Outlook,
    Pages,
    ParentNotebook,
    ParentSection,
    ParentSectionGroup,
    Places,
    Planner,
    Plans,
    Policies,
    Posts,
    Reports,
    SchemaExtensions,
    SectionGroups,
    Sections,
    Security,
    ServicePrincipals,
    Sessions,
    Settings,
    Shares,
    Sites,
    SubscribedSkus,
    Subscriptions,
    Tasks,
    Teams,
    Teamwork,
    Threads,
    Users,
    Workbooks,
}

impl ToString for ResourceIdentity {
    fn to_string(&self) -> String {
        self.as_ref().to_camel_case()
    }
}

impl Default for ResourceIdentity {
    fn default() -> Self {
        ResourceIdentity::Me
    }
}

impl ResourceIdentity {
    pub fn enum_string(&self) -> String {
        format!("ResourceIdentity::{:#?}", self)
    }
}
