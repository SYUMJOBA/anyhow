
use crate::anyhow::valuekind::ValueKind;

use super::*;

pub struct Service;

impl Service {
    fn verify_schema(schema: &ComplexSchema, complex: &Complex) -> Result<(), ComplexProcessError> {
        if schema.schema.len() != complex.members.len() {
            return Err(ComplexProcessError::InvalidFormat)
        }

        for i in 0..schema.schema.len() {
            Self::is_same_kind(&schema.schema[i], &complex.members[i])?;
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