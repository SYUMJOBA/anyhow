use std::sync::{Mutex, MutexGuard, OnceLock};

mod service;

use service::Service;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
    Complex(usize) // table id
}

#[derive(Clone, Debug)]
pub struct Complex {
    pub(super) id: usize,
    pub(super) schema_id: usize,
    pub(super) members: Vec<Value>
}