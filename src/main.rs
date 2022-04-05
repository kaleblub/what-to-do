use clap::{Arg, Command};
use std::fs::OpenOptions;
use std::io::{Write, Read, stdin};

const FILE_NAME: &str = ".what-to-do.dat";

fn main() {
    // Create 'my_file' new if it does not exist, or open if it does exist.
    // std::fs::File::create(FILE_NAME).expect("creation failed");

    // Using clap to add subcommands
    let matches = Command::new("What To Do? (To-do list)")
        .version("1.0")
        .author("Kaleb H. <kaleblub.pythonanywhere.com>")
        .about("CLI to-go list reminder on request.")
        .subcommand(Command::new("add")
            .about("Add a new item that needs to be completed eventually.")
            .version("1.0")
            .author("Kaleblub <kaleblub.pythonanywhere.com>"))
        .subcommand(Command::new("delete")
            .about("Delete an item.")
            .version("1.0")
            .author("Kaleblub <kaleblub.pythonanywhere.com>"))
        .subcommand(Command::new("print-all")
            .about("Print the whole list of items to do.")
            .version("1.0")
            .author("Kaleblub <kaleblub.pythonanywhere.com>"))
        .get_matches();


    // Check for existence of subcommands
    if let Some(ref _matches) = matches.subcommand_matches("add") {
        let mut file = OpenOptions::new().append(true).create(true).open(&FILE_NAME).unwrap(); //.expect("Unable to add item");
        let mut input_string = String::new();

        // Get the input for what task to add to list
        println!("What task would you like to store to be reminded about?");
        stdin().read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        
        // Write the user-given line to the file or return an error
        if let Err(e) = write!(file, "{}", input_string) {
            eprintln!("Couldn't write to file: {}", e);
        }
        println!("Added {}", input_string);

    } else if let Some(ref _matches) = matches.subcommand_matches("delete") {
        // my_file.write_all("Delete something".as_bytes()).expect("delete failed");
        
        println!("Deleted something");
    } else if let Some(ref _matches) = matches.subcommand_matches("print-all") {
        // my_file.write_all("Print all".as_bytes()).expect("print all failed");
        let mut file = std::fs::File::open(FILE_NAME).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
}
