mod data;

use data::DataService;

use crate::data::{Complex, ComplexSchema, Value, ValueSchema};

fn main() {

    let first_schema_id = DataService::create_schema(
        ComplexSchema::new(
            "certificate".to_string(), 
        vec![
            ValueSchema::Integer("gem_code".to_string()),
            ValueSchema::Float("certificate_cost".to_string()),
            ValueSchema::String("gem_id".to_string()),
        ])
    ).expect("could not add schema, why??");

    println!("{:?}", DataService::get_schema_id_by_name("certificate".to_string()));
    println!("{:?}", DataService::dump_schemas());
    println!("{:?}", first_schema_id);

    let new_schema_position = DataService::create_complex(Complex::new(first_schema_id, vec![
        Value::Integer(1234),
        Value::Float(4567.60),
        Value::String("ZeQmyMt8jln9nft2/8TxVnF9GGQfXuV4SpNoAgd+a77E".to_string())
    ])).expect("could not add schema");

    println!("{}", new_schema_position);
    println!("{:?}", DataService::get_complex_by_id(new_schema_position));
}
