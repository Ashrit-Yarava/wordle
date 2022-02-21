use std::env;

pub struct Args {
    pub choice: String,
    pub wordfile: String,
    pub args: Vec<String>,
}

pub fn get_arguments() -> Args {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0); // Remove the bin name.
    let mut choice: String = "help".to_string();
    if args.len() > 0 {
        choice = args.remove(0).to_string();
    }
    let mut wordfile = "".to_string();
    if args.len() > 0 {
        wordfile = args.remove(0).to_string();
    }

    let mut arguments: Vec<String> = Vec::new();
    for arg in args.iter() {
        arguments.push(arg.to_string());
    }

    let arguments = Args {
        choice,
        wordfile,
        args: arguments,
    };

    return arguments;
}
