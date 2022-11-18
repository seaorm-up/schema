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
async fn test_db_relate() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
    let db = DBX.get().unwrap();
    let create_identity = format!("relate mobile:123->identity->account:hz");
    let v = db.raw_execute_one(create_identity.clone()).await.unwrap();
    let v2 = serialize(v.clone());
    let v3 = db.execute_one::<Relation>(create_identity).await;

    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    assert_debug_snapshot!((v, v2, v3));
}

#[derive(Debug, Clone, Serialize)]
pub struct Relation {
    pub id: Thing,
    #[serde(rename = "in")]
    pub from: Thing,
    #[serde(rename = "out")]
    pub with: Thing,
}

#[cfg(test)]
fn instance() -> Relation {
    Relation {
        id: thing("identity:1").unwrap(),
        from: thing("mobile:1").unwrap(),
        with: thing("account:1").unwrap(),
    }
}

#[tokio::test]
async fn test_relate() {
    // into value
    // dbg!(Into::<Value>::into(instance()));
    // // display
    // dbg!(format!("{}", Into::<Value>::into(instance())));
    // // into value and ser
    // dbg!(serialize(Into::<Value>::into(instance())));
    assert_debug_snapshot!((
        Into::<Value>::into(instance()),
        format!("{}", Into::<Value>::into(instance())),
        serialize(Into::<Value>::into(instance()))
    ));
}

impl Into<Value> for Relation {
    fn into(self) -> Value {
        Value::from(surrealdb::sql::Object::from(
            std::collections::BTreeMap::from([
                ("id".into(), self.id.into()),
                ("in".into(), self.from.into()),
                ("out".into(), self.with.into()),
            ]),
        ))
    }
}

impl CompressionWith for Relation {
    fn compression_with(value: Value) -> Self {
        dbg!(value.is_object());
        dbg!(&value);
        let tmp = surrealdb::sql::Object::try_from(value);
        let mut map = tmp.unwrap().0;
        Relation {
            id: map.remove("id").unwrap().record().unwrap(),
            from: map.remove("in").unwrap().record().unwrap(),
            with: map.remove("out").unwrap().record().unwrap(),
        }
    }
}

// impl<'de> Deserialize<'de> for Relation {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let mut map = surrealdb::sql::Object::try_from(value).unwrap().0;
//         Ok(Relation {
//             id: map.remove("id").unwrap().record().unwrap(),
//             from: map.remove("in").unwrap().record().unwrap(),
//             with: map.remove("out").unwrap().record().unwrap(),
//         })
//     }
// }
