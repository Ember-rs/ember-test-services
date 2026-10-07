/// Example scheduled task used by the Petstore test service.

use scafra::prelude::*;

/// Emits a heartbeat so the scheduler can be verified while the service runs.
#[logger]
pub fn pet_maintenance() {
    info!(task = "pet-maintenance", "scheduled pet maintenance ran");

}

scafra::register_scheduled_task!("pet-maintenance", 30_000, pet_maintenance);
