use std::{fs::File, io::{self, Write}};
use rusqlite::{params, Connection};
use serde_json::Value;
fn main() {
    println!("\nWuthering Waves FPS unlocker.\n");
    println!("Make sure to set your FPS cap to 60 in game, turn off V-Sync and turn off your game.");
    println!("After patching, don't set the FPS value in game, it will reset the value.");
    println!("File repair after patching is normal, just let it run and start the after.\n");
    println!("Press Enter to patch...");

    let _ = io::stdin().read_line(&mut String::new());

    println!("Enter the path to the LocalStorage.db file (C:\\Wuthering Waves\\Wuthering Waves Game\\Client\\Saved\\LocalStorage\\LocalStorage.db)\n");

    let mut db_path = String::new();

    io::stdin()
        .read_line(&mut db_path)
        .expect("Failed to read input.");
    let db_path = db_path.trim();

    println!("Enter the desired FPS value (75, 240, 360)\n");

    let mut fps_value = String::new();

    io::stdin()
        .read_line(&mut fps_value)
        .expect("Failed to read input.");

    let fps_value: i32 = fps_value.trim().parse()
        .expect("Please enter a valid integer.");


    let connection = Connection::open(db_path).expect("Failed to connect to LocalStorage.db.");
    println!("Connected to LocalStorage.db.");

    let select_query = "SELECT value FROM LocalStorage WHERE key = 'GameQualitySetting';";
    let update_query = "UPDATE LocalStorage SET value = ? WHERE key = 'GameQualitySetting';";

    let game_quality_setting_json: String = connection
        .query_row(select_query, params![], |row| row.get(0))
        .unwrap_or_else(|_| {
            eprintln!("No GameQualitySetting key found.");
            std::process::exit(1);
        });

    println!("Original JSON has been written to original.json.");
    write_to_file("original.json", &game_quality_setting_json);

    let mut game_quality_setting: Value =
        serde_json::from_str(&game_quality_setting_json).expect("Parsing failed.");
    game_quality_setting["KeyCustomFrameRate"] = Value::Number(serde_json::Number::from(fps_value));

    let updated_game_quality_setting_json =
        serde_json::to_string(&game_quality_setting).expect("Serializing failed.");

    println!("Patched JSON has been written to patched.json.");
    write_to_file("patched.json", &updated_game_quality_setting_json);

    connection
        .execute(update_query, params![&updated_game_quality_setting_json])
        .expect("Failed to update database.");
    println!("Closed connection to LocalStorage.db.");

    println!("\nGame should be patched. Press Enter to exit.");
    let _ = io::stdin().read_line(&mut String::new());
}

fn write_to_file(filename: &str, data: &str) {
    let mut file = File::create(filename).expect("Failed to create file.");
    file.write_all(data.as_bytes())
        .expect("Failed to write to file.");
}
