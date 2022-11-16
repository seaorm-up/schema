use crate::*;

#[tokio::test]
async fn test_raw_db_relate() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
    let db = DBX.get().unwrap();
    let create_identity = format!("relate mobile:123->identity->account:hz");
    let v = db.raw_execute_one(create_identity).await.unwrap();
    // dbg!(&v);
    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    assert_debug_snapshot!(v);
}

#[test]
fn test_obj_serde() {
    let i = instance();
    assert_debug_snapshot!((
        &serialize(&i),
        deserialize::<App>(serialize(&i)),
        Into::<Value>::into(i)
    ));
}

/// in [bincode] serialize [App] is different with [Value]
/// from https://www.107000.com/T-Ascii
/// [App] is "app:i2esfym6g2fh06svw449adf"
/// [Value] is "idapp:ec4bmojb7l7exs6tqiq1nameadf"
///
///  Notice use default serde would get
/// ```
/// [src/db_ex.rs:106] &serialize(&i) = Object {
///     "id": String("app:i2esfym6g2fh06svw449"),
///     "name": String("adf"),
/// }
/// [src/db_ex.rs:112] serialize(&ret) = Array [
///     Object {
///         "Object": Object {
///             "id": Object {
///                 "Thing": String("app:sik8l26fjisr175wog63"),
///             },
///             "name": Object {
///                 "Strand": String("adf"),
///             },
///         },
///     },
/// ]
/// ```
#[tokio::test]
async fn compare_obj_and_dbvalue() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
    let i = instance();
    let ret = "create app CONTENT { name: 'adf' }"
        .to_owned()
        .raw_execute_one()
        .await
        .unwrap();
    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    assert_debug_snapshot!([serialize(i), serialize(&ret)]);
}

#[tokio::test]
async fn bin_compare_obj_and_dbvalue() {
    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
    let i = instance();
    assert_debug_snapshot!(&serialize(serde_json::to_value(&i).unwrap()));
    let ret = "create app CONTENT { name: 'adf' }"
        .to_owned()
        .raw_execute_one()
        .await
        .unwrap();
    assert_debug_snapshot!(serialize(serde_json::to_value(&ret).unwrap()));
}

#[tokio::test]
async fn test_value_serde_to_obj() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();

    let ret = "create app CONTENT { name: 'adf' }"
        .to_owned()
        .raw_execute_one()
        .await
        .unwrap();

    let reset_bin = &serialize(&ret);
    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    assert_debug_snapshot!(deserialize::<Vec<App>>(reset_bin.clone()));
}

#[tokio::test]
async fn test_vec_to_value() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
    let i = vec![instance()];
    set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
    assert_debug_snapshot!(IntoValue::into(i));
}
