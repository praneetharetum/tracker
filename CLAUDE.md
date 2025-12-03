# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Install dependencies
npm install

# Development (runs Vite + Tauri together)
source ~/.cargo/env && npm run tauri:dev

# Production build
npm run tauri:build
```

## Architecture

Tauri 2 desktop application with React frontend and Rust backend.

**Frontend (`src/`):** React 18 + TypeScript + Vite
- `src/App.tsx` - Main component
- `src/main.tsx` - React entry point
- `src/db.ts` - Database service layer (SQLite CRUD operations)

**Backend (`src-tauri/src/`):** Rust + Tauri
- `src-tauri/src/lib.rs` - Tauri commands, app initialization, and database migrations
- `src-tauri/src/main.rs` - Entry point

**Configuration:**
- `src-tauri/tauri.conf.json` - App config (window size, bundle settings, plugins)
- `vite.config.ts` - Frontend build config (dev server on port 5173)

## Database

Uses SQLite via `tauri-plugin-sql` for local data persistence.

**Database file:** `tracker.db` (created automatically in app data directory)

**Schema:**
- `family_members` - Stores family member profiles (id, name, icon, created_at)
- `tracked_items` - Stores tracked data per family member (id, family_member_id, name, category, value, notes, tracked_at)

**Usage from TypeScript:**
```typescript
import { getFamilyMembers, createTrackedItem } from './db'

// Get all family members
const members = await getFamilyMembers()

// Create a tracked item
await createTrackedItem(memberId, 'Item name', 'category', 10.5, 'notes')
```

**Migrations:** Defined in `src-tauri/src/lib.rs` and run automatically on app startup.

## Adding Tauri Commands

Define Rust commands in `src-tauri/src/lib.rs`:

```rust
#[tauri::command]
fn my_command(arg: &str) -> String {
    format!("Result: {}", arg)
}
```

Register in the invoke handler:
```rust
.invoke_handler(tauri::generate_handler![greet, my_command])
```

Call from React:
```typescript
import { invoke } from '@tauri-apps/api/core'
const result = await invoke('my_command', { arg: 'value' })
```

## Decisions

### Issue #35: Local Database Storage

**Decision:** Use `tauri-plugin-sql` with SQLite backend.

**Rationale:**
- Official Tauri plugin with first-class support for Tauri 2
- SQLite is lightweight, requires no external server, and stores data locally in a single file
- Built-in migration system for schema evolution
- Type-safe queries from TypeScript via the plugin API

**Alternatives considered:**
- IndexedDB: Browser-based, would require custom bridge; less suitable for desktop apps
- File-based JSON: Simpler but lacks querying capabilities and ACID guarantees
- External database (Postgres/MySQL): Overkill for local desktop app, requires server setup
