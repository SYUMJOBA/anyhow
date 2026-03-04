
use crate::anyhow::valuekind::ValueKind;

use super::*;

pub struct Service;

impl Service {
    fn verify_schema(schema: &ComplexSchema, complex: &Complex) -> Result<(), ComlpexProcessError> {
        if schema.schema.len() != complex.members.len() {
            return Err(ComlpexProcessError::InvalidFormat)
        }

        for i in 0..schema.schema.len() {
            Self::is_same_kind(&schema.schema[i], &complex.members[i])?;
        }

        Ok(())
    }

    fn is_same_kind(schema: &ValueSchema, value: &Value) -> Result<(), ComlpexProcessError> {
        if ValueKind::from(schema) == ValueKind::from(value) {
            return Ok(())
        }
        Err(ComlpexProcessError::InvalidFormat)
    }
}