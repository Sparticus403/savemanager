
use crate::file_manip;
use std::path::PathBuf;

pub fn process_args(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Too few arguments!");
        print_help();
        return;
    }

    let mut mc_path = file_manip::find_dir();
    let flag = &args[1];

    match flag.as_str() {
        "-b" | "--backup" => create_backup(&mut mc_path, &args[2]),
        "-s" | "--swap" => swap_save(&mut mc_path, &args[2]),
        "-h" | "--help" => print_help(),
        "-r" | "--revert" => revert_save(&mut mc_path, &args[2]),
        _ => {
            println!("unrecognized command!\n");
            print_help()
        }
    };
}
fn print_help() {
    println!("help statement and description");
    let help_backup_text =
        String::from("-b   --backup    backup save file of passed as an additional parameter\n");
    // let help_create_text = String::from(
    //     "-c   --create    create new backup files for game passed as an additional parameter\n",
    // );
    let help_revert_text =
        String::from("-r   --revert    revert save file to version stored as a backup\n");
    let help_swap_text = String::from("-s   --swap      swap game save file with file stored in this program. Will take name of save file as additional parameter. Save file can only be swapped with file of same name\n");
    let help_text = String::from("-h   --help      shows this help text\n");
    let full_help_text = format!(
        "{}{}{}{}",
        help_backup_text, help_text, help_revert_text, help_swap_text
    );
    println!("{full_help_text}");
    // std::process::exit(0);
}

fn revert_save(path: &mut PathBuf, save_name: &str) {
    let _ = file_manip::replace_game_save_file(path, save_name);
}

fn create_backup(path: &mut PathBuf, save_name: &str) {
    let _ = file_manip::find_save_file(path, save_name);
}

fn swap_save(path: &mut PathBuf, save_name: &str) {
    let _ = file_manip::swap_save_files(path, save_name);
    // TODO: swap stored save file in local path with save file in game
}
