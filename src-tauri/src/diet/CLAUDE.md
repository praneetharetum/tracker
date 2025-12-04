# Diet Module (src-tauri/src/diet/)

Diet entry management for tracking meals and nutrition.

## Files

- `mod.rs` - Module exports and re-exports
- `models.rs` - Data types (MealType, DietEntry, request/filter structs)
- `commands.rs` - Tauri commands for CRUD operations

## Models (models.rs)

### MealType
```rust
pub enum MealType {
    Breakfast, Lunch, Dinner, Snack, Other
}
```
Implements `Display`, `FromStr`, `Serialize`, `Deserialize`.

### DietEntry
```rust
pub struct DietEntry {
    pub id: i64,
    pub member_id: i64,
    pub timestamp: String,      // ISO 8601 UTC
    pub meal_type: MealType,
    pub description: String,
    pub calories: Option<i64>,
    pub notes: Option<String>,
}
```

### Request Types
- `CreateDietEntryRequest` - All required fields for creation
- `UpdateDietEntryRequest` - All fields optional except id
- `DietEntryFilter` - Optional member_id, start_date, end_date

## Commands (commands.rs)

### create_diet_entry
Creates a new diet entry with member validation.

### get_diet_entries
Retrieves entries with optional filtering by member and date range.

### update_diet_entry
Partial update - only modifies provided fields.

### delete_diet_entry
Removes entry with existence validation.

## Usage

Commands are registered in `lib.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    diet::create_diet_entry,
    diet::get_diet_entries,
    diet::update_diet_entry,
    diet::delete_diet_entry
])
```

Called from frontend:
```typescript
import { invoke } from '@tauri-apps/api/core';
await invoke('create_diet_entry', { memberId: 1, ... });
```

## Adding New Commands

1. Add function in `commands.rs`:
```rust
#[tauri::command]
pub async fn new_command(...) -> Result<T, String> {
    // implementation
}
```

2. Export in `mod.rs`:
```rust
pub use commands::new_command;
```

3. Register in `lib.rs` invoke_handler
