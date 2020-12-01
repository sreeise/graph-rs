use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::posts::{PostRequest, PostsRequest};
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ThreadRequest,);
register_client!(ThreadsRequest, ());

impl<'a, Client> ThreadRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ThreadsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Threads);
        ThreadsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get threads from groups",
        name: list_threads,
        response: Collection<serde_json::Value>,
        path: "/threads",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to threads for groups",
        name: create_threads,
        response: serde_json::Value,
        path: "/threads",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> ThreadsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn posts(&self) -> PostRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        PostRequest::new(self.client)
    }
    pub fn post<ID: AsRef<str>>(&self, id: ID) -> PostsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Posts);
        PostsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get threads from groups",
        name: get_threads,
        response: serde_json::Value,
        path: "/threads/{{RID}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property threads in groups",
        name: update_threads,
        response: GraphResponse<Content>,
        path: "/threads/{{RID}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get posts from groups",
        name: list_posts,
        response: Collection<serde_json::Value>,
        path: "/threads/{{RID}}/posts",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to posts for groups",
        name: create_posts,
        response: serde_json::Value,
        path: "/threads/{{RID}}/posts",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action reply",
        name: reply,
        response: GraphResponse<Content>,
        path: "/threads/{{RID}}/reply",
        params: 1,
        has_body: true
    });
}
