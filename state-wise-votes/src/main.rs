use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

type PartyVotes = BTreeMap<String, i32>;
type StateVotes = BTreeMap<String, PartyVotes>;

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
        println!("{}", state_name);

        // Create a new HashMap to store the party votes for this state
        let mut party_votes: PartyVotes = BTreeMap::new();

        // Iterate over each constituency in the state
        for party_data in state["party_state_data"].as_array().unwrap() {
            for constituency in party_data["constituencies"].as_array().unwrap() {
                // Iterate over each candidate in the constituency
                for candidate in constituency["candidates_info"].as_array().unwrap() {
                    let party = candidate["party"].as_str().unwrap().to_string();
                    let votes = candidate["votes"].as_str().unwrap().parse::<i32>().unwrap();

                    // Update the party votes for this state
                    *party_votes.entry(party).or_insert(0) += votes;
                }
            }
        }

        // Update the state votes with the party votes for this state
        *state_votes.entry(state_name).or_insert(PartyVotes::new()) = party_votes;
    }

    // Serialize the state_votes HashMap back to JSON
    let serialized_data = serde_json::to_string_pretty(&state_votes).unwrap();
    let write_path = "data/state_wise_votes.json";
    // let write_path = "test_data/state_votes.json";

    // Save the serialized data to a new JSON file
    fs::write(write_path, serialized_data).expect("Unable to write file");

    println!("Total votes by party in each state saved to state_votes.json");
}
