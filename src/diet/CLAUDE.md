# Diet Module (src/diet/)

Frontend API for diet entry management via Tauri commands.

## Files

- `index.ts` - Types and API wrapper functions

## Types

### MealType
```typescript
type MealType = 'Breakfast' | 'Lunch' | 'Dinner' | 'Snack' | 'Other';
```

### DietEntry
```typescript
interface DietEntry {
  id: number;
  member_id: number;
  timestamp: string;      // ISO 8601 UTC
  meal_type: MealType;
  description: string;
  calories: number | null;
  notes: string | null;
}
```

### Request Types
- `CreateDietEntryParams` - Required: memberId, timestamp, mealType, description; Optional: calories, notes
- `UpdateDietEntryParams` - All fields optional
- `DietEntryFilter` - Optional: memberId, startDate, endDate

## Functions

### createDietEntry(params: CreateDietEntryParams): Promise<DietEntry>
Creates a new diet entry.

### getDietEntries(filters?: DietEntryFilter): Promise<DietEntry[]>
Retrieves entries with optional filtering. Returns sorted by timestamp DESC.

### updateDietEntry(id: number, params: UpdateDietEntryParams): Promise<DietEntry>
Updates an existing entry. Only provided fields are modified.

### deleteDietEntry(id: number): Promise<void>
Deletes an entry by ID.

## Usage

```typescript
import {
  createDietEntry,
  getDietEntries,
  updateDietEntry,
  deleteDietEntry,
  type DietEntry,
  type MealType
} from './diet';

// Create
const entry = await createDietEntry({
  memberId: 1,
  timestamp: new Date().toISOString(),
  mealType: 'Lunch',
  description: 'Grilled chicken salad',
  calories: 450
});

// Read with filters
const entries = await getDietEntries({
  memberId: 1,
  startDate: '2024-01-01'
});

// Update
await updateDietEntry(entry.id, { calories: 500 });

// Delete
await deleteDietEntry(entry.id);
```

## Implementation

All functions wrap Tauri's `invoke` API to call Rust backend commands:
```typescript
import { invoke } from '@tauri-apps/api/core';
return invoke<DietEntry>('create_diet_entry', { ... });
```
