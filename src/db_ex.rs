use crate::*;
use once_cell::sync::OnceCell;

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
    pub async fn check_exist(&self, txt: String) -> bool {
        self.raw_execute_one(txt).await.unwrap().len() > 0
    }
    pub async fn execute_one<T: CompressionWith>(&self, txt: String) -> Result<Vec<T>, Error> {
        let v = self
            .datastore
            .execute(&txt, &self.session, None, false)
            .await
            .unwrap();

        Ok(v.into_iter()
            .map(|response| response.result.unwrap())
            .next()
            .map(|result_value| Vec::<T>::compression_with(result_value))
            .unwrap())
    }
}

fn first(responses: Vec<surrealdb::Response>) -> Vec<Value> {
    responses
        .into_iter()
        .map(|response| response.result.unwrap())
        .next()
        .map(|result_value| Vec::<Value>::try_from(result_value).unwrap())
        .unwrap()
}

#[cfg(test)]
mod db_test;
