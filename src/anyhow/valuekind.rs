use super::complex::Value;
use super::schema::ValueSchema;
use super::ComplexService;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueKind {
    Integer,
    Float,
    String,
    Complex(usize)
}

impl From<&ValueSchema> for ValueKind {
    fn from(value: &ValueSchema) -> Self {
        value.kind.clone()
    }
}

impl From<&Value> for ValueKind {
    fn from(value: &Value) -> Self {
        match value {
            Value::Integer(_) => Self::Integer,
            Value::Float(_) => Self::Float,
            Value::String(_) => Self::String,
            Value::Complex(id) => Self::Complex(ComplexService::get_schema_id_by_complex_id(id.clone()).expect(format!("[CRITICAL ERROR] From conversion failed with schema id {}", id).as_str())),
        }
    }
}