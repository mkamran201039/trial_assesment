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
    #[serde(rename = "type")] // Rename field to "type" in JSON
    monitor_type: Option<String>,    // Use different name in Rust to avoid conflict with keyword
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
            monitor_type: Some(String::from("none")),
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













// async fn update_monitors(monitors_data: &mut MonitorsData) {
//     loop {
//         // Update each monitor with a random Result
//         for monitor in &mut monitors_data.monitors {
//             monitor.result = Some(Result::new());
//         }
//         // Wait for 30 seconds before next update
//          sleep(Duration::from_secs(30)).await;
//     }
// }

// async fn store_monitors(mut monitors_data: MonitorsData) {
//     loop {
//         // Generate the filename with current timestamp
//         let current_time = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
//         let filename = format!("{}_monitors.json", current_time);

//         // Serialize the monitors data to JSON
//         let json_data = match serde_json::to_string_pretty(&monitors_data) {
//             Ok(data) => data,
//             Err(err) => {
//                 eprintln!("Error converting to JSON: {}", err);
//                 continue; // Skip this iteration if serialization fails
//             }
//         };

//         // Write JSON data to file
//         match fs::write(&filename, json_data) {
//             Ok(()) => println!("Monitors data stored in file: {}", filename),
//             Err(err) => eprintln!("Error writing to file: {}", err),
//         }

//         // Wait for 1 minute before next store operation
//          sleep(Duration::from_secs(60)).await;
//     }
// }

// async fn process_monitors2(json_file_path: String) {
//     // Read the JSON data from the file
//     let json_data = match fs::read_to_string(&json_file_path) {
//         Ok(data) => data,
//         Err(err) => {
//             eprintln!("Error reading file: {}", err);
//             return;
//         }
//     };

//     // Deserialize the JSON data into MonitorsData struct
//     let mut monitors_data: MonitorsData = match serde_json::from_str(&json_data) {
//         Ok(data) => data,
//         Err(err) => {
//             eprintln!("Error parsing JSON: {}", err);
//             return;
//         }
//     };

//     // Create a new task to update monitors concurrently
//     let update_task = update_monitors(&mut monitors_data);

//     // Create a new task to store monitors data concurrently
//     let store_task = store_monitors(monitors_data);

//     // // Wait for both tasks to finish
//     // tokio::try_join!(update_task, store_task).unwrap_or_else(|e| {
//     //     eprintln!("Error: {:?}", e);
//     // });
// }






















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
