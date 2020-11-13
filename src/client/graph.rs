use crate::activities::ActivitiesRequest;
use crate::app_catalogs::AppCatalogsRequest;
use crate::applications::{ApplicationRequest, ApplicationsRequest};
use crate::attachments::AttachmentRequest;
use crate::audit_logs::AuditLogsRequest;
use crate::calendar::CalendarRequest;
use crate::certificate_based_auth_configuration::CertificateBasedAuthConfigurationRequest;
use crate::communications::CommunicationsRequest;
use crate::contacts::ContactsRequest;
use crate::contracts::ContractsRequest;
use crate::data_policy_operations::DataPolicyOperationsRequest;
use crate::device_app_management::DeviceAppManagementRequest;
use crate::device_management::DeviceManagementRequest;
use crate::directory::DirectoryRequest;
use crate::domain_dns_records::DomainDnsRecordsRequest;
use crate::domains::DomainsRequest;
use crate::drive::{DriveRequest, DrivesRequest};
use crate::education::{EducationMeRequest, EducationRequest};
use crate::group_lifecycle_policies::GroupLifecyclePoliciesRequest;
use crate::groups::{
    GroupConversationPostRequest, GroupConversationRequest, GroupThreadPostRequest,
};
use crate::identity::IdentityRequest;
use crate::invitations::InvitationsRequest;
use crate::mail::MailRequest;
use crate::onenote::OnenoteRequest;
use crate::places::PlacesRequest;
use crate::planner::PlannerRequest;
use crate::policies::PoliciesRequest;
use crate::schema_extensions::SchemaExtensionsRequest;
use crate::service_principals::ServicePrincipalsRequest;
use crate::sites::{SiteRequest, SitesRequest};
use crate::subscribed_skus::SubscribedSkusRequest;
use crate::subscriptions::SubscriptionsRequest;
use crate::teams::{TeamRequest, TeamsRequest};
use crate::teamwork::TeamworkRequest;
use crate::users::{UserRequest, UsersRequest};
use crate::{GRAPH_URL, GRAPH_URL_BETA};
use graph_error::{GraphFailure, GraphRsError};
use graph_http::url::GraphUrl;
use graph_http::{
    types::Collection, types::Content, types::DeltaPhantom, AsyncHttpClient, BlockingHttpClient,
    GraphResponse, IntoResponse, RequestClient,
};
use graph_oauth::oauth::{AccessToken, OAuth};
use handlebars::*;
use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::Method;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Ident {
    Applications,
    Calendars,
    Drives,
    Groups,
    Me,
    Sites,
    Teams,
    Users,
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        match self {
            Ident::Applications => "applications",
            Ident::Calendars => "calendars",
            Ident::Drives => "drives",
            Ident::Groups => "groups",
            Ident::Me => "me",
            Ident::Sites => "sites",
            Ident::Teams => "teams",
            Ident::Users => "users",
        }
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        match self {
            Ident::Applications => "applications".into(),
            Ident::Calendars => "calendars".into(),
            Ident::Drives => "drives".into(),
            Ident::Groups => "groups".into(),
            Ident::Me => "me".into(),
            Ident::Sites => "sites".into(),
            Ident::Teams => "teams".into(),
            Ident::Users => "users".into(),
        }
    }
}

impl FromStr for Ident {
    type Err = GraphRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            b"applications" => Ok(Ident::Applications),
            b"calendars" => Ok(Ident::Calendars),
            b"drives" => Ok(Ident::Drives),
            b"groups" => Ok(Ident::Groups),
            b"me" => Ok(Ident::Me),
            b"sites" => Ok(Ident::Sites),
            b"teams" => Ok(Ident::Teams),
            b"users" => Ok(Ident::Users),
            _ => Err(GraphRsError::InvalidOrMissing {
                msg: "Not a valid Ident".into(),
            }),
        }
    }
}

impl Default for Ident {
    fn default() -> Self {
        Ident::Me
    }
}

/// The graph client.
///
/// The graph client supports blocking and asynchronous requests but
/// you must declare which type of client you want to use beforehand.
///
/// # Blocking - Most simple and easy to use
/// ```
/// # use graph_rs::client::Graph;
/// let _client = Graph::new("ACCESS_TOKEN");
/// ```
///
/// # Async
/// ```
/// # use graph_rs::client::Graph;
/// let _client = Graph::new_async("ACCESS_TOKEN");
/// ```
pub struct Graph<Client> {
    pub(crate) request: Client,
}

impl<'a, Client> Graph<Client>
where
    Client: graph_http::RequestClient,
{
    /// Use the V1.0 api.
    pub fn v1(&'a self) -> Identify<'a, Client> {
        self.request.set_url(GraphUrl::from_str(GRAPH_URL).unwrap());
        Identify { client: &self }
    }

    /// Use the beta API.
    pub fn beta(&'a self) -> Identify<'a, Client> {
        self.request
            .set_url(GraphUrl::from_str(GRAPH_URL_BETA).unwrap());
        Identify { client: &self }
    }

    /// Check if the current host is v1.0.
    pub fn is_v1(&self) -> bool {
        self.request.url().as_str().starts_with(GRAPH_URL)
    }

    /// Check if the current host is beta.
    pub fn is_beta(&self) -> bool {
        self.request.url().as_str().starts_with(GRAPH_URL_BETA)
    }

    /// Set the access token used for requests.
    pub fn set_token(&self, token: &str) {
        self.request.set_token(token);
    }

    pub fn ident(&self) -> Ident {
        Ident::from_str(self.request.ident().as_str()).unwrap()
    }

    pub(crate) fn set_ident(&self, ident: Ident) {
        self.request.set_ident(ident.to_string());
    }

    pub(crate) fn request(&self) -> &Client {
        &self.request
    }
}

impl<Client> Debug for Graph<Client>
where
    Client: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.request.fmt(f)
    }
}

type GraphBlocking = Graph<BlockingHttpClient>;
type GraphAsync = Graph<AsyncHttpClient>;

impl<'a> GraphBlocking {
    /// Create a new client with an access token.
    ///
    /// # Example
    /// ```
    /// use graph_rs::client::Graph;
    ///
    /// let client = Graph::new("ACCESS_TOKEN");
    /// ```
    /// ```rust,ignore
    /// // Use the v1.0 API
    ///
    /// // Returns a response object with the body
    /// // converted to serde_json::Value.
    /// let response = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .send()?;
    ///
    /// println!("{:#?}", response.body());
    ///
    /// // Use a custom data structure. The json method
    /// // will convert anything that implements serde deserialize.
    /// let drive_items: serde_json::Value = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .json()?;
    /// ```
    pub fn new(token: &str) -> GraphBlocking {
        let request = BlockingHttpClient::new(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph { request }
    }

    pub fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl),
    {
        self.request.inner_url_ref(f)
    }
}

impl From<&str> for GraphBlocking {
    fn from(token: &str) -> Self {
        Graph::new(token)
    }
}

impl From<String> for GraphBlocking {
    fn from(token: String) -> Self {
        Graph::new(token.as_str())
    }
}

impl From<&AccessToken> for GraphBlocking {
    fn from(token: &AccessToken) -> Self {
        Graph::new(token.bearer_token())
    }
}

impl TryFrom<&OAuth> for GraphBlocking {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth.get_access_token()?;
        Ok(Graph::from(&access_token))
    }
}

impl<'a> GraphAsync {
    /// Create a new client with an access token.
    ///
    /// # Example
    /// ```
    /// use graph_rs::client::Graph;
    ///
    /// let client = Graph::new_async("ACCESS_TOKEN");
    /// ```
    /// ```rust,ignore
    /// // Use the v1.0 API
    ///
    /// // Returns a response object with the body
    /// // converted to serde_json::Value.
    /// let response = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .await
    ///     .send()?;
    ///
    /// println!("{:#?}", response.body());
    ///
    /// // Use a custom data structure. The json method
    /// // will convert anything that implements serde deserialize.
    /// let drive_items: serde_json::Value = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .await
    ///     .json()?;
    /// ```
    pub fn new_async(token: &str) -> GraphAsync {
        let request = AsyncHttpClient::new(GraphUrl::parse(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph { request }
    }

    pub fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl) + Sync,
    {
        self.request.url_ref(f)
    }
}

impl From<&str> for GraphAsync {
    fn from(token: &str) -> Self {
        Graph::new_async(token)
    }
}

impl From<String> for GraphAsync {
    fn from(token: String) -> Self {
        Graph::new_async(token.as_str())
    }
}

impl From<&AccessToken> for GraphAsync {
    fn from(token: &AccessToken) -> Self {
        Graph::new_async(token.bearer_token())
    }
}

impl TryFrom<&OAuth> for GraphAsync {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth.get_access_token()?;
        Ok(Graph::from(&access_token))
    }
}

pub struct Identify<'a, Client> {
    client: &'a Graph<Client>,
}

impl<'a, Client> Identify<'a, Client>
where
    Client: graph_http::RequestClient,
{
    fn set_path(&self, id: &str) {
        let ident = self.client.ident();
        if self.client.ident().eq(&Ident::Me) {
            self.client.request().extend_path(&[ident.as_ref()]);
        } else {
            self.client.request().extend_path(&[ident.as_ref(), id]);
        }
    }

    pub fn activities(&self) -> ActivitiesRequest<'a, Client> {
        ActivitiesRequest::new(self.client)
    }

    pub fn app_catalogs(&self) -> AppCatalogsRequest<'a, Client> {
        AppCatalogsRequest::new(self.client)
    }

    pub fn application<S: AsRef<str>>(&self, id: S) -> ApplicationsRequest<'a, Client> {
        self.client.set_ident(Ident::Applications);
        ApplicationsRequest::new(id.as_ref(), self.client)
    }

    pub fn applications(&self) -> ApplicationRequest<'a, Client> {
        self.client.set_ident(Ident::Applications);
        ApplicationRequest::new(self.client)
    }

    pub fn audit_logs(&self) -> AuditLogsRequest<'a, Client> {
        AuditLogsRequest::new(self.client)
    }

    pub fn certificate_based_auth_configuration(
        &self,
    ) -> CertificateBasedAuthConfigurationRequest<'a, Client> {
        CertificateBasedAuthConfigurationRequest::new(self.client)
    }

    pub fn communications(&self) -> CommunicationsRequest<'a, Client> {
        CommunicationsRequest::new(self.client)
    }

    pub fn contracts(&self) -> ContractsRequest<'a, Client> {
        ContractsRequest::new(self.client)
    }

    pub fn data_policy_operations(&self) -> DataPolicyOperationsRequest<'a, Client> {
        DataPolicyOperationsRequest::new(self.client)
    }

    pub fn device_app_management(&self) -> DeviceAppManagementRequest<'a, Client> {
        DeviceAppManagementRequest::new(self.client)
    }

    pub fn device_management(&self) -> DeviceManagementRequest<'a, Client> {
        DeviceManagementRequest::new(self.client)
    }

    pub fn directory(&self) -> DirectoryRequest<'a, Client> {
        DirectoryRequest::new(self.client)
    }

    pub fn domain_dns_records(&self) -> DomainDnsRecordsRequest<'a, Client> {
        DomainDnsRecordsRequest::new(self.client)
    }

    pub fn domains(&self) -> DomainsRequest<'a, Client> {
        DomainsRequest::new(self.client)
    }

    pub fn drive(&self) -> DriveRequest<'a, Client> {
        self.client.set_ident(Ident::Drives);
        DriveRequest::new(self.client)
    }

    pub fn drives<S: AsRef<str>>(&self, id: S) -> DrivesRequest<'a, Client> {
        self.client.set_ident(Ident::Drives);
        self.set_path(id.as_ref());
        DrivesRequest::new(self.client)
    }

    pub fn education(&self) -> EducationRequest<'a, Client> {
        EducationRequest::new(self.client)
    }

    pub fn groups<S: AsRef<str>>(&self, id: S) -> IdentGroups<'a, Client> {
        self.client.set_ident(Ident::Groups);
        IdentGroups::new(id.as_ref(), self.client)
    }

    pub fn group_lifecycle_policies(&self) -> GroupLifecyclePoliciesRequest<'a, Client> {
        GroupLifecyclePoliciesRequest::new(self.client)
    }

    pub fn identity(&self) -> IdentityRequest<'a, Client> {
        IdentityRequest::new(self.client)
    }

    pub fn invitations(&self) -> InvitationsRequest<'a, Client> {
        InvitationsRequest::new(self.client)
    }

    pub fn me(&self) -> IdentMe<'a, Client> {
        self.client.set_ident(Ident::Me);
        IdentMe::new("", self.client)
    }

    pub fn places(&self) -> PlacesRequest<'a, Client> {
        PlacesRequest::new(self.client)
    }

    pub fn planner(&self) -> PlannerRequest<'a, Client> {
        PlannerRequest::new(self.client)
    }

    pub fn policies(&self) -> PoliciesRequest<'a, Client> {
        PoliciesRequest::new(self.client)
    }

    pub fn schema_extensions(&self) -> SchemaExtensionsRequest<'a, Client> {
        SchemaExtensionsRequest::new(self.client)
    }

    pub fn service_principals(&self) -> ServicePrincipalsRequest<'a, Client> {
        ServicePrincipalsRequest::new(self.client)
    }

    pub fn subscribed_skus(&self) -> SubscribedSkusRequest<'a, Client> {
        SubscribedSkusRequest::new(self.client)
    }

    pub fn subscriptions(&self) -> SubscriptionsRequest<'a, Client> {
        SubscriptionsRequest::new(self.client)
    }

    pub fn site<S: AsRef<str>>(&self, id: S) -> SitesRequest<'a, Client> {
        self.client.set_ident(Ident::Sites);
        SitesRequest::new(id.as_ref(), self.client)
    }

    pub fn sites(&self) -> SiteRequest<'a, Client> {
        self.client.set_ident(Ident::Sites);
        SiteRequest::new(self.client)
    }

    pub fn teamwork(&self) -> TeamworkRequest<'a, Client> {
        TeamworkRequest::new(self.client)
    }

    pub fn team<S: AsRef<str>>(&self, id: S) -> TeamsRequest<'a, Client> {
        self.client.set_ident(Ident::Teams);
        TeamsRequest::new(id.as_ref(), self.client)
    }

    pub fn teams(&self) -> TeamRequest<'a, Client> {
        self.client.set_ident(Ident::Teams);
        TeamRequest::new(self.client)
    }

    pub fn user<S: AsRef<str>>(&self, id: S) -> UsersRequest<'a, Client> {
        self.client.set_ident(Ident::Users);
        UsersRequest::new(id.as_ref(), self.client)
    }

    pub fn users(&self) -> UserRequest<'a, Client> {
        self.client.set_ident(Ident::Users);
        UserRequest::new(self.client)
    }

    /// Perform a batch requests which can store multiple requests
    /// in the request body.
    pub fn batch<B: serde::Serialize>(
        &self,
        batch: &B,
    ) -> IntoResponse<'a, DeltaPhantom<serde_json::Value>, Client> {
        let client = self.client.request();
        client.set_method(Method::POST);
        client.header(ACCEPT, HeaderValue::from_static("application/json"));
        let body = serde_json::to_string(batch).map_err(GraphFailure::from);
        if let Err(err) = body {
            return IntoResponse::new_error(self.client.request(), err);
        } else if let Ok(body) = body {
            client.set_body(body);
        }
        render_path!(self.client, "$batch", &serde_json::json!({}));
        IntoResponse::new(&self.client.request)
    }
}

register_ident_client!(IdentMe,);
register_ident_client!(IdentDrives,);
register_ident_client!(IdentGroups,);

impl<'a, Client> IdentMe<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( get, serde_json::Value => "me" );
    get!( list_events, Collection<serde_json::Value> => "me/events" );
    get!( settings, serde_json::Value => "me/settings" );
    get!( list_planner_tasks, Collection<serde_json::Value> => "me/planner/tasks");
    patch!( [ update_settings, serde_json::Value => "me/settings" ] );

    pub fn activities(&'a self) -> ActivitiesRequest<'a, Client> {
        self.set_path();
        ActivitiesRequest::new(self.client)
    }

    pub fn education(&self) -> EducationMeRequest<'a, Client> {
        EducationMeRequest::new(&self.client)
    }
}

impl<'a, Client> IdentGroups<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "groups" );
    get!( get, serde_json::Value => "groups/{{RID}}" );
    get!( delta, DeltaPhantom<Collection<serde_json::Value>> => "groups/delta" );
    get!( list_events, Collection<serde_json::Value> => "groups/{{RID}}/events" );
    get!( list_lifecycle_policies, Collection<serde_json::Value> => "groups/{{RID}}/groupLifecyclePolicies" );
    get!( list_member_of, Collection<serde_json::Value> => "groups/{{RID}}/memberOf" );
    get!( list_transitive_member_of, Collection<serde_json::Value> => "groups/{{RID}}/transitiveMemberOf" );
    get!( list_members, Collection<serde_json::Value> => "groups/{{RID}}/members"  );
    get!( list_transitive_members, Collection<serde_json::Value> => "groups/{{RID}}/transitiveMembers" );
    get!( list_owners, Collection<serde_json::Value> => "groups/{{RID}}/owners" );
    get!( list_photos, Collection<serde_json::Value> => "groups/{{RID}}/photos" );
    get!( root_site, Collection<serde_json::Value> => "groups/{{RID}}/sites/root" );
    get!( list_planner_plans, Collection<serde_json::Value> => "groups/{{RID}}/planner/plans" );
    post!( [ create, serde_json::Value => "groups" ] );
    post!( add_favorite, GraphResponse<Content> => "groups/{{RID}}/addFavorite" );
    post!( [ add_member, GraphResponse<Content> => "groups/{{RID}}/members/$ref" ] );
    post!( [ add_owner, GraphResponse<Content> => "groups/{{RID}}/owners/$ref" ] );
    post!( [ check_member_groups, Collection<String> => "groups/{{RID}}/checkMemberGroups" ] );
    post!( [ member_groups, Collection<String> => "groups/{{RID}}/getMemberGroups" ] );
    post!( [ member_objects, Collection<String> => "groups/{{RID}}/getMemberObjects" ] );
    post!( remove_favorite, GraphResponse<Content> => "groups/{{RID}}/removeFavorite" );
    post!( renew, GraphResponse<Content> => "groups/{{RID}}/renew" );
    post!( reset_unseen_count, GraphResponse<Content> => "groups/{{RID}}/resetUnseenCount" );
    post!( subscribe_by_mail, GraphResponse<Content> => "groups/{{RID}}/subscribeByMail" );
    post!( unsubscribe_by_mail, GraphResponse<Content> => "groups/{{RID}}/unsubscribeByMail" );
    post!( [ validate_properties, GraphResponse<Content> => "groups/{{RID}}/validateProperties" ] );
    patch!( [ update, serde_json::Value => "groups/{{RID}}" ] );
    delete!( delete, GraphResponse<Content> => "groups/{{RID}}" );
    delete!( | remove_member, GraphResponse<Content> => "groups/{{RID}}/members/{{id}}/$ref" );
    delete!( | remove_owner, GraphResponse<Content> => "groups/{{RID}}/owners/{{id}}/$ref" );

    pub fn conversations(&self) -> GroupConversationRequest<'a, Client> {
        GroupConversationRequest::new(self.client)
    }

    pub fn conversation_posts(&'a self) -> GroupConversationPostRequest<'a, Client> {
        GroupConversationPostRequest::new(self.client)
    }

    pub fn thread_posts(&'a self) -> GroupThreadPostRequest<'a, Client> {
        GroupThreadPostRequest::new(self.client)
    }
}
