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
    let v = db.execute_one::<Relation>(create_identity).await;

    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    assert_debug_snapshot!(v);
}

#[derive(Debug, Clone, Deserialize, Serialize, SurrealDbObject)]
pub struct Relation {
    pub id: Thing,
    #[serde(rename = "in")]
    pub from: Thing,
    #[serde(rename = "out")]
    pub with: Thing,
}

impl Relation {
    pub async fn save(&self) -> Vec<Relation> {
        db_relate(&self.from.to_string(), &self.id.tb, &self.with.to_string()).await
    }
}
