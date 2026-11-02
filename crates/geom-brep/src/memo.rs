//! **A bounded thread-local memo of a pure function**, the one shape
//! the certification arithmetic's structural memos share (`carrier_net`'s
//! span bases, `pcurve_cache`'s rounds, first-reading spans and twin
//! span hulls).
//!
//! A memo here holds a pure function's values under a key that is the
//! function's whole input, compared whole (no hash stands for it, so no
//! collision returns another input's value: D9, a hit returns exactly
//! what a miss computes). It lives in a thread-local, so it needs no
//! owner threaded through the doors that read it, and an entry left
//! from another body or document is still that key's value, never
//! stale.
//!
//! **One capacity rule**: a memo holds at most [`CAPACITY`] entries and
//! is cleared whole on reaching it. Clearing whole rather than evicting
//! keeps the memo a map with no recency bookkeeping on the hit path; a
//! cleared memo only costs recomputes, never a different value.

use core::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread::LocalKey;

/// The most entries any memo holds before it is cleared.
pub(crate) const CAPACITY: usize = 4096;

/// A memo's entries and its hit and miss counts.
pub(crate) struct Memo<K, V> {
    map: HashMap<K, V>,
    counts: (u64, u64),
}

impl<K, V> Memo<K, V> {
    /// An empty memo.
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
            counts: (0, 0),
        }
    }
}

/// The memo's value at `key`, or `compute`'s, which is then kept
/// (clearing the memo first where it holds [`CAPACITY`] entries).
pub(crate) fn read<K: Eq + Hash, V: Clone>(
    memo: &'static LocalKey<RefCell<Memo<K, V>>>,
    key: K,
    compute: impl FnOnce() -> V,
) -> V {
    let hit = memo.with(|m| {
        let mut m = m.borrow_mut();
        let hit = m.map.get(&key).cloned();
        if hit.is_some() {
            m.counts.0 += 1;
        } else {
            m.counts.1 += 1;
        }
        hit
    });
    if let Some(hit) = hit {
        return hit;
    }
    let value = compute();
    memo.with(|m| {
        let mut m = m.borrow_mut();
        if m.map.len() >= CAPACITY {
            m.map.clear();
        }
        m.map.insert(key, value.clone());
    });
    value
}

/// This thread's hits and misses of `memo` so far.
#[cfg(test)]
pub(crate) fn counts<K, V>(memo: &'static LocalKey<RefCell<Memo<K, V>>>) -> (u64, u64) {
    memo.with(|m| m.borrow().counts)
}

/// Empties this thread's `memo`, so the next read of every key
/// computes.
#[cfg(test)]
pub(crate) fn clear<K, V>(memo: &'static LocalKey<RefCell<Memo<K, V>>>) {
    memo.with(|m| m.borrow_mut().map.clear());
}
