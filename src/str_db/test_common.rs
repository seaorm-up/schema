#![cfg(test)]

use crate::*;

#[derive(Debug, Clone, Deserialize, Serialize, SurrealDbObject)]
pub struct App {
    pub id: String,
    pub name: String,
}
pub fn instance() -> App {
    App {
        id: "app:i2esfym6g2fh06svw449".to_owned(),
        name: "adf".to_owned(),
    }
}
#[test]
fn test_display() {
    let app = instance();
    let sql = format!(
        "create app CONTENT {}",
        // serde_json::to_string(&app).unwrap()
        app
    );
    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    assert_debug_snapshot!(sql);
}
