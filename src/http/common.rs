use crate::error::HttpSerializeError;

pub trait HttpReqRes {
    fn set_header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>;

    fn serialize(buff: &Vec<u8>) -> Result<Self, HttpSerializeError>
    where
        Self: Sized;

    fn deserialize(&self) -> Vec<u8>;
}
