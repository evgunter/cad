//! **Where a value with no expression gets its name.**
//!
//! [`Sym::opaque`](crate::sym::Sym::opaque) mints an unknown of its own, and
//! an unknown needs a name no other unknown it can meet has: two
//! different reals under one name are one unknown, and their difference
//! is the zero polynomial, a `Zero` about two values that are not
//! equal. A name is a pair, a scope and an ordinal in it, and where it
//! comes from depends on whether a session is installed.
//!
//! # Inside a session: the mint scope and its ordinal
//!
//! The scope is the **mint scope** open on the minting thread, the
//! ordinal the value's place in the order that scope mints, from zero.
//! A scope is opened by whoever owns a unit of work whose mints should
//! be named by that unit: the evaluation service opens one per document
//! node (and per member of a family), named by the node's id, so a
//! value a node mints is named by the node and its place in the node's
//! own walk, whatever ran before the node and in whichever replay. A
//! mint under no opened scope is in the session's **root** scope.
//!
//! **A session mints no name twice.** A scope's count is kept by the
//! session while the scope is closed, so a scope opened a second time
//! (a node evaluated twice in one session, a scope entered from inside
//! itself or its child) continues where it stopped.
//!
//! **Two sessions mint the same names**, and may bind them to two
//! numbers: a node whose inputs differ between two replays mints
//! different values at the same ordinals. Those meet only where a value
//! built in one session is read in another, and the one door for that
//! is a node's result served under its content key. The key hashes
//! every expression and number the node read and, through its inputs'
//! keys, everything they read; a value can reach a node only through an
//! input; so a served result holds a name only if the node that minted
//! it has the same key in both replays, and then it is not run in the
//! serving one: the name is minted there by nobody. A root-scope name
//! reaches a node's result the same way, through an input the key
//! hashes with its number. A value carried from one session into
//! another by any other route has no such argument: nothing in the
//! tree does it.
//!
//! **The names are the same in every replay (D9).** A name is a hash of
//! the scope's words and a count, and the count is advanced by the
//! order ONE scope mints in: a session keeps its walks on the thread
//! that installed it ([`crate::sym::decisions_are_thread_portable`]), so
//! that order is the single-threaded walk of one node's op. The name of
//! a node's k-th mint is a function of the node's id, the member, the
//! enclosing scope and k.
//!
//! # Outside a session: the thread's own count
//!
//! Nothing is proved outside a session, but a value minted there can be
//! handed into one, so its name has to be one nothing else has. It is
//! the minting thread's token, unique in the process, and the thread's
//! count of such mints, which never restarts: no scope enters it, so no
//! opening, re-opening or parallel map of a scope can give it twice,
//! and no session's name is one of these. It is NOT the same in two
//! runs: which thread mints, and what it minted before, are the
//! schedule's. A walk that maps its units in parallel outside a session
//! and runs them in order inside one therefore names the same work two
//! ways; what is a function of the document is the name inside a
//! session, which is where a name is read.

use core::cell::Cell;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU64, Ordering};

use super::{Hash128, SESSION};

/// The scope [`MintScope::current`] answers where none is open: the
/// empty child slot's bits, which [`Hash128::finish`] never answers, so
/// no child scope has it. Nothing is minted under it.
const NONE: u128 = 0;

/// A session's root scope: a fixed name, the same in every session.
const SESSION_ROOT: u128 = 0x5345_5353_494f_4e5f_524f_4f54_5f5f_5f5f;

/// The next thread token: a thread takes one at its first mint outside
/// a session.
static THREADS: AtomicU64 = AtomicU64::new(1);

thread_local! {
    /// The scope open on this thread and the ordinal its next mint
    /// inside a session takes.
    static OPEN: Cell<(u128, u64)> = const { Cell::new((NONE, 0)) };
    /// This thread's token (zero until it has one) and how many values
    /// it has minted outside a session.
    static OUTSIDE: Cell<(u64, u64)> = const { Cell::new((0, 0)) };
    /// How many sessions this thread has installed, and which of them
    /// is installed now (zero for none): what ties a [`MintGuard`] to
    /// the session it was opened in.
    static SESSIONS: Cell<(u64, u64)> = const { Cell::new((0, 0)) };
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
    /// two scopes, and no child is a root or its own parent up to a
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
    /// where it was. A guard drops in the session it was opened in, and
    /// before any guard opened earlier.
    #[must_use]
    pub fn enter(self) -> MintGuard {
        let outer = OPEN.get();
        let next = parked(outer, self.0).unwrap_or(0);
        OPEN.set((self.0, next));
        MintGuard {
            opened: self.0,
            outer,
            session: SESSIONS.get().1,
            _on_this_thread: PhantomData,
        }
    }
}

/// An open [`MintScope`]. Not `Send`: it closes on the thread that
/// opened it.
pub struct MintGuard {
    opened: u128,
    outer: (u128, u64),
    session: u64,
    _on_this_thread: PhantomData<*const ()>,
}

impl Drop for MintGuard {
    fn drop(&mut self) {
        let open = OPEN.get();
        let in_order = self.session == SESSIONS.get().1 && open.0 == self.opened;
        debug_assert!(
            in_order || std::thread::panicking(),
            "a mint scope closes in the session it was opened in, innermost first"
        );
        // Out of order, the scope that is open is some other opening's
        // and its count is not this guard's to move.
        if in_order {
            let next = parked(open, self.outer.0).unwrap_or(self.outer.1);
            OPEN.set((self.outer.0, next));
        }
    }
}

/// Hands the installed session `leaving`'s count and answers the count
/// it holds for `entering`; `None` outside a session, where no count is
/// read, or for a scope the session has not seen.
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
    if SESSIONS.get().1 != 0 {
        let (scope, ordinal) = OPEN.get();
        OPEN.set((scope, ordinal + 1));
        return (scope, ordinal);
    }
    let (token, ordinal) = match OUTSIDE.get() {
        (0, ordinal) => (THREADS.fetch_add(1, Ordering::Relaxed), ordinal),
        held => held,
    };
    OUTSIDE.set((token, ordinal + 1));
    let thread = Hash128::new()
        .word(0x4d49_4e54_5f54_4852)
        .word(token)
        .finish();
    (thread, ordinal)
}

/// A session's root scope, open for as long as the session is
/// installed; what was open before is restored when the session
/// leaves, by any path including a panic.
pub(super) struct SessionRoot {
    open: (u128, u64),
    installed: u64,
}

impl SessionRoot {
    /// Opens the installing session's root scope, counting from zero.
    pub(super) fn open() -> Self {
        let (started, installed) = SESSIONS.get();
        SESSIONS.set((started + 1, started + 1));
        Self {
            open: OPEN.replace((SESSION_ROOT, 0)),
            installed,
        }
    }
}

impl Drop for SessionRoot {
    fn drop(&mut self) {
        OPEN.set(self.open);
        SESSIONS.set((SESSIONS.get().0, self.installed));
    }
}
