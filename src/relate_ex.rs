use crate::*;
pub async fn raw_db_relate(from: &str, table: &str, with: &str) -> Vec<Value> {
    let db = DBX.get().unwrap();
    let create_identity = format!("relate {from}->{table}->{with}");
    let vs = db.raw_execute_one(create_identity).await.unwrap();
    dbg!(&vs);
    vs
}
pub async fn db_relate(from: &str, table: &str, with: &str) -> Vec<Relation> {
    let db = DBX.get().unwrap();
    let create_identity = format!("relate {from}->{table}->{with}");
    let vs = db.execute_one::<Relation>(create_identity).await.unwrap();
    dbg!(&vs);
    vs
}

#[tokio::test]
async fn test_raw_db_relate() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
    let db = DBX.get().unwrap();
    let create_identity = format!("relate mobile:123->identity->account:hz");
    let v = db.raw_execute_one(create_identity).await.unwrap();
    // dbg!(&v);
    assert_debug_snapshot!(v);
}
#[tokio::test]
async fn test_db_relate() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
    let db = DBX.get().unwrap();
    let create_identity = format!("relate mobile:123->identity->account:hz");
    let v = db.execute_one::<Relation>(create_identity).await;
    assert_debug_snapshot!(v);
}

#[derive(Debug, Clone)]
pub struct Relation {
    pub id: Thing,
    pub from: Thing,
    pub with: Thing,
}

impl Relation {
    pub async fn save(&self) -> Vec<Relation> {
        db_relate(&self.from.to_string(), &self.id.tb, &self.with.to_string()).await
    }
}

impl TryFrom<surrealdb_obj_derive::SurrealValue> for Relation {
    type Error = surrealdb_obj_derive::SurrealDbSchemaDeriveQueryError;
    fn try_from(value: surrealdb_obj_derive::SurrealValue) -> Result<Self, Self::Error> {
        if let surrealdb_obj_derive::surrealdb::sql::Value::Object(object_value) = value.0 {
            Ok(Self {
                id: surrealdb_obj_derive::SurrealValue(object_value.0.get("id").unwrap().clone())
                    .try_into()?,
                from: surrealdb_obj_derive::SurrealValue(object_value.0.get("in").unwrap().clone())
                    .try_into()?,
                with: surrealdb_obj_derive::SurrealValue(
                    object_value.0.get("out").unwrap().clone(),
                )
                .try_into()?,
            })
        } else {
            Err(
                surrealdb_obj_derive::SurrealDbSchemaDeriveQueryError::InvalidValueTypeError(
                    surrealdb_obj_derive::InvalidValueTypeError {
                        expected_type: "Relation".into(),
                        received_type: value.0.to_string(),
                    },
                ),
            )
        }
    }
}
impl Into<surrealdb_obj_derive::SurrealValue> for Relation {
    fn into(self) -> surrealdb_obj_derive::SurrealValue {
        surrealdb_obj_derive::SurrealValue(surrealdb_obj_derive::surrealdb::sql::Value::Object(
            surrealdb_obj_derive::surrealdb::sql::Object(std::collections::BTreeMap::from([
                ("id".into(), {
                    let surreal_value: surrealdb_obj_derive::SurrealValue = self.id.into();
                    surreal_value.into()
                }),
                ("in".into(), {
                    let surreal_value: surrealdb_obj_derive::SurrealValue = self.from.into();
                    surreal_value.into()
                }),
                ("out".into(), {
                    let surreal_value: surrealdb_obj_derive::SurrealValue = self.with.into();
                    surreal_value.into()
                }),
            ])),
        ))
    }
}
