# Frontend (src/)

React 18 + TypeScript frontend for the Tracker desktop application.

## File Structure

```
src/
├── main.tsx      # React entry point, renders App into #root
├── App.tsx       # Main application component
├── App.css       # Application styles (light/dark theme support)
├── index.css     # Global styles and CSS reset
├── db.ts         # Database service layer for SQLite operations
├── diet/
│   └── index.ts  # Diet entry API (types + Tauri command wrappers)
└── vite-env.d.ts # Vite TypeScript declarations
```

## Key Files

### main.tsx
Entry point that mounts the React app with StrictMode into the DOM.

### App.tsx
Main component displaying a family member selection UI with emoji icons (Mom, Dad, Child). Currently logs selections to console.

### db.ts
Database service layer providing typed CRUD operations via `@tauri-apps/plugin-sql`:

**Interfaces:**
- `FamilyMember` - id, name, icon, created_at
- `TrackedItem` - id, family_member_id, name, category, value, notes, tracked_at

**Functions:**
- `getFamilyMembers()` / `getFamilyMember(id)` - Read family members
- `createFamilyMember(name, icon)` - Create with auto-generated ID
- `updateFamilyMember(id, name, icon)` - Update existing
- `deleteFamilyMember(id)` - Delete (cascades to tracked_items)
- `getTrackedItems(familyMemberId?)` - Get all or filtered by member
- `getTrackedItem(id)` - Get single item
- `createTrackedItem(familyMemberId, name, category?, value?, notes?)` - Create item
- `updateTrackedItem(id, name, category?, value?, notes?)` - Update item
- `deleteTrackedItem(id)` - Delete item
- `seedDefaultFamilyMembers()` - Seeds Mom, Dad, Child if no members exist

### diet/index.ts
Diet entry service layer using Tauri commands (via `invoke`):

**Types:**
- `MealType` - 'Breakfast' | 'Lunch' | 'Dinner' | 'Snack' | 'Other'
- `DietEntry` - id, member_id, timestamp, meal_type, description, calories, notes
- `CreateDietEntryParams` - Input for creating entries
- `UpdateDietEntryParams` - Input for updating entries (all fields optional)
- `DietEntryFilter` - Query filters (memberId, startDate, endDate)

**Functions:**
- `createDietEntry(params)` - Create new diet entry
- `getDietEntries(filters?)` - Get entries with optional filters
- `updateDietEntry(id, params)` - Update existing entry
- `deleteDietEntry(id)` - Delete entry

## Database Access Pattern

Uses singleton pattern for database connection:
```typescript
import Database from '@tauri-apps/plugin-sql';

let db: Database | null = null;
async function getDb(): Promise<Database> {
  if (!db) {
    db = await Database.load('sqlite:tracker.db');
  }
  return db;
}
```

All queries use parameterized statements with `$1`, `$2`, etc. placeholders.

## Styling

- Light theme by default, dark theme via `prefers-color-scheme: dark`
- Family member buttons use emoji icons with hover scale effects
- Responsive flexbox layout centered on screen

## Calling Backend Commands

Use the typed wrapper functions from `diet/`:
```typescript
import { createDietEntry, getDietEntries, updateDietEntry, deleteDietEntry } from './diet';

// Create a diet entry
const entry = await createDietEntry({
  memberId: 1,
  timestamp: '2024-01-15T12:00:00Z',
  mealType: 'Lunch',
  description: 'Grilled chicken salad',
  calories: 450,
  notes: 'High protein meal'
});

// Get entries with filters
const entries = await getDietEntries({ memberId: 1, startDate: '2024-01-01' });

// Update entry
await updateDietEntry(entry.id, { calories: 500 });

// Delete entry
await deleteDietEntry(entry.id);
```

Or use Tauri's invoke API directly:
```typescript
import { invoke } from '@tauri-apps/api/core';
const entry = await invoke('create_diet_entry', { memberId: 1, ... });
```

## Development

Frontend runs on Vite dev server at `http://localhost:5173` during development.
