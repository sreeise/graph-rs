use graph_rs::prelude::*;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::THROTTLE_MUTEX;

#[test]
fn user_request_test() {
    if OAuthRequest::is_appveyor() {
        return;
    }

    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let _lock = THROTTLE_MUTEX.lock().unwrap();
            let client = Graph::new(bearer.as_str());
            let users = client.v1().users(id.as_str()).list().send();

            if let Ok(response) = users {
                assert!(
                    response.status() == 200 ||
                        response.status() == 201 ||
                        response.status() == 204
                );
            } else if let Err(e) = users {
                panic!("Request error. Method: users list. Error: {:#?}", e);
            }

            let user_res = client.v1().users(id.as_str()).get().send();

            if let Ok(response) = user_res {
                assert!(
                    response.status() == 200 ||
                        response.status() == 201 ||
                        response.status() == 204
                );
            } else if let Err(e) = user_res {
                panic!("Request error. Method: users list. Error: {:#?}", e);
            }
        }
    })
}
