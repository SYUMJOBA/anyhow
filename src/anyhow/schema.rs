use std::sync::{Arc, Mutex, OnceLock};

#[derive(Clone, Debug)]
pub enum ValueSchema {
    Integer(String),
    Float(String),
    String(String),
    Complex(String, usize) // schema id
}

#[derive(Clone, Debug)]
pub struct ComplexSchema {
    pub(super) id: usize,
    pub(super) name: String,
    pub(super) schema: Vec<ValueSchema>
}

impl ComplexSchema {
    pub fn new(name: String, schema_members: Vec<ValueSchema>) -> Self {
        ComplexSchema { id: get_next_id(), name, schema: schema_members }
    }
}

static IDPROVIDER: OnceLock<Mutex<usize>> = OnceLock::new();
fn get_next_id() -> usize {
    let mut m = IDPROVIDER.get_or_init(|| Mutex::new(0)).lock().unwrap_or_else(|poison| poison.into_inner());
    *m += 1;
    *m
}

static BUFFER: OnceLock<Mutex<Vec<Arc<Mutex<ComplexSchema>>>>> = OnceLock::new();

