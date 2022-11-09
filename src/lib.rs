pub use db_dep::*;
pub use dbs::DB;
// pub use surreal_simple_querybuilder;
// pub use surreal_simple_querybuilder::prelude::*;

pub use surrealdb::{
    sql::{thing, Thing, Value},
    Datastore, Error, Session,
};
pub use surrealdb_schema_derive;
pub use surrealdb_schema_derive::{SurrealDbObject, SurrealDbTable};
pub use tokio::sync::{mpsc, oneshot};
// pub use {SurrealDbObject, SurrealDbTable};

mod query;
pub use query::*;
mod db_ex;
pub use db_ex::*;

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }
// pub use map;
