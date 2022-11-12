use crate::*;
use once_cell::sync::OnceCell;
use surrealdb_obj_derive::SurrealValue;

pub static DBX: OnceCell<DbX> = OnceCell::new();

pub struct DbX {
    pub datastore: Datastore,
    pub session: Session,
}

impl DbX {
    pub async fn new(namespace: String, database: String, path: String) -> Result<(), DbX> {
        let datastore = Datastore::new(&path).await.unwrap();
        let session = Session::for_db(namespace.to_string(), database.to_string());
        DBX.set(Self {
            datastore,
            session: session,
        })
    }

    /// silent execute
    pub async fn execute_silent(&self, txt: String) -> bool {
        let responses = self
            .datastore
            .execute(&txt, &self.session, None, false)
            .await
            .unwrap();
        responses
            .into_iter()
            .all(|response| response.result.is_ok())
    }

    pub async fn raw_execute_one(&self, txt: String) -> Result<Vec<Value>, Error> {
        Ok(first(
            self.datastore
                .execute(&txt, &self.session, None, false)
                .await
                .unwrap(),
        ))
    }
    pub async fn execute_one<T: TryFrom<SurrealValue>>(&self, txt: String) -> Result<Vec<T>, Error>
    where
        <T as TryFrom<SurrealValue>>::Error: std::error::Error,
        <T as TryFrom<SurrealValue>>::Error: Send,
        <T as TryFrom<SurrealValue>>::Error: Sync,
        <T as TryFrom<SurrealValue>>::Error: 'static,
    {
        let v = self
            .datastore
            .execute(&txt, &self.session, None, false)
            .await
            .unwrap();
        Ok(to_objects(first(v)))
    }
}

fn to_objects<T: TryFrom<SurrealValue>>(values: Vec<Value>) -> Vec<T>
where
    <T as TryFrom<SurrealValue>>::Error: std::error::Error,
    <T as TryFrom<SurrealValue>>::Error: Send,
    <T as TryFrom<SurrealValue>>::Error: Sync,
    <T as TryFrom<SurrealValue>>::Error: 'static,
{
    values
        .into_iter()
        .map(|value| SurrealValue(value).try_into().unwrap())
        .collect::<Vec<T>>()
}

fn first(responses: Vec<surrealdb::Response>) -> Vec<Value> {
    responses
        .into_iter()
        .map(|response| response.result.unwrap())
        .next()
        .map(|result_value| Vec::<Value>::try_from(result_value).unwrap())
        .unwrap()
}
