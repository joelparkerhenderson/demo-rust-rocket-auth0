use alcoholic_jwt::JWKS;
use reqwest::{blocking::Client,Url};
use std::error::Error;

/// Auth0 endpoint: JSON Web Key Set (JWKS) download
///
pub fn url (
    base: reqwest::Url,
) -> Result<Url, url::ParseError> {
    base.join(".well-known/jwks.json")
}

pub fn get(client: Client, base: Url) -> Result<JWKS, Box<dyn Error>> {
    let url = url(base)?;
    Ok(client.get(url).send()?.json::<JWKS>()?)
    // let mut response = client.get(url);
    // let val = response.json::<JWKS>()?;
    // return Ok(val);
}

#[cfg(test)]
mod tests {

    use reqwest::Url;
 
    #[test]
    fn url() {
        let base = Url::parse("https://example.com").expect("uri");
        let x = crate::auth0::api::endpoints::jwks::url(base);
        assert!(x.is_ok());
        assert_eq!(x.unwrap().as_str(), "https://example.com/.well-known/jwks.json");
    }

}