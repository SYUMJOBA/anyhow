use std::sync::{Mutex, MutexGuard, OnceLock};

mod service;

pub(super) use service::Service as Service;

#[derive(Clone, Debug)]
struct ForeignComplexPointer {
    pub(super) schema_id: usize,
    pub(super) table_id: usize
}

#[derive(Clone, Debug)]
struct ForeignOptionalAnyComplexPointer {
    pub(super) schema_id: Option<usize>,
    pub(super) table_id: Option<usize>
}

#[derive(Clone, Debug)]
struct ForeignOptionalComplexPointer {
    pub(super) schema_id: usize,
    pub(super) table_id: Option<usize>
}

#[derive(Clone, Debug)]
struct ForeignSelectComplexPointer {
    pub(super) accepted_schemas: Vec<usize>,
    pub(super) schema_id: usize,
    pub(super) table_id: usize
}

#[derive(Clone, Debug)]
struct ForeignOptionalSelectComplexPointer {
    pub(super) accepted_schemas: Vec<usize>,
    pub(super) schema_id: usize,
    pub(super) table_id: Option<usize>
}

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
    Complex(ForeignComplexPointer),
    AnyComplex(ForeignComplexPointer),
    SelectComplex(ForeignSelectComplexPointer), // table id
    NullableInteger(Option<i32>),
    NullableFloat(Option<f32>),
    NullableString(Option<String>),
    NullableComplex(ForeignOptionalComplexPointer),
    NullableAnyComplex(ForeignOptionalAnyComplexPointer),
    NullableSelectComplex(ForeignOptionalSelectComplexPointer),
}

#[derive(Clone, Debug)]
pub struct Complex {
    pub(super) id: usize,
    pub(super) schema_id: usize,
    pub(super) members: Vec<Value>
}