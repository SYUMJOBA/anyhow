mod anyhow;

use anyhow::*;

fn main() {

    let schema_id = DataService::create_schema(
        "certificate".to_string(),
        vec![
            ValueSchema::new(0, "gem_code".to_string(), ValueKind::Integer, None),
            ValueSchema::new(1, "certificate_cost".to_string(), ValueKind::Float, None),
            ValueSchema::new(2, "gem_id".to_string(), ValueKind::String, None),
        ]
    ).expect("could not add schema, why?");

    println!("{:?}", DataService::get_schema(schema_id));
    println!("{:?}", DataService::list_schemas());
    println!("{:?}", schema_id);

    let complex_id = DataService::create_complex(schema_id, vec![
        Member::new(0, Value::Integer(1234)),
        Member::new(1, Value::Float(4567.60)),
        Member::new(2, Value::String("8TxVnF9GGQfXuV4SpNoAgd".to_string()))
    ]).expect("could not add complex, why so?");

    println!("{}", complex_id);
    println!("{:?}", DataService::get_complex(complex_id).expect("could not fetch complex").get_schema().expect("could not get schema"));

    let members = DataService::get_complex(complex_id).expect("could not get complex").get_members();
}
