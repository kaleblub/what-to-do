use clap::{Arg, Command};

fn main() {
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
    if let Some(ref matches) = matches.subcommand_matches("add") {
        println!("Add something");
    } else if let Some(ref matches) = matches.subcommand_matches("delete") {
        println!("Delete something")
    } else if let Some(ref matches) = matches.subcommand_matches("print-all") {
        println!("Print all.")
    }
}
