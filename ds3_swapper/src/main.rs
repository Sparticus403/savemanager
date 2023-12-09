use ds3_swapper::file_manip;
use std::{env, error::Error};
mod processes;
use processes::process_args;

/// command line arguments should be `-b`, `-r`, or `-s` followed by the name of the minecraft
/// world to be backed up, or `-h` for help
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    process_args(&args);
    Ok(())
}
