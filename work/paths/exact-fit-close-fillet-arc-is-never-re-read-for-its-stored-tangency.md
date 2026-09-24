---
id: exact-fit-close-fillet-arc-is-never-re-read-for-its-stored-tangency
kind: issue
title: The exact-fit close's fillet arc is absent from fillet_arcs, so nothing re-reads its stored tangency: a 2x bulge there passes every row in both crates
status: closed
opened: 2026-09-19
closed: 2026-09-19
branch: edit/radius-emission-record
pr: 2892
---


Found by both review lanes on PR 2892
(`edit/radius-emission-record`), which disclosed the shape as an
observation it could not adjudicate. Measured here; it predates that
unit and is `crates/profile`'s ground. (`radius-r1` filed the same
finding as `exact-fit-close-skips-the-fillet-tangency-re-read`;
one row, so that file is deleted in the commit that folds its two
additions — the other four recording sites, and the two doc
sentences — into this one.)

## The finding

`family::resolve_arc_close`'s EXACT-FIT arm
(`crates/profile/src/path/family.rs:374-388`) makes the fillet arc the
closing segment through `Core::set_leaving(trims.bulge, FirstSeg::Arc)`
WITHOUT going through `Core::record_fillet_arc`
(`crates/profile/src/path.rs:2455`). So that one arc never enters
`Core::fillet_arcs`, and `Core::fillets_carry_their_tangency`
(`crates/profile/src/path.rs:2949`) — the close-time re-read that
classifies each stored fillet arc back off its chord and bulge and
refuses `FilletArcFlattenedInStorage` / checks the declared joints
around it — never sees it. Every other fillet arc in the crate is
re-read; this one is not. The four sites that DO record are
`Core::emit_fillet_arc`, `resolve_fillet`'s two arms and
`resolve_arc_pending_ray_arrival`'s seam arm.

Two doc sentences are false for this branch as a result:

- `Core::build`'s doc (`path.rs`): "the door's output is a loop whose
  fillet declarations the stored form carries, or it is a refusal".
- `fillets_carry_their_tangency`'s reach doc lists "every door but
  two" at which a fillet's own joint is declared; the exact-fit close
  is a third door, and there the arc is not in the list at all.

**The reason the PR gave for leaving it is measured false.** The PR
body argues that "the close's own junction check covers the same
joint". It does not cover the same FACT:
`crates/profile/src/path/family.rs:379-387` hands `junction_check` the
COMPUTED carrier `trims.arc`, not the bulge that was stored. The
stored form is never read back on this path at all.

## The measurement

Mutating that one line to store a bulge twice the computed one —

```
core.set_leaving(trims.bulge * T::from_f64(2.0), FirstSeg::Arc)?;
```

— leaves the closing "fillet" arc stored at radius 1.25 where the
authored radius is 1.0, and:

- `cargo test -p profile --test all`: 466 passed, 0 failed (the whole
  suite, including
  `fillet_stored_tangency::an_exact_outgoing_fit_leaves_its_joint_undeclared_and_still_validates`,
  which builds exactly this chain and calls `validates`).
- `cargo test -p editor-core --test all`: 1529 passed, 0 failed.

Nothing anywhere reds. The only row that reds is the review lane's own
probe, which reads the stored chord-and-bulge back
(`crates/profile/tests/review_radius_emission_r2_probes.rs::the_exact_fit_close_records_its_fillet_on_the_closing_segment`,
adopted on `review/radius-r2`).

The chain that reaches the branch is the line × arc corner at the
radius that consumes its outgoing side exactly:

```
Open.at(p2(0.0, 2.0)).line_to(p2(0.0, 0.0), tol())
    .toward(2.0, 0.0, tol())
    .fillet_arc(1.0, Center { c: p2(0.0, 0.0), winding: Ccw, p: Start }, tol())
```

`arc_fillet::fillet_leg_fit_trio_definite_and_exact`,
`arc_fillet::fillet_offset_line_circle_trio` and
`cert4r2_e2e::a_true_tangency_classifies_as_an_exact_fit_through_the_public_door`
reach the same branch, so it is not an exotic corner.

## What a taker does

Decide whether the exact-fit close's arc belongs in `fillet_arcs` — it
is a fillet arc by every other definition, and the field's own doc
calls itself "every fillet arc emitted into the chain" — or whether
the omission is deliberate, in which case the reason belongs at the
branch, because the reader who wrote the PR's observation looked for
one and found none. Either way one row that reds when that segment's
STORED form stops being the arc the resolver computed is what the
shape is missing; the review probe above is the shape of it.

## Closed — the arm goes through the shared door (2026-09-19, PR 2892's fix pass)

Ruled by the EDIT orchestrator on the union of the two reviews: the
exact-fit arm routes through `Core::record_fillet_arc` like every
other fillet-arc emission, so the arc joins `fillet_arcs` and
`fillets_carry_their_tangency` re-reads its stored form.

**The re-read HOLDS for this arc, measured both ways.** With the arm
still hand-writing its emission, `core.set_leaving(trims.bulge *
2.0, FirstSeg::Arc)` passes the whole profile suite except the row
that reads the stored chord-and-bulge back
(`path_program::the_exact_fit_close_records_its_fillet_on_the_closing_segment`,
adopted from this lane): 467 passed, 1 failed. With the arm routed
through the shared door the same mutant is refused BY THE DOOR —
`FilletArcFlattenedInStorage`'s sibling
`FilletCarrierBelowSceneResolution { turn: 3.21, radius: 0.5,
predicate: "carrier_line_circle", margin: 0.083 }` — before `build`
returns, and both exact-fit rows red on that refusal. The arc's
INCOMING joint is the declared one `emit_fillet_in` pushes; its
outgoing joint is the seam, which `junction_check` requires
transversal and no door declares.

Both doc sentences are true again: `Core::build`'s promise now covers
this branch, and `fillets_carry_their_tangency`'s reach paragraph
names the three doors that leave a fillet's OUTGOING joint undeclared
and says for each why the direction leaving the arc is not the door's
to claim — the exact-fit close's reason being the seam check beside
it, not a free direction.
