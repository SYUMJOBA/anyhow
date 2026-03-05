
use super::Member;
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
    pub fn add_complex(schema_id: usize, members: Vec<Member>) -> usize {
        let id = get_next_id();
        get_buffer().push(
            Arc::new(Mutex::new(Complex { id, schema_id: schema_id, members }))
        );
        id
    }

    pub fn remove_complex(id: usize) -> Result<(), Error> {
        let mut b = get_buffer();

        let p = b.iter().position(|c| c.lock().unwrap_or_else(|p| p.into_inner()).id == id);

        if let Some(i) = p {
            b.remove(i);
            return Ok(())
        } else {
            return Err(Error::NotFound)
        }
    }

    pub fn replace_value(id: usize, value_id: usize, new_value: Value) -> Result<(), Error> {
        with_complex(id, move |complex| {
            if complex.members.len() >= value_id {
                return Err(Error::InvalidIndex)
            }

            complex.replace_value_by_id(value_id, new_value)?;

            Ok(())
        })?
    }

    pub fn get_complex_by_id(id: usize) -> Result<Arc<Mutex<Complex>>, Error> {
        get_buffer().iter().find(|c| c.lock().unwrap_or_else(|posion| posion.into_inner()).id == id).ok_or(Error::NotFound).cloned()
    }

    pub fn get_schema_id_by_complex_id(complex_id: usize) -> Result<usize, Error> {
        with_complex(complex_id, |complex| {
            complex.schema_id
        })
    }

    pub fn get_complexes_by_schema_id(schema_id: usize) -> Result<Vec<Arc<Mutex<Complex>>>, Error> {
        Ok(get_buffer().iter().filter(|p| p.lock().unwrap_or_else(|p| p.into_inner()).schema_id == schema_id).cloned().collect())
    }

    pub fn remove_member_from_complex(complex_id: usize, member_id: usize) -> Result<(), Error> {
        with_complex(complex_id, |complex| {
            let p = match complex.members.iter().position(|p| p.id == member_id) {
                Some(p) => p,
                None => return Err(Error::NotFound)
            };
            complex.members.remove(p);

            Ok(())
        })?
    }

    pub fn list() -> Vec<Complex> {
        get_buffer().iter().map(|p| p.lock().unwrap_or_else(|p| p.into_inner()).clone()).collect()
    }
}
