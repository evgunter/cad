# CONTACT-4: contfp reads every loop on its edges' own carriers

This file is the PR body for the combined landing. The orchestrator
folds it into that PR and deletes it at merge.

Carries `work/contact/contfp-walks-the-vertex-polygon-of-an-arc-bearing-loop`
(P0) and `work/contact/point-on-arc-endpoint-zone-compresses-by-sin-half-width`
(P1). It includes the review's fix pass (F1–F6) and the delta review's
(M1, M2, m1, m2 and the style items).

## What changed, and why

### `contfp`: one boundary pass, then one walk

`boolean::contain::contfp` used to pick a walk per loop from
`loop_shape`:
- `Disc` went to `disc_side`;
- `Polygon` and `ArcParity` went to `point_in_loop`, the vertex-polygon
  ray parity;
- `NoWalk` was refused as `ArcLoopUnsupported`.

`ArcParity` was answered from a polygon that is not the loop's region.
An outward arc leaves region between the polygon and the boundary, and
a point there read `Out`. An inward arc puts non-region inside the
polygon, and a point there read `In`.

`contfp` now runs:
1. **One boundary pre-pass** (`boundary_pre_pass`). It checks vertices
   over all loops first, then every edge over all loops, each on its
   own carrier:
   - a `Line` (or null scaffolding) is the distance to its closed
     segment (`ray_parity::on_segment`);
   - a **circle or ellipse** is read on its own conic: the distance
     from the conic, then the arc's trim as distances;
   - a spiric or spline edge gets no verdict.

   All three arms are one function,
   `splitting::containment::LoopEdge::contact`. It is the one boundary
   reading of a planar loop's edge, shared with the carrier walk's own
   pass.
2. **One walk that trusts that pass**
   (`splitting::containment::carrier_loop_side`). It reads each loop on
   its carriers and answers `In` or `Out`. It runs no boundary pass of
   its own, so a point the pre-pass placed off the boundary is never
   re-decided by a second set of rows. A crossing at `q` itself, which
   the pre-pass rules out, is a graze. `None` becomes
   `ContainError::ArcLoopUnsupported`.

**Why the pre-pass is one pass (F1).** Before the fix pass, `contfp`
ran two pre-passes in series:
- its own (lines and circles);
- then the carrier walk's (lines and conics).

A point on an ELLIPSE edge fell through the first, was caught by the
second as `OnBoundary`, and came out as `Escalated(invalid(band,
"bool_contfp_boundary"))`. That is a minted `Invalid` diagnostic, which
the census carries as `CensusEscalated`: the fabricated-margin shape
`validate.rs`'s `CensusUnsupportedCause::Containment` doc condemns. The
review probe hit it on 80 of 80 on-edge points of the cut cylinder's
section face.

Now the conic arm is in the one pass, so such a point reads
`OnEdge(edge)`, or `OnVertex` within the band of an end.
`bool_contfp_boundary` is gone from the tree. The walk's own pre-pass
survives only for `point_in_carrier_loop`'s other caller,
`solid_contain::point_in_face`, which wants `OnBoundary` as a verdict.
That pass is the same `LoopEdge::contact` over the loop's edges, not a
second spelling.

**Line rows.** A line is now read as the distance to its closed segment
(`on_segment`, rows `bool_contact_edge_length` and `bool_contact_edge`)
instead of two span gates and a perpendicular (`bool_contact_edge_span`,
retired). The walk after the pre-pass no longer re-checks lines. The old
span gates had a hole: a point whose foot fell in the span gate's band
near an end, but more than the band from the vertex, was never decided
`OnEdge`. With nothing downstream to catch it, it would have been
answered `In`/`Out`.

**Other doors checked against F1:**
- `curved_boundary_containment` is the same `boundary_pre_pass`, so it
  gains the conic arm as well. The conic reading folds the out-of-plane
  miss `(q − c)·n̂` into the distance, so a point on a cylinder wall far
  from a rim's plane is off that rim, exactly as `point_on_circle`'s
  hypotenuse was.
- `solid_contain::point_on_wall_in_face` asks a different question: a
  chart trim on a ray lane, where a `Zero` is a graze that abandons the
  ray. It is not a boundary verdict, and it is untouched.

**Deleted:**
- `point_on_arc`, which lost its last caller;
- `UnrowedCarriers` and its `Chord` arm, which judged an ellipse by its
  chord;
- the minted `bool_contfp_boundary`.

`point_on_circle` stays for `reduce.rs`'s point split. Its row,
`bool_contact_arc`, is the same distance a circle edge's reading meters
under the same name.

What happened to each `LoopShape` arm inside `contfp`:

| arm | fate in `contfp` | why |
|---|---|---|
| `Polygon` | deleted | An all-line loop goes to `point_in_loop`'s own walk (`polygon_walk`, the same `point_in_loop_*` rows), minus its pre-pass. |
| `Disc` | deleted | The walk is exact on a loop of one circle's arcs, and refuses nothing `disc_side` answers: it refuses only spiric or spline edges. No hot path was identified where one radial decide beats ≤16 one-root rays, so no speed claim is made, and none is needed to delete it. |
| `ArcParity` | deleted | This is the defect: a polygon that is not the region. |
| `NoWalk` | deleted | Answered. A half-disc cap, a half-cylinder cap and a lens are walked on their carriers. |

`LoopShape`, `loop_shape`, `LoopCircle` and `disc_side` stay in
`contain.rs`. Their only consumer is now tier 3's check 9
(`validate.rs`), which ATREST-12 moves onto the same walk; after that
they have no caller, and ATREST retires them. Their docs are made true
for that consumer (F3). `loop_shape`'s doc said the points it serves
come through a boundary pre-pass, and check 9 runs none. It now says
the off-boundary premise is the caller's, supplied by check 9's contact
arms. `validate.rs` is not touched.

### A steep ellipse: bounded on both sides (M1, M2)

Before the second fix pass, an ellipse was read in unit coordinates
through ONE lever, the smaller semi-axis `b`. That is conservative
where a `Zero` only abandons a ray. In the boundary pass a `Zero` is a
VERDICT, and the lever was wrong in both directions:

- **M1, `Corrupt` on a legal body.** The end row was the unit chord
  levered by `b`. At an end on the minor axis the edge's speed is `a`,
  so the End zone reached `(a/b)·ε` along the edge, while the vertex
  pass calls a point definite from `10ε`. Past `a/b = 10`, a point
  between `10ε` and `(a/b)·ε` from the vertex read `End`, which the
  pre-pass turned into `Corrupt`.
  - The review measured this on a disc prism cut at tilt 1.5 rad
    (`a/b = 14.1`) and 1.52 rad (`19.7`).
  - A brick corner on that edge 15ε from the vertex gave the census's
    `CensusUnsupported{Face, Containment(Corrupt)}`, which tells the
    user to repair their topology.
- **M2, `OnEdge` for a point definitely off.** `(ρ − 1)·b` understates
  the distance by `b/a` near the major vertex.
  - At tilt 1.5, a point ±12ε off read `OnEdge`; at tilt 1.52, ±15ε
    did.
  - The band sweep found 24 such `OnEdge` answers at 10.5–13ε off the
    middle of an edge.
  - The reduction splits an edge at an `OnEdge` point.

`LoopEdge::contact` now reads an ellipse with its metric bounded on
BOTH sides, each verdict on the bound that makes it sound:

- **On the conic only where an UPPER bound is within the zero band.**
  The upper bound is `|Q − P|` for `P` = one Newton step of
  `F = X²/a² + Y²/b² − 1` from `Q`, snapped radially onto the
  ellipse: any point of the ellipse bounds the distance from above.
- **Off only where a LOWER bound clears the escalation band.** The
  lower bound is `2|F|/(g + √(g² + 4|F|/b²))`, `g = |∇F(Q)|`: along the
  segment to the foot, `|∇F|` grows at most at the Hessian's norm
  `2/b²`, so `|F| ≤ g·d + d²/b²`.
- **Between the two, it escalates** on the bound that landed in the
  band. The bounds part by more than the band's own ratio only where
  the ellipse's tightest radius of curvature `b²/a` is within a few ε.
  That is the one residue with no single margin, and there it escalates
  `Invalid`, as the site says.
- **An end is the EXACT distance from `q` to the end point.** The side
  of the ends is the Newton foot's chordal defect levered by the
  LARGER semi-axis. A unit angle costs at most `a` metres of arc, so
  the margin bounds the arc length to the nearer end from above.
- **A circle is untouched,** bit-identical: its unit coordinates are an
  isometry scaled by the radius, and one lever is exact.

**`End` against the vertex pass.** An end is now read as the exact
distance from `q`. If it is within the band while the vertex pass
placed `q` definitely clear of the edge's vertices, the stored vertices
sit off the carrier's end by more than 8ε. The pre-pass decides that
distance (`bool_contact_arc_end`) and escalates on it if it is in the
band. `Corrupt` is left only where the edge's end is definitely off
BOTH its vertices: a body whose vertex is not where its own edge ends.

**Measured,** at ε = 1e-9 and K = 10, with the review's probes:

| probe | at `94a2feb` (review) | now |
|---|---|---|
| `contfp` 11–18ε inside each end, tilt 1.5 and 1.52 | `Corrupt` (M1) | `OnEdge` at every probe, every tilt (0.3 to 1.52) |
| census: brick corner 15ε from the end, tilt 1.52 | `CensusUnsupported{Face, Containment(Corrupt)}` | no containment refusal: the contact is read (the remaining findings are the curved-face `CensusUndecidable` pairs the body carries anyway) |
| ±12ε and ±15ε radial at the major vertex, tilt 1.5 and 1.52 | `OnEdge` (M2) | `Out` outside and `In` inside, at ±12, 15, 20, 40 and 100ε, every tilt |
| band sweep (vertex rings and both normals of every edge, 10.5ε to 1e5ε; slot, lens, inward square, half-disc, pac-man, pie, cut at 0.3 and 1.52) | 24 wrong `OnEdge` at 10.5–13ε, and `Corrupt` | **0 wrong, 0 `Corrupt`, 0 escalations** (edges 23,492 `In` / 23,442 `Out`; rings 3,388 `In` / 5,687 `Out` / 353 `OnEdge`, where a ring crosses an edge) |
| the region probe (every face, grid and on-carrier points) | 0 wrong | 0 wrong; spiric caps unchanged (1899 right, 0 wrong, 318 refused of 2217 in-face) |

### A ball clearance in the band skips the ray (m1)

The ray test against an uncrossable edge's ball used to propagate an
escalation of `point_in_arc_loop_reach`, which escalated the whole
walk. Points 0.3–1 m outside vessel face 2v1 read `Escalated`. A
clearance in the band is a ray that COULD meet the ball, so it is now
abandoned like any graze. The review's probe reads all 32 of its points
`Out` (it read escalations before). The row is
`an_in_band_ball_clearance_skips_the_ray`.

### The spiric ball's reach is pinned (m2)

`a_spiric_ball_that_is_nearly_tight_still_holds_the_arc` builds a
spiric cap of the torus `R = 10, r = 1` cut at `offset = 5`, with a
short arc (`w = 0.2`) about `v = π/2`. There the oval's speed is within
4% of the bound. A point 3 mm past the arc's end along its tangent lies
inside the ball (0.1184 m from its centre against a reach of 0.1203 m),
and the cap refuses.

Both mutations go red on that row:
- the speed taken as `r` (the offset factor dropped);
- the reach × 0.9.

Each shrinks the ball past the point and lets a ray that clips the
uncrossable arc answer.

### Style items

- **One home for the invalid-margin helper.** `fn invalid` had three
  identical copies (`containment.rs`, `contain.rs`, `census.rs`). It now
  lives at `ray_parity::invalid`.
- **One boundary reading.** The walk's `Boundary::Verdict` pass and
  `contain`'s edge pass are both `LoopEdge::contact` over a
  `CarrierLoop`. The pre-pass hands its `CarrierLoop`s to the walk
  rather than reading the loops twice.
- **The type says what the walk answers.** `carrier_loop_side` returns
  `Option<bool>` (inside), so `contfp` has no `OnBoundary` arm to rule
  out.
- **`Boundary::Decided` asserts instead of folding.** Its `On`/`End`
  arm is an `unreachable!` stating the precondition: the caller's pass
  reads the same arithmetic and placed `q` off the edge.
- **The spline ball** is centred on its control points' bounding-box
  centre, with reach to the farthest control point. It still holds the
  control hull, and so the curve.

### The end-of-arc zone (`arc_trim`)

The old `point_on_arc` decided the trim through the cosine window
`Margin::levered(r̂·m̂ − cos(w/2), R)`. For a point an arc length `s`
past an end, that margin is about `−sin(w/2)·s`. So the window's `Zero`
reached `ε/sin(w/2)` along the carrier, and its escalation reached
`10ε/sin(w/2)`.

`splitting::containment::arc_trim` works in the conic's unit
coordinates:
1. The chord distance to either end. `Zero` means `End`.
2. Otherwise, the chordal-defect sum
   `(|e − m| − |p − m|) + (|p + m| − |e + m|)`, which is `2(g(α) −
   g(w/2))` for `g(x) = cos(x/2) − sin(x/2)`. Its slope is at least ½
   on `[0, π]`, so the margin is never smaller than the arc length to
   the nearer end.

The site states:
- the end symmetry the sum relies on (it is taken from `t0`'s end
  because `|e₀ ∓ m| = |e₁ ∓ m|`);
- its floating floor (unit-coordinate rounding is about `1e-16 · lever`
  metres, and the fixtures stay three orders clear of it);
- that it is a second home of `validate::window`'s arc arm. One home is
  filed on ATREST's slate (F4).

`ConicArc` carries its window once, as the end parameters (F6). The
crossing row's cosine window is derived from them where a ray needs it.
There a `Zero` only abandons the ray, so its compression costs a short
arc rays, never a count.

**The `point_on_arc` row: retired, but not by the switch alone.**
Measured, not assumed. The carrier walk's pre-pass used the same cosine
window, so switching `contfp` onto it would have moved the compressed
zone, not removed it. On the base, a point 50ε inside the end of a
`w = 0.02`, `R = 10` arc escalates `bool_contfp_boundary`; on the
near-full mirror, the same point reads `Out` (wrong). Both doors read
`arc_trim` now. The rows probe both ends of both arcs and are green at
every ε row.

### The spiric and spline refusal is confined (F2)

The walk has no crossing row for a spiric or spline edge. It used to
refuse anywhere inside a ball centred on the loop's FIRST VERTEX with
radius `|c − anchor| + R + r`, about `3(R + r)` from the torus centre.
On the torus vessel cavity's spiric caps, that refused every in-face
point.

Now:
- **Each such edge is held as its own ball.** A spiric arc's ball is
  centred at the arc's midpoint `P(mid)`, with reach equal to the oval's
  speed bound `r(R − r)/√((R − r)² − offset²)` (the carrier's own doc)
  times half the parameter width. A spline keeps its control-hull ball.
- **A scheduled ray that could meet any such ball is abandoned like a
  graze.** The test is `point_in_arc_loop_reach`, the ray's distance
  `|w − d·max(w·d, 0)|` from the ball's centre less its reach, taken
  without a branch.
- **The rest of the loop answers along any ray that definitely misses
  every ball.**
- **`None` (the refusal) only where no scheduled ray avoids them.** That
  includes every point inside a ball, and so every point on the edge.

Measured with the review's probe on the vessel cavity (`t = 1/128`), at
the spiric caps 1v1 and 2v1:

| | in-face points | answered right | wrong | refused |
|---|---|---|---|---|
| base (vertex polygon) | 2217 | 2094 (94.5%) | **123 (5.5%, false `Out`)** | 0 |
| head before fix pass | 2217 | 0 | 0 | 2217 |
| head | 2217 | 1899 (85.7%) | **0** | 318 (14.3%) |

| | whole grid, both caps (7442 points) | right | wrong | refused |
|---|---|---|---|---|
| base | | 7318 | 123 | 0 |
| head | | 6660 | 0 | 781 |

At `t = 1/256`: head 1968 of 2298 in-face points right, 0 wrong, 330
refused. The 80 on-edge probes on the spiric caps refuse (each is on
the spiric edge). All other faces of the vessel cavity, the vessel
quarter, the D-rod, the slot, the lens, the inward-bulge square, the
half-disc, the pie, the pac-man and both halves of the cut cylinder
read 0 wrong and 0 refused on the grid, and every on-edge probe reads
`OnEdge`/`OnVertex`.

### The refusal's name (F6)

`ContainError::ArcLoopUnsupported` now means "a spiric or spline edge
stands in the way of every ray from the point". Its definition says so
and says why the name is kept: tier 3's census renders the arm in
`validate.rs` (ATREST's ground), and a rename has to move that match
with it. Its `Display` and `BooleanError::ArcLoopContainmentUnsupported`'s
doc and message name the mechanism. `validate.rs`'s `classify_contain`
still renders the old sentence, which is filed.

### Predicate roster

- **New:**
  - `bool_contact_edge_length`;
  - `bool_contact_arc_end` and `bool_contact_arc_trim`;
  - `point_in_arc_loop_conic_end` and `point_in_arc_loop_conic_trim`.
- **Retired:** `bool_contact_edge_span`, and `bool_contfp_boundary`,
  which never reached the funnel.
- **Changed meaning:**
  - `point_in_arc_loop_conic_on` is now a distance;
  - `point_in_arc_loop_reach` is now a ray's clearance from an
    uncrossable edge's ball;
  - `bool_contact_arc` also serves ellipses, where it is a lower bound.

`docs/K-REPORT.md` and `docs/predicate-dimension-audit.md` carry the
rows.

## Per-caller mapping

Every caller already matched all four `ContainError` arms, and none of
their match arms changed. What changes is which inputs reach which arm.

| caller | `Escalated` | `RayExhausted` | `Corrupt` | `ArcLoopUnsupported` | non-`Out` verdicts |
|---|---|---|---|---|---|
| `reduce.rs` edge-crossing, root lane (`contfp(y, face, …)` after `roots.first()`) | `BooleanError::Escalated` | `ClassificationInvariant` | `CorruptOperand` | `ArcLoopContainmentUnsupported { operand, loop }` | trace-accepted |
| `reduce.rs` edge-crossing, line lane (the `d1/(d1 − d2)` crossing) | same `esc` | same | same | same | same |
| `reduce.rs` vertex-on-face (`vertex_face_contact`, `esc(e, x_is.other())`) | same `esc` | same | same | same | `In` → vf contact; `OnEdge` → split the edge; `OnVertex` → vv |
| `ops.rs` extent scan (`contfp(y, yf, normal, witness, …)`) | `Escalated` | `ClassificationInvariant("extent scan: …")` | `ClassificationInvariant("… corrupt …")` | `ArcLoopContainmentUnsupported` | `Out` skips, `In` escapes |
| `census.rs` `contain` helper | `CensusEscalated` | `CensusUnsupported(Containment)` | same | same | the census's own arms |
| `chart_region.rs` witness search (`inside`) | not a witness | not a witness | not a witness | not a witness | only `In` counts |

**New inputs to `Corrupt`:**
- a conic edge whose end, measured EXACTLY, lies within the band of
  `q` while the vertex pass placed `q` definitely clear of the edge's
  vertices, AND whose end is definitely off both of its vertices. Where
  that last distance is in the band, the answer is an `Escalated` on
  it instead.
- a conic span wound past a period.

A certified body reaches neither.

**New input to `Escalated(Invalid)` on `bool_contact_arc`:** an ellipse
bending tighter than the band resolves (`b²/a` within a few ε), where
the two bounds on one distance straddle the whole band.

**`Escalated(bool_contfp_boundary)` is gone.** A point the pre-pass
misjudges within the band now makes every crossing ray graze. That ends
in `RayExhausted` rather than a minted margin.

**Answer → refusal:** a planar face with a spiric or spline edge, asked
about a point where every scheduled ray could meet that edge's ball. The
base answered from the vertex polygon (5.5% false `Out` on the cavity
caps). The head refuses `ArcLoopUnsupported`: 318 of the caps' 2217
in-face points, all near the spiric rim.

**Refusal → answer, and wrong → right.** Each row below is red on the
base and green at the head, at every ε row:

| row | base | head |
|---|---|---|
| bored D-rod (flat 0.3, bore 0.15, both creases filleted), transverse caps `z = 0, L`, lune point `(−0.3, 0)` via `contfp` | **`Out`** | **`In`** |
| the same cap with a brick standing in the lune, through `validate_pseudomanifold` (public door) | 0 `VertexOnFace` contacts: the body passes the vertex-on-face sweep silently | 4 `VertexOnFace` contacts on the cap |
| slot (4 vertices, two semicircles), 41×25 grid | `Out` inside the semicircular ends, e.g. `(−1.4388, −0.2390)` | every probe matches the analytic region |
| rounded rectangle (8 vertices, four quarter arcs), same grid | not reached on the base: the row stops at the slot's first wrong probe | every probe matches the analytic region |
| pie slice `w = 0.02`, `R = 10`: 50ε and 500ε inside each end | `Escalated(bool_contfp_boundary)` | `OnEdge(arc)` |
| the same, past each end | not reached on the base (the row stops at its first case) | `Out` |
| pac-man `w = 2π − 0.02`: 50ε inside each end | **`Out`** | `OnEdge(arc)` |
| the same, past each end (in the mouth), and deep in the body | not reached on the base; its vertex polygon is the mouth triangle, not the region | `Out`, and `In` |
| cut cylinder's section face (two ellipse arcs over two vertices): 80 points on the ellipse edges | `ArcLoopUnsupported`: the base refused the whole face as `NoWalk` | `OnEdge`/`OnVertex` (before the fix pass: minted `Escalated`) |
| a brick whose corner sits on the section's ellipse edge, through the census | the face refused | a contact, with no `Invalid` escalation |
| vessel cavity (`t = 1/128`) face 5v1, circle arcs, grid | 841 wrong, including false **`In`** (e.g. `(−0.0985, 0.1009)`) | 0 wrong |
| vessel cavity face 7v1 | 699 wrong, including false **`In`** | 0 wrong |
| vessel cavity faces 3v1 and 9v1 | 560 and 533 false `Out` | 0 wrong |
| `ConicHit` unit row, both arcs, `s ∈ {20, 50, 500, 5000}ε` on both sides of both ends | `None` inside the compressed zone (as `point_on_arc`) | `On` / `Carrier`; `End` only within the band |
| `verbs_pierce_r1` half-disc cap × box, both senses | the containing sense refuses `ArcLoopContainmentUnsupported` | buried truth `π + 0.09` and disjoint `π + 0.18`, one each |
| `verbs_pierce_r2` half-disc cap × box, both senses | refuses | buried and disjoint truths |
| `verbs_pierce_r2` lens cap × box | refuses | `va + 0.04` (exact union) |
| `census_containment_cause` lens under a box | `CensusUnsupported(ArcLoopUnsupported)` ×N | 4 `VertexOnFace` contacts, no refusal |

The review measured the vessel QUARTER's face 5v1 with false `In` as
well. The spec said the slot and the rounded rectangle "must answer as
before". That holds only where the polygon and the region agree: in the
arc bulges the base was wrong, and the head is right there.

## Rows added, moved, deleted

- **Added in `crates/sweep/tests/contfp_reads_arcs_on_their_carriers.rs`:**
  - the D-rod lune via `contfp`, and via the census;
  - the slot and the rounded rectangle;
  - the short arc and the near-full arc, each probed at both ends;
  - a point on an ellipse edge, via `contfp`;
  - a corner on an ellipse edge, via the census.
- **Added in `topo`:**
  - `contain::tests::an_arcs_end_zone_is_the_bands_own_width`;
  - `census::tests::a_spiric_caps_refusal_reaches_the_census_as_itself`,
    a hand-built planar cap with a certified spiric edge. It executes the
    census's `ContainError → CensusUnsupported` carriage and the "no
    fabricated margin" invariant.
- **Re-signed to the right answer.** Three rows asserted the refusal
  and now assert the exact volumes:
  - `verbs_pierce_r1_probes::r1_a_box_through_a_half_disc_cap`;
  - `verbs_pierce_r2_probes::r2_a_box_through_a_half_disc_cap_measures_the_mixed_loop_remainder`;
  - `…::r2_a_box_through_a_lens_cap_measures_the_all_arc_remainder`.
- **Reshaped.** `census::tests::a_region_walk_refusal_on_a_separated_vertex_is_the_filters_to_answer`
  became `a_separated_vertex_in_a_half_disc_caps_plane_is_decided_by_both_sweeps`.
  It asserts the decided answer: each far vertex is `Out` through
  `contfp`, and none is a `VertexOnFace` contact.
- **Deleted.** `census_containment_cause::no_fabricated_containment_margin`
  and `…::the_arc_loop_refusal_reaches_the_user_as_itself`. Their lens no
  longer refuses. The spiric census row above carries both invariants.

## Sweep

**Pattern 1: a vertex-polygon walk handed a loop that may carry an
arc.** `grep -rn 'point_in_loop('` over `crates/`:

| hit | disposition |
|---|---|
| `boolean/contain.rs` `contfp` (`Polygon`/`ArcParity` arm) | **fixed** |
| `splitting/containment.rs` `carrier_walk`'s all-`Chord` branch | correct: every edge is a line |
| `solid_contain::point_in_face` | fixed by ATREST-9 (`point_in_carrier_loop`) |
| `chord_join::rehome_rings` (`point_in_loop` over the split's run) | not this unit: REACH/TANG ground, already filed as `work/reach/rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk` |
| `validate.rs` check 9 nesting arm | gated to `Polygon`/`Disc`, so correct as gated; ATREST-12 moves it onto the walk |
| `topo/tests/review_m3_pr3_pil.rs`, `m3_pr3_split.rs` | test fixtures on polygon loops |

**Blind spot of pattern 1: a walk that reaches `ray_parity` directly.**
`grep ray_parity::`:

| hit | disposition |
|---|---|
| `chart_region.rs` `point_in_polygon` | not an instance: `loop_uv_polygon` refuses every non-straight chart image |
| `chart_bound.rs` | not an instance: it carries each arc's envelope `E_L` beside the chord polygon |
| `boolean/contain.rs` `on_segment` | the line arm of the pre-pass, over a straight edge |
| `splitting/containment.rs` | the walk itself |

**Pattern 2: a cosine-window end zone read as a verdict.**
`grep -rn 'cos_half\|r̂·m̂\|(width \* half).sin_cos'`:

| hit | disposition |
|---|---|
| the old `contain::point_on_arc` | **deleted**; the pre-pass reads `LoopEdge::contact` |
| `containment` pre-pass (`in_window`) | **fixed** (`LoopEdge::contact`) |
| `containment::conic_crossings` (`in_window`) | kept: a `Zero` there is a graze that abandons a ray, never a verdict |
| `solid_contain::point_on_wall_in_face` / `point_on_cone_in_face` / `point_on_torus_in_face` (`chart_azimuth_margin`), `point_on_sphere_in_face` | not this unit: a boundary graze on a ray lane (the doc's ledger row F8 owns the narrow-window fix) |
| `curved_face_containment` period guard | a chart-form question, not an end zone |

`solid_contain`'s by-hand site list is updated: `point_on_arc` is off
it, and `ConicArc::in_window` is on it. The count stays at three
restated sites.

**The pattern's blind spot:** an end-zone test without a cosine, such
as an `atan2` window. `grep atan2` over `topo/src/boolean` and
`topo/src/splitting` finds five sites, and none is a window membership
test:
- `join.rs`: a test's round trip onto an ellipse;
- `boxes.rs` ×3: a box's extremal angles;
- `classify.rs`: a crossing root's phase.

**Pattern 3 (F1's shape): an `invalid(band, …)` minted where a real
margin existed.** `grep -rn 'invalid(band'` over `boolean/contain.rs`
and `splitting/containment.rs`:
- `bool_contfp_boundary`: deleted.
- `bool_contact_vertex`, the conic reading's `on` row and `arc_trim`'s
  `end` row: these remain, and each is the impossible NEGATIVE arm of a
  distance. It is a broken invariant, not a quantity.

## Territory seam

- `crates/topo/src/splitting/containment.rs` is REACH's path; the walk
  in it is ATREST-9's. The edits there are `ConicRows`, `ConicArc` (now
  `pub(crate)`, with `of` and `hit`), `ArcTrim`/`arc_trim`,
  `polygon_walk`, `carrier_loop_side`, and the Unrowed ball and ray
  test. They are announced on `work/atrest/log.md`.
  `scripts/work.py territory` flags the path.
- `validate.rs` is untouched. After this change `LoopShape` has one
  consumer, check 9, announced on `work/atrest/log.md`. Its stale text
  is filed as
  `work/atrest/check-9-and-classify-contain-describe-contfps-retired-polygon-walk`:
  `classify_contain`, plus the review's four sites at `:593-596`,
  `:5572-5575`, `:6497` and `:6530`.
- `work/tang/arc-aware-point-in-loop.md` (#1076) has a dated note that
  the `contfp` site is closed.
- `docs/KERNEL-VERBS.md`: two sentences about `contfp`'s posture are
  re-worded to the moved code. `git log -S` finds them first written by
  an agent commit (`4d7faeb02`), so there was no ratification to wait
  for.

## Filed and closed

- **Filed on ATREST's slate:**
  - `check-9-and-classify-contain-describe-contfps-retired-polygon-walk`
    (P2);
  - `validate-window-arc-arm-folds-onto-arc-trim` (P3; ATREST-12 is the
    natural taker).
- **Filed and closed on CONTACT's slate, in the fix pass:**
  - `census-containment-refusal-carriage-has-no-executed-fixture`,
    closed by the spiric census row;
  - `contfp-has-no-typed-on-edge-row-for-an-elliptic-arc`, closed by
    F1.

## Local verification

At the head of the second fix pass, with `CARGO_INCREMENTAL=0` and a
rebuilt target:

- **`cargo nextest run -p topo -p sweep --no-fail-fast`** (3155 tests)
  at eps unset, 1e-6 and 1e-12: all green.
- **The review's probes** (`zz_c4d.rs`, `zz_c4_probe.rs`): 0 wrong,
  0 `Corrupt`; numbers above.
- **Mutations of the spiric ball** (speed = `r`; reach × 0.9): each
  reds `a_spiric_ball_that_is_nearly_tight_still_holds_the_arc`.
- **`cargo clippy -p topo -p sweep --all-targets -- -D warnings`:**
  clean.
- **Rustdoc,** `RUSTDOCFLAGS='-D warnings -A rustdoc::private_intra_doc_links'
  cargo doc -p topo --no-deps --document-private-items --all-features`
  (doc-gate's flags): clean.
- **Every `scripts/gates/*.sh`:** `--selftest` and the real pass are
  green. `interval-square-allowlist` caught a `two * two` spelling,
  which is now the literal `4.0`, and exact.
- **`python3 scripts/work.py lint`:** ok.
