
use crate::anyhow::valuekind::ValueKind;

use super::*;

pub struct Service;

impl Service {
    pub fn verify_schema(schema: &ComplexSchema, members: &Vec<Member>) -> Result<(), ComplexProcessError> {
            if schema.schema.len() != members.len() {
                return Err(ComplexProcessError::InvalidFormat)
            }

            let mut checked_ids = Vec::new();

            for item in schema.schema.iter() {
                let id = item.id;
                let m = members.iter().find(|m| m.id == id).ok_or(ComplexProcessError::InvalidFormat)?;
                Self::is_same_kind(&item, &m.value)?;
                checked_ids.push(id);
            }

            if checked_ids.len() == schema.schema.len() {
                return Ok(())
            }

            Err(ComplexProcessError::InvalidFormat)
        }

    pub fn is_same_kind(schema: &ValueSchema, value: &Value) -> Result<(), ComplexProcessError> {
        if ValueKind::is_semantically_coherent(value, schema) {
            return Ok(())
        }
        Err(ComplexProcessError::InvalidFormat)
    }
}
