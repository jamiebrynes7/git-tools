use std::process;

pub fn error_and_exit(error_message: String) {
    println!("{}", error_message);
    process::exit(1);
}
