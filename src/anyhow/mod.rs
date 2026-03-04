mod complex;
mod schema;
mod service;
mod valuekind;
mod verifier;
mod error;

pub use complex::Complex as Complex;
pub use complex::Member as Member;
pub use schema::ComplexSchema as ComplexSchema;
pub use complex::Value as Value;
pub use schema::ValueSchema as ValueSchema;
pub use error::Error as ComplexProcessError;
use valuekind::ValueKind as ValueKind;

use complex::Service as ComplexService;
use schema::Service as SchemaService;

struct DataServiceHelper;

impl DataServiceHelper {
    // helpers, specifically intended for syntax sugar and code readability
}

pub struct DataService;

impl DataService {
    // verifies that the name isn't already taken and adds the schema
    pub fn create_schema(name: String, members: Vec<ValueSchema>) -> Result<usize, ComplexProcessError> {
        if let Ok(schema) = SchemaService::get_schema_id_by_name(name.clone()) {
            return Err(ComplexProcessError::AlreadyPresent)
        }

        SchemaService::create_schema(name, members)
    }

    pub fn list_schemas() -> Vec<ComplexSchema> {
        todo!()
    }

    pub fn get_schema(id: usize) -> Result<ComplexSchema, ComplexProcessError> {
        Ok(SchemaService::get_schema_by_id(id)?.lock().unwrap_or_else(|p| p.into_inner()).clone())
    }

    pub fn delete_schema(id: usize) -> Result<(), ComplexProcessError> {
        // delete all child schemas
        let schemas = ComplexService::get_complexes_by_schema_id(id)?;

        for s in schemas.iter() {
            let id = s.lock().unwrap_or_else(|p| p.into_inner()).id;
            ComplexService::remove_complex(id)?;
        }

        SchemaService::delete_schema(id)?;

        Ok(())
    }

    pub fn add_member_to_schema(schema_id: usize, new_member: ValueSchema) -> Result<(), ComplexProcessError> {
        todo!()
    }

    pub fn remove_member_from_schema(schema_id: usize, member_index: usize) -> Result<(), ComplexProcessError> {
        todo!()
    }

    pub fn add_complex(schema_id: usize, members: Vec<Value>) -> Result<usize, ComplexProcessError> {
        todo!()
    }

    pub fn update_complex_member(complex_id: usize, member_index: usize, new_value: Value) -> Result<(), ComplexProcessError> {
        todo!()
    }

    pub fn delete_complex(complex_id: usize) -> Result<(), ComplexProcessError> {
        todo!()
    }

    pub fn list_complexes_by_schema(schema_id: usize) -> Result<Vec<Complex>, ComplexProcessError> {
        todo!()
    }

    pub fn list_complexes() -> Vec<Complex> {
        todo!()
    }
}