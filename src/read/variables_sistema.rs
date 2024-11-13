use std::time::Instant;

use serde_json::{Map, Value};

use crate::{
    models::errors::{RequestError, RequestErrorKind},
    utils::file_utils::{open_json, JsonStructure},
};

pub async fn get_inventario() -> Result<Map<String, Value>, RequestError> {
    let inicio = Instant::now();

    let inventario = match open_json("./DB/inventario.json") {
        Ok(JsonStructure::Object(map)) => map,
        Ok(JsonStructure::Array(vec)) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Error en la estructura del JSON {:?}", vec),
                "read::variables_sistema::get_inventario",
            ))
        }
        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON {}", err),
                "read::variables_sistema::get_inventario",
            ));
        }
    };

    let duracion = inicio.elapsed();
    println!("La función tomó: {:?}", duracion);

    Ok(inventario)
}
