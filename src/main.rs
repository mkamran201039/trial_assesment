use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use rand::{thread_rng, Rng};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Result {
    value: i32,
    processed_at: i64,
}

impl Result {
    // Create a new Result instance with random values
    fn new() -> Result {
        let mut rng = thread_rng();
        Result {
            value: rng.gen_range(1..100), // Any numeric value
            processed_at: Utc::now().timestamp(),
        }
    }
}




#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct Monitor {
    monitor_id: Option<u32>, // Use Option to handle null values
    name: String,
    #[serde(default)] // Use default value if field is missing in JSON
    script: Option<String>,
    #[serde(default)] // Use default value if field is missing in JSON
    result: Option<Result>,
    code: String,
}

impl Monitor {
    // Create a new Monitor instance with a random Result
    #[allow(dead_code)] 
    fn new_with_random_result() -> Monitor {
        Monitor {
            monitor_id: None,
            name: String::from("New Monitor"),
            script: None,
            result: Some(Result::new()),
            code: String::from("random_code"),
        }
    }
}

// Define the MonitorsData struct to represent the entire JSON structure
#[derive(Debug, Deserialize, Serialize)]
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
    let mut monitors_data: MonitorsData = match serde_json::from_str(&json_data) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            return;
        }
    };


    

     
    println!("\n JSON data parsed from monitors.json file and stored in MonitorsData structure \n ");
    // Print the deserialized data
    for i in 0..monitors_data.monitors.len() {
        println!("{}. {:?} ", i, monitors_data.monitors[i]);
    }





    // Update each monitor with a random Result
    for monitor in &mut monitors_data.monitors {
        monitor.result = Some(Result::new());
    }

    // Convert the updated data back to JSON
    let updated_json_data = match serde_json::to_string_pretty(&monitors_data) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error converting to JSON: {}", err);
            return;
        }
    };

    // Print the updated JSON data
    println!(
        "\n JSON data with updated random results for monitors:\n{}",
        updated_json_data
    );

    
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
