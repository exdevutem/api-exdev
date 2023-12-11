//! Módulo inicial de la API, o implementación base.
//!
//! Cada módulo de la API contiene los modelos de conexión con la BDD, los handles (en la carpeta
//! res) para cada respuesta de la API, los responders (o estructuras diseñadas para tener
//! respuestas estándar) y los schemas (estructuras para las respuestas específicas de cada
//! handler.)
//!
//! La idea es que, en caso de necesitar cambiar las rutas, eliminar algunas, o qué se yo, una vez
//! que la API esté en "producción", podamos crear un nuevo módulo copia de este, cosa de que el
//! periodo de migración hacia una "versión 2" de la API sea mucho más manejable.

use actix_web::web;

pub mod models;
pub mod res;
pub mod responders;
pub mod schemas;

/// Función de rutas del módulo
///
/// Esta función agrupa todas las rutas dentro del módulo "res" bajo el scope `/v1`. En términos
/// prácticos, esto significa que todas las rutas de este módulo empiezan con este `/v1` antes del
/// resto de su URL.
pub fn routes() -> actix_web::Scope {
    web::scope("/v1")
        .service(res::add_member_routes())
        .service(res::add_auth_routes())
        .service(res::add_project_routes())
}
