---
id: a-flat-and-a-slanted-rim-on-tangent-walls-escalate-tier-3-prime-or-refuse-seam-orientation
kind: issue
title: Two rims crossing on internally tangent walls refuse SeamOrientation when the inner rod's cut is steeper, and escalate tier 3′ on two ops otherwise
status: dispatched
opened: 2026-10-30
priority: P1
cost: M
branch: splitdoor/rims-seam-orientation
---


Found by `splitdoor/sheets-part-conic`'s fixtures for
`a-boundary-crossing-tangent-reads-lines-and-circles-only-in-sheets-part`
(`crates/sweep/tests/sheets_part_on_slanted_rims.rs`), the tilt pattern
by that PR's review (PR 6067). Measured at ε = 1e-6, 1e-9 and 1e-12, on
main and on that branch alike: the `sheets_part` change moves neither
finding.

## The fixture

An inner rod of radius 1 about `(1, 0)` is cut below `z = y·tan t_a`.
An outer rod of radius 2 about the origin is cut above `z = y·tan t_b`.
Each is a two-arc extrude over `z ∈ [−3, 3]` whose vertices sit at
`±y`. The walls are tangent inside one another along `x = 2, y = 0`.
The rims cross at `(2, 0, 0)`, each a circle where its tilt is zero and
an ellipse elsewhere, and the walls part there. Closed forms:
`A = 3π`, `B = 12π`, and they share `(2/3)|tan t_a − tan t_b|`.

## 1. `SeamOrientation` wherever `tan t_a > tan t_b`

Every op refuses `SeamOrientation`, the orientation chain's kernel-bug
invariant, at `(t_a, t_b)` = `(0, −0.5)`, `(0.3, −0.4)`, `(0.5, 0.2)`
and `(0.5, 0.45)`. Every op builds at `(0, 0.3)`, `(0, 1.0)`,
`(0.2, 0.5)`, `(−0.3, 0.6)` and `(0.45, 0.5)`. The verdict follows the
order of the two planes' slopes, right up to their near-equality. It
does not depend on which rim is flat. The walls' parting
(`bool_touch_sheets_part`) is read, Negative, on both sides.

The raising site, measured at `(0, −0.5)`, is
`crates/topo/src/boolean/zip.rs`'s ring-match arm (`None if leaves`): a
seam half-edge that leaves the face has no ring half-edge matching it.
It is not the one-vertex sense check below it. Pinned in
`sheets_part_on_slanted_rims::rims_crossing_with_the_inner_plane_steeper_refuse_seam_orientation`
(slow set); the fix updates that pin.

**Why P1.** A boolean that refuses as a kernel bug on a legal input
(`work/README.md` Priority).

## 2. Tier 3′ escalates on `A ∪ B` and `B ∖ A` where the ops build

Every op builds at its closed form within its pad and passes tiers 1–3.
`A ∖ B` and `A ∩ B` pass tier 3′. On `A ∪ B` and `B ∖ A`,
`validate_pseudomanifold` returns `CensusEscalated` with
`predicate: "curve_pair_coplanar_sub_side"`
(`geom_brep::curve_pair::pose`). At `(0, 1.0)` the margins are about
−2.04ε, 1.47ε and 1.78ε, at every ε. Those bodies keep `B`'s cut cap
with the pinch at `(2, 0, 0)`. Which pair of curved edges the census
reads there is not yet measured. Pinned in
`sheets_part_on_slanted_rims::rims_crossing_on_tangent_walls_build_every_op`
(slow set).

## Root cause of 1 (2026-10-30, `splitdoor/rims-seam-orientation`)

The cause is not in the zip. The zip catches a section loop on B that is wound backwards. At the pinch `(2, 0, 0)` B's cut cap has a corner of exactly a half-turn, bisected into two entries at `−x`. Two of B's germs leave into that corner:
- the cut-planes' line, along `−x`, on the bisector;
- the section arc of B's cap across A's wall, along B's rim's tangent: the walls are tangent, so the arc and the rim are tangent there.

Both germs fall in the corner's first entry, so their null edge is a strut, and its two halves are faced by `insert::walk_faces_first` → `strut_order`. That reading measured each germ against the arrival edge's **chord** (`anchor_dir`, the line to the edge's far vertex), not its tangent. B's rim is curved, so its chord lies inside the corner. The arc germ, which leaves along the tangent, read on the chord's far side: past a half-turn, after the line. So the strut's `he_plus` faced the line, and B's section loop was spliced backwards (B's In region, signed area −2.27 about its outward normal against +2.27 in the mirror pose). In the mirror pose (`tan t_a < tan t_b`) the arc leaves at the corner's departure end, where the chord's error moves no order. That is why only one handedness refused.

**The fix.** `walk_faces_first` reads the arrival bound's tangent, which the corner's first sector already holds (`BoolSector::end`), so inside one entry it agrees with `walks_after`, as `strut_order`'s doc says. Every pose above now builds every op at its closed form, tiers 1–3, at ε = 1e-6, 1e-9 and 1e-12, and tier 3′ passes on `A ∖ B` and `A ∩ B` (`sheets_part_on_slanted_rims::rims_crossing_on_tangent_walls_build_every_op`).

## 2, measured after the fix

The tier-3′ escalation on `A ∪ B` and `B ∖ A` is unchanged, in both handednesses. The census's curved edge-edge pass (`census::curved::interior_meeting`) reads `geom_brep::curve_pair::meet` on B's cut cap in the result: the section arcs (A's wall across B's plane, an ellipse about `(1, 0)`) against the pieces of B's rim (an ellipse about the origin). The two carriers are coplanar and internally tangent at `(2, 0, 0)`. Every such pair answers `MeetFault::Touch` on `curve_pair_coplanar_sub_side` at about 2ε, which the census types `CensusEscalated`:
- **A shared vertex.** Pairs whose edges share `(2, 0, 0)` as an end, e.g. the arc piece `(2, 0, 0)–(1, ±1, z)` against the rim piece from `(2, 0, 0)`, touch at that common end. This is `work/cpair/two-coplanar-ellipses-have-no-meeting-method.md`'s open touch.
- **Off the span.** Pairs whose arc ends nowhere near the touch, e.g. the arc piece `(1, ±1, z)–(0, 0, 0)`, escalate on the carriers' touch, off that arc's span. This is `work/cpair/the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc.md`'s shape.

Both are cpair's ground, and the escalation is an honest one: a touch the reader cannot place, in band. It stays as is. The evidence is added to both rows, and the pin above asserts it.
