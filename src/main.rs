mod tools;

fn main() {

    let args = tools::parser::get_arguments();

    if args.choice == "solve" {
        tools::solve::solve_algorithm(args);
    }
    else {
        tools::help::help();
    }
    
}
