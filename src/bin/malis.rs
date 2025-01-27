use malis::{Malis, MalisError};

fn main() {
    let mut args = std::env::args();
    // First arguments is always the current binary's path, which we do not need
    let _ = args.next();

    match args.next() {
        // If we do have a second argument, we execute it
        Some(arg) => {
            let execution = Malis::execute(&arg);
            match execution {
                Err(MalisError::RuntimeError(e)) => {
                    println!("{}", e);
                    std::process::exit(70);
                }
                _ => {}
            }
        }
        // If not, we enter interactive mode in the prompt
        None => Malis::interactive().expect("Failed to execut script"),
    };
}
