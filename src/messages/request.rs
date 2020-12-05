use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use graph_error::GraphFailure;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use graph_http::UploadSessionClient;
use handlebars::*;
use reqwest::Method;
use std::path::Path;

register_client!(AttachmentsRequest,);
register_client!(MessageRequest,);
register_client!(MessagesRequest, ());

impl<'a, Client> AttachmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createUploadSession",
        name: create_upload_session,
        path: "/messages/{{RID}}/attachments/createUploadSession",
        params: 0,
        has_body: true,
        upload_session: true
    });
}

impl<'a, Client> MessageRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> MessagesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Messages);
        MessagesRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get messages from me",
        name: list_messages,
        response: Collection<serde_json::Value>,
        path: "/messages",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to messages for me",
        name: create_messages,
        response: serde_json::Value,
        path: "/messages",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/messages/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> MessagesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn attachments(&self) -> AttachmentsRequest<'a, Client> {
        AttachmentsRequest::new(self.client)
    }
    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
    }
    get!({
        doc: "# Get messages from me",
        name: get_messages,
        response: serde_json::Value,
        path: "/messages/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property messages in me",
        name: update_messages,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}",
        params: 0,
        has_body: true
    });
    delete!({
        name: delete_messages,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}",
        params: 0,
        has_body: false
    });
    get!({
        name: get_message_content,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}/$value",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get attachments from me",
        name: list_attachments,
        response: Collection<serde_json::Value>,
        path: "/messages/{{RID}}/attachments",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to attachments for me",
        name: create_attachments,
        response: serde_json::Value,
        path: "/messages/{{RID}}/attachments",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get attachments from me",
        name: get_attachments,
        response: serde_json::Value,
        path: "/messages/{{RID}}/attachments/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property attachments in me",
        name: update_attachments,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}/attachments/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action copy",
        name: copy,
        response: serde_json::Value,
        path: "/messages/{{RID}}/copy",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action createForward",
        name: create_forward,
        response: serde_json::Value,
        path: "/messages/{{RID}}/createForward",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action createReply",
        name: create_reply,
        response: serde_json::Value,
        path: "/messages/{{RID}}/createReply",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action createReplyAll",
        name: create_reply_all,
        response: serde_json::Value,
        path: "/messages/{{RID}}/createReplyAll",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get extensions from me",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/messages/{{RID}}/extensions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for me",
        name: create_extensions,
        response: serde_json::Value,
        path: "/messages/{{RID}}/extensions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get extensions from me",
        name: get_extensions,
        response: serde_json::Value,
        path: "/messages/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in me",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action forward",
        name: forward,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}/forward",
        params: 0,
        has_body: true
    });
    post!({
        name: move_message,
        response: serde_json::Value,
        path: "/messages/{{RID}}/move",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action reply",
        name: reply,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}/reply",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action replyAll",
        name: reply_all,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}/replyAll",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action send",
        name: send,
        response: GraphResponse<Content>,
        path: "/messages/{{RID}}/send",
        params: 0,
        has_body: false
    });
}
