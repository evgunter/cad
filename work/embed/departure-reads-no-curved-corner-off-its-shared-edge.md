---
id: departure-reads-no-curved-corner-off-its-shared-edge
kind: issue
title: The injectivity check reads no corner of two arcs of one conic, nor a cylinder's curved corner at a vertex the pair shares alone, so such pairs refuse; the tee, the two drums, the two-arc lune and a torus belly refuse so
status: open
opened: 2026-10-29
priority: P0
cost: M
---

## Finding

From CORNER (`departure-reads-no-curved-corner`).

`topo::injective`'s corner presentation (`present_corner`, `reading`) reads a corner where a curved edge meets another in two cases only: when the shared edge is a curve `present_curved` reads, the other edge straight in the chart; and when the shared edge is a coordinate line of a cylinder's chart, the other a harmonic chart curve. Three kinds of corner fall outside them. `beside_curve` refuses each `Structural`, so a pair whose two faces both meet one refuses at once.

- **A pair touching at the corner's vertex alone.** `corner_of` needs one of the corner's two edges on the shared cell. A face pair that shares only the vertex has neither, so no zero side holds the vertex. The iso corners answer this with `Zero::Point`; a curved corner has no such reading.
- **A corner of two curved edges.** The curve reading needs the other edge straight in the chart (`straight`). The line reading needs the read edge a coordinate line.
- **A plane face whose shared edge is a line meeting an arc.** The line reading (`present_line_corner`) is built on curved charts only. A plane has `present_plane`'s affine line presentation, which needs a span check (`self_intersection_span`) that a line ending at the corner fails.

## Evidence

The boolean tee `crossing_equal_rods` at spins `(0, 0)`, `B` from `x = 0.2`, pinned by `a_cylinder_cylinder_tee_builds_and_its_injectivity_refuses_typed_at_a_vertex_its_junction_shares_alone` in `crates/sweep/tests/self_intersection_proves_injectivity.rs` (which pins `refused == 9`). It refuses `Uncertified { Departure }` with nine refusals. The CORNER review instrumented them: all at depth 0, both faces' whole windows.

- **Six vertex-only pairs.** These are the walls against the mitre and against each other:
  - at `(0.2, ±0.98, 0.2)`, where `B`'s cap circle meets the mitre plane;
  - at `(1, 0, 1)`, where `A`'s seam line meets the junction.
- **One plane–plane pair across a `Line`.** `A`'s mitre against `B`'s cap, along the line where the mitre plane cuts the cap (in the review's run: mitre `5v9`, cap `6v5`, edge `10v3`). Its ends are corners where the line meets each face's arcs.
- **Two cylinder–mitre pairs across `Ellipse` edges.** These are each of `A`'s wall halves against the mitre, across its arc of the mitre's ellipse (`4v1`–`5v9` on `3v5`, `7v1`–`5v9` on `14v1`). The likely cause is a corner of two curved edges, which no reading reads:
  - the mitre's two ellipse arcs meet at `(−1, 0, −1)`, on `A`'s seam;
  - on the wall, the mitre's arc gives way to the junction's arc of the same ellipse at `(0.2, ±0.98, 0.2)`.

  At the root the cell also holds three or more walls in reach, which no corner reading takes either.

The junction never reaches the node plane `x = 0` (`x0 > 0` keeps the walls' tangent points out of both solids), so none of these refusals is the second-order ground.

## What a fix needs

- For the vertex alone: a point zero for a curved corner. The quadrant reading generalises if the line reading's rectangle is cut at the vertex on the moving axis too, and the curve is decided to leave into that quadrant on both axes; then `Zero::Point`'s two derivatives apply.
- For two curved edges: the other edge's height over the continued curve, decided along it, in place of `leaves_graph`'s straight-line rate. Where the two are arcs of one closed form (the ellipse split at a vertex), `continued` reads them as one curve.
- For a plane's line edge: the line reading through `present_plane`'s affine map, with `leaves_line`'s functional the line's face-side normal and the span check dropped at the vertex end.

## Further evidence (SEAMDIAG, 2026-11-01)

Lofted caps meet the second bullet, a corner of two curved edges on a
plane, at depth rather than at the root. `lily_leaf_a`'s and
`lily_leaf_b`'s caps are bounded by two circles meeting at the lens
tip. The Klein loop's caps are bounded by four arcs of one circle,
meeting where the wall seams land. A cap cell at such a vertex is
`beside_curve`'s `Structural`, and one beside it is `Local`. The pair
still splits, because the wall presents. Today it refuses at depth 40
(`work/clear/departure-reads-a-spline-sides-zero-to-second-order-so-a-cap-corner-outruns-the-depth-budget.md`
has the cells). There the binding obstacle is the wall's zero-side
residual, so this row's corner reading alone would not certify those
pairs.

## Further evidence: eight finished tour bodies (CLEAR, lane clear-gate, 2026-11-12)

Measured for `the-at-rest-gate-certifies-embedding` (its Measured
section), at main `348ec09727`, f64, the body-level question at the
default depth and 65,536 pairs. These tour bodies pass tier 3 and
refuse `Uncertified { arm: Departure }` at depth 40. Their pairs have
this row's shapes (not traced to the cell):
- **Plane × plane across a `Line`**: `chute` (faces 1 and 6, edge
  `20v1`, 0.06 s) and the lily's three buds (faces 1 and 6, edge
  `12v1`, 0.18–0.31 s). This is the third bullet's shape, a plane's
  line edge meeting an arc.
- **Plane × sphere at a vertex alone**: `diefillet` (1.5 s) and
  `heatsink5`, `7` and `9` (1.8–2.2 s, about 29,000 pairs each). This
  is the first bullet's shape.

Re-banded P0: the at-rest gate (P0) refuses these bodies at their
doors until this row lands, and a blocker inherits the band of what
waits on it.

## Further evidence: a plane cap's two semicircles (CLEAR, lane clear-planecap, 2026-11-12)

The tour's seven plane-cap bodies (`the-certificate-refuses-a-plane-cap-on-a-torus-or-cone-wall-at-their-circle`)
have this row's second bullet on the cap. Each cap is a disc on two
arcs of one circle, meeting where the walls' seams land, and a cap
cell at that vertex is `beside_curve`'s `Structural`. Their refusal
was not this row's, though. The carrier arm decides such a pair with
no presentation of the cap, and it lacked only a tight reading of the
cap's plane along the wall's side. That reading has landed, and the
seven certify with the cap still unpresented at its vertices.

A reading for a corner of two arcs of one closed conic would still
pay where no carrier arm applies, as on a curved face whose two
arcs meet on a non-planar partner. In that reading, the corner's
other edge is the read edge's own continuation, so `continued` holds
both.

## Trace: `chute` and `diefillet` (CLEAR, lane clear-corner, 2026-11-12)

Main at `eb63c45bbf`, f64, the gate harness of
`the-at-rest-gate-certifies-embedding`, with `departure` instrumented
to print both faces' presentations.

**`chute`** (a C-channel revolved 270°). The refusing pair is the end
cap `1v1` against the annulus `6v1`, the channel's floor at
`y = 0.1875`, across their line `20v1`. Every refused cell holds one of
the line's ends, `12v1` or `13v1`; the other end cap `2v1` against the
same annulus across `4v1` reads the same. At the floor (cap cell about
1.5e-3 square, annulus cell 1.5e-3 square):
- the cap is a reflex corner of two lines, where the floor meets the
  channel's wall. `present_plane`'s two-edge case reads only a convex
  corner, so it is `Local` at every depth;
- the annulus is a corner where the line meets an arc of the floor's
  rim (`13v1` at `r = 1.5625`, `12v1` at `r = 1.1875`). `reading` has
  no reading for a plane's line edge, so `beside_curve` says
  `Structural`.

The arm that decides it is the carrier arm, with the cap's plane as the
separator and the annulus presented. The cap's plane holds the whole
line and its continuation, and the annulus leaves it at a right angle.
The annulus cannot be the separator, since the reflex cap has points on
both sides of the floor's plane. **The reading it lacked** is the
annulus's corner where its shared line edge meets an arc (this row's
third bullet). The lily's buds are the same shape: `lily_bud_a`'s caps
`1v1` and `6v1` meet across the line `12v1`, each a line–arc corner at
`8v1`. Both read `Structural` at the root.

**`diefillet`** (the die blank). The refusing pair is the top plane
`1v1` against the sphere octant `9v11`, at the vertex `10v1` alone, all
at depth 40. The plane is presented, as the convex corner of its two
line edges (`Zero::Point`). The sphere is not: `10v1` is its **pole**.
The octant's two meridian edges (`22v3`, `48v3`, both `iso U`) meet
there, and the cell reaches `v = π/2`. `present_chart` clips the cell
by the one meridian it reaches. Nothing places the pole, so `zero_of`
finds the shared vertex on no zero side and no corner of two clips:
`Local`, at every depth. `heatsink5` is the same: plane `1v1` against
the sphere `34v11` at the vertex `52v3`, the sphere's pole.

The plane and the sphere are tangent at the pole, so neither carrier
separates to first order. The arm that decides it is `plane_through` at
a shared vertex alone: a plane through the vertex across the two
faces' headings. The plane face departs it along both its edges, and
the sphere departs it across the collapsed side `v = π/2`, where
`∂S/∂v` is horizontal and points back to the pole. **The reading it
lacked** is the sphere's pole as a collapsed side of its chart, the
reading a cone's apex already has. This is not this row's first
bullet (a curved corner at a vertex alone): the sphere's corner there
is two meridians, read as the collapsed side once the pole is placed.

## Read: a plane's line edge, and a sphere's pole (CLEAR, lane clear-corner, 2026-11-13)

The traced readings have landed in `topo::injective`:
- **a plane's line edge through its corner with a conic**
  (`present_plane_corner`, `Reading::PlaneLine`). It works in
  `present_plane`'s affine chart, the line at `y = 0` and the face at
  `y > 0`. The other edge's chart curve must leave the vertex into
  `y > 0` and stay there within reach (`leaves_line`), and nothing else
  may be in reach. The far end needs no span check, because it is out
  of reach. A reflex corner is `Structural`. The sliver concession
  applies as in `present_line_corner`. Without it, `chute`'s annulus
  cells, which touch the line from its far side within rounding, stay
  undecided at depth 40;
- **a sphere's pole as a collapsed side of its chart**
  (`present_chart`), as a cone's apex already was. The vertex is placed
  at the pole by `geom_brep::chart_singularity::vertex_at_singular_point`,
  the one reading of that incidence, so this adds no site of its own.
  PR 6892 (`clear-pairs`, below) landed a pole reading of its own in the
  same place, deciding the distance at `self_intersection_apex`. On the
  merge, this one is the only pole reading: it decides the incidence
  where tier 3's check 6 places the singularity, and it clips the
  rectangle to the pole's latitude.

Measured with the gate harness at `eb63c45bbf` plus this change (f64,
release, one core, 65,536 pairs, depth 40). Every one of the eight tour
bodies holds:

| body | before | after | pairs | deepest |
|---|---|---:|---:|---:|
| `chute` | refused, 0.06–0.10 s | 0.07 s | 2,073 | 19 |
| `lily_bud_a` | refused | 0.10 s | 826 | 20 |
| `lily_bud_b` | refused | 0.08 s | 711 | 21 |
| `lily_bud_c` | refused | 0.08 s | 857 | 19 |
| `diefillet` | refused, 1.5–2.2 s | 0.44 s | 4,133 | 16 |
| `heatsink5` | refused, 1.8–3.3 s | 1.25 s | 16,093 | 22 |
| `heatsink7` | refused | 1.25 s | 16,295 | 22 |
| `heatsink9` | refused | 1.25 s | 16,839 | 22 |

Over the whole walk, 74 of 85 hold, up from 66. The other 11 are the
eight out-of-pairs bodies and the three assemblies, as before, and no
body that held has moved. Re-measured on the merge with PR 6892 (main
`adc5cd2a3e`), 80 hold, up from 6892's 76. The four new ones are
`chute` and the three buds, with the same pairs as above. The other
five are `diecomposed` and `klein_bulb`, which run out of pairs, and
the three assemblies.

**What remains** is this row's first two bullets:
- **The tee** (`crossing_equal_rods`) now refuses 8 root pairs, not 9.
  The mitre's line on `B`'s cap is read through its two line–arc
  corners. Six pairs at a vertex alone and two across the mitre's
  ellipse remain.
- **Five of the f64 cavity row's eight departure rows**, asked at f64
  through the verdict door (a local, uncommitted `CavityGate` switch):
  - `shell9_r1_probes::r1_drum_reverted_cavity_alone` and
    `shell7_seam_corner::a_collinear_cap_vertex_drum_shells_to_its_closed_form`:
    the wall against the cap at `5v1` (and `2v1`) alone. The cap's
    corner there is a line meeting its rim circle, neither edge
    shared, and reads `Structural` (the first bullet);
  - the three `torax_axial` sphere-lune rows: the cap and the lune
    across a rim arc. The plane's arc meets the next arc of its circle,
    or the line closes a lens. The sphere's arcs meet other circles.
    Both faces read `Structural` at the root (the second bullet).

  The other three (`shell_curved_mouth`'s sphere window,
  `spiric_rim`'s sectioned vessel and `verbs_shell`'s dome sectors)
  shell at f64 on this branch with the gate asking. The row counted
  them refused on PR 6885's branch, which predates PR 6886's plane-cap
  reading, so this run does not say which change moved them.
## Further evidence: the plane × sphere bodies certify (CLEAR, lane clear-pairs, 2026-11-12)

`diefillet` and `heatsink5`, `7` and `9`, the "plane × sphere at a
vertex alone" bodies above, hold once `present_chart` reads a
sphere's pole as a collapsed side of its chart
(`the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies`,
its Fix): 4,133 and about 16,100–16,800 pairs, against 20,885 and
about 29,000 refused before. The vertex those pairs share was the
sphere patch's pole, which the presentation read as a loose vertex.
So this row's first bullet no longer has tour evidence; `chute` and
the lily's three buds (the third bullet) still refuse.

## Trace: the drums, the lunes, the tee and a torus belly (CLEAR, lane clear-corner2, 2026-11-13)

Main at `2b68e8551c`, f64. The shell rows are asked through the verdict
door with a local, uncommitted `CavityGate` switch, and `departure` is
instrumented to print both faces' presentations. The f64 cavity row has
the measurement.

**The drums** (`shell9_r1_probes::r1_drum_reverted_cavity_alone`;
`shell7_seam_corner::a_collinear_cap_vertex_drum_shells_to_its_closed_form`
refuses the same way). The cavity's cap is one plane in three faces:
two half-annuli, `2v1` and `6v3`, and the inner disc `1v3`. The disc is
bounded by the ring's two half-circles, `6v3` and `8v3`, which meet at
`4v3` and `1v3`. The wall is two half-cylinders, `3v1` and `5v1`. Two
shapes refuse:
- **At a vertex alone.** Half-annulus `2v1` meets wall half `3v1` at
  the rim vertices `2v1` and `5v1` alone, and `6v3` meets `5v1` the same
  way.
  - The wall is presented as a corner of two coordinate lines
    (`Zero::Point`).
  - The half-annulus's corner there is its radial line `3v3` and its
    rim arc `9v1`, at 90°, neither shared. That is `beside_curve`'s
    `Structural` at every depth, so the pair split to depth 40.
  - The arm that decides it is `plane_through` at a vertex alone. A
    plane through the vertex, tilted off the cap, has the half-annulus's
    wedge on one side and the wall's on the other.
  - **The reading it lacked** is the plane's line–arc corner as a
    quadrant about the vertex (the first bullet).
- **Two arcs of one circle.** The disc `1v3` meets the half-annulus
  `2v1` across `6v3`, and `6v3` across `8v3`, near `4v3` and `1v3`.
  - The half-annulus is presented through its line–arc corner.
  - The disc's corner is the ring's two half-circles, `Structural` at
    every depth, so these pairs split to depth 40 and refuse there. This
    is the refusal the row reports:
    `Uncertified { 1v3, 2v1, Edge(6v3), Departure }`.
  - All three faces lie in one plane, so no carrier separates them, and
    the conic's own plane is that plane. The graph separator is built
    on the shared edge's piece, so it does not vanish on the disc's
    other half-circle.

**The lunes** (`torax_axial`'s three sphere-lune rows).
- **The one-arc lune's cavity** is three faces: two caps and the
  sphere.
  - Each cap is a lens of the axis line `1v1` and one arc (`2v1`,
    `3v1`). The sphere is a lens of the two arcs.
  - The arcs are circles off the sphere's axis. The sphere's chart
    images of them are neither coordinate lines nor harmonic, so the
    sphere is never presented beside them.
  - At the root all three pairs read both faces `Structural`, because
    `corner_of` refused a lens (two edges meeting at both ends), so each
    pair refused at once.
  - The arm that decides cap against sphere is the carrier arm, with the
    sphere's carrier and the cap presented. Cap against cap is the
    carrier arm across the line.
  - **The reading the caps lacked** is a lens read at the one end the
    cell holds. The corner there is already read: `Reading::Curve`
    across the arc, `Reading::PlaneLine` across the line.
- **The two-arc lune** re-authors the equator.
  - Each cap's arc is two arcs of one circle meeting at the equator
    vertex: on cap `1v1`, `3v1` and `5v1` at `4v1`. The sphere is two
    faces.
  - Cap `1v1` against sphere `3v1` across `3v1` has, near `4v1`, the
    cap's corner of two arcs of one circle.
  - Only the carrier arm can decide it, since the sphere is not
    presented. Read through `4v1` by the continued circle, the cap's
    zero runs on along `5v1`, which `3v1` does not bound. The carrier
    arm's separating face lies wholly on its zero set, so nothing rules
    out a point of `3v1` on `5v1`.

**The tee** (`crossing_equal_rods`). Its eight root refusals, traced to
the vertex:
- The junction's ellipse and the mitre's are one ellipse in the mitre
  plane. Its arcs (`13v3` and `9v3` on `B`'s wall, `3v5` and `14v1` on
  the mitre) run on through `1v3` and `9v1` at `(0.2, ±0.98, 0.2)`,
  from one face to the face across the vertex.
- So at `1v3`, `B`'s wall `2v3` has the ellipse's tangent `T` on its
  wedge and the mitre `5v9` has `−T`. A plane through the vertex
  separating them contains `T`, and then no quadrant reads it strictly,
  since `Zero::Point` needs both derivatives strict.
- `A`'s wall `4v1` against `B`'s cap `6v5` at `1v3` is the same. `A`'s
  wall's corner there is the ellipse's two arcs, a straight angle, which
  no quadrant holds.
- The walls against each other at `2v3`, `(1, 0, 1)`, are the junction
  meeting `A`'s seam.
- These vertex-alone pairs are second order at the vertex. No
  first-order arm decides them, whatever corner is read.
- The two pairs across the mitre's ellipse (`4v1` and `7v1` against
  `5v9`) are two arcs of one ellipse: at `7v1`, `(−1, 0, −1)`, on the
  mitre, and at `1v3` and `9v1` on `A`'s wall.

**A torus belly re-posed**
(`torax_axial::torax_the_torus_corners_survive_a_rigid_re_pose`). The
f64 row counted it out of pairs; it now refuses on the departure arm.
- The cylinder `4v1` meets the torus `6v1` at `7v1` and `3v1` alone.
- The torus is presented as a corner of two coordinate lines.
- The cylinder's corner is the coordinate line `3v1` and the harmonic
  circle `6v1`, `Structural`.
- This is the first bullet on a cylinder's chart, which is periodic.
  The reading below is built on a plane's.

## Read: a lens, and a plane's corner at a vertex alone (CLEAR, lane clear-corner2, 2026-11-13)

- **A lens is a corner at the one end the cell holds** (`corner_of`).
  Its other end lies outside `R`:
  - the curve reading's run passes both of `R`'s column ends;
  - the line readings admit no vertex but the corner's into `R`'s
    reach.

  A cell holding both ends is `Local`, not `Structural`, since a smaller
  one holds one end.
- **A plane's corner of a line and a conic at a vertex the pair shares
  alone is a quadrant** (`vertex_corner`, `present_plane_vertex`). It is
  presented with the corner as its zero, `Zero::Point`, as
  `present_plane` reads a corner of two lines.
  - The affine chart's first axis is the line's direction away from the
    vertex. The second leans back off the conic's tangent by a quarter
    of the first, so the tangent is strictly inside the quadrant.
  - The second axis must be decided on the line's face side
    (`self_intersection_corner`).
  - `only` admits nothing in `R`'s reach but the two edges and the
    vertex.
  - `leaves_line` decides the conic's chart curve rising from the
    vertex on both axes while it is in `R`.
  - `R` off the quadrant is then connected, holds no boundary, and meets
    the outside of the face at the vertex, so it lies outside the face.
  - A reflex corner is `Structural`.
  - The reading adopts no sliver and has no vacuous path.

**Measured.**
- Both one-arc lune rows shell at f64 with the gate asking.
- The drums' vertex-alone pairs certify at cells 0.24 wide, where they
  ran to the floor before. The drums still refuse, on the disc.
- The tee's four plane–wall vertex-alone pairs now present the plane
  and not the wall, so they split to the floor. The tee refuses 12: four
  pairs at the root and eight cells at the floor.

**What remains:**
- **Two arcs of one conic at a corner**: the drums' disc, the two-arc
  lune's caps, and the tee's ellipse pairs. Read by the continued
  conic, the face's zero runs on past the shared cell along the next
  arc. That needs two things this row does not settle:
  - a rule for when a zero side past the shared cell is read. `sign_on`
    reads `Zero::Past` only against a presentation whose zero is on the
    shared cell, and never in the carrier arm. A `Zero::Curve` past the
    shared cell needs the same rule, in `sign_on`, which lane B owns.
  - a separator for the drum's three coplanar faces that vanishes on the
    whole circle: the graph separator over the conic continued, or the
    circle's cylinder.

  The two-arc lune has only the carrier arm, where that rule does not
  hold.
- **A cylinder's corner at a vertex alone**: the torus belly re-posed.
- **The tee's vertex-alone pairs**, which are second order at the
  vertex.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.
