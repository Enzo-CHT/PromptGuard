use crate::service::{
    anonymization_service::AnonymizationService, drs_service::DrsService, mask_service::MaskService,
};
use config::AppConfig;
use controller::app_state::AppState;
use repository::sled_mask_repository::SledMaskRepository;
use std::sync::Arc;
use tracing::{Level, event, instrument};
use tracing_subscriber::fmt;

mod controller;
mod service;

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    fmt().init();

    let config = AppConfig::load("../config/properties.yml")
        .expect("impossible de charger la configuration");
    let base_path = config.resources.db.base.to_string().clone();

    let sled_repo = Arc::new(SledMaskRepository::new(
        sled::open(base_path).expect("Impossible d'ouvrir la source de données persistante"),
    ));

    let mask_service = MaskService::new(sled_repo);

    let app_state = AppState {
        d2c_service: Arc::new(AnonymizationService::new(mask_service)),
        drs_service: Arc::new(DrsService::new("http://localhost:8080")),
    };

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9990")
        .await
        .unwrap();
    event!(
        Level::INFO,
        "Le serveur est disponible à l'adresse : http://localhost:9990"
    );
    axum::serve(
        listener,
        controller::routes::app_routes().with_state(app_state),
    )
    .await
    .unwrap();

    Ok(())
}
