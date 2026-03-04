use super::Value;
use super::ValueKind;
use super::error::Error;

mod service;

pub(super) use service::Service;

#[derive(Clone, Debug)]
pub struct ValueSchema {
    pub(super) id: usize,
    pub(super) name: String,
    pub(super) kind: ValueKind,
    pub(super) default: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct ComplexSchema {
    pub(super) id: usize,
    pub(super) name: String,
    pub(super) schema: Vec<ValueSchema>,
}

impl ComplexSchema {
    pub(super) fn get_member_id_by_name(&self, name: String) -> Result<usize, Error> {
        match self.schema.iter().find(|p| p.name == name.clone()) {
            Some(d) => Ok(d.id),
            None => Err(Error::NotFound),
        }
    }
}
