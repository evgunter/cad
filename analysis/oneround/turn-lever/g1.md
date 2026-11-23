# G1: the dihedral lever read from the edge's line

## What was switched, and what each count is judged against

The measurement build (`measure.patch`, `TL_SITE=line`) changes only the wedge
reach's doors, `WedgeReach::reach` with its cover-reading caches. On an edge
whose carrier is a line, each door returns the lever read from the LINE:
- **from below:** the farthest face vertex, or the edge's end, from the line;
- **from above:** the farthest of `Cover::distance_range(Site::Line)` over the
  two faces' covers and the edge's own, and those points.

Every caller takes the bound it took on the base, `.below()`. So the suites
below run with the line site in place of the middle. Unchanged:
- curved edges, which stay at the middle;
- the lift's turns (`WedgeReach::unlifted`) and the site mint, which stay at
  the middle.

The same build records at every door call:
- **The body class.**
  - `plane-plane-straight`: both faces planes, every boundary edge a line.
  - `plane-plane-arc`: some boundary edges circle or ellipse arcs.
  - `plane-plane-other`: some boundary edge another kind.
  - Otherwise the two surface kinds (a curved face along a straight edge).
- **The exact truth, where one exists**, on two plane faces: the largest
  distance of the boundary from the line. A convex function's maximum over a
  planar region lies on its boundary.
  - A line edge contributes its ends.
  - A circle or ellipse arc contributes the closed form: its in-plane signed
    distance `A + p cos θ + q sin θ`, maximised over the window at its ends and
    its two critical angles.
  - Elsewhere: no truth.
- **Each composite**, run twice:
  - at the line site's bound from below (the run's own verdict);
  - at the middle site's bound from below (the base's reading), detached.
  A verdict is "moved" when the two differ.
- **The line site's two bounds** against each other: every scalar decision
  paired, as in M-b.

**Test outcomes** are compared with the middle-site run under the same
instrumentation (`m-nextest-*.txt`, M-b's runs). Tests that time out under the
instrumentation in one run and not the other are listed apart.

**Coverage:**

| ε | crates | tests |
|---|---|---|
| 1e-9 | topo, step-import, sweep, editor-core | all 12,157 |
| 1e-12 | topo, step-import | all 4,799 |
| 1e-12 | sweep | 1,350 of 3,825 run at both sites (one hour) |
| 1e-6 | topo, step-import | all 4,799 |
| 1e-6 | sweep | all 3,825 (it finished inside its hour) |

The lanes are f64, Interval, Dual and Sym. The body classes and truths are f64
only: on the other lanes the class column is the surface kinds, and the
zero-lever count is not read.

## (1) Tests whose outcome changes with the line site

**1e-9: 149 tests change**, plus 14 that only timed out under the middle run's
instrumentation. 149 go PASS → FAIL; none go FAIL → PASS.

| What refuses them | tests |
|---|---|
| **The arm gate** (`dihedral_arm`, a `decide_positive` on the folded arm's length): the line site's bound from below reads 0 or a collapsed arm. Certification refuses `ArmCollapsed`, tier 3 `NoDihedralArm`, the revolve `SliverRim` (rung `Arm`), the boolean's seam `Escalated(LeverArm(Seam))` | 115 |
| editor-core corpus and golden rows that fail because a corpus document (`revolves_on_one_axis`) no longer evaluates, by the same gate | 15 |
| other assertions on bodies that moved: sliver volumes, grazing vertices, a quarter ball's window, two rods' enclosure; and one instrument artefact (`every_suite_file_is_aggregated`, the probe file this measurement adds) | 16 |
| a sliver join or rim read in band | 2 |
| another in-band escalation | 1 |

**Other rows** (every change, PASS → FAIL except where noted):

| ε | topo and step-import | sweep |
|---|---|---|
| 1e-12 | 22 of 23 changes (one TIMEOUT → PASS) | 39 of the 43 changes among the 1,350 run at both sites |
| 1e-6 | 23 of 26 changes | 86 of 91 changes |

Most are M-e's degenerate fixtures, the tier-3 rows that read a lever of 0, and
the same ruling and arc bodies as at 1e-9.

**What makes the lever 0 or short.** The line site's bound from below is read
off the faces' vertices, as the middle's is. A face whose vertices all lie on
or near the edge's line reads a lever of about 0, however far its boundary
runs from the line between them. Such faces are:
- a full cylinder or cone wall whose only vertices are on its seam ruling;
- a plane face bounded by arcs whose ends sit on the line;
- a cylinder wall beside a strut, both ends of every arc on the strut
  (`a_closed_conic_in_the_partners_face::a_boss_inside_a_hole_sharing_a_vertex_beside_the_tip`
  reads a lever of 6e-7 at the line against 0.5 at the middle).
The middle site hid this, because a vertex on the line still stands away from
the edge's middle.

## (2) Composite verdicts that move (line site against the middle site), against truth

1e-9, f64 unless noted, verdicts at the bound from below, counted over every composite call (`CD` rows):

| Class | Moved, same action (PAYLOAD) | Moved action | Judged against |
|---|---|---|---|
| plane-plane-straight, line bound = truth | ≈ 6.57 M (WV, margins) | 25 WV + 1 seam: `Seam`/`Lamina`/`Crease` → `InBand`, `Transverse` → `Err(Lever)` | **the truth**: the line site's bound from below IS the truth here, so every move is toward it |
| plane-plane-arc, line bound = truth | 62 | 2 | the truth: toward it |
| plane-plane-arc, line bound < truth | 578 | 347 WV (mostly `Crease` → `InBand`), 166 seam (`Transverse` → `Err`), 3 rider (`± → 0`) | the truth: the line site's bound from below under-reads it (402,282 of 1,388,796 reads), so these moves are AWAY from the truth: refusals the truth would not make |
| Cylinder or cone along a line | ≈ 3.2 k | WV 1,420 (`Crease` → `Seam` among them; `End` → `InBand`), must-carry 98 (`Transverse` → `UnderDetermined`), seam 74 (`Smooth` → `Err`), rider 2, `rules::edge_wedge` 1 | **no truth**. The `Crease` → `Seam` and `Transverse` → `UnderDetermined` moves come from a lever under-read by the vertices: the corner becomes smooth on a lever of 6e-7. That is the wrong direction, a served verdict, not a refusal |
| other (spline, mixed) | | a few `InBand` | no truth |

**Plane-plane-straight is not quite exact.** Its bound from below read above the
truth on 1,567 reads and under it on 4,919, of 11.93 M. These are faces whose
boundary edges' carrier ends are not the face's vertex points. That is the
mid-construction case M-c found, and it was not chased further.

## (3) The open row's extrude table, against the profile's declared tangency

Probe: the row's own profile. A line 0.05 long into an arc of radius 1
sweeping 0.05 rad, `φ` off tangent, extruded at depths 1 and 4 and revolved
20 m off the axis, at 1e-9.

| φ | middle site (base) | line site |
|---|---|---|
| 5ε, 10ε | wall join escalates at φ·0.5 (depth 1), φ·1 (depth 4): the row's table | wall join reads **smooth**: lever ≤ ε/φ, so ≈ 0.05, not 0.5 or 1. Certification's `tangent_normal_parallel` (levered at 1/κ_rel = 1 m) then escalates at φ·1 |
| 15ε, 19ε | depth 1 escalates; depth 4 **builds a sharp edge, tier 3 passes** | wall join reads smooth (lever ≤ 0.0526). `tangent_normal_parallel` refuses, "definitely exceeds the tolerance band" |

The line site reads the lever the row expects, 0.05. The tangency the profile
declared is then read as tangency. What refuses it is certification's
normal-parallelism check, at the arc's curvature arm. The revolve's rim is a
circle, so it is unchanged.

## (4) Both bounds at the line site: straddles with both finite

At the middle there were none (M-b). At the line site, 1e-9, f64 (pairs below
→ above):
- **`dihedral_wedge` 0 → `+`:** WV 1,414, must-carry 98.
- **`dihedral_arm` 0 → `+`:** WV 351, seam 234, rules 1.
- **`bool_rider_wall_side` 0 → `−`/`E`:** 3.
- Interval: arm 9.

At 1e-12: wedge 0 → `+` WV 594, must-carry 38; arm 0 → `+` WV 66, seam 46.
At 1e-6: wedge 0 → `+` WV 120, must-carry 8; arm 0 → `+` WV 314, seam 167, rules 1;
rider 1.

Every one has the bound from below at or near 0 and the bound from above
definite. The bodies are those of (1) and (2): faces whose vertices lie on the
line while their arcs run off it. **The vertices are not a bound from below
worth the name at the line site** on any face with a curved boundary.

## (5) Is `Cover::distance_range(Site::Line)` tight?

The bound from above against the truth, 1e-9:

| | = truth | (1, 1.1] | (1.1, 2] | (2, 10] | > 10 | below the truth |
|---|---|---|---|---|---|---|
| plane-plane-straight (11.93 M) | 11,929,555 | 1,104 | 71 | 0 | 0 | 0 |
| plane-plane-arc (1.39 M) | 926,273 | 92,330 | 248,776 | 107,253 | 14,129 | **80** |

- **Straight edges:** tight on segments and points: the face cover is the
  convex hull of its boundary, and a segment's distance from a line is exact.
- **Arcs:** loose. Over a third of arc-bounded reads exceed the truth, by up to
  more than 10×. An arc piece's distance from a line is not read in closed form.
- **Curved faces along a line** have no truth. Their bound from above against
  the bound from below is not a looseness measure there, since the bound from
  below under-reads.
- **The reads where the bound from above fell below the truth are not
  attributed:** 80 at 1e-9, 78 at 1e-12, 78 at 1e-6. Either a cover that is not a cover on an arc piece read from a
  line, or this truth's closed form applied to an arc that is not in its face's
  plane (mid-construction, as in M-c). The closed form assumes the arc lies in
  the face's plane. One run with the door and the arc's off-plane distance
  logged would settle it.

## M-e's degenerate fixtures, once the lever is 0

Every face has all its vertices on the edge's line, so the line site reads 0.
Each is refused by the arm gate:
- certification `ArmCollapsed`:
  - `tier3_tests::description_references_keep_a_surface_alive`
  - `tier3_tests::dangling_description_is_a_tier1_error`
  - `euler::tests::the_gate_carries_and_refuses_a_line_under_an_intersection_description`
  - `geometric_cube::near_tangent_intersection_attachment_escalates`
- tier 3 `NoDihedralArm` on the edges:
  - `tier3_tests::sliver_dihedral_at_rest_is_rejected`
  - `review_m2_pr3::survives_sliver_dihedral_isolated_at_rest`
- the boolean's seam `Escalated(LeverArm(Seam))` with margin 0:
  - `a_vertex_through_a_declared_seat::a_seat_divided_along_a_ring_or_met_with_a_seat_refuses_at_the_carry`

Each such test fails because it pins the refusal it gave under the middle's
lever.
