use std::sync::{Arc, Mutex, OnceLock};

mod service;

use service::Service as Service;

#[derive(Clone, Debug)]
pub enum ValueSchema {
    Integer(String),
    Float(String),
    String(String),
    Complex(String, usize) // schema id
}

#[derive(Clone, Debug)]
pub struct ComplexSchema {
    pub(super) id: usize,
    pub(super) name: String,
    pub(super) schema: Vec<ValueSchema>
}
