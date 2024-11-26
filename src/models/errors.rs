use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ServerErrorKind {
    BindError,        // Error al vincular el socket
    AcceptError,      // Error al aceptar una conexión
    ReadError,        // Error al leer datos del socket
    WriteError,       // Error al escribir datos en el socket
    ShutdownError,    // Error al apagar el servidor
    InvalidRequest, 
}

#[derive(Debug)]
pub struct ServerError {
    code: i32,
    kind: ServerErrorKind,
    message: String,
    location: String,
}

impl ServerError {
    // Constructor para crear un nuevo error del servidor
    pub fn new(code: i32, kind: ServerErrorKind, message: &str, location: &str) -> Self {
        ServerError {
            code,
            kind,
            message: message.to_string(),
            location: location.to_string(),
        }
    }

    // Métodos para acceder a los campos de `ServerError`
    pub fn kind(&self) -> &ServerErrorKind {
        &self.kind
    }

    pub fn code(&self) -> &i32 {
        &self.code
    }


    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn location(&self) -> &String {
        &self.location
    }
}

// Implementar el trait `fmt::Display` para mostrar mensajes de error legibles
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {} en '{}': {}", self.code,  self.location, self.message)
    }
}

// Implementar el trait `Error` para permitir el manejo de errores
impl Error for ServerError {}


//#region request errors 
#[derive(Debug)]
pub enum RequestErrorKind {
    EmptyAction,
    InvalidAction,
    InvalidJSON,
    UnknownAction,
    InvalidData,
    MissingField(String), // Nombre del campo que falta
    InvalidFieldType(String, String), // Nombre del campo y tipo esperado
    ActionNotAllowed(String), // Acción no permitida
    QueryError(String), // Error general en la consulta
    DatabaseError(String), // Error de base de datos
    ReadError, // Error al leer el archivo
    WriteError,
    DeserializeError

}

#[derive(Debug)]
pub struct RequestError {
    code: i32,
    kind: RequestErrorKind,
    message: String,
    location: String,
}

impl RequestError {
    // Constructor para crear un nuevo error del servidor
    pub fn new(code:i32, kind: RequestErrorKind, message: &str, location: &str) -> Self {
        RequestError {
            code,
            kind,
            message: message.to_string(),
            location: location.to_string(),
        }
    }

    // Métodos para acceder a los campos de `ServerError`
    pub fn kind(&self) -> &RequestErrorKind {
        &self.kind
    }

    pub fn code(&self) -> &i32 {
        &self.code
    }
    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn location(&self) -> &String {
        &self.location
    }
}

// Implementar el trait `fmt::Display` para mostrar mensajes de error legibles
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error en '{}': {}", self.location, self.message)
    }
}

// Implementar el trait `Error` para permitir el manejo de errores
impl Error for RequestError {}


// Implementación de From para convertir errores de MongoDB en RequestError
impl From<mongodb::error::Error> for RequestError {
    fn from(err: mongodb::error::Error) -> Self {
        RequestError::new(
            500,
            RequestErrorKind::DatabaseError(err.to_string()),
            &format!("Error de MongoDB: {}", err),
            "database_operation"
        )
    }
}



//error al inciar la base de datos mongoDB
#[derive(Clone, Debug)]
pub enum DbErrorKind {
    ConnectionError,     // Error al conectar con la base de datos
    ConfigurationError,  // Error en la configuración del cliente de la base de datos
    TimeoutError,        // Error por tiempo de espera al intentar conectar
    QueryError,          // Error al ejecutar una consulta
    DisconnectionError,  // Error al desconectar de la base de datos
}

#[derive(Debug)]
pub struct DbError {
    code: i32,
    kind: DbErrorKind,
    message: String,
    location: String,
}

impl DbError {
    pub fn new(code: i32, kind: DbErrorKind, message: &str, location: &str) -> Self {
        DbError {
            code,
            kind,
            message: message.to_string(),
            location: location.to_string(),
        }
    }

    pub fn kind(&self) -> &DbErrorKind {
        &self.kind
    }

    pub fn code(&self) -> &i32 {
        &self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn location(&self) -> &String {
        &self.location
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {} en '{}': {}", self.code, self.location, self.message)
    }
}

impl Error for DbError {}