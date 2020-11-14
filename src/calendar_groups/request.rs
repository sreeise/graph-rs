use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(CalendarGroupsRequest,);
register_client!(CalendarsRequest,);
register_client!(EventsRequest,);
register_client!(InstancesRequest,);

impl<'a, Client> CalendarGroupsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn calendars(&self) -> CalendarsRequest<'a, Client> {
        CalendarsRequest::new(&self.client)
    }
    get!({
        doc: "# Get calendars from users",
        name: list_calendars,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendars for users",
        name: create_calendars,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendarGroups from users",
        name: list_calendar_groups,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarGroups for users",
        name: create_calendar_groups,
        response: serde_json::Value,
        path: "/calendarGroups",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendars from users",
        name: get_calendars,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendars in users",
        name: update_calendars,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get calendarGroups from users",
        name: get_calendar_groups,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarGroups in users",
        name: update_calendar_groups,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> CalendarsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn events(&self) -> EventsRequest<'a, Client> {
        EventsRequest::new(&self.client)
    }
    get!({
        doc: "# Get events from users",
        name: get_events,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property events in users",
        name: update_events,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}",
        params: 3,
        has_body: true
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/getSchedule",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get calendarPermissions from users",
        name: get_calendar_permissions,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/calendarPermissions/{{id3}}",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarPermissions in users",
        name: update_calendar_permissions,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/calendarPermissions/{{id3}}",
        params: 3,
        has_body: true
    });
    get!({
        doc: "# Get calendarPermissions from users",
        name: list_calendar_permissions,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/calendarPermissions",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarPermissions for users",
        name: create_calendar_permissions,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/calendarPermissions",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get events from users",
        name: list_events,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to events for users",
        name: create_events,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> EventsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn instances(&self) -> InstancesRequest<'a, Client> {
        InstancesRequest::new(&self.client)
    }
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/delta()",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/decline",
        params: 3,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/extensions",
        params: 3,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/extensions",
        params: 3,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/dismissReminder",
        params: 3,
        has_body: false
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/accept",
        params: 3,
        has_body: true
    });
    get!({
        doc: "# Get instances from users",
        name: get_instances,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances/{{id4}}",
        params: 4,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property instances in users",
        name: update_instances,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances/{{id4}}",
        params: 4,
        has_body: true
    });
    get!({
        doc: "# Get calendar from users",
        name: get_calendar,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/calendar",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendar in users",
        name: update_calendar,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/calendar",
        params: 3,
        has_body: true
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/calendar/getSchedule",
        params: 3,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/extensions/{{id4}}",
        params: 4,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/extensions/{{id4}}",
        params: 4,
        has_body: true
    });
    get!({
        doc: "# Get instances from users",
        name: list_instances,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances",
        params: 3,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to instances for users",
        name: create_instances,
        response: serde_json::Value,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances",
        params: 3,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/snoozeReminder",
        params: 3,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/tentativelyAccept",
        params: 3,
        has_body: true
    });
}

impl<'a, Client> InstancesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances/{{id4}}/decline",
        params: 4,
        has_body: true
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances/{{id4}}/accept",
        params: 4,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances/{{id4}}/snoozeReminder",
        params: 4,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances/{{id4}}/tentativelyAccept",
        params: 4,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances/delta()",
        params: 3,
        has_body: false
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{id}}/calendars/{{id2}}/events/{{id3}}/instances/{{id4}}/dismissReminder",
        params: 4,
        has_body: false
    });
}
