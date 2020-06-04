use serde::{Serialize, Deserialize};

/// Auth0 token request struct that models this request:
///
///     curl 
///         --request POST \
///         --url 'https://YOUR_DOMAIN/oauth/token' \
///         --header 'content-type: application/x-www-form-urlencoded' \
///         --data 'grant_type=authorization_code' \
///         --data 'client_id=YOUR_CLIENT_ID' \
///         --data 'client_secret=YOUR_CLIENT_SECRET' \
///         --data code=YOUR_AUTHORIZATION_CODE \
///         --data 'redirect_uri=https://YOUR_APP/callback'
///
#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri: String,
}

impl TokenRequest {
    pub fn from_params(
        grant_type: &str, // e.g. "authorization_code"
        client_id: &str,  // e.g. "123456789"
        client_secret: &str,  // e.g. "mysecret"    
        code: &str, // e.g. "TODO"
        redirect_uri: &str, // e.g. "http://localhost:8000/callback"
    ) -> TokenRequest {
        TokenRequest {
            grant_type: grant_type.to_string().clone(),
            client_id: client_id.to_string().clone(),
            client_secret: client_secret.to_string().clone(),
            code: code.to_string().clone(),
            redirect_uri: redirect_uri.to_string().clone(),
        }
    }
}

#[cfg(test)]
mod tests {
     
    #[test]
    fn test_request() {
        let grant_type = "authorization_code";
        let client_id = "123";
        let client_secret = "mysecret";
        let code = "1234";
        let redirect_uri = "http://localhost:8000/callback";
        let x = crate::auth0::api::entities::token_request::TokenRequest::from_params(
            grant_type,
            client_id,
            client_secret,
            code,
            redirect_uri,
        );
        assert_eq!(x.grant_type, grant_type);
        assert_eq!(x.client_id, client_id);
        assert_eq!(x.client_secret, client_secret);
        assert_eq!(x.code, code);
        assert_eq!(x.redirect_uri, redirect_uri);
    }

}
