use colored::*;

pub fn print_verbose(m: &str, is_verbose: bool) {
    if is_verbose {
        println!("{}: {}", "verbose".red(), m.replace('\n', "\\n"));
    }
}
