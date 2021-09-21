use crate::config::AppConfig;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request, State};
use rocket::http::Status;
use crate::services::authenticators::models::{AuthenticationResult, AuthenticationPayload};
use crate::services::authenticators::authenticator::Authenticator;
use std::collections::HashMap;
use reqwest::header;

pub struct GithubAuthenticatorService {
    config: AppConfig,
}

impl GithubAuthenticatorService {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct UserResponse {
    email: String,
}

impl GithubAuthenticatorService {
    pub fn exchange_code_for_token(&self, code: &str) -> anyhow::Result<String> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", header::HeaderValue::from_static("application/json"));

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        let mut bdy = HashMap::new();
        bdy.insert("code", code.to_string());
        bdy.insert("client_id", self.config.github_client_id.clone());
        bdy.insert("client_secret", self.config.github_secret.clone());
        bdy.insert("redirect_uri", self.config.github_redirect_url.clone());

        let resp: HashMap<String, String> = client.post("https://github.com/login/oauth/access_token")
            .json(&bdy)
            .send()
            .map_err(|_| anyhow::Error::msg("Couldn make request to github!"))?
            .json::<HashMap<String, String>>()
            .map_err(|_| anyhow::Error::msg("Couldnt parse request from github!"))?;

        resp.get("access_token")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow::Error::msg("No access token in response!"))
    }

    fn get_user_email_from_token(&self, token: &str) -> anyhow::Result<String> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static("snapchallange - Dewyer"));
        headers.insert("Accept", header::HeaderValue::from_static("application/json"));
        headers.insert("Authorization", header::HeaderValue::from_str(&format!("token {}", token)).map_err(|_| anyhow::Error::msg("No header!"))?);

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        let resp_one = client.get("https://api.github.com/user")
            .send()
            .map_err(|_| anyhow::Error::msg("Couldn make request to github!"))?;

        let resp = resp_one
            .json::<UserResponse>()
            .map_err(|_| anyhow::Error::msg("Couldn parse request from github!"))?;

        Ok(resp.email)
    }
}

impl Authenticator for GithubAuthenticatorService {
    fn authenticate(&self, payload: AuthenticationPayload) -> anyhow::Result<AuthenticationResult> {
        let email = self.get_user_email_from_token(&payload.token)?;

        Ok(
            AuthenticationResult {
                email,
            }
        )
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for GithubAuthenticatorService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<GithubAuthenticatorService, Self::Error> {
        let config = req.guard::<State<AppConfig>>().map_failure(|_| {
            (
                Status::InternalServerError,
                ServiceError::ServiceGuardFailed,
            )
        })?;

        request::Outcome::Success(GithubAuthenticatorService::new(
            config.inner().clone(),
        ))
    }
}
