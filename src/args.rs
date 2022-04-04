// use clap::Parser;

// /// CLI to-do list reminder on request.
// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[clap(short, long)]
//     name: String,

//     /// Number of times to greet
//     #[clap(short, long, default_value_t = 1)]
//     count: u8,
// }



pub fn get_args() {
    // let args = Args::parse();
    


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


    // if command == "" || command == "-h" || command == "--help" {
    //     println!("This is the help screen..");
    // } else if command == "-p" || command == "--print" {
    //     println!("Printing a random thing that you can do...");
    // } else if command == "-P" || command == "--print-all" {
    //     println!("Printing all things that you can do...");
    // } else if command == "-a" || command == "--add" {
    //     println!("");
    // } else if command == "-d" || command == "--delete" {
    //     println!("Print options of things to delete");
    //     println!("What would you like to delete?");
    // }
}