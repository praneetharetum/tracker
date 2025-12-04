use rusqlite::Connection;

fn setup_test_db() -> Connection {
    let db = Connection::open_in_memory().expect("Failed to create in-memory database");

    // Create tables
    db.execute_batch(r#"
        CREATE TABLE IF NOT EXISTS family_members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            icon TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

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

        -- Insert a test family member
        INSERT INTO family_members (name, icon) VALUES ('Test User', '👤');
    "#).expect("Failed to create tables");

    db
}

#[test]
fn test_create_diet_entry() {
    let db = setup_test_db();

    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description, calories, notes) VALUES (?, ?, ?, ?, ?, ?)",
        rusqlite::params![1, "2024-01-15T12:00:00Z", "Lunch", "Test meal", 500, "Test notes"],
    ).expect("Failed to insert diet entry");

    let id = db.last_insert_rowid();
    assert!(id > 0, "Should have created entry with valid ID");

    // Verify entry exists
    let count: i64 = db.query_row(
        "SELECT COUNT(*) FROM diet_entries WHERE id = ?",
        [id],
        |row| row.get(0)
    ).expect("Failed to count entries");

    assert_eq!(count, 1, "Should have exactly one entry");
}

#[test]
fn test_get_diet_entries_no_filter() {
    let db = setup_test_db();

    // Insert multiple entries
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description) VALUES (1, '2024-01-15T08:00:00Z', 'Breakfast', 'Entry 1')",
        [],
    ).unwrap();
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description) VALUES (1, '2024-01-15T12:00:00Z', 'Lunch', 'Entry 2')",
        [],
    ).unwrap();

    let mut stmt = db.prepare("SELECT id FROM diet_entries ORDER BY timestamp DESC").unwrap();
    let entries: Vec<i64> = stmt.query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(entries.len(), 2, "Should return all entries");
}

#[test]
fn test_get_diet_entries_filter_by_member() {
    let db = setup_test_db();

    // Add another member
    db.execute("INSERT INTO family_members (name, icon) VALUES ('User 2', '👥')", []).unwrap();

    // Insert entries for different members
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description) VALUES (1, '2024-01-15T08:00:00Z', 'Breakfast', 'Member 1 entry')",
        [],
    ).unwrap();
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description) VALUES (2, '2024-01-15T12:00:00Z', 'Lunch', 'Member 2 entry')",
        [],
    ).unwrap();

    let mut stmt = db.prepare("SELECT id FROM diet_entries WHERE member_id = ? ORDER BY timestamp DESC").unwrap();
    let entries: Vec<i64> = stmt.query_map([1], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(entries.len(), 1, "Should return only member 1's entries");
}

#[test]
fn test_get_diet_entries_filter_by_date_range() {
    let db = setup_test_db();

    // Insert entries on different dates
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description) VALUES (1, '2024-01-14T12:00:00Z', 'Lunch', 'Yesterday')",
        [],
    ).unwrap();
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description) VALUES (1, '2024-01-15T12:00:00Z', 'Lunch', 'Today')",
        [],
    ).unwrap();
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description) VALUES (1, '2024-01-16T12:00:00Z', 'Lunch', 'Tomorrow')",
        [],
    ).unwrap();

    let mut stmt = db.prepare(
        "SELECT id FROM diet_entries WHERE timestamp >= ? AND timestamp <= ? ORDER BY timestamp DESC"
    ).unwrap();
    let entries: Vec<i64> = stmt.query_map(["2024-01-15T00:00:00Z", "2024-01-15T23:59:59Z"], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(entries.len(), 1, "Should return only today's entries");
}

#[test]
fn test_update_diet_entry() {
    let db = setup_test_db();

    // Create entry
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description, calories) VALUES (1, '2024-01-15T12:00:00Z', 'Lunch', 'Original', 500)",
        [],
    ).unwrap();
    let id = db.last_insert_rowid();

    // Update entry
    db.execute(
        "UPDATE diet_entries SET description = ?, calories = ? WHERE id = ?",
        rusqlite::params!["Updated", 600, id],
    ).unwrap();

    // Verify update
    let (desc, cal): (String, i64) = db.query_row(
        "SELECT description, calories FROM diet_entries WHERE id = ?",
        [id],
        |row| Ok((row.get(0)?, row.get(1)?))
    ).unwrap();

    assert_eq!(desc, "Updated");
    assert_eq!(cal, 600);
}

#[test]
fn test_update_diet_entry_not_found() {
    let db = setup_test_db();

    let rows_affected = db.execute(
        "UPDATE diet_entries SET description = ? WHERE id = ?",
        rusqlite::params!["Updated", 99999],
    ).unwrap();

    assert_eq!(rows_affected, 0, "Should not update any rows");
}

#[test]
fn test_delete_diet_entry() {
    let db = setup_test_db();

    // Create entry
    db.execute(
        "INSERT INTO diet_entries (member_id, timestamp, meal_type, description) VALUES (1, '2024-01-15T12:00:00Z', 'Lunch', 'To delete')",
        [],
    ).unwrap();
    let id = db.last_insert_rowid();

    // Delete entry
    db.execute("DELETE FROM diet_entries WHERE id = ?", [id]).unwrap();

    // Verify deletion
    let count: i64 = db.query_row(
        "SELECT COUNT(*) FROM diet_entries WHERE id = ?",
        [id],
        |row| row.get(0)
    ).unwrap();

    assert_eq!(count, 0, "Entry should be deleted");
}

#[test]
fn test_delete_diet_entry_not_found() {
    let db = setup_test_db();

    // Check non-existent entry
    let exists: bool = db.query_row(
        "SELECT EXISTS(SELECT 1 FROM diet_entries WHERE id = ?)",
        [99999],
        |row| row.get(0)
    ).unwrap();

    assert!(!exists, "Entry should not exist");
}
