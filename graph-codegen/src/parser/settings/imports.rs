use graph_core::resource::ResourceIdentity;

pub fn get_imports(resource_identity: ResourceIdentity) -> Vec<&'static str> {
    match resource_identity {
        ResourceIdentity::Buckets => vec![
            "crate::tasks::{TaskRequest, TasksRequest}",
            "crate::core::ResourceIdentity",
        ],
        ResourceIdentity::Calendar | ResourceIdentity::Calendars => vec![
            "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::core::ResourceIdentity",
            // TODO: Handlebars should be imported by the builder. Figure out why this is not happening.
            "handlebars::*",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::CalendarGroup | ResourceIdentity::CalendarGroups => vec![
            "crate::calendar::{CalendarRequest, CalendarsRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::core::ResourceIdentity",
        ],
        ResourceIdentity::CalendarView => vec![
            "crate::instances::{InstanceRequest, InstancesRequest}",
            "crate::calendar::CalendarRequest",
            "crate::core::ResourceIdentity",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::Calls => vec![
            "crate::core::ResourceIdentity",
        ],
        ResourceIdentity::CallRecords => vec![
            "crate::core::ResourceIdentity",
            "crate::sessions::{SessionRequest, SessionsRequest}",
        ],
        ResourceIdentity::Communications => vec![
            "crate::core::ResourceIdentity",
            "crate::call_records::{CallRecordRequest, CallRecordsRequest}",
            "crate::calls::{CallRequest, CallsRequest}",
        ],
        ResourceIdentity::Contacts => vec![
            "crate::core::ResourceIdentity",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::ContactFolders => vec![
            "crate::core::ResourceIdentity",
            "crate::extended_properties::ExtendedPropertiesRequest"
        ],
        ResourceIdentity::Conversations => vec![
            "crate::core::ResourceIdentity",
            "crate::threads::{ThreadRequest, ThreadsRequest}",
        ],
        ResourceIdentity::Drive | ResourceIdentity::Drives => vec![
            "std::path::Path",
            "crate::core::ResourceIdentity",
            "crate::items::{ItemRequest, ItemsRequest}",
            "crate::lists::{ListRequest, ListsRequest}",
            "graph_http::types::DeltaPhantom",
            // TODO: Handlebars should be imported by the builder. Figure out why this is not happening.
            "handlebars::*",
        ],
        ResourceIdentity::Domains => vec!["crate::core::ResourceIdentity"],
        ResourceIdentity::Lists => vec![
            "crate::content_types::{ContentTypeRequest, ContentTypesRequest}",
            "crate::items::{ItemRequest, ItemsRequest}",
        ],
        ResourceIdentity::Events => vec![
            "crate::calendar::CalendarRequest",
            "crate::instances::{InstanceRequest, InstancesRequest}",
            "crate::core::ResourceIdentity",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::Sites => vec![
            "crate::core::ResourceIdentity",
            "crate::content_types::{ContentTypeRequest, ContentTypesRequest}",
            "crate::lists::{ListRequest, ListsRequest}",
            "crate::drive::DrivesRequest",
            "crate::onenote::OnenoteRequest",
        ],
        ResourceIdentity::Onenote => vec![
            "crate::core::ResourceIdentity",
            "crate::notebooks::{NotebookRequest, NotebooksRequest}",
            "crate::pages::{PagesRequest, PageRequest}",
            "crate::sections::{SectionRequest, SectionsRequest}",
            "crate::section_groups::{SectionGroupRequest, SectionGroupsRequest}",
        ],
        ResourceIdentity::Pages => vec![
            "crate::core::ResourceIdentity",
            "crate::parent_notebook::ParentNotebookRequest",
            "crate::parent_section::ParentSectionRequest",
            "graph_http::{BlockingDownload, AsyncDownload, BlockingHttpClient, AsyncHttpClient, RequestClient}",
            "std::path::Path",
        ],
        ResourceIdentity::Notebooks => vec![
            "crate::core::ResourceIdentity",
            "crate::sections::SectionsRequest",
            "crate::section_groups::SectionGroupsRequest",
        ],
        ResourceIdentity::SectionGroups => vec![
            "crate::core::ResourceIdentity",
            "crate::sections::SectionsRequest",
        ],
        ResourceIdentity::Sections => vec![
            "crate::core::ResourceIdentity",
            "crate::pages::PagesRequest",
            "crate::section_groups::SectionGroupsRequest",
            "crate::parent_notebook::ParentNotebookRequest",
            "crate::parent_section_group::ParentSectionGroupRequest",
        ],
        ResourceIdentity::ParentNotebook => vec![
            "crate::core::ResourceIdentity",
            "crate::sections::SectionsRequest",
            "crate::section_groups::SectionGroupsRequest",
        ],
        ResourceIdentity::ParentSectionGroup => vec![
            "crate::core::ResourceIdentity",
            "crate::sections::SectionsRequest",
            "crate::section_groups::SectionGroupsRequest",
            "crate::parent_notebook::ParentNotebookRequest",
        ],
        ResourceIdentity::ParentSection => vec![
            "crate::core::ResourceIdentity",
            "crate::pages::PagesRequest",
            "crate::parent_section_group::ParentSectionGroupRequest",
            "crate::parent_notebook::ParentNotebookRequest",
        ],
        ResourceIdentity::Plans => vec![
            "crate::buckets::{BucketRequest, BucketsRequest}",
            "crate::tasks::{TaskRequest, TasksRequest}",
            "crate::core::ResourceIdentity",
        ],
        ResourceIdentity::Posts => vec![
            "crate::core::ResourceIdentity",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::ManagedDevices => vec!["crate::core::ResourceIdentity"],
        ResourceIdentity::MailFolders => vec![
            "crate::core::ResourceIdentity",
            "crate::messages::{MessageRequest, MessagesRequest}",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::Messages => vec![
            "crate::core::ResourceIdentity",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::Me => vec![
            "crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest}",
            "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
            "crate::calendar::{CalendarRequest, CalendarsRequest}",
            "crate::education::{MeRequest as EducationMeRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::managed_devices::{ManagedDeviceRequest, ManagedDevicesRequest}",
            "crate::contact_folders::{ContactFolderRequest, ContactFoldersRequest}",
            "crate::messages::{MessageRequest, MessagesRequest}",
            "crate::mail_folders::{MailFolderRequest, MailFoldersRequest}",
            "crate::insights::InsightsRequest",
            "crate::inference_classification::InferenceClassificationRequest",
            "crate::activities::ActivitiesRequest",
            "crate::settings::SettingsRequest",
            "crate::outlook::OutlookRequest",
            "crate::drive::DrivesRequest",
            "crate::onenote::OnenoteRequest",
            "crate::core::ResourceIdentity",
            "crate::contacts::{ContactRequest, ContactsRequest}",

        ],
        ResourceIdentity::Sessions => vec!["crate::core::ResourceIdentity"],
        ResourceIdentity::Users => vec![
            "crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest}",
            "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
            "crate::calendar::{CalendarRequest, CalendarsRequest}",
            "crate::education::{UsersRequest as EducationUsersRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::managed_devices::{ManagedDeviceRequest, ManagedDevicesRequest}",
            "crate::contact_folders::{ContactFolderRequest, ContactFoldersRequest}",
            "crate::messages::{MessageRequest, MessagesRequest}",
            "crate::mail_folders::{MailFolderRequest, MailFoldersRequest}",
            "crate::insights::InsightsRequest",
            "crate::inference_classification::InferenceClassificationRequest",
            "crate::activities::ActivitiesRequest",
            "crate::settings::SettingsRequest",
            "crate::outlook::OutlookRequest",
            "crate::drive::DrivesRequest",
            "crate::onenote::OnenoteRequest",
            "crate::core::ResourceIdentity",
            "crate::contacts::{ContactRequest, ContactsRequest}",
        ],
        ResourceIdentity::Groups => vec![
            "crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest}",
            "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
            "crate::calendar::{CalendarRequest, CalendarsRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::drive::DrivesRequest",
            "crate::onenote::OnenoteRequest",
            "crate::threads::{ThreadRequest, ThreadsRequest}",
            "crate::conversations::{ConversationRequest, ConversationsRequest}",
            "crate::planner::PlannerRequest",
            "crate::core::ResourceIdentity",
        ],
        ResourceIdentity::Tasks => vec![
            "crate::core::ResourceIdentity",
        ],
        ResourceIdentity::Threads => vec![
            "crate::core::ResourceIdentity",
            "crate::posts::{PostRequest, PostsRequest}",
        ],
        _ => vec![],
    }
}
