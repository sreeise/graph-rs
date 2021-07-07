use crate::parser::filter::{Filter, FilterIgnore};
use graph_core::resource::ResourceIdentity;

pub fn get_path_filters(resource_identity: ResourceIdentity) -> Vec<Filter> {
    match resource_identity {
        ResourceIdentity::Buckets => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["buckets/{plannerBucket-id}/tasks/"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "calendarGroup",
                    "instances",
                    "calendarView",
                    "events",
                    "/attachments/",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::CalendarGroups => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "/calendar/",
                    "events",
                    "attachments",
                    "instances",
                    "calendarView",
                    "calendarPermissions",
                    "getSchedule",
                    "allowedCalendarSharingRoles",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::CalendarView | ResourceIdentity::CalendarViews => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "/calendar/calendarView",
                    "events",
                    "/calendar/calendarPermissions",
                    "/calendar/getSchedule",
                    "instances",
                    "/attachments/",
                    "/calendar/allowedCalendarSharingRoles",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::CallRecords => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["sessions/{session-id}"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::Communications => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "callRecords/{callRecord-id}/",
                    "calls/{call-id}/",
                    "calls/logTeleconferenceDeviceQuality",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Conversations => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["/threads/"].iter().map(|s| s.to_string()).collect(),
            ))]
        }
        ResourceIdentity::ChildFolders => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["/move"].iter().map(|s| s.to_string()).collect(),
            ))]
        }
        ResourceIdentity::ContactFolders => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "childFolders",
                    "contactFolders/{contactFolder-id}/contacts/",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Drives | ResourceIdentity::Drive => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["/list/", "versions", "items"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::Events => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "/calendar/calendarView",
                    "instances",
                    "calendar/events",
                    "/calendar/getSchedule",
                    "calendarPermissions",
                    "/attachments/",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Lists => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["contentTypes", "items"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::MailFolders => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["/move", "messages", "childFolders"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::Messages => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["/move", "/attachments/"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::Onenote => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "sections/{onenoteSection-id}",
                    "sectionGroups/{sectionGroup-id}",
                    "pages/{onenotePage-id}",
                    "notebooks/{notebook-id}",
                    "getNotebookFromWebUrl",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Pages => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "sections/{onenoteSection-id}",
                    "sectionGroups/{sectionGroup-id}",
                    "notebooks/{notebook-id}",
                    "/parentNotebook/",
                    "/parentSection/",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Notebooks => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "sections/{onenoteSection-id}",
                    "sectionGroups/{sectionGroup-id}",
                    "pages/{onenotePage-id}",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::SectionGroups => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "sections/{onenoteSection-id}",
                    "pages/{onenotePage-id}",
                    "notebooks/{notebook-id}",
                    "/sectionGroups/{sectionGroup-id}/sectionGroups/{sectionGroup-id}",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Sections => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "pages/{onenotePage-id}",
                    "sectionGroups/{sectionGroup-id}",
                    "notebooks/{notebook-id}",
                    "/parentSectionGroup/",
                    "/parentNotebook/",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::ParentNotebook => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "/parentNotebook/sectionGroups/{sectionGroup-id}",
                    "/parentNotebook/sections/{onenoteSection-id}",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::ParentSectionGroup => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "/parentSectionGroup/parentNotebook/",
                    "/parentSectionGroup/sectionGroups/",
                    "/parentSectionGroup/sections/",
                    "/parentSectionGroup/parentSectionGroup",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::ParentSection => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "/parentSection/pages/",
                    "/parentSectionGroup/",
                    "/parentNotebook/",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Plans => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["/buckets/", "/tasks/"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::Planner => vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
            vec![
                "plans/{plannerPlan-id}/",
                "buckets/{plannerBucket-id}/",
                "tasks/{plannerTask-id}/",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        ))],
        ResourceIdentity::Posts => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["/attachments/"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::Me => vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
            vec![
                "activities",
                "historyItems",
                "contacts",
                "onlineMeetings",
                "outlook",
                "/settings/",
                "calendarGroup",
                "calendars",
                "calendar",
                "calendarView",
                "contactFolder",
                "events",
                "inferenceClassification",
                "insights",
                "instances",
                "mailFolders",
                "managedDevices",
                "messages",
                "onenote",
                "planner",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        ))],
        ResourceIdentity::Sites => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["onenote", "contentTypes", "lists"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ))]
        }
        ResourceIdentity::Users => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "activities",
                    "historyItems",
                    "contacts",
                    "onlineMeetings",
                    "outlook",
                    "/settings/",
                    "calendarGroup",
                    "calendars",
                    "calendar",
                    "calendarView",
                    "contactFolder",
                    "events",
                    "inferenceClassification",
                    "insights",
                    "instances",
                    "mailFolders",
                    "managedDevices",
                    "messages",
                    "onenote",
                    "planner",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Groups => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec![
                    "/calendarGroup/",
                    "/calendars/",
                    "/calendar/",
                    "/calendarView/",
                    "/events/",
                    "/onenote/",
                    "/planner/",
                    "/conversations/",
                    "/threads/",
                    "/conversations/",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ))]
        }
        ResourceIdentity::Threads => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
                vec!["/posts/"].iter().map(|s| s.to_string()).collect(),
            ))]
        }
        _ => vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(
            vec![
                "singleValueExtendedProperties",
                "multiValueExtendedProperties",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        ))],
    }
}
