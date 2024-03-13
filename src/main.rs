use serde::{Deserialize};
use std::env;
use std::fs;





#[derive(Debug)]
#[allow(dead_code)]
struct Result {
    value: i32,
    processed_at: i32,
}




// Define the Monitor struct to represent each monitor object
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Monitor {
    #[serde(rename = "monitor_id")] // Rename the field to match JSON key
    id: Option<u32>,  // Use Option to handle null values
    name: String,
    #[serde(default)] // Use default value if field is missing in JSON
    script: Option<String>,
    #[serde(default)] // Use default value if field is missing in JSON
    result: Option<String>,
    code: String,
}




// Define the MonitorsData struct to represent the entire JSON structure
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MonitorsData {
    monitors: Vec<Monitor>,
}



fn process_monitor(json_file_path: &str) {
    // Read the JSON data from the file
    let json_data = match fs::read_to_string(json_file_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    // Deserialize the JSON data into MonitorsData struct
    let monitors_data: MonitorsData = match serde_json::from_str(&json_data) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            return;
        }
    };

    // Print the deserialized data
    println!("{:?}", monitors_data);
    println!("now ok");
}



fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        println!("Usage: {} <json-file-path>", args[0]);
        return;
    }

    // Get the JSON file path from the command-line argument
    let json_file_path = &args[1];

    // Call the process_monitor function
    process_monitor(json_file_path);
}
