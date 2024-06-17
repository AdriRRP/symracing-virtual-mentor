use crate::analysis::domain::analyses::Analyses;
use crate::analysis::domain::repository::Repository;

use std::sync::Arc;

/// A struct responsible for finding data asynchronously.
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    /// Creates a new `Finder` instance.
    ///
    /// # Parameters
    ///
    /// - `repository`: An asynchronous repository for finding operations.
    ///
    /// # Returns
    ///
    /// A new `Finder` instance.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Asynchronously finds data based on the provided criteria using the repository.
    ///
    /// # Parameters
    ///
    /// - `criteria`: The criteria used for finding the data.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the underlying repository fails during the find operation.
    pub async fn find(&self, criteria: &str) -> Result<Option<Analyses>, String> {
        self.repository.find_by_criteria(criteria).await
        // Send domain events
    }
}
