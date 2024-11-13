use crate::{
    controllers::handle_request::Query,
    models::{
        errors::{RequestError, RequestErrorKind},
        lotes::Lote,
    },
    read::lotes::{get_lotes, GetLotesOptions},
};
use mongodb::bson::{from_document, to_document};


pub async fn route_functions_lotes(request: Query) -> Result<Vec<Lote>, RequestError> {
    match request.action.as_str() {
        "get_lotes" => {
            // Convertir request.data a Document
            let document_data = match to_document(&request.data) {
                Ok(doc) => doc,
                Err(err) => {
                    return Err(RequestError::new(
                        400,
                        RequestErrorKind::InvalidData,
                        &format!("Error al convertir datos a Document: {}", err),
                        "routes::lotes::route_functions_lotes",
                    ));
                }
            };

            // Ahora puedes deserializar document_data a GetLotesOptions
            let options: GetLotesOptions = match from_document(document_data) {
                Ok(opts) => opts,
                Err(err) => {
                    return Err(RequestError::new(
                        400,
                        RequestErrorKind::InvalidData,
                        &format!("Error al deserializar datos: {}", err),
                        "routes::lotes::route_functions_lotes",
                    ));
                }
            };

            // Ahora puedes llamar a get_lotes
            match get_lotes(options).await {
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
