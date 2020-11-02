use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(PoliciesRequest,);
register_client!(PermissionGrantPoliciesRequest,);

impl<'a, Client> PoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn permission_grant_policies(&self) -> PermissionGrantPoliciesRequest<'a, Client> {
        PermissionGrantPoliciesRequest::new(&self.client)
    }
    get!({
        doc: "# Get claimsMappingPolicies from policies",
        name: list_claims_mapping_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/claimsMappingPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to claimsMappingPolicies for policies",
        name: create_claims_mapping_policies,
        response: serde_json::Value,
        path: "/policies/claimsMappingPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get activityBasedTimeoutPolicies from policies",
        name: list_activity_based_timeout_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/activityBasedTimeoutPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to activityBasedTimeoutPolicies for policies",
        name: create_activity_based_timeout_policies,
        response: serde_json::Value,
        path: "/policies/activityBasedTimeoutPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get conditionalAccessPolicies from policies",
        name: get_conditional_access_policies,
        response: serde_json::Value,
        path: "/policies/conditionalAccessPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property conditionalAccessPolicies in policies",
        name: update_conditional_access_policies,
        response: GraphResponse<Content>,
        path: "/policies/conditionalAccessPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from policies",
        name: get_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/policies/homeRealmDiscoveryPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property homeRealmDiscoveryPolicies in policies",
        name: update_home_realm_discovery_policies,
        response: GraphResponse<Content>,
        path: "/policies/homeRealmDiscoveryPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get tokenLifetimePolicies from policies",
        name: list_token_lifetime_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/tokenLifetimePolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tokenLifetimePolicies for policies",
        name: create_token_lifetime_policies,
        response: serde_json::Value,
        path: "/policies/tokenLifetimePolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get claimsMappingPolicies from policies",
        name: get_claims_mapping_policies,
        response: serde_json::Value,
        path: "/policies/claimsMappingPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property claimsMappingPolicies in policies",
        name: update_claims_mapping_policies,
        response: GraphResponse<Content>,
        path: "/policies/claimsMappingPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get conditionalAccessPolicies from policies",
        name: list_conditional_access_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/conditionalAccessPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to conditionalAccessPolicies for policies",
        name: create_conditional_access_policies,
        response: serde_json::Value,
        path: "/policies/conditionalAccessPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get permissionGrantPolicies from policies",
        name: list_permission_grant_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/permissionGrantPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to permissionGrantPolicies for policies",
        name: create_permission_grant_policies,
        response: serde_json::Value,
        path: "/policies/permissionGrantPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get activityBasedTimeoutPolicies from policies",
        name: get_activity_based_timeout_policies,
        response: serde_json::Value,
        path: "/policies/activityBasedTimeoutPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property activityBasedTimeoutPolicies in policies",
        name: update_activity_based_timeout_policies,
        response: GraphResponse<Content>,
        path: "/policies/activityBasedTimeoutPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get policies",
        name: get_policy_root,
        response: serde_json::Value,
        path: "/policies",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update policies",
        name: update_policy_root,
        response: GraphResponse<Content>,
        path: "/policies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get tokenLifetimePolicies from policies",
        name: get_token_lifetime_policies,
        response: serde_json::Value,
        path: "/policies/tokenLifetimePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property tokenLifetimePolicies in policies",
        name: update_token_lifetime_policies,
        response: GraphResponse<Content>,
        path: "/policies/tokenLifetimePolicies/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get tokenIssuancePolicies from policies",
        name: get_token_issuance_policies,
        response: serde_json::Value,
        path: "/policies/tokenIssuancePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property tokenIssuancePolicies in policies",
        name: update_token_issuance_policies,
        response: GraphResponse<Content>,
        path: "/policies/tokenIssuancePolicies/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get permissionGrantPolicies from policies",
        name: get_permission_grant_policies,
        response: serde_json::Value,
        path: "/policies/permissionGrantPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property permissionGrantPolicies in policies",
        name: update_permission_grant_policies,
        response: GraphResponse<Content>,
        path: "/policies/permissionGrantPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get identitySecurityDefaultsEnforcementPolicy from policies",
        name: get_identity_security_defaults_enforcement_policy,
        response: serde_json::Value,
        path: "/policies/identitySecurityDefaultsEnforcementPolicy",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property identitySecurityDefaultsEnforcementPolicy in policies",
        name: update_identity_security_defaults_enforcement_policy,
        response: GraphResponse<Content>,
        path: "/policies/identitySecurityDefaultsEnforcementPolicy",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from policies",
        name: list_home_realm_discovery_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/homeRealmDiscoveryPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to homeRealmDiscoveryPolicies for policies",
        name: create_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/policies/homeRealmDiscoveryPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get tokenIssuancePolicies from policies",
        name: list_token_issuance_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/tokenIssuancePolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tokenIssuancePolicies for policies",
        name: create_token_issuance_policies,
        response: serde_json::Value,
        path: "/policies/tokenIssuancePolicies",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> PermissionGrantPoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get excludes from policies",
        name: list_excludes,
        response: Collection<serde_json::Value>,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to excludes for policies",
        name: create_excludes,
        response: serde_json::Value,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get includes from policies",
        name: get_includes,
        response: serde_json::Value,
        path: "/policies/permissionGrantPolicies/{{id}}/includes/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property includes in policies",
        name: update_includes,
        response: GraphResponse<Content>,
        path: "/policies/permissionGrantPolicies/{{id}}/includes/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get includes from policies",
        name: list_includes,
        response: Collection<serde_json::Value>,
        path: "/policies/permissionGrantPolicies/{{id}}/includes",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to includes for policies",
        name: create_includes,
        response: serde_json::Value,
        path: "/policies/permissionGrantPolicies/{{id}}/includes",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get excludes from policies",
        name: get_excludes,
        response: serde_json::Value,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property excludes in policies",
        name: update_excludes,
        response: GraphResponse<Content>,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes/{{id2}}",
        params: 2,
        has_body: true
    });
}
