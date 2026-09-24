---
id: plain-transform-rigid-still-refuses-the-m7-8-class
kind: issue
title: transform_rigid_via gives the capability but the plain door still refuses an M7-8 body, and closing that gap needs a compound-bound ratification
status: open
opened: 2026-09-12
refs: [2418, graft-recertifies-through-the-narrow-lane]
priority: P0
cost: D
---



(FIX orchestrator, 2026-09-12) The half of
`transform-recertifies-through-the-narrow-lane` that PR 2418 could not
take, filed at the moment of disclosure rather than left in that PR's
"where I was unsure" list.

## What 2418 fixed and what it did not

**Fixed:** an M7-8 plane × described-NURBS body can now be moved —
`topo::transform_rigid_via` with `geom_brep::plane_nurbs_limbs`, the
mint-side twin `EdgeCurve::certify_via` beside the existing
`recertify_via`. The kernel no longer lacks a door for a body it
certifies at rest.

**Not fixed:** `transform_rigid` — the plain door, the one a caller
reaches for first — **still refuses that body**, typed, with
`CertifyError::Unimplemented`. PR 2418 pins exactly that
(`m4_pr2_transform::an_m7_8_body_validates_at_rest_and_moves_through_the_lane`
asserts the refusal and then the success through the lane door), and
argues at the site that *"the door a caller takes decides whether the
map is attempted at all, and the lane-free door's refusal is a fact
about that door's rights, never about the body."*

That framing is honest and matches how `validate.rs` and `euler.rs`
already work. **It still leaves a user who calls `transform_rigid` with
a refusal they can only escape by knowing a second door exists.**

## Why the obvious fix is unavailable, and it is a RATIFIED rule not a preference

The parent item prescribed raising `transform_rigid`'s bound to
`T: Decide + geom_core::CertifiedBounds`. **That does not compile**, and
the reason is structural:

- `transform_rigid` has a generic caller chain —
  `boolean::ops::apply_recuts` → `boolean_op_recut` → `boolean_op_with`
  → `verbs::Verb`'s `impl<T: Decide + Bounds + PcurveFittedLane>` →
  `editor_core::evaluate::<Dual64>`.
- `CertifiedBounds` is blanket over `Bounds + CertifiedEnclosure`, and
  `CertifiedEnclosure` is implemented for `Interval`, `RingInterval`,
  `f64`, `Sym<T>` and `Probe` — **no `Dual`**. Verified at
  `crates/geom-core/src/{real,interval,ring_interval,sym,k_stats}.rs`.
- `crates/geom-core/src/real.rs:1140` records the discriminator **Ev
  ratified in conversation (2026-08-29)**: *"the discriminator is that
  nothing generic calls this door"* — which is why `topo::separation`
  was not tightened and `editor_core::checks` was. `transform_rigid`
  fails that test.

So the parent item's fix would have broken the `Dual`-instantiated
boolean chain **and** violated a ratified rule. That is why 2418 took
the lane-as-argument shape instead.

## What this row actually asks

A `transform_rigid_certified` convenience door — the plain ergonomics
with the lane supplied — **needs a compound-bound ratification in
`real.rs`, which is Ev's call and not a lane's** (PR 2418's lane said
so and declined to make it). The alternatives worth putting beside it:

1. **The convenience door**, with the ratification.
2. **Do nothing and document at `transform_rigid`** that the lane-free
   door is lane-free by construction and name `transform_rigid_via`
   there — cheapest, and it converts a silent trap into a signposted
   one. Check whether 2418 already did this before assuming it did not.
3. **Decide the asymmetry is correct** and say so once at the door: a
   caller that has not named a lane has not earned the wider class.
   That is 2418's argument, and if it is right this row closes with a
   sentence rather than code.

**Establish which before writing anything** — this program has closed
three rows by refuting them and one by discovering its own prescribed
fix does not compile.

## Also open, from the same lane

`work/fix/graft-recertifies-through-the-narrow-lane` —
`boolean/combine.rs`'s `graft_solids_with` is the same shape with a
weaker bound (`T: Decide`), filed by 2418's lane with reachability
explicitly NOT established.

## Re-homed to SHELL, 2026-09-20

(FIX orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* FIX is the
program for rows whose fix is already written; a row whose blocking
question is a DESIGN decision belongs to the track that owns the surface
the decision is about.

**The decision this row is blocked on:** one of three: a `transform_rigid_certified` convenience door (which needs a
compound-bound ratification in `crates/geom-core/src/real.rs`), a signpost at
the plain door naming `transform_rigid_via`, or deciding the asymmetry is
correct and saying so once.

**Why SHELL.** `crates/topo/src/transform.rs` is **owned by shell**, and SHELL's slate
already carries this row's siblings: `transform-rigid-refuses-approx-face` (the
Approx arm of the same two matches, named in FIX's own `keep_out`) and
`no-approx-faced-body-is-both-movable-and-valid`. The transform split is SHELL's
family, not FIX's.

**Ev declined to rule it in chat on 2026-09-20 and sent it here instead**, so
no option is pre-empted; option 3 is PR 2418's own argument and would close the
row with a sentence. The ratification option is the one that is NOT a lane's to
take: `real.rs` is PROPS's ground and carries Ev's 2026-08-29 discriminator
(*"nothing generic calls this door"*), which `transform_rigid` fails because
its caller chain runs through `verbs::Verb` at `Dual64`. Check first whether PR
2418 already signposted the plain door before assuming it did not.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for
you except where this note says Ev answered it.

## Reference note (FIX's sweep, 2026-09-21)

`transform-recertifies-through-the-narrow-lane` was dropped from this row's `refs` because the row closed with **FIX**, which left the tracker at sweep 18 — `work/fix/` is deleted and `docs/DOC-LEDGER.md` is its done-state of record. The finding is unchanged and still readable: `git show 6f0e04ce1534:work/fix/transform-recertifies-through-the-narrow-lane.md`.
