---
id: a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read
kind: issue
title: A needle cavity 10.5–15ε across refuses SectionLoopUndecided at the join before any side or role is read
status: closed
pr: 5694
closed: 2026-10-25
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
  against `escalate = 9.999999999999999e-6`.

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
case).
- **The read.** Each chord of the loop lies on the other boundary.
  Where its midpoint lies strictly inside one face `F` of the other
  operand (`contain::face_placement`, the one containment ask), the
  region face `R` beside the chord leaves it along its inward direction
  (`stands::inward`, the left of a half-edge, shared with rung 3's
  `across_edges`). Its side is the sign of that direction against
  `F`'s outward normal, levered at the chord's length
  (`bool_join_seam_germ_side`).
- **Reads nothing:** zero (a flush region), a curved chord or region, a
  midpoint no one face holds, and any margin in band.
- **Every chord is read.** Two that disagree refuse
  `ClassificationInvariant`, a kernel defect, so the loop's start
  cannot matter.
- **The walk is `region_halves`**, shared with `region_faces`.
- **Errors:** a stale face panics as `place_witness` does, and a normal
  that reads the midpoint off its surface is `ClassificationInvariant`.
- **The dihedral lever door** (`classify_dihedral`) does not serve: it
  reads `|sin θ|`, not a side.
- No tie-break is represented, and no thinness enters.

**Pins** (ε 1e-9 and 1e-6, the first in the slow set; every operand and every built body
passes `validate_pseudomanifold` with no contacts):
- `shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`:
  10.5, 12, 15, 21 and 30ε; poses aligned, turned (corners turned) and
  moved (`transform_rigid`); the union in both orders and the bar
  through `∖` and `∩` in both orders.
- `shell_role_reader::a_needle_channel_whose_fill_face_leans_reads_its_side_off_its_seam`:
  the fill's face leaning `s = 0.5`, 30ε, aligned and turned. At
  ε 1e-6 this refuses `SectionLoopUndecided` on main (the review's
  measurement).
- **What each built run checks:**
  - the union reads `[Outer, Void]`;
  - the channel's own shell (the cavity, or the bar) holds its signed
    volume, `−area` or `area` (from `classify_shells`), to `1e-6` at
    ε 1e-6 and to 5% at ε 1e-9, where a turned bar's sums round by up to
    2%;
  - that shell's corners are the channel's.
- `27 − w²` is not a check at ε 1e-9: it rounds to 27.

Refusals that are not this row's, which the pins assert with their rows:
- at ε 1e-9, turned from 12ε up and moved below 30ε, every run refuses
  on the reduction's sector question (`bool_sector_within`,
  `bool_sector_coplanar`), on main too:
  `work/germcert/a-sector-direction-read-off-a-needles-short-run-carries-its-rounding-over-1-w.md`;
- at ε 1e-6, turned, 30ε, `slab ∖ fill` refuses
  `ResultVolumeImplausible`, on main too:
  `work/hone/the-subtract-volume-backstop-holds-flush-operands-bit-exact-and-refuses-a-correct-bar.md`;
- the leaning face at 10.5ε refuses `Coincidence(VertexOnFace)` before
  the join, on a corner `w/√(1 + s²)` from the face's carrier but `w`
  from the face:
  `work/hone/a-vertex-near-a-tilted-faces-carrier-but-w-from-its-face-refuses-vertex-on-face.md`.

The row's "from 21ε up … builds" holds axis-aligned only.

**Residue, not this row's.** A region face nearly flush with `F` reads
its germ in band and nothing. It and `F` are then a face pair whose
angle over the chord lies in band, which is the carrier-pair verdict's
to refuse, not the join's.

## Resolution

Closed by PR 5694 (2026-10-25). A section loop that no witness reads,
because every probe lies in band of the other boundary, now reads its
side at a chord of the loop through `seam_germ_side`. That read uses the
census's own containment ask (`contain::face_placement`). An in-band
margin, a flush region or a curved chord reads nothing, and two definite
chords that disagree refuse `ClassificationInvariant`. The cavity builds
and reads `[Outer, Void]` at 10.5–30ε in every pinned pose where main
refused `SectionLoopUndecided`. No body main built changes. The five
batteries are byte-identical to base.

A single FULL review (APPROVE-WITH-FIXES, no MAJOR) and its fix-pass check
(APPROVE) found the volume check vacuous (now the cavity's own shell
volume, shown able to fail), two unpinned poses (now pinned), and no
within-loop agreement (now refused on disagreement).

Main's refusals the pins assert as they stand, filed:
- GERMCERT `a-sector-direction-read-off-a-needles-short-run-carries-its-rounding-over-1-w`;
- HONE `the-subtract-volume-backstop-holds-flush-operands-bit-exact-and-refuses-a-correct-bar`;
- HONE `a-vertex-near-a-tilted-faces-carrier-but-w-from-its-face-refuses-vertex-on-face`;
- `work/issues/the-witness-ladder-reads-one-interior-candidate-per-face-so-one-in-band-hides-a-definite-one`.
