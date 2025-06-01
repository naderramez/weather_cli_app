use serde::{self, Serialize, de::DeserializeOwned};
use serde_json;

pub fn parse_value_from_json<T>(json_text: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    serde_json::from_str::<T>(json_text)
}

pub fn serialize_data<T>(data: &T) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    serde_json::to_string(data)
}
