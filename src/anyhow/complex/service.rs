use super::Complex;
use super::Value;
use super::super::error::Error;

use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;
use std::sync::Arc;

static IDPROVIDER: OnceLock<Mutex<usize>> = OnceLock::new();
fn get_next_id() -> usize {
    let mut m = IDPROVIDER.get_or_init(|| Mutex::new(0)).lock().unwrap_or_else(|posion| posion.into_inner());
    *m += 1;
    *m
}

static BUFFER: OnceLock<Mutex<Vec<Arc<Mutex<Complex>>>>> = OnceLock::new();
fn get_buffer() -> MutexGuard<'static, Vec<Arc<Mutex<Complex>>>> {
    BUFFER.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap_or_else(|poison| poison.into_inner())
}

fn with_complex<F, R>(id: usize, f: F) -> Result<R, Error> where F: FnOnce(&mut Complex) -> R {
    let c = Service::get_complex_by_id(id)?.clone();
    let mut c = c.lock().unwrap_or_else(|posion| posion.into_inner());
    Ok(f(&mut *c))
}

pub struct Service;

impl Service {

}