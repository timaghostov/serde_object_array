use serde_object_array::SerializeToArray;
use serde_object_array::DeserializeFromArray;

pub type Size = i64;
pub type ClientId = u32;

#[derive(Debug, Clone, Eq, PartialEq, SerializeToArray, DeserializeFromArray)]
pub struct Position {
    pub currency_name: String,
    pub value: Size,
    pub counterparty_id: ClientId,
}

fn main() {
    let json = r#"[ 
        "BTC",
        10000000,
        2
    ]"#;
    let position1: Position = serde_json::from_str(json).expect("Failed from_str");
    println!("test_json_array_position position1 :: {:?}", position1);

    let json = serde_json::to_string(&position1).expect("Failed to_string");
    println!("test_json_array_position json :: {:?}", json);

    let position2: Position = serde_json::from_str(&json).expect("Failed from_str");
    println!("test_json_array_position position2 :: {:?}", position2);

    assert_eq!(position1, position2);
}