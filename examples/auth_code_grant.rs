use examples_common::TestServer;
use from_as::*;
use graph_rs_sdk::oauth::OAuth;
use std::sync::{mpsc, Arc, Mutex};
use warp::{Filter, Reply};

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

#[tokio::main]
async fn main() {
    let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .add_scope("files.read")
        .add_scope("files.readwrite")
        .add_scope("files.read.all")
        .add_scope("files.readwrite.all")
        .add_scope("offline_access")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .response_type("code")
        .logout_url("https://login.microsoftonline.com/common/oauth2/v2.0/logout")
        // If this is not set, the redirect_url given above will be used for the logout redirect.
        // See logout.rs for an example.
        .post_logout_redirect_uri("http://localhost:8000/redirect");

    // Make sure the server gets the same oauth configuration as the client
    let server_oauth = oauth.clone();
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));
    let server = TestServer::serve(
        warp::get()
            .and(warp::path("redirect"))
            .and(warp::query::raw())
            .and(warp::any().map(move || tx.clone()))
            .and(warp::any().map(move || server_oauth.clone()))
            .and_then(handle),
        ([127, 0, 0, 1], 8000),
    );

    oauth
        .build()
        .authorization_code_grant()
        .browser_authorization()
        .open()
        .expect("Failed to open browser");

    // Wait for the server to get the redirect, then shut it down
    rx.recv().expect("Failed to receive");
    server.shutdown().await;
}

async fn handle(
    access_code: String,
    tx: Arc<Mutex<mpsc::Sender<()>>>,
    mut oauth: OAuth,
) -> Result<impl Reply, std::convert::Infallible> {
    // Print out the code for debugging purposes.
    println!("{:#?}", access_code);

    // Let the main thread know we've received a response and can
    // shut down (the server will stop listening, but will complete
    // requests in progress)
    tx.clone()
        .lock()
        .expect("poisoned!")
        .send(())
        .expect("failed to send");
    // The response type is automatically set to token and the grant type is automatically
    // set to authorization_code if either of these were not previously set.
    // This is done here as an example.
    oauth.access_code(&access_code);
    let mut request = oauth.build_async().authorization_code_grant();

    let access_token = request
        .access_token()
        .send()
        .await
        .expect("failed to send access token");
    oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);

    // Save our configuration to a file so we can retrieve it from other requests.
    oauth
        .as_file("./examples/example_files/web_oauth.json")
        .unwrap();

    // Generic login page response.
    Ok(warp::reply::html(
        "Successfully Logged In! You can close your browser.",
    ))
}
