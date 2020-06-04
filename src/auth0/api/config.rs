use reqwest::Url;

/// Auth0 application configuration struct of settings we keep together.
///
///   * We prefer to implement configuration as a struct, so we can pass 
///     it around, rather than as separate strings such as parameters.
///
///   * The configuration data comes from the Auth0 service, when we 
///     create an Auth0 application, and choose a domain and redirect URL,
///     and receive an Auth0 app client id and client secret.
///
///   * We prefer to load the configuration via environment variables.
///     For this demo, we use an example file `env.example`.
/// 
///   * You can implement configuration as you wish; our way is one example.
///
pub struct Config {
    pub base: Url,
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

/// Initiatlize Auth0 app configuration. 
///
///   * We prefer to implement this by reading environment variables,
///     and by using a struct, as described in the section above.
///
///   * You can implement configuration as you wish; our way is one example.
///
impl Config {
    pub fn from_env() -> Config {
        Config {
            base: reqwest::Url::parse(&format!("https://{}/", std::env::var("AUTH0_APP_DOMAIN").expect("AUTH0_APP_DOMAIN"))).expect("url"),
            domain: std::env::var("AUTH0_APP_DOMAIN").expect("AUTH0_APP_DOMAIN"),
            client_id: std::env::var("AUTH0_APP_CLIENT_ID").expect("AUTH0_APP_CLIENT_ID"),
            client_secret: std::env::var("AUTH0_APP_CLIENT_SECRET").expect("AUTH0_APP_CLIENT_SECRET"),
            redirect_uri: std::env::var("AUTH0_APP_REDIRECT_URI").expect("AUTH0_APP_REDIRECT_URI"),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_env() {
        std::env::set_var("AUTH0_APP_DOMAIN", "example.com");
        std::env::set_var("AUTH0_APP_CLIENT_ID", "my_client_id");
        std::env::set_var("AUTH0_APP_CLIENT_SECRET", "my_client_secret");
        std::env::set_var("AUTH0_APP_REDIRECT_URI", "my_redirect_uri");
        let x = crate::auth0::api::config::Config::from_env();
        assert_eq!(x.base.as_str(), "https://example.com/");
        assert_eq!(x.domain, "example.com");
        assert_eq!(x.client_id, "my_client_id");
        assert_eq!(x.client_secret, "my_client_secret");
        assert_eq!(x.redirect_uri, "my_redirect_uri");
    }

}