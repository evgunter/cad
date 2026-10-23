---
id: a-boolean-cuts-a-flush-face-kept-once-along-a-slits-line
kind: issue
title: A Boolean whose other solid lies flush with a slit's face beside the slit's line, where the op keeps one of the two flush faces, refuses SlitLineCrossed Flush: the fate cut has no corner to end in
status: dispatched
opened: 2026-10-20
priority: P1
cost: M
branch: cleave/fate-cut-ends-at-a-slit-tip
---


Found by the review of PR 5479 (`cleave/recl-places-germs-along-a-slit-line`).

## What

An operand carries a slit or ring in face F (PR 5380's slit form). The
other solid has an edge along the slit's line, with one of its faces
flush with F beside the line (coplanar, its flank read `Along` F's) and
its other face in the far slit wedge. recl reads two germs on the line
(`crates/topo/src/boolean/recl.rs`, `recl_edges`). Where the op keeps
one copy of the two flush faces (`tables::kept_copy`), placing them
leaves the fate stage a cut along the line that it cannot end:
`ClassificationInvariant` "a fate cut's end lies in no corner of the
dropped face" (`boolean::fate`). So recl refuses first, typed
`SlitLineCrossed { gap: SlitLineGap::Flush }`, no lever, wherever such a
flank reads `Along` on a slit's line holding two germs
(`FlankRead::along`).

## Witnesses

The plate less the void lying under its top
(`below_the_top(lying(0.0))`), against the tetrahedron at `MEET` over
`[bearing(60, 0.6, 0), bearing(150, 0.6, 0), bearing(20, 0.6, −0.7)]`
and its mirror over the ray, `[bearing(60, 0.6, 0), bearing(−30, 0.6, 0),
bearing(100, 0.6, −0.7)]`. At rest and turned, at ε default, 1e-6 and
1e-12: `x ∩ y` and `y ∪ x` refuse `Flush`, 8 cells at each ε. `x − y`,
`y − x`, `y ∩ x` and `x ∪ y` build (one germ, or no kept copy), tier 3
and 3′, the line's edges the material's creases. Pinned by
`a_partner_flush_with_the_rings_face_refuses_typed_where_one_copy_is_kept`
in `crates/topo/tests/a_ring_crossed_along_its_line_places_its_germs_apart.rs`.
Main refused all six `SlitLineCrossed`.

## Fix

Give the fate stage a corner where the germs' cut along the slit's line
ends in the dropped flush face, or place the germs so the cut runs
along an edge of it. Then drop the `Flush` arm and re-pin the cells as
building.

## Found (branch `cleave/fate-cut-ends-at-a-slit-tip`)

Two readings of the fate cut (`crates/topo/src/boolean/fate.rs`), one
mechanism under all three symptoms (this row, FLUSH's
`a-fate-cut-refuses-a-slit-floor-flush-with-an-inner-slab`, and the
`SlitFaceGlued` guard PR 5538 put in `join.rs` `resolve_roles_geometric`).

- **A slit's tip read as a cusp.** `places` (`fate.rs:376`) reads each
  corner at the cut's end by `corner_holds` (`fate.rs:343`). At a tip of
  a slit or zero-area ring, the corner's two bounds are the slit's two
  distinct line edges, leaving along one ray. The zero-turn arm held a
  direction only where both bounds were one dangling edge (`strut`),
  so a slit's tip held nothing, and the cut's end, or a corner's
  dropped sector, lay in no corner: "a fate cut's end lies in no corner
  of the dropped face". The face wraps round a slit's tip, so that
  corner is the full turn, except in a sliver
  (`slit::sliver_halves`), the join's face of no area, whose outline is
  the one segment run twice. Now `wraps` (`fate.rs:456`). The tip is
  read from the body at the cut, not from the operand at rest
  (`sectors::SlitTips`): the join also cuts a ring into the other
  operand's flush face along the slit's two edges (`the slab inside ∩
  slit`, the box `x ∩ y`), whose vertices are no tips at rest.
- **A cut along both of a slit's edges.** Where the slit's operand
  keeps its face and the other's flush face is dropped (`slit ∩ the
  slab inside`, `y ∩ x` with the box), both of the slit's edges are
  fate edges over the one dropped face, each with the kept face on its
  side. The kept face lies on both sides of the line, so the dropped
  face's fate does not change there; the two cuts made one dangling
  chord: "a fate cut leaves a slit in the dropped face". `border_fates`
  (`fate.rs:191`) now drops two fate edges on one segment by structure
  (`census::shared_segments`) over one dropped face.

With both, recl's `SlitLineGap::Flush` arm and 5538's guard no longer
fire on any scene that reaches them, and both are gone with the
`Flush` variant and `FlankRead::along`.

| scene | main | branch |
|---|---|---|
| ring, flush left and right, `x ∩ y` | `SlitLineCrossed` `Flush` | builds as `y ∩ x`, v9 e16 f8, tier 3, 3′, the line's edges the material's creases |
| ring, flush left and right, `y ∪ x` | `SlitLineCrossed` `Flush` | `SlitLineCrossed` `Closes` |
| box under the ringed top, `x ∩ y`, `lo` 0, 0.3, 0.6 | `SlitFaceGlued` | builds, `|x| − p` |

At rest and turned, at ε default, 1e-6 and 1e-12. `y ∪ x` keeps only
the top on the line, on both sides (the partner's wedge fills the
void's), so it closes the top across the line:
`a-boolean-closes-a-slits-face-across-its-line`'s gap, typed, no lever.
`x ∪ y` builds the same body (its line reads one germ).

Outcome diff, main `f742e4c97b` against the branch, at ε default, 1e-6
and 1e-12, over 810 cells per ε: the ring's partners (both flush ones,
both again with the edge ending inside the ring, and the two scenes of
`a_ring_crossed_along_its_line_places_its_germs_apart.rs`), the slit
forms of `a_keel_on_the_floor_subtracts_as_a_slit.rs` (three extents,
five poses, both slit operands) against the slab under and the slab
inside in all six ops, and the box under the ringed top at
`lo ∈ {0, 0.3, 0.6}`, `hi ∈ {1, 1.4, 0.99}`, five poses, all six ops.
Every cell that moves goes from a refusal to a build, tier 3 and 3′,
the two orders of ∩ alike, or (`y ∪ x`) from `Flush` to `Closes`;
nothing that built moves.
