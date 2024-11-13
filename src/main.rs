use std::{error::Error, process};

use bdserver::{
    config::config_db::{check_mongo_running, start_mongo},
    controllers::connections::handle_connection,
    models::errors::{DbError, ServerError, ServerErrorKind},
};
use tokio::{self, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        if let Some(source) = e.source() {
            eprintln!("Caused by:{source}")
        }
        process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    let is_running = match check_mongo_running().await {
        Ok(is_running) => is_running,
        Err(err) => {
            let code = *err.code(); // Aquí ya no es necesario el * para mover el código
            let kind = err.kind().clone(); // Clonamos DbErrorKind
            let message = err.message().clone();
            let location = err.location().clone();

            return Err(Box::new(DbError::new(
                code,
                kind,
                message.as_str(),
                location.as_str(),
            )));
        }
    };

    println!("{}", is_running);

    if is_running {
        println!("La base de datos esta conectada")
    }

    if !is_running {
        println!("Iniciando la base de datos...");

        match start_mongo().await {
            Ok(_) => (),
            Err(err) => {
                let code = *err.code(); // Aquí ya no es necesario el * para mover el código
                let kind = err.kind().clone(); // Clonamos DbErrorKind
                let message = err.message().clone();
                let location = err.location().clone();

                return Err(Box::new(DbError::new(
                    code,
                    kind,
                    message.as_str(),
                    location.as_str(),
                )));
            }
        };
    }

    let listener = match TcpListener::bind("127.0.0.1:3030").await {
        Ok(listener) => {
            println!("Servidor escuchando en 127.0.0.1:3030");
            listener
        }
        Err(err) => {
            return Err(Box::new(ServerError::new(
                400,
                ServerErrorKind::BindError,
                &format!("Error al vincular el socket: {}", err),
                "run", // Puedes cambiar esto a la ubicación adecuada
            )));
        }
    };

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("Conexión aceptada de {:?}", addr);
                tokio::spawn(async move {
                    if let Err(err) = handle_connection(socket).await {
                        eprintln!("Error al manejar la conexión: {}", err);
                    }
                });
            }

            Err(err) => {
                return Err(Box::new(ServerError::new(
                    400,
                    ServerErrorKind::AcceptError,
                    &format!("Error al aceptar la conexión: {}", err),
                    "run", // Cambia esto a la ubicación adecuada
                )));
            }
        }
    }
}
