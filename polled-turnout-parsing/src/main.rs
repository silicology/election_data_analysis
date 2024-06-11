use csv::Writer;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EVMPolledTurnout {
    sl_no: String,
    state: String,
    pc_name: String,
    count_of_elector: String,
    poll_percentage: String,
    count_of_votes: String,
}

pub fn parse_to_json(filename: String) -> Result<(), Box<dyn Error>> {

    let mut file = File::open(format!("data_polled_evm_turnout/turnout_data_html/{}.xml", filename))?;
    let mut xml_data = String::new();
    file.read_to_string(&mut xml_data)?;

    let mut reader = Reader::from_str(&xml_data);
    let config = reader.config_mut();

    config.trim_text(true);

    let mut turnout_vec: Vec<EVMPolledTurnout> = Vec::new();
    let mut count = 0;

    let mut turnout = EVMPolledTurnout {
        sl_no: String::new(),
        state: String::new(),
        pc_name: String::new(),
        count_of_elector: String::new(),
        poll_percentage: String::new(),
        count_of_votes: String::new(),
    };
    let mut paragraph_index = 0;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"p" => {
                let txt = reader
                    .read_text(e.name())
                    .expect("cannot decode text value");
                // println!("{:?}", txt);
                match paragraph_index {
                    0 => turnout.sl_no = txt.into_owned().trim().parse().unwrap(),
                    1 => turnout.state = txt.into_owned().trim().parse().unwrap(),
                    2 => {
                        turnout.pc_name =
                            txt.into_owned().trim().parse().unwrap();
                        
                    }
                    3 => {
                        turnout.count_of_elector =
                            txt.into_owned().trim().parse().unwrap();
                    }
                    4 => {
                        turnout.poll_percentage =
                            txt.into_owned().trim().parse().unwrap();
                    }
                    5 => {
                        turnout.count_of_votes =
                            txt.into_owned().trim().parse().unwrap();
                        // Add turnout vec to the vector every five paragraphs
                        turnout_vec.push(turnout.clone());
                        count += 1;
                    }
                    _ => {}
                }
                paragraph_index = (paragraph_index + 1) % 6;
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    // Add turnout to the vector if it's not yet added (for the last set of paragraphs)
    if paragraph_index != 0 {
        turnout_vec.push(turnout);
        count += 1;
    }

    println!("Total constituency processed: {}", count);


    // Write data to CSV
    let mut csv_writer = Writer::from_path(format!("data/voter_turnout_data/polled_evm_turnout_data/{}.csv", filename))?;
    csv_writer.write_record(&["sl_no", "state", "pc_name", "count_of_elector", "poll_percentage", "count_of_votes"])?;
    for turnout in &turnout_vec {
        csv_writer.write_record(&[
            &turnout.sl_no,
            &turnout.state,
            &turnout.pc_name,
            &turnout.count_of_elector,
            &turnout.poll_percentage,
            &turnout.count_of_votes,
        ])?;
    }
    csv_writer.flush()?;

    // Write data to JSON
    let json_data = serde_json::to_string_pretty(&turnout_vec)?;
    let mut json_file = File::create(format!("data/voter_turnout_data/polled_evm_turnout_data/{}.json", filename))?;
    json_file.write_all(json_data.as_bytes())?;

    Ok(())
}


fn main() {
    let filename = "phase7".to_owned();
    let _data = parse_to_json(filename);
  
}

