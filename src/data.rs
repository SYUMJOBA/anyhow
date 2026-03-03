use std::{sync::{Mutex, MutexGuard, OnceLock}};

#[derive(Clone, Debug)]
pub enum ValueSchema {
    Integer(String),
    Float(String),
    String(String),
    Complex(String, usize) // schema id
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueKind {
    Integer,
    Float,
    String,
    Complex
}

impl From<ValueSchema> for ValueKind {
    fn from(value: ValueSchema) -> Self {
        match value {
            ValueSchema::Integer(_) => ValueKind::Integer,
            ValueSchema::Float(_) => ValueKind::Float,
            ValueSchema::String(_) => ValueKind::String,
            ValueSchema::Complex(_, _) => ValueKind::Complex,
        }
    }
}

impl From<Value> for ValueKind {
    fn from(value: Value) -> Self {
        match value {
            Value::Integer(_) => ValueKind::Integer,
            Value::Float(_) => ValueKind::Float,
            Value::String(_) => ValueKind::String,
            Value::Complex(_) => ValueKind::Complex,
        }
    }
}

impl ValueSchema {
    pub fn is_same_type(&self, value: &Value) -> bool {
        ValueKind::from(self.clone()) == ValueKind::from(value.clone())
    }
}

static SCHEMA_NEXTID_PROVIDER: OnceLock<Mutex<usize>> = OnceLock::new();
fn get_next_schemaid() -> usize {
    let mut m = SCHEMA_NEXTID_PROVIDER.get_or_init(|| Mutex::new(0)).lock().unwrap_or_else(|posion| posion.into_inner());
    *m += 1;
    *m
}

#[derive(Clone, Debug)]
pub struct ComplexSchema {
    id: usize,
    name: String,
    schema: Vec<ValueSchema>
}

impl ComplexSchema {
    pub fn new(name: String, schema_members: Vec<ValueSchema>) -> Self {
        ComplexSchema { id: get_next_schemaid(), name, schema: schema_members }
    }

    pub fn is_valid_schema(&self, complex: &Complex) -> bool {
        if self.schema.len() != complex.members.len() {
            return false;
        }

        for i in 0..self.schema.len() {
            if !self.schema[i].is_same_type(&complex.members[i]) {
                return false
            }
        }

        true
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
    Complex(usize) // table id
}

static NEXT_COMLPEXID_PROVIDER: OnceLock<Mutex<usize>> = OnceLock::new();
fn get_next_complexid() -> usize {
    let mut m = NEXT_COMLPEXID_PROVIDER.get_or_init(|| Mutex::new(0)).lock().unwrap_or_else(|poison| poison.into_inner());
    *m += 1;
    *m
}

#[derive(Clone, Debug)]
pub struct Complex {
    id: usize,
    schema_id: usize,
    members: Vec<Value>
}

impl Complex {
    pub fn new(schema_id: usize, members: Vec<Value>) -> Self {
        Complex { id: get_next_complexid(), schema_id, members }
    }
}

static SCHEMAS: OnceLock<Mutex<Vec<ComplexSchema>>> = OnceLock::new();
fn get_schemas() -> MutexGuard<'static, Vec<ComplexSchema>> {
    SCHEMAS.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap_or_else(|posion| posion.into_inner())
}

static COMPLEXES: OnceLock<Mutex<Vec<Complex>>> = OnceLock::new();
fn get_complexes() -> MutexGuard<'static, Vec<Complex>> {
    COMPLEXES.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap_or_else(|posion| posion.into_inner())
}

pub struct DataService;

#[derive(Clone, Debug)]
pub enum ComplexProcessError {
    InvalidIndex,
    InvalidFormat,
    NotFound
}

impl DataService {
    fn get_schema(id: usize) -> Result<ComplexSchema, ComplexProcessError> {
        get_schemas().iter().find(|s| s.id == id).ok_or(ComplexProcessError::InvalidIndex).cloned()
    }

    fn check_format(complex: &Complex, schema: &ComplexSchema) -> Result<(), ComplexProcessError> {
        match schema.is_valid_schema(complex) {
            true => Ok(()),
            false => Err(ComplexProcessError::InvalidFormat)
        }
    }

    fn add_complex(complex: Complex) -> usize {
        let mut c = get_complexes();
        let id = complex.id;
        c.push(complex);
        id
    }

    fn add_schema(complex_schema: ComplexSchema) -> usize {
        let mut s = get_schemas();
        let id = complex_schema.id;
        s.push(complex_schema);
        id
    }

    pub fn get_schema_id_by_name(name: String) -> Result<usize, ComplexProcessError> {
        get_schemas().iter().position(|s| s.name == name).ok_or(ComplexProcessError::NotFound)
    }

    pub fn get_schema_members(id: usize) -> Result<Vec<ValueSchema>, ComplexProcessError> {
        Ok(Self::get_schema(id)?.schema.clone())
    }

    pub fn get_complex_by_id(id: usize) -> Result<Complex, ComplexProcessError> {
        get_complexes().iter().find(|c| c.id == id).ok_or(ComplexProcessError::NotFound).cloned()
    }

    pub fn create_complex(complex: Complex) -> Result<usize, ComplexProcessError> {
        let schema = Self::get_schema(complex.schema_id)?;

        Self::check_format(&complex, &schema)?;

        Ok(Self::add_complex(complex))
    }

    pub fn create_schema(schema: ComplexSchema) -> Result<usize, ComplexProcessError> {
        if Self::get_schema_id_by_name(schema.name.clone()).is_ok() {
            return Err(ComplexProcessError::NotFound)
        }

        Ok(Self::add_schema(schema))
    }

    pub fn dump_schemas() -> Vec<ComplexSchema> {
        get_schemas().clone()
    }

    pub fn dump_complexes() -> Vec<Complex> {
        get_complexes().clone()
    }
}