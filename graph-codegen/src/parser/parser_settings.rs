use crate::{
    builder::ClientLinkSettings,
    parser::{
        filter::{
            Filter, FilterIgnore, ModifierMap, ResourceIdentityModifier, SecondaryModifierMap,
        },
        settings::{
            get_client_link_settings, get_custom_requests, get_imports, get_path_filters,
            get_target_map_modifier,
        },
        DirectoryModFile, RequestSet,
    },
};
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct ParserSettings;

impl ParserSettings {
    /// Imports that won't be added from parsing and need to be manually added.
    pub fn imports(resource_identity: ResourceIdentity) -> BTreeSet<String> {
        let mut vec: Vec<&'static str> = vec![
            "crate::client::Graph",
            "graph_http::IntoResponse",
            "reqwest::Method",
        ];
        vec.extend(get_imports(resource_identity));
        vec.sort_unstable();
        let mut set: BTreeSet<String> = BTreeSet::new();
        set.extend(vec.into_iter().map(|s| s.to_string()));
        set
    }

    pub fn default_path_filters() -> Vec<Filter> {
        vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
            "singleValueExtendedProperties".into(),
            "multiValueExtendedProperties".into(),
        ]))]
    }

    // Filters for clients when the parsing and generation happens. Some clients,
    // such as Users and Groups use the same path for resources like calendars, and
    // so we generate a separate module for calendars. In cases like these, Users
    // and Groups will use the same calendar module. This cuts down on the size
    // of the crate and makes it easier to generate clients that use the same
    // resources.
    pub fn path_filters(resource_identity: ResourceIdentity) -> Vec<Filter> {
        get_path_filters(resource_identity)
    }

    pub fn custom_register_clients(resource_identity: ResourceIdentity) -> Option<String> {
        match resource_identity {
            ResourceIdentity::Drives => Some(
                "register_client!(
                    () DrivesRequest,
                    drive_item => \"drive/items\", \"items\", ResourceIdentity::Drives,
                    drive_root => \"drive\", \"\", ResourceIdentity::Drives,
                    drive_root_path => \"drive/root\", \"root\", ResourceIdentity::Drives,
                );"
                .to_string(),
            ),
            _ => None,
        }
    }

    pub fn custom_methods(
        resource_identity: ResourceIdentity,
    ) -> Option<HashMap<String, RequestSet>> {
        get_custom_requests(resource_identity)
    }

    // Secondary links and modifiers. These are api's that are used multiple times
    // such as calendars and calendar groups where we might have two resources
    // such as groups and users with the same ending path:
    // groups/{group-id}/calendars/{calendar-id}
    // users/{user-id}/calendars/{calendar-id}
    // We do not want each api implementation to have its own calendar struct
    // and methods to prevent repeated code. So we separate these out here
    // and add a link between them. We change the operation map of the
    // requests so they are generated within the correct client.
    pub fn secondary_modifier_map(resource_identity: ResourceIdentity) -> SecondaryModifierMap {
        let mut map = SecondaryModifierMap::with_capacity(5);

        match resource_identity {
            ResourceIdentity::Activities => {
                map.insert_operation_mapping("me.activities", "activities");
            }
            ResourceIdentity::Attachments => {
                map.insert_operation_mapping("groups.calendar.events.attachments", "attachments");
            }
            ResourceIdentity::Buckets => {
                map.insert_operation_mapping("planner.buckets", "buckets");
            }
            ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
                map.insert_operation_mapping("users.calendar", "calendar");
                map.insert_operation_mapping("users.calendars", "calendars");
            }
            ResourceIdentity::CalendarGroups => {
                map.insert_operation_map_and_id("users.calendarGroups", "calendarGroups");
            }
            ResourceIdentity::CalendarView => {
                map.insert_operation_map_and_id("me.calendarView", "calendarView");
            }
            ResourceIdentity::Calls => {
                map.insert_operation_mapping("communications.calls", "calls");
            }
            ResourceIdentity::CallRecords => {
                map.insert_operation_mapping("communications.callRecords", "callRecords");
            }
            ResourceIdentity::ContactFolders => {
                map.insert_operation_mapping("me.contactFolders", "contactFolders");
            }
            ResourceIdentity::Contacts => {
                map.insert_operation_mapping("me.contacts", "contacts");
            }
            ResourceIdentity::ContentTypes => {
                map.insert_operation_mapping("sites.contentTypes", "contentTypes");
            }
            ResourceIdentity::Conversations => {
                map.insert_operation_mapping("groups.conversations", "conversations");
            }
            ResourceIdentity::ChildFolders => {
                map.insert_operation_mapping("me.mailFolders.childFolders", "childFolders");
            }
            ResourceIdentity::Events => {
                map.insert_operation_id("users.events", "events");
            }
            ResourceIdentity::InferenceClassification => {
                map.insert_operation_mapping(
                    "me.inferenceClassification",
                    "inferenceClassification",
                );
            }
            ResourceIdentity::Instances => {
                map.insert_operation_mapping("me.calendarView.instances", "instances");
            }
            ResourceIdentity::Items => {
                map.insert_operation_mapping("sites.lists.items", "items");
            }
            ResourceIdentity::Lists => {
                map.insert_operation_mapping("sites.lists", "lists");
            }
            ResourceIdentity::MailFolders => {
                map.insert_operation_mapping("me.mailFolders", "mailFolders");
            }
            ResourceIdentity::Messages => {
                map.insert_operation_mapping("me.messages", "messages");
            }
            ResourceIdentity::Me => {
                map.insert_operation_mapping("me.user", "me");
            }
            ResourceIdentity::Outlook => {
                map.insert_operation_mapping("me.outlook", "outlook");
            }
            ResourceIdentity::Plans => {
                map.insert_operation_mapping("planner.plans", "plans");
            }
            ResourceIdentity::Settings => {
                map.insert_operation_mapping("me.settings", "settings");
            }
            ResourceIdentity::Notebooks => {
                map.insert_operation_mapping("me.onenote.notebooks", "notebooks");
            }
            ResourceIdentity::SectionGroups => {
                map.insert_operation_mapping("me.onenote.sectionGroups", "sectionGroups");
            }
            ResourceIdentity::Sections => {
                map.insert_operation_mapping("me.onenote.sections", "sections");
            }
            ResourceIdentity::Sessions => {
                map.insert_operation_mapping("communications.callRecords.sessions", "sessions");
            }
            ResourceIdentity::Pages => {
                map.insert_operation_mapping("me.onenote.pages", "pages");
            }
            ResourceIdentity::ParentSection => {
                map.insert_operation_mapping("me.onenote.pages.parentSection", "parentSection");
            }
            ResourceIdentity::Posts => {
                map.insert_operation_mapping("groups.threads.posts", "posts");
            }
            ResourceIdentity::Tasks => {
                map.insert_operation_mapping("planner.tasks", "tasks");
            }
            ResourceIdentity::Threads => {
                map.insert_operation_mapping("groups.threads", "threads");
            }
            _ => {}
        }

        map
    }

    pub fn directory_mod(resource_identity: ResourceIdentity) -> Option<DirectoryModFile> {
        match resource_identity {
            ResourceIdentity::Drive | ResourceIdentity::Drives => Some(DirectoryModFile {
                resource_identity,
                mod_name: "manual_request".into(),
                use_all: true,
            }),
            ResourceIdentity::Pages => Some(DirectoryModFile {
                resource_identity,
                mod_name: "manual_request".into(),
                use_all: true,
            }),
            ResourceIdentity::ChildFolders => Some(DirectoryModFile {
                resource_identity,
                mod_name: "manual_request".into(),
                use_all: true,
            }),
            _ => None,
        }
    }

    pub fn resource_identity_modifier(
        resource_identity: ResourceIdentity,
    ) -> Option<ResourceIdentityModifier> {
        match resource_identity {
            ResourceIdentity::Applications
            | ResourceIdentity::Attachments
            | ResourceIdentity::Buckets
            | ResourceIdentity::Calendars
            | ResourceIdentity::CalendarGroups
            | ResourceIdentity::CalendarView
            | ResourceIdentity::CallRecords
            | ResourceIdentity::Calls
            | ResourceIdentity::ChildFolders
            | ResourceIdentity::ContactFolders
            | ResourceIdentity::Contacts
            | ResourceIdentity::ContentTypes
            | ResourceIdentity::Conversations
            | ResourceIdentity::Drives
            | ResourceIdentity::Domains
            | ResourceIdentity::Events
            | ResourceIdentity::Groups
            | ResourceIdentity::Instances
            | ResourceIdentity::Items
            | ResourceIdentity::Lists
            | ResourceIdentity::MailFolders
            | ResourceIdentity::Messages
            | ResourceIdentity::ManagedDevices
            | ResourceIdentity::Notebooks
            | ResourceIdentity::Onenote
            | ResourceIdentity::Pages
            | ResourceIdentity::Posts
            | ResourceIdentity::Sections
            | ResourceIdentity::Plans
            | ResourceIdentity::SectionGroups
            | ResourceIdentity::Sessions
            | ResourceIdentity::Sites
            | ResourceIdentity::Teams
            | ResourceIdentity::Threads
            | ResourceIdentity::Users
            | ResourceIdentity::Tasks
            | ResourceIdentity::Workbooks => {
                Some(ResourceIdentityModifier::new(resource_identity, false))
            }
            ResourceIdentity::Calendar => {
                Some(ResourceIdentityModifier::new(resource_identity, true))
            }
            _ => None,
        }
    }

    /// These are clients that will have {{RID}} in the path of requests
    /// Ident clients are also those that have a ResourceUrlModifier.
    pub fn is_registered_ident_client(resource_identity: ResourceIdentity) -> bool {
        matches!(
            resource_identity,
            ResourceIdentity::Applications
                | ResourceIdentity::Attachments
                | ResourceIdentity::Drive
                | ResourceIdentity::Drives
                | ResourceIdentity::Calendars
                | ResourceIdentity::CalendarGroups
                | ResourceIdentity::CalendarView
                | ResourceIdentity::CallRecords
                | ResourceIdentity::Calls
        )
    }

    pub fn client_link_settings(
        resource_identity: ResourceIdentity,
    ) -> BTreeMap<String, BTreeSet<ClientLinkSettings>> {
        get_client_link_settings(resource_identity)
    }

    // Modifiers that need to be explicitly declared.
    // The struct names for clients are generated based on the operation id
    // which is also modified when the clients are generated. This can result
    // in naming conflicts that is fixed by these modifiers.
    pub fn target_modifiers(resource_identity: ResourceIdentity) -> ModifierMap {
        get_target_map_modifier(resource_identity)
    }

    pub fn links_override(resource_identity: ResourceIdentity) -> HashMap<String, Vec<String>> {
        let mut links_override = HashMap::new();
        if resource_identity == ResourceIdentity::Directory {
            links_override.insert(
                "directory".to_string(),
                [
                    "directoryRoles",
                    "directoryObjects",
                    "directoryRoleTemplates",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            );
        }

        links_override
    }
}
