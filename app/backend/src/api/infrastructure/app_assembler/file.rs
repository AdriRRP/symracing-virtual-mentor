use shared::common::infrastructure::event::tokio_bus::TokioBus;
use shared::file::application::create::service::Creator as FileCreator;
use shared::file::application::delete::service::Deleter as FileDeleter;
use shared::file::application::find::by_criteria::service::Finder as FileByCriteriaFinder;
use shared::file::application::find::by_id::service::Finder as FileByIdFinder;
use shared::file::application::validate::service::Validator as FileValidator;
use shared::file::infrastructure::repository::in_memory::InMemory as InMemoryFileRepository;

use std::sync::Arc;

pub struct Assembler {
    pub creator: Arc<FileCreator<InMemoryFileRepository, TokioBus>>,
    pub deleter: Arc<FileDeleter<InMemoryFileRepository, TokioBus>>,
    pub by_id_finder: Arc<FileByIdFinder<InMemoryFileRepository>>,
    pub by_criteria_finder: Arc<FileByCriteriaFinder<InMemoryFileRepository>>,
    pub validator: Arc<FileValidator<InMemoryFileRepository, TokioBus>>,
}

impl Assembler {
    #[must_use]
    pub fn new(event_bus: &Arc<TokioBus>) -> Self {
        let repository = Arc::new(InMemoryFileRepository::default());
        let creator = Arc::new(FileCreator::new(
            Arc::clone(&repository),
            Arc::clone(event_bus),
        ));
        let deleter = Arc::new(FileDeleter::new(
            Arc::clone(&repository),
            Arc::clone(event_bus),
        ));
        let by_id_finder = Arc::new(FileByIdFinder::new(Arc::clone(&repository)));
        let by_criteria_finder = Arc::new(FileByCriteriaFinder::new(Arc::clone(&repository)));
        let validator = Arc::new(FileValidator::new(
            Arc::clone(&repository),
            Arc::clone(event_bus),
        ));

        Self {
            creator,
            deleter,
            by_id_finder,
            by_criteria_finder,
            validator,
        }
    }
}
