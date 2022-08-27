// use diesel::prelude::*;

#[macro_export]
macro_rules! crud_repo {
    ($name: ident,$dbname: ident, $table: ident, $trec: ty, $tnew: ty, $nameStr: expr) => {
        pub trait $name {
            fn list(
                &self,
                tm: &crate::db::transaction_manager::ITransaction,
            ) -> anyhow::Result<Vec<$trec>>;

            fn insert(
                &self,
                new_rec: &$tnew,
                tm: &crate::db::transaction_manager::ITransaction,
            ) -> anyhow::Result<$trec>;

            fn find_by_id(
                &self,
                id: uuid::Uuid,
                tm: &crate::db::transaction_manager::ITransaction,
            ) -> anyhow::Result<$trec>;

            fn clear(
                &self,
                td: &crate::db::transaction_manager::ITransaction,
            ) -> anyhow::Result<usize>;
        }

        pub struct $dbname {}

        impl $name for $dbname {
            fn list(
                &self,
                tm: &crate::db::transaction_manager::ITransaction,
            ) -> anyhow::Result<Vec<$trec>> {
                $table::table
                    .select($table::all_columns)
                    .load::<$trec>(tm.get_db_connection())
                    .map_err(|_| anyhow::Error::msg(format!("Geting {} failed!", $nameStr)))
            }

            fn insert(
                &self,
                new_rec: &$tnew,
                tm: &crate::db::transaction_manager::ITransaction,
            ) -> anyhow::Result<$trec> {
                diesel::insert_into($table::table)
                    .values(new_rec)
                    .get_result(tm.get_db_connection())
                    .map_err(|_| anyhow::Error::msg(format!("Couldn't insert {}!", $nameStr)))
            }

            fn find_by_id(
                &self,
                id: uuid::Uuid,
                tm: &crate::db::transaction_manager::ITransaction,
            ) -> anyhow::Result<$trec> {
                $table::table
                    .select($table::all_columns)
                    .filter($table::id.eq(id))
                    .first::<$trec>(tm.get_db_connection())
                    .map_err(|_| anyhow::Error::msg(format!("{} not found!", $nameStr)))
            }

            fn clear(
                &self,
                td: &crate::db::transaction_manager::ITransaction,
            ) -> anyhow::Result<usize> {
                diesel::delete($table::table)
                    .execute(td.get_db_connection())
                    .map_err(|_| anyhow::Error::msg(format!("{} table cant be cleared!", $nameStr)))
            }
        }
    };
}
