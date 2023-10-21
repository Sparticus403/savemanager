use std::{env, error::Error, fs};
/// command line arguments should be either 'guts' or 'legacy' to specify wanted save file
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // TODO: if no args are given, and no files yet exist for the program, do an initial run to
    // create needed files and folders. If filepaths already exist: backup
    if args.len() == 1 {
        match fs::create_dir_all("game/darksouls3/guts") {
            Ok(()) => println!("guts directory created!"),
            // TODO: when error occurs check if file path already exists
            Err(_) => {
                eprintln!("error occurred while creating directory");
                std::process::exit(1);
            }
        };
        match fs::create_dir_all("game/darksouls3/legacy") {
            Ok(()) => println!("legacy directory created!"),
            // TODO: when error occurs check if file path already exists
            Err(_) => {
                eprintln!("error occurred while creating directory");
                std::process::exit(1);
            }
        };

        // std::process::exit(1);
    } else {
        // note to self: cannot use `~` in filepath
        let _prod_path = "home/todd/.steam/steam/steamapps/compatdata/374320/pfx/drive_c/users/steamuser/'Application Data'/DarkSoulsIII/011000010574e989/DS30000.sl2";

        println!("argument given: {}", args[1]);
        // probably not great to use create_dir to check for the existance of a directory...
        match fs::create_dir("home/todd/pictures") {
            Ok(()) => eprintln!("something is wrong here..."),
            Err(_) => println!("filepath already exists!! success!!"),
        }
    }

    Ok(())
}
