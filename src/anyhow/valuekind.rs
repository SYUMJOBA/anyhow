use super::complex::Value;
use super::schema::ValueSchema;
use super::ComplexService;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueKind {
    Integer,
    Float,
    String,
    Complex(usize),
    AnyComplex(usize),
    SelectComplex(Vec<usize>),
    NullableInteger,
    NullableFloat,
    NullableString,
    NullableComplex(usize),
    NullableAnyComplex(Option<usize>),
    NullableSelectComplex(Vec<usize>),
}

impl ValueKind {
    pub(super) fn is_nullable(&self) -> bool {
        match self {
            ValueKind::Integer => false,
            ValueKind::Float => false,
            ValueKind::String => false,
            ValueKind::Complex(_) => false,
            ValueKind::AnyComplex(_) => false,
            ValueKind::SelectComplex(_) => false,
            ValueKind::NullableInteger => true,
            ValueKind::NullableFloat => true,
            ValueKind::NullableString => true,
            ValueKind::NullableComplex(_) => true,
            ValueKind::NullableAnyComplex(_) => true,
            ValueKind::NullableSelectComplex(_) => true,
        }
    }

    pub fn is_semantically_coherent(value: &Value, schema: &ValueSchema) -> bool {
        schema.kind == match value {

            Value::Integer(_) => ValueKind::Integer,

            Value::Float(_) => ValueKind::Float,

            Value::String(_) => ValueKind::String,

            Value::Complex(foreign_complex_pointer) => ValueKind::Complex(foreign_complex_pointer.schema_id),

            Value::AnyComplex(_foreign_complex_pointer) => if let ValueKind::AnyComplex(_) = schema.kind { return true } else { return false },

            Value::SelectComplex(foreign_select_complex_pointer) => ValueKind::SelectComplex(foreign_select_complex_pointer.accepted_schemas.clone()),

            Value::NullableInteger(_) => ValueKind::NullableInteger,

            Value::NullableFloat(_) => Self::NullableFloat,

            Value::NullableString(_) => Self::NullableString,

            Value::NullableComplex(foreign_optional_complex_pointer) => Self::NullableComplex(foreign_optional_complex_pointer.schema_id),

            Value::NullableAnyComplex(_foreign_optional_any_complex_pointer) => if let ValueKind::AnyComplex(_) = schema.kind { return true } else { return false },

            Value::NullableSelectComplex(foreign_optional_select_complex_pointer) => ValueKind::NullableSelectComplex(foreign_optional_select_complex_pointer.accepted_schemas.clone()),

        }
    }
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
            Value::Complex(pointer) => Self::Complex(pointer.schema_id),
            Value::AnyComplex(pointer) => Self::AnyComplex(pointer.schema_id),
            Value::SelectComplex(pointer) => Self::SelectComplex(pointer.accepted_schemas.clone()),
            Value::NullableInteger(_) => Self::NullableInteger,
            Value::NullableFloat(_) => Self::NullableFloat,
            Value::NullableString(_) => Self::NullableString,
            Value::NullableComplex(pointer) => Self::NullableComplex(pointer.schema_id),
            Value::NullableSelectComplex(pointer) => Self::NullableSelectComplex(pointer.accepted_schemas.clone()),
            Value::NullableAnyComplex(pointer) => Self::NullableAnyComplex(pointer.schema_id),
            Value::NullableSelectComplex(pointer) => Self::NullableSelectComplex(pointer.accepted_schemas.clone())
        }
    }
}
