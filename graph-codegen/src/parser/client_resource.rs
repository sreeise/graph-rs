use crate::parser::filter::Filter;
use graph_core::resource::ResourceIdentity;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClientResource {
    Main {
        modifier: String,
    },
    Secondary {
        start_filter: Filter,
        modifier: String,
    },
    MainResourceIdentity {
        modifier: ResourceIdentity,
    },
    SecondaryResourceIdentity {
        start_filter: Filter,
        modifier: ResourceIdentity,
    },
}

impl Default for ClientResource {
    fn default() -> Self {
        ClientResource::Main {
            modifier: Default::default(),
        }
    }
}

impl TryFrom<ResourceIdentity> for ClientResource {
    type Error = ();

    fn try_from(resource_identity: ResourceIdentity) -> Result<Self, Self::Error> {
        match resource_identity {
            ResourceIdentity::Activities => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/activities".into()),
                modifier: "activities".to_string(),
            }),
            ResourceIdentity::AppCatalogs => Ok(ClientResource::Main {
                modifier: "appCatalogs".to_string(),
            }),
            ResourceIdentity::Applications => Ok(ClientResource::Main {
                modifier: "applications".to_string(),
            }),
            ResourceIdentity::AuditLogs => Ok(ClientResource::Main {
                modifier: "auditLogs".to_string(),
            }),
            ResourceIdentity::Attachments => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/calendar/events/{event-id}/attachments".into(),
                ),
                modifier: "attachments".to_string(),
            }),
            ResourceIdentity::Buckets => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/planner/buckets".into()),
                modifier: "buckets".to_string(),
            }),
            ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
                Ok(ClientResource::SecondaryResourceIdentity {
                    start_filter: Filter::PathStartsWith("/users/{user-id}/calendar".into()),
                    modifier: ResourceIdentity::Calendar,
                })
            }
            ResourceIdentity::CalendarGroups => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/users".into()),
                modifier: "calendarGroups".to_string(),
            }),
            ResourceIdentity::CalendarView | ResourceIdentity::CalendarViews => {
                Ok(ClientResource::Secondary {
                    start_filter: Filter::PathStartsWith("/me/calendarView".into()),
                    modifier: "calendarView".to_string(),
                })
            }
            ResourceIdentity::CallRecords => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/communications/callRecords".into()),
                modifier: "callRecords".to_string(),
            }),
            ResourceIdentity::CertificateBasedAuthConfiguration => Ok(ClientResource::Main {
                modifier: "certificateBasedAuthConfiguration".to_string(),
            }),
            ResourceIdentity::ContactFolders => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/contactFolders".into()),
                modifier: "contactFolders".to_string(),
            }),
            ResourceIdentity::Contacts => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/contacts".into()),
                modifier: "contacts".to_string(),
            }),
            ResourceIdentity::ContentTypes => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/sites/{site-id}/contentTypes".into()),
                modifier: "contentTypes".to_string(),
            }),
            ResourceIdentity::Conversations => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/groups/{group-id}/conversations".into()),
                modifier: "conversations".to_string(),
            }),
            ResourceIdentity::Calls => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/communications/calls".into()),
                modifier: "calls".to_string(),
            }),
            ResourceIdentity::Communications => Ok(ClientResource::Main {
                modifier: "communications".to_string(),
            }),
            ResourceIdentity::DataPolicyOperations => Ok(ClientResource::Main {
                modifier: "dataPolicyOperations".to_string(),
            }),
            ResourceIdentity::Drive | ResourceIdentity::Drives => Ok(ClientResource::Main {
                modifier: "drive".to_string(),
            }),
            ResourceIdentity::Domains => Ok(ClientResource::Main {
                modifier: "domains".to_string(),
            }),
            ResourceIdentity::Education => Ok(ClientResource::Main {
                modifier: ResourceIdentity::Education.to_string(),
            }),
            ResourceIdentity::Events => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/users".into()),
                modifier: "events".to_string(),
            }),
            ResourceIdentity::Groups => Ok(ClientResource::Main {
                modifier: "groups".to_string(),
            }),
            ResourceIdentity::InferenceClassification => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/inferenceClassification".into()),
                modifier: "inferenceClassification".to_string(),
            }),
            ResourceIdentity::Insights => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/insights".into()),
                modifier: "insights".to_string(),
            }),
            ResourceIdentity::Instances => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/calendarView/{event-id}/instances".into(),
                ),
                modifier: "instances".to_string(),
            }),
            ResourceIdentity::Items => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/sites/{site-id}/lists/{list-id}/items".into(),
                ),
                modifier: "items".to_string(),
            }),
            ResourceIdentity::Lists => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/sites".into()),
                modifier: "lists".to_string(),
            }),
            ResourceIdentity::ManagedDevices => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/managedDevices".into()),
                modifier: "managedDevices".to_string(),
            }),
            ResourceIdentity::MailFolders => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/mailFolders".into()),
                modifier: "mailFolders".to_string(),
            }),
            ResourceIdentity::ChildFolders => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/mailFolders/{mailFolder-id}/childFolders".into(),
                ),
                modifier: "childFolders".to_string(),
            }),
            ResourceIdentity::Messages => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/messages".into()),
                modifier: "messages".to_string(),
            }),
            ResourceIdentity::Me => Ok(ClientResource::Main {
                modifier: "me".to_string(),
            }),
            ResourceIdentity::Outlook => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/outlook".into()),
                modifier: "outlook".to_string(),
            }),
            ResourceIdentity::Onenote => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote".into()),
                modifier: "onenote".to_string(),
            }),
            ResourceIdentity::Notebooks => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote/notebook".into()),
                modifier: "notebooks".to_string(),
            }),
            ResourceIdentity::Sections => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote/sections".into()),
                modifier: "sections".to_string(),
            }),
            ResourceIdentity::SectionGroups => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote/sectionGroups".into()),
                modifier: "sectionGroups".to_string(),
            }),
            ResourceIdentity::Pages => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote/pages".into()),
                modifier: "pages".to_string(),
            }),
            ResourceIdentity::ParentNotebook => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/onenote/sections/{onenoteSection-id}/parentNotebook".into(),
                ),
                modifier: "parentNotebook".to_string(),
            }),
            ResourceIdentity::ParentSectionGroup => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/onenote/sections/{onenoteSection-id}/parentSectionGroup".into(),
                ),
                modifier: "parentSectionGroup".to_string(),
            }),
            ResourceIdentity::ParentSection => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/onenote/pages/{onenotePage-id}/parentSection".into(),
                ),
                modifier: "parentSection".to_string(),
            }),
            ResourceIdentity::Planner => Ok(ClientResource::Main {
                modifier: "planner".to_string(),
            }),
            ResourceIdentity::Posts => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/groups/{group-id}/threads/{conversationThread-id}/posts".into(),
                ),
                modifier: "posts".to_string(),
            }),
            ResourceIdentity::Plans => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/planner/plans".into()),
                modifier: "plans".to_string(),
            }),
            ResourceIdentity::Sites => Ok(ClientResource::Main {
                modifier: "sites".to_string(),
            }),
            ResourceIdentity::Sessions => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/communications/callRecords/{callRecord-id}/sessions".into(),
                ),
                modifier: "sessions".to_string(),
            }),
            ResourceIdentity::Settings => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/settings".into()),
                modifier: "settings".to_string(),
            }),
            ResourceIdentity::Users => Ok(ClientResource::Main {
                modifier: "users".to_string(),
            }),
            ResourceIdentity::Tasks => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/planner/tasks".into()),
                modifier: "tasks".to_string(),
            }),
            ResourceIdentity::Threads => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/groups/{group-id}/threads".into()),
                modifier: "threads".to_string(),
            }),
            _ => Err(()),
        }
    }
}
