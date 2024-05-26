use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};
use rusqlite::{params, Connection};
use serde_json::Value;
use nfd::Response;

fn main() {
    println!("\nWuthering Waves FPS Unlocker\n");
    println!("Ensure that your game's FPS cap is set to 60, V-Sync is disabled, and the game is closed before proceeding.");
    println!("Do not adjust the FPS value in-game after patching, as it will reset to default.");
    println!("File repair after patching is normal, just let it run and start the game after.\n");
    println!("If the patch will not work, you might need to run the program as administrator.\n");

    loop {
        println!("Please select the Wuthering Waves Game directory (e.g. C:\\Wuthering Waves Game):");
        let result = nfd::open_pick_folder(None).expect("Error opening folder picker dialog.");

        let game_dir = match result {
            Response::Okay(path) => path,
            Response::OkayMultiple(_) => {
                println!("Please select only one directory.");
                continue;
            }
            Response::Cancel => {
                println!("User canceled folder selection. Exiting program.");
                return;
            }
        };

        println!("Selected game directory: {}", game_dir);

        let db_path = match find_local_storage_db(&game_dir) {
            Some(path) => path,
            None => {
                println!("Failed to locate LocalStorage.db in the provided directory. Please try again.");
                continue;
            }
        };

        println!("Located LocalStorage.db at: {}", db_path.display());
        println!("Is this the correct path? (y/n)");

        let mut confirmation = String::new();
        io::stdin()
            .read_line(&mut confirmation)
            .expect("Failed to read input.");

        if confirmation.trim().to_lowercase() != "y" {
            println!("Please rerun the program and provide the correct path to LocalStorage.db.");
            return;
        }

        println!("Enter the desired FPS value (120 is max):");
        
        let mut fps_value = String::new();
        io::stdin()
            .read_line(&mut fps_value)
            .expect("Failed to read input.");

        let fps_value: i32 = fps_value.trim().parse().expect("Please enter a valid integer.");
        if fps_value < 30 || fps_value > 120 {
            println!("Please enter a value between 30 and 120.");
            continue;
        }

        let connection = Connection::open(&db_path).expect("Failed to connect to LocalStorage.db.");
        println!("Connected to LocalStorage.db.");

        let select_query = "SELECT value FROM LocalStorage WHERE key = 'GameQualitySetting';";
        let update_query = "UPDATE LocalStorage SET value = ? WHERE key = 'GameQualitySetting';";

        let game_quality_setting_json: String = connection
            .query_row(select_query, params![], |row| row.get(0))
            .unwrap_or_else(|_| {
                eprintln!("No GameQualitySetting key found.");
                std::process::exit(1);
            });

        println!("Original JSON has been saved to original.json.");
        write_to_file("original.json", &game_quality_setting_json);

        let mut game_quality_setting: Value =
            serde_json::from_str(&game_quality_setting_json).expect("Parsing failed.");
        game_quality_setting["KeyCustomFrameRate"] =
            Value::Number(serde_json::Number::from(fps_value));

        let updated_game_quality_setting_json =
            serde_json::to_string(&game_quality_setting).expect("Serialization failed.");

        println!("Patched JSON has been saved to patched.json.");
        write_to_file("patched.json", &updated_game_quality_setting_json);

        connection
            .execute(update_query, params![&updated_game_quality_setting_json])
            .expect("Failed to update database.");
        println!("Connection to LocalStorage.db closed.");

        println!("\nThe game has been successfully patched. Press Enter to exit.");
        let _ = io::stdin().read_line(&mut String::new());
        return;
    }
}

fn find_local_storage_db(game_dir: &str) -> Option<PathBuf> {
    for entry in walkdir::WalkDir::new(game_dir) {
        let entry = entry.expect("Error reading directory");
        if let Some(filename) = entry.file_name().to_str() {
            if filename == "LocalStorage.db" {
                return Some(entry.path().to_path_buf());
            }
        }
    }
    None
}

fn write_to_file(filename: &str, data: &str) {
    let mut file = File::create(filename).expect("Failed to create file.");
    file.write_all(data.as_bytes())
        .expect("Failed to write to file.");
}
