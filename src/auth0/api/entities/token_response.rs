use serde::{Serialize, Deserialize};

/// Auth0 token response struct that models the response from Auth0TokeRequest.
///
#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u32, // e.g. 86400
    pub id_token: String,
    pub token_type: String, // e.g. "Bearer"
    pub scope: String, // e.g. "openid profile"
}

impl TokenResponse {
    pub fn from_params(
        access_token: &str, // e.g. "…"
        expires_in: u32, // e.g. 86400
        id_token: &str, // e.g. "…"    
        token_type: &str, // e.g. "Bearer"
        scope: &str, // e.g. "openid profile"
    ) -> TokenResponse {
        TokenResponse {
            access_token: access_token.to_string().clone(),
            expires_in: expires_in.clone(),
            id_token: id_token.to_string().clone(),
            token_type: token_type.to_string().clone(),
            scope: scope.to_string().clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_auth0_token_response() {
        let access_token = "my_access_token";
        let expires_in = 123;
        let id_token = "my_id_token";
        let token_type = "my_token_type";
        let scope = "my_scope";
        let x = crate::auth0::api::entities::token_response::TokenResponse::from_params(
            access_token,
            expires_in,
            id_token,
            token_type,
            scope,
        );
        assert_eq!(x.access_token, access_token);
        assert_eq!(x.expires_in, expires_in);
        assert_eq!(x.id_token, id_token);
        assert_eq!(x.token_type, token_type);
        assert_eq!(x.scope, scope);
    }

}
