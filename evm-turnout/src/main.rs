use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

type StateVotes = BTreeMap<String, Vec<Value>>;

fn main() {
    let path = "data/2024_loksabha_election_evm_postal_data.json";
    // let path = "test_data/test_data.json";
    // Load the JSON data from the file
    let json_data = fs::read_to_string(path)
        .expect("Unable to read file");
    // Deserialize the JSON data into our struct
    let election_data: Value = serde_json::from_str(&json_data).unwrap();
    // Create a new HashMap to store the total votes by party for each state
    let mut state_votes: StateVotes = BTreeMap::new();

    // Iterate over each state in the election data
    for state in election_data.as_array().unwrap() {
        let state_name = state["state"].as_str().unwrap().to_string();
        // println!("{}", state_name);


        let mut constituency_data: Vec<Value> = Vec::new();

         // Iterate over each constituency in the state
    for party_data in state["party_state_data"].as_array().unwrap() {
        for constituency in party_data["constituencies"].as_array().unwrap() {
            // Iterate over each candidate in the constituency
            let mut turnout_votes = 0;
            for candidate in constituency["candidates_info"].as_array().unwrap() {
                let candidate_votes;
                let votes_value =  candidate["EVM Votes"].as_str().unwrap();
                // println!("votes_value= {}", votes_value);
                if votes_value == "-" {
                    candidate_votes = 0;
                } else {
                    candidate_votes = candidate["EVM Votes"].as_str().unwrap().parse::<i32>().unwrap();
                }
                turnout_votes += candidate_votes;
            }
            let turnout_data = json!({
                "constituency":constituency["constituency"],
                "evm_turnout_votes": turnout_votes
            });

            constituency_data.push(turnout_data);
           
        }
    }

        // Update the state votes with the party votes for this state
        state_votes.insert(state_name, constituency_data);
    }

    // Serialize the state_votes HashMap back to JSON
    let serialized_data = serde_json::to_string_pretty(&state_votes).unwrap();
    let write_path = "data/voter_turnout_data/evm_voter_turnout.json";
    // let write_path = "test_data/voter_turnout.json";

    // Save the serialized data to a new JSON file
    fs::write(write_path, serialized_data).expect("Unable to write file");

    println!("Total votes by party in each state saved to state_votes.json");
}
