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

Use Tauri's invoke API to call Rust commands:
```typescript
import { invoke } from '@tauri-apps/api/core';

// Example: Create a diet entry
const entry = await invoke('create_diet_entry', {
  memberId: 1,
  timestamp: '2024-01-15T12:00:00Z',
  mealType: 'Lunch',
  description: 'Grilled chicken salad',
  calories: 450,
  notes: 'High protein meal'
});
```

## Development

Frontend runs on Vite dev server at `http://localhost:5173` during development.
