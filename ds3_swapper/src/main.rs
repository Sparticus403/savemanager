use std::{env, error::Error, fs};

enum Flag {
    Backup,
    Create,
    Swap
}

#[derive(serde::Serialize)]
struct Game {
    name: String,
    local_path: String,
    game_path: String,
    // maybe make an array or Vector of savefiles, so each game only needs one JSON file to
    // find each savefile
}

/// command line arguments should be `-b`, `-c`, or `-s` followed by the name of the game to be
/// backed up, or `-h` for help
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

        process_args(&args);

        // let _prod_path = "home/todd/.steam/steam/steamapps/compatdata/374320/pfx/drive_c/users/steamuser/'Application Data'/DarkSoulsIII/011000010574e989/DS30000.sl2";

        println!("argument given: {}", args[1]);
        
        // probably not great to use create_dir to check for the existance of a directory...
        // match fs::create_dir("home/todd/pictures") {
        //     Ok(()) => eprintln!("something is wrong here..."),
        //     Err(_) => println!("filepath already exists!! success!!"),
        // }
    }

    Ok(())
}

fn process_args(args: &[String]) {
    let flag = &args[1];

        match flag.as_str() {
            "-b" | "--backup" => create_backup(&args[2]),
            "-c" | "--create" => create_save(&args[2]),
            "-s" | "--swap" => swap_save(&args[2]),
            "-h" | "--help" => print_help(),
            _ => ()
        };
}

fn print_help() {
    println!("help statement and description");
    std::process::exit(0);
}

fn create_backup(game_name: &str) {
    // TODO: copy save file from game filepath and store in local path `games/<game_name>` 
}

fn swap_save(game_name: &str) {
    // TODO: swap stored save file in local path with save file in game
}

fn create_save(game_name: &str) {
    // TODO: find file path of game saves

    let local_path = ("games/".to_string() + game_name.to_lowercase().as_str() + "/").to_string();
    let new_game = Game {
        name: game_name.to_string(),
        local_path,
        game_path: "".to_string()
    };

    let local_path = ("games/".to_string() + game_name.to_lowercase().as_str() + "/").to_string();
    // If local path already exists, great! If not, it'll be created
    let _ = fs::create_dir_all(local_path.clone());

    if let Ok(json) = serde_json::to_string_pretty(&new_game) {
        println!("Creating new save file!");

        match fs::write(local_path + "save.json", json.as_str()){
            Ok(()) => (),
            Err(_) => panic!("Could not write file!"),
        };
    }
    // TODO: store save file in local filepath


    // TODO: serialize new_game into a json file and save it under a saves or data directory
}
