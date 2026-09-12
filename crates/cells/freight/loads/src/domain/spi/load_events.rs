use crate::domain::events::LoadEvent;
#[cfg(test)]
use std::sync::{Arc, Mutex};

pub(crate) trait LoadEvents {
    fn publish(&self, events: Vec<LoadEvent>) -> impl Future<Output = ()> + Send;
}

#[cfg(test)]
#[derive(Clone)]
pub(crate) struct FakeLoadEvents {
    published: Arc<Mutex<Vec<LoadEvent>>>,
}

#[cfg(test)]
impl FakeLoadEvents {
    pub(crate) fn new() -> Self {
        Self {
            published: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub(crate) fn published(&self) -> Vec<LoadEvent> {
        self.published
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }
}

#[cfg(test)]
impl LoadEvents for FakeLoadEvents {
    fn publish(&self, events: Vec<LoadEvent>) -> impl Future<Output = ()> + Send {
        let published = self.published.clone();
        async move {
            published
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .extend(events);
        }
    }
}
