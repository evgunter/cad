//! **What a spline remembers of its own readings.** Two kinds, both
//! functions of fields nothing mutates after construction, so neither can
//! go stale:
//!
//! - a meter read once per value ([`session_memo`]);
//! - an opt-in table of restrictions keyed by a parameter range's exact
//!   bits ([`Table`], [`kept`]), switched on only for a chart that lives
//!   as long as one question.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Mutex, OnceLock, PoisonError};

/// A spline's memoised meter: `compute` afresh inside a symbolic
/// session, `cell`'s value outside one (filled on first read). A
/// session's table holds the nodes it saw built, and a node minted
/// anywhere else freezes in it to an unknown of its own
/// (`geom_core::sym`, "No session, no tier"), so a memo served across
/// that boundary would change what the session decides; inside one
/// nothing the meter mints is kept.
pub(crate) fn session_memo<V: Copy>(cell: &OnceLock<V>, compute: impl FnOnce() -> V) -> V {
    if geom_core::sym::session_installed() {
        return compute();
    }
    *cell.get_or_init(compute)
}

/// Readings keyed by their arguments' exact bits: two keys are one
/// entry only when every bound is the same `f64`.
pub(crate) struct Table<K, V>(Mutex<HashMap<K, V>>);

impl<K, V> Default for Table<K, V> {
    fn default() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

impl<K: Eq + Hash, V: Clone> Table<K, V> {
    /// `self[key]`, computed on its first read and served after. The
    /// lock is not held while `compute` runs, so a race computes twice
    /// and keeps one of two equal answers.
    fn get_or(&self, key: K, compute: impl FnOnce() -> V) -> V {
        let lock = || self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(hit) = lock().get(&key) {
            return hit.clone();
        }
        let value = compute();
        lock().entry(key).or_insert(value).clone()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).len()
    }
}

/// `read()` through `memo`'s table for `key` where a memo is kept, and
/// afresh where none is.
pub(crate) fn kept<M, K: Eq + Hash, V: Clone>(
    memo: Option<&M>,
    table: impl FnOnce(&M) -> &Table<K, V>,
    key: K,
    read: impl FnOnce() -> V,
) -> V {
    match memo {
        Some(m) => table(m).get_or(key, read),
        None => read(),
    }
}
