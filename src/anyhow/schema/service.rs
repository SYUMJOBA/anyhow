use super::ComplexSchema;
use super::ValueSchema;
use super::super::error::Error;

use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;
use std::sync::Arc;


static IDPROVIDER: OnceLock<Mutex<usize>> = OnceLock::new();
fn get_next_id() -> usize {
    let mut m = IDPROVIDER.get_or_init(|| Mutex::new(0)).lock().unwrap_or_else(|poison| poison.into_inner());
    *m += 1;
    *m
}

static BUFFER: OnceLock<Mutex<Vec<Arc<Mutex<ComplexSchema>>>>> = OnceLock::new();
fn get_buffer() -> MutexGuard<'static, Vec<Arc<Mutex<ComplexSchema>>>> {
    BUFFER.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap_or_else(|posion| posion.into_inner())
}

fn with_schema<F, R>(id: usize, f: F) -> Result<R, Error> where F: FnOnce(&mut ComplexSchema) -> R {
    let a = Service::get_schema_by_id(id)?;
    let mut m = a.lock().unwrap_or_else(|posion| posion.into_inner());
    Ok(f(&mut m))
}

pub struct Service;

impl Service {
    pub fn create_schema(members: Vec<ValueSchema>) -> usize {
        todo!()
    }

    pub fn add_member_to_schema(id: usize, new_member: ValueSchema) -> Result<(), Error> {
        todo!()
    }

    pub fn remove_member_from_schema(id: usize, member_id: usize) -> Result<(), Error> {
        todo!()
    }

    pub fn get_member_id_from_schema_by_name(schema_id: usize, member_name: String) -> Result<(), Error> {
        todo!()
    }

    pub fn get_schema_by_id(id: usize) -> Result<Arc<Mutex<ComplexSchema>>, Error> {
        todo!()
    }

    pub fn delete_schema(id: usize) -> Result<(), Error> {
        todo!()
    }

    pub fn get_schema_id_by_name(name: String) -> Result<(), Error> {
        todo!()
    }
}