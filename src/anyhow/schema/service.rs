use super::ComplexSchema;
use super::ValueSchema;

use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;
use std::sync::Arc;

pub struct Service;

static IDPROVIDER: OnceLock<Mutex<usize>> = OnceLock::new();
fn get_next_id() -> usize {
    let mut m = IDPROVIDER.get_or_init(|| Mutex::new(0)).lock().unwrap_or_else(|poison| poison.into_inner());
    *m += 1;
    *m
}

static BUFFER: OnceLock<Mutex<Vec<Arc<Mutex<ComplexSchema>>>>> = OnceLock::new();

