use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tauri_plugin_sql::{Migration, MigrationKind};

// ============================================================================
// Data Models (#37)
// ============================================================================

/// Meal type enum for categorizing diet entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Other,
}

impl std::fmt::Display for MealType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MealType::Breakfast => write!(f, "Breakfast"),
            MealType::Lunch => write!(f, "Lunch"),
            MealType::Dinner => write!(f, "Dinner"),
            MealType::Snack => write!(f, "Snack"),
            MealType::Other => write!(f, "Other"),
        }
    }
}

impl std::str::FromStr for MealType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Breakfast" => Ok(MealType::Breakfast),
            "Lunch" => Ok(MealType::Lunch),
            "Dinner" => Ok(MealType::Dinner),
            "Snack" => Ok(MealType::Snack),
            "Other" => Ok(MealType::Other),
            _ => Err(format!("Unknown meal type: {}", s)),
        }
    }
}

/// Diet entry as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietEntry {
    pub id: i64,
    pub member_id: i64,
    pub timestamp: String,
    pub meal_type: MealType,
    pub description: String,
    pub calories: Option<i64>,
    pub notes: Option<String>,
}

/// Request payload for creating a new diet entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDietEntryRequest {
    pub member_id: i64,
    pub timestamp: String,
    pub meal_type: MealType,
    pub description: String,
    pub calories: Option<i64>,
    pub notes: Option<String>,
}

/// Request payload for updating an existing diet entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDietEntryRequest {
    pub id: i64,
    pub member_id: Option<i64>,
    pub timestamp: Option<String>,
    pub meal_type: Option<MealType>,
    pub description: Option<String>,
    pub calories: Option<i64>,
    pub notes: Option<String>,
}

/// Filter criteria for querying diet entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietEntryFilter {
    pub member_id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// ============================================================================
// Database State
// ============================================================================

/// Holds the database path for use in commands
pub struct DbPath(pub String);

// ============================================================================
// Tauri Commands
// ============================================================================

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Create a new diet entry (#38)
#[tauri::command]
async fn create_diet_entry(
    db_path: State<'_, DbPath>,
    member_id: i64,
    timestamp: String,
    meal_type: String,
    description: String,
    calories: Option<i64>,
    notes: Option<String>,
) -> Result<DietEntry, String> {
    // Parse and validate meal_type
    let meal_type_enum: MealType = meal_type.parse().map_err(|e: String| e)?;

    // Connect to database
    let db = rusqlite::Connection::open(&db_path.0)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Validate that member_id exists in family_members table
    let member_exists: bool = db
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM family_members WHERE id = ?)",
            [member_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to validate member_id: {}", e))?;

    if !member_exists {
        return Err(format!("Member with id {} does not exist", member_id));
    }

    // Insert the new diet entry
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description, calories, notes) VALUES (?, ?, ?, ?, ?, ?)",
        rusqlite::params![member_id, timestamp, meal_type_enum.to_string(), description, calories, notes],
    )
    .map_err(|e| format!("Failed to insert diet entry: {}", e))?;

    // Get the ID of the newly inserted entry
    let new_id = db.last_insert_rowid();

    // Return the created entry
    Ok(DietEntry {
        id: new_id,
        member_id,
        timestamp,
        meal_type: meal_type_enum,
        description,
        calories,
        notes,
    })
}

/// Get diet entries with optional filters (#39)
#[tauri::command]
async fn get_diet_entries(
    db_path: State<'_, DbPath>,
    member_id: Option<i64>,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<Vec<DietEntry>, String> {
    // Connect to database
    let db = rusqlite::Connection::open(&db_path.0)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Build dynamic query based on provided filters
    let mut sql = String::from(
        "SELECT id, member_id, timestamp, meal_type, description, calories, notes FROM diet_entries WHERE 1=1",
    );
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(mid) = member_id {
        sql.push_str(" AND member_id = ?");
        params.push(Box::new(mid));
    }

    if let Some(ref start) = start_date {
        sql.push_str(" AND timestamp >= ?");
        params.push(Box::new(start.clone()));
    }

    if let Some(ref end) = end_date {
        sql.push_str(" AND timestamp <= ?");
        params.push(Box::new(end.clone()));
    }

    sql.push_str(" ORDER BY timestamp DESC");

    // Execute query
    let mut stmt = db
        .prepare(&sql)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let entries = stmt
        .query_map(param_refs.as_slice(), |row| {
            let meal_type_str: String = row.get(3)?;
            let meal_type: MealType = meal_type_str
                .parse()
                .unwrap_or(MealType::Other);

            Ok(DietEntry {
                id: row.get(0)?,
                member_id: row.get(1)?,
                timestamp: row.get(2)?,
                meal_type,
                description: row.get(4)?,
                calories: row.get(5)?,
                notes: row.get(6)?,
            })
        })
        .map_err(|e| format!("Failed to execute query: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect results: {}", e))?;

    Ok(entries)
}

/// Update an existing diet entry (#40)
#[tauri::command]
async fn update_diet_entry(
    db_path: State<'_, DbPath>,
    id: i64,
    member_id: Option<i64>,
    timestamp: Option<String>,
    meal_type: Option<String>,
    description: Option<String>,
    calories: Option<i64>,
    notes: Option<String>,
) -> Result<DietEntry, String> {
    // Connect to database
    let db = rusqlite::Connection::open(&db_path.0)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Check if entry exists and get current values
    let existing: DietEntry = db
        .query_row(
            "SELECT id, member_id, timestamp, meal_type, description, calories, notes FROM diet_entries WHERE id = ?",
            [id],
            |row| {
                let meal_type_str: String = row.get(3)?;
                let meal_type: MealType = meal_type_str.parse().unwrap_or(MealType::Other);
                Ok(DietEntry {
                    id: row.get(0)?,
                    member_id: row.get(1)?,
                    timestamp: row.get(2)?,
                    meal_type,
                    description: row.get(4)?,
                    calories: row.get(5)?,
                    notes: row.get(6)?,
                })
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => format!("Diet entry with id {} not found", id),
            _ => format!("Failed to query diet entry: {}", e),
        })?;

    // Parse meal_type if provided
    let meal_type_enum = match meal_type {
        Some(mt) => mt.parse().map_err(|e: String| e)?,
        None => existing.meal_type.clone(),
    };

    // Use provided values or fall back to existing
    let new_member_id = member_id.unwrap_or(existing.member_id);
    let new_timestamp = timestamp.unwrap_or(existing.timestamp);
    let new_description = description.unwrap_or(existing.description);
    let new_calories = calories.or(existing.calories);
    let new_notes = notes.or(existing.notes);

    // If member_id is being changed, validate the new member exists
    if member_id.is_some() {
        let member_exists: bool = db
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM family_members WHERE id = ?)",
                [new_member_id],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to validate member_id: {}", e))?;

        if !member_exists {
            return Err(format!("Member with id {} does not exist", new_member_id));
        }
    }

    // Update the entry
    db.execute(
        "UPDATE diet_entries SET member_id = ?, timestamp = ?, meal_type = ?, description = ?, calories = ?, notes = ? WHERE id = ?",
        rusqlite::params![new_member_id, new_timestamp, meal_type_enum.to_string(), new_description, new_calories, new_notes, id],
    )
    .map_err(|e| format!("Failed to update diet entry: {}", e))?;

    // Return the updated entry
    Ok(DietEntry {
        id,
        member_id: new_member_id,
        timestamp: new_timestamp,
        meal_type: meal_type_enum,
        description: new_description,
        calories: new_calories,
        notes: new_notes,
    })
}

/// Delete a diet entry (#40)
#[tauri::command]
async fn delete_diet_entry(db_path: State<'_, DbPath>, id: i64) -> Result<(), String> {
    // Connect to database
    let db = rusqlite::Connection::open(&db_path.0)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Check if entry exists
    let exists: bool = db
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM diet_entries WHERE id = ?)",
            [id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to check if entry exists: {}", e))?;

    if !exists {
        return Err(format!("Diet entry with id {} not found", id));
    }

    // Delete the entry
    db.execute("DELETE FROM diet_entries WHERE id = ?", [id])
        .map_err(|e| format!("Failed to delete diet entry: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: r#"
                CREATE TABLE IF NOT EXISTS family_members (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    icon TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TABLE IF NOT EXISTS tracked_items (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    family_member_id INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    category TEXT,
                    value REAL,
                    notes TEXT,
                    tracked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (family_member_id) REFERENCES family_members(id) ON DELETE CASCADE
                );
            "#,
            kind: MigrationKind::Up,
        },
        // Migration #36: Create diet_entries table
        Migration {
            version: 2,
            description: "create_diet_entries_table",
            sql: r#"
                CREATE TABLE IF NOT EXISTS diet_entries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    member_id INTEGER NOT NULL,
                    timestamp DATETIME NOT NULL,
                    meal_type TEXT NOT NULL,
                    description TEXT NOT NULL,
                    calories INTEGER,
                    notes TEXT,
                    FOREIGN KEY (member_id) REFERENCES family_members(id) ON DELETE CASCADE
                );

                CREATE INDEX IF NOT EXISTS idx_diet_entries_member_id ON diet_entries(member_id);
                CREATE INDEX IF NOT EXISTS idx_diet_entries_timestamp ON diet_entries(timestamp);
                CREATE INDEX IF NOT EXISTS idx_diet_entries_member_timestamp ON diet_entries(member_id, timestamp);
            "#,
            kind: MigrationKind::Up,
        },
    ];

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
            create_diet_entry,
            get_diet_entries,
            update_diet_entry,
            delete_diet_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
