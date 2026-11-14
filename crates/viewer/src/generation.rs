//! A request's identity across the three seams: a monotone counter and
//! nothing else.
//!
//! **This module depends on nothing** — not the kernel, not another
//! module of this crate — and that is the whole reason it is a module
//! rather than a type inside one. [`Generation`] is what every seam
//! keys its answers by and what the session, the frame policies, the
//! pick index and the app all compare; sited inside either seam it
//! makes every one of those readers import a seam to name a counter,
//! and it made `evalseam` and `pickcache` import each other
//! (`crates/viewer/README.md`, *Module boundaries*).
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

/// A request's identity: a monotone counter minted by the session on
/// every submit, and compared by every seam.
///
/// Distinct from the shipped evaluation `Epoch`, which identifies the
/// RUN. This identifies the REQUEST, and the session mints a fresh one
/// for every submit — including a re-submit of an unchanged document
/// (`SessionOp::Reevaluate`). That is deliberately stricter than
/// "identifies the document version": a result may land only against
/// the request that asked for it, so a run canceled and then re-asked
/// can never have its abandoned answer accepted for the new ask.
///
/// The counter is not readable: every consumer compares generations
/// and none displays one, so `Debug` is the only door to the number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generation(u64);

impl Generation {
    /// The first generation.
    pub const FIRST: Self = Self(0);

    /// The next generation after this one.
    ///
    /// Saturating, not wrapping. A wrap would make a stale result
    /// compare equal to the current request — the one thing this type
    /// exists to prevent — so the arithmetic that cannot produce that
    /// failure is the one to write. The ceiling it saturates at is not
    /// reachable: [`Generation::FIRST`] is zero and this is the only
    /// way to advance one, so getting there is 2^64 submits.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// **A value derived once per generation and key, and kept**: one slot,
/// answered while both still match and derived again otherwise — what
/// the session keeps for a question the panes ask every frame of the
/// committed document. Behind a mutex because the panes ask through a
/// shared borrow.
pub struct GenerationMemo<K, V>(std::sync::Mutex<Option<(Generation, K, V)>>);

impl<K, V> Default for GenerationMemo<K, V> {
    fn default() -> Self {
        Self(std::sync::Mutex::new(None))
    }
}

impl<K: PartialEq + Clone, V: Clone> GenerationMemo<K, V> {
    /// The value kept for `generation` and `key`, or `derive`'s, kept.
    pub fn get_or(&self, generation: Generation, key: &K, derive: impl FnOnce() -> V) -> V {
        let mut memo = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((at, kept, value)) = memo.as_ref()
            && *at == generation
            && kept == key
        {
            return value.clone();
        }
        let value = derive();
        *memo = Some((generation, key.clone(), value.clone()));
        value
    }
}

/// **Values derived once per generation and key, every key kept**: a
/// [`GenerationMemo`] for a question several rows of one frame ask under
/// different keys, which one slot would answer by deriving each again
/// every frame. A new generation clears every key.
pub struct GenerationMap<K, V>(
    std::sync::Mutex<Option<(Generation, std::collections::BTreeMap<K, V>)>>,
);

impl<K, V> Default for GenerationMap<K, V> {
    fn default() -> Self {
        Self(std::sync::Mutex::new(None))
    }
}

impl<K: Ord + Clone, V: Clone> GenerationMap<K, V> {
    /// The value kept for `generation` and `key`, or `derive`'s, kept.
    pub fn get_or(&self, generation: Generation, key: &K, derive: impl FnOnce() -> V) -> V {
        let mut memo = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let kept = match memo.as_mut() {
            Some((at, kept)) if *at == generation => kept,
            _ => {
                &mut memo
                    .insert((generation, std::collections::BTreeMap::new()))
                    .1
            }
        };
        kept.entry(key.clone()).or_insert_with(derive).clone()
    }
}
