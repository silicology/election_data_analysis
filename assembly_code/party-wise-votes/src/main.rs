use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;

fn main() {
    let path = "data/2024_assembly/constituency_data_odisha.json";

    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let votes_data: Value = serde_json::from_str(&json_data).unwrap();

    let mut party_votes: BTreeMap<String, i64> = BTreeMap::new();
    // Iterate through each constituency
    if let Some(constituencies) = votes_data.as_array() {
        for constituency in constituencies {
            if let Some(candidates_info) = constituency
                .get("Candidates_info")
                .and_then(|ci| ci.as_array())
            {
                // Iterate through each candidate's info
                for candidate in candidates_info {
                    if let (Some(party), Some(votes)) = (
                        candidate.get("Party").and_then(|p| p.as_str()),
                        candidate
                            .get("Votes")
                            .and_then(|v| v.as_str().unwrap_or("").parse::<i64>().ok()),
                    ) {
                        // Add votes to the corresponding party
                        *party_votes.entry(party.to_string()).or_insert(0) += votes;
                    }
                }
            }
        }
    }

    // // Print the party-wise vote counts
    // for (party, votes) in &party_votes {
    //     println!("{}: {}", party, votes);
    // }

    let result_json = serde_json::to_string_pretty(&party_votes).unwrap();
    fs::write(
        "data/2024_assembly/analysis/party_wise_votes_odisha.json",
        result_json,
    )
    .expect("Unable to write file");
}
