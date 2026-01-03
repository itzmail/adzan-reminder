use serde::de::Deserializer;
use serde::Deserialize;

/// Deserialize string yang bisa null menjadi String kosong
pub fn string_or_null<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// Deserialize string yang bisa null menjadi Option<String>
pub fn option_string_or_null<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)
}
