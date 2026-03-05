
use crate::anyhow::ComplexSchema;

use super::SchemaService;

use super::error::Error;

mod service;

pub(super) use service::Service as Service;

#[derive(Clone, Debug)]
pub(super) struct ForeignComplexPointer {
    pub(super) schema_id: usize,
    pub(super) table_id: usize
}

#[derive(Clone, Debug)]
pub(super) struct ForeignOptionalAnyComplexPointer {
    pub(super) schema_id: Option<usize>,
    pub(super) table_id: Option<usize>
}

#[derive(Clone, Debug)]
pub(super) struct ForeignOptionalComplexPointer {
    pub(super) schema_id: usize,
    pub(super) table_id: Option<usize>
}

#[derive(Clone, Debug)]
pub(super) struct ForeignSelectComplexPointer {
    pub(super) accepted_schemas: Vec<usize>,
    pub(super) schema_id: usize,
    pub(super) table_id: usize
}

#[derive(Clone, Debug)]
pub(super) struct ForeignOptionalSelectComplexPointer {
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
pub struct Member {
    pub(super) id: usize,
    pub(super) value: Value
}

impl Member {
    pub fn new(id: usize, value: Value) -> Self {
        Self { id, value }
    }
}

#[derive(Clone, Debug)]
pub struct Complex {
    pub(super) id: usize,
    pub(super) schema_id: usize,
    pub(super) members: Vec<Member>
}

impl Complex {
    pub fn get_member_id_by_name(&self, name: String) -> Result<usize, Error> {
        SchemaService::get_schema_by_id(self.schema_id)?.lock().unwrap_or_else(|posion| posion.into_inner()).get_member_id_by_name(name)
    }

    pub fn get_member_by_id(&self, id: usize) -> Result<Member, Error> {
        self.members.iter().find(|p| p.id == id).ok_or(Error::NotFound).cloned()
    }

    pub fn replace_value_by_id(&mut self, id: usize, new_value: Value) -> Result<(), Error> {
        if let Some(s) = self.members.iter_mut().find(|p| p.id == id) {
            s.value = new_value;
            return Ok(())
        } else {
            return Err(Error::NotFound)
        }
    }

    pub fn get_schema(&self) -> Result<ComplexSchema, Error> {
        Ok(SchemaService::get_schema_by_id(self.schema_id)?.lock().unwrap_or_else(|poison| poison.into_inner()).clone())
    }
}
