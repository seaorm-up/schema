use std::collections::BTreeMap;

use crate::*;
use once_cell::sync::OnceCell;
use surrealdb_schema_derive::SurrealValue;
type TxtQueryResult = (
    oneshot::Sender<Result<surrealdb::sql::Query, Error>>,
    String,
);
type AstQueryResult = (
    oneshot::Sender<Result<Vec<surrealdb::Response>, Error>>,
    surrealdb::sql::Query,
);
type TryFromResult<T> = (oneshot::Sender<Result<T, Error>>, Vec<surrealdb::Response>);
// type TryFromResult = Vec<surrealdb::Response>;

pub static DBX: OnceCell<DbX> = OnceCell::new();
type Variables = Option<BTreeMap<String, Value>>;
/// [Query::query_str] would get [surrealdb::sql::Query] first, and call [Query::query_ast]
/// [Query::query_ast], would return Result<Vec<Value>, Error>(Value is from [surrealdb::Response])
/// also impl query_first
// #[derive(Clone)]
pub struct DbX {
    pub db: Datastore,
    pub session: Session,
}

impl DbX {
    pub async fn new(namespace: String, database: String, path: String) -> Result<(), DbX> {
        let datastore = Datastore::new(&path).await.unwrap();
        let session = Session::for_db(namespace.to_string(), database.to_string());
        DBX.set(Self {
            db: datastore,
            session: session,
        })
    }

    pub async fn execute<T: TryFrom<SurrealValue>>(
        &self,
        txt: &str,
        sess: &Session,
        vars: Variables,
        strict: bool,
    ) -> Result<Vec<T>, Error>
    where
        <T as TryFrom<SurrealValue>>::Error: std::error::Error,
        <T as TryFrom<SurrealValue>>::Error: Send,
        <T as TryFrom<SurrealValue>>::Error: Sync,
        <T as TryFrom<SurrealValue>>::Error: 'static,
    {
        let v = self.db.execute(txt, sess, vars, strict).await.unwrap();
        let v = to_objects(to_values(v));
        Ok(v)
        // Ok(to_objects(  to_values(v)))
    }

    pub async fn create<T: SurrealDbObject, U>(
        &self,
        vars: BTreeMap<String, Value>,
    ) -> Result<Vec<T>, Error> {
        let sql = "create $table content $vars";

        let vars = map! {
            String::from("table") => Value::from(T::get_table_name()),
            String::from("vars") => Value::from(vars),
        };
        self.execute::<T>(sql, &self.session, Some(vars), false)
            .await
    }

    pub async fn delete_by_id<T: SurrealDbObject, U>(&self, rid: Value) -> Result<Vec<T>, Error> {
        let sql = "Delete type::thing($table, $id)";

        let vars = map! {
            String::from("table") => Value::from(T::get_table_name()),
            String::from("id") => rid,
        };
        self.execute::<T>(sql, &self.session, Some(vars), false)
            .await
        // Ok(())
    }

    pub async fn update<T: SurrealDbObject, U>(
        &self,
        rid: Value,
        mut vars: BTreeMap<String, Value>,
    ) -> Result<Vec<T>, Error> {
        let sql = "UPDATE type::thing($table, $id) MERGE $vars";

        let vars = map! {
            String::from("table") => Value::from(T::get_table_name()),
            String::from("id") => rid,
            String::from("vars") => Value::from(vars),
        };
        self.execute::<T>(sql, &self.session, Some(vars), false)
            .await
    }
    pub async fn get_by_id<T: SurrealDbObject>(&self, rid: Value) -> Result<T, Error> {
        let sql = "SELECT * FROM type::thing($table, $id)";

        let vars = map! {
            String::from("table") => Value::from(T::get_table_name()),
            String::from("id") => rid,
        };
        Ok(self
            .execute::<T>(sql, &self.session, Some(vars), false)
            .await
            .unwrap()
            .remove(0))
    }

    pub async fn getx<T: SurrealDbObject>(
        &self,
        mut vars: BTreeMap<String, Value>,
    ) -> Result<T, Error> {
        let sql = "SELECT $fields FROM type::thing($table, $id)";
        vars.insert("table".to_owned(), Value::from(T::get_table_name()));

        Ok(self
            .execute::<T>(sql, &self.session, Some(vars), false)
            .await
            .unwrap()
            .remove(0))
    }

    pub async fn get_list<T: SurrealDbObject, U>(&self) -> Vec<T> {
        let sql = "SELECT * FROM $table";

        let vars = map! {
            String::from("table") => Value::from(T::get_table_name()),
            // String::from("id") => rid,
        };
        self.execute::<T>(sql, &self.session, Some(vars), false)
            .await
            .unwrap()
    }
}

fn to_objects<T: TryFrom<SurrealValue>>(responses: Vec<Value>) -> Vec<T>
where
    <T as TryFrom<SurrealValue>>::Error: std::error::Error,
    <T as TryFrom<SurrealValue>>::Error: Send,
    <T as TryFrom<SurrealValue>>::Error: Sync,
    <T as TryFrom<SurrealValue>>::Error: 'static,
{
    responses
        .into_iter()
        .map(|response| SurrealValue(response).try_into().unwrap())
        .collect::<Vec<T>>()
}

fn to_values(responses: Vec<surrealdb::Response>) -> Vec<Value> {
    responses
        .into_iter()
        .map(|response| response.result.unwrap())
        .collect::<Vec<Value>>()
}
