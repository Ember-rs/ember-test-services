use scafra_bootui as _;

mod controllers {
    pub(crate) mod pet_controller {
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
    pub(crate) mod pet_service {
        include!("main/services/pet_service.rs");
    }
}

/// Builds a router from the same PetStore, service, and controller types used
/// by the generated application graph, with the supplied runtime settings.
pub fn build_petstore_router(
    config: &scafra::ScafraConfig,
) -> Result<scafra::web::axum::Router, scafra::web::WebError> {
    use scafra::web::ControllerRoutes;
    use std::sync::Arc;

    let store = Arc::new(services::pet_service::pet_store());
    let readiness_store = store.clone();
    let service = services::pet_service::PetService::new(store);
    let controller = controllers::pet_controller::PetController::new(service);
    let router = scafra::actuator::router(&config.actuator);
    let router = controllers::pet_controller::PetController::__scafra_register_routes_with(
        router, controller,
    );
    let routes = controllers::pet_controller::PetController::route_metadata();
    let router = scafra::web::finish_router(
        router,
        routes,
        &config.actuator,
        &config.security,
        config
            .server
            .request_timeout_seconds
            .map(std::time::Duration::from_secs),
    )?;
    // The health hook is process-wide; keep this router's store alive for its
    // full lifetime so that building a router is reflected in readiness.
    Ok(router.layer(scafra::web::axum::Extension(readiness_store)))
}
