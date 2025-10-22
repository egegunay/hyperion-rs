use crate::state::State;
use std::{convert::Infallible, sync::Arc};

// AI helped with the exceptions, I wanna understand deadlocks and poisoning later on

pub async fn insert(key: String, state: Arc<State>) -> Result<String, Infallible> {
    let mut guard = state.data.write().expect("RwLock poisoned");

    let value = guard.entry(key).or_insert(0);

    *value += 1;

    Ok(value.to_string())
}

pub async fn read(key: String, state: Arc<State>) -> Result<String, Infallible> {
    let guard = state.data.read().expect("RwLock poisoned");

    if let Some(value) = guard.get(&key) {
        Ok(value.to_string())
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}

pub async fn update(key: String, new_value: u64, state: Arc<State>) -> Result<String, Infallible> {
    let mut guard = state.data.write().expect("RwLock poisoned");

    if let Some(value) = guard.get_mut(&key) {
        *value = new_value;
        Ok(format!("Key '{}' is now value {}", key, new_value))
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}

pub async fn delete(key: String, state: Arc<State>) -> Result<String, Infallible> {
    let mut guard = state.data.write().expect("RwLock poisoned");

    if let Some(value) = guard.remove(&key) {
        Ok(format!("Key '{}' with value {} is deleted", key, value))
    } else {
        Ok(format!("Key '{}' not found", key))
    }
}

// Proudly vibe-coded with Cursor
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_state() -> Arc<State> {
        Arc::new(State::default())
    }

    fn create_state_with_data() -> Arc<State> {
        let state = Arc::new(State::default());
        {
            let mut guard = state.data.write().unwrap();
            guard.insert("existing_key".to_string(), 42);
            guard.insert("another_key".to_string(), 100);
        }
        state
    }

    #[tokio::test]
    async fn test_insert_new_key() {
        let state = create_test_state();
        let key = "new_key".to_string();
        
        let result = insert(key.clone(), state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1");
        
        // Verify the key was actually inserted
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), Some(&1));
    }

    #[tokio::test]
    async fn test_insert_existing_key() {
        let state = create_state_with_data();
        let key = "existing_key".to_string();
        
        let result = insert(key.clone(), state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "43"); // 42 + 1
        
        // Verify the value was incremented
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), Some(&43));
    }

    #[tokio::test]
    async fn test_insert_multiple_times() {
        let state = create_test_state();
        let key = "counter".to_string();
        
        // Insert multiple times
        let result1 = insert(key.clone(), state.clone()).await.unwrap();
        let result2 = insert(key.clone(), state.clone()).await.unwrap();
        let result3 = insert(key.clone(), state.clone()).await.unwrap();
        
        assert_eq!(result1, "1");
        assert_eq!(result2, "2");
        assert_eq!(result3, "3");
        
        // Verify final state
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), Some(&3));
    }

    #[tokio::test]
    async fn test_read_existing_key() {
        let state = create_state_with_data();
        let key = "existing_key".to_string();
        
        let result = read(key, state).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[tokio::test]
    async fn test_read_non_existing_key() {
        let state = create_state_with_data();
        let key = "non_existing_key".to_string();
        
        let result = read(key.clone(), state).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' not found", key));
    }

    #[tokio::test]
    async fn test_read_empty_state() {
        let state = create_test_state();
        let key = "any_key".to_string();
        
        let result = read(key.clone(), state).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' not found", key));
    }

    #[tokio::test]
    async fn test_update_existing_key() {
        let state = create_state_with_data();
        let key = "existing_key".to_string();
        let new_value = 999;
        
        let result = update(key.clone(), new_value, state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' is now value {}", key, new_value));
        
        // Verify the value was actually updated
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), Some(&new_value));
    }

    #[tokio::test]
    async fn test_update_non_existing_key() {
        let state = create_state_with_data();
        let key = "non_existing_key".to_string();
        let new_value = 123;
        
        let result = update(key.clone(), new_value, state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' not found", key));
        
        // Verify the key was not added
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), None);
    }

    #[tokio::test]
    async fn test_update_empty_state() {
        let state = create_test_state();
        let key = "any_key".to_string();
        let new_value = 456;
        
        let result = update(key.clone(), new_value, state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' not found", key));
        
        // Verify the key was not added
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), None);
    }

    #[tokio::test]
    async fn test_update_zero_value() {
        let state = create_state_with_data();
        let key = "existing_key".to_string();
        let new_value = 0;
        
        let result = update(key.clone(), new_value, state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' is now value {}", key, new_value));
        
        // Verify the value was set to 0
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), Some(&0));
    }

    #[tokio::test]
    async fn test_delete_existing_key() {
        let state = create_state_with_data();
        let key = "existing_key".to_string();
        
        let result = delete(key.clone(), state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' with value 42 is deleted", key));
        
        // Verify the key was actually removed
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), None);
    }

    #[tokio::test]
    async fn test_delete_non_existing_key() {
        let state = create_state_with_data();
        let key = "non_existing_key".to_string();
        
        let result = delete(key.clone(), state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' not found", key));
        
        // Verify other keys are still there
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get("existing_key"), Some(&42));
        assert_eq!(guard.get("another_key"), Some(&100));
    }

    #[tokio::test]
    async fn test_delete_empty_state() {
        let state = create_test_state();
        let key = "any_key".to_string();
        
        let result = delete(key.clone(), state.clone()).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Key '{}' not found", key));
        
        // Verify state is still empty
        let guard = state.data.read().unwrap();
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn test_delete_all_keys() {
        let state = create_state_with_data();
        
        // Delete first key
        let result1 = delete("existing_key".to_string(), state.clone()).await.unwrap();
        assert_eq!(result1, "Key 'existing_key' with value 42 is deleted");
        
        // Delete second key
        let result2 = delete("another_key".to_string(), state.clone()).await.unwrap();
        assert_eq!(result2, "Key 'another_key' with value 100 is deleted");
        
        // Verify state is empty
        let guard = state.data.read().unwrap();
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let state = create_test_state();
        let key = "concurrent_key".to_string();
        
        // Spawn multiple tasks that insert the same key
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let state = state.clone();
                let key = key.clone();
                tokio::spawn(async move {
                    insert(key, state).await
                })
            })
            .collect();
        
        // Wait for all tasks to complete
        let results: Vec<_> = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|result| result.unwrap().unwrap())
            .collect();
        
        // All results should be valid
        for result in &results {
            assert!(result.parse::<u64>().is_ok());
        }
        
        // The final value should be 10 (since we ran 10 inserts)
        let guard = state.data.read().unwrap();
        assert_eq!(guard.get(&key), Some(&10));
    }

    #[tokio::test]
    async fn test_full_crud_workflow() {
        let state = create_test_state();
        let key = "workflow_key".to_string();
        
        // Create (insert)
        let insert_result = insert(key.clone(), state.clone()).await.unwrap();
        assert_eq!(insert_result, "1");
        
        // Read
        let read_result = read(key.clone(), state.clone()).await.unwrap();
        assert_eq!(read_result, "1");
        
        // Update
        let update_result = update(key.clone(), 50, state.clone()).await.unwrap();
        assert_eq!(update_result, "Key 'workflow_key' is now value 50");
        
        // Read again to verify update
        let read_result2 = read(key.clone(), state.clone()).await.unwrap();
        assert_eq!(read_result2, "50");
        
        // Delete
        let delete_result = delete(key.clone(), state.clone()).await.unwrap();
        assert_eq!(delete_result, "Key 'workflow_key' with value 50 is deleted");
        
        // Read after delete should return not found
        let read_result3 = read(key.clone(), state.clone()).await.unwrap();
        assert_eq!(read_result3, "Key 'workflow_key' not found");
    }
}
