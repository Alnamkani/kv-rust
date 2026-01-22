use crate::app::models::{CreateKVRequest, KeyValueResponse, ValueResponse};
use crate::types::Key;

pub trait Storage {
    fn get(&self, key: Key) -> Option<ValueResponse>;
    fn upsert(&self, body: CreateKVRequest) -> KeyValueResponse;
    fn delete(&self, key: Key) -> Option<ValueResponse>;
    fn contains_key(&self, key: Key) -> bool;
    fn list_keys(&self) -> Vec<Key>;
}
