use crate::db::transaction_manager::{TransactionManager, ITransaction};
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};
use crate::db::api_token_repo::{IApiTokenRepo, DbApiTokenRepo};
use crate::models::http::responses::GetApiTokenResponse;
use crate::guards::{AuthTokenGuard, AccessToken};
use crate::models::user::User;
use crate::models::api_token::{ApiToken, NewApiToken};
use crate::services::crypto_service::CryptoService;

pub struct ApiTokenService {
    api_token_repo: IApiTokenRepo,
    tm: TransactionManager,
}

impl ApiTokenService {
    pub fn new(
        api_token_repo: IApiTokenRepo,
        tm: TransactionManager,
    ) -> Self {
        Self {
            api_token_repo,
            tm,
        }
    }

    fn generate_new_api_token_for_user(&self, user: &User, td: &ITransaction) -> anyhow::Result<ApiToken> {
        self.api_token_repo.soft_delete_all_tokens_for_user(user.id, td)?;

        self.api_token_repo.crud().insert(&NewApiToken {
            is_deleted: false,
            owner_id: user.id,
            token: &CryptoService::get_random_string(16),
        }, td)
    }

    fn ensure_and_get_api_token_for_user(&self, user: &User, td: &ITransaction) -> anyhow::Result<ApiToken> {
        let existing_token = self.api_token_repo.find_api_token_by_user(user.id, td);
        if let Ok(token) = existing_token {
            Ok(token)
        } else {
            self.generate_new_api_token_for_user(user, td)
        }
    }

    pub fn get_api_token_for_user(&self, user_guard: AuthTokenGuard<AccessToken>) -> anyhow::Result<GetApiTokenResponse> {
        self.tm.transaction(|td| {
            let users_token = self.ensure_and_get_api_token_for_user(&user_guard.user, &td)?;

            Ok(GetApiTokenResponse {
                token: users_token.to_dto(),
            })
        })
    }

    pub fn refresh_api_token_for_user(&self, user_guard: AuthTokenGuard<AccessToken>) -> anyhow::Result<GetApiTokenResponse> {
        self.tm.transaction(|td| {
            let users_token = self.generate_new_api_token_for_user(&user_guard.user, &td)?;

            Ok(GetApiTokenResponse {
                token: users_token.to_dto(),
            })
        })
    }

    pub fn get_authenticated_user_by_api_token(&self, token: String) -> anyhow::Result<User> {
        self.tm.transaction(|td| {
            let token_and_user = self.api_token_repo.find_with_user_by_token(&token, &td)?;
            Ok(token_and_user.1)
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiTokenService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<ApiTokenService, Self::Error> {
        let api_token_repo = req.guard::<DbApiTokenRepo>()?;
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(ApiTokenService::new(
            Box::new(api_token_repo),
            db_tm,
        ))
    }
}
