
use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;

use super::lotes::Lote;

#[derive(Serialize)]
pub enum ApiResponse {
    Lotes(Vec<Lote>),
    VariablesDelSistema(HashMap<String, Value>),
    // Puedes añadir más variantes según necesites
    Empty,
    Error(String)
}
