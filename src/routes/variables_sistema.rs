use std::collections::HashMap;

use serde_json::Value;

use crate::{
    controllers::handle_request::Query,
    models::errors::{RequestError, RequestErrorKind},
    read::variables_sistema::get_inventario,
    write::variables_sistema::{ingresar_inventario, modificar_inventario},
};

pub async fn routes_variables_sistema(
    request: Query,
) -> Result<HashMap<String, Value>, RequestError> {
    println!("{:?}", request);
    match request.action.as_str() {
        "get_inventario" => match get_inventario().await {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        },
        "ingresar_inventario" => match ingresar_inventario(request).await {
            Ok(_) => {
                let mut response = HashMap::new();
                response.insert("status".to_string(), Value::String("200".to_string()));
                response.insert("message".to_string(), Value::String("Ok".to_string()));
                Ok(response)
            }
            Err(err) => Err(err),
        },
        "modificar_inventario" => match modificar_inventario(request).await {
            Ok(_) => {
                let mut response = HashMap::new();
                response.insert("status".to_string(), Value::String("200".to_string()));
                response.insert("message".to_string(), Value::String("Ok".to_string()));
                Ok(response)
            }
            Err(err) => Err(err),
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
