use crate::shared::domain::event::Event;
use std::any::Any;

use std::fmt::Debug;

#[derive(Debug)]
pub struct Deleted {
    pub id: String,
}

impl Deleted {
    #[must_use]
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl Event for Deleted {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
