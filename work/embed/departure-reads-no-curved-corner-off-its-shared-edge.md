---
id: departure-reads-no-curved-corner-off-its-shared-edge
kind: issue
title: The injectivity check still reads no two-arc lune (a sphere face beside an off-axis circle arc), no corner of two different conics and no spiric wall continued onto its next arc; the tee, the two drums and the torus belly now hold
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
- These vertex-alone pairs were called second order at the vertex,
  decided by no first-order arm. Wrong on both counts: the walls'
  halves and `B`'s wall against the mitre are first order to a plane
  (lane `embed/cylinder-vertex`), and `A`'s wall against `B`'s cap,
  second order to every plane, is first order to the graph separator
  over the ellipse (lane `embed/conic-vertex`). All certify.
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
- **A cylinder's corner at a vertex alone**: the torus belly re-posed
  (read since by lane `embed/cylinder-vertex`).
- **The tee's vertex-alone pairs** (read since by lanes
  `embed/cylinder-vertex` and `embed/conic-vertex`; the tee holds).

## Read: a cylinder's corner at a vertex alone (EMBED, lane embed/cylinder-vertex, 2026-11-17)

**Measured first.** On main `362cd72fd`, f64, the torus belly re-posed
was asked through the local `CavityGate` switch, with `departure`
instrumented.
- The cylinder `4v1` is a band, `v` from −0.0078 to −0.0038. Its edge
  `6v1` is a parallel, `v = −0.00379`, but the re-pose stores it as a
  harmonic image (`pb_v = −3.5e−18`) and not as a coordinate line.
- So its corner with the ruling `3v1` at `7v1` and `3v1` is a right
  angle of the chart. A quadrant on the chart's own axes cannot hold
  it strictly: the parallel's rate along the ruling is 3.5e−18.
- The plane's leaned quadrant reads it. Its lean has to be chosen in
  the surface's metric, not in chart units. In chart units, with the
  cylinder's `u`-speed 0.055 against `v`'s 1, the quadrant opened to
  about 167° on the surface and no plane through the vertex separated
  the pair. That was measured as a first attempt.

**What landed.** `vertex_corner` and `present_vertex_corner` (the
plane's `present_plane_vertex`, generalised) read the quadrant on any
chart that is regular everywhere and whose curved images are harmonic:
a plane's line and conic, and a cylinder's coordinate line and
harmonic curve.
- `V` is the straight edge's chart point, moved by whole turns nearest
  the cell.
- No copy of either edge by whole turns but the one through `V` may
  meet `R`'s chart footprint. For the curve this is load-bearing: a
  helix's next turn can pass through `R` with neither end in reach.
  For the straight edge it is defence.
- The face side is `face_side`'s on a chart, and the traversal turned a
  quarter on a plane.
- The lean is chosen in the metric at `V`.
- `leaves_line` then reads the curve's copy through `V` in the
  quadrant's coordinates, with no periodic axis.
- `turns_to`, `copies` and `leaves_line` now take the periodic axes
  rather than the surface.

The torus, the cone and the sphere are not covered:
`a-torus-or-cone-corner-at-a-vertex-alone-reads-no-quadrant`.

**Measured after** (merge base `362cd72fd`; the same runs on main):
- **The torus belly re-posed** shells at f64 with the gate asking.
- **The f64 cavity rows**, the switch on. On main 15 valid rows
  refuse, and here 14. The torus belly re-posed is the only verdict
  that moves. The two crossing pins refuse `CavityCrosses` on both.
- **The tee** (`crossing_equal_rods`) refuses 8 cells, not 12: two root
  pairs and six floor cells. The receipt has 607 splits, against 545.
  - Four pairs that refused now certify. Two are the walls' diagonal
    halves at `2v3`, `(1, 0, 1)`: `2v3 × 7v1` and `4v1 × 10v1`, which
    refused at the root. Two are `B`'s wall against the mitre at `1v3`
    and `9v1`: `2v3 × 5v9` and `5v9 × 10v1`, which refused at the
    floor.
  - Each is first order, not second, as the trace above had it. At
    `2v3`, `B`'s half `2v3` is the quadrant `(+x, +y)` of the plane
    `z = 1`, and `A`'s half `7v1` is `(+z, −y)` of `x = 1`. A plane
    through the vertex with `n ≈ (0.46, 0.89, −0.02)` has them strictly
    apart.
  - At `1v3`, `B`'s wall's wedge holds the ellipse's tangent `T` and
    the mitre's holds `−T`. Separating them needs `n·T > 0` on both
    sides, which `n ≈ (0.92, 0.38, −0.02)` gives. It does not need a
    plane containing `T`.
  - Both were checked by hand against the union's first-order regions
    at the vertex.
  - What still refuses is two arcs of one ellipse at a corner. `A`'s
    wall halves against the mitre across its ellipse refuse at the
    root. `A`'s wall halves against `B`'s cap at `1v3` and `9v1`, where
    `A`'s corner is the ellipse's two arcs, refuse three floor cells
    each. That is `embed/conic-arcs`' ground.
  - The verdict now names `A`'s wall `4v1` against the mitre `5v9`
    across `3v5`. The pin moves to `refused == 8`.
- **The injectivity suite** (`self_intersection_proves_injectivity`)
  is green at the default ε, 1e−6 and 1e−12. Only the tee moved.
- **The tour walk** holds 82 of 85, on main and here alike. No verdict
  and no body's pair count moves. The three that do not hold are the
  bench assemblies' recorded contacts. (Main already held 82, where the
  brief counted 81.)

**What remains:**
- two arcs of one conic at a corner (`embed/conic-arcs`), which was then
  every tee refusal (read since; the tee holds);
- the drums' disc;
- the two-arc lune.

### Review fix pass (2026-11-17)

**Which commit each measurement was taken at:**
- The first pass above was measured on merge base `362cd72fd`, against
  main at the same commit.
- The fix pass head is `f3087b7070`, which merges main through PR
  7071. At that head:
  - the f64 cavity rows (switch on) refuse the same 16 rows as the
    first pass, the belly shelling;
  - the tour walk holds 82 of 85, with every verdict and pair count
    identical to the first pass;
  - `self_intersection_proves_injectivity` is green. The tee reads 8.

**The tee with PR 7089 landed** (merged here with main `7cb71c2981`):
it refuses 6 floor cells. Two pairs refuse: `A`'s wall halves against
`B`'s cap at `(0.2, ±0.98, 0.2)`, a straight angle of two ellipse arcs
at a vertex alone. The verdict names `4v1 × 6v5` at `1v3`. Every other
touching pair holds asked alone. That is the prediction both PRs made.
The pin is re-set to it.

**Rows filed:**
- `the-vertex-alone-plane-leans-along-a-cells-long-metric-side-so-an-anisotropic-pair-splits`
  (the review's N1, traced to `planes_through`'s midpoint headings);
- `the-corner-presentations-each-spell-their-reach-preamble`.
## Read: two arcs of one conic at a corner (EMBED, lane conic-arcs, 2026-11-17)

**What landed** (`crates/topo/src/injective.rs`):
- **`Reading::Conic`.** A corner whose shared edge is a curve on a
  conic and whose other edge continues it through the vertex is read on
  through the vertex along the other arc.
  - "Continues it" is `continues`: the two are `one_conic` (one plane
    through one centre and one radius, or for an ellipse the same
    semi-axes along the same lines, each compared as one value, sharing
    `one_value` with `one_plane`); the other edge has one use; the two
    are no more than a turn together; and the other leaves the vertex
    the way the shared edge arrives. So the two meet only at their
    common ends. Arcs split from one rim are not stored on one frame:
    the drum's half-circles have opposed axes, and the tee's ellipse
    arcs have opposed `u_ref`.
  - The curve is read by the shared edge's own closed form over a whole
    turn and a little more, centred on the vertex (`through_of`). Step
    2's `apart` then leaves one run of the conic in `R`. `alone` admits
    only the two arcs and the shared edge's ends, so the run in `R` is
    those two arcs. The argument is at `present_corner`.
  - Its zero is a `Zero::Curve` marked `past` (`Curved::past`), since the
    face has points on it off the shared cell.
- **When a past curve is read** (`sign_on`, `past_read`). This is the
  first of the two things the previous section named, the rule for a
  zero past the shared cell:
  - The rule is the one `Zero::Past` already had: only against a
    partner presentation whose zero set is on the shared cell, on an
    analytic face, and never in the carrier arm.
  - It holds under either separator the engine builds. A plane must hold
    the whole conic (`On::whole`). The graph separator must hold the
    curve's parameters in its piece (`Curved::within`). Those parameters
    are the shared edge's own carrier's, continued, so `graph_of`
    already extends the piece to them.
  - The argument (`past_read`): where both are zero, a point is on the
    partner's zero set, so on the shared cell, and on this face's zero
    in `R`, so on the shared edge or the next arc. Nothing of the shared
    cell but the shared edge and its ends is in `R`'s reach, and the
    next arc meets the shared edge only at their common ends
    (`continues`). So the point is on the shared cell. This rests on
    `continues`, checked in the reading, and not on the face's
    injectivity: past curves add no exposure to
    `an-analytic-face-may-cover-its-carrier-more-than-once`, which past
    sides (`Zero::Past`) still lean on.
- **A separator for the coplanar faces.** This is the second thing the
  previous section named, and it needed no new separator. The graph
  separator over the conic continued vanishes on the whole run read. On
  the drum's three coplanar faces its bisector lies in their plane, so
  it departs across the circle where the conic's own plane cannot.
- **A `Conic` cell that `graph_read` does not read is `Local`**, where
  main typed every such corner `Structural`. On purpose: a smaller cell
  is often read where a large one is not (step 2's `apart` and `alone`
  refuse big cells), and the drum's cavity certifies only through those
  smaller cells. A pair whose corner is never read now splits to
  `Budget(Depth)` where main refused it `Uncertified` at once. No
  measured row moved that way: the tee, the lune and the drums refuse
  or hold as above, and the tour walk's pair counts did not move.
- **`stuck`.** A pair refuses at once when each face is `Structural` or
  read past at a vertex in its reach. This is the case the both-
  `Structural` rule already refused: such a cell was `Structural` before
  this reading. Without it, the two-arc lune's cap–sphere pairs split
  to the floor, 0.27 s to 2.1 s, and still refuse.

**Measured** (f64; the shell rows through the local, uncommitted
`CavityGate` switch; before is the merge base `362cd72fd`):

| row | before | after |
|---|---|---|
| `shell9_r1_probes::r1_drum_reverted_cavity_alone` | `Uncertified { 1v3, 2v1, Edge(6v3), Departure }` | shells |
| `shell7_seam_corner::a_collinear_cap_vertex_drum_shells_to_its_closed_form` | refuses, the same | shells |
| `torax_a_two_arc_lune_re_authors_its_equator_seam_and_hollows` | `Uncertified { 1v1, 3v1, Edge(3v1) }`, 0.27 s | `Uncertified { 3v1, 4v1, Edge(6v1) }`, 0.41 s |
| `torax_the_torus_corners_survive_a_rigid_re_pose` | refuses | refuses, unchanged (not this reading) |
| the tee (`crossing_equal_rods`), refused cells | 12 | 10, the six refusing pairs each at a vertex alone (pinned pair by pair) |
| `self_intersection_proves_injectivity` | 46 pass | 47 pass (one new) |
| the tour walk | 82 of 85 hold | 82 of 85, no body's pairs moved |

- **The tee.** Both pairs across the mitre's ellipse now certify:
  - `A`'s wall `4v1` against the mitre `5v9` across `3v5`;
  - the mitre against `7v1` across `14v1`;
  - each by the graph separator, against the corner read through the
    line–arc corner.

  The junction's arcs between the walls (`2v3`–`4v1` across `13v3`,
  `7v1`–`10v1` across `9v3`) also certify cells at the junction's
  vertices that a wall reads past, under the ellipse's own plane. What
  still refuses:
  - the walls against each other at `2v3`, `(1, 0, 1)`, two root pairs;
  - the four plane–wall vertex-alone pairs, eight cells at the floor.

  PR 7093 (`embed/cylinder-vertex`) certifies the walls at `(1, 0, 1)`
  and `B`'s wall against the mitre, and leaves `A`'s wall halves against
  the mitre (this reading's pairs) and against `B`'s cap at the floor.
  With both, the tee should refuse only `A`'s wall halves against `B`'s
  cap, at a vertex alone where `A`'s wall's corner is two arcs of the
  ellipse, a straight angle no quadrant holds. Whichever lands second
  re-pins against main and measures it.
- **The two-arc lune.** Its caps are now presented at the equator
  vertex, and its spheres are still not presented:
  - The cap–sphere pairs have only the carrier arm, which a past curve
    is not read in, so they refuse (`stuck`).
  - The sphere halves against each other across the equator arc `6v1`
    refuse at the root, both `Structural`. That pair refused before as
    well, and the cap pair was reported first.
- **The tour walk.** The merge base walks 82 of 85, not the 81 the
  program counted at the cut. The three that do not hold are the
  assemblies `bench`, `bench60` and `bench90`, at their recorded
  contacts.

**What remains of this row:**
- **A cylinder's corner at a vertex alone**: the torus belly re-posed
  (lane `embed/cylinder-vertex`).
- **The tee's vertex-alone pairs** (read since: the tee holds, by the
  conic corner at a vertex alone below).
- **Two arcs of one conic where no partner is presented**, so only the
  carrier arm applies. This is the two-arc lune: its sphere halves read
  no image of an off-axis circle, so neither the cap pairs nor the
  sphere pair can certify. It needs a presentation of the sphere beside
  such an arc, or a carrier-arm reading of a past curve that rules out
  the separating face holding the next arc. That reading would rest on
  another pair's certificate, which is a design question and not
  settled here.
- **A corner of two curved edges of different conics** (the lily
  leaves' lens tips). `Reading::Conic`
  reads only a straight angle on one conic. A corner of two different
  curves still needs the other edge's height over the continued curve.
  No evidence row refuses on it today: SEAMDIAG's caps refuse on the
  wall's zero side first.
- **A spiric wall image continued onto the next arc of its spiric.**
  `Reading::Wall` takes a straight parallel only. There is no instance.

Filed beside it:
`a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth`.
The drum's own body, not its cavity, runs out of depth at f64 on a
half-annulus cell a rounding off its half-circle, on main as on this
branch.

## Read: two arcs of one conic at a vertex alone (EMBED, lane conic-vertex, 2026-11-18)

**Measured first** (main `763d6fe0`, f64, `departure` instrumented,
uncommitted). The tee refuses `4v1 × 6v5` at `1v3` with six cells at
the floor, the other half `6v5 × 7v1` at `9v1` the same. In every one of
those pair cells `A`'s wall reads `Err(Structural)`: its corner at the
vertex is the ellipse's two arcs, neither shared, and no reading reads
it. `B`'s cap reads its quadrant (`present_vertex_corner`, a line on the
mitre plane and the cap's rim), `Zero::Point`.

**What the pair is.** `A`'s wall is the cylinder `x² + y² = 1` on the
side `z ≥ x` of the ellipse, both arcs through `V = (0.2, 0.98, 0.2)`;
the cap is the plane `x = 0.2` on `z ≤ 0.2`, `y² + z² ≤ 1`. They meet
at `V` alone. The wall's first-order cone at `V` is a half-plane holding
the ellipse's tangent `±T`, so a plane through `V` parting the two must
contain `T`; to second order the ellipse then leaves it to the cap's
side unless the plane is the mitre plane, which holds the cap's line
edge. So **no plane through `V` parts the pair**: a sampled search over
plane normals gave a best margin of −1.4e−3, at the mitre plane. A
surface holding the ellipse does: `ψ = (z − x) + β(x² + y² − 1)` is
zero on the wall exactly on the ellipse and negative on the cap off `V`
for `0 < β < 2.5`. The trace above was right that this pair is second
order to a plane, and wrong that no first-order arm decides it.

**What landed** (`crates/topo/src/injective.rs`): `Reading::Conic` at
a vertex the pair shares alone, and the graph separator built there.
- `corner_of` returns an `alone` corner where neither wall is on the
  shared cell and their common end is the shared cell alone;
  `reading` gives it `Reading::Conic` only (`continues`, and the curve
  one `present_curved` reads), so a straight angle of two conics that
  are not one stays `Structural`.
- `present_curved` presents it by the run of the conic through the
  vertex, as through a shared edge's end, its zero the run `past` the
  vertex. The region argument at `present_corner` does not use the
  shared edge; only its zero's reading does.
- `past_read`'s argument gains the vertex-alone case: a common zero is
  on the partner's zero set, so at `V`, and the run read, less than a
  turn and a little more centred on `V`, meets `V` once.
- `graph_of` at a vertex alone takes the edge a past curve lies on, and
  lists it and `V`; `V`'s parameter is among the curve's, which the
  piece holds.

**Measured after** (the same commit with this change):

| measure | main | here |
|---|---|---|
| the tee (`crossing_equal_rods`) | `Uncertified { 4v1, 6v5, Vertex(1v3) }`, 6 refused, 601 splits, 622 discharged | holds: 0 refused, 465 splits, 492 discharged; every touching pair holds asked alone |
| `self_intersection_proves_injectivity`, default ε / 1e−6 / 1e−12 | 48 pass each | 48 pass each |
| the f64 cavity rows (local switch) | 14 fail: the 2 crossing pins and 12 valid rows | the same 14; the two-arc lune still `Uncertified { 3v1, 4v1, Edge(6v1) }` |
| the tour walk, 85 stops | 82 hold | 82 hold; no body's candidates, discharged, refused or splits moved |

On the tee both `A`-wall halves against `B`'s cap certify under the
graph separator at the vertex, the cap as its quadrant.

**Filed:** `a-vertex-alone-pair-both-read-on-through-the-vertex-refuses`
(what the reading leaves out), and BAND's
`blend-reach-crossing-is-a-sole-bracket-door-missing-from-the-bounds-roster`,
a main red found running the censuses. (The endpoint census's drift on
`injective.rs` was main's, and PR 7104 re-blessed it.)

**What remains of this row:** the two-arc lune (no presentation of the
sphere beside an off-axis circle), a corner of two different conics,
and a spiric continued onto its next arc, as the previous section says.

## Trace: the two-arc lune (EMBED, lane cavity-trace, 2026-11-18)

**Setup.** Main `da49c63f`, f64:
`torax_axial::torax_a_two_arc_lune_re_authors_its_equator_seam_and_hollows`,
its cavity asked through the local `CavityGate` switch, `departure`
instrumented (uncommitted).

**The verdict** is `Uncertified { 3v1, 4v1, Edge(6v1), Departure }`,
with receipt 120 discharged, 21 refused and 131 splits.

**The cavity.**
- Two plane caps: `1v1` on `x = 0.05` and `2v1` on `z = −0.05`.
- Two sphere faces, `3v1` and `4v1`: radius 0.25, axis `y`, split at
  the equator arc `6v1`.
- Each cap's arc on a sphere face is a circle in the cap's plane, so its
  axis is `x` or `z`, never the sphere's `y`.

**What each face presents:**
- **The sphere faces read `Err(Structural)` in every cell**, against
  every partner.
  - Beside a cap arc, `reads_curve` fails: the sphere's chart has no
    harmonic image of a circle off its axis. That arc is the shared edge
    against a cap, so it is `Structural` (`unread`).
  - Beside the equator arc `6v1`, every cell also reaches a cap arc's
    corner at the equator vertex, and is `Structural` there.
- **The caps are presented.** Each reads its two arcs of one circle
  through the equator vertex (`Reading::Conic`, `Curve(XHi, past)`), or
  its lens's line (`Side`). `Err(Local)` where a cell holds both ends.

**What refuses:**
- **Each cap against each sphere face** refuses five cells across the
  cap's arc (`1v1 × 3v1` across `3v1`, `1v1 × 4v1` across `5v1`,
  `2v1 × 3v1` across `2v1`, `2v1 × 4v1` across `4v1`).
  - The cap is read past the equator vertex and the sphere is
    `Structural`, so the pair is `stuck`.
  - Only the carrier arm could decide it, the sphere's carrier against
    the cap, and a past curve is not read there.
- **The sphere faces against each other** across `6v1` refuse at the
  root, both `Structural`.

**(a) What a presentation of the sphere beside an off-axis circle would
have to read.** The face's part of a cell lies on one side of the
circle: on the sphere, one side of the circle's plane. That region is a
coordinate half of a sphere chart whose axis is the circle's plane
normal, in which the circle is a parallel. Presenting the face in that
re-posed chart (the sphere is symmetric about its centre, so any axis
through it charts it) turns the arc into a coordinate side. It lies on
its edge, so its zero is on the shared cell. Two consequences:
- **The cap–sphere pairs should then close with readings that exist**
  (predicted from the arms' rules, not measured). The cap's
  past curve against the sphere's exact side is admitted by
  `past_read`, under the graph separator over the arc or under the cap's
  plane (both vanish on the whole circle). So nothing short of the
  design question is needed for these four pairs, and the carrier arm
  reading a past curve, resting on another pair's certificate, is not
  needed either.
- **The sphere pair across `6v1` needs one more reading:** each sphere
  face's corner at the equator vertex is the equator (a parallel) meeting
  a cap circle (a parallel only of the re-posed chart). That is a corner
  of two different conics, which this row also names, unread in any
  chart.

**The cheapest evidence** is this row's lune at 0.34 s.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.

## Census: 49 cylinder-bucket tests at the at-rest gate (EMBED, lane embed/trace-cylinder, 2026-11-21)

Traced from the per-solid census (`measure/gate/census-c2a0c41f40.json`
on `embed/gate-measure`), on `embed/trace-cylinder-measure` with
`CV_TRACE=1` and the Structural exits printed. **49 valid bodies refuse
at their door here**: 19 of the cylinder/cylinder edge bucket, 16
cylinder/plane, 8 plane/cylinder, 4 cylinder/cylinder at a vertex, 2
cylinder/plane at a vertex.

- **The exit.** `beside_curve`'s `unread_corner` (`Unpresented::Structural`):
  the cell holds a vertex of the shared cell where a curved edge meets
  another and neither `corner_of`/`reading` nor `vertex_corner` reads
  it. In 39 of the 49 both faces exit there and `departure` refuses at
  once (`stuck`); in the other 10 one face does and the pair runs to
  `Budget(Depth)`.
- **The families.** `bisector_nodes` 9 (equal cylinders whose axes meet:
  the intersection's ellipse arcs meet at a node vertex), the
  hand-built bicylinder and its subtraction piece (`topo boolean::node_probe_rows`,
  2), `crossing_equal_rods` 3 (the tee of this row's Evidence),
  `a_vertex_on_a_grazing_plane` 3, `a_ruling_lying_on_a_wall` 2 (a
  ruling across an ellipse), `ring_carry_through_by_piece` 2,
  `sheets_part_on_slanted_rims` 2, `bisector_lane_stubs` 2,
  `verbs_germarms2` 2, `parallel_cylinder_join` 2, `pis_arc_capped_poses`
  2, and 18 singletons.
- **Cheapest reproducer:** `topo boolean::node_probe_rows::the_bicylinder_built_by_hand_is_a_finished_body`
  (0.08 s): cylinder `1v1` against cylinder `2v1` across three shared
  edges, both `Structural` at `beside_curve`, refused `stuck` at the
  root cell.
- **Not separated:** which of this row's shapes each corner is (two
  different conics, a line and a conic the line corner does not read, a
  spiric continued). The trace does not print the corner's two edges;
  the bicylinder and `bisector_nodes` corners are of two ellipse arcs
  by construction, the rest unchecked.

## Census tests whose cylinder corner certifies now and whose partner stops here (EMBED lane `embed/iso-end`, 2026-11-22)

With the cylinder carrier's rate read square to its axis, the cylinder bucket's reflex and lone-clip pairs certify where their partner is presented. In these 12 the partner, a plane or cylinder face holding a line and an arc at the vertex, is refused at `beside_curve`'s unread corner (`Structural`) or at `curved_walls`' unread wall (`Local`), so the test still refuses at that pair or at a later one. Whether the corner is tangent was not checked per test.

Measured on `embed/iso-end-measure2` (`origin/embed/gate-measure` @ `ebb2e6f8e` plus `embed/iso-end`'s diff), where each refusal is the same pair, test by test, as on the trace run that named the exits (`embed/iso-end-measure` @ `030c75763`, `CV_TRACE`).

- `editor-core::all emit_union_seam_rank::crossings_of_one_open_arc_name_apart_by_sense_in_every_member_order`
- `sweep::all parallel_cylinder_join::equal_wider_and_nested_rods_join_along_their_rulings`
- `sweep::all parallel_cylinder_join::one_face_walls_read_their_axes_as_an_angle`
- `sweep::all ring_carry_through_by_piece::a_ring_refuses_at_its_least_margin_in_either_subtraction_order`
- `editor-core::all intent_s6_d_tangency::a_tangent_arc_leaving_the_trace_proves`
- `editor-core::all m10_di_dual_corpus::at_rest_census_verdicts_match_f64_at_interval`
- `editor-core::all m4_pr8_corpus_interval::every_document_evaluates_green_at_interval`
- `editor-core::all review_m5_pr9_doc_probe::the_boss_union_document_row_interval_bit_replay`
- `sweep::all parallel_cylinder_join::an_island_and_a_turned_pose_join_along_their_rulings`
- `sweep::all review_fillet_e2_probes::a_ring_of_arcs_of_two_circles_is_metered_arc_by_arc`
- `sweep::all ring_carry_through_by_piece::a_lens_ring_beside_a_hole_rim_builds_in_either_order`
- `sweep::all ring_carry_through_by_piece::a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order`
