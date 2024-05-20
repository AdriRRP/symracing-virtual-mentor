use crate::file::domain::event::created::Created;
use crate::file::domain::file::File;
use crate::file::domain::repository::Repository;
use crate::shared::domain::event::bus::Bus as EventBus;

use std::sync::Arc;

pub struct Creator<R: Repository> {
    repository: Arc<R>,
    event_bus: Arc<dyn EventBus>,
}

impl<R: Repository> Creator<R> {
    pub fn new(repository: Arc<R>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `self.repository` fail creating
    pub async fn create(&self, file: File) {
        let event = Arc::new(Created::new(&file));
        self.repository.create(file).await;
        let _ = self.event_bus.dispatch(event).await;
        // TODO: Log error dispatching event
    }
}