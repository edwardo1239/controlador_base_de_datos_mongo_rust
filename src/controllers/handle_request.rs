use crate::{
    models::{
        errors::{RequestError, RequestErrorKind},
        response::ApiResponse,
    },
    routes::{lotes::route_functions_lotes, variables_sistema::routes_variables_sistema},
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub action: String,
    pub collection: String,
    pub data: HashMap<String, serde_json::Value>,
}

pub async fn serde_request(request: &str) -> Result<Query, RequestError> {
    if request.trim().is_empty() {
        return Err(RequestError::new(
            400,
            RequestErrorKind::EmptyAction,
            "La acción está vacía.",
            "controllers::handle_request::serde_request",
        ));
    }

    let query: Query = match serde_json::from_str::<Query>(request) {
        Ok(query) => {
            if query.action.is_empty() {
                return Err(RequestError::new(
                    422,
                    RequestErrorKind::EmptyAction,
                    "El campo 'action' está vacío.",
                    "Deseriacontrollers::handle_request::serde_requestlización",
                ));
            }
            query
        }
        Err(err) => {
            let (kind, message) = match err.classify() {
                serde_json::error::Category::Syntax => (
                    RequestErrorKind::InvalidJSON,
                    "El formato del JSON es inválido.",
                ),
                serde_json::error::Category::Data => (
                    RequestErrorKind::InvalidJSON,
                    "Los datos no coinciden con la estructura esperada.",
                ),
                _ => (
                    RequestErrorKind::InvalidJSON,
                    "Error desconocido en la deserialización.",
                ),
            };
            return Err(RequestError::new(
                400,
                kind,
                message,
                "controllers::handle_request::serde_request",
            ));
        }
    };
    Ok(query)
}

pub async fn route_request(request: Query) -> Result<ApiResponse, RequestError> {
    match request.collection.as_str() {
        "lotes" => match route_functions_lotes(request).await {
            Ok(data) => Ok(ApiResponse::Lotes(data)),
            Err(err) => Err(err),
        },
        "variables_del_sistema" => match routes_variables_sistema(request).await {
            Ok(data) => Ok(ApiResponse::InventarioFrutaSinProcesar(data)),
            Err(err) => Err(err),
        },
        _ => Err(RequestError::new(
            404,
            RequestErrorKind::UnknownAction,
            "Acción no reconocida",
            "Deseriacontrollers::handle_request::route_request",
        )),
    }
}
