---
id: a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read
kind: issue
title: A needle cavity 10.5–15ε across refuses SectionLoopUndecided at the join before any side or role is read
status: review
pr: 5694
opened: 2026-10-19
priority: P2
cost: M
refs: [5119]
branch: flush/needle-cavity-section-loop
---


Found by PR 5119's lane while measuring the needle class (2026-10-19).

**What happens.** `brick ∖ bar`, where the bar is `(0,1)×(0,w)×(0,w)`
with w 10.5–15ε, at ε 1e-9, refuses at the join with
`SectionLoopUndecided`. This happens before any result exists, so the
side reader never sees it. From 21ε up, and in R1's posed sweep, the
cavity builds and reads `[Outer, Void]`.

The pairs are definite (> Kε). So this is a thinness refusal in the
join's section-loop decision, which `[coincidence.booleans]` ("how thin
a result is gets judged by its pairs alone") does not allow.

**What is owed.**
- Find which margin the section loop holds against the band at a
  needle's narrow faces: likely an in-face walk near a loop boundary,
  the same shape the edge-meeting ray removes from the side reader.
- Say whether it decides a tie the band forbids (`[verdicts]`), or
  measures a real pair.
- Fix it at the join (`join.rs` is FLUSH's), and pin the cavity at
  10.5, 12 and 15ε at ε 1e-9 and 1e-6, both orientations.

## Measured

On main (merge base `28e9e447`), with the probe
`crates/topo/tests/needle_cavity_probe.rs` (`#[ignore]`d; its doc has
the run line) at ε 1e-9 and 1e-6.

- **The plain `brick ∖ bar` builds** at every width (10.5–30ε) and in
  both poses, and so do `bar ∩ brick` and `brick ∪ bar`: the
  bar lies inside the brick, crosses nothing, and the containment
  fallback reads it. The refusal is the R2 pose the
  `shell_role_reader` doc named: `brick (−1,2)³ ∖ slab`, then `∪` a
  fill flush with the cavity on five faces, which leaves a `w × w`
  channel. Its bar alone refuses the same way through the other two ops:
  `slab ∖ fill` and `slab ∩ (brick ∖ fill)`.
- **All three refuse `SectionLoopUndecided`** at 10.5, 12 and 15ε, at
  both ε, axis-aligned. Turned about `(1, 2, 3)` by 1.1 rad, they
  refuse it at ε 1e-6, and at 10.5ε at ε 1e-9. The union of the
  axis-aligned operands moved by `transform_rigid` refuses it at
  ε 1e-6 at 10.5, 12, 15 and 30ε (21ε builds), and at ε 1e-9 at 30ε.
  Below 30ε at ε 1e-9 the sector question refuses first (below).
- **Which comparison.** `join.rs` `resolve_roles_geometric` →
  `loop_roles`. Both loops of the fill's null face (the section polygon
  round the fill's `y = 0.5 + w` face) read `Undecided`, with no limit
  met. The outer loop's regions are the four faces flush with the
  cavity, and all 24 of their witnesses read `OnBoundary`. The ring
  loop's region is the fill's face toward the channel, `w` wide, whose
  rim lies on the cavity's floor, ceiling and end walls. Its vertices
  and edge midpoints read `OnBoundary`, and its interior witness
  (`stands::face_interior_point`, rung 3) reads
  `bool_point_in_solid_plane` in band: at 10.5ε, `5.25e-9` above the
  floor and inside the floor's trim. A face narrower than `2Kε` holds
  no point off the band of the faces its rim lies on. In the moved
  pose at 30ε the first candidate, a corner triple's centroid, lies
  `w/3 = Kε` off the floor, and rounding puts it at `9.999999999996e-6`
  against `escalate = 1e-5`.

## Classification

(c). Every pair is definite: the fill's face and the cavity's wall
`y = 0.5` are `w` apart, and the fill's face meets the floor and
ceiling at a right angle. The refusal is a point read near a loop's
boundary, the same shape PR 5422 took out of the side reader. It is
not (a): no pair is in band, and the point it reads is the ladder's own
witness. It is not (b): the answer cannot depend on where in the band
the witness falls, since every point of the face is on one side.

## Fix

`join.rs` `seam_germ_side`, which `resolve_roles_geometric` reads where
no witness of either loop decides and no limit was met (after the slit
case). Each chord of the loop lies on the other boundary. Where its
midpoint lies strictly inside one face `F` of the other operand
(`contain::face_placement`, the one containment ask), the region face
`R` beside the chord leaves it along `n_R × t`. Its side is the sign of
that direction against `F`'s outward normal, levered at the chord's
length as an edge's dihedral is (`bool_seam_germ_side`). Zero is a
flush region and reads nothing. So do a curved chord or region, a
midpoint no one face holds, and any margin in band. No tie-break is
represented. The read is about two crossing carriers, so no thinness
enters.

Pinned by `shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`
(slow set) at 10.5, 12, 15, 21 and 30ε, ε 1e-9 and 1e-6,
axis-aligned and turned. Each built body passes `validate_pseudomanifold`
with no contacts, and so does every operand. The union's roles read
`[Outer, Void]` and its volume is `27 − w²` to `w/100`. Each bar's eight
corners are the channel's.

Two refusals in the turned pose are not this row's, and the pin pins
them with their rows:
- at ε 1e-9 from 12ε up, all three refuse on the reduction's sector
  question (`bool_sector_within`, `bool_sector_coplanar`), on main
  too: `work/germcert/a-sector-direction-read-off-a-needles-short-run-carries-its-rounding-over-1-w.md`;
- at ε 1e-6, 30ε, `slab ∖ fill` refuses `ResultVolumeImplausible`, on
  main too:
  `work/hone/the-subtract-volume-backstop-holds-flush-operands-bit-exact-and-refuses-a-correct-bar.md`.

The row's "from 21ε up … builds" holds axis-aligned only.
