use super::complex::Value;
use super::schema::ValueSchema;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueKind {
    Integer,
    Float,
    String,
    Complex
}

impl From<&ValueSchema> for ValueKind {
    fn from(value: &ValueSchema) -> Self {
        match value {
            ValueSchema::Integer(_) => ValueKind::Integer,
            ValueSchema::Float(_) => ValueKind::Float,
            ValueSchema::String(_) => ValueKind::String,
            ValueSchema::Complex(_, _) => ValueKind::Complex,
        }
    }
}

impl From<&Value> for ValueKind {
    fn from(value: &Value) -> Self {
        match value {
            Value::Integer(_) => ValueKind::Integer,
            Value::Float(_) => ValueKind::Float,
            Value::String(_) => ValueKind::String,
            Value::Complex(_) => ValueKind::Complex,
        }
    }
}

