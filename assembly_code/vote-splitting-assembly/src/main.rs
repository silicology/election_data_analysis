use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

fn main() {
    let path = "data/2025_assembly/constituency_data_delhi_2025.json";

    // Read JSON data from the file
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let votes_data: Value = serde_json::from_str(&json_data).unwrap();

    let mut results: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();

    // Iterate through each constituency
    if let Some(constituencies) = votes_data.as_array() {
        for constituency in constituencies {
            let constituency_name = constituency["Constituency"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string();
            let candidates_info = constituency["Candidates_info"].as_array().unwrap();

            let mut winner_name = String::new();
            let mut winner_party = String::new();
            let mut winner_votes = 0;
            let mut first_runner_up_name = String::new();
            let mut first_runner_up_party = String::new();
            let mut first_runner_up_votes = 0;
            let mut loser_total_votes = 0;

            for candidate in candidates_info {
                let status = candidate["Status"].as_str().unwrap_or("");
                let candidate_name = candidate["Candidate Name"].as_str().unwrap().to_string();
                let party_name = candidate["Party"].as_str().unwrap().to_string();
                let votes: i64 = candidate["Votes"].as_str().unwrap().parse().unwrap_or(0);

                if status == "won" {
                    // Winner information
                    winner_name = candidate_name;
                    winner_party = party_name;
                    winner_votes = votes;
                } else {
                    // Accumulate votes for losers and check if candidate is the first runner-up
                    loser_total_votes += votes;
                    if votes > first_runner_up_votes {
                        first_runner_up_name = candidate_name;
                        first_runner_up_party = party_name;
                        first_runner_up_votes = votes;
                    }
                }
            }

            // Store results for the constituency
            let mut constituency_result = BTreeMap::new();
            constituency_result.insert("Winner Name".to_string(), Value::String(winner_name));
            constituency_result.insert("Winner Party".to_string(), Value::String(winner_party));
            constituency_result.insert(
                "Winner Votes".to_string(),
                Value::Number(winner_votes.into()),
            );
            constituency_result.insert(
                "Loser Total Votes".to_string(),
                Value::Number(loser_total_votes.into()),
            );
            constituency_result.insert(
                "First Runner-Up Name".to_string(),
                Value::String(first_runner_up_name),
            );
            constituency_result.insert(
                "First Runner-Up Party".to_string(),
                Value::String(first_runner_up_party),
            );

            constituency_result.insert(
                "First Runner-Up Votes".to_string(),
                Value::Number(first_runner_up_votes.into()),
            );

            // Check if the winner has more votes than the total votes of the losers
            let winner_won = winner_votes > loser_total_votes;
            constituency_result.insert(
                "Winner Won Majority".to_string(),
                Value::String(if winner_won { "Yes" } else { "No" }.to_string()),
            );

            results.insert(constituency_name, constituency_result);
        }
    }

    // Save results to a JSON file
    let result_json = serde_json::to_string_pretty(&results).unwrap();
    fs::write(
        "data/2025_assembly/analysis/vote-splitting-delhi-2025.json",
        result_json,
    )
    .expect("Unable to write file");

    println!("Results saved to data/2024_assembly/analysis/test-data.json");
}
