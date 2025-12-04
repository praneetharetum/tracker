use tauri::State;

use crate::db::DbPath;
use super::models::{DietEntry, MealType};

/// Create a new diet entry (#38)
#[tauri::command]
pub async fn create_diet_entry(
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
pub async fn get_diet_entries(
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
pub async fn update_diet_entry(
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
pub async fn delete_diet_entry(db_path: State<'_, DbPath>, id: i64) -> Result<(), String> {
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
