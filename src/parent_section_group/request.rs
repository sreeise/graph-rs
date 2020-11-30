use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::parent_notebook::ParentNotebookRequest;
use crate::section_groups::SectionGroupsRequest;
use crate::sections::SectionsRequest;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ParentSectionGroupRequest,);

impl<'a, Client> ParentSectionGroupRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn parent_notebook(&self) -> ParentNotebookRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::ParentNotebook);
        ParentNotebookRequest::new(self.client)
    }
    pub fn parent_section_group(&self) -> ParentSectionGroupRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        ParentSectionGroupRequest::new(self.client)
    }
    pub fn section_group<ID: AsRef<str>>(&self, id: ID) -> SectionGroupsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::SectionGroups);
        SectionGroupsRequest::new(id.as_ref(), self.client)
    }
    pub fn section<ID: AsRef<str>>(&self, id: ID) -> SectionsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Sections);
        SectionsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get parentSectionGroup from me",
        name: get_parent_section_group,
        response: serde_json::Value,
        path: "/parentSectionGroup",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentSectionGroup in me",
        name: update_parent_section_group,
        response: GraphResponse<Content>,
        path: "/parentSectionGroup",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get parentNotebook from me",
        name: get_parent_notebook,
        response: serde_json::Value,
        path: "/parentSectionGroup/parentNotebook",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentNotebook in me",
        name: update_parent_notebook,
        response: GraphResponse<Content>,
        path: "/parentSectionGroup/parentNotebook",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get sectionGroups from me",
        name: list_section_groups,
        response: Collection<serde_json::Value>,
        path: "/parentSectionGroup/sectionGroups",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sectionGroups for me",
        name: create_section_groups,
        response: serde_json::Value,
        path: "/parentSectionGroup/sectionGroups",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get sections from me",
        name: list_sections,
        response: Collection<serde_json::Value>,
        path: "/parentSectionGroup/sections",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sections for me",
        name: create_sections,
        response: serde_json::Value,
        path: "/parentSectionGroup/sections",
        params: 1,
        has_body: true
    });
}
