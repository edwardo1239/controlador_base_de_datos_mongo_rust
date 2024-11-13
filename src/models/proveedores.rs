use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LimonPrecio {
    #[serde(rename = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    uno: Option<f64>,

    #[serde(rename = "15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    quince: Option<f64>,

    #[serde(rename = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dos: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descarte: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    combinado: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NaranjaPrecio {
    #[serde(rename = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    uno: Option<f64>,

    #[serde(rename = "15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    quince: Option<f64>,

    #[serde(rename = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dos: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descarte: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zumex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Precio {
    #[serde(skip_serializing_if = "Option::is_none")]
    limon: Option<LimonPrecio>,

    #[serde(skip_serializing_if = "Option::is_none")]
    naranja: Option<NaranjaPrecio>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fecha: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GGN {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fecha_vencimiento: Option<DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    paises: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Predio {
    #[serde(rename = "PREDIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    predio: Option<String>,

    #[serde(rename = "ICA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ica: Option<String>,

    #[serde(rename = "CODIGO INTERNO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    codigo_interno: Option<String>,

    #[serde(rename = "GGN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ggn: Option<GGN>,

    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<bool>,

    #[serde(rename = "L")]
    #[serde(skip_serializing_if = "Option::is_none")]
    l: Option<bool>,

    #[serde(rename = "M")]
    #[serde(skip_serializing_if = "Option::is_none")]
    m: Option<bool>,

    #[serde(rename = "PROVEEDOR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    proveedores: Option<String>,

    #[serde(rename = "DEPARTAMENTO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    departamento: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url_archivos: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activo: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    precio: Option<Precio>,

    #[serde(rename = "SISPAP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sispap: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    alt: Option<ObjectId>,
}
