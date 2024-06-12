use std::fs;
use serde_json::{json, Value};

fn main() -> std::io::Result<()> {
    let mut combined_data: Vec<Value> = Vec::new();

    let file_paths = vec![
        "data/voter_turnout_data/polled_evm_turnout_data/phase1.json",
        "data/voter_turnout_data/polled_evm_turnout_data/phase2.json",
        "data/voter_turnout_data/polled_evm_turnout_data/phase3.json",
        "data/voter_turnout_data/polled_evm_turnout_data/phase4.json",
        "data/voter_turnout_data/polled_evm_turnout_data/phase5.json",
        "data/voter_turnout_data/polled_evm_turnout_data/phase6.json",
        "data/voter_turnout_data/polled_evm_turnout_data/phase7.json"
    ];

    for path in file_paths {
        let json_data = fs::read_to_string(path)
           .expect("Unable to read file");
        let file_data: Value = serde_json::from_str(&json_data).unwrap();
        let array = file_data.as_array().unwrap();
        combined_data.extend(array.clone());
    }

    let combined_json = json!(combined_data);
    // println!("{}", combined_json);

    let serialized_data = serde_json::to_string_pretty(&combined_json).unwrap();
    let write_path = "data/voter_turnout_data/polled_evm_turnout_data/all_phases.json";
    // let write_path = "test_data/voter_turnout.json";

    // Save the serialized data to a new JSON file
    fs::write(write_path, serialized_data).expect("Unable to write file");


    Ok(())
}