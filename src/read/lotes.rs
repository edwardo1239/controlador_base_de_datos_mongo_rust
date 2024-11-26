use std::time::Instant;

use mongodb::{
    bson::{doc, from_document, oid::ObjectId, Document},
    options::Hint,
    Client,
};
use serde::Deserialize;

use crate::models::{
    errors::{RequestError, RequestErrorKind},
    lotes::Lote,
};

use futures_util::stream::StreamExt;

#[derive(Debug, Deserialize)]
pub struct GetLotesOptions {
    #[serde(default)]
    ids: Vec<ObjectId>,
    #[serde(default)]
    query: Document,
    #[serde(default)]
    select: Document,
    #[serde(default = "default_sort")]
    sort: Document,
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    skip: i64,
}

fn default_sort() -> Document {
    doc! { "fecha_ingreso": -1 }
}

fn default_limit() -> i64 {
    50
}

impl Default for GetLotesOptions {
    fn default() -> Self {
        Self {
            ids: Vec::new(),
            query: Document::new(),
            select: Document::new(),
            sort: default_sort(),
            limit: default_limit(),
            skip: 0,
        }
    }
}

pub async fn get_lotes(options: GetLotesOptions) -> Result<Vec<Lote>, RequestError> {
    let client = match Client::with_uri_str("mongodb://localhost:27017/?maxPoolSize=50&minPoolSize=5&connectTimeoutMS=1000&serverSelectionTimeoutMS=5000&readConcernLevel=local&readPreference=secondaryPreferred").await {
        Ok(client) => client,
        Err(err) => {
            return Err(RequestError::new(
                500,
                RequestErrorKind::DatabaseError(err.to_string()),
                "No se pudo conectar a la base de datos",
                "read::lotes::get_lotes",
            ))
        }
    };

    let db = client.database("proceso");
    let collection = db.collection::<Document>("lotes");

    // Construir el match stage para los filtros
    let mut match_stage = options.query.clone();
    if !options.ids.is_empty() {
        match_stage.insert("_id", doc! { "$in": options.ids.clone() });
    }

    let mut pipeline = vec![
        // Stage 1: Match (filtros)
        doc! { "$match": match_stage }, // Stage 2: Lookup
    ];

    // Stage 3: Sort (si hay sort)
    if !options.sort.is_empty() {
        pipeline.push(doc! { "$sort": options.sort });
    }

    // Stage 4: Skip (paginación)
    if options.skip > 0 {
        pipeline.push(doc! { "$skip": options.skip });
    }

    // Stage 5: Limit (paginación)
    if options.limit > 0 {
        pipeline.push(doc! { "$limit": options.limit });
    }

    // Stage 6: Project (selección de campos)
    if !options.select.is_empty() {
        pipeline.push(doc! { "$project": options.select });
    }

    pipeline.push(doc! {
        "$lookup": doc! {
            "from": "proveedors",
            "localField": "predio",
            "foreignField": "_id",
            "as": "predio",
            "pipeline": [{
                "$project": {
                    "_id": 1,
                    "PREDIO": 1,
                    "ICA": 1
                }
            }]

        }
    });

    let inicio = Instant::now();


    let mut cursor = match collection
        .aggregate(pipeline)
        .hint(Hint::Keys(
            doc! { "fechaCreacion": 1, "_id": 1, "predios": 1 },
        )).await
    {
        Ok(cursor) => cursor,
        Err(err) => {
            return Err(RequestError::new(
                500,
                RequestErrorKind::DatabaseError(err.to_string()),
                "Error al ejecutar la agregación en la colección 'lotes'",
                "read::lotes::get_lotes",
            ));
        }
    };

    let duracion = inicio.elapsed();
    println!("La función tomó: {:?}", duracion);


    let mut results = Vec::new();
    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(document) => {
                // Intenta convertir el documento BSON en un `Lote`
                match from_document::<Lote>(document) {
                    Ok(lote) => results.push(lote),
                    Err(err) => {
                        // Manejar error de deserialización
                        eprintln!("Error al deserializar el documento: {:?}", err);
                    }
                }
            }
            Err(err) => {
                // Manejar el error si ocurre en la iteración del cursor
                eprintln!("Error al leer el documento: {:?}", err);
            }
        }
    }

    // Ahora puedes trabajar con los resultados
    Ok(results)
}
