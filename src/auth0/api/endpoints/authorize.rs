use reqwest::{blocking::Client,Url};

/// Auth0 endpoint: authorize.
///
/// The format is:
///
///     https://YOUR_DOMAIN/authorize?
///     response_type=code&
///     client_id=YOUR_CLIENT_ID&
///     redirect_uri=YOUR_CALLBACK&
///     scope=SCOPE&
///     state=STATE    
///
pub fn url(
    base: Url,
    response_type: &str, // e.g. "code"
    client_id: &str,  // e.g. "123456789"
    redirect_uri: &str, // e.g. "http://localhost:8000/callback"
    scope: &str, // e.g. "openid profile"
    state: &str
) -> Result<reqwest::Url, url::ParseError> {
    Ok(base.join("authorize")?.query_pairs_mut()
    .append_pair("response_type", response_type)
    .append_pair("client_id", client_id)
    .append_pair("redirect_uri", redirect_uri)
    .append_pair("scope", scope)
    .append_pair("state", state)
    .finish().clone())
}

#[cfg(test)]
mod tests {

    use reqwest::Url;

    #[test]
    fn url() {
        let base = Url::parse("https://example.com").expect("url");
        let response_type = "my_response_type";
        let client_id = "my_client_id";
        let redirect_uri = "http://localhost:8000/callback";
        let scope = "openid profile";
        let state = "my_state";
        let x = crate::auth0::api::endpoints::authorize::url(base, response_type, client_id, redirect_uri, scope, state);
        assert!(x.is_ok());
        assert_eq!(x.unwrap().as_str(), "https://example.com/authorize?response_type=my_response_type&client_id=my_client_id&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fcallback&scope=openid+profile&state=my_state");
    }

}