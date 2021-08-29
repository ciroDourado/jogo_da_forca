use serde::Deserialize;

pub fn deserializar<'a, T>(json: &'a str) -> T
    where T: Deserialize<'a>
{
    serde_json::from_str(json).unwrap()
} // deserializar
