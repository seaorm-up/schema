use crate::*;

#[async_trait]
pub trait StrDb {
    async fn execute_silent(&self) -> bool;
    async fn raw_execute_one(&self) -> Result<Vec<Value>, Error>;
    async fn check_exist(&self) -> bool;
    async fn execute_one<T: CompressionWith>(&self) -> Vec<T>;
}
#[async_trait]
impl StrDb for &str {
    async fn execute_silent(&self) -> bool {
        DBX.get().unwrap().execute_silent(self.to_string()).await
    }
    async fn raw_execute_one(&self) -> Result<Vec<Value>, Error> {
        DBX.get().unwrap().raw_execute_one(self.to_string()).await
    }
    async fn check_exist(&self) -> bool {
        DBX.get().unwrap().check_exist(self.to_string()).await
    }
    async fn execute_one<T: CompressionWith>(&self) -> Vec<T> {
        DBX.get()
            .unwrap()
            .execute_one::<T>(self.to_string())
            .await
            .unwrap()
    }
}

#[async_trait]
impl StrDb for String {
    async fn execute_silent(&self) -> bool {
        self.as_str().execute_silent().await
    }
    async fn raw_execute_one(&self) -> Result<Vec<Value>, Error> {
        self.as_str().raw_execute_one().await
    }
    async fn check_exist(&self) -> bool {
        self.as_str().check_exist().await
    }
    async fn execute_one<T: CompressionWith>(&self) -> Vec<T> {
        self.as_str().execute_one::<T>().await
    }
}

#[cfg(feature = "test")]
pub async fn new_db() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
}

mod test_common;
#[cfg(test)]
pub use test_common::*;

#[tokio::test]
async fn test_compression_with_with_execute_one() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();

    let app = instance();

    let ret = format!("insert into app {}", app)
        .execute_one::<App>()
        .await;

    assert_debug_snapshot!(ret);
}
