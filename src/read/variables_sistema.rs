use chrono::{Datelike, Utc};
use serde_json::{Map, Value};
use std::{collections::HashMap, time::Instant};

use crate::{
    models::errors::{RequestError, RequestErrorKind},
    utils::file_utils::{open_bin_inventario_fruta_sin_procesar, open_json, JsonStructure},
};

pub async fn get_inventario() -> Result<HashMap<String, Value>, RequestError> {

    let inventario = match open_bin_inventario_fruta_sin_procesar("./DB/inventario.bin") {
        Ok(map) => map,

        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el inventario fruta sin procesar {}", err),
                "read::variables_sistema::get_inventario",
            ));
        }
    };

    println!("{:?}", inventario);
    Ok(inventario)
}

pub async fn generar_ef1_serial() -> Result<Map<String, Value>, RequestError> {
    let inicio = Instant::now();

    let seriales = match open_json("./DB/seriales.json") {
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
                "read::variables_sistema::generar_ef1_serial",
            ));
        }
    };

    let enf_serial = match seriales.get("enf").and_then(|enf| enf.as_i64()) {
        Some(enf_data) => enf_data,
        None => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON no existe enf"),
                "read::variables_sistema::generar_ef1_serial",
            ));
        }
    };

    let fecha_actual = Utc::now();
    let year = fecha_actual.year() % 100;
    let month = format!("{:02}", fecha_actual.month());

    let enf = if enf_serial < 10 {
        format!("EF1-{}{}0{}", year, month, enf_serial)
    } else {
        format!("EF1-{}{}{}", year, month, enf_serial)
    };

    // Crear el HashMap
    let mut resultado = Map::new();
    resultado.insert("enf".to_string(), Value::String(enf));

    let duracion = inicio.elapsed();
    println!("La función tomó: {:?}", duracion);

    Ok(resultado)
}
