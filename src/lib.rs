use ember_bootui as _;

mod controllers {
    mod pet_controller {
        include!("main/controllers/pet_controller.rs");
    }
}

mod jobs {
    mod pet_maintenance {
        include!("main/jobs/pet_maintenance.rs");
    }
}

mod models {
    pub mod pet {
        include!("main/models/pet.rs");
    }
}

mod services {
    pub mod pet_service {
        include!("main/services/pet_service.rs");
    }
}
