// GENERATED CODE

use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(AuditLogsRequest,);

impl<'a, Client> AuditLogsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get auditLogs",
        name: get_audit_log_root,
        response: serde_json::Value,
        path: "/auditLogs",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update auditLogs",
        name: update_audit_log_root,
        response: NoContent,
        path: "/auditLogs",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get directoryAudits from auditLogs",
        name: list_directory_audits,
        response: serde_json::Value,
        path: "/auditLogs/directoryAudits",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to directoryAudits for auditLogs",
        name: create_directory_audits,
        response: serde_json::Value,
        path: "/auditLogs/directoryAudits",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get directoryAudits from auditLogs",
        name: get_directory_audits,
        response: serde_json::Value,
        path: "/auditLogs/directoryAudits/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property directoryAudits in auditLogs",
        name: update_directory_audits,
        response: NoContent,
        path: "/auditLogs/directoryAudits/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete navigation property directoryAudits for auditLogs",
        name: delete_directory_audits,
        response: NoContent,
        path: "/auditLogs/directoryAudits/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get provisioning from auditLogs",
        name: list_provisioning,
        response: serde_json::Value,
        path: "/auditLogs/provisioning",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to provisioning for auditLogs",
        name: create_provisioning,
        response: serde_json::Value,
        path: "/auditLogs/provisioning",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get provisioning from auditLogs",
        name: get_provisioning,
        response: serde_json::Value,
        path: "/auditLogs/provisioning/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property provisioning in auditLogs",
        name: update_provisioning,
        response: NoContent,
        path: "/auditLogs/provisioning/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete navigation property provisioning for auditLogs",
        name: delete_provisioning,
        response: NoContent,
        path: "/auditLogs/provisioning/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get restrictedSignIns from auditLogs",
        name: list_restricted_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/restrictedSignIns",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to restrictedSignIns for auditLogs",
        name: create_restricted_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/restrictedSignIns",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get restrictedSignIns from auditLogs",
        name: get_restricted_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/restrictedSignIns/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property restrictedSignIns in auditLogs",
        name: update_restricted_sign_ins,
        response: NoContent,
        path: "/auditLogs/restrictedSignIns/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete navigation property restrictedSignIns for auditLogs",
        name: delete_restricted_sign_ins,
        response: NoContent,
        path: "/auditLogs/restrictedSignIns/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get signIns from auditLogs",
        name: list_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/signIns",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to signIns for auditLogs",
        name: create_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/signIns",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get signIns from auditLogs",
        name: get_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/signIns/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property signIns in auditLogs",
        name: update_sign_ins,
        response: NoContent,
        path: "/auditLogs/signIns/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete navigation property signIns for auditLogs",
        name: delete_sign_ins,
        response: NoContent,
        path: "/auditLogs/signIns/{{id}}",
        params: 1,
        has_body: false
    });
}
