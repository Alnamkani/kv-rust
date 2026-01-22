use super::interface::Storage;
use crate::app::models::CreateKVRequest;
use crate::types::Key;

pub fn test_get_nonexistent_key<S: Storage>(storage: &S) {
    let key = Key::new("nonexistent".to_string()).unwrap();
    let result = storage.get(key);
    assert!(result.is_err(), "Getting nonexistent key should return Err");
}

pub fn test_upsert_new_key<S: Storage>(storage: &S) {
    let request = CreateKVRequest {
        key: Key::new("test-key".to_string()).unwrap(),
        value: "test-value".to_string(),
    };

    let response = storage.upsert(request);

    assert_eq!(response.key.as_str(), "test-key");
    assert_eq!(response.value, "test-value");
    assert_eq!(response.metadata.created_at, response.metadata.updated_at);
}

pub fn test_upsert_existing_key_preserves_created_at<S: Storage>(storage: &S) {
    let key = Key::new("test-key".to_string()).unwrap();

    let first_request = CreateKVRequest {
        key: key.clone(),
        value: "first-value".to_string(),
    };
    let first_response = storage.upsert(first_request);
    let original_created_at = first_response.metadata.created_at;

    std::thread::sleep(std::time::Duration::from_millis(10));

    let second_request = CreateKVRequest {
        key: key.clone(),
        value: "second-value".to_string(),
    };
    let second_response = storage.upsert(second_request);

    assert_eq!(second_response.value, "second-value");
    assert_eq!(
        second_response.metadata.created_at, original_created_at,
        "created_at should be preserved on update"
    );
    assert!(
        second_response.metadata.updated_at > second_response.metadata.created_at,
        "updated_at should be newer than created_at after update"
    );
}

pub fn test_get_existing_key<S: Storage>(storage: &S) {
    let request = CreateKVRequest {
        key: Key::new("test-key".to_string()).unwrap(),
        value: "test-value".to_string(),
    };
    storage.upsert(request);

    let key = Key::new("test-key".to_string()).unwrap();
    let result = storage.get(key);

    assert!(result.is_ok());
    let value_response = result.unwrap();
    assert_eq!(value_response.value, "test-value");
}

pub fn test_delete_existing_key<S: Storage>(storage: &S) {
    let request = CreateKVRequest {
        key: Key::new("test-key".to_string()).unwrap(),
        value: "test-value".to_string(),
    };
    storage.upsert(request);

    let key = Key::new("test-key".to_string()).unwrap();
    let deleted = storage.delete(key.clone());

    assert!(deleted.is_ok());
    let deleted_value = deleted.unwrap();
    assert_eq!(deleted_value.value, "test-value");

    let get_result = storage.get(key);
    assert!(get_result.is_err(), "Key should not exist after deletion");
}

pub fn test_delete_nonexistent_key<S: Storage>(storage: &S) {
    let key = Key::new("nonexistent".to_string()).unwrap();
    let result = storage.delete(key);
    assert!(
        result.is_err(),
        "Deleting nonexistent key should return Err"
    );
}

pub fn test_list_keys_empty<S: Storage>(storage: &S) {
    let keys = storage.list_keys();
    assert_eq!(keys.len(), 0, "Empty storage should have 0 keys");
}

pub fn test_list_keys_multiple<S: Storage>(storage: &S) {
    storage.upsert(CreateKVRequest {
        key: Key::new("key1".to_string()).unwrap(),
        value: "value1".to_string(),
    });
    storage.upsert(CreateKVRequest {
        key: Key::new("key2".to_string()).unwrap(),
        value: "value2".to_string(),
    });
    storage.upsert(CreateKVRequest {
        key: Key::new("key3".to_string()).unwrap(),
        value: "value3".to_string(),
    });

    let keys = storage.list_keys();
    assert_eq!(keys.len(), 3, "Should have 3 keys");

    let key_strs: Vec<String> = keys.iter().map(|k| k.as_str().to_string()).collect();
    assert!(key_strs.contains(&"key1".to_string()));
    assert!(key_strs.contains(&"key2".to_string()));
    assert!(key_strs.contains(&"key3".to_string()));
}

pub fn test_concurrent_upserts<S: Storage + Sync + Send + 'static>(storage: S) {
    use std::sync::Arc;
    use std::thread;

    let storage_arc = Arc::new(storage);
    let mut handles = vec![];

    for i in 0..10 {
        let storage_clone = Arc::clone(&storage_arc);
        let handle = thread::spawn(move || {
            let request = CreateKVRequest {
                key: Key::new(format!("key-{}", i)).unwrap(),
                value: format!("value-{}", i),
            };
            storage_clone.upsert(request);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let keys = storage_arc.list_keys();
    assert_eq!(keys.len(), 10, "All concurrent upserts should succeed");
}
