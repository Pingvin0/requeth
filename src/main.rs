use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TestStruct {
    name: String,
    age: i32
}

fn main() {
    println!("Hello, world!");

    let st = TestStruct {
        name: "crack".to_string(),
        age: 48
    };
    let serialized = serde_json::to_string(&st).unwrap();

    println!("{}", serialized);
}
