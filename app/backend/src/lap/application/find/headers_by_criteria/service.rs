use crate::lap::domain::lap::headers::Headers;
use crate::lap::domain::repository::Repository;

use std::sync::Arc;

pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// # Errors
    ///
    /// Will return `Err` if `self.repository` fail finding by criteria
    pub async fn find(&self, criteria: &str) -> Result<Option<Headers>, String> {
        self.repository.find_header_by_criteria(criteria).await
        // Send domain events
    }
}
