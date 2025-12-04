# Backend (src-tauri/)

Rust + Tauri 2 backend for the Tracker desktop application.

## File Structure

```
src-tauri/
├── src/
│   ├── lib.rs        # Main library: commands, migrations, data models
│   └── main.rs       # Entry point (calls tracker_lib::run())
├── build.rs          # Tauri build script
├── Cargo.toml        # Rust dependencies
├── Cargo.lock        # Locked dependency versions
├── tauri.conf.json   # Tauri app configuration
└── icons/            # App icons for all platforms
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

## Data Models (lib.rs)

### MealType Enum
```rust
pub enum MealType {
    Breakfast, Lunch, Dinner, Snack, Other
}
```
Implements `Display`, `FromStr`, `Serialize`, `Deserialize`.

### Structs
- `DietEntry` - Database row representation
- `CreateDietEntryRequest` - Input for create command
- `UpdateDietEntryRequest` - Input for update command (all fields optional except id)
- `DietEntryFilter` - Query filters (member_id, start_date, end_date)
- `DbPath` - Managed state holding database file path

## Tauri Commands

### greet(name: &str) -> String
Demo command returning a greeting message.

### create_diet_entry(...) -> Result<DietEntry, String>
Creates a new diet entry with validation.

**Parameters:**
- `member_id: i64` - Must exist in family_members
- `timestamp: String` - ISO 8601 UTC datetime
- `meal_type: String` - One of: Breakfast, Lunch, Dinner, Snack, Other
- `description: String` - Meal description
- `calories: Option<i64>` - Optional calorie count
- `notes: Option<String>` - Optional notes

**Returns:** Created `DietEntry` with generated ID or error string.

## App Initialization

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_sql::Builder::default()
        .add_migrations("sqlite:tracker.db", migrations)
        .build())
    .setup(|app| {
        // Set up DbPath state with app data directory
        let app_data_dir = app.path().app_data_dir()?;
        app.manage(DbPath(app_data_dir.join("tracker.db")));
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet, create_diet_entry])
    .run(tauri::generate_context!())
```

## Adding New Commands

1. Define the command function with `#[tauri::command]`:
```rust
#[tauri::command]
async fn my_command(param: String) -> Result<MyResponse, String> {
    // Implementation
}
```

2. Register in `invoke_handler`:
```rust
.invoke_handler(tauri::generate_handler![greet, create_diet_entry, my_command])
```

3. Call from frontend:
```typescript
const result = await invoke('my_command', { param: 'value' });
```

## Adding Migrations

Add new `Migration` to the `migrations` vec in `run()`:
```rust
Migration {
    version: 3,  // Increment version
    description: "description_here",
    sql: r#"
        -- SQL statements
    "#,
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
