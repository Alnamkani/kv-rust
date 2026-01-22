use crate::app::models::{CreateKVRequest, KeyValueResponse, ValueResponse};
use crate::service::StorageError;
use crate::types::Key;

pub trait Storage {
    fn get(&self, key: Key) -> Result<ValueResponse, StorageError>;
    fn insert(&self, body: CreateKVRequest) -> Result<KeyValueResponse, StorageError>;
    fn upsert(&self, body: CreateKVRequest) -> KeyValueResponse;
    fn delete(&self, key: Key) -> Result<ValueResponse, StorageError>;
    fn list_keys(&self) -> Vec<Key>;
}
