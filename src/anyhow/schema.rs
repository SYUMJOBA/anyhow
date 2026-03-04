use std::sync::{Arc, Mutex, OnceLock};

use super::ValueKind;
mod service;

pub(super) use service::Service as Service;

#[derive(Clone, Debug)]
pub struct ValueSchema {
    pub(super) name: String,
    pub(super) kind: ValueKind,
    pub(super) nullable: bool
}

#[derive(Clone, Debug)]
pub struct ComplexSchema {
    pub(super) id: usize,
    pub(super) name: String,
    pub(super) schema: Vec<ValueSchema>
}
