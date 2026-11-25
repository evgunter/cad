---
id: a-cell-past-an-iso-edges-end-is-clipped-by-its-whole-line
kind: issue
title: A curved face's cell that holds the end of an iso edge, beside a reflex corner or a non-iso edge, is clipped by that edge's whole line and reads Local at every depth
status: closed
opened: 2026-11-21
priority: P0
cost: M
pr: 7327
branch: embed/iso-end
closed: 2026-11-23
---


## Finding

Found by EMBED's lane `embed/trace-cylinder` (2026-11-21), tracing the
cylinder buckets of the per-solid census (`measure/gate/census-c2a0c41f40.json`
on `embed/gate-measure`; the at-rest gate row's "Census, second take").
**83 tests** of valid bodies refuse at their door on it: 73 of the
plane/cylinder and cylinder/plane edge buckets (32 and 41), 10 of the
cylinder/cylinder edge bucket.

`present_chart` (`crates/topo/src/injective.rs`) clips the cell's
rectangle by the line of every iso edge near it, placed on the face's
sheet, and keeps a line on the side its face is on. It then asks
`sides_are_the_faces` (`:1682`, called at `:2104`) that every clip's side
be its edges' own, run on past a corner it makes with a clip on the other
axis (`run_on`, `:1730`), and that every such corner be convex
(`convex`). Where a cell holds the END of an iso edge and the corner
there is not a convex corner of two clips, that check is false at every
depth, rightly: the rectangle the clips leave is not the face's part of
the cell. The presentation returns `Unpresented::Local`, the pair splits,
and one child always holds the vertex again, so the ask ends at
`Budget(Depth { max_cell_depth: 40 })`, reported as
`Uncertified { arm: Departure }` across the shared edge.

Two shapes, both measured with the clips printed (`CVF` rows of the
measuring branch `embed/trace-cylinder-measure`):

- **A reflex corner of two iso edges (63 tests).** A ruling and a circle
  of a cylinder face meet at a vertex where the face has 270° (a notch
  cut into a wall's rim, a pocket across a rim, a joined rim arc). The
  cell sits in the face, beside the corner, straddling one edge's line
  on the far side of the vertex from the edge. Both clips fail the
  cover and the corner fails `convex`. Example,
  `editor-core names::emit_shell::tests::a_joined_ring_arc_is_named_as_the_set_of_its_halves`
  (cylinder `4v1` against plane `8v1` across `13v1`), at depth 40:
  cell `u ∈ (0.098175, 0.101243)`, `v ∈ (0.049219, 0.05)`; clip `XHi`
  at `u = 0.100167` from the ruling `13v1`, whose range is
  `v ∈ [0.05, 0.75]`, above the cell; clip `YHi` at `v = 0.05` from the
  circle `16v1`, whose range is `u ∈ [0.100167, 3.0414]`, right of the
  line. The face excludes only the quadrant `u > 0.100167, v > 0.05`,
  so the whole cell is the face's, and the line `u = 0.100167` below
  the vertex is no boundary of it.
- **An iso edge ending at a vertex whose other edge is no clip in the
  cell (17 tests).** One clip, its edge stopping at the vertex inside
  the cell's side. Example,
  `editor-core::all intent_s6_d_tangency::a_heading_typed_along_the_tangent_is_unproven`
  (plane `2v1` against cylinder `7v3` across the cylinder's edge 2):
  clip `YLo` at `v = 0` from the circle `7v1`, range `u ∈ [π/2, π]`,
  against the cell's `u ∈ (1.567728, π/2)`. The family is the
  tangent-boss documents of `intent_s6_*`, `coincidence_door`'s
  tangent boss, a seam ruling on the other wall
  (`parallel_cylinder_join`), a box-corner ruling. In ten of these the
  plane partner is unpresented too (`present_curved` reads no column,
  `graph` fails): the corner's other edge leaves the vertex tangent to
  the circle, so a fix here alone may not close those ten.

The cousin across the corner, a cell OUTSIDE the face touching a
corner, is `vacuous-reads-a-cell-at-a-corner-local-at-every-depth`;
this is the cell inside.

## Valid or in band

Valid. Every body is a finished solid its test builds as an ordinary
result (a notched or pocketed rim, a boss, a joined arc); none is built
at the band's edge on purpose. The tangent-boss family declares an
unproven tangency at its Boolean's door; the body itself is a valid
embedding with a tangent corner on the wall.

## What a fix needs

Not a presentation of the corner where the carrier arm reads it (measured
and argued by `embed/iso-end`, 2026-11-22, below). The cylinder's cell is
refused at `sides_are_the_faces` rightly. No one presentation of the
face's part, the cell less a quadrant, is signed by a first-order
separator holding the shared edge's carrier (`planes_through`,
`graph_of`, the partner's carrier): it vanishes on that edge's chart line
past the corner, the face's interior, and the part lies on both sides of
it. Nor by one through the vertex alone. The carrier arm reads the corner
with the face's own carrier against the partner's presentation where (i)
the face has a carrier implicit, (ii) the partner is presented and (iii)
the partner lies on one side of the face's whole carrier in the cell.
Across a shared edge with a transverse partner (iii) holds for a valid
body; at a vertex alone (four or more faces there) a valid partner may
cross the carrier in the quadrant the face lacks, and that pair is
refused at every depth. There, and where (i) fails, the reading is the
cell cut at the vertex into two convex pieces each certified alone
(`embed/corner-pieces` for a plane;
`a-reflex-corner-of-a-face-with-no-implicit-carrier-has-no-reading`).

What failed in this row's tests was the carrier arm's read: `Psi`'s
gradient over the cell's image was a box enclosure of the cylinder's
radial unit, dotted with the chart direction, and along a ruling the
direction is the axis, against which the box's slop is all the dot reads.

The lone clip's tests are not this read (below): tangent contacts and
partners unread.

## Evidence

- Cheapest reproducers: `step-import::all s58_iso_rectangle::tee_the_one_sided_variant_imports_and_measures_exactly`
  (0.17 s, reflex), `editor-core::all intent_s6_d_tangency::a_heading_typed_along_the_tangent_is_unproven`
  (0.35 s, one clip).
- Run on `embed/trace-cylinder-measure` with `CV_TRACE=1`: the refusing
  pair's last presentation is `Err(Local)` from `present_chart` at
  `sides_are_the_faces`, the partner `Of(Side)`; the `CVF` rows print
  each clip's side, line, edges, ranges and `covers`, and each corner's
  `convex`.
- Checked: the clip ranges show the edge off the cell's side in every
  traced test (63 + 17); the reflex reading is read off those ranges and
  the faces' sides, not flipped by an edit.
- Three slow tests (`spline_spiric_operands`, `pocket_wall_crossing_a_side_face`,
  one `m10_p_fence` row) reach the same exit, traced without the clip
  print.

## For the unit's brief: what each shape's cell holds (EMBED, lane embed/trace-cylinder, 2026-11-21)

Second pass, `embed/trace-cylinder-measure` (`CVF`, `CVC` rows).

- **Reflex corner (63).** The face excludes one quadrant at the vertex
  `V`: the one bounded by the two edges' half-lines. A cell at depth 40
  lies in the face's three quadrants, straddling one line's extension
  past `V`. In the measured example (`names::emit_shell`, ruling
  `u = 0.100167` for `v ≥ 0.05`, circle `v = 0.05` for `u ≥ 0.100167`,
  cell `v ∈ (0.049219, 0.05)`) the cell is wholly below the circle, so
  it holds the **whole rectangle**. Its zero is the point `V` on its top
  side, because the shared ruling leaves `V` upward, out of the cell. A
  cell straddling `V` on both axes holds the cell less the excluded
  quadrant, an L of two rectangles. A presentation could hold this
  shape (the rectangle unclipped by a line past its edge's end, with a
  point zero on a side), but no separator would sign it: a separator
  holding the shared ruling vanishes on the ruling's line below `V`,
  which crosses the cell (What a fix needs). The point zero
  `a-shared-vertex-the-chart-holds-on-one-clip-only-reads-no-zero` asks
  for is independent of this row: its reading neither supplies nor
  needs it.
- **Lone clip (17).** The cell lies along an iso edge's line just past
  its end `V`. The edge leaving `V` the other way is no clip in the
  cell.
  - In the tangent-boss family (`intent_s6_*`, `coincidence_door`'s
    boss, 10 of the 17), the iso edge is the shared arc itself (`7v1`,
    the circle `v = 0` over `u ∈ [π/2, π]`). The cell is at
    `u ∈ (1.5677, π/2)`, `v ∈ (0, 0.00098)`.
  - In `parallel_cylinder_join` (4) it is a ruling `4v1` ending at
    `12v1`, which the partner wall continues by a collinear ruling
    (`9v5`) on its own clip.

  What the cell holds is a corner of the iso edge and the vertex's other
  edge, which is curved or not on a coordinate line. The reading is the
  line corner (`present_line_corner`) with the iso edge as its line;
  `present_chart` never offers it, since the curved wall is not in the
  cell's local edges.
- **The ten whose plane partner is unpresented too.** In the
  tangent-boss family the plane exits at `present_curved` (no column's
  `graph` reads), not at `leaves_line`. So these are not the exit of
  `a-plane-corner-where-a-line-runs-on-tangent-into-an-arc-reads-no-leaving`.
  The fixtures build the boss tangent to the wall ("tangent by
  construction"). That the plane's curve meets its other edge tangent at
  `V`, the same G1 ground read in another function, is likely but
  unchecked. A fix to the cylinder side alone will not close these ten.

## Measured and landed (EMBED lane `embed/iso-end`, 2026-11-22)

**The cause, told apart from the row's.** In the cheapest reproducer
(`s58_iso_rectangle::tee_…`) the cylinder cell beside the reflex vertex
is refused at `sides_are_the_faces`, as above, but its partner, the
column's side plane, is presented (`Of(Side(YLo))`). The departure arm's
carrier separator, the cylinder's own carrier against that presentation,
reads `ψ ∈ ±1.14e-8` along the plane's side on the ruling: the centred
form's middle is ±7e-18 and its rate term ±1.17e-5, because `Psi`'s
gradient was a box enclosure of the cylinder's radial unit (z ±0.00195
over a cell 1.2e-5 tall), dotted with the side's direction, which along
a ruling is the axis. Flipping that read alone (a projector probe,
then the landed reading) makes every ask of the tee `Embedded`.

**What landed.** `Psi::rate(box, v)` replaces the gradient: a cylinder's
radial unit is dotted with `v` less its part along the axis
(`square_to`), and a cone's radial term likewise, its axial term over
the unit axis as its value reads the height; the sphere reads
`(w·v)/|w|` and the graph `m·v − (t·v)·γ′`, each naming a quantity once
where the old dot named it per component; plane and linear read `n·v`.
`rate`'s doc says which spelling each arm uses and why. No
presentation changed. The argument that no presentation of the corner
would be signed is at What a fix needs, and at `present_chart`'s
refusal.

**Measured** through the at-rest gate on `embed/iso-end-measure2`
(`origin/embed/gate-measure` @ `ebb2e6f8e` plus this diff, `ISO_OLD=1`
restoring the old reads; one build), `f64`, over the three cylinder
buckets of the census (Departure/Edge plane/cyl, cyl/plane, cyl/cyl:
236 tests; 5 `intent_s6_*` names no longer exist):

- before: 230 refuse, 1 passes; after: 183 refuse, 48 pass; none worse.
  `reach_wall_chord_rows::an_x_bar_sunk_into_either_cap_answers_its_closed_form`
  passed on an earlier harness (`c691709cb`, main `45023f4e3`) and reads
  `Budget(Depth { 40 })` on this one with either spelling of the graph
  arm's rate (the review's switch, same receipt): the difference was the
  base, not the spelling.
- **reflex shape** (two clips uncovered, the corner not convex; 66 of
  the 199 tests ≤ 30 s, classed by the trace run): 43 pass, 19 now
  refuse at another pair (a partner's exit), 4 stay where both faces are
  unpresented. The residue is on its owners' rows by name
  (`departure-reads-no-curved-corner-off-its-shared-edge`,
  `a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation`,
  `a-plane-faces-corner-that-is-not-strictly-convex-is-read-at-no-depth`,
  `the-second-order-arm-certifies-a-declared-wedge-0-edge`,
  `zero-of-reads-local-where-a-shared-vertex-is-the-corner-of-no-one-clip-pair`,
  `a-shared-vertex-the-chart-holds-on-one-clip-only-reads-no-zero`,
  `presented-partners-the-carrier-arm-still-does-not-sign-at-a-cylinder-or-plane-corner`).
- **lone clip** (16): none close. They are tangent contacts (bosses
  typed tangent, rods in bores, tangent cylinders along a ruling, whose
  rate across the side straddles 0) and partners unread at
  `present_curved` or `graph`; listed on the same rows.
- One test past its refusal now reads `Crosses` at a line contact its
  body holds unrecorded
  (`work/heldrecords/a-ringed-walls-pocketed-pair-against-the-plate-holds-its-line-contact-unrecorded.md`).
