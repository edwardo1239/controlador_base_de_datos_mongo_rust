
use serde::Serialize;
use serde_json::{Map, Value};

use super::lotes::Lote;

#[derive(Serialize)]
pub enum ApiResponse {
    Lotes(Vec<Lote>),
    InventarioFrutaSinProcesar(Map<String, Value>),
    // Puedes añadir más variantes según necesites
    Empty,
    Error(String)
}
