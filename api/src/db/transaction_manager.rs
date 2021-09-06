use std::rc::Rc;

use rocket::{request, Request};
use rocket::request::FromRequest;

use crate::db::{db_connection_guard, DbConn};
use crate::diesel::Connection;
use crate::errors::ServiceError;

pub trait Transaction {
    fn get_db_connection(&self) -> &DbConn;
}

pub type ITransaction = Box<dyn Transaction>;

pub struct DbTransaction {
    conn: Rc<DbConn>,
}

impl DbTransaction {
    pub fn new(conn: Rc<DbConn>) -> Self {
        Self { conn }
    }
}

impl Transaction for DbTransaction {
    fn get_db_connection(&self) -> &DbConn {
        return &self.conn;
    }
}

pub struct TransactionManager {
    conn: Option<Rc<DbConn>>,
}

impl TransactionManager {
    pub fn new(conn: DbConn) -> Self {
        Self {
            conn: Some(Rc::new(conn)),
        }
    }

    pub fn testing() -> Self {
        Self { conn: None }
    }

    pub fn transaction<T, E, F>(&self, f: F) -> Result<T, E>
        where
            F: FnOnce(ITransaction) -> Result<T, E>,
            E: std::convert::From<diesel::result::Error>,
    {
        if let Some(conn) = self.conn.as_ref() {
            conn.transaction::<T, E, _>(|| {
                let tr = DbTransaction::new(conn.clone());

                f(Box::new(tr))
            })
        } else {
            return Err(diesel::result::Error::NotFound.into());
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for TransactionManager {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<TransactionManager, Self::Error> {
        let db_conn = db_connection_guard(req)?;
        request::Outcome::Success(TransactionManager::new(db_conn))
    }
}
