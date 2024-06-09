use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

fn main() {
    let path = "data/state_wise_votes.json";
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let votes_data: Value = serde_json::from_str(&json_data).unwrap();

    let mut result: BTreeMap<String, Value> = BTreeMap::new();

    for (state, parties) in votes_data.as_object().unwrap() {
        let mut total_votes: i64 = 0;
        let mut party_votes: Vec<(String, i64)> = Vec::new();

        for (party, votes) in parties.as_object().unwrap() {
            total_votes += votes.as_i64().unwrap();
            party_votes.push((party.clone(), votes.as_i64().unwrap()));
        }

        party_votes.sort_by(|(_, a), (_, b)| b.cmp(a)); // sort in descending order by vote count

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

        result.insert(state.clone(), json!(state_data));
    }

    let result_json = serde_json::to_string_pretty(&result).unwrap();
    fs::write("data/percentage_votes_state_wise.json", result_json).expect("Unable to write file");
}