// use levenshtein_automata::{Distance, LevenshteinAutomatonBuilder};
use serde_json::{json, Value};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the input file
    let path = "data/voter_turnout_data/evm_voter_turnout.json";
    // let path = "test_data/test_data.json";
    // Load the JSON data from the file
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    // Deserialize the JSON data into our struct
    let election_data: Value = serde_json::from_str(&json_data).unwrap();

    let path_counting_evm = "data/voter_turnout_data/evm_tunout_matching/all_phases.json";

    // Load the JSON data from the file
    let json_data_evm_counting =
        fs::read_to_string(path_counting_evm).expect("Unable to read file");
    // Deserialize the JSON data into our struct
    let evm_counting_data: Value = serde_json::from_str(&json_data_evm_counting).unwrap();

    let mut evm_match = vec![];

    // Iterate over the states
    for (state, constituencies) in election_data.as_object().unwrap() {
        //  println!("State: {}", state);
        // Iterate over the constituencies
        for constituency in constituencies.as_array().unwrap() {
            let constituency_name_str = constituency
                .get("constituency")
                .unwrap()
                .as_str()
                .unwrap()
                .split('(')
                .next()
                .unwrap_or("");
            let constituency_name = constituency_name_str.trim().to_lowercase();
            let evm_turnout_votes = constituency
                .get("evm_turnout_votes")
                .unwrap()
                .as_i64()
                .unwrap();
            //  println!("  Constituency: {}, EVM Turnout Votes: {}", constituency_name, evm_turnout_votes);

            for list in evm_counting_data.as_array().unwrap() {
                let pc_name_string = list["pc_name"].as_str().unwrap();
                let evm_turnout_votes_polled_str = list["count_of_votes"].as_str().unwrap();
                let evm_turnout_votes_polled: i64 = evm_turnout_votes_polled_str.parse().expect("Failed ot parse");
                let pc_name = pc_name_string.trim().to_lowercase();
                if pc_name == constituency_name {
                    // println!("{}", constituency_name);

                    let evm_value = json!({
                        "state": state,
                        "constituency_name": constituency_name_str,
                        "evm_turnout_votes": evm_turnout_votes,
                        "evm_turnout_votes_polled": evm_turnout_votes_polled,
                        "difference": evm_turnout_votes - evm_turnout_votes_polled,
                    });

                    evm_match.push(evm_value);
                }
            }
        }
    }

    // Serialize the state_votes HashMap back to JSON
    let serialized_data = serde_json::to_string_pretty(&evm_match).unwrap();
    let write_path = "data/voter_turnout_data/evm_tunout_matching/evm_voter_turnout_comparison.json";
    // let write_path = "test_data/voter_turnout.json";

    // Save the serialized data to a new JSON file
    fs::write(write_path, serialized_data).expect("Unable to write file");

    Ok(())
}
