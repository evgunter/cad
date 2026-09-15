---
id: the-decision-door-is-opaque-to-the-tier
kind: issue
title: a frame minted through Real::select_le_zero is opaque to the tier, and SYM-5's tilted derived-boss acceptance row goes red on it
status: open
opened: 2026-09-15
---


## What was measured

PROPS's sign-hull unit (PR #2468) replaces `Vec3::orthonormal_basis`'s
branchless Duff construction with one that CHOOSES a world axis through
the new value-level door `Real::select_le_zero`, which mints
`SymOp::Select` — the tier's first three-child atom. On the symbolic
lane that atom is opaque unless the decision's form can be read, and on
a frame whose normal carries a parameter it cannot be. Two of this
program's rows go red on the merge of that branch with `main`:

- `editor-core/tests/m10_derived_frame_tilted_interval::m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does`
  — SYM-5's acceptance row, rule E's. At `half = ε/8 = 1.25e-10` under
  `Guided` the derived boss no longer certifies: the cap plane's
  `newell_plane_residual` escalates on `[-1.42e-8, 1.42e-8]` against
  the band's `escalate = 1e-8`.
- `editor-core/tests/m10_derived_frame_interval::m10_the_derived_frames_refusal_is_not_a_freeze`
  — the refusal it names (`carrier_endpoint_start`, with every rule
  off) is now a `newell_plane_residual` on a side plane instead.
- `editor-core/tests/m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`
  — the pinned walk ledgers move, which is the same cause seen as a
  golden: the forms the walks build are different forms now.

**It is the tier, not the arithmetic.** The plain (numeric-only) lane
gives BIT-IDENTICAL results under both constructions on the failing
document: `boss_on_tilted(1.25e-10, derived)` under `Guided` refuses at
`carrier_endpoint_end` with the enclosure `[0, 1.5895373848295267e-9]`
either way, and the authored twin refuses under neither. What Duff's
spelling buys is that its form is a rational expression over `copysign`
and `abs` atoms that rules A/C/E can cancel; the new spelling's frame
is an indeterminate the tier cannot look inside, so every identity over
a face built on that frame freezes.

## What folds today, and what cannot

The unit's PR adds the fold that rule A0 can carry: a `Select` whose
DECISION form is a constant is decided (`d ≤ 0` on an exact rational),
so the arm is read and the atom is not minted
(`sym.rs::a_constant_decision_folds_to_the_arm_it_reads`). That covers
every axis-aligned frame and fixes two further rows of this family. It
cannot cover a parameter-dependent decision.

Rule C's certified-sign fold (`signed::fold`) is the natural next
candidate — a decision whose form has a strictly certified sign over
the parameter brackets is decided at every point of the box, exactly
the door's `Interval` semantics — **but it does not reach this case
either**: `signed::fold` requires the form to be ENCLOSABLE, every
indeterminate a parameter or π, and a tilted frame's decision
`|n.z| − max(|n.x|, |n.y|)/2` carries `abs` atoms over a `sqrt` atom
(`1/√(1+t²)`). Rule F (SYM-8, open at #2616) folds an `abs` whose sign
the form shows, which would clear the `abs` layer but not the `sqrt`
atom under it.

A shape that WOULD reach it, noted for whoever takes this: compare
SQUARES rather than magnitudes (`n.z² ≤ max(n.x², n.y²)/4`, the same
comparison on non-negative reals). Rule A's square substitution turns
`sqrt(X)²` back into `X`, so the decision becomes a rational form in
the parameter, enclosable and signable — provided `max` also folds
where one side is the zero form and the other is manifestly
non-negative (`max(0, X) → X`), which is a fold this tier does not have
(`Min | Max` folds only when BOTH sides are zero). The cost of that
shape is on the constructor's side and is PROPS's to weigh: squaring
halves the exponent range a scale-invariant comparison otherwise has.

## Home

`crates/geom-core/src/sym.rs` (`combine`'s `SymOp::Select` arm),
`crates/geom-core/src/sym/signed.rs` (the certified-sign fold) and the
`Min`/`Max` folds beside them. Found by PROPS's sign-hull unit at its
merge with `main`; filed here because the rules are this program's.
Whether the decision door should be foldable at all — and at what
gating — is a design question for SYM, and the construction that mints
it is Ev's ruling (#1944), so a change on either side is a
conversation rather than a lane's call.
