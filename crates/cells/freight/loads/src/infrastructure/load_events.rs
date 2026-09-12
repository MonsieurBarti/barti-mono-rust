use crate::domain::events::LoadEvent;
use crate::domain::spi::load_events::LoadEvents;

#[derive(Clone, Copy)]
pub(crate) struct InCellLoadEvents;

impl LoadEvents for InCellLoadEvents {
    async fn publish(&self, _events: Vec<LoadEvent>) {}
}
