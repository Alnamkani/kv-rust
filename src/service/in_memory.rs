use crate::app::models::{CreateKVRequest, KeyValueResponse, Metadata, ValueResponse};
use crate::service::{Storage, StorageError};
use crate::types::Key;
use chrono::Utc;

pub struct InMemoryStorage {
    map: dashmap::DashMap<Key, ValueResponse>,
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            map: dashmap::DashMap::new(),
        }
    }
}

impl Storage for InMemoryStorage {
    fn get(&self, key: Key) -> Result<ValueResponse, StorageError> {
        self.map
            .get(&key)
            .map(|entry| entry.value().clone())
            .ok_or(StorageError::KeyNotFound(key))
    }

    fn insert(&self, body: CreateKVRequest) -> Result<KeyValueResponse, StorageError> {
        if self.map.contains_key(&body.key) {
            return Err(StorageError::KeyAlreadyExists(body.key));
        }

        let now = Utc::now();
        let value_response = ValueResponse {
            value: body.value.clone(),
            metadata: Metadata {
                created_at: now,
                updated_at: now,
            },
        };

        self.map.insert(body.key.clone(), value_response.clone());

        Ok(KeyValueResponse {
            key: body.key,
            value: body.value,
            metadata: value_response.metadata,
        })
    }

    fn upsert(&self, body: CreateKVRequest) -> KeyValueResponse {
        let now = Utc::now();

        let value_response = self
            .map
            .entry(body.key.clone())
            .and_modify(|existing| {
                existing.value = body.value.clone();
                existing.metadata.updated_at = now;
            })
            .or_insert_with(|| ValueResponse {
                value: body.value.clone(),
                metadata: Metadata {
                    created_at: now,
                    updated_at: now,
                },
            });

        KeyValueResponse {
            key: body.key,
            value: body.value,
            metadata: value_response.value().metadata.clone(),
        }
    }

    fn delete(&self, key: Key) -> Result<ValueResponse, StorageError> {
        self.map
            .remove(&key)
            .map(|(_, value)| value)
            .ok_or(StorageError::KeyNotFound(key))
    }

    fn list_keys(&self) -> Vec<Key> {
        self.map.iter().map(|entry| entry.key().clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::tests::*;

    fn create_storage() -> InMemoryStorage {
        InMemoryStorage::new()
    }

    #[test]
    fn test_in_memory_get_nonexistent() {
        let storage = create_storage();
        test_get_nonexistent_key(&storage);
    }

    #[test]
    fn test_in_memory_upsert_new() {
        let storage = create_storage();
        test_upsert_new_key(&storage);
    }

    #[test]
    fn test_in_memory_upsert_preserves_created_at() {
        let storage = create_storage();
        test_upsert_existing_key_preserves_created_at(&storage);
    }

    #[test]
    fn test_in_memory_get_existing() {
        let storage = create_storage();
        test_get_existing_key(&storage);
    }

    #[test]
    fn test_in_memory_delete_existing() {
        let storage = create_storage();
        test_delete_existing_key(&storage);
    }

    #[test]
    fn test_in_memory_delete_nonexistent() {
        let storage = create_storage();
        test_delete_nonexistent_key(&storage);
    }

    #[test]
    fn test_in_memory_list_keys_empty() {
        let storage = create_storage();
        test_list_keys_empty(&storage);
    }

    #[test]
    fn test_in_memory_list_keys_multiple() {
        let storage = create_storage();
        test_list_keys_multiple(&storage);
    }

    #[test]
    fn test_in_memory_concurrent_access() {
        let storage = create_storage();
        test_concurrent_upserts(storage);
    }
}
