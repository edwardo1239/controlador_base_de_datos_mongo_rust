extern crate dotenv;
use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::{
    env,
    process::{Command, Stdio},
};

use crate::models::errors::{DbError, DbErrorKind};

pub async fn init_db() -> Result<Client, DbError> {
    dotenv().ok();

    let db_uri = match env::var("MONGODB_URI") {
        Ok(db_uri) => db_uri,
        Err(err) => {
            return Err(DbError::new(
                500,
                DbErrorKind::ConfigurationError,
                &format!("No se pudo obtener la URI de la base de datos: {}", err),
                "init_db",
            ))
        }
    };

    let client_options = match ClientOptions::parse(&db_uri).await {
        Ok(client_options) => client_options,
        Err(err) => {
            return Err(DbError::new(
                500,
                DbErrorKind::ConfigurationError,
                &format!("Error en la configuración de la base de datos: {}", err),
                "init_db",
            ));
        }
    };

    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(err) => {
            return Err(DbError::new(
                500,
                DbErrorKind::ConfigurationError,
                &format!("Error al crear el cliente de la base de datos: {}", err),
                "init_db",
            ));
        }
    };

    println!("Conexión a la base de datos MongoDB establecida correctamente.");

    Ok(client)
}

pub async fn check_mongo_running() -> Result<bool, DbError> {

    // Intentar crear el cliente
    let client = match Client::with_uri_str("mongodb://localhost:27017/?maxPoolSize=50&minPoolSize=5&connectTimeoutMS=1000&serverSelectionTimeoutMS=5000&readConcernLevel=local&readPreference=secondaryPreferred").await {
        Ok(client) => client,
        Err(err) => {
            return Err(DbError::new(
                500,
                DbErrorKind::ConnectionError,
                &format!("Error al conectar con MongoDB: {}", err),
                "check_mongo_running",
            ));
        }
    };

    // Intentar hacer un ping a MongoDB
    match client.database("admin").run_command(doc! { "ping": 1 }).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub async fn start_mongo() -> Result<(), DbError> {
    match Command::new("mongod")
        .arg("--port")
        .arg("27017")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
            Ok(command) => command,
            Err(err) => {
                return Err(DbError::new(
                    500,
                    DbErrorKind::ConfigurationError,
                    &format!("No se pudo iniciar el servidor MongoDB: {}", err),
                    "start_mongo",
                ));
            }
        };

    // No se espera que el comando termine
    println!("MongoDB debería estar corriendo ahora en segundo plano.");
    Ok(())
}

