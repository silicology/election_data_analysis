use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;

fn main() {
    let path = "data/state_wise_votes.json";
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let votes_data: Value = serde_json::from_str(&json_data).unwrap();

    let mut total_votes: i64 = 0;
    let mut party_totals: BTreeMap<String, i64> = BTreeMap::new();

    // Aggregate votes across all states
    for (_state, parties) in votes_data.as_object().unwrap() {
        for (party, votes) in parties.as_object().unwrap() {
            let count = votes.as_i64().unwrap();
            *party_totals.entry(party.clone()).or_insert(0) += count;
            total_votes += count;
        }
    }

    // Calculate percentage and collect data
    let mut party_percentages: Vec<Value> = party_totals
        .into_iter()
        .map(|(party, votes)| {
            let percentage = (votes as f64 / total_votes as f64) * 100.0;
            json!({
                "party": party,
                "votes": votes,
                "percentage": percentage
            })
        })
        .collect();

    // Sort by decreasing percentage
    party_percentages.sort_by(|a, b| {
        b["percentage"]
            .as_f64()
            .partial_cmp(&a["percentage"].as_f64())
            .unwrap()
    });

    let result_json = serde_json::to_string_pretty(&party_percentages).unwrap();
    fs::write(
        "data/party_percentage_votes_sorted_desc_country.json",
        result_json,
    )
    .expect("Unable to write file");
}
