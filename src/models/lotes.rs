use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::proveedores::Predio;

#[derive(Debug, Serialize, Deserialize)]
struct CalidadInterna {
    acidez: Option<f64>,
    brix: Option<f64>,
    ratio: Option<f64>,
    peso: Option<f64>,
    zumo: Option<f64>,
    fecha: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClasificacionCalidad {
    acaro: Option<f64>,
    alsinoe: Option<f64>,
    dannos_mecanicos: Option<f64>,
    deshidratada: Option<f64>,
    division: Option<f64>,
    escama: Option<f64>,
    fruta_madura: Option<f64>,
    fruta_verde: Option<f64>,
    fumagina: Option<f64>,
    grillo: Option<f64>,
    herbicida: Option<f64>,
    mancha: Option<f64>,
    melanosis: Option<f64>,
    oleocelosis: Option<f64>,
    piel: Option<f64>,
    sombra: Option<f64>,
    trips: Option<f64>,
    wood: Option<f64>,
    nutrientes: Option<f64>,
    antracnosis: Option<f64>,
    fruta_rajada: Option<f64>,
    ombligona: Option<f64>,
    despezonada: Option<f64>,
    variegacion: Option<f64>,
    verde_manzana: Option<f64>,
    otras_plagas: Option<f64>,
    fecha: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FotosCalidad {
    any: Option<HashMap<String, String>>,
    fecha_ingreso: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InspeccionIngreso {
    maduro: Option<f64>,
    deshidratacion: Option<f64>,
    mancha: Option<f64>,
    defecto: Option<f64>,
    oleocelosis: Option<f64>,
    daño_mecanico: Option<f64>,
    verde_manzana: Option<f64>,
    parejo: Option<f64>,
    exportacion1: Option<f64>,
    exportacion15: Option<f64>,
    exportacion2: Option<f64>,
    fecha: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Calidad {
    inspeccion_ingreso: Option<InspeccionIngreso>,
    calidad_interna: Option<CalidadInterna>,
    clasificacion_calidad: Option<ClasificacionCalidad>,
    fotos_calidad: Option<FotosCalidad>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DescarteLavado {
    descarte_general: Option<f64>,
    pareja: Option<f64>,
    balin: Option<f64>,
    descompuesta: Option<f64>,
    piel: Option<f64>,
    hojas: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DescarteEncerado {
    descarte_general: Option<f64>,
    pareja: Option<f64>,
    balin: Option<f64>,
    extra: Option<f64>,
    descompuesta: Option<f64>,
    suelo: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SalidaDirectoNacional {
    placa: Option<String>,
    nombre_conductor: Option<String>,
    telefono: Option<String>,
    cedula: Option<String>,
    remision: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Parametro {
    fecha: Option<DateTime>,
    temperatura: Option<f64>,
    etileno: Option<f64>,
    carbono: Option<f64>,
    humedad: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Desverdizado {
    canastillas_ingreso: Option<i32>,
    kilos_ingreso: Option<f64>,
    cuarto_desverdizado: Option<String>,
    fecha_ingreso: Option<DateTime>,
    fecha_finalizar: Option<DateTime>,
    desverdizando: Option<bool>,
    canastillas_salida: Option<i32>,
    parametros: Option<Vec<Parametro>>,
    fecha_procesado: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContenedorDetalle {
    #[serde(rename = "1")]
    uno: Option<f64>,
    #[serde(rename = "15")]
    quince: Option<f64>,
    #[serde(rename = "2")]
    dos: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExportacionDetallada {
    any: Option<HashMap<String, ContenedorDetalle>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrecioLote {
    #[serde(rename = "1")]
    uno: Option<f64>,
    #[serde(rename = "15")]
    quince: Option<f64>,
    #[serde(rename = "2")]
    dos: Option<f64>,
    #[serde(rename = "frutaNacional")]
    fruta_nacional: Option<f64>,
    descarte: Option<f64>,
    zumex: Option<f64>,
    combinado: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lote {
    _id: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    aprobacion_comercial: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calidad: Option<Calidad>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calidad1: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calidad15: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calidad2: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    canastillas: Option<String>,

    #[serde(rename = "clasificacionCalidad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    clasificacion_calidad: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    contenedores: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descarte_encerado: Option<DescarteEncerado>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descarte_lavado: Option<DescarteLavado>,

    #[serde(skip_serializing_if = "Option::is_none")]
    deshidratacion: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    desverdizado: Option<Desverdizado>,

    #[serde(skip_serializing_if = "Option::is_none")]
    directo_nacional: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enf: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exportacion_detallada: Option<ExportacionDetallada>,

    #[serde(rename = "fechaIngreso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    fecha_ingreso: Option<DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fecha_proceso: Option<DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fecha_finalizado_proceso: Option<DateTime>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    fruta_nacional: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    flag_is_favorita: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    historial_descarte: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    informe_enviado: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    info_salida_directo_nacional: Option<SalidaDirectoNacional>,

    #[serde(skip_serializing_if = "Option::is_none")]
    kilos: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    kilos_reprocesados: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    kilos_vaciados: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    kilos_ggn: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    numero_precintos: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    numero_remision: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    observaciones: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placa: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    precio: Option<PrecioLote>,

    #[serde(skip_serializing_if = "Option::is_none")]
    predio: Option<Vec<Predio>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    promedio: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rendimiento: Option<f64>,
    
    #[serde(rename = "tipoFruta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tipo_fruta: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url_bascula: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url_informe_calidad: Option<String>,
}
