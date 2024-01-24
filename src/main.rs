use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// Define the CSVData struct
struct CSVData {
    data: Vec<Vec<String>>,
    records: usize,
    fields: usize,
}

// Implement a custom error type for better error handling
#[derive(Debug)]
struct CsvError {
    message: String,
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CSV Error: {}", self.message)
    }
}

impl Error for CsvError {}

// Function to load CSV data from a file into CSVData struct
fn load_csv(file_path: &str) -> Result<CSVData, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    let mut records = 0;
    let mut fields = 0;

    for line in reader.lines() {
        let record: Vec<String> = line?.split(',').map(|s| s.trim().to_string()).collect();
        if fields == 0 {
            fields = record.len();
        } else if fields != record.len() {
            return Err(Box::new(CsvError {
                message: "Inconsistent number of fields in CSV".to_string(),
            }));
        }
        data.push(record);
        records += 1;
    }

    Ok(CSVData { data, records, fields })
}

// Function to display entire file
fn display_entire_file(csv_data: &CSVData) {
    for record in &csv_data.data {
        println!("{}", record.join(", "));
    }
}

// Function to paginate and display rows from xa to xb
fn paginate(csv_data: &CSVData, xa: usize, xb: usize) {
    if xa > csv_data.records || xb > csv_data.records || xa > xb || xa == 0 || xb == 0 {
        println!("Invalid pagination parameters");
    } else {
        for i in xa - 1..xb {
            println!("{}", csv_data.data[i].join(", "));
        }
    }
}

// Function to delete a row
fn delete_row(csv_data: &mut CSVData, row_index: usize) {
    if row_index > 0 && row_index <= csv_data.records {
        csv_data.data.remove(row_index - 1);
        csv_data.records -= 1;
        display_entire_file(csv_data);
    } else {
        println!("Invalid row index for deletion");
    }
}

// Function to modify a field in a row
fn modify_field(csv_data: &mut CSVData, row_index: usize, field_index: usize, new_value: &str) {
    if row_index > 0 && row_index <= csv_data.records && field_index > 0 && field_index <= csv_data.fields {
        csv_data.data[row_index - 1][field_index - 1] = format!(r#""{}""#, new_value);
        paginate(csv_data, row_index, row_index);
    } else {
        println!("Invalid row or field index for modification");
    }
}

// Function to save CSVData to a new or existing CSV file
fn save_to_csv(csv_data: &CSVData, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = if file_path.is_empty() {
        File::create("testdata.csv")?
    } else {
        File::create(file_path)?
    };
    for record in &csv_data.data {
        writeln!(file, "{}", record.join(", "))?;
    }
    Ok(())
}

fn main() {
    let file_path = "testdata.csv";
    let mut csv_data = match load_csv(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading CSV file: {}", e);
            std::process::exit(1);
        }
    };

    loop {
        println!("BootlegEditor3000 Help:");
        println!("1. Display entire file");
        println!("2. Paginate");
        println!("3. Delete row");
        println!("4. Modify field");
        println!("5. Save to CSV");
        println!("6. Exit");

        let choice: usize = get_user_input();

        match choice {
            1 => display_entire_file(&csv_data),
            2 => {
                println!("Enter starting row: ");
                let xa: usize = get_user_input();
                println!("Enter ending row: ");
                let xb: usize = get_user_input();
                paginate(&csv_data, xa, xb);
            }
            3 => {
                println!("Enter row index to delete: ");
                let row_index: usize = get_user_input();
                delete_row(&mut csv_data, row_index);
            }
            4 => {
                println!("Enter row index: ");
                let row_index: usize = get_user_input();
                println!("Enter field index: ");
                let field_index: usize = get_user_input();
                println!("Enter new value: ");
                let new_value = get_user_input_string();
                modify_field(&mut csv_data, row_index, field_index, &new_value);
            }
            5 => {
                println!("Enter file path to save CSV (leave it blank to override the file): ");
                let save_path = get_user_input_string();
                match save_to_csv(&csv_data, &save_path) {
                    Ok(_) => println!("CSV data saved successfully."),
                    Err(e) => eprintln!("Error saving CSV file: {}", e),
                }
            }
            6 => {
                println!("Exiting BootlegEditor3000. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 6.");
            }
        }
    }
}

// Helper function to get user input as usize
fn get_user_input() -> usize {
    let input = get_user_input_string();
    input.trim().parse().unwrap_or(0)
}

// Helper function to get user input as String
fn get_user_input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
