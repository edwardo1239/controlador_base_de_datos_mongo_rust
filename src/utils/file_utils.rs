use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use bincode::{deserialize, serialize};
use serde_json::{Map, Value};

use crate::models::errors::{RequestError, RequestErrorKind};
pub enum JsonStructure {
    Object(Map<String, Value>),
    Array(Vec<Value>),
}

pub fn open_json(path: &str) -> Result<JsonStructure, RequestError> {
    let path = Path::new(path);

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Error abriendo inventario.json {}", err),
                "utils::open_json",
            ))
        }
    };

    let reader = BufReader::new(file);
    let json_data: Value = match serde_json::from_reader(reader) {
        Ok(json_data) => json_data,
        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Error parseando inventario.json {}", err),
                "utils::open_json",
            ))
        }
    };

    //se determina que tipo de estructura JSON se devuelve
    match json_data {
        Value::Object(map) => Ok(JsonStructure::Object(map)),
        Value::Array(vec) => Ok(JsonStructure::Array(vec)),
        _ => Err(RequestError::new(
            404,
            RequestErrorKind::ReadError,
            "El JSON no es ni un objeto ni un array",
            "utils::open_json",
        )),
    }
}

pub fn open_bin_inventario_fruta_sin_procesar(
    path: &str,
) -> Result<HashMap<String, Value>, RequestError> {
    // Intentar leer el archivo binario
    let file_data = match fs::read(path) {
        Ok(data) => data,
        Err(err) => {
            return Err(RequestError::new(
                403,
                RequestErrorKind::ReadError,
                &format!("Error abriendo el archivo {:?}", err),
                "utils::file_utils::open_bin_inventario_fruta_sin_procesar",
            ));
        }
    };

    // Verificar si el archivo está vacío
    if file_data.is_empty() {
        return Ok(HashMap::new());
    }

    let serialized_map = match deserialize::<HashMap<String, i32>>(&file_data) {
        Ok(datos) => datos,
        Err(err) => {
            return Err(RequestError::new(
                500,
                RequestErrorKind::DeserializeError,
                &format!("Error deserializando los datos: {:?}", err),
                "utils::file_utils::open_bin_inventario_fruta_sin_procesar",
            ));
        }
    };

    // Convertir de InventarioItem a Value
    let map: HashMap<String, Value> = serialized_map
        .into_iter()
        .map(|(k, v)| {
            let value = Value::Number(serde_json::Number::from(v));
            (k, value)
        })
        .collect();

    println!("{:?}", map);
    // Si todo salió bien, retornamos el HashMap
    Ok(map)
}

pub fn save_bin_inventario_fruta_sin_procesar(
    path: &str,
    map: &HashMap<String, Value>,
) -> Result<(), RequestError> {
    println!("{:?}", map);

    let serialized_map: HashMap<String, i32> = map
    .into_iter()
    .map(|(k, v)| {
        let value = match v {
            Value::Number(i) => i.as_i64().unwrap_or(0) as i32,
            _ => 0
        };
        (k.to_string(), value)
    })
    .collect();

    let encode = match serialize::<HashMap<String, i32>>(&serialized_map) {
        Ok(encode) => encode,
        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::WriteError,
                &format!("Error {:?}", err),
                "utils::file_utils::save_bin_inventario_fruta_sin_procesar",
            ));
        }
    };

    match fs::write(path, encode) {
        Ok(_) => println!("Guardado con exito!"),
        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::WriteError,
                &format!("Error {:?}", err),
                "utils::file_utils::save_bin_inventario_fruta_sin_procesar",
            ));
        }
    }

    Ok(())
}
