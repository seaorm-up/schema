* usage
```rs
let v = "insert into mobile {id:123,num:123}"
        .execute_one::<Mobile>()
        .await;
```
or something ref to 
```rs
#[async_trait]
pub trait StrDb {
    async fn execute_silent(&self) -> bool;
    async fn raw_execute_one(&self) -> Result<Vec<Value>, Error>;
    async fn execute_one<T: TryFrom<SurrealValue>>(&self) -> Vec<T>
    where
        <T as TryFrom<SurrealValue>>::Error: std::error::Error,
        <T as TryFrom<SurrealValue>>::Error: Send,
        <T as TryFrom<SurrealValue>>::Error: Sync,
        <T as TryFrom<SurrealValue>>::Error: 'static;
}
```

* release
```
cargo release publish --workspace --execute
```