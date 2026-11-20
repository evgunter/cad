# GERM crossing U3 — `curve_face::pose`, `curve_face::cross`, and the small consumers (unit spec)

This spec carries U3 of `work/germ/curve-surface-crossing-has-no-kind-pair-table.md` (its
Ratified section), building C5's curve × face paragraph as ratified in PR 7167
(`crates/geom-brep/README.md`). U3 is one PR, which:
- builds the carrier pose (`curve_face::pose`);
- builds the one crossing reading (`curve_face::cross`), dispatched through `curve_face::route`,
  with one answer type and one refusal (the meetings found plus undecided windows);
- moves five small consumers onto them.

The sweep (U4) and the ray lane's remaining reads (U5) are out of scope. Until U4 the sweep may
call a door's method directly: one method with two callers is acceptable for that span. Read on
`c093993bb2`, with U1 read on the `germ/line-roots-door` lane's uncommitted working tree. Names are
cited, not lines. The spec is deleted when U3 merges.

## 0. What U3 needs from U1 and U2

- **U1** (the line roots door moved, no behaviour change). U3 needs these from it:
  - `curve_face::line_quadric`, with `line_quadric_roots`, `LineRoots::{Roots, Constant}`,
    `LineQuadricRefusal`, `WallRoots`, `About`, the per-kind finders and the private
    `line_cone_roots`/`apex_root_spread`;
  - `reduce::line_wall_roots_of` gone, and trace already reading `line_quadric_roots`. U1 removes
    `ray_crossings`' `Some(Err(_)) => unreachable!` and adds `line_door_refusal`.
  - The axis-parallel, cone apex, lead and spread rungs callable as functions of their own, so that
    pose and the old entries call one predicate per row.
  - U1b (one line × cone rule set) is not a prerequisite, but it must not be in flight with U3:
    both rewrite the apex rungs.
- **U2** (ConicQuadric moved). U3 needs these from it:
  - `curve_face::conic_quadric::conic_quadric_roots`, refusing `DoorRefusal` rather than `BooleanError`;
  - `first_harmonic_arm`, `quadric_ladder`, `cone_roots`, `through_the_apex` and `off_the_apex`
    beside it, its rows renamed under a `DOOR_K_PREFIXES` prefix.
  - U3 must not be in flight with the first-harmonic arm's Part 2 (`germ-rounds` P/Q). Both rewrite
    `first_harmonic_roots`' decide and count, and U3 takes its coaxial and extreme rungs out.

## 1. The types

**`curve_face::Pose<T>`** (new; `curve_pair::pose::Pose` is the curve-pair pose and stays):
- `Lies`: over the whole span; a value reading only.
- `Apart`.
- `Tangent(Touch<T>)`: the pose-decided special case of `Isolated`, one touch.
  `Touch { at: T, point, arc: (T, T) }` is a point of the in-band arc and the arc in span parameters.
- `Through(Singular<T>)`: `Singular { at, point, spread: (T, T) }`, the point where the surface has
  no tangent plane and its spread. In U3 only a cone's apex reaches it, because the spindle route is unbuilt.
- `Isolated(Form<T>)`: `Form` is private and per arm. It holds the harmonics, quadratic or
  candidates the pose computed, which the method takes without re-deciding them.

**`curve_face::PoseRefusal`**:
- `InBand { rung: PoseRung, diag }`;
- `Enclosed`: a spline arm, which has no closed pose;
- `Unbuilt(Route)`;
- `Torus(TorusRefusal)`;
- `Fault(&'static str)`.

`PoseRung` is `PlaneParallel | BellyGraze | AxisParallel | Coaxial | Extreme | Apex | ConeLead |
Meridian`. It takes over the job of `ConicRootFault::decision() == None`: a consumer maps a pose
rung to its coincidence lever (`BooleanDecision::of_conic_root`'s `Coincide::EdgeOnPlane` today)
and a method fault to its crossing decision.

**The meeting.** The curve-pair vocabulary has no one-span meeting.
- `curve_pair::Meeting { point, on_a, on_b }` is two-sided and tells no crossing from a touch.
- `MeetRefusal { fault, found, shared }` is the refusal's shape.
- The nearest type is `curve_face::BoundaryMeeting { point, on_span: Hit<T>, .. }`.

So U3 mints only `curve_face::Meeting<T> { point, on_span: Hit<T>, kind: Kind<T> }`, with
`Kind::{Cross, Touch { arc }}`. It reuses `curve_pair::Hit`, whose `Hit::End` is C5's end hit,
and reuses `MeetFault` for every fault (below). `Hit::On`'s `margin: MarginDiag` does not fit a
parameter-placed root (Q1).

**The answer:** `Crossings<T> { pose: Pose<T>, meetings: Vec<Meeting<T>> }`.
- The meetings ascend along the span and are decided apart by their chord.
- Two meetings within the band of each other are a window.
- `Lies` and `Apart` hold none; `Tangent` holds its one touch; `Through` holds the rest (the
  singular point's own status is Q2).

**The refusal:** `CrossRefusal<T> { found: Vec<Meeting<T>>, windows: Vec<Undecided<T, CrossWhy>> }`.
It is shaped like `MeetRefusal` (meetings decided before it, never the complete set) and read like
`curve_face::Located`: exact outside the windows, silent inside them.
- **The window type:** `curve_face::Undecided`, made generic in its why
  (`Undecided<T, W = BoundaryRefusal>`), so the boundary pass is unchanged.
  `Located::undecided_over` is lifted to a function over `&[Undecided<T, W>]`, so the face reading
  (U4) reads boundary and crossing windows alike. No `Located` twin.
- **`CrossWhy`:** `Pose(PoseRefusal)` or `Method { door: Door, fault: MeetFault }`.
- **A pose refusal** is one window over the whole span. An unbuilt route is the same, so no arm
  refuses whole by kind.

**Entry points** (`curve_face.rs`):
- `pose(span: &Span<'_, T>, surface: &Surface<T>, band) -> Result<Pose<T>, PoseRefusal>`;
- `cross(span, surface, band) -> Result<Crossings<T>, CrossRefusal<T>>`.

`cross` matches `route(..).door` exhaustively, with no wildcard: `pose` first, then that door's one
method. An arm handed a wrong kind answers `MeetFault::Desync`. `Span::endless` replaces
`plane_crossing::SpanEnds`.

## 2. Per arm (`route`'s `Door`)

Each rung below is verified in the tree. It moves out of its door into `pose` as a
`curve_face_pose_*` row: under `TABLE_K_PREFIX`, a row is named by the module that decides it. The
door keeps only its method. An old entry that an unmoved caller still reads (§2.7) becomes a wrapper:
`pose`, then the method, mapped to the old answer. The wrapper decides nothing and is deleted with its
last reader.

**2.1 `PlaneCrossing`** (line, circle or ellipse × plane; `plane_crossing.rs`).
- **Pose rungs.**
  - `conic_plane_candidates`' `plane_crossing_belly_graze`: Negative is `Apart`, Zero is `Tangent`,
    in band is `BellyGraze`.
  - `_plane_parallel` Zero, with the offset Zero, is `Lies`; otherwise `Apart`. The offset decision is
    new in the door, because the caller decides `Parallel { offset }` today. In band is `PlaneParallel`.
  - `_graze_side`: in band is `BellyGraze`.
  - The line arm's `_line_parallel` (today in `span_plane_meet`).
  - Tangent's arc is `graze_reaches_span`'s `δ = acos(1 − z/r)` about the extremum.
- **Method:** `span_roots`, which places each root by `Span::place_param` and decides `_roots_apart`
  and `_root_order`; and the line's closed-form root.
- **Windows.**
  - A root in band of an end (`RootPlace::Undecided`): from that end to the root, widened by the
    escalation at the span's least speed.
  - Two roots in band of each other.
- **Apart stage:** the closed-form belly; no cull.

**2.2 `LineRoots`** (U1's `line_quadric.rs`).
- **Cylinder.**
  - `line_quadric_cylinder_axis_parallel` Zero gives a constant residual, decided `Lies`/`Apart` in
    pose as `constant_residual_roots` decides one. `LineRoots::Constant` leaves it to each consumer
    today (Q3).
  - `_cylinder_disc` Zero is `Tangent` (today `WallRoots::Tangent`, refused `Unsettled::Tangency`);
    Negative is `Apart`.
- **Sphere:** its discriminant, the same way.
- **Cone.**
  - `line_quadric_cone_apex` Zero with `_cone_lead` Zero is `Lies`, a generator (today
    `Unsettled::Degenerate`).
  - Apex Zero with the lead definite is `Through`, its spread `_cone_apex_spread`. Its in-band case is
    today's `Unsettled::AtApex`.
  - The lead Zero off the apex stays the method's `Degenerate`, a whole-span window. It is owned by
    `a-line-parallel-to-a-cone-generator-has-no-root-lane`.
- **Torus:** no closed pose; the pose answers `Isolated` after the ring convention.
- **Tangent's arc** is new: `±√(z/a)` about the double root, in `Centred`'s parameter.
- **Method:** `Centred::roots` and the root slacks, and `line_torus_roots`. A method meeting
  `AxisParallel` after the pose decided otherwise is `MeetFault::Contradiction`.
- **Windows.**
  - An `Unsettled` count is one window over the span, since a closed form has no local spread today.
  - A refused root slack: the root ± its slack.
- **Apart:** the closed-form discriminant. `bool_line_cylinder_clearance` stays in the sweep until U4.

**2.3 `ConicQuadric`** (U2's module).
- **Zero lane (§3) pose rungs**, from `first_harmonic_roots_in_band`:
  - noise;
  - `coaxial`, through `constant_residual_roots`: `CircleRoots::On` becomes `Lies`, a Miss `Apart`;
  - `extreme`: both one-signed is `Apart`; one in the zero band is `Tangent` (today
    `Unsettled::Tangency`), with the near extreme's half-chord `2·asin(√(z/(hi − lo)))` as its arc.
- **The coaxial fall-through.** Today an escalated coaxial test "falls through to the extremes". It
  stays only as a stage of one rule, the extremes deciding the same class (D3). The PR body states
  that proof.
- **Cone pose rungs.**
  - `through_the_apex`'s `bool_conic_cone_apex` Zero is `Through`, with spread `_apex_spread`.
  - `off_the_apex`'s per-root `AtApex` becomes the `Apex` rung in band.
  - `bool_conic_cone_on_surface` gives `Lies`.
- **Method.**
  - The Zero lane: the core's root slack and placement.
  - Otherwise: `door_half_angle_roots`, the ladder for its escalations and `certified_subdivision`
    for the answer.
  - Through the apex: the deflated `G` roots.
- **Windows.**
  - The subdivision's undecided pieces. Threading them out of `crate::half_angle` is new: it refuses
    whole on `Floor` or `Tangency` today.
  - A refused slack: the root ± its slack.
- **Apart:** the extremes on the Zero lane. The ladder lane has no closed form for it. The sweep's
  `conic_clearance` moves into pose in U4, as one match on surface kind;
  `implicit::conic_residual_extremes` is harmonic-else-sampled today.

**2.4 `ConicTorus`** (`circle_torus.rs`, `ellipse_torus.rs`).
- **Pose rungs.**
  - `circle_torus_coaxial_tilt`, `_offset` and `_residual`: `Lies` (a rim circle) or `Apart`.
  - `circle_torus_meridian`: `Lies`.
  - `DoorRefusal::Torus` becomes `PoseRefusal::Torus`, read before any pose as today.
- **The parallel-axes contour reach.** It reads a tangency in band, and stays in the method (Q4).
- **A Villarceau circle** has no rung: it is `Isolated`, and the ladder's `Unsettled` makes it a
  whole-span window, as today.
- **Ellipse × torus** has no special pose (it answers only `Certified`, `Miss` or `Unsettled`).
- **Method:** the two doors' roots without those rungs.
- **Windows:** as in §2.3.
- **Apart:** none closed. The torus clearance stays in the sweep until U4.

**2.5 Spline arms.**
- `SplineComposite` (`composite::crossings`) and `SplineFace` (`spline_face::settle`): the pose
  answers `PoseRefusal::Enclosed`, and their enclosures are the method.
- **Windows:** `composite`'s in-band pieces, and `spline_face`'s floor boxes projected on `t`. A run of
  a spline span in band is a window (Ev, PR 7167).

**2.6 Unbuilt routes** (`Spiric`, `Sheet`, `Periodic`, spiric × spline): `PoseRefusal::Unbuilt(route)`,
one whole-span window.

**2.7 Wrappers kept for unmoved readers.**

| Entry | Its unmoved readers |
|---|---|
| `plane_crossing_lane` | the sweep (`reduce`, U4), `slit`, `factored_crossing`, `offer_rows` |
| `line_quadric_roots` | the sweep |
| `conic_quadric_roots`, `circle_torus_roots`, `ellipse_torus_roots` | `reduce::wall_crossing` |
| `line_wall_roots`, `line_cone_quadratic`, `WallRoots` | the ray lane (U5) |

C5's "the plane door has one entry point" therefore holds only after U4 and the slit's move (Q6).

## 3. The conic × quadric lane choice

- **The rule (C5).** `bool_conic_quadric_second_harmonic` decides `A₂`. Zero takes the
  first-harmonic core, with `A₂` charged to the extremes' noise. In band and definite take the
  ladder. This is a lane choice (`[verdicts.consumers]`), and it records nothing.
- **The pose follows the lane.**
  - The pose decides `Lies`, `Apart` and `Tangent` only on the Zero lane.
  - On the ladder lane every non-`Through` pose is `Isolated`, and a constant or tangent residual
    there is a subdivision window.
  - Circle × sphere (`A₂ ≡ 0` by kind) takes the Zero lane with no decision.
  - The tree already chooses this way ("a second harmonic in the band's GAP is not an error: it takes
    the ladder").
- **The check owed** is D3's: "the general lane's certificate does not rest on the answer". The
  ladder must be sound at `A₂` = 0 and in the gap.
  - **By construction it is.** The answer is `certified_subdivision`'s, which decides on `F` itself.
    A piece is clear where `|F(m)|` exceeds `F`'s fall, and monotone where `|F′|` is provably
    positive; a bisected root must read ON the surface. A constant `F` in band never clears and is
    `Uncertain`.
  - **The measurement is not enough.** `the-half-angle-ladder-certifies-in-band-configurations`
    (closed, PR 4600) found no wrong definite answer, but only at tilts the switch sent to the ladder
    (`A₂` definite). Near-square circles took the first-harmonic arm. Nothing has measured the
    ladder at `A₂` in the gap or exactly 0.
  - **U3 owes a fuzz row** through a test-only switch that forces the ladder lane, judged against
    `conic_oracle::exact`: never a definite answer the oracle contradicts, with `Uncertain` allowed.
    It covers:
    - circles within the gap of square to a wall, coaxial ones included;
    - near-circular ellipses on a sphere;
    - a circle exactly square to a wall at depths across the band;
    - a cone's `F`.
  - **Its mutant** is `certified_subdivision` counting an undecided split as clear (the
    `in_band_sweep` mutant).
- **If the row is red**, the ratified lane choice is unsound. That is a blocker for an `[ev]` PR, not
  the implementer's to settle.

## 4. The consumers

**4.1 `boolean::trace`.**
- **`ray_crossings`.**
  - The span is `(ray.start(), ray.from_start(reach))`.
  - Every kind goes through `cross`, the plane included: its inline arm goes.
  - `Nurbs`/`Approx` stay `Unsupported`, read from a `Pose(Unbuilt|Enclosed)` window. A spline trace is
    not U3's.
  - `Hit::End(Start)` is skipped.
- **`circle_crossings`.**
  - The span is `(0, τ)`, and both end hits are skipped ("strictly past the start, at most a whole turn").
  - Its plane arm, the torus-vs-`_ => conic_quadric_roots` match and the outer `_ =>` all go.
- **The mapping.**
  - `Lies` is `OnCarrier`.
  - `Tangent`, a `Touch` meeting and a two-in-band window are `Grazing`.
  - `Through` is `Apex`, replacing `crossing`'s `TRACE_CROSSING_APEX`.
  - `Pose(InBand)` is `Escalated`; `Pose(Torus)` is `Torus`.
  - `Method` goes through `MeetFault::kind`: `Escalated` is `Escalated`, `Touch` is `Grazing`,
    `Unread` is `Unsettled`, and `Fault` is a defect panic.
- **Retired:** `door`, `door_refusal`, `line_door_refusal`, `roots_of`, `unsettled` and their
  `unreachable!`s; the rows `TRACE_ON_CARRIER`, `_PLANE_DEPTH`, `_CROSSING_AHEAD`, `_CROSSING_APEX`
  and `_CROSSING_ORDER`.
- **Kept:** `TRACE_CROSSING_SENSE` (a face reading), and `plane_misses` with `TRACE_PLANE_LEAD`
  (plane × surface, not this question).

**4.2 Census touch confirm (`census::point::edge_face`).**
- **What changes.** The kind match and its `_ =>` refusal go. It reads `cross` on the edge's span
  (`curved::carrier` and its params) against any face surface. The private `Surface` enum stays for
  `face_face` alone.
- **The mapping.**
  - A `Tangent` touch on `Hit::On` is a candidate `Met`, checked by `in_both` as today.
  - `Apart`, and `Isolated` with crossings only, are `Stale`: "or cross", as the module docs say.
  - A touch on `Hit::End` is `Stale`, as `on_curved_interior` rules it today.
  - `Lies` keeps today's typed refusal, and `Through` refuses typed.
  - A window becomes `CensusUnsupported` or `CensusEscalated` by its why.
- **Retired:** `circle_plane`, `Locus`, `plane_across`, and `edge_face`'s line × sphere arm.
  `on_sphere` stays for `face_face`.
- **Q5:** whether `pm_census_point_kappa` survives.

**4.3 `ring_path::plane_roots`.**
- **What changes.** It reads `cross` on `Span::new(p.carrier(), p.span)` against the section's plane,
  as `OnSurface::across` reads its plane.
- **The mapping.**
  - `Isolated` meetings are the roots, each slope `|n̂·τ̂|` read at its point.
  - The arrival pick stays, a raw comparison.
  - `Tangent` and pose windows are `None`, as `split_ring_path_meets_plane` Zero is today.
- **Retired:** that row and the hand-rolled harmonic.
- **Blocking question:** today's decision charges `slack`, the stored terms' amplification (`charge`,
  `slack_of`), and the plane door charges nothing of the kind (Q7).

**4.4 `splitting::classify`.**
- **What changes.** Both `plane_crossing_lane` reads go through `cross`; one is in the tests.
- **The mapping.**
  - `Lies` and `Apart` mean `continue`.
  - Each `Hit::On` meeting, crossing or touch, splits ascending; the graze net inserts the graze root
    as today.
  - End hits are skipped, since the vertex sweep owns them.
  - A window is `SplitReduceError::CrossingEscalated`, whose `fault` changes from `ConicRootFault` to
    `CrossWhy`. That is a public type, re-exported in `topo::lib`. The lever split is kept: a pose rung
    is coincidence, a method fault is crossing.
- **The `Unlaned` box gate** (`edge_clears`) goes. A spline edge is read by `composite`, a gain; a
  spiric edge refuses on its window.
- **Q8:** the line arm.

**4.5 `curve_pair::on_surface`.**
- **What changes.** `OnSurface::across` and `::under` read `cross` against a `Surface::Plane` built
  from `curve.plane()`.
- **The mapping.**
  - `InPlane` is `Lies`, and `Touch` is `Tangent`.
  - `Roots` carries `Meeting`s instead of `(SpanRoot, Point3)`.
  - `RootPlace::Inside`/`AtEnd` read as `Hit::On`/`Hit::End`, and `Undecided` as a window.
- **The callers** (`region`, `splitting::containment` and `boolean::ops`' sphere cut-in) move to `Hit`
  and windows.
- **The `row` parameter goes.** It decided the parallel offset (`"bool_sphere_cut_in_plane"`,
  `D::COMPONENT`); the pose owns the offset now, so those K names change.
- **`CoSurface::Unlaned`.** For a spline edge, `cross` now answers through `composite`. For a spiric
  edge it is the unbuilt window. `region`'s "Unlaned, then `under`" is a second reading reached because
  the first declined (Q9).

## 5. Retired in this PR

Each is deleted with its last reader.
- **Retired.**
  - `plane_crossing::span_plane_meet`, `SpanPlaneMeet`, `SpanEnds` and `SpanRoot`.
  - `CoSurface::Unlaned`, if Q9 allows.
  - `LineRoots::Constant`.
  - `CircleRoots::On` (`Roots::On` out of `first_harmonic_roots`, `constant_residual_roots` and
    `circle_torus_roots`).
  - `Unsettled::Tangency` from the first-harmonic and wall arms.
  - `Unsettled::AtApex` and `Degenerate`-on-a-generator from the line arm.
  - Trace's and the census's items (§4).
- **Kept, produced by wrappers from `pose` (§2.7).** Each names its last reader in a doc line:
  - `ConicPlaneMeet::Parallel`;
  - `ConicRootFault::{PlaneParallel, BellyGraze}`;
  - `WallRoots::{AxisParallel, Tangent}`;
  - `SpanVerdict::{Constant, LiesOn, AtApex}`;
  - `RootPlace` and `span_place` (`torus_region` reads them).

## 6. Rows owed

The row rules are in `docs/prompts/implementer-discipline.md` §8.

- **A pose in band escalates.** One `cross` row per class and arm, answering `PoseRefusal::InBand`
  with its rung. Each also pins the lever its consumer renders: coincidence for a pose rung, crossing
  for a method fault.
  - *Lies*:
    - a circle in a plane at an offset in the gap;
    - a wall ruling at an offset in the gap;
    - a coaxial circle on a wall or sphere, and a torus rim circle, at a constant residual in the gap;
    - a torus meridian in the gap;
    - a cone generator at a lead in the gap.
  - *Apart and Tangent*:
    - a plane belly in the gap;
    - a wall, sphere or cone discriminant in the gap;
    - a first-harmonic extreme with an end in the gap;
    - `a_graze_side_decided_zero_keeps_its_margin`, re-aimed.
  - *Through*: a line's apex foot, and `bool_conic_cone_apex`, each in the gap.
  - *Isolated*: `A₂` in the gap takes the ladder, unrefused (§3).
- **A mutant per retired self-decided pose.** Each mutant restores the door's own decision or removes
  the pose stage, and its row reds.

  | Mutant | The row that reds |
  |---|---|
  | The pose skips `PlaneParallel` | a circle in its plane answers `Lies`, not a 0/0 phase in `span_roots` |
  | The belly's Zero is read as two roots | a graze answers one `Touch` |
  | An axis-parallel line answers `Isolated` | a ruling is `Lies`, a parallel line off the wall `Apart`, through both `cross` and the wrapper |
  | The coaxial stage is deleted | a rim circle of a wall, a sphere and a torus is `Lies`, not a ladder window |
  | The apex stage always reads Positive | a line and a circle through the apex answer `Through` |
  | `edge_face`'s kind match is restored | the census serves a circle × sphere touch |

  A static guard backs them: no `curve_face_pose_*` row is decided outside `curve_face.rs`. It extends
  `every_door_name_carries_a_listed_prefix`.
- **The census serves every pair `cross` serves.** A table row over every built
  `(CurveKind, SurfaceKind)` of `route`:
  - a touch reads `Met`;
  - a crossing reads `Stale`;
  - `Apart` reads `Stale`;
  - an unbuilt pair refuses typed.

  It reds under the kind-match mutant.
- **Wrapper parity.** The sweep's existing pose rows answer unchanged through the wrappers, so U3 is
  behaviour-free for U4's lane. The ONEDOOR and `in_band_sweep` rows run unchanged.
- **Windows.** One row per method window kind: the window is placed, and a meeting outside it is still
  reported (`window_fuzz` is the template).

## 7. Out of scope

- **U4:** the sweep's lanes, `carrier_touch::off_face`, the culls (`conic_clearance`,
  `bool_line_cylinder_clearance`), and the face reading's window rule.
- **U5:** the ray lane (`solid_contain::cast_ray`, `bool_ray_cone_*`).
- **U1b:** one line × cone rule set.
- **Not this unit's:**
  - `slit` and `factored_crossing`, which the designers' staging put in U3 (Q6);
  - `ops::apply_cut_ins`, `sphere_region::ray_roots` and `pcurve_cache` incidence.

## 8. Open questions (each decides something this spec does not)

- **Q1 — `Hit::On`'s margin.** `Hit::On { t, margin: MarginDiag }` carries an on-carrier reading's
  margin. A root a door places is on the carrier by construction and has no such reading. The options:
  - make the margin optional;
  - carry `Span::place_param_read`'s decision;
  - use `curve_pair::Place` for `on_span`.
- **Q2 — The apex meeting.** Is the singular point a meeting, and of which kind? C5 names three kinds:
  crossing, touch and end hit. Today the conic door answers the apex as a double root, which the sweep
  places on the apex vertex; the trace refuses it.
- **Q3 — The axis-parallel line's constant residual.** Its sign moves into the pose, which C5 implies.
  Today each consumer decides it (trace: `on_carrier` at the pen; the sweep: `SpanVerdict::Constant`).
  Which spread does it charge?
- **Q4 — The torus parallel-axes tangency.** Does its in-band contour reach move to the pose as
  `Tangent`, or stay a method window?
- **Q5 — The census's `κ_rel` and touch-point accuracy.**
  - The pose's plane arm reads `Conic::along` (through `curve_pair::pose::height`), which forms
    `û·n̂, v̂·n̂` as plain dot products. That is the form `plane_across` documents as `r·u/sin` off:
    1.8e-6 at 3e-11 rad.
  - `a_circle_nearly_parallel_to_a_plane_is_touched_where_it_touches` then reds, although that point
    is inside its in-band arc (≈0.26 rad).
  - The options: the census reads the arc, or `along` adopts `plane_across`'s form.
  - Separately: does `pm_census_point_kappa` survive as a kind-specific second-order check, or does
    `Tangent`'s arc stand for it?
- **Q6 — `slit` and `factored_crossing`.** Do they join U3, which is what makes the plane door's single
  entry true before U4, or U4?
- **Q7 — Ring path's `slack`.** Does `cross` charge the stored-term amplification that ring path
  charges?
  - If it does, every consumer's verdicts move.
  - If it does not, ring path loses a charge it documents as necessary.
  - Blocks §4.3.
- **Q8 — The split's line root.** `splitting::classify`'s line root is interpolated from vertex sides
  (`PlaneSide`), and `cross` reads the line's closed form. These are two predicates for one fact.
  - The split reads `cross`, and a mismatch with the sides is a fault; or
  - the sides-derived root stays as the split's own question.
- **Q9 — `region`'s fallback.** Is `across`-then-`under` retired in U3, or kept for spirics?
- **Q10 — Splitting U3.** U3 is X-sized. One cut:
  - U3a: the types, `pose`, `cross`, the plane and line arms, trace and on_surface;
  - U3b: the conic arms, the census, classify and ring path.

  The orchestrator's sequencing call.

## 9. Review tier

**Dual** (one Opus, one Fable), for these reasons:
- The unit fixes the answer and refusal types that U4, U5 and the face reading build on.
- It moves self-decided poses out of every closed door at once, which changes what `Tangency`, `On`
  and `AtApex` mean to the readers that remain.
- It changes a public error type (`SplitReduceError::CrossingEscalated`).
- Its §3 check can become a ratification question.

A wrong shape here is expensive to change after U4 builds on it.
