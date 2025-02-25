use malis::Malis;

fn main() {
    let mut args = std::env::args();
    // First arguments is always the current binary's path, which we do not need
    let _ = args.next();

    match args.next() {
        // If we do have a second argument, we execute it
        Some(arg) => {
            let execution = Malis::execute(&arg);
            if let Err(e) = execution {
                println!("{}", e);
                std::process::exit(70);
            }
        }
        // If not, we enter interactive mode in the prompt
        None => Malis::interactive().expect("Failed to execut script"),
    };
}
