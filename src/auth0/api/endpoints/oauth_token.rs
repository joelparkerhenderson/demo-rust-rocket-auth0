use reqwest::{blocking::Client,Url};

pub fn url (
    base: Url
) -> Result<Url, url::ParseError> {
    base.join("oauth/token")
}

#[cfg(test)]
mod tests {

    use reqwest::Url;

    #[test]
    fn url() {
        let base = Url::parse("https://example.com").expect("url");
        let x = crate::auth0::api::endpoints::oauth_token::url(base);       
        assert!(x.is_ok());
        assert_eq!(x.unwrap().as_str(), "https://example.com/oauth/token");
    }

}
