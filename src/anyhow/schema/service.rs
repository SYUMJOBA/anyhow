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
    pub fn create_schema(name: String, members: Vec<ValueSchema>) -> Result<usize, Error> {
        if let Ok(_) = Self::get_schema_id_by_name(name.clone()) {
            return Err(Error::AlreadyPresent)
        }

        let id = get_next_id();

        get_buffer().push(Arc::new(Mutex::new(ComplexSchema {
            id,
            name,
            schema: members
        })));

        Ok(id)
    }

    pub fn add_member_to_schema(id: usize, new_member: ValueSchema) -> Result<(), Error> {
        with_schema(id, |schema| {
            if let Some(_) = schema.schema.iter().find(|v| v.name == new_member.name) {
                return Err(Error::InvalidIndex)
            }

            schema.schema.push(new_member);

            Ok(())
        })?
    }

    pub fn remove_member_from_schema(id: usize, member_id: usize) -> Result<(), Error> {
        with_schema(id, |schema| {
            if id >= schema.schema.len() {
                return Err(Error::InvalidIndex)
            }

            schema.schema.remove(member_id);

            Ok(())
        })?
    }

    pub fn get_member_id_from_schema_by_name(schema_id: usize, member_name: String) -> Result<usize, Error> {
        with_schema(schema_id, |schema| {
            if let Some(p) = schema.schema.iter().position(|v| v.name == member_name.clone()) {
                return Ok(p)
            }

            return Err(Error::NotFound)
        })?
    }

    pub fn get_schema_by_id(id: usize) -> Result<Arc<Mutex<ComplexSchema>>, Error> {
        get_buffer().iter().find(|v| v.lock().unwrap_or_else(|posion| posion.into_inner()).id == id).ok_or(Error::NotFound).cloned()
    }

    pub fn delete_schema(id: usize) -> Result<(), Error> {
        let mut m = get_buffer();

        if let Some(p) = m.iter().position(|schema| schema.lock().unwrap_or_else(|poison| poison.into_inner()).id == id) {
            m.remove(p);
        } else {
            return Err(Error::NotFound)
        }

        Ok(())
    }

    pub fn get_schema_id_by_name(name: String) -> Result<usize, Error> {
        match get_buffer().iter().find(|v| v.lock().unwrap_or_else(|posion| posion.into_inner()).name == name.clone()) {
            Some(s) => Ok(s.lock().unwrap_or_else(|pos| pos.into_inner()).id),
            None => Err(Error::NotFound),
        }
    }

    pub fn list() -> Vec<ComplexSchema> {
        get_buffer().iter().map(|p| p.lock().unwrap_or_else(|p| p.into_inner()).clone()).collect()
    }
}
