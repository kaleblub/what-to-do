use std::env;

pub fn get_args() {
    // Recieve Arguments in Vector
    let args: Vec<String> = env::args().collect();
    // Clone the first argument
    let command = args[1].clone();
    println!("Args: {:?}", args);
    println!("Command: {}", command);
    
    /* 
    Have Multiple commands:
    -p --print : Print Something I can do
        option to have an argument to print multiple things, -> Int
    -P --print-all : Print the whole list of things I can do
        no argument
    -a --add : Add a new thing that I can do
        Argument should have "" around to be entered as string
    -d --delete : Delete a thing that I can do
        Argument must be l integer    
    */


    if command == "" || command == "-h" || command == "--help" {
        println!("This is the help screen..");
    } else if command == "-p" || command == "--print" {
        println!("Printing a random thing that you can do...");
    } else if command == "-P" || command == "--print-all" {
        println!("Printing all things that you can do...");
    } else if command == "-a" || command == "--add" {
        println!("");
    } else if command == "-d" || command == "--delete" {
        println!("Print options of things to delete");
        println!("What would you like to delete?");
    }
}