
use serde_json::{Map, Value};

use crate::{controllers::handle_request::Query, models::errors::{RequestError, RequestErrorKind}, read::variables_sistema::get_inventario};


pub async fn routes_variables_sistema(request: Query) -> Result<Map<String, Value>, RequestError>{
    match request.action.as_str() {
        "get_inventario" => {
            match get_inventario().await {
                Ok(data) => Ok(data),
                Err(err) => Err(err),
            }
        }
        _ => {
            let action_err = request.action;
            return Err(RequestError::new(
                404,
                RequestErrorKind::InvalidAction,
                &format!("No existe {:?} no existe", action_err),
                "routes::lotes::route_functions_lotes",
            ));
        }
    }
}