
use crate::anyhow::ComplexSchema;
use crate::anyhow::ValueKind;
use crate::anyhow::verifier;

use super::SchemaService;
use super::ComplexService;

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

#[derive(Clone, Debug)]
pub struct NamedMember {
    pub name: String,
    pub value: Value,
    pub index: usize,
}

impl NamedMember {
    pub fn new(name: String, value: Value, index: usize) -> Self {
        Self { name, value, index }
    }
}

impl From<&NamedMember> for ValueKind {
    fn from(value: &NamedMember) -> Self {
        ValueKind::from(&value.value)
    }
}

#[derive(Clone, Debug)]
pub struct NamedMemberInterface {
    pub schema_id: usize,
    pub members: Vec<NamedMember>
}

impl NamedMemberInterface {
    pub fn new(schema: ComplexSchema, complex: Complex) -> Result<NamedMemberInterface, Error> {
        verifier::Service::verify_schema(&schema, &complex.members)?;

        let schema_id = schema.id;
        let mut named_members = Vec::new();

        for item in schema.schema.iter() {
            let id = item.id;
            let name = item.name.clone();
            let member = complex.members.iter().find(|p| p.id == id).expect("could not find value as expected regardless of schema verification").clone();
            named_members.push(NamedMember::new(name, member.value.clone(), member.id));
        }

        Ok(
                NamedMemberInterface {
                    schema_id,
                    members: named_members
                }
        )
    }
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

    pub fn get_members(&self) -> Result<Vec<Member>, Error> {
        Ok(ComplexService::get_complex_by_id(self.id)?.lock().unwrap_or_else(|poison| poison.into_inner()).members.clone())
    }

    pub fn get_named_members(&self) -> Result<NamedMemberInterface, Error> {
        let schema = SchemaService::get_schema_by_id(self.schema_id)?.lock().unwrap_or_else(|poison| poison.into_inner()).clone();
        let complex = ComplexService::get_complex_by_id(self.id)?.lock().unwrap_or_else(|poison| poison.into_inner()).clone();

        Ok(NamedMemberInterface::new(schema, complex)?)
    }
}
