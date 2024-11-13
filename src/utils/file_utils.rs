use std::{fs::File, io::BufReader, path::Path};

use serde_json::{Map, Value};

use crate::models::errors::{RequestError, RequestErrorKind};
pub enum JsonStructure {
    Object(Map<String, Value>),
    Array(Vec<Value>),
}

pub fn open_json(path: &str) -> Result<JsonStructure, RequestError>{
    let path = Path::new(path);

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            return Err(RequestError::new(
                404, 
                RequestErrorKind::ReadError, 
                &format!("Error abriendo inventario.json {}", err), 
                "utils::open_json"
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
                "utils::open_json"
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
                "utils::open_json"
            ))
        }
}