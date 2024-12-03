use serde_json::Value;

use crate::{
    controllers::handle_request::Query,
    models::errors::{RequestError, RequestErrorKind},
    utils::file_utils::{
        open_bin_inventario_fruta_sin_procesar, save_bin_inventario_fruta_sin_procesar,
    },
};

pub async fn ingresar_inventario(request: Query) -> Result<(), RequestError> {
    let data = request.data;

    let _id = match data.get("_id").and_then(|_id| _id.as_str()) {
        Some(_id) => _id,
        None => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON no existe enf"),
                "read::variables_sistema::generar_ef1_serial",
            ));
        }
    };

    let canastillas = match data
        .get("canastillas")
        .and_then(|canastillas| canastillas.as_i64())
    {
        Some(canastillas) => canastillas,
        None => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON no existe enf"),
                "write::variables_sistema::ingresar_inventario",
            ));
        }
    };

    let mut inventario = match open_bin_inventario_fruta_sin_procesar("./DB/inventario.bin") {
        Ok(map) => map,

        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON {}", err),
                "write::variables_sistema::ingresar_inventario",
            ));
        }
    };

    // Insertar el nuevo dato en el inventario
    inventario.insert(_id.to_string(), Value::Number(canastillas.into()));

    match save_bin_inventario_fruta_sin_procesar("./DB/inventario.bin", &inventario) {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON {}", err),
                "write::variables_sistema::ingresar_inventario",
            ));
        }
    }
}

pub async fn modificar_inventario(request: Query) -> Result<(), RequestError> {
    let data = request.data;

    let _id = match data.get("_id").and_then(|_id| _id.as_str()) {
        Some(_id) => _id,
        None => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON no existe el id"),
                "read::variables_sistema::modificar_inventario",
            ));
        }
    };

    let canastillas = match data
        .get("canastillas")
        .and_then(|canastillas| canastillas.as_i64())
    {
        Some(canastillas) => canastillas,
        None => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON no existe canastillas"),
                "write::variables_sistema::modificar_inventario",
            ));
        }
    };

    let mut inventario = match open_bin_inventario_fruta_sin_procesar("./DB/inventario.bin") {
        Ok(map) => map,

        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON {}", err),
                "write::variables_sistema::modificar_inventario",
            ));
        }
    };

    let item = match inventario.get(_id) {
        Some(data) => match data.as_i64() {
            Some(value) => value,
            None => {
                return Err(RequestError::new(
                    408,
                    RequestErrorKind::ReadError,
                    "Error convirtiendo el dato en i64",
                    "write::variables_sistema::modificar_inventario",
                ));
            }
        },
        None => {
            // Insertar el nuevo dato en el inventario
            inventario.insert(_id.to_string(), Value::Number(canastillas.into()));

            match save_bin_inventario_fruta_sin_procesar("./DB/inventario.bin", &inventario) {
                Ok(_) => {
                    println!("Modificado con exito");
                    return Ok(())
                }
                Err(err) => {
                    return Err(RequestError::new(
                        404,
                        RequestErrorKind::ReadError,
                        &format!("Erros leyendo el archivo JSON {}", err),
                        "write::variables_sistema::ingresar_inventario",
                    ));
                }
            }

        
        }
    };

    let new_canastillas = item - canastillas;
    // Convierte el nuevo valor a `Value`
    let new_value = Value::from(new_canastillas);

    if new_canastillas > 0 {
        match inventario.insert(_id.to_string(), new_value) {
            Some(data) => data,
            None => {
                return Err(RequestError::new(
                    408,
                    RequestErrorKind::WriteError,
                    &format!("Error modificando el dato del inventario:"),
                    "write::variables_sistema::modificar_inventario",
                ))
            }
        };
    } else if new_canastillas == 0 {
        match inventario.remove(_id) {
            Some(data) => data,
            None => {
                return Err(RequestError::new(
                    408,
                    RequestErrorKind::WriteError,
                    &format!("Error modificando el dato del inventario:"),
                    "write::variables_sistema::modificar_inventario",
                ))
            }
        };
    } else if new_canastillas < 0 {
        return Err(RequestError::new(
            408,
            RequestErrorKind::WriteError,
            &format!("Error modificando el dato del inventario: el resultado es negativo"),
            "write::variables_sistema::modificar_inventario",
        ));
    }

    match save_bin_inventario_fruta_sin_procesar("./DB/inventario.bin", &inventario) {
        Ok(_) => {
            println!("Modificado con exito")
        }
        Err(err) => {
            return Err(RequestError::new(
                404,
                RequestErrorKind::ReadError,
                &format!("Erros leyendo el archivo JSON {}", err),
                "write::variables_sistema::modificar_inventario",
            ));
        }
    }

    Ok(())
}
