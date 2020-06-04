#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate hex;
extern crate log;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate url;
use alcoholic_jwt::{JWKS, Validation, validate, token_kid};
use log::*;
use rocket::http::Cookie;
use std::convert::TryFrom;
pub mod auth0;
pub mod util;
use util::nonce::nonce;

/// Typical home page that simply prints "home".
/// This doesn't use any authentication.
#[get("/")]
fn home() -> String {
    info!("get /");
    "home".to_string()
}

/// The route `/login` is what triggers the start of authentication.
///
/// If the user's login is successful, then Auth0 will redirect the user
/// to the route `/callback` with query params for `code` and `state`.
///
/// In this demo implementation, we want to show how to use the `state`
/// option to mitigate CSRF attacks: we generate a random state string,
/// a.k.a. nonce.  We send the state along in the URL, and we also save it
/// as a cookie so we can compare the cookie state to the response state;
/// the two states must match in order for the authentication to be valid.
///
#[get("/login")]
fn login(mut cookies: rocket::http::Cookies) -> Result<rocket::response::Redirect, rocket::http::Status> {
    info!("get /login");
    let config = crate::auth0::api::config::Config::from_env();
    let state = nonce();
    cookies.add(rocket::http::Cookie::new("state", state.clone()));
let url = crate::auth0::api::endpoints::authorize::url(
        config.base, 
        &"code",
        &config.client_id,
        &config.redirect_uri,
        &"openid profile",
        &state,
    ).expect("url");
    let uri = rocket::http::uri::Uri::try_from(url.as_str().to_owned()).expect("uri");
    Ok(rocket::response::Redirect::to(uri))
}

/// The route `/callback` is where Auth0 calls us back about a login.
/// The purpose to exchange the Authorization Code for an Access Token.
///
/// If the user's login is successful, then Auth0 will redirect the user
/// to the route `/callback` with query params for `code` and `state`.
///
/// This route must be the same enpoint as our Auth0 app config 
/// value `redirect_uri`, and the same as configured via the
/// Auth0 website management console.
///
/// We verify that `state` parameter matches the state string that we previously
/// saved as a cookie andsent to Auth0's /authorize endpoint.
///
/// Now that you have an Authorization Code, you must exchange it for an Access
/// Token that can be used to call your API. Using the Authorization Code (code)
/// from the previous step, you will POST to the Token URL.
///
/// We use `code` in a TokenRequest to the /oauth/token endpoint.
///
/// Where:
///
///   * grant_type: This must be authorization_code.
///   * client_id: Your application's Client ID.
///   * client_secret: Your application's Client Secret.
///   * code: The Authorization Code received from the initial authorize call.
///   * redirect_uri: The URL must match exactly the redirect_uri passed to /authorize.
/// 
/// The response contains, for example:
///
///   * access_token
///   * refresh_token
///   * id_token
///   * token_typs
///
/// Example JSON response:
///
///     {
///      	"access_token":"…",
///      	"id_token":"…",
///      	"scope":"openid profile",
///      	"expires_in":86400,
///      	"token_type":"Bearer"
///     }
///
#[get("/callback?<code>&<state>")]
fn callback(
    code: String,
    state: String,
    mut cookies: rocket::http::Cookies,
) ->  Result<rocket::response::Redirect, rocket::http::Status> {
    info!("get /callback code:{} state:{}", code, state);
    util::cookies::require(&cookies, &Cookie::new("state", state))?;
    cookies.remove(rocket::http::Cookie::named("state"));

    // Create configuration
    info!("get /callback -> create configuration");
    let config = crate::auth0::api::config::Config::from_env();

    // Create a token request that contains the code.
    info!("get /callback -> create token request");
    let token_request = crate::auth0::api::entities::token_request::TokenRequest::from_params(
        "authorization_code",
        &config.client_id,
        &config.client_secret,
        &code,
        &config.redirect_uri,
    );

    // Create a HTTP request that contains the token request.
    info!("get /callback -> create reqwest");
    let config = crate::auth0::api::config::Config::from_env();
    let url = crate::auth0::api::endpoints::oauth_token::url(config.base).expect("url");
    let client = reqwest::blocking::Client::new(); // TODO remove blocking
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&token_request)
        .send()
        .unwrap();
    if response.status().is_success() {
        info!("get /callback -> response -> success");
    } 
    else {
        warn!("get /callback -> response -> not success");
        warn!("get /callback -> response -> not success -> status:{:?} text:{:?}", response.status(), response.text());
        return Err(rocket::http::Status::BadRequest);
    }

    // TODO: Can we unwrap here because we know for certain we've populated the cert in the db?
    // let pub_key: Vec<u8> = db.get(b"jwt_pub_key_pem").unwrap().unwrap().to_vec();
    // let payload = decode_and_validate_jwt(
    //     pub_key,
    //     &resp.id_token,
    //     &settings.client_id,
    //     &settings.auth0_domain,
    // )
    // .map_err(|_| Status::Unauthorized)?;
    // let user = get_or_create_user(&db, &payload).map_err(|e| match e.downcast_ref() {
    //     Some(AuthError::MalformedJWT { .. }) => Status::BadRequest,
    //     _ => Status::InternalServerError,
    // })?;

    // let jwt = &resp.id_token.clone();
    // let hashed_jwt = hex_digest(HashAlgorithm::SHA256, jwt.as_bytes());
    // let new_session = Session {
    //     user_id: user.user_id,
    //     expires: payload.exp,
    //     raw_jwt: jwt.as_bytes().to_vec(),
    // };
    // let encoded_session = serialize(&new_session).map_err(|_| Status::Unauthorized)?;
    // let session_key = make_key!("sessions/", hashed_jwt.clone());
    // db.set(session_key.0, encoded_session).unwrap();
    // let cookie = Cookie::build("session", hashed_jwt)
    //     .path("/")
    //     .secure(true)
    //     .http_only(true)
    //     .finish();
    // cookies.add(cookie);

    Ok(rocket::response::Redirect::to("/loggedin"))

}

// We separate creation of the Rocket instance from launch of the instance.
// This makes testing easier, less verbose, and less error-prone.

fn rocketeer() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![
        home,
        login,
        callback,
    ])
}

fn main() {
    rocketeer().launch();
}

#[cfg(test)]
mod tests {

    use super::*;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn test_home() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some(String::from("home")));
    }
}
