use crate::lap::domain::lap::Lap;
use crate::lap::domain::laps::Laps;

use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, laps: Laps);
    async fn delete(&self, id: &Uuid) -> Result<(), String>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Lap>, String>;
    async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Laps>, String>;
}