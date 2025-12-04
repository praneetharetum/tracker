use tauri::Manager;

pub mod db;
pub mod diet;

// Re-export for external use
pub use db::DbPath;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = db::migrations::get_migrations();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:tracker.db", migrations)
                .build(),
        )
        .setup(|app| {
            // Get the app data directory and set up the database path
            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
            let db_path = app_data_dir.join("tracker.db").to_string_lossy().to_string();
            app.manage(DbPath(db_path));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            diet::create_diet_entry,
            diet::get_diet_entries,
            diet::update_diet_entry,
            diet::delete_diet_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
