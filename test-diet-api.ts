// Test script for diet entry API
// Run with: npx ts-node test-diet-api.ts (requires app to be running)

import { invoke } from '@tauri-apps/api/core';

interface DietEntry {
  id: number;
  member_id: number;
  timestamp: string;
  meal_type: string;
  description: string;
  calories: number | null;
  notes: string | null;
}

async function runTests() {
  console.log('=== Diet Entry API Tests ===\n');

  let createdId: number;

  // First, ensure we have a family member to use
  try {
    // Test 1: Create a diet entry
    console.log('Test 1: Create diet entry...');
    const created = await invoke<DietEntry>('create_diet_entry', {
      memberId: 1,
      timestamp: new Date().toISOString(),
      mealType: 'Lunch',
      description: 'Test meal',
      calories: 500,
      notes: 'Test notes'
    });
    createdId = created.id;
    console.log('✓ Created entry with ID:', createdId);
    console.log('  Entry:', JSON.stringify(created, null, 2));

    // Test 2: Get all entries (no filter)
    console.log('\nTest 2: Get all entries (no filter)...');
    const allEntries = await invoke<DietEntry[]>('get_diet_entries', {
      memberId: null,
      startDate: null,
      endDate: null
    });
    console.log('✓ Got', allEntries.length, 'entries');

    // Test 3: Get entries filtered by member
    console.log('\nTest 3: Get entries filtered by member_id=1...');
    const filteredByMember = await invoke<DietEntry[]>('get_diet_entries', {
      memberId: 1,
      startDate: null,
      endDate: null
    });
    console.log('✓ Got', filteredByMember.length, 'entries for member 1');

    // Test 4: Get entries filtered by date range
    console.log('\nTest 4: Get entries filtered by date range...');
    const today = new Date().toISOString().split('T')[0];
    const filteredByDate = await invoke<DietEntry[]>('get_diet_entries', {
      memberId: null,
      startDate: today + 'T00:00:00Z',
      endDate: today + 'T23:59:59Z'
    });
    console.log('✓ Got', filteredByDate.length, 'entries for today');

    // Test 5: Update the entry
    console.log('\nTest 5: Update entry...');
    const updated = await invoke<DietEntry>('update_diet_entry', {
      id: createdId,
      memberId: null,
      timestamp: null,
      mealType: null,
      description: 'Updated test meal',
      calories: 600,
      notes: 'Updated notes'
    });
    console.log('✓ Updated entry');
    console.log('  New description:', updated.description);
    console.log('  New calories:', updated.calories);

    // Test 6: Update non-existent entry (should fail)
    console.log('\nTest 6: Update non-existent entry (should fail)...');
    try {
      await invoke<DietEntry>('update_diet_entry', {
        id: 99999,
        memberId: null,
        timestamp: null,
        mealType: null,
        description: 'Should fail',
        calories: null,
        notes: null
      });
      console.log('✗ Should have thrown error');
    } catch (e) {
      console.log('✓ Correctly returned error:', e);
    }

    // Test 7: Delete the entry
    console.log('\nTest 7: Delete entry...');
    await invoke<void>('delete_diet_entry', { id: createdId });
    console.log('✓ Deleted entry', createdId);

    // Test 8: Delete non-existent entry (should fail)
    console.log('\nTest 8: Delete non-existent entry (should fail)...');
    try {
      await invoke<void>('delete_diet_entry', { id: 99999 });
      console.log('✗ Should have thrown error');
    } catch (e) {
      console.log('✓ Correctly returned error:', e);
    }

    // Test 9: Verify entry was deleted
    console.log('\nTest 9: Verify entry was deleted...');
    const afterDelete = await invoke<DietEntry[]>('get_diet_entries', {
      memberId: null,
      startDate: null,
      endDate: null
    });
    const found = afterDelete.find(e => e.id === createdId);
    if (!found) {
      console.log('✓ Entry correctly removed from database');
    } else {
      console.log('✗ Entry still exists in database');
    }

    console.log('\n=== All tests completed ===');

  } catch (error) {
    console.error('Test failed:', error);
  }
}

runTests();
