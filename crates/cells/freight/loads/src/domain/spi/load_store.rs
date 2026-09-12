use crate::domain::entities::load::Load;
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::sync::Mutex;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum LoadStoreError {
    Conflict,
    Unexpected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IdempotencyRecord {
    pub(crate) actor_id: String,
    pub(crate) key: String,
    pub(crate) fingerprint: String,
    pub(crate) outcome: String,
}

pub(crate) trait LoadStore {
    fn get_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Load>, LoadStoreError>> + Send;

    fn save(
        &self,
        load: &Load,
        idempotency: Option<IdempotencyRecord>,
    ) -> impl Future<Output = Result<(), LoadStoreError>> + Send;
}

#[cfg(any(test, feature = "contract"))]
pub(crate) async fn load_store_contract<S: LoadStore>(
    store: &S,
    missing_id: &str,
    load: &Load,
    keyed: &Load,
    record: IdempotencyRecord,
) {
    assert_eq!(store.get_by_id(missing_id).await.unwrap(), None);
    store.save(load, None).await.unwrap();
    assert_eq!(
        store.get_by_id(&load.id).await.unwrap().as_ref(),
        Some(load)
    );
    store.save(keyed, Some(record)).await.unwrap();
    assert_eq!(
        store.get_by_id(&keyed.id).await.unwrap().as_ref(),
        Some(keyed)
    );
}

#[cfg(test)]
pub(crate) struct FakeLoadStore {
    loads: Mutex<HashMap<String, Load>>,
    keys: Mutex<HashMap<(String, String), IdempotencyRecord>>,
}

#[cfg(test)]
impl FakeLoadStore {
    pub(crate) fn new() -> Self {
        Self {
            loads: Mutex::new(HashMap::new()),
            keys: Mutex::new(HashMap::new()),
        }
    }

    fn insert(
        &self,
        load: &Load,
        idempotency: Option<IdempotencyRecord>,
    ) -> Result<(), LoadStoreError> {
        let mut loads = self
            .loads
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if loads.contains_key(&load.id) {
            return Err(LoadStoreError::Conflict);
        }
        loads.insert(load.id.clone(), load.clone());
        drop(loads);
        if let Some(record) = idempotency {
            let mut keys = self
                .keys
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let key = (record.actor_id.clone(), record.key.clone());
            if keys.contains_key(&key) {
                return Err(LoadStoreError::Conflict);
            }
            keys.insert(key, record);
        }
        Ok(())
    }
}

#[cfg(test)]
impl LoadStore for FakeLoadStore {
    fn get_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Load>, LoadStoreError>> + Send {
        let found = self
            .loads
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(id)
            .cloned();
        async move { Ok(found) }
    }

    fn save(
        &self,
        load: &Load,
        idempotency: Option<IdempotencyRecord>,
    ) -> impl Future<Output = Result<(), LoadStoreError>> + Send {
        let result = self.insert(load, idempotency);
        async move { result }
    }
}

#[cfg(test)]
mod tests {
    use super::{FakeLoadStore, IdempotencyRecord, load_store_contract};
    use crate::domain::entities::load::LoadBuilder;

    #[tokio::test]
    async fn fake_satisfies_load_store_contract() {
        let store = FakeLoadStore::new();
        let load = LoadBuilder::new().build_new();
        let keyed = LoadBuilder::new()
            .id("01900000-0000-7000-8000-000000000011")
            .pickup_id("01900000-0000-7000-8000-000000000012")
            .delivery_id("01900000-0000-7000-8000-000000000013")
            .build_new();
        load_store_contract(
            &store,
            "01900000-0000-7000-8000-000000000099",
            &load,
            &keyed,
            IdempotencyRecord {
                actor_id: keyed.actor_id.clone(),
                key: "post-loads-1".to_owned(),
                fingerprint: "fp-1".to_owned(),
                outcome: "ok".to_owned(),
            },
        )
        .await;
    }
}
