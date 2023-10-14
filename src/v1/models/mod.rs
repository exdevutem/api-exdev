//! Módulo de modelos de la base de datos.
//!
//! Este modulo contiene una abstracción directa de una fila de una base de datos hacia una
//! estructura de Rust para cada tabla de la Base de datos.
//!
//! Además de eso, cada módulo contiene las funciones típicas de un CRUD sobre este recurso (o sea,
//! cinco funciones: obtener uno, obtener varios, crear, actualizar y eliminar)

pub mod auth;
pub mod club_member;
pub mod project;
