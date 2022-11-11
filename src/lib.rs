pub use db_dep::*;
pub use dbs::DB;
pub use insta;
pub use insta::assert_debug_snapshot;

#[macro_export]
macro_rules! set_snapshot_suffix {
  ($($expr:expr),*) => {
      let mut settings = insta::Settings::clone_current();
      settings.set_snapshot_suffix(format!($($expr,)*));
      let _guard = settings.bind_to_scope();
  }
}
// pub use surreal_simple_querybuilder;
// pub use surreal_simple_querybuilder::prelude::*;
pub use surrealdb::sql::{thing, Id, Thing, Value, Values};
pub use surrealdb::{Datastore, Error, Session};
pub use surrealdb_schema_derive;
pub use surrealdb_schema_derive::anyhow::Result;
pub use surrealdb_schema_derive::async_trait::async_trait;
pub use surrealdb_schema_derive::SurrealValue;
pub use surrealdb_schema_derive::{SurrealDbObject, SurrealDbTable};
pub use tokio::sync::{mpsc, oneshot};
// pub use {SurrealDbObject, SurrealDbTable};

// mod query;
// pub use query::*;
mod db_ex;
pub use db_ex::*;
mod relate_ex;
pub use relate_ex::*;
mod str_db;
pub use str_db::*;

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }
// pub use map;
