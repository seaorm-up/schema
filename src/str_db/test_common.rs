#![cfg(test)]

use crate::*;

#[derive(Debug, Clone, Deserialize, Serialize, SurrealDbObject)]
pub struct App {
    pub id: String,
    pub name: String,
    pub opt_name: Option<String>,
    pub accounts: Vec<u64>,
}
pub fn instance() -> App {
    App {
        id: "app:i2esfym6g2fh06svw449".to_owned(),
        opt_name: None,
        // opt_name: Some("opt".to_owned()),
        name: "adf".to_owned(),
        accounts: vec![17682318111, 17681951111],
    }
}

#[test]
fn test_display() {
    let app = instance();
    let sql = format!("create app CONTENT {}", app);
    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    assert_debug_snapshot!(sql);
}

#[test]
fn test_into() {
    let app = instance();
    dbg!(&{
        let v: Value = app.into();
        v
    });
}
