---
id: a-strut-rings-side-is-read-from-its-own-germ
kind: issue
title: A strut ring no path places is read from its own germ: the ball-donut spline placement stops where its polygon completes inside an unplaced strut ring
status: open
opened: 2026-11-28
priority: P1
cost: M
refs: [a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate]
---


Filed by the LATHE strut-ring lane (`lathe/strut-ring-homing`,
2026-11-28) on the orchestrator's ruling. It is the residue of
`a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate`,
a regression from the two-band base (84588ee6fa built all four
placements).

## What

`sweep` `germ_sphere_torus_placements`, the spline placement (the ball
turned 1.1 rad about `(0.3, 1, 0.2)` and moved to `(0.42, −0.15, 0.3)`,
against the donut at the origin):

- When the donut's torus face (2v1) is divided, run 3v5 walls off two
  strut rings, pierce rings of two null halves each: 6v1 and 7v1. No
  path decides either one (`chord_join::path_ring_side` → `Undecided`).
- Since `lathe/strut-ring-homing`, both are deferred
  (`ChordJoiner::rehome_rings`, as an `OnRun` one on a plane is).
- A later join then matches two halves of 7v1 itself. Its own polygon
  completes inside it, so no join can place it, and `place_pending`
  refuses `RingHomingAmbiguous { ring: 7v1 }`.
- With the donut first, ∪, ∩ and ∖ stop there. With the ball first they
  build (`the_spline_edge_placement_under_every_op`).

## What it needs: option (b)

Read a strut ring's side at the division that meets it, from its own
germ:

- The ring sits at a pierce point `p`, on the run or grazing it. Its
  null halves carry the germs along which its section polygon leaves
  `p`.
- Read the germ against `newf`'s inward direction at `p` (the run
  edge's tangent there, crossed with the face's outward normal), as
  `boolean::join::germ_side` reads a chord's region.
- Positive is In and negative is Out. In band or zero defers, as now.
- The premise: the ring does not cross the run (`paths_rung`'s), so its
  polygon lies on its germ's side of the run near `p`. A path that
  reaches `rehome_rings` without that premise must refuse the germ
  reading.

**The plumbing.** `rehome_rings` belongs to `chord_join::ChordJoiner`,
which the boolean and the split lane share, and the joiner carries no
germs. Each lane would hand it a half → germ lookup:

- the boolean, from its reduction's per-operand null-edge records
  (`NullEdgeRecord::germs`, `HalfGerm::dir`);
- the split, from `red.apex_germs`.

## Not option (c)

The alternative was to read the side at `place_pending`'s `l1 == l2`
case, where the germ is in scope (`boolean/join.rs`'s call,
`splitting/join.rs`'s). It was weighed and ruled out:

- It needs the run the ring was deferred against, which `pending` does
  not record (`SecondaryMap<LoopKey, ()>`).
- That face may have been divided again between the deferral and the
  placement, so the reading would be of a run that has since changed.

## Pin

- `germ_sphere_torus_placements::a_ball_against_a_donut_carries_onto_the_torus_and_builds`
  is pinned red at every ε at `RingHomingAmbiguous { ring: LoopKey(7v1) }`.
- `the_spline_edge_placement_under_every_op` pins the donut-first ops'
  stop, and holds the ball-first ops to tier 3′ and inclusion–exclusion.
