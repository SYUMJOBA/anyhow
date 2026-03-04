use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
    Complex(usize) // table id
}

#[derive(Clone, Debug)]
pub struct Complex {
    pub(super) id: usize,
    pub(super) schema_id: usize,
    pub(super) members: Vec<Value>
}

impl Complex {
    pub fn new(schema_id: usize, members: Vec<Value>) -> Self {
        Complex { id: get_next_id(), schema_id, members }
    }
}

static IDPROVIDER: OnceLock<Mutex<usize>> = OnceLock::new();
fn get_next_id() -> usize {
    let mut m = IDPROVIDER.get_or_init(|| Mutex::new(0)).lock().unwrap_or_else(|posion| posion.into_inner());
    *m += 1;
    *m
}

static BUFFER: OnceLock<Mutex<Vec<Mutex<Complex>>>> = OnceLock::new();
fn get_buffer() -> MutexGuard<'static, Vec<Mutex<Complex>>> {
    BUFFER.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap_or_else(|poison| poison.into_inner())
}

