//! **Surgery scopes — where D1's tier-1 postcondition is paid** (the
//! ruling on `work/perf/d1-per-op-tier1-sweep-price`, Ev, PR 2305).
//!
//! D1 requires that every public mutation path preserve tier 1 and
//! that a debug build check it. The check itself is a whole-body
//! re-derivation ([`fn@crate::validate`]), O(body); a composite door
//! runs tens to hundreds of operators, so paying it per operator makes
//! a door quadratic in its own size for a property that is only
//! observable at the door.
//!
//! A **surgery scope** is the door saying so. While one is open on a
//! body, that body's operators check their declared [`ArenaDelta`]
//! and nothing else; the door runs the whole-body sweep once, at its
//! end, over the state the caller will actually see. Nesting is a
//! depth rather than a flag because doors compose — a boolean calls
//! `split`, a shell calls `replace_faces_offset` — and the OUTER door
//! is the observable boundary.
//!
//! [`ArenaDelta`]: crate::euler::ArenaDelta
//!
//! # The three spellings, and what each claims
//!
//! - [`Body::begin_surgery`] opens the scope and hands back a
//!   [`Surgery`] guard that borrows the body. Every operator reached
//!   through the guard is inside the scope.
//! - [`Surgery::sweep_and_close`] closes it and, at the outermost
//!   level, runs the tier-1 sweep. This is the door's postcondition.
//! - [`Surgery::close_already_checked`] closes it and sweeps nothing,
//!   for a door that runs its own whole-body check of tier 1 or
//!   stronger after the scope — [`fn@crate::validate_closed`], or the
//!   operators' own `assert_euler_postcondition`. It is not an
//!   escape hatch: `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
//!   requires such a door to carry that check in its own body.
//!
//! Dropping the guard closes the scope and sweeps nothing, which is
//! what an `Err` returned mid-sequence does. **The sweep is never in
//! `Drop`**: a `debug_assert` firing while a panic unwinds aborts the
//! process and takes the original error with it.
//!
//! # Release builds carry none of this
//!
//! The depth is a `#[cfg(debug_assertions)]` field on [`Body`], so a
//! release [`Body`] does not have it and its layout is the one it had
//! before this module existed. The guard survives as a borrow with an
//! empty `Drop`.
//!
//! # The scalpel
//!
//! A door-level failure names the door. To name the OPERATOR inside
//! it, build with `--features topo/per-op-postcondition`: the sweep
//! then runs after every operator exactly as it did before the scopes
//! landed, and the panic message carries the operator's name. Opt-in,
//! never default-on — the `shadow-exec` shape (`work/perf/plan.md`
//! §4.5).

use geom_core::Real;

use crate::Body;

/// How many surgery scopes are open on one body.
///
/// Debug builds only, and one `Cell` rather than a `&mut` path
/// because the operators that read it hold `&self`.
#[cfg(debug_assertions)]
#[derive(Debug, Default)]
pub(crate) struct SurgeryDepth(core::cell::Cell<u32>);

#[cfg(debug_assertions)]
impl Clone for SurgeryDepth {
    /// **A clone starts outside every scope**, whatever the original
    /// was inside.
    ///
    /// The scope is a claim about one door and the body it will
    /// hand back. A clone is a different body: no door has promised
    /// to sweep it, so nothing may be skipped on it, and the doors
    /// that stage a mutation into a clone open their own scope on
    /// the clone. Copying the depth instead would silence a body
    /// nobody had undertaken to check.
    fn clone(&self) -> Self {
        Self::default()
    }
}

#[cfg(debug_assertions)]
impl SurgeryDepth {
    /// The current depth.
    fn get(&self) -> u32 {
        self.0.get()
    }

    /// Opens a scope.
    fn open(&self) {
        self.0.set(self.0.get() + 1);
    }

    /// Closes a scope, returning the depth it was at.
    fn close(&self) -> u32 {
        let was = self.0.get();
        let Some(next) = was.checked_sub(1) else {
            unreachable!(
                "surgery scope closed at depth 0: a `Surgery` guard exists only between an \
                 open and its close, so the depth cannot have been decremented already"
            )
        };
        self.0.set(next);
        was
    }
}

impl<T: Real> Body<T> {
    /// **Opens a surgery scope on this body** — the door's declaration
    /// that it, and not the operators it runs, owns the tier-1
    /// postcondition.
    ///
    /// The returned guard derefs to the body, so the door's operator
    /// sequence is written through it unchanged. Close it with
    /// [`Surgery::sweep_and_close`] on the success path; an `Err`
    /// returned mid-sequence drops the guard instead, which closes
    /// the scope and sweeps nothing.
    ///
    /// See the [module docs](self) for what the scope claims and for
    /// the per-operator scalpel that recovers the failing operator's
    /// name.
    #[must_use = "a scope that is opened and immediately dropped silences nothing"]
    pub fn begin_surgery(&mut self) -> Surgery<'_, T> {
        #[cfg(debug_assertions)]
        self.surgery.open();
        Surgery { body: self }
    }

    /// How many surgery scopes are open on this body — **always `0` in
    /// a release build**, where no scope is tracked at all.
    ///
    /// A door that has returned, by either path, leaves this at `0`.
    /// That is what the guard's `Drop` is for, and it is asserted
    /// rather than assumed: a scope a door forgot to close would
    /// silence every later operator on that body.
    #[must_use]
    pub fn open_surgery_scopes(&self) -> u32 {
        #[cfg(debug_assertions)]
        {
            self.surgery.get()
        }
        #[cfg(not(debug_assertions))]
        {
            0
        }
    }

    /// Whether the tier-1 whole-body sweep is this call's to run.
    ///
    /// `false` exactly while a door has a scope open and has
    /// undertaken to sweep at its end. The `per-op-postcondition`
    /// scalpel makes it unconditionally `true`.
    #[cfg(debug_assertions)]
    pub(crate) fn tier1_sweep_is_mine(&self) -> bool {
        cfg!(feature = "per-op-postcondition") || self.surgery.get() == 0
    }

    /// D1's whole-body tier-1 postcondition for a door that mints
    /// nothing and so has no [`ArenaDelta`](crate::euler::ArenaDelta)
    /// to declare — the attach setters.
    ///
    /// Skipped inside an open surgery scope, where the door sweeps
    /// once at its end instead.
    #[cfg(debug_assertions)]
    pub(crate) fn assert_tier1_postcondition(&self, op: &str) {
        if !self.tier1_sweep_is_mine() {
            return;
        }
        debug_assert_eq!(
            crate::validate::validate(self),
            Ok(()),
            "{op} postcondition: result is not tier-1 valid (kernel bug)",
        );
    }
}

impl<T: Real> Body<T> {
    /// **Opens a surgery scope without a guard**, for a body a
    /// [`Surgery`] cannot borrow for the span the scope needs.
    ///
    /// [`Body::begin_surgery`] is the spelling to reach for: it cannot
    /// leak a scope, because the borrow it holds is released only by
    /// dropping it. This pair exists for the one shape that borrow
    /// forbids — a pipeline phase that must pass the body on as part
    /// of a larger value (`bool_connect` takes the whole reduction,
    /// whose two operand bodies are what the scope is about), where a
    /// guard borrowing one field would deny the phase the struct.
    ///
    /// **The obligation, and it is on the caller**: the open and its
    /// [`Body::leave_surgery`] sit in one function, and the body they
    /// are about is a LOCAL VALUE THE FAILURE PATH DROPS. That is what
    /// makes an early `?` between them harmless — the scope dies with
    /// the body it was open on, and no later call can reach a body a
    /// scope was left open on. A scope left open on a body that
    /// SURVIVES silences every later operator on it, which is why the
    /// guard is the default and this is the exception.
    pub fn enter_surgery(&self) {
        #[cfg(debug_assertions)]
        self.surgery.open();
    }

    /// Closes a scope opened by [`Body::enter_surgery`]. Sweeps
    /// nothing: the door runs its own whole-body check after it.
    pub fn leave_surgery(&self) {
        #[cfg(debug_assertions)]
        let _ = self.surgery.close();
    }

    /// Closes a scope opened by [`Body::enter_surgery`] and runs D1's
    /// tier-1 sweep, once, at the outermost level — the guardless
    /// spelling of [`Surgery::sweep_and_close`], for a phase whose
    /// body no stronger check follows.
    pub fn leave_surgery_and_sweep(&self) {
        #[cfg(debug_assertions)]
        if self.surgery.close() == 1 {
            debug_assert_eq!(
                crate::validate::validate(self),
                Ok(()),
                "door postcondition: result is not tier-1 valid (kernel bug)",
            );
        }
    }
}

/// An open surgery scope on a [`Body`] — see the [module docs](self).
///
/// Derefs to the body, so the door's operator sequence is written
/// through it unchanged.
#[derive(Debug)]
pub struct Surgery<'b, T: Real> {
    body: &'b mut Body<T>,
}

impl<T: Real> Surgery<'_, T> {
    /// **Closes the scope and runs D1's tier-1 sweep** over the body
    /// the door is about to hand back — once, at the outermost level.
    ///
    /// A nested scope sweeps nothing: the outer door is the observable
    /// boundary and it has not finished yet.
    pub fn sweep_and_close(self) {
        #[cfg(debug_assertions)]
        if self.body.surgery.get() == 1 {
            // Still inside the scope, deliberately: this call IS the
            // door's postcondition, and running it here rather than
            // after the decrement keeps `Drop` free of assertions.
            debug_assert_eq!(
                crate::validate::validate(self.body),
                Ok(()),
                "door postcondition: result is not tier-1 valid (kernel bug)",
            );
        }
    }

    /// **Closes the scope without sweeping**, for a door that runs its
    /// own whole-body check — [`fn@crate::validate_closed`], a tier-2
    /// or stronger gate, or the operators' own
    /// `assert_euler_postcondition` — AFTER this point.
    ///
    /// Tier 2 subsumes tier 1, so a door that already pays a stronger
    /// sweep must not pay a weaker one beside it. What makes this
    /// honest rather than an escape hatch is that the closure-property
    /// walk requires the check to be present in the door's own body.
    pub fn close_already_checked(self) {}
}

impl<T: Real> Drop for Surgery<'_, T> {
    /// Closes the scope. **Sweeps nothing, by construction**: this
    /// runs on the `Err` path and during unwinding, where a
    /// `debug_assert` firing would abort the process and hide the
    /// error that got here first.
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        let _ = self.body.surgery.close();
    }
}

impl<T: Real> core::ops::Deref for Surgery<'_, T> {
    type Target = Body<T>;

    fn deref(&self) -> &Body<T> {
        self.body
    }
}

impl<T: Real> core::ops::DerefMut for Surgery<'_, T> {
    fn deref_mut(&mut self) -> &mut Body<T> {
        self.body
    }
}
