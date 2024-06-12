use std::fs;
use serde_json::{json, Value};


fn main() -> std::io::Result<()> {
    // Read the input file
    let path = "data/voter_turnout_data/evm_voter_turnout.json";
    // let path = "test_data/test_data.json";
    // Load the JSON data from the file
    let json_data = fs::read_to_string(path)
        .expect("Unable to read file");
    // Deserialize the JSON data into our struct
    let election_data: Value = serde_json::from_str(&json_data).unwrap();

     // Iterate over the states
     for (state, constituencies) in election_data.as_object().unwrap() {
        println!("State: {}", state);
        // Iterate over the constituencies
        for constituency in constituencies.as_array().unwrap() {
            let constituency_name = constituency.get("constituency").unwrap().as_str().unwrap();
            let evm_turnout_votes = constituency.get("evm_turnout_votes").unwrap().as_u64().unwrap();
            println!("  Constituency: {}, EVM Turnout Votes: {}", constituency_name, evm_turnout_votes);
        }
    }
 
   
    // let write_path= "data/voter_turnout_data/counting_evm_tunout_data/all_phases.json";
    // fs::write(write_path, serialized_data).expect("Unable to write file");

    Ok(())
}