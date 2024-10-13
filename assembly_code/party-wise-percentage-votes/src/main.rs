use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

fn main() {
    // Step 1: Load the vote data from a JSON file
    let input_path = "data/2024_assembly/analysis/party_wise_votes_jammu_kashmir.json";
    let json_data = fs::read_to_string(input_path).expect("Unable to read file");

    // Parse the input JSON data
    let parties: Value = serde_json::from_str(&json_data).expect("Error parsing JSON");

    let mut total_votes: i64 = 0;
    let mut party_votes: Vec<(String, i64)> = Vec::new();

    // Calculate total votes and push party votes into a vector
    for (party, votes) in parties.as_object().unwrap() {
        total_votes += votes.as_i64().unwrap();
        party_votes.push((party.clone(), votes.as_i64().unwrap()));
    }

    // Sort the vector in descending order by vote count
    party_votes.sort_by(|(_, a), (_, b)| b.cmp(a)); // This ensures descending order

    // Prepare the result data with both vote count and percentage
    let mut state_data: Vec<Value> = Vec::new();
    for (party, votes) in party_votes {
        let percentage = (votes as f64 / total_votes as f64) * 100.0;
        let party_data = json!({
            "party": party,
            "votes": votes,
            "percentage": percentage,
        });
        state_data.push(party_data);
    }

    // Prepare the final result to save
    let result = json!({
        "state_data": state_data
    });

    // Save the result to a file
    let output_path = "data/2024_assembly/analysis/percentage_votes_state_wise_jammu_kashmir.json";
    let result_json = serde_json::to_string_pretty(&result).unwrap();
    fs::write(output_path, result_json).expect("Unable to write file");

    println!("Party-wise votes and percentages saved to {}", output_path);
}
