import { invoke } from '@tauri-apps/api/core';

// ============================================================================
// Types (#41)
// ============================================================================

/** Meal type matching Rust MealType enum */
export type MealType = 'Breakfast' | 'Lunch' | 'Dinner' | 'Snack' | 'Other';

/** Diet entry as stored in the database */
export interface DietEntry {
  id: number;
  member_id: number;
  timestamp: string;
  meal_type: MealType;
  description: string;
  calories: number | null;
  notes: string | null;
}

/** Parameters for creating a new diet entry */
export interface CreateDietEntryParams {
  memberId: number;
  timestamp: string;
  mealType: MealType;
  description: string;
  calories?: number | null;
  notes?: string | null;
}

/** Parameters for updating an existing diet entry */
export interface UpdateDietEntryParams {
  memberId?: number;
  timestamp?: string;
  mealType?: MealType;
  description?: string;
  calories?: number | null;
  notes?: string | null;
}

/** Filter parameters for querying diet entries */
export interface DietEntryFilter {
  memberId?: number;
  startDate?: string;
  endDate?: string;
}

// ============================================================================
// API Functions (#41)
// ============================================================================

/**
 * Create a new diet entry
 * @param params - The diet entry data
 * @returns The created DietEntry with generated ID
 */
export async function createDietEntry(params: CreateDietEntryParams): Promise<DietEntry> {
  return invoke<DietEntry>('create_diet_entry', {
    memberId: params.memberId,
    timestamp: params.timestamp,
    mealType: params.mealType,
    description: params.description,
    calories: params.calories ?? null,
    notes: params.notes ?? null,
  });
}

/**
 * Get diet entries with optional filters
 * @param filters - Optional filter criteria
 * @returns Array of DietEntry objects sorted by timestamp DESC
 */
export async function getDietEntries(filters?: DietEntryFilter): Promise<DietEntry[]> {
  return invoke<DietEntry[]>('get_diet_entries', {
    memberId: filters?.memberId ?? null,
    startDate: filters?.startDate ?? null,
    endDate: filters?.endDate ?? null,
  });
}

/**
 * Update an existing diet entry
 * @param id - The ID of the entry to update
 * @param params - The fields to update (only provided fields are changed)
 * @returns The updated DietEntry
 */
export async function updateDietEntry(
  id: number,
  params: UpdateDietEntryParams
): Promise<DietEntry> {
  return invoke<DietEntry>('update_diet_entry', {
    id,
    memberId: params.memberId ?? null,
    timestamp: params.timestamp ?? null,
    mealType: params.mealType ?? null,
    description: params.description ?? null,
    calories: params.calories ?? null,
    notes: params.notes ?? null,
  });
}

/**
 * Delete a diet entry
 * @param id - The ID of the entry to delete
 */
export async function deleteDietEntry(id: number): Promise<void> {
  return invoke<void>('delete_diet_entry', { id });
}
