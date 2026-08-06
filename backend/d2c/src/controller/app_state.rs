use crate::service::anonymization_service::AnonymizationService;
use crate::service::drs_service::DrsService;
use axum::extract::FromRef;

use std::sync::Arc;
#[derive(Clone)]
pub struct AppState {
    pub d2c_service: Arc<AnonymizationService>,
    pub drs_service: Arc<DrsService>,
}

impl FromRef<AppState> for Arc<AnonymizationService> {
    fn from_ref(state: &AppState) -> Self {
        state.d2c_service.clone()
    }
}
impl FromRef<AppState> for Arc<DrsService> {
    fn from_ref(state: &AppState) -> Self {
        state.drs_service.clone()
    }
}
