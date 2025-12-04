# Backend (src-tauri/)

Rust + Tauri 2 backend for the Tracker desktop application.

## File Structure

```
src-tauri/
├── src/
│   ├── lib.rs           # App setup, re-exports
│   ├── main.rs          # Entry point (calls tracker_lib::run())
│   ├── db/
│   │   ├── mod.rs       # DbPath state, module exports
│   │   └── migrations.rs # Database migrations
│   └── diet/
│       ├── mod.rs       # Module exports
│       ├── models.rs    # MealType, DietEntry, request/response types
│       └── commands.rs  # Tauri commands (create, get, update, delete)
├── build.rs             # Tauri build script
├── Cargo.toml           # Rust dependencies
├── Cargo.lock           # Locked dependency versions
├── tauri.conf.json      # Tauri app configuration
└── icons/               # App icons for all platforms
```

## Dependencies (Cargo.toml)

- `tauri` v2 - Core framework with devtools feature
- `tauri-plugin-shell` v2 - Shell command execution
- `tauri-plugin-sql` v2 - SQLite database via plugin
- `serde` v1 - JSON serialization/deserialization
- `serde_json` v1 - JSON parsing
- `rusqlite` v0.31 - Direct SQLite access for Tauri commands

## Database Schema

### Tables

**family_members** (Migration v1)
| Column | Type | Constraints |
|--------|------|-------------|
| id | INTEGER | PRIMARY KEY AUTOINCREMENT |
| name | TEXT | NOT NULL |
| icon | TEXT | NOT NULL |
| created_at | DATETIME | DEFAULT CURRENT_TIMESTAMP |

**tracked_items** (Migration v1)
| Column | Type | Constraints |
|--------|------|-------------|
| id | INTEGER | PRIMARY KEY AUTOINCREMENT |
| family_member_id | INTEGER | NOT NULL, FK -> family_members(id) CASCADE |
| name | TEXT | NOT NULL |
| category | TEXT | nullable |
| value | REAL | nullable |
| notes | TEXT | nullable |
| tracked_at | DATETIME | DEFAULT CURRENT_TIMESTAMP |

**diet_entries** (Migration v2)
| Column | Type | Constraints |
|--------|------|-------------|
| id | INTEGER | PRIMARY KEY AUTOINCREMENT |
| member_id | INTEGER | NOT NULL, FK -> family_members(id) CASCADE |
| timestamp | DATETIME | NOT NULL |
| meal_type | TEXT | NOT NULL (Breakfast/Lunch/Dinner/Snack/Other) |
| description | TEXT | NOT NULL |
| calories | INTEGER | nullable |
| notes | TEXT | nullable |

**Indexes:** `idx_diet_entries_member_id`, `idx_diet_entries_timestamp`, `idx_diet_entries_member_timestamp`

## Modules

### db/ - Database Module
- `db::DbPath` - Managed state holding database file path
- `db::migrations::get_migrations()` - Returns all database migrations

### diet/ - Diet Entry Module
**Models (diet/models.rs):**
- `MealType` enum - Breakfast, Lunch, Dinner, Snack, Other
- `DietEntry` - Database row representation
- `CreateDietEntryRequest` - Input for create command
- `UpdateDietEntryRequest` - Input for update command
- `DietEntryFilter` - Query filters (member_id, start_date, end_date)

**Commands (diet/commands.rs):**
- `create_diet_entry` - Creates a new diet entry with member validation
- `get_diet_entries` - Retrieves entries with optional filters
- `update_diet_entry` - Updates an existing entry
- `delete_diet_entry` - Deletes an entry

## Tauri Commands

### greet(name: &str) -> String
Demo command returning a greeting message.

### diet::create_diet_entry(...) -> Result<DietEntry, String>
Creates a new diet entry with validation.

**Parameters:**
- `member_id: i64` - Must exist in family_members
- `timestamp: String` - ISO 8601 UTC datetime
- `meal_type: String` - One of: Breakfast, Lunch, Dinner, Snack, Other
- `description: String` - Meal description
- `calories: Option<i64>` - Optional calorie count
- `notes: Option<String>` - Optional notes

**Returns:** Created `DietEntry` with generated ID or error string.

### diet::get_diet_entries(...) -> Result<Vec<DietEntry>, String>
Retrieves diet entries with optional filters.

**Parameters:**
- `member_id: Option<i64>` - Filter by family member
- `start_date: Option<String>` - Filter entries >= this timestamp
- `end_date: Option<String>` - Filter entries <= this timestamp

**Returns:** Array of `DietEntry` sorted by timestamp DESC.

### diet::update_diet_entry(...) -> Result<DietEntry, String>
Updates an existing diet entry. Only provided fields are changed.

**Parameters:**
- `id: i64` - Required. Entry ID to update
- `member_id: Option<i64>` - New member (validated if provided)
- `timestamp: Option<String>` - New timestamp
- `meal_type: Option<String>` - New meal type
- `description: Option<String>` - New description
- `calories: Option<i64>` - New calorie count
- `notes: Option<String>` - New notes

**Returns:** Updated `DietEntry` or error if not found.

### diet::delete_diet_entry(id: i64) -> Result<(), String>
Deletes a diet entry.

**Parameters:**
- `id: i64` - Entry ID to delete

**Returns:** Success or error if not found.

## App Initialization

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_sql::Builder::default()
        .add_migrations("sqlite:tracker.db", db::migrations::get_migrations())
        .build())
    .setup(|app| {
        let app_data_dir = app.path().app_data_dir()?;
        app.manage(DbPath(app_data_dir.join("tracker.db")));
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
```

## Adding New Commands

1. Add command to appropriate module (e.g., `diet/commands.rs`):
```rust
#[tauri::command]
pub async fn my_command(param: String) -> Result<MyResponse, String> {
    // Implementation
}
```

2. Export from module's `mod.rs`:
```rust
pub use commands::my_command;
```

3. Register in `lib.rs` invoke_handler:
```rust
.invoke_handler(tauri::generate_handler![..., diet::my_command])
```

## Adding Migrations

Add new migration in `db/migrations.rs`:
```rust
Migration {
    version: 3,  // Increment version
    description: "description_here",
    sql: r#"-- SQL statements"#,
    kind: MigrationKind::Up,
},
```

## Configuration (tauri.conf.json)

- **Window:** 800x600, resizable, not fullscreen
- **Dev URL:** http://localhost:5173
- **Frontend dist:** ../dist
- **Plugins:** shell (open), sql (preload sqlite:tracker.db)
- **Bundle:** All targets enabled with platform-specific icons

## Build Commands

```bash
# Check compilation
cargo check

# Build debug
cargo build

# Build release (optimized, stripped)
cargo build --release
```

Release profile enables LTO, single codegen unit, size optimization, and symbol stripping.
