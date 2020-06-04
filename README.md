# Demo Rust Rocket Auth0

* [How to create this project](#how-to-create-this-project)
  * [Create a Rust app](#create-a-rust-app)
  * [Sign up with Auth0](#sign-up-with-auth0)
  * [Create an Auth0 application](#create-an-auth0-application)
  * [Auth0 application settings](#auth0-application-settings)
  * [Get Your Application Keys](#get-your-application-keys)
  * [Configure Callback URLs](#configure-callback-urls)
  * [JSON Web Key Set (JWKS)](#json-web-key-set-jwks)
  * [Validation function](#validation-function)
* [State Parameter](#state-parameter)
  * [CSRF attacks](#csrf-attacks)


## How to create this project


### Create a Rust app

Create:
```
cargo new demo_rust_rocket_auth0
cd demo_rust_rocket_auth0
cargo build
cargo test
```

### Sign up with Auth0

Sign up with Auth0 at https://auth0.com/

Create an API https://manage.auth0.com/dashboard/us/joelparkerhenderson/apis

Auth0 will generate these:

* API id: such as "0f4f6ff2c7aaa1de6a9b".

* Name: such as "Auth0 Management API". 

* Identifier: such as "https://joelparkerhenderson.auth0.com/api/v2/"


### Create an Auth0 application

Choose the Auth0 Applications menu item.

Click "Create Application" then:

* Name: `Demo Rust Rocket Auth0 JWT`

* Choose an application type: `Regular Web Applications`

* Click "Create".


### Auth0 application settings

Click the "Settings" tab and adjust your settings as you want.

Basic information:

* Name: `Demo Rust Rocket Auth0 JWT`

* Domain: `joelparkerhenderson.auth0.com`

* Client id: `C9w1oG5e5Gevay5CNmdgiOaS1kG3oqH`

* Client secret: `******************`

Application URIs:

* Allowed Callback URLs: http://localhost:8000/callback

* Allowed Logout URLs: http://localhost:8000

Note: we prefer to use port 8000 because that's the Rocket default port.


### Get Your Application Keys

When you signed up for Auth0, a new application was created for you, or you
could have created a new one.

You will need some details about that application to communicate with Auth0. You
can get these details from the Application Settings section in the Auth0
dashboard.

You need the following information:

* Domain

* Client ID

* Client Secret

In our app, we prefer to set these via environment variables.

The domain means the domain we want to validate our token against. Any token that is not issued by this domain should fail validation. 


### Configure Callback URLs

A callback URL is a URL in your application where Auth0 redirects the user after
they have authenticated.

The callback URL for your app must be whitelisted in the Allowed Callback URLs
field in your Application Settings. If this field is not set, users will be
unable to log in to the application and will get an error.

If you are following along with the sample project you downloaded from the top
of this page, the callback URL you need to whitelist in the Allowed Callback
URLs field is http://localhost:3000/callback.


### JSON Web Key Set (JWKS)

The JSON Web Key Set (JWKS) is a set of keys which contains the public keys used to verify any JSON Web Token (JWT) issued by the authorization server.


### Validation function

We do these steps:

* Get the authority domain via environment variables

* Fetch the JSON Web Key Set to validate our token from Auth0. 

* Use the crate `alcoholic_jwt` for the actual validation. 

* Return a boolean indicating the validation result.


## State Parameter

Authorization protocols provide a state parameter that allows you to restore the previous state of your application. The state parameter preserves some state object set by the client in the Authorization request and makes it available to the client in the response.


### CSRF attacks

The primary reason for using the state parameter is to mitigate CSRF attacks.

When you use state for CSRF mitigation on the redirection endpoint, that means that within the state value there is a unique and non-guessable value associated with each authentication request about to be initiated. It’s that unique and non-guessable value that allows you to prevent the attack by confirming if the value coming from the response matches the one you expect (the one you generated when initiating the request). The state parameter is a string so you can encode any other information in it.

The way this works is that you send a random value when starting an authentication request and validate the received value when processing the response. This requires you to store something on the client application side (such as in a session, or cookie, or browser local storage, or another medium) that allows you to perform the validation.

 If you receive a response with a state that does not match, then you may be the target of an attack because this is either a response for an unsolicited request or someone trying to forge the response.

For the most basic cases the state parameter should be a nonce, used to correlate the request with
the response received from the authentication.

Most modern OIDC and OAuth2 SDKs, including Auth0.js in single-page
applications, handle the state generation and validation automatically.

1. Before redirecting a request to the Identity Provider, have the application
   generate a random string.

2. Stqore this string locally. Choose a method based on your type of application. For example:

    * For regular web apps, use a cookie or session

    * For a single-page app, use local storage in the browser

    * For a native app, use memory or local storage

3. Add the state parameter to the request. Use URL-encoding if necessary. Send the request..

4. Auth0 handles the request, and redirects the user back to the application. The state value will be included in this redirect. Note that depending on the type of connection used, this value might be in the body of the request or in the query string.

5. Retrieve the returned state value and compare it with the one you stored earlier. If the values match, then approve the authentication response, else deny it.
