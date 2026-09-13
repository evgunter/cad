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
//! end, over the state the caller will actually see.
//!
//! [`ArenaDelta`]: crate::euler::ArenaDelta
//!
//! # What the depth does, and what it does not
//!
//! **The depth is per BODY.** Nesting is a depth rather than a flag
//! because one body can be inside two scopes at once — a boolean
//! opens one on the body it is finishing and the merge it calls opens
//! another on that same body — and then the outer close is the
//! observable boundary and the inner one sweeps nothing.
//!
//! **A door that stages into a CLONE is its own boundary**, and that
//! is the commoner shape of composition here. `shell_open` calls
//! `replace_faces_offset`, which clones the destination, mutates the
//! clone, gates it, and adopts it. The clone starts at depth 0
//! whatever scope the destination is inside (`SurgeryDepth`'s
//! `Clone`), so the staging door's own scope on it is outermost and
//! its close DOES sweep — once per call, not once per setter.
//! `merge_coplanar_faces_declared` is the same shape per group. That
//! is correct rather than unfortunate: no outer door ever promised to
//! sweep that clone, and the clone is the body the staging door is
//! about to hand back.
//!
//! Such a door therefore pays two whole-body reads at its end — the
//! tier-1 sweep its close runs, and its own typed
//! [`fn@crate::validate_closed`] gate — and the pair is deliberate:
//! **a panic and a typed refusal are different contracts.** A kernel
//! bug has to panic, as loudly and as early as it is detectable (D9);
//! a body that is tier-1 sound but not closed is a refusal the caller
//! is entitled to receive. Collapsing the two would answer the first
//! with the second.
//!
//! # The spellings, and what each claims
//!
//! - [`Body::begin_surgery`] opens the scope and hands back a
//!   [`Surgery`] guard that borrows the body. Every operator reached
//!   through the guard is inside the scope.
//! - [`Surgery::sweep_and_close`] closes it and, at the outermost
//!   level, runs the tier-1 sweep. This is the door's postcondition.
//! - [`Surgery::close_already_checked`] closes it and sweeps nothing,
//!   for a door whose own trailing check is a **debug assertion** at
//!   tier 1 or stronger — `debug_assert_eq!(validate_closed(&body),
//!   Ok(()))`. It is not an escape hatch, and the qualifier is the
//!   whole of it: a TYPED gate is not that check. A door that ends
//!   `validate_closed(&work).map_err(…)?` turns a tier-1 failure into
//!   an error return, and before this rule existed that same failure
//!   panicked out of the operator that caused it. Such a door closes
//!   with [`Surgery::sweep_and_close`], which keeps the split the door
//!   already had: a kernel bug panics, a tier-2 refusal returns typed.
//!   `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
//!   requires a non-sweeping close to carry that debug assertion in
//!   the door's own body.
//!
//! Dropping the guard closes the scope and sweeps nothing, which is
//! what an `Err` returned mid-sequence does. **The sweep is never in
//! `Drop`**: a `debug_assert` firing while a panic unwinds aborts the
//! process and takes the original error with it.
//!
//! `Body::enter_surgery` and its two closes are the same three
//! meanings without the guard, for the two sites in this crate where
//! a borrow forbids one. They are `pub(crate)` and their obligation is
//! the caller's; the method docs carry it.
//!
//! # What actually fires, and what does not
//!
//! Three mechanisms, and it is worth being exact about their reach
//! because the difference is where the residue lives
//! (`work/perf/door-scopes-outside-topo-are-unguarded`).
//!
//! - **A guarded scope cannot be left open**: the borrow is released
//!   only by dropping the guard or consuming it in a close, and both
//!   decrement. That is every scope in the tree but two.
//! - **A lexical read** —
//!   `source_walk::MutationDoor::surgery_posture`, used by
//!   `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
//!   — reds on a door that opens a scope and closes nothing. Its
//!   population is the `pub fn … &mut self` doors of `topo/src`, so it
//!   covers none of `sweep`, `step-import` or `editor-core`, and it
//!   reads text: two opens against one close, or a close on one path
//!   only, read as closed.
//! - **A runtime read** — [`Body::open_surgery_scopes`], which any
//!   crate can call — is what the two guardless sites have, in the
//!   shape of a `debug_assert` at the next phase boundary
//!   (`boolean::finish`, `splitting::finish`) plus
//!   `surgery::tests::every_door_returns_with_its_scopes_closed`.
//!
//! **What none of them sees** is a DELETED SWEEP, as opposed to a
//! leaked scope: a close that decrements without sweeping when it
//! should have swept leaves the depth right and the check gone. What
//! catches that is a corruption row per door class, and there are
//! three of them.
//!
//! # The field is exactly as wide as the postcondition
//!
//! The depth is a `#[cfg(debug_assertions)]` field on [`Body`], so it
//! exists in precisely the builds the postcondition exists in and no
//! others. A build with debug assertions off — cargo's release
//! default, and what `benches/` sets — has neither, and its [`Body`]
//! has the layout it had before this module existed. This workspace's
//! own `[profile.release]` turns debug assertions ON deliberately
//! ("the profile that would meet real parts is the one profile
//! checking nothing" is the thing it refuses), so the shipped release
//! binary carries both, as it carried the per-operator sweep before.
//! Nothing about a body's SERIALIZED form moves either way: [`Body`]
//! has no `Serialize`, and `editor-core` persists documents as
//! recipes rather than as arenas.
//!
//! The guard is a borrow with an empty `Drop` wherever the field is
//! absent.
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
/// Under `debug_assertions` only, and interior-mutable rather than a
/// `&mut` path because the operators that read it hold `&self`.
///
/// **An atomic and not a `Cell`, and it is not a thread-safety
/// claim.** [`Body`] is `Sync` and has to stay so — `editor-core`'s
/// evaluator hands `&Body` to a rayon `par_iter` over one recipe
/// level — and a `Cell` field would take that away from every body in
/// the workspace. Nothing here is contended: a scope is opened and
/// closed by one door on one thread, `Relaxed` is the whole of the
/// ordering, and a body reached through `&Body` from several threads
/// has no scope open on it to begin with.
#[cfg(debug_assertions)]
#[derive(Debug, Default)]
pub(crate) struct SurgeryDepth(core::sync::atomic::AtomicU32);

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
        self.0.load(core::sync::atomic::Ordering::Relaxed)
    }

    /// The depth, written.
    fn set(&self, depth: u32) {
        self.0.store(depth, core::sync::atomic::Ordering::Relaxed);
    }

    /// Opens a scope.
    ///
    /// **A load and a store, not a `fetch_add`.** The atomic is here
    /// to keep [`Body`] `Sync`, not because two threads share a depth:
    /// a scope belongs to one door running on one thread, and a body
    /// reached through `&Body` from several threads has no scope open
    /// on it at all. A read-modify-write would buy atomicity between
    /// the load and the store, which is a race this value cannot have
    /// — and if it could, an atomic increment would silently produce a
    /// wrong depth rather than the loud wrong answer a torn pair
    /// gives. Keeping it non-atomic keeps the claim honest.
    fn open(&self) {
        self.set(self.get() + 1);
    }

    /// Closes a scope, returning the depth it was at.
    fn close(&self) -> u32 {
        let was = self.get();
        let Some(next) = was.checked_sub(1) else {
            unreachable!(
                "surgery scope closed at depth 0: every close is paired with an open, \
                 either by a `Surgery` guard's `Drop` or by the one `enter_surgery` that \
                 precedes it, so the depth cannot already be 0 here"
            )
        };
        self.set(next);
        was
    }

    /// **D1's closing sweep**: the tier-1 re-derivation over `body`,
    /// run at the outermost level only.
    ///
    /// Both closes call this and then decrement — the guarded one
    /// through its `Drop`, the guardless one on the next line — so
    /// there is one spelling of the check and one ordering of it. The
    /// order is deliberate: the sweep is the door's postcondition and
    /// it belongs INSIDE the scope it closes, so that a validator
    /// reached through a public door could not re-enter this path at
    /// depth 0.
    fn sweep_if_outermost<T: Real>(&self, body: &Body<T>) {
        if self.get() == 1 {
            debug_assert_eq!(
                crate::validate::validate(body),
                Ok(()),
                "door postcondition: result is not tier-1 valid (kernel bug)",
            );
        }
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
    /// **Replaces this body's contents with `next`, keeping the
    /// surgery scopes open on THIS one.**
    ///
    /// The staging doors — mutate a clone, gate it, adopt it — write
    /// their result back over the destination, and a plain `*self =
    /// next` would carry the clone's scope depth with it. The
    /// destination's depth is not the clone's to set: it says how many
    /// doors up the stack have undertaken to sweep the body this
    /// reference names, and that is true of it before and after the
    /// swap, whatever was staged. Overwriting it closes a scope its
    /// owner still holds — a `Surgery` guard whose depth has already
    /// been decremented, which announces itself at the guard's close
    /// rather than silently.
    ///
    /// **Crate-internal, and it stays that way.** It replaces a body
    /// wholesale, which is not a mutation D1 sanctions from outside;
    /// the staging doors are its whole population and each gates the
    /// clone before it adopts it.
    pub(crate) fn adopt(&mut self, next: Self) {
        #[cfg(debug_assertions)]
        let held = self.surgery.get();
        *self = next;
        #[cfg(debug_assertions)]
        self.surgery.set(held);
    }

    /// **Opens a surgery scope without a guard.**
    ///
    /// [`Body::begin_surgery`] is the spelling for every site that can
    /// take it, and almost every site can: the guard cannot leak a
    /// scope, because the only way to release the borrow it holds is
    /// to drop it or to consume it in a close. **This pair exists for
    /// the one shape that borrow forbids** — a scope on a body that is
    /// a FIELD OF A STRUCT the next call takes whole, where a guard
    /// borrowing the field would deny the call the struct. Two sites
    /// in this crate are that shape and there are no others:
    /// `boolean::ops`' reduction operands across `bool_connect`, and
    /// `splitting`'s reduced body across `split_connect`.
    ///
    /// **The obligation, and it is on the caller**: the open and its
    /// close sit in one function, and the body they are about is a
    /// LOCAL VALUE THE FAILURE PATH DROPS. That is what makes an early
    /// `?` between them harmless — the scope dies with the body it was
    /// open on, and no later call can reach a body a scope was left
    /// open on. A scope left open on a body that SURVIVES silences
    /// every later operator on it.
    ///
    /// **Nothing checks that obligation**, and that is why this is
    /// `pub(crate)` and takes `&mut self`. A deleted close at one of
    /// the two surviving sites is undetected — the residue is sized in
    /// `work/perf/door-scopes-outside-topo-are-unguarded`. `&mut self`
    /// is not needed by the mechanism (the depth is interior-mutable
    /// for the operators, which hold `&self`); it is here so that
    /// opening a scope reads as the mutation it is, and so a body
    /// reached by shared reference cannot have one opened on it.
    pub(crate) fn enter_surgery(&mut self) {
        #[cfg(debug_assertions)]
        self.surgery.open();
    }

    /// Closes a scope opened by [`Body::enter_surgery`] **without
    /// sweeping** — the guardless spelling of
    /// [`Surgery::close_already_checked`].
    ///
    /// Its one caller is an `Err` path: `boolean::ops` closes its two
    /// operand scopes this way when the join refused, because the
    /// sweep is the SUCCESS path's and a partially carved operand is
    /// not a state any door undertook to certify. It asserts nothing
    /// and claims nothing.
    pub(crate) fn leave_surgery(&mut self) {
        #[cfg(debug_assertions)]
        let _ = self.surgery.close();
    }

    /// Closes a scope opened by [`Body::enter_surgery`] and runs D1's
    /// tier-1 sweep, once, at the outermost level — the guardless
    /// spelling of [`Surgery::sweep_and_close`].
    pub(crate) fn leave_surgery_and_sweep(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.surgery.sweep_if_outermost(self);
            let _ = self.surgery.close();
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
        // The sweep runs here and the decrement in `Drop`, at the end
        // of this call: this IS the door's postcondition, and keeping
        // it out of `Drop` is what keeps `Drop` free of assertions.
        #[cfg(debug_assertions)]
        self.body.surgery.sweep_if_outermost(self.body);
    }

    /// **Closes the scope without sweeping**, for a door whose own
    /// trailing check is a DEBUG ASSERTION at tier 1 or stronger —
    /// `debug_assert_eq!(validate_closed(&body), Ok(()))` — placed
    /// after this point.
    ///
    /// Tier 2 subsumes tier 1, so a door that already asserts a
    /// stronger sweep must not assert a weaker one beside it. **A
    /// typed gate is not that check**: a door ending
    /// `validate_closed(&work).map_err(…)?` answers a kernel bug with
    /// an error return, where the operators it composes used to panic,
    /// so such a door closes with [`Surgery::sweep_and_close`]
    /// instead. The closure-property walk requires the debug assertion
    /// to be present in the door's own body.
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

#[cfg(test)]
pub(crate) mod tests {
    // Test-support code: panicking is a test's failure mechanism (L5).
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Point3, Tol};

    use crate::Body;
    use crate::entity::HalfEdgeKey;
    use crate::euler::MevSite;

    /// A cube, built through the operators, tier-1 valid.
    fn cube() -> Body<f64> {
        crate::splitting::reassembly::quad_prism(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            1.0,
            Tol::witness(),
        )
    }

    /// **Corruption tier 1 rejects and no operator's precondition can
    /// see**: a second copy of a face's surface, referenced by
    /// nothing. Tier 1's orphan-geometry pass reports it; every key in
    /// the body still resolves, every loop still closes, so an
    /// operator run afterwards plans and mutates exactly as it would
    /// on a sound body. That is what makes it the right plant for this
    /// question — the operator does not refuse, it simply no longer
    /// looks.
    fn plant_an_orphan_surface(body: &mut Body<f64>) {
        let face = body.faces().next().unwrap().0;
        let key = body.get_face(face).unwrap().surface;
        let copy = body.get_surface(key).unwrap().clone();
        let _ = body.add_surface(copy);
        assert!(
            crate::validate::validate(body).is_err(),
            "the plant is a tier-1 error"
        );
    }

    /// A half-edge of the body, for a strut `mev` (an empty fan run).
    fn a_half_edge(body: &Body<f64>) -> HalfEdgeKey {
        body.half_edges().next().unwrap().0
    }

    /// **The one lock every panic-capturing row in this crate takes.**
    ///
    /// The panic hook is PROCESS-global and the suite runs its rows in
    /// parallel threads of one process, so two rows capturing at once
    /// clobber each other — the second installs its hook over the
    /// first's and one of them reads an empty or foreign message. That
    /// is a flake, not a failure, and it is the kind that arrives once
    /// the third such row lands. Every taker of the hook holds this
    /// first: [`panic_message`] here, and
    /// `review_m1_pr2::release_corruption`'s own capture.
    pub(crate) static PANIC_HOOK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// The message of the panic `f` raises, captured through a panic
    /// HOOK rather than by downcasting `catch_unwind`'s payload — the
    /// `bit-identity punning` gate bans `Any` downcasts outside
    /// `geom_core::bit_identity`, test code included. The hook is
    /// process-global, so [`PANIC_HOOK`] serializes the takers, it is
    /// restored immediately, and a foreign or empty capture fails the
    /// caller's assertion loudly rather than passing quietly.
    fn panic_message(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
        let _serialized = PANIC_HOOK.lock().unwrap_or_else(|e| e.into_inner());
        let captured = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let sink = std::sync::Arc::clone(&captured);
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if let Ok(mut slot) = sink.lock() {
                *slot = info.to_string();
            }
        }));
        let outcome = std::panic::catch_unwind(f);
        std::panic::set_hook(previous);
        assert!(outcome.is_err(), "expected a panic and got none");
        let message = captured.lock().map(|slot| slot.clone()).unwrap_or_default();
        assert!(!message.is_empty(), "the hook captured no message");
        message
    }

    /// **A corruption planted MID-SEQUENCE is caught when the door
    /// closes** — and the operator that ran after it did not catch it,
    /// which is the whole of what this unit changed.
    ///
    /// The scope stands in for a composing door: the body is corrupted
    /// raw underneath it, an operator runs and returns `Ok` without
    /// re-deriving anything, and the close fires.
    ///
    /// **The same row is the scalpel's.** Built with
    /// `--features topo/per-op-postcondition` the sweep runs after
    /// every operator again, so the panic comes from the OPERATOR and
    /// carries its name — which is how a door-level failure is
    /// localized to the operator without a replay. The two arms are
    /// the two builds, and each names what it expects to see.
    #[test]
    fn a_corruption_planted_mid_sequence_is_caught_when_the_door_closes() {
        let message = panic_message(|| {
            let mut body = cube();
            let mut door = body.begin_surgery();
            let he = a_half_edge(&door);
            plant_an_orphan_surface(&mut door);
            door.mev_line(
                MevSite::Fan { he1: he, he2: he },
                Point3::new(0.5, 0.5, 2.0),
                Tol::witness(),
            )
            .expect("mev's preconditions cannot see this corruption");
            door.sweep_and_close();
        });
        if cfg!(feature = "per-op-postcondition") {
            assert!(
                message.contains("mev postcondition: result is not tier-1 valid"),
                "the scalpel is on, so the OPERATOR after the plant should have fired and \
                 named itself: {message}"
            );
        } else {
            assert!(
                message.contains("door postcondition: result is not tier-1 valid"),
                "the door's close is what catches a mid-sequence plant; nothing inside the \
                 scope re-derives the body: {message}"
            );
        }
    }

    /// **An operator a consumer calls directly is itself a door**, and
    /// sweeps at its end exactly as it did before this unit. No scope
    /// is open, so nothing has undertaken to check the body later.
    #[test]
    fn an_operator_called_directly_still_sweeps() {
        let message = panic_message(|| {
            let mut body = cube();
            let he = a_half_edge(&body);
            plant_an_orphan_surface(&mut body);
            let _ = body.mev_line(
                MevSite::Fan { he1: he, he2: he },
                Point3::new(0.5, 0.5, 2.0),
                Tol::witness(),
            );
        });
        assert!(
            message.contains("mev postcondition: result is not tier-1 valid"),
            "a directly-called operator is a door and sweeps: {message}"
        );
    }

    /// **A nested door sweeps nothing; the OUTER door is the
    /// observable boundary.** The inner close runs at depth 2 and
    /// passes over a corrupt body; the outer close, at depth 1, fires.
    #[test]
    fn only_the_outermost_close_sweeps() {
        let mut body = cube();
        let mut outer = body.begin_surgery();
        {
            let mut inner = outer.begin_surgery();
            plant_an_orphan_surface(&mut inner);
            // Passes: the outer door has undertaken to check.
            inner.sweep_and_close();
        }
        assert_eq!(outer.open_surgery_scopes(), 1);
        let message = panic_message(std::panic::AssertUnwindSafe(move || {
            outer.sweep_and_close();
        }));
        assert!(
            message.contains("door postcondition: result is not tier-1 valid"),
            "the outermost close is the one that sweeps: {message}"
        );
    }

    /// **A door leaves no scope open, by either path** — and every arm
    /// here drives a door that actually opens one.
    ///
    /// A scope a door forgot to close silences every later operator on
    /// that body, and this is the row that sees it. It is the only
    /// check the **guardless** pair has: the two sites that cannot
    /// take a guard
    /// (`BooleanReduction::enter_join_surgery`, `splitting`'s reduced
    /// body across `split_connect`) hold their scope on a body the
    /// pipeline then MOVES into its result, so a deleted close travels
    /// out of the pipeline as a nonzero depth on a body a caller
    /// holds — which is exactly what the two pipeline arms below
    /// assert away. Deleting either surviving `leave_surgery*` line
    /// reds this row.
    ///
    /// **The arms this replaced were vacuous** and worth naming so
    /// they are not written again: `merge_coplanar_faces_declared` on
    /// an empty declaration list over a prism returns before any group
    /// is staged, and with an unresolvable declared key it refuses at
    /// the entry gate — neither reached a `begin_surgery` at all, so
    /// both passed whatever the mechanism did.
    #[test]
    fn every_door_returns_with_its_scopes_closed() {
        let tol = Tol::witness();
        let body = cube();
        assert_eq!(body.open_surgery_scopes(), 0, "a fresh build");

        // A `topo` pipeline whose reduced body crosses the ONE
        // guardless site in `splitting`, and whose two result bodies
        // are what a caller gets back.
        let plane = crate::splitting::SplitPlane {
            origin: Point3::new(0.5, 0.5, 0.5),
            normal: geom_core::Vec3::new(0.0, 0.0, 1.0),
        };
        let cut = crate::splitting::split(&body, &plane, tol).expect("a cube splits");
        for (side, part) in [("above", &cut.above), ("below", &cut.below)] {
            let Some(part) = part.body() else {
                panic!("a mid-height cut leaves material on both sides")
            };
            assert_eq!(
                part.open_surgery_scopes(),
                0,
                "the {side} side of a split came back inside a scope"
            );
        }

        // A boolean, whose reduction operands cross the other
        // guardless site and whose result body is grafted out of them.
        let offset = crate::transform_rigid(
            &body,
            &geom_core::Affine3::translation(geom_core::Vec3::new(0.5, 0.5, 0.5)),
            tol,
        )
        .expect("a rigid move of a cube");
        let united = crate::boolean::union(&body, &offset, tol).expect("two boxes unite");
        let crate::boolean::BooleanResult::Body(united) = united else {
            panic!("overlapping boxes produce a body")
        };
        assert_eq!(
            united.body.open_surgery_scopes(),
            0,
            "a boolean result came back inside a scope"
        );

        // A door that opens a scope and REFUSES: the staging clone's
        // scope is closed by the guard's `Drop`, and the destination
        // it would have been adopted into never moves.
        let mut refused = cube();
        crate::validate::validate(&refused).expect("the fixture is tier-1 valid");
        let faces: Vec<_> = refused.faces().map(|(k, _)| k).collect();
        let err = crate::replace_faces_offset(
            &mut refused,
            &faces,
            1.0,
            geom_core::Band::linear(tol).expect("a band"),
            tol,
        );
        assert!(err.is_err(), "one chart's faces are not a whole group");
        assert_eq!(
            refused.open_surgery_scopes(),
            0,
            "after a staging door returned Err"
        );

        // And the guard's own contract: dropping it closes the scope.
        let mut dropped = body.clone();
        {
            let scope = dropped.begin_surgery();
            assert_eq!(scope.open_surgery_scopes(), 1);
        }
        assert_eq!(
            dropped.open_surgery_scopes(),
            0,
            "after the guard was dropped"
        );
    }

    /// **A clone starts outside every scope.** A body nobody has
    /// undertaken to sweep must not inherit somebody else's promise;
    /// the doors that stage a mutation into a clone open their own
    /// scope on the clone.
    #[test]
    fn a_clone_is_outside_the_original_scope() {
        let mut body = cube();
        let scope = body.begin_surgery();
        assert_eq!(scope.open_surgery_scopes(), 1);
        let twin: Body<f64> = scope.clone();
        assert_eq!(twin.open_surgery_scopes(), 0);
        scope.sweep_and_close();
    }

    /// **The depth is `#[cfg(debug_assertions)]`, read back out of the
    /// source.** That attribute is the whole of the claim that a
    /// release `Body`'s layout is the one it had before this module
    /// existed — nothing at runtime in a debug build can observe the
    /// release struct — so what a bug could break here is the
    /// attribute, and this is what reads it.
    #[test]
    fn the_surgery_depth_is_declared_debug_only() {
        let src = crate::source_walk::src_root().join("body.rs");
        let text = std::fs::read_to_string(&src).expect("a readable body.rs");
        let code = test_utils::source::code_only(&text);
        let field = code
            .find("surgery: crate::surgery::SurgeryDepth")
            .expect("`Body`'s surgery depth field is declared in body.rs");
        // Everything since the previous field's terminating comma:
        // this field's attributes and its visibility keyword.
        let declaration = code[..field].rsplit(',').next().unwrap_or_default();
        assert!(
            declaration.contains("#[cfg(debug_assertions)]"),
            "the surgery depth field is no longer gated on `debug_assertions` — a release \
             `Body` would carry it, and this unit's claim that the release layout is \
             untouched would be false"
        );
    }
}
