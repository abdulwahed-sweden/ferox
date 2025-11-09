use crate::core::module::{ModuleInfo, ModuleResult};
use std::collections::VecDeque;
use uuid::Uuid;

/// Stored result entry combining module info and result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredResult {
    pub id: Uuid,
    pub module_info: ModuleInfo,
    pub result: ModuleResult,
}

/// In-memory storage for module execution results
pub struct ResultStore {
    results: VecDeque<StoredResult>,
    max_results: usize,
}

impl ResultStore {
    /// Create a new result store with specified max capacity
    pub fn new(max_results: usize) -> Self {
        Self {
            results: VecDeque::with_capacity(max_results),
            max_results,
        }
    }

    /// Add a result to the store
    /// If the store is full, the oldest result is removed
    pub fn add(&mut self, module_info: ModuleInfo, result: ModuleResult) -> Uuid {
        let id = Uuid::new_v4();
        let stored = StoredResult {
            id,
            module_info,
            result,
        };

        // Remove oldest if at capacity
        if self.results.len() >= self.max_results {
            self.results.pop_front();
        }

        self.results.push_back(stored);
        id
    }

    /// Get a result by ID
    pub fn get(&self, id: &Uuid) -> Option<&StoredResult> {
        self.results.iter().find(|r| &r.id == id)
    }

    /// Get all results
    pub fn get_all(&self) -> Vec<&StoredResult> {
        self.results.iter().collect()
    }

    /// Get the last N results
    pub fn get_last(&self, count: usize) -> Vec<&StoredResult> {
        self.results
            .iter()
            .rev()
            .take(count)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    /// Get successful results only
    pub fn get_successful(&self) -> Vec<&StoredResult> {
        self.results
            .iter()
            .filter(|r| r.result.success)
            .collect()
    }

    /// Get failed results only
    pub fn get_failed(&self) -> Vec<&StoredResult> {
        self.results
            .iter()
            .filter(|r| !r.result.success)
            .collect()
    }

    /// Get results by module type
    pub fn get_by_module(&self, module_path: &str) -> Vec<&StoredResult> {
        self.results
            .iter()
            .filter(|r| {
                format!("{}/{}", r.module_info.category, r.module_info.name) == module_path
            })
            .collect()
    }

    /// Get total count of stored results
    pub fn count(&self) -> usize {
        self.results.len()
    }

    /// Clear all results
    pub fn clear(&mut self) {
        self.results.clear();
    }

    /// Get results within a time range
    pub fn get_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Vec<&StoredResult> {
        self.results
            .iter()
            .filter(|r| r.result.timestamp >= start && r.result.timestamp <= end)
            .collect()
    }
}

impl Default for ResultStore {
    fn default() -> Self {
        Self::new(100) // Default to storing last 100 results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::module::{ModuleInfo, ModuleResult, ModuleType};

    fn create_test_module_info() -> ModuleInfo {
        ModuleInfo {
            name: "test".to_string(),
            version: "1.0".to_string(),
            author: "Test".to_string(),
            description: "Test module".to_string(),
            module_type: ModuleType::Scanner,
            category: "test".to_string(),
        }
    }

    #[test]
    fn test_add_and_get() {
        let mut store = ResultStore::new(10);
        let info = create_test_module_info();
        let result = ModuleResult::success("Test");

        let id = store.add(info.clone(), result);
        assert_eq!(store.count(), 1);

        let retrieved = store.get(&id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().module_info.name, "test");
    }

    #[test]
    fn test_capacity_limit() {
        let mut store = ResultStore::new(3);
        let info = create_test_module_info();

        for i in 0..5 {
            let result = ModuleResult::success(format!("Test {}", i));
            store.add(info.clone(), result);
        }

        assert_eq!(store.count(), 3); // Should only keep last 3
    }

    #[test]
    fn test_get_last() {
        let mut store = ResultStore::new(10);
        let info = create_test_module_info();

        for i in 0..5 {
            let result = ModuleResult::success(format!("Test {}", i));
            store.add(info.clone(), result);
        }

        let last_two = store.get_last(2);
        assert_eq!(last_two.len(), 2);
        assert!(last_two[1].result.message.contains("Test 4"));
    }

    #[test]
    fn test_filter_successful() {
        let mut store = ResultStore::new(10);
        let info = create_test_module_info();

        store.add(info.clone(), ModuleResult::success("Good"));
        store.add(info.clone(), ModuleResult::error("Bad"));
        store.add(info.clone(), ModuleResult::success("Good"));

        let successful = store.get_successful();
        assert_eq!(successful.len(), 2);
    }
}
