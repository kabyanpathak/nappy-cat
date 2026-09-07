use oauth2::RequestTokenError;
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{
    AuthUrl, AuthorizationCode, BasicErrorResponseType, ClientId, ClientSecret, CsrfToken,
    HttpClientError, PkceCodeChallenge, RedirectUrl, Scope, StandardErrorResponse, TokenResponse,
    TokenUrl,
};
use url::Url;

//struct for
pub struct GoogleToken {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub expires_at: u64,

    // don't know for now
    pub id_token: String,
    pub scope: Option<String>,
}

pub fn check_key(security_key: String) -> bool {
    true
}

pub async fn auth(
    security_key: String,
) -> Result<
    bool,
    RequestTokenError<
        HttpClientError<oauth2::reqwest::Error>,
        StandardErrorResponse<BasicErrorResponseType>,
    >,
> {
    let check: bool = check_key(security_key);

    if check == false {
        return Err(String::from("false key value"));
    }

    let client = BasicClient::new(ClientId::new("client_id".to_string()))
        .set_client_secret(ClientSecret::new("client_secret".to_string()))
        .set_auth_uri(AuthUrl::new("http://authorize".to_string())?)
        .set_token_uri(TokenUrl::new("http://token".to_string())?)
        .set_redirect_uri(RedirectUrl::new("http://redirect".to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let token_result = client
        .exchange_code(AuthorizationCode::new(
            "some authorization code".to_string(),
        ))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    ok(true)
}
