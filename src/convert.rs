use crate::*;
#[cfg(feature = "serde_json")]
pub fn serialize<T: Serialize>(i: T) -> serde_json::Value {
    serde_json::to_value(i).unwrap()
}
#[cfg(feature = "serde_json")]
pub fn deserialize<T: for<'a> Deserialize<'a>>(i: serde_json::Value) -> T {
    dbg!(&i);
    serde_json::from_value::<T>(i.clone()).unwrap_or_else(|e| {
        panic!(
            "Failed to convert to JSON, results: {:?}, detail: {:?}",
            i, e
        )
    })
}

#[cfg(feature = "bincode")]
pub fn serialize<T: Serialize>(i: T) -> Vec<u8> {
    bincode::serialize(&i).unwrap()
}
#[cfg(feature = "bincode")]
pub fn deserialize<T: for<'a> Deserialize<'a>>(i: Vec<u8>) -> T {
    bincode::deserialize::<T>(&i).unwrap_or_else(|e| {
        panic!(
            "Failed to convert to JSON, results: {:?}, detail: {:?}",
            i, e
        )
    })
}

pub trait CompressionWith {
    fn compression_with(value: Value) -> Self;
}

impl<T: CompressionWith> CompressionWith for Vec<T> {
    fn compression_with(value: Value) -> Self {
        dbg!("Vec:");
        dbg!(&value);
        let vs = Vec::<Value>::try_from(value).unwrap();
        vs.into_iter().map(|e| T::compression_with(e)).collect()
    }
}
