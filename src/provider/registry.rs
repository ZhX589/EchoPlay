use std::collections::HashMap;
use std::sync::Arc;

use crate::error::EchoError;
use crate::provider::Provider;

/// Registry of all loaded music source providers.
///
/// Providers register themselves here. The [`Library`](crate::library::Library)
/// uses this registry to dispatch operations to the correct provider.
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider. Fails if a provider with the same identifier exists.
    pub fn register(&mut self, provider: Arc<dyn Provider>) -> Result<(), EchoError> {
        let id = provider.identifier().to_string();
        if self.providers.contains_key(&id) {
            return Err(EchoError::ProviderAlreadyRegistered(id));
        }
        tracing::info!("registered provider: {} ({})", provider.name(), id);
        self.providers.insert(id, provider);
        Ok(())
    }

    /// Get a provider by identifier.
    pub fn get(&self, identifier: &str) -> Option<&Arc<dyn Provider>> {
        self.providers.get(identifier)
    }

    /// List all registered providers.
    pub fn list(&self) -> Vec<&Arc<dyn Provider>> {
        self.providers.values().collect()
    }

    /// Number of registered providers.
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::any::Any;

    struct DummyProvider {
        id: String,
    }

    #[async_trait]
    impl Provider for DummyProvider {
        fn identifier(&self) -> &str {
            &self.id
        }
        fn name(&self) -> &str {
            "Dummy"
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_register_and_get() {
        let mut reg = ProviderRegistry::new();
        let p = Arc::new(DummyProvider { id: "test".into() });
        reg.register(p).unwrap();
        assert!(reg.get("test").is_some());
        assert!(reg.get("missing").is_none());
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_duplicate_register() {
        let mut reg = ProviderRegistry::new();
        let p1 = Arc::new(DummyProvider { id: "dup".into() });
        let p2 = Arc::new(DummyProvider { id: "dup".into() });
        reg.register(p1).unwrap();
        assert!(reg.register(p2).is_err());
    }
}
