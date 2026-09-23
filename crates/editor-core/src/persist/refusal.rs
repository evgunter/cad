//! How a structured refusal leaves a `Deserialize` impl.
//!
//! `Deserialize` hands an impl exactly one error type and it is the
//! FORMAT's: everything a rebuild refusal knows has to fit through
//! `serde::de::Error::custom`, which takes prose. That is why an
//! ill-dimensioned expression in a save file used to reach a caller as
//! a sentence inside [`super::PersistError::Unreadable`] — the
//! [`crate::expr::DimensionError`] itself had nowhere to go.
//!
//! A [`Parse`](crate::persist::refusal::Parse) guard is where it goes. [`Parse::open`](crate::persist::refusal::Parse::open) pushes a frame
//! for one body parse; a refusing impl [`record`](crate::persist::refusal::record)s the typed value
//! into the innermost open frame on its way out; [`Parse::finish`](crate::persist::refusal::Parse::finish)
//! pops the frame and answers what it holds, so the door can raise
//! [`super::PersistError::Dimension`] carrying the refusal rather than
//! a description of it.
//!
//! # Who records, and who harvests
//!
//! The coupling is a side channel and therefore says at both ends what
//! it is doing, because no signature between them does:
//!
//! - **Recorders** — every site that turns a [`crate::expr::DimensionError`] into
//!   `Error::custom` prose: [`crate::persist::wire`]'s `Deserialize` impls for
//!   `Expr` and `MeasureExpr`, and [`crate::expr::UnitSym`]'s, which
//!   refuses an off-table display-unit symbol at the token. Each calls
//!   [`record`](crate::persist::refusal::record) beside its `custom`, and says so there.
//! - **The harvester** — [`crate::persist::parse_body`], the ONE holder of a
//!   [`Parse`](crate::persist::refusal::Parse), which hands what it finds to `parse_err` as an
//!   argument. From there the channel is an ordinary parameter.
//!
//! # Why reading it back is sound
//!
//! `docs/PERF-SCAN-2026-08.md` §2.4 sets the bar for a production
//! value delivered by thread-local side effect, and this module is
//! one: **an RAII guard so a forgotten harvest is impossible;
//! re-entrancy that never silently overwrites; thread-confinement
//! enforced by the type; and the coupling visible at both ends.**
//! `k_stats::Bracket` (PR 1969, `work/scalar/D283.md`) is the in-tree
//! shape this copies, one frame deep instead of many.
//!
//! - **A frame cannot leak or be forgotten.** [`Parse`](crate::persist::refusal::Parse) pops its frame
//!   in `Drop` as well as in [`Parse::finish`](crate::persist::refusal::Parse::finish), so a parse that panics
//!   or returns early leaves nothing armed for the next one on this
//!   thread — the property `topo`'s `ArmedTear` disarms for. The value
//!   comes back only from `finish`, which consumes the guard, so a
//!   frame is harvested at most once.
//! - **Re-entrancy composes; it never overwrites.** A parse that
//!   somehow ran inside another gets its OWN frame and records into
//!   it; the outer frame is untouched and still holds the outer
//!   parse's first refusal. Nothing is discarded to make room.
//! - **The guard is thread-confined by the type.** A
//!   `PhantomData<*const ()>` makes [`Parse`](crate::persist::refusal::Parse) `!Send`, so the compiler
//!   refuses to move one to a thread that would pop another thread's
//!   frame. (`Bracket` pins that with a `compile_fail` doctest;
//!   rustdoc runs no doctest on a private item, so what stands here is
//!   the bound itself and the type's single construction site.)
//! - **[`record`](crate::persist::refusal::record) outside a parse is a no-op**, because there is no
//!   frame to write to. A `Deserialize` impl driven by something other
//!   than [`crate::persist::parse_body`] — a test reading one wire type, a
//!   caller of `serde_json::from_str` — cannot leave a value behind
//!   for a later parse to adopt.
//!
//! **First refusal wins.** Every impl between an expression and the
//! file body propagates the first error it meets (`?` in `rebuild`,
//! `?` in the derived visitors above it), so the refusal that reaches
//! the caller is the first one raised — which is the one the frame
//! keeps. The premise is that nothing in that chain asks serde to TRY
//! an alternative and then fall back: a recorded refusal that did not
//! decide the parse would have the door name the wrong one. Five
//! declarations would break it — `#[serde(untagged)]`,
//! `#[serde(other)]`, `#[serde(flatten)]`, a `deserialize_with` that
//! retries, and a `serde_json::Value` intermediate — and
//! `scripts/gates/persist-no-backtracking.sh` refuses all five across
//! `editor-core`, so the premise is a gate rather than a reading. (The
//! crate's one `deserialize_with`, [`crate::persist::wire`]'s `plane_ref`, is a
//! single `deserialize_u64` with one visit method and no fallback; the
//! gate allows it by name and reds if a second appears.)
//!
//! The reading back is narrow for the same reason: a recorded refusal
//! is adopted only when serde_json classifies the failure as
//! [`serde_json::error::Category::Data`] — the class a rebuild refusal
//! raises — and a parse that succeeds discards its frame rather than
//! reporting it.
//!
//! # The cost, disclosed
//!
//! This is the fourth install/record/take scaffold over a thread-local
//! in `crates/`. The family is `geom-core/src/sym/report.rs` and
//! `sym/profile.rs` — the same shape spelled twice, each saying so at
//! its copy — plus `k_stats`, which is the shape lifted into a guard.
//! This one is a guard from birth and carries a PRODUCTION value
//! rather than an instrument's, which is why it is held to §2.4 above;
//! what it shares with the other three is the mechanism, and naming
//! that is what this paragraph is for.

use core::cell::RefCell;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use crate::expr::DimensionError;

thread_local! {
    /// One frame per open [`Parse`] on this thread, innermost last.
    /// A frame holds the first refusal its parse recorded, if any.
    static FRAMES: RefCell<Vec<Option<DimensionError>>> = const { RefCell::new(Vec::new()) };
}

/// The refusal channel for ONE body parse: a guard whose lifetime is
/// the frame a rebuild refusal is recorded into (module docs).
///
/// `!Send` by construction (the `*const ()` phantom): the guard closes
/// its frame on the thread that opened it, and the compiler refuses to
/// move it anywhere else.
#[must_use = "a frame is open only while the guard is held; bind it, then `finish` it"]
#[derive(Debug)]
pub(super) struct Parse {
    /// This frame's index on the stack.
    depth: usize,
    _confined: PhantomData<*const ()>,
}

impl Parse {
    /// Opens a fresh, empty frame on this thread's stack. Every
    /// refusal [`record`]ed from now until the guard is finished or
    /// dropped — and outside any inner guard — lands in it.
    pub(super) fn open() -> Self {
        let depth = FRAMES.with_borrow_mut(|frames| {
            frames.push(None);
            frames.len() - 1
        });
        Self {
            depth,
            _confined: PhantomData,
        }
    }

    /// Closes the frame and answers the refusal it holds. Consumes the
    /// guard, so a frame is harvested at most once.
    pub(super) fn finish(self) -> Option<DimensionError> {
        // `Drop` would close the frame a second time; skipping it is
        // the whole reason for the wrapper.
        let this = ManuallyDrop::new(self);
        close(this.depth)
    }
}

impl Drop for Parse {
    fn drop(&mut self) {
        // Unwinding through a parse must not leave a frame armed for
        // whatever runs next on this thread.
        drop(close(self.depth));
    }
}

/// Pops the frame at `depth` and everything above it, answering what
/// that frame held.
///
/// Guards are locals of one function, so they close innermost-first
/// and `depth` is the last frame; truncating rather than popping makes
/// the out-of-order case DEFINED anyway — an inner frame still open is
/// discarded with the outer one, and a `depth` no longer on the stack
/// answers `None` and touches nothing.
fn close(depth: usize) -> Option<DimensionError> {
    FRAMES.with_borrow_mut(|frames| {
        let refused = frames.get_mut(depth).and_then(Option::take);
        frames.truncate(depth);
        refused
    })
}

/// Records a rebuild refusal on its way out through `Error::custom`.
///
/// Keeps the FIRST of a parse, which is the one that propagates, and
/// does nothing at all when no [`Parse`] is open — a `Deserialize`
/// impl driven from anywhere else leaves no trace.
pub(crate) fn record(err: &DimensionError) {
    FRAMES.with_borrow_mut(|frames| {
        if let Some(frame) = frames.last_mut()
            && frame.is_none()
        {
            *frame = Some(err.clone());
        }
    });
}
