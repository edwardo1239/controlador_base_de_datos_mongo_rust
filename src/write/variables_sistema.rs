use serde_json::Value;

use crate::{
    controllers::handle_request::Query,
    models::errors::{RequestError, RequestErrorKind},
    utils::file_utils::{open_bin_inventario_fruta_sin_procesar, save_bin_inventario_fruta_sin_procesar},
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
    println!("{:?}",_id);

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

    println!("{:?}",canastillas);

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

    println!("1  {:?}", inventario);
    // Insertar el nuevo dato en el inventario
    inventario.insert(_id.to_string(), Value::Number(canastillas.into())); 

    println!("2  {:?}", inventario);

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
