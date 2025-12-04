use tauri_plugin_sql::{Migration, MigrationKind};

/// Returns all database migrations
pub fn get_migrations() -> Vec<Migration> {
    vec![
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
    ]
}
