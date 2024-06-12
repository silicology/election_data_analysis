use levenshtein_automata::{Distance, LevenshteinAutomatonBuilder};
use serde_json::{json, Value};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "phase1".to_owned();
    // Load the two JSON files
    let file1 = std::fs::read_to_string(format!(
        "data/voter_turnout_data/polled_evm_turnout_data/{}.json",
        filename
    ))?;
    let file2 = std::fs::read_to_string("data/voter_turnout_data/evm_voter_turnout.json")?;

    // Parse the JSON files
    let file1_json: Vec<Value> = serde_json::from_str(&file1)?;
    let file2_json: Value = serde_json::from_str(&file2)?;

    let lev_automaton_builder = LevenshteinAutomatonBuilder::new(2, true);

    // Iterate over the first file and find matches in the second file
    let mut output = json!({});
    for pc in file1_json {
        let pc_name = pc["pc_name"].as_str().unwrap().to_owned();
        let state = pc["state"].as_str().unwrap().to_owned();
        let count_of_votes = pc["count_of_votes"].as_str().unwrap().to_owned();

        let mut min_distance = std::usize::MAX as u8;
        let mut best_match = String::new();
        for (state_name, constituencies) in file2_json.as_object().unwrap() {
            if state == *state_name {
                for constituency in constituencies.as_array().unwrap() {
                    let constituency_name = constituency["constituency"]
                     .as_str()
                     .unwrap()
                     .to_owned();
                    let dfa = lev_automaton_builder.build_dfa(&pc_name);
                    let mut state = dfa.initial_state();
                    for &b in constituency_name.as_bytes() {
                        state = dfa.transition(state, b);
                    }
                    let distance = dfa.distance(state);
                    let distance = match distance {
                        Distance::Exact(d) => d,
                        Distance::AtLeast(d) => d,
                    };
                    if distance < min_distance {
                        min_distance = distance;
                        best_match = constituency_name.clone();
                    }
                }
            }
            let constituency = json!({
                "constituency": best_match.clone(),
                "evm_turnout_votes": constituencies[0]["evm_turnout_votes"].clone(),
                "pc_name": pc_name.clone(),
                "count_of_votes": count_of_votes.clone(),
            });
            output[state_name.clone()] = json!(vec![constituency]);
        }
    }
    let json_data = serde_json::to_string_pretty(&output)?;

    let write_path = format!(
        "data/voter_turnout_data/matched_voter_turnout_{}.json",
        filename
    );
    // let write_path = "test_data/vote_splitting.json";

    // Save the serialized data to a new JSON file
    fs::write(write_path, json_data).expect("Unable to write file");

    Ok(())
}
