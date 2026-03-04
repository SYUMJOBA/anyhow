
use crate::anyhow::valuekind::ValueKind;

use super::*;

pub struct Service;

impl Service {
    fn verify_schema(schema: &ComplexSchema, complex: &Complex) -> Result<(), ComplexProcessError> {
        if schema.schema.len() != complex.members.len() {
            return Err(ComplexProcessError::InvalidFormat)
        }

        for item in schema.schema.iter() {
            let id = item.id;
            let m = complex.get_member_by_id(id)?;
            Self::is_same_kind(&item, &m.value)?;
        }

        Ok(())
    }

    fn is_same_kind(schema: &ValueSchema, value: &Value) -> Result<(), ComplexProcessError> {
        if ValueKind::is_semantically_coherent(value, schema) {
            return Ok(())
        }
        Err(ComplexProcessError::InvalidFormat)
    }
}