---
id: verbs-1031b-assigner-checker-divergence
kind: issue
title: the winding assigner/checker divergence: merge_faces assigns arc-bounded roles validate check 6 cannot check
status: open
opened: 2026-09-03
refs: [1671, m6-sense-gate-recorded-residuals]
priority: P1
cost: H
---

## The divergence

VERBS-1031B ported `boolean::join::ring_run_ccw`'s arc machinery into
`merge_faces::loop_winding`, so the `bool_ring_run_winding` predicate
now decides Line-, Circle- and Ellipse-bounded cycles at that site.
`validate.rs`'s tier-3 **check 6** — the third site of the same
predicate — still guards on `all_lines` and `continue`s past every
conic-bounded loop (`crates/topo/src/validate.rs:3549`, the guard;
`:3594`, the shared `decide` call).

The three sites no longer cover the same carrier set, and the split is
not in a harmless direction:

> **a merged face's outer/ring roles are now ASSIGNED by a functional
> the validator cannot CHECK.**

Nothing regressed — check 6 skipped those loops before this unit too,
and it skipped them back when the merge simply refused
`MergedFaceRoleAmbiguous` instead of assigning anything. What changed is
that a PRODUCER now feeds the class: before VERBS-1031B no arm minted a
merged planar face whose loops ride arcs, because the role pass refused
the whole call first. `merge_coplanar_faces` now mints them routinely
(four per teapot cup), so the uncovered class has live inhabitants for
the first time.

## The evidence: MUT-2

Measured, not argued. VERBS-1031B's mutation battery includes **MUT-2**
— `newell = newell + bulge` → `newell = newell - bulge`, the arc
correction applied backwards, a mutation strictly inside the guarded
block. Under it, on the teapot cup:

- `merge_coplanar_faces` **SUCCEEDS**. The sign flip does not make the
  roles undecidable; it makes them WRONG. Exactly one loop is still
  positively wound — it is the wrong one — so `merged_outline_ring`
  finds its unique positive cycle and swaps the ANNULUS into the outer
  slot and the OUTLINE into the ring slot.
- `topo::validate_geometric(&cup, tol)` is **`Ok(())`** on that body.
  Tier 3 is green on a body whose every merged annulus is inside out.

The inversion is caught only by the acceptance rows' own
`outer[0] > ring[1]` assertion
(`crates/sweep/tests/verbs_1031b_arcwind.rs`). That is a fixture
assertion about one shape, not a validator gate: any other producer of
an arc-bounded merged face would get no gate at all. Role inversion
passes every volume gate by construction (they are role-invariant) and
corrupts tessellation and export silently — the exact class check 6
exists to close.

## Flip condition

**Port the winding arm into `validate.rs`'s check-6 site** — widen the
`all_lines` guard at `validate.rs:3549` the way
`merge_faces::loop_winding` was widened, adding the same per-conic
bulge `axis · sa·sb · (Δ − sin Δ)` and the same arc-length re-metering,
leaving NURBS as the honest remainder. The arithmetic is already
written identically at three sites; this is a fourth statement of it,
not a new derivation.

**Its cost, which is why VERBS-1031B's fence deferred it rather than
taking it as a rider:** check 6 is a REFUSAL surface and the other two
sites are not. `join` and `merge_faces` ask the predicate a question
they need an answer to; check 6 asks it in order to FAIL a body. Its
posture today is that only a definite wrong sign refuses — Zero and
escalated windings are exempt, the check-7 posture. Widening the
carrier set therefore widens the set of bodies tier 3 can REJECT, and
does it on a margin (`2A/P` over an arc-metered perimeter) whose
in-band behaviour on real revolve output has never been measured: a
merged annulus whose mean width lands in the ambiguity band would newly
escalate where it is silently exempt today. That is a refusal-surface
change, and it deserves its own opening measurement rather than a rider
on a merge-op unit whose spec fenced it to `merge_faces.rs`.

## Schedule

A VERBS unit of its own, sized on that measurement: run the widened
check 6 over the existing revolve/shell/merge fixtures first, count what
newly refuses and what newly escalates, then decide the posture.
Sequencing is open — it is not blocking: the class has exactly one
producer (`merge_coplanar_faces`) and that producer's own output is
pinned by name in `crates/sweep/tests/verbs_1031b_arcwind.rs` in the
meantime.

Recorded against the fourth residual in
`work/props/m6-sense-gate-recorded-residuals.md:20`, which owns the flip
condition for the arc-bounded planar class.

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Adopted by CURVED** at its opening for dispatch (2026-09-04, Ev's
in-chat direction): the plan's lane that carries this item is in
`work/curved/plan.md`.

## `all_lines` itself is a third spelling, and two of the three are unnamed (ATREST-2, 2026-09-21)

The row above is about the WINDING arm diverging across three sites.
The `all_lines` GATE that decides which loops each site runs on has the
same shape and is worth naming separately, because a lane widening one
site has to find the others by reading:

- `crates/topo/src/validate.rs`, check 6's planar arm — a `let
  all_lines = cycle.iter().all(…)` binding, the only NAMED spelling;
- `crates/topo/src/merge_faces.rs`, inside `loop_winding` — **unnamed**,
  the same carrier test written as part of a larger expression;
- `crates/sweep/tests/m5_s10_face_sense.rs` — a third, added by
  ATREST-2 and self-declared *"re-derived here from the same stored
  data the arm reads"*.

The third is deliberate and stays: it is a TEST re-derivation, and a
row that pinned the arm's behaviour by calling the arm's own helper
would pin nothing. What it does show is the cost of the divergence
from the outside — the re-derivation had to restate all four of the
arm's entry conditions (planar surface, outer-plus-rings, `Cycle`
boundary, `all_lines`), and its first version got two of them wrong
(it omitted the planarity filter, which is a FALSE RED the day
`loft_body` mints `Line` carriers for straight rails, and it answered
"yes" for the non-`Cycle` boundary the arm skips, an inverted
semantic). Both were caught in review, not by a test. A fourth
re-derivation by a fourth lane is the predictable next instance.

ATREST-2's measurement of what the gate costs at rest, with its 2x2
isolation of the `Circle` carrier as the whole discriminant, is in
`work/atrest/sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts.md`
and pinned in `crates/sweep/tests/m5_s10_face_sense.rs`. It is
evidence for the widening this row schedules: on an arc-capped loft
the gate hides a WHOLE-BODY sense inversion, reached through the
public `topo::Body::set_face_sense` door, with `validate_geometric`
`Ok(())` and the enclosure unmoved and positive.

## The checker half closed for circles; the arithmetic has one home (ATREST-4, 2026-09-24)

The winding arm moved out of `merge_faces.rs` into
`crates/topo/src/loop_winding.rs` (`Body::planar_loop_winding`), and
both the merge's role assigner and tier 3's check 6 planar arm now call
it — the fourth statement this row's flip condition anticipated was
not written. The assigner's `merge_faces::loop_winding` is a thin
wrapper that maps the shared result onto `MergeCoplanarError`; its
reach is `LoopCarriers::Elliptic`, check 6's is
`LoopCarriers::Circular`. Two behaviour notes on the assigner side:
the arithmetic, its order and the escalation are unchanged; a torn
lookup on the way to a CARRIER now announces `StaleKey` where it
previously answered `Ok(None)` (a torn point lookup already
announced) — unreachable on a tier-1 body either way.

What that closes and what it does not:

- A merged face whose outer/ring roles are STORED wrongly — by any
  producer — is now falsified by check 6 when its loops ride lines and
  circles.
- An error in the shared functional itself (MUT-2's sign flip) now
  moves the assigner and the checker together, so tier 3 cannot see
  it. That is the price of one home, and it is covered by rows whose
  roles come from a DIFFERENT derivation: the extruded washer in
  `crates/sweep/tests/m5_s10_face_sense.rs`
  (`an_inverted_cap_refuses_at_its_arc_ring_as_well_as_its_outline`),
  whose roles are `profile`'s containment pass and whose chord terms
  are zero, refuses the HONEST body under a mis-signed bulge; the cup's
  `outer[0] > ring[1]` assertion in `verbs_1031b_arcwind.rs` still
  stands.
- Ellipse-bearing loops stay outside the checker:
  `work/atrest/check-6-planar-arm-skips-ellipse-and-nurbs-loops.md`.

The refusal-surface measurement this row asked for was run before the
arm landed; its table is in ATREST-4's PR.

The `all_lines` gate: the checker's named spelling is gone (the carrier
class is `LoopCarriers`, computed once in the shared function); the
test re-derivation in `m5_s10_face_sense.rs`'s `planar_arm_reaches`
stays, re-cut to `Line`-or-`Circle`.
