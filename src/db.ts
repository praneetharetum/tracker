import Database from '@tauri-apps/plugin-sql';

export interface FamilyMember {
  id: number;
  name: string;
  icon: string;
  created_at: string;
}

export interface TrackedItem {
  id: number;
  family_member_id: number;
  name: string;
  category: string | null;
  value: number | null;
  notes: string | null;
  tracked_at: string;
}

let db: Database | null = null;

async function getDb(): Promise<Database> {
  if (!db) {
    db = await Database.load('sqlite:tracker.db');
  }
  return db;
}

// Family Members CRUD
export async function getFamilyMembers(): Promise<FamilyMember[]> {
  const database = await getDb();
  return database.select<FamilyMember[]>('SELECT * FROM family_members ORDER BY name');
}

export async function getFamilyMember(id: number): Promise<FamilyMember | null> {
  const database = await getDb();
  const results = await database.select<FamilyMember[]>(
    'SELECT * FROM family_members WHERE id = $1',
    [id]
  );
  return results[0] || null;
}

export async function createFamilyMember(name: string, icon: string): Promise<number> {
  const database = await getDb();
  const result = await database.execute(
    'INSERT INTO family_members (name, icon) VALUES ($1, $2)',
    [name, icon]
  );
  return result.lastInsertId as number;
}

export async function updateFamilyMember(id: number, name: string, icon: string): Promise<void> {
  const database = await getDb();
  await database.execute(
    'UPDATE family_members SET name = $1, icon = $2 WHERE id = $3',
    [name, icon, id]
  );
}

export async function deleteFamilyMember(id: number): Promise<void> {
  const database = await getDb();
  await database.execute('DELETE FROM family_members WHERE id = $1', [id]);
}

// Tracked Items CRUD
export async function getTrackedItems(familyMemberId?: number): Promise<TrackedItem[]> {
  const database = await getDb();
  if (familyMemberId !== undefined) {
    return database.select<TrackedItem[]>(
      'SELECT * FROM tracked_items WHERE family_member_id = $1 ORDER BY tracked_at DESC',
      [familyMemberId]
    );
  }
  return database.select<TrackedItem[]>('SELECT * FROM tracked_items ORDER BY tracked_at DESC');
}

export async function getTrackedItem(id: number): Promise<TrackedItem | null> {
  const database = await getDb();
  const results = await database.select<TrackedItem[]>(
    'SELECT * FROM tracked_items WHERE id = $1',
    [id]
  );
  return results[0] || null;
}

export async function createTrackedItem(
  familyMemberId: number,
  name: string,
  category?: string,
  value?: number,
  notes?: string
): Promise<number> {
  const database = await getDb();
  const result = await database.execute(
    'INSERT INTO tracked_items (family_member_id, name, category, value, notes) VALUES ($1, $2, $3, $4, $5)',
    [familyMemberId, name, category ?? null, value ?? null, notes ?? null]
  );
  return result.lastInsertId as number;
}

export async function updateTrackedItem(
  id: number,
  name: string,
  category?: string,
  value?: number,
  notes?: string
): Promise<void> {
  const database = await getDb();
  await database.execute(
    'UPDATE tracked_items SET name = $1, category = $2, value = $3, notes = $4 WHERE id = $5',
    [name, category ?? null, value ?? null, notes ?? null, id]
  );
}

export async function deleteTrackedItem(id: number): Promise<void> {
  const database = await getDb();
  await database.execute('DELETE FROM tracked_items WHERE id = $1', [id]);
}

// Seed default family members if none exist
export async function seedDefaultFamilyMembers(): Promise<void> {
  const members = await getFamilyMembers();
  if (members.length === 0) {
    await createFamilyMember('Mom', '👩');
    await createFamilyMember('Dad', '👨');
    await createFamilyMember('Child', '🧒');
  }
}
