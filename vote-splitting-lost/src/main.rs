use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

type StateVotes = BTreeMap<String, Vec<Value>>;

fn main() {
    let path = "data/2024_loksabha_election_data_total.json";
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
            let mut winner_votes = 0;
            let mut loser_votes = 0;
            let mut winner_party = "";
            let mut winning_candidate = "";
            for candidate in constituency["candidates_info"].as_array().unwrap() {
                if candidate["status"].as_str().unwrap() == "won" {
                    winner_votes = candidate["votes"].as_str().unwrap().parse::<i32>().unwrap();
                    winning_candidate = candidate["candidate_name"].as_str().unwrap();
                    winner_party = candidate["party"].as_str().unwrap();
                } else {
                    loser_votes += candidate["votes"].as_str().unwrap().parse::<i32>().unwrap();
                }
            }
            if loser_votes < winner_votes {
                let votespliting_data = json!({
                    "constituency":constituency["constituency"],
                    "winning_candidate":winning_candidate,
                    "winner_party":winner_party,
                    "winner_votes": winner_votes,
                    "loser_votes": loser_votes,
                });
                constituency_data.push(votespliting_data);

            }
           
        }
    }

        // Update the state votes with the party votes for this state
        state_votes.insert(state_name, constituency_data);
    }

    // Serialize the state_votes HashMap back to JSON
    let serialized_data = serde_json::to_string_pretty(&state_votes).unwrap();
    let write_path = "data/vote_splitting_lost.json";
    // let write_path = "test_data/vote_splitting.json";

    // Save the serialized data to a new JSON file
    fs::write(write_path, serialized_data).expect("Unable to write file");

    println!("Total votes by party in each state saved to state_votes.json");
}
