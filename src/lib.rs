pub use db_dep::*;
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
#[macro_export]
macro_rules! set_filters {
    ($regex:literal, $replace:literal,) => {
        let mut settings = insta::Settings::clone_current();
        settings.add_filter($regex, $replace);
        let _guard = settings.bind_to_scope();
    };
}

pub use async_trait::async_trait;
#[cfg(feature = "bincode")]
pub use bincode;
pub use serde::{self, Deserialize, Serialize};
#[cfg(feature = "serde_json")]
pub use serde_json;

pub use surrealdb::sql::{thing, Array, Id, Thing, Value, Values};
pub use surrealdb::{Datastore, Error, Session};
pub use surrealdb_obj_derive::{IntoValue, SurrealDbObject};
pub use tokio::sync::{mpsc, oneshot};

mod db_ex;
pub use db_ex::*;
mod relate_ex;
pub use relate_ex::*;
mod str_db;
pub use str_db::*;
mod macro_model;
pub use macro_model::*;

pub use paste::paste;

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }
