use clap::{Arg, Command};
use reqwest::blocking::Client;
use serde_json::json; // Import json macro directly
use std::fs::File;
use std::io::Write;

fn main() {
    let matches = Command::new("RuralHealth CLI")
        .version("1.0")
        .about("Improving healthcare in rural areas")
        .author("Your Name")
        .subcommand(
            Command::new("triage")
                .about("Check symptoms and get health advice")
                .arg(
                    Arg::new("symptoms")
                        .short('s')
                        .long("symptoms")
                        .required(true) // Makes this argument mandatory
                        .num_args(1) // Specifies it takes exactly one argument
                        .help("Enter symptoms (comma-separated, e.g., fever,cough)"),
                ),
        )
        .subcommand(
            Command::new("book")
                .about("Book a teleconsultation")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .required(true) // Mandatory
                        .num_args(1) // Takes exactly one argument
                        .help("Patient's name"),
                )
                .arg(
                    Arg::new("phone")
                        .short('p')
                        .long("phone")
                        .required(true) // Mandatory
                        .num_args(1) // Takes exactly one argument
                        .help("Patient's phone number for confirmation"),
                ),
        )
        .subcommand(Command::new("tips").about("View general health tips"))
        .subcommand(
            Command::new("supplies")
                .about("Request medical supplies")
                .arg(
                    Arg::new("item")
                        .short('i')
                        .long("item")
                        .required(true) // Mandatory
                        .num_args(1) // Takes exactly one argument
                        .help("Name of the medical supply item to request"),
                ),
        )
        .get_matches();

    // Handle the subcommands
    match matches.subcommand() {
        Some(("triage", sub_matches)) => {
            let symptoms = sub_matches.get_one::<String>("symptoms").unwrap();
            handle_triage(symptoms);
        }
        Some(("book", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let phone = sub_matches.get_one::<String>("phone").unwrap();
            handle_booking(name, phone);
        }
        Some(("tips", _)) => {
            display_health_tips();
        }
        Some(("supplies", sub_matches)) => {
            let item = sub_matches.get_one::<String>("item").unwrap();
            request_supplies(item);
        }
        _ => {
            println!("Use --help to see available commands.");
        }
    }
}

fn handle_triage(symptoms: &str) {
    let symptom_list: Vec<&str> = symptoms.split(',').collect();
    println!("Analyzing symptoms: {:?}", symptom_list);

    // Basic triage logic
    if symptom_list.contains(&"fever") && symptom_list.contains(&"cough") {
        println!("Recommendation: Visit a health center (Possible respiratory infection).");
    } else {
        println!("Recommendation: Monitor symptoms and stay hydrated.");
    }
}

fn handle_booking(name: &str, phone: &str) {
    println!(
        "Booking teleconsultation for {} at phone number {}.",
        name, phone
    );

    // Create a new HTTP client
    let client = Client::new();

    // Use the json macro to create the payload
    let payload = json!({
        "to": phone,
        "message": format!("Hello {}, your teleconsultation is booked!", name)
    });

    // Simulate sending an SMS confirmation using a mock API
    let response = client
        .post("https://api.example.com/send-sms") // Replace with a real API endpoint
        .json(&payload) // Use the JSON payload
        .send();

    // Handle the response
    match response {
        Ok(_) => println!("Booking confirmed! SMS sent."),
        Err(e) => eprintln!("Failed to send SMS: {}", e),
    }
}

fn display_health_tips() {
    let tips = vec![
        "Drink clean, boiled water to prevent cholera.",
        "Sleep under treated mosquito nets to prevent malaria.",
        "Wash hands regularly to avoid infections.",
    ];
    for tip in tips {
        println!("- {}", tip);
    }
}

fn request_supplies(item: &str) {
    let mut file = File::create("supply_requests.csv").expect("Unable to create file");
    writeln!(file, "Item,Requested").expect("Unable to write to file");
    writeln!(file, "{},Yes", item).expect("Unable to write to file");
    println!("Request for '{}' submitted successfully.", item);
}
