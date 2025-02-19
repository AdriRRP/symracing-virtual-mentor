/// Module for lap headers.
pub mod header;

/// Module for lap headers (plural form).
pub mod headers;

/// Module for lap variables.
pub mod variables;

use crate::lap::domain::lap::header::Header;
use crate::lap::domain::lap::variables::Variables;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a lap with associated header and variables.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Lap {
    /// Header information for the lap.
    pub header: Header,

    /// Variables data for the lap.
    pub variables: Variables,
}

impl Lap {
    /// Constructs a new Lap instance.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the lap.
    /// * `file_id` - Identifier of the file associated with the lap.
    /// * `number` - Lap number.
    /// * `driver` - Name of the driver.
    /// * `category` - Category of the race (e.g., GT, Formula 1).
    /// * `car` - Name or model of the car.
    /// * `circuit` - Name of the circuit where the lap was performed.
    /// * `date` - Date and time when the lap was performed (in UTC).
    /// * `variables` - Variables data associated with the lap.
    ///
    /// # Returns
    ///
    /// A new `Lap` instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        id: Uuid,
        file_id: String,
        number: u16,
        driver: String,
        category: String,
        car: String,
        circuit: String,
        date: DateTime<Utc>,
        variables: Variables,
    ) -> Self {
        let time = *variables.lap_current_lap_time.last().unwrap_or(&0f32);
        let header = Header::new(
            id, file_id, number, driver, category, car, circuit, date, time,
        );

        Self { header, variables }
    }
}
