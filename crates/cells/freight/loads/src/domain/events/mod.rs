#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LoadEvent {
    Created { load_id: String },
}
