use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{request, Request};

use crate::crud_repo;
use crate::db::transaction_manager::ITransaction;
use crate::errors::ServiceError;
use crate::schema::api_tokens;
use crate::models::api_token::{NewApiToken, ApiToken};
use uuid::Uuid;
use crate::models::user::User;
use crate::schema::users;

crud_repo!(ApiTokenCrudRepo, DbApiTokenCrudRepo, api_tokens, ApiToken, NewApiToken, "ApiTokens");
pub type IApiTokenCrudRepo = Box<dyn ApiTokenCrudRepo>;

pub trait ApiTokenRepo {
    fn crud(&self) -> &IApiTokenCrudRepo;

    fn save(&self, apiToken: &ApiToken, td: &ITransaction) -> anyhow::Result<ApiToken>;

    fn find_api_token_by_user(&self, user_id: Uuid, td: &ITransaction) -> anyhow::Result<ApiToken>;

    fn soft_delete_all_tokens_for_user(&self, user_id: Uuid, td: &ITransaction) -> anyhow::Result<()>;

    fn find_with_user_by_token(&self, token: &str, td: &ITransaction) -> anyhow::Result<(ApiToken, User)>;
}

pub struct DbApiTokenRepo {
    crud: IApiTokenCrudRepo,
}

impl DbApiTokenRepo {
    pub fn new() -> Self {
        Self {
            crud: Box::new(DbApiTokenCrudRepo {}),
        }
    }
}

impl ApiTokenRepo for DbApiTokenRepo {
    fn crud(&self) -> &IApiTokenCrudRepo {
        &self.crud
    }

    fn save(&self, apiToken: &ApiToken, td: &ITransaction) -> anyhow::Result<ApiToken> {
        diesel::update(api_tokens::table)
            .filter(api_tokens::id.eq(apiToken.id))
            .set(apiToken)
            .get_result(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Can't save ApiToken!"))
    }

    fn find_api_token_by_user(&self, user_id: Uuid, td: &ITransaction) -> anyhow::Result<ApiToken> {
        api_tokens::table
            .select(api_tokens::all_columns)
            .filter(api_tokens::owner_id.eq(user_id).and(api_tokens::is_deleted.eq(false)))
            .first::<ApiToken>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("ApiToken not found!"))
    }

    fn soft_delete_all_tokens_for_user(&self, user_id: Uuid, td: &ITransaction) -> anyhow::Result<()> {
        diesel::update(api_tokens::table)
            .filter(api_tokens::owner_id.eq(user_id))
            .set((api_tokens::is_deleted.eq(true)))
            .execute(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Can't save ApiTokens!"))?;

        Ok(())
    }

    fn find_with_user_by_token(&self, token: &str, td: &ITransaction) -> anyhow::Result<(ApiToken, User)> {
        api_tokens::table
            .inner_join(users::table)
            .filter(
                api_tokens::is_deleted.eq(false)
                    .and(api_tokens::token.eq(token))
            ).first::<(ApiToken, User)>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Api token with user not found!"))
    }
}

pub type IApiTokenRepo = Box<dyn ApiTokenRepo>;

impl<'a, 'r> FromRequest<'a, 'r> for DbApiTokenRepo {
    type Error = ServiceError;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<DbApiTokenRepo, Self::Error> {
        request::Outcome::Success(DbApiTokenRepo::new())
    }
}
