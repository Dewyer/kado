use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request, State};
use sha2::Sha256;

use crate::blocklists::BLOCK_LIST;
use crate::config::AppConfig;
use crate::db::transaction_manager::{ITransaction, TransactionManager};
use crate::db::user_repo::{DbUserRepo, IUserRepo};
use crate::errors::ServiceError;
use crate::guards::{AuthTokenGuard, RefreshToken};
use crate::models::http::access_claim::AccessClaim;
use crate::models::http::requests::{CheckUserRequest, LoginRequest, RegisterRequest};
use crate::models::http::responses::{AuthorizingResponse, CheckUserResponse};
use crate::models::user::{NewUser, User};
use crate::services::authenticators::models::{AuthenticationPayload, Authorizer};
use crate::services::authenticators::ExternalAuthenticatorService;
use crate::services::utils_service::UtilsService;

pub const REFRESH_TOKEN_EXPIRATION_SECONDS: u64 = 3 * 24 * 60 * 60;
// 3 days
pub const ACCESS_TOKEN_EXPIRATION_SECONDS: u64 = 12 * 60 * 60; // 12 hours

pub struct AuthService {
    config: AppConfig,
    user_repo: IUserRepo,
    external_authenticator_service: ExternalAuthenticatorService,
    tm: TransactionManager,
}

impl AuthService {
    pub fn new(
        config: AppConfig,
        user_repo: IUserRepo,
        external_authenticator_service: ExternalAuthenticatorService,
        tm: TransactionManager,
    ) -> Self {
        Self {
            config,
            user_repo,
            external_authenticator_service,
            tm,
        }
    }

    pub fn get_key_from_config(&self) -> anyhow::Result<Hmac<Sha256>> {
        Hmac::new_varkey(&self.config.secret).map_err(|_| anyhow::Error::msg("Cant get JWT key!"))
    }

    pub fn is_admin_key_correct(&self, key: String) -> bool {
        key == self.config.admin_key
    }

    fn sign_jwt_for_user(
        &self,
        user: &User,
        validity_duration_seconds: u64,
    ) -> anyhow::Result<String> {
        let now_stamp: u64 = chrono::Utc::now().timestamp() as u64;

        let claim = AccessClaim {
            user_id: user.id.to_string(),
            expires_at: now_stamp + validity_duration_seconds,
        };
        let hmac = self.get_key_from_config()?;

        claim
            .sign_with_key(&hmac)
            .map_err(|_| anyhow::Error::msg("Jwt signing failed!"))
    }

    fn verify_jwt_get_claim(&self, jwt_token: &str) -> anyhow::Result<AccessClaim> {
        let now_stamp: u64 = chrono::Utc::now().timestamp() as u64;
        let hmac = self.get_key_from_config()?;
        let claim: AccessClaim = jwt_token
            .verify_with_key(&hmac)
            .map_err(|_| anyhow::Error::msg("JWT token cannot be verified!"))?;

        if claim.expires_at < now_stamp {
            bail!("Refresh token expired!");
        }

        Ok(claim)
    }

    fn authorize_user(&self, user: &User) -> anyhow::Result<AuthorizingResponse> {
        let refresh_token = self.sign_jwt_for_user(user, REFRESH_TOKEN_EXPIRATION_SECONDS)?;
        let access_token = self.sign_jwt_for_user(user, ACCESS_TOKEN_EXPIRATION_SECONDS)?;

        Ok(AuthorizingResponse {
            access_token,
            user: user.to_dto(),
            refresh_token,
        })
    }

    /// Refresh or access works too
    pub fn get_authenticated_user_by_authorization_token(
        &self,
        refresh: String,
    ) -> anyhow::Result<User> {
        self.tm.transaction(|td| {
            let claim = self.verify_jwt_get_claim(&refresh)?;

            let user_uuid = UtilsService::parse_uuid(&claim.user_id)?;

            let user = self
                .user_repo
                .crud()
                .find_by_id(user_uuid, &td)
                .map_err(|_| anyhow::Error::msg("No user found for authorization token"))?;

            if !user.is_active {
                bail!("User has been disabled!");
            }

            Ok(user)
        })
    }

    pub fn authenticate_user_by_refresh_token(
        &self,
        refresh_guard: AuthTokenGuard<RefreshToken>,
    ) -> anyhow::Result<AuthorizingResponse> {
        self.authorize_user(&refresh_guard.user)
    }

    fn create_user(
        &self,
        payload: RegisterRequest,
        email: &str,
        tr: &ITransaction,
    ) -> anyhow::Result<AuthorizingResponse> {
        let created_user = self.user_repo.create(
            &NewUser {
                username: &payload.username,
                email,
                authenticator: &payload.authorizer.to_lowercase(),
                participate_in_leaderboards: payload.participate_in_leaderboards,
                is_active: true,
                is_admin: false,
                individual_points: 0,
                last_gained_points_at: None,
                created_at: UtilsService::naive_now(),
                team_id: None,
            },
            tr,
        )?;

        self.authorize_user(&created_user)
    }

    fn assert_username_not_blocked(&self, username: &str) -> anyhow::Result<()> {
        if username == "anonymous" {
            bail!("Your name can't be anonymous.");
        }

        for bl in BLOCK_LIST.iter() {
            if username.contains(bl) {
                return Err(anyhow::Error::msg(format!(
                    "Blocked, username contains: {}",
                    bl
                )));
            }
        }

        Ok(())
    }

    pub fn register_user(&self, payload: RegisterRequest) -> anyhow::Result<AuthorizingResponse> {
        self.tm.transaction(|tr| {
            let authorizer = Authorizer::from_string(payload.authorizer.clone())?;
            let external_auth_result =
                self.external_authenticator_service
                    .authenticate(AuthenticationPayload {
                        authorizer,
                        token: payload.token.clone(),
                    })?;

            self.assert_username_not_blocked(&payload.username)?;

            let existing_user = self.user_repo.find_by_email_or_username(
                &external_auth_result.email,
                &payload.username,
                &tr,
            );
            if existing_user.is_ok() {
                bail!("User already exists");
            }

            self.create_user(payload, &external_auth_result.email, &tr)
        })
    }

    pub fn login_user(&self, payload: LoginRequest) -> anyhow::Result<AuthorizingResponse> {
        self.tm.transaction(|tr| {
            let authorizer = Authorizer::from_string(payload.authorizer.clone())?;
            let external_auth_result =
                self.external_authenticator_service
                    .authenticate(AuthenticationPayload {
                        authorizer,
                        token: payload.token.clone(),
                    })?;

            let existing_user = self
                .user_repo
                .find_by_email_or_username(&external_auth_result.email, "", &tr)
                .map_err(|_| anyhow::Error::msg("Email address is not registered!"))?;

            if existing_user.authenticator != payload.authorizer {
                bail!("Uses different authenticator!");
            }

            if !existing_user.is_active {
                bail!("User is inactive!");
            }

            self.authorize_user(&existing_user)
        })
    }

    pub fn check_user(&self, payload: CheckUserRequest) -> anyhow::Result<CheckUserResponse> {
        self.tm.transaction(|tr| {
            let authorizer = Authorizer::from_string(payload.authorizer.clone())?;
            let external_auth_result =
                self.external_authenticator_service
                    .authenticate(AuthenticationPayload {
                        authorizer,
                        token: payload.token,
                    })?;

            let existing_user = self
                .user_repo
                .find_by_email_or_username(&external_auth_result.email, "", &tr)
                .ok();

            Ok(CheckUserResponse {
                user_exists: existing_user.is_some(),
                uses_different_authenticator: !existing_user
                    .as_ref()
                    .map(|uu| uu.authenticator.clone())
                    .contains(&payload.authorizer),
                user_inactive: existing_user.map_or(false, |usr| usr.is_active),
            })
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<AuthService, Self::Error> {
        let config = req.guard::<State<AppConfig>>().map_failure(|_| {
            (
                Status::InternalServerError,
                ServiceError::ServiceGuardFailed,
            )
        })?;
        let user_repo = req.guard::<DbUserRepo>()?;
        let external_authenticator_service = req.guard::<ExternalAuthenticatorService>()?;
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(AuthService::new(
            config.inner().clone(),
            Box::new(user_repo),
            external_authenticator_service,
            db_tm,
        ))
    }
}
