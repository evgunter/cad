//! **Where a value with no expression gets its name.**
//!
//! [`Sym::opaque`](super::Sym::opaque) mints an unknown of its own, and
//! an unknown needs a name no other unknown has. The name is a pair:
//! the **mint scope** open on the minting thread, and the value's
//! **ordinal** in that scope, counted from zero in the order the scope
//! mints.
//!
//! A scope is opened by whoever owns a unit of work whose mints should
//! be named by that unit: the evaluation service opens one per document
//! node (and per member of a family), named by the node's id, so a
//! value a node mints is named by the node and its place in the node's
//! own walk, whatever ran before the node and in whichever replay. A
//! mint under no opened scope is in the thread's **root** scope.
//!
//! # One name, one value
//!
//! Two different reals under one name would be one unknown, and their
//! difference the zero polynomial: a `Zero` about two values that are
//! not equal. What keeps that from happening is argued per place two
//! values can meet.
//!
//! **Inside one session a name is minted once.** A scope's count is
//! kept by the session while the scope is closed, so a scope opened a
//! second time (a node evaluated twice in one session, a scope entered
//! from inside itself) continues where it stopped. A session's root
//! scope is its own, counting from zero: not the thread's root, so a
//! value minted under no scope before the session and the session's
//! first share no name.
//!
//! **Across two sessions a name is minted once each**, and may hold two
//! numbers: a node whose inputs differ between two replays mints
//! different values at the same ordinals. Those meet only where a value
//! built in one session is read in another, which is a node's result
//! served under its content key. The key hashes every expression and
//! number the node read and, through its inputs' keys, everything they
//! read; a value can reach a node only through an input; so a served
//! result holds a name only if the node that minted it has the same key
//! in both replays, and then it is not run in the serving one: the name
//! is minted there by nobody. A root-scope name reaches a node's result
//! the same way, through an input the key hashes with its number.
//!
//! **Outside a session nothing is proved**, so no name can be a false
//! theorem there. A scope opened outside a session counts from zero
//! each time it is opened, which is what makes a document's names, and
//! the content keys that hash them, the same in two evaluations.
//!
//! # Determinism (D9)
//!
//! A name is a hash of the scope's words and a count, and the count is
//! advanced by the order ONE scope mints in: the single-threaded walk
//! of one node's op on the thread that opened the scope. A scope does
//! not cross a thread on its own: a parallel map that should name its
//! units' mints opens a child scope per unit by the unit's index
//! (`k_stats::map_detached`), and the evaluation service names a
//! node's scope from a parent it captured before its own map. So the
//! name of a node's k-th mint is a function of the node's id, the
//! member, the enclosing scope and k, under every schedule and in
//! every replay.

use core::cell::Cell;
use core::marker::PhantomData;

use super::{Hash128, SESSION};

/// The thread's root scope outside a session: the empty child slot's
/// bits, which [`Hash128::finish`] never answers, so no opened scope
/// has it.
const ROOT: u128 = 0;

/// A session's root scope: a fixed name of its own, so a mint under no
/// scope inside a session is never named as one outside it is.
const SESSION_ROOT: u128 = 0x5345_5353_494f_4e5f_524f_4f54_5f5f_5f5f;

thread_local! {
    /// The scope open on this thread and the ordinal its next mint
    /// takes.
    static OPEN: Cell<(u128, u64)> = const { Cell::new((ROOT, 0)) };
}

/// **A mint scope's name** (module docs): a hash of the enclosing
/// scope's name and the words the opener gave.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MintScope(u128);

impl MintScope {
    /// The scope open on this thread.
    #[must_use]
    pub fn current() -> Self {
        Self(OPEN.get().0)
    }

    /// The scope inside this one that `words` name. Two word lists name
    /// two scopes, and no child is the root or its own parent up to a
    /// 128-bit collision.
    #[must_use]
    pub fn child(self, words: &[u64]) -> Self {
        let named = words.iter().fold(
            Hash128::new()
                .word(0x4d49_4e54_5f53_434f)
                .wide(self.0)
                .word(words.len() as u64),
            |h, &w| h.word(w),
        );
        Self(named.finish())
    }

    /// Opens this scope on the calling thread until the guard drops,
    /// when the scope that was open is open again and counts on from
    /// where it was.
    #[must_use]
    pub fn enter(self) -> MintGuard {
        let outer = OPEN.get();
        let next = parked(outer, self.0).unwrap_or(0);
        OPEN.set((self.0, next));
        MintGuard {
            outer,
            _on_this_thread: PhantomData,
        }
    }
}

/// An open [`MintScope`]. Not `Send`: it closes on the thread that
/// opened it.
pub struct MintGuard {
    outer: (u128, u64),
    _on_this_thread: PhantomData<*const ()>,
}

impl Drop for MintGuard {
    fn drop(&mut self) {
        let next = parked(OPEN.get(), self.outer.0).unwrap_or(self.outer.1);
        OPEN.set((self.outer.0, next));
    }
}

/// Hands the installed session `leaving`'s count and answers the count
/// it holds for `entering`; `None` outside a session, or for a scope
/// the session has not seen.
fn parked(leaving: (u128, u64), entering: u128) -> Option<u64> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        sess.mints.insert(leaving.0, leaving.1);
        sess.mints.get(&entering).copied()
    })
}

/// The name of this thread's next mint, taken: its scope and ordinal.
pub(super) fn take() -> (u128, u64) {
    let (scope, ordinal) = OPEN.get();
    OPEN.set((scope, ordinal + 1));
    (scope, ordinal)
}

/// Restores the scope a session interrupted when the session leaves,
/// by any path including a panic.
pub(super) struct SessionRoot((u128, u64));

impl SessionRoot {
    /// Opens the session's own root scope, counting from zero.
    pub(super) fn open() -> Self {
        Self(OPEN.replace((SESSION_ROOT, 0)))
    }
}

impl Drop for SessionRoot {
    fn drop(&mut self) {
        OPEN.set(self.0);
    }
}
