pub mod file_manip {
    use home::home_dir;
    use std::{
        env,
        error::Error,
        fs::{self},
        io,
        path::{Path, PathBuf},
    };

    pub fn find_dir() -> PathBuf {
        let home = home_dir().unwrap();

        let mut abs_path: PathBuf = PathBuf::new();
        abs_path = abs_path.join(home.as_path());
        abs_path.push(".minecraft/saves");

        println!("{}", abs_path.display());

        abs_path
    }

    pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        // create_dir_all doesn't throw an error, so no need for a match block
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    /// Finds specified save file.
    /// __Note: This currently only works for minecraft worlds!__
    pub fn find_save_file(abs_path: &mut PathBuf, savefile: &str) -> Result<(), Box<dyn Error>> {
        // Note for future iterations
        // steam filepath: home/<usr>/.steam/steam/steamapps/compatdata/374320/pfx/drive_c/users/steamuser/'Application Data'
        // may need to begin with ../.. replacing home/<usr>
        println!("current directory: {:?}", env::current_dir());
        // let filepath = format!("../../.minecraft/saves/{}", savefile);
        abs_path.push(savefile);
        match fs::read_dir(&abs_path) {
            Ok(_) => {
                let dest = format!("games/minecraft/saves/{}", savefile);
                copy_dir_all(&abs_path, dest)?;
            }
            Err(e) => {
                eprintln!("Couldn't read directory due to following error:\n{e}");
                panic!();
            }
        };
        Ok(())
    }

    pub fn replace_game_save_file(
        abs_path: &mut PathBuf,
        savefile: &str,
    ) -> Result<(), Box<dyn Error>> {
        // let filepath = format!("../../.minecraft/saves/{}", savefile);
        abs_path.push(savefile);
        let local_path = format!("games/minecraft/saves/{}", savefile);

        fs::remove_dir_all(&abs_path)?;
        copy_dir_all(local_path, &abs_path)?;

        match fs::read_dir(&abs_path) {
            Ok(_) => println!("save file successfully replaced!"),
            Err(_) => println!("Error occurred when replacing file!"),
        }
        Ok(())
    }

    pub fn swap_save_files(abs_path: &mut PathBuf, save_name: &str) -> Result<(), Box<dyn Error>> {
        // let filepath = format!("../../.minecraft/saves/{}", save_name);
        abs_path.push(save_name);
        let temp_path = format!("games/minecraft/temp/{}", save_name);
        let local_path = format!("games/minecraft/saves{}", save_name);

        copy_dir_all(&abs_path, &temp_path)?;
        fs::remove_dir_all(&abs_path)?;
        copy_dir_all(local_path, &abs_path)?;
        fs::remove_dir_all(&temp_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use crate::file_search::find_save_file;

    #[test]
    fn find_file() {
        //assert!(find_save_file("minecraft"));
    }
}
