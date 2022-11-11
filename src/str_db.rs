use crate::*;

#[async_trait]
pub trait StrDb {
    async fn execute_silent<T: TryFrom<SurrealValue>>(&self) -> bool;
    async fn raw_execute_one(&self) -> Result<Vec<Value>, Error>;
    async fn execute_one<T: TryFrom<SurrealValue>>(&self) -> Result<Vec<T>, Error>
    where
        <T as TryFrom<SurrealValue>>::Error: std::error::Error,
        <T as TryFrom<SurrealValue>>::Error: Send,
        <T as TryFrom<SurrealValue>>::Error: Sync,
        <T as TryFrom<SurrealValue>>::Error: 'static;
}
#[async_trait]
impl StrDb for &str {
    async fn execute_silent<T: TryFrom<SurrealValue>>(&self) -> bool {
        DBX.get()
            .unwrap()
            .execute_silent::<T>(self.to_string())
            .await
    }
    async fn raw_execute_one(&self) -> Result<Vec<Value>, Error> {
        DBX.get().unwrap().raw_execute_one(self.to_string()).await
    }
    async fn execute_one<T: TryFrom<SurrealValue>>(&self) -> Result<Vec<T>, Error>
    where
        <T as TryFrom<SurrealValue>>::Error: std::error::Error,
        <T as TryFrom<SurrealValue>>::Error: Send,
        <T as TryFrom<SurrealValue>>::Error: Sync,
        <T as TryFrom<SurrealValue>>::Error: 'static,
    {
        DBX.get().unwrap().execute_one::<T>(self.to_string()).await
    }
}

#[async_trait]
impl StrDb for String {
    async fn execute_silent<T: TryFrom<SurrealValue>>(&self) -> bool {
        self.as_str().execute_silent::<T>().await
    }
    async fn raw_execute_one(&self) -> Result<Vec<Value>, Error> {
        self.as_str().raw_execute_one().await
    }
    async fn execute_one<T: TryFrom<SurrealValue>>(&self) -> Result<Vec<T>, Error>
    where
        <T as TryFrom<SurrealValue>>::Error: std::error::Error,
        <T as TryFrom<SurrealValue>>::Error: Send,
        <T as TryFrom<SurrealValue>>::Error: Sync,
        <T as TryFrom<SurrealValue>>::Error: 'static,
    {
        self.as_str().execute_one::<T>().await
    }
}