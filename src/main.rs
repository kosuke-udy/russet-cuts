mod shortcuts;
mod cli; 

fn main() {
    let cli = cli::load();

    match &cli.command {
        Some(cli::Commands::Run { name , input_path}) => {
            let result = match input_path {
                Some(path) => {
                    shortcuts::run_with_input(name, path)
                },
                None => {
                    shortcuts::run(name)
                }
            }; 
            shortcuts::print_result(result);
        }
        None => {}
    }
}

