mod complex;
mod schema;

mod valuekind;
mod verifier;
mod error;

pub use complex::Complex as Complex;
pub use complex::Member as Member;
pub use schema::ComplexSchema as ComplexSchema;
pub use complex::Value as Value;
pub use schema::ValueSchema as ValueSchema;
pub use error::Error as ComplexProcessError;
pub use valuekind::ValueKind as ValueKind;
use verifier::Service as Verifier;

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
        SchemaService::list()
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

    pub fn add_member_to_schema(schema_id: usize, new_member: ValueSchema, filler_value: Option<Value>) -> Result<(), ComplexProcessError> {
        let schema = SchemaService::get_schema_by_id(schema_id)?;
        let mut schema = schema.lock().unwrap_or_else(|poison| poison.into_inner());


        if let Ok(_) = schema.get_member_id_by_name(new_member.name.clone()) {
            return Err(ComplexProcessError::AlreadyPresent)
        }

        if let Ok(_) = schema.get_member_by_id(new_member.id) {
            return Err(ComplexProcessError::InvalidIndex)
        }

        if !new_member.kind.is_nullable() && new_member.default.is_none() && filler_value.is_none() {
            return Err(ComplexProcessError::IllegalRepresentation)
        } else if let Some(v) = filler_value.clone() {
            if ! ValueKind::is_semantically_coherent(&v, &new_member) {
                return Err(ComplexProcessError::InvalidFormat)
            }
        } else if let Some(v) = new_member.default.clone() {
            if ! ValueKind::is_semantically_coherent(&v, &new_member) {
                return Err(ComplexProcessError::InvalidFormat)
            }
        }

        let related_complexes = ComplexService::get_complexes_by_schema_id(schema_id)?;
        for c in related_complexes.iter() {
            let mut c = c.lock().unwrap_or_else(|p| p.into_inner());

            let v = if let Some(val) = filler_value.clone() {
                val
            } else if let Some(val) = new_member.default.clone() {
                val
            } else if new_member.kind.is_nullable() {
                unreachable!("a nullable value had no filler and no default, but the above type checks should have caught this!")
            } else {
                panic!("a non nullable value was about to be initialized nullable and the above type checks didn't work!")
            };

            c.members.push(Member { id: new_member.id, value: v });
        }

        schema.schema.push(new_member);

        Ok(())
    }

    pub fn remove_member_from_schema(schema_id: usize, member_id: usize) -> Result<(), ComplexProcessError> {
        let schema = SchemaService::get_schema_by_id(schema_id)?;
        let mut schema = schema.lock().unwrap_or_else(|poison| poison.into_inner());

        schema.get_member_by_id(member_id)?;

        let complexes_id = ComplexService::get_complexes_by_schema_id(schema_id)?.iter().map(|c| c.lock().unwrap_or_else(|poison| poison.into_inner()).id).collect::<Vec<usize>>();

        for id in complexes_id.iter() {
            ComplexService::remove_member_from_complex(id.clone(), member_id)?;
        }

        let p = match schema.schema.iter().position(|p| p.id == member_id) {
            Some(p) => p,
            None => unreachable!("an id was considered valid beforehand yet not now")
        };
        schema.schema.remove(p);


        Ok(())
    }

    pub fn create_complex(schema_id: usize, members: Vec<Member>) -> Result<usize, ComplexProcessError> {
        let schema = SchemaService::get_schema_by_id(schema_id)?;
        let schema = schema.lock().unwrap_or_else(|p| p.into_inner());

        Verifier::verify_schema(&schema, &members)?;

        Ok(ComplexService::add_complex(schema_id, members))
    }

    pub fn get_complex(complex_id: usize) -> Result<Complex, ComplexProcessError> {
        Ok(ComplexService::get_complex_by_id(complex_id)?.lock().unwrap_or_else(|p| p.into_inner()).clone())
    }

    pub fn update_complex_member(complex_id: usize, member_index: usize, new_value: Value) -> Result<(), ComplexProcessError> {
        let complex = ComplexService::get_complex_by_id(complex_id)?;
        let mut complex = complex.lock().unwrap_or_else(|p| p.into_inner());

        let schema = SchemaService::get_schema_by_id(complex.schema_id)?;
        let schema = schema.lock().unwrap_or_else(|p| p.into_inner());

        let member_schema = schema.get_member_by_id(member_index)?;
        Verifier::is_same_kind(&member_schema, &new_value)?;

        if let Some(member) = complex.members.iter_mut().find(|p| p.id == member_index) {
            member.value = new_value;
            return Ok(())
        } else {
            return Err(ComplexProcessError::NotFound)
        }
    }

    pub fn delete_complex(complex_id: usize) -> Result<(), ComplexProcessError> {
        ComplexService::remove_complex(complex_id)
    }

    pub fn list_complexes_by_schema(schema_id: usize) -> Result<Vec<Complex>, ComplexProcessError> {
        Ok(ComplexService::get_complexes_by_schema_id(schema_id)?.iter().map(|v| v.lock().unwrap_or_else(|p| p.into_inner()).clone()).collect())
    }

    pub fn list_complexes() -> Vec<Complex> {
        ComplexService::list()
    }
}
