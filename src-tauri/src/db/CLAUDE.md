# Database Module (src-tauri/src/db/)

Database configuration and migrations for the Tracker application.

## Files

- `mod.rs` - Module exports and `DbPath` state struct
- `migrations.rs` - Database migration definitions

## DbPath State

```rust
pub struct DbPath(pub String);
```

Managed Tauri state holding the SQLite database file path. Initialized in `lib.rs` setup with the app data directory path.

## Migrations

### get_migrations() -> Vec<Migration>

Returns all database migrations to be applied on startup.

**Migration v1: Initial Tables**
- `family_members` - User profiles (id, name, icon, created_at)
- `tracked_items` - Generic tracked items with FK to family_members

**Migration v2: Diet Entries**
- `diet_entries` - Diet/meal tracking with member FK
- Indexes on member_id, timestamp, and composite

## Adding New Migrations

1. Add to `migrations.rs`:
```rust
Migration {
    version: 3,
    description: "your_migration_name",
    sql: r#"
        -- SQL statements here
    "#,
    kind: MigrationKind::Up,
},
```

2. Migrations run automatically on app startup via `tauri-plugin-sql`

## Usage

```rust
// In lib.rs
use crate::db;

let migrations = db::migrations::get_migrations();
// Pass to tauri_plugin_sql builder

// In commands
use crate::db::DbPath;
fn my_command(db_path: State<'_, DbPath>) -> Result<...> {
    let conn = rusqlite::Connection::open(&db_path.0)?;
    // ...
}
```
