# GERM crossing U3 — `curve_face::pose`, `curve_face::cross`, and the small consumers (unit spec)
This spec covers U3 of `work/germ/curve-surface-crossing-has-no-kind-pair-table.md` (its Ratified
section), building C5's curve × face paragraph as PR 7167 ratified it (`crates/geom-brep/README.md`). U3
is one PR. It builds `curve_face::pose` and the one crossing reading `curve_face::cross`, dispatched
through `curve_face::route`, with one answer type and one refusal carrying meetings found plus undecided
windows. It also moves five small consumers onto them. The sweep (U4) and the ray lane's remaining reads (U5)
are out of scope. Until U4 the sweep may call a door's method directly (one method, two callers). This
spec was read on `c093993bb2`, and U1 on the `germ/line-roots-door` lane's uncommitted tree. It cites names, not lines, and
is deleted when U3 merges.

## 0. What U3 needs from U1 and U2
**U1** delivers `curve_face::line_quadric` (`line_quadric_roots`, `LineRoots::{Roots, Constant}`,
`LineQuadricRefusal`, `WallRoots`, `About`, the per-kind finders, the private `line_cone_roots` and
`apex_root_spread`). It also deletes `reduce::line_wall_roots_of`, and points trace at
`line_quadric_roots`; there it removes `ray_crossings`' `Some(Err(_)) => unreachable!` and adds
`line_door_refusal`. U3 needs the axis-parallel, cone apex, lead and spread rungs callable on their own,
so that `pose` and the old entries share one predicate per row. U1b (one line × cone rule set) is not a
prerequisite, but it must not be in flight with U3: both rewrite the apex rungs.

**U2** delivers `curve_face::conic_quadric::conic_quadric_roots`, which refuses `DoorRefusal` rather
than `BooleanError`. Beside it go `first_harmonic_arm`, `quadric_ladder`, `cone_roots`,
`through_the_apex` and `off_the_apex`, and its rows move under a `DOOR_K_PREFIXES` prefix. U3 must not be
in flight with the first-harmonic arm's Part 2 (`germ-rounds` P/Q). Both rewrite `first_harmonic_roots`'
decide and count, and U3 takes the coaxial and extreme rungs out of it.

## 1. The types
**`curve_face::Pose<T>`** is new. `curve_pair::pose::Pose` is the curve-pair pose, and it stays.
- `Lies`: holds over the whole span; it reads values only.
- `Apart`.
- `Tangent(Touch<T>)`: `Isolated`'s pose-decided special case. `Touch { at: T, point, arc: (T, T) }`
  is a point of the in-band arc, plus the arc, in span parameters.
- `Through(Singular<T>)`: `Singular { at, point, spread: (T, T) }`. In U3 only a cone's apex reaches
  it, because the spindle route is unbuilt.
- `Isolated(Form<T>)`: `Form` is private and per arm. It holds the harmonics, quadratic or candidates
  the pose computed, and the method takes them without re-deciding anything.

**`PoseRefusal`** is `InBand { rung: PoseRung, diag } | Enclosed | Unbuilt(Route) | Torus(TorusRefusal) |
Fault(&'static str)`. `Enclosed` is a spline arm, which has no closed pose. **`PoseRung`** is
`PlaneParallel | BellyGraze | AxisParallel | Coaxial | Extreme | Apex | ConeLead | Meridian`. It takes
over `ConicRootFault::decision() == None`. A consumer maps a pose rung to its coincidence lever (today
`BooleanDecision::of_conic_root` gives `Coincide::EdgeOnPlane`), and a method fault to its crossing
decision.

**The meeting.** The curve-pair vocabulary has no one-span meeting. `curve_pair::Meeting { point, on_a,
on_b }` is two-sided and does not tell crossing from touch, `MeetRefusal { fault, found, shared }` gives
the refusal's shape, and the nearest one-span type is `curve_face::BoundaryMeeting { point, on_span, .. }`.
So U3 mints only `curve_face::Meeting<T> { point, on_span: Hit<T>, kind: Kind<T> }`, with
`Kind::{Cross, Touch { arc }}`. It reuses `curve_pair::Hit` for the place (`Hit::End` is C5's end hit),
and `MeetFault` for every fault. `Hit::On`'s `margin: MarginDiag` does not fit a root placed by
parameter (Q1).

**The answer** is `Crossings<T> { pose: Pose<T>, meetings: Vec<Meeting<T>> }`. Meetings ascend along
the span, decided apart by their chord, and two in band of each other are a window. `Lies` and `Apart`
hold no meetings, `Tangent` holds its one touch, and `Through` holds the rest (Q2 asks about the
singular point itself).

**The refusal** is `CrossRefusal<T> { found: Vec<Meeting<T>>, windows: Vec<Undecided<T, CrossWhy>> }`:
shaped like `MeetRefusal` (meetings decided before it, never the complete set), read like
`curve_face::Located` (exact outside the windows, silent inside). The window type is
`curve_face::Undecided`, made generic in its why (`Undecided<T, W = BoundaryRefusal>`), so the boundary
pass is unchanged. `Located::undecided_over` lifts to a function over `&[Undecided<T, W>]`, so the face
reading (U4) reads both kinds of window alike; no `Located` twin is minted. `CrossWhy` is
`Pose(PoseRefusal) | Method { door: Door, fault: MeetFault }`. A pose refusal, an unbuilt route included,
is one whole-span window, so no arm refuses whole by kind.

**Entry points** (`curve_face.rs`): `pose(span: &Span<'_, T>, surface: &Surface<T>, band) ->
Result<Pose<T>, PoseRefusal>`, and `cross(span, surface, band) -> Result<Crossings<T>, CrossRefusal<T>>`.
`cross` matches `route(..).door` exhaustively, with no wildcard: `pose` first, then the door's one method.
A wrong kind at an arm is `MeetFault::Desync`. `Span::endless` replaces `plane_crossing::SpanEnds`.

## 2. Per arm (`route`'s `Door`)
Each rung below is verified against the tree. Each moves out of its door into `pose` as a
`curve_face_pose_*` row (`TABLE_K_PREFIX`: the deciding module names the row), and the door keeps only
its method. An old entry that an unmoved caller still reads (§2.7) becomes a wrapper: `pose`, then the
method, mapped back to the old answer. A wrapper decides nothing, and it goes with its last reader.

**2.1 `PlaneCrossing`** (`plane_crossing.rs`).

| | |
|---|---|
| Pose | `conic_plane_candidates`' `plane_crossing_belly_graze`: Negative is `Apart`, Zero is `Tangent`, in band is `BellyGraze`. `_plane_parallel` Zero with the offset Zero is `Lies`, else `Apart`; the offset decision is new to the door (today the caller decides `Parallel { offset }`), and in band is `PlaneParallel`. `_graze_side` in band is `BellyGraze`. The line arm's `_line_parallel` (today in `span_plane_meet`). |
| Tangent's arc | `graze_reaches_span`'s `δ = acos(1 − z/r)` about the extremum. |
| Method | `span_roots` (each root placed by `Span::place_param`; `_roots_apart`, `_root_order`), plus a line's closed-form root. |
| Windows | A root in band of an end (`RootPlace::Undecided`): from the end to the root, widened by the escalation at the span's least speed. Two roots in band of each other. |
| Apart | The belly; no cull. |

**2.2 `LineRoots`** (U1's `line_quadric.rs`).

| | |
|---|---|
| Cylinder | `line_quadric_cylinder_axis_parallel` Zero is a constant residual. The pose decides it `Lies` or `Apart` as `constant_residual_roots` does; today `LineRoots::Constant` leaves this to the consumer (Q3). `_cylinder_disc` Zero is `Tangent` (today `WallRoots::Tangent`, refused as `Unsettled::Tangency`), Negative is `Apart`. |
| Sphere | Its discriminant, likewise. |
| Cone | `line_quadric_cone_apex` Zero with `_cone_lead` Zero is `Lies`, a generator (today `Unsettled::Degenerate`). Apex Zero with the lead definite is `Through`, its spread `_cone_apex_spread`; the in-band case is today's `Unsettled::AtApex`. The lead Zero off the apex stays the method's `Degenerate`, a whole-span window, owned by `a-line-parallel-to-a-cone-generator-has-no-root-lane`. |
| Torus | No closed pose; `Isolated` once the ring convention passes. |
| Tangent's arc | New: `±√(z/a)` about the double root, in `Centred`'s parameter. |
| Method | `Centred::roots` with the root slacks, and `line_torus_roots`. The method meeting `AxisParallel` after the pose said otherwise is `MeetFault::Contradiction`. |
| Windows | An `Unsettled` count is the whole span (no closed form has a local spread today). A refused slack is the root ± its slack. |
| Apart | The discriminant. `bool_line_cylinder_clearance` stays in the sweep until U4. |

**2.3 `ConicQuadric`** (U2's module).

| | |
|---|---|
| Zero lane (§3) | From `first_harmonic_roots_in_band`: the noise rung. `coaxial`, through `constant_residual_roots`: `CircleRoots::On` becomes `Lies`, a Miss becomes `Apart`. `extreme`: one-signed is `Apart`; an extreme in the zero band is `Tangent` (today `Unsettled::Tangency`), its arc `2·asin(√(z/(hi − lo)))` about the near extreme. Today's "escalated coaxial test falls through to the extremes" stays only as a stage of one rule (D3): the extremes decide the same class, and the PR body states the proof. |
| Cone | `through_the_apex`'s `bool_conic_cone_apex` Zero is `Through`, its spread `_apex_spread`. `off_the_apex`'s per-root `AtApex` becomes the `Apex` rung in band. `bool_conic_cone_on_surface` gives `Lies`. |
| Method | The Zero lane: the core's root slack and placement. Otherwise `door_half_angle_roots`: the ladder for its escalations, `certified_subdivision` for the answer. Through the apex: the deflated `G` roots. |
| Windows | The subdivision's undecided pieces, threaded out of `crate::half_angle` (new: today it refuses whole on `Floor` or `Tangency`). A refused slack is the root ± its slack. |
| Apart | The extremes on the Zero lane; nothing closed on the ladder lane. In U4 the sweep's `conic_clearance` moves into `pose` as one match on surface kind (`implicit::conic_residual_extremes` is harmonic-else-sampled today). |

**2.4 `ConicTorus`** (`circle_torus.rs`, `ellipse_torus.rs`).

| | |
|---|---|
| Pose | `circle_torus_coaxial_tilt`, `_offset`, `_residual`: `Lies` (a rim circle) or `Apart`. `circle_torus_meridian`: `Lies`. `DoorRefusal::Torus` becomes `PoseRefusal::Torus`, read before any pose as today. |
| Method | The parallel-axes arm's contour reach stays here; it also reads an in-band tangency (Q4). |
| Special cases | A Villarceau circle has no rung: it is `Isolated`, and the ladder's `Unsettled` makes a whole-span window, as today. Ellipse × torus has no special pose; it answers only `Certified`, `Miss` or `Unsettled`. |
| Windows | As in §2.3. |
| Apart | Nothing closed. The torus clearance stays in the sweep until U4. |

**2.5 Spline arms.** `SplineComposite` (`composite::crossings`) and `SplineFace` (`spline_face::settle`):
pose `Enclosed`, enclosures the method; windows are `composite`'s in-band pieces and `spline_face`'s floor
boxes projected on `t`; a spline run in band is a window (Ev, PR 7167). **2.6 Unbuilt routes** (`Spiric`,
`Sheet`, `Periodic`, spiric × spline): `PoseRefusal::Unbuilt(route)`, one whole-span window.

**2.7 Wrappers, kept for their remaining readers.**

| Wrapper | Remaining readers |
|---|---|
| `plane_crossing_lane` | the sweep (`reduce`, U4), `slit`, `factored_crossing`, `offer_rows` |
| `line_quadric_roots` | the sweep |
| `conic_quadric_roots`, `circle_torus_roots`, `ellipse_torus_roots` | `reduce::wall_crossing` |
| `line_wall_roots`, `line_cone_quadratic`, `WallRoots` | the ray lane (U5) |

C5's "the plane door has one entry point" therefore holds only after U4 and the slit's move (Q6).

## 3. The conic × quadric lane choice
C5's rule: `bool_conic_quadric_second_harmonic` decides `A₂`. Zero takes the first-harmonic core, with
`A₂` charged to the extremes' noise; in band and definite take the ladder. That is a lane choice
(`[verdicts.consumers]`), and it records nothing. The tree already chooses this way: "a second harmonic
in the band's GAP … takes the ladder".

The pose decides `Lies`, `Apart` and `Tangent` only on the Zero lane. On the ladder lane every pose but
`Through` is `Isolated`, and a constant or tangent residual is a subdivision window. Circle × sphere
(`A₂ ≡ 0` by kind) takes the Zero lane with no decision.

**The check owed** (D3: "the general lane's certificate does not rest on the answer"): the ladder must be
sound at `A₂` = 0 and with `A₂` in the gap. **By construction it is:** the answer is
`certified_subdivision`'s, which decides on `F` itself (clear where `|F(m)|` exceeds its fall, monotone
where `|F′|` is provably positive, a bisected root read ON the surface; a constant `F` in band never
clears and is `Uncertain`). **Unmeasured there:** `the-half-angle-ladder-certifies-in-band-configurations`
(closed, PR 4600) found no wrong definite answer, but only at tilts the switch sent to the ladder (`A₂`
definite); near-square circles took the first-harmonic arm. **Owed:** a fuzz row forcing the ladder lane
through a test-only switch on circles within the gap of square to a wall (coaxial included),
near-circular ellipses on a sphere, a circle exactly square to a wall at depths across the band, and a
cone's `F`, judged against `conic_oracle::exact`: no definite answer the oracle contradicts (`Uncertain`
allowed). Its mutant is `in_band_sweep`'s (an undecided split counted clear). **If it is red**, the
ratified lane choice is unsound: a blocker for an `[ev]` PR, not the implementer's to settle.

## 4. The consumers
**4.1 `boolean::trace`.**
- `ray_crossings` reads `cross` on `(ray.start(), ray.from_start(reach))` for every kind, the plane
  included (its inline arm goes), and skips `Hit::End(Start)`. `Nurbs` and `Approx` stay `Unsupported`,
  read off a `Pose(Unbuilt|Enclosed)` window.
- `circle_crossings` reads `(0, τ)` and skips both end hits ("strictly past the start, at most a whole
  turn"). Its plane arm, its torus-vs-`_ => conic_quadric_roots` match, and its outer `_ =>` go.

| Answer | Trace result |
|---|---|
| `Lies` | `OnCarrier` |
| `Tangent`, a `Touch` meeting, or a two-in-band window | `Grazing` |
| `Through` | `Apex` (replaces `crossing`'s `TRACE_CROSSING_APEX`) |
| `Pose(InBand)` / `Pose(Torus)` | `Escalated` / `Torus` |
| `Method` | by `MeetFault::kind`: `Escalated` to `Escalated`, `Touch` to `Grazing`, `Unread` to `Unsettled`, `Fault` to a defect panic |

Retired: `door`, `door_refusal`, `line_door_refusal`, `roots_of`, `unsettled` and their
`unreachable!`s, and the rows `TRACE_ON_CARRIER`, `_PLANE_DEPTH`, `_CROSSING_AHEAD`, `_CROSSING_APEX` and
`_CROSSING_ORDER`. Kept: `TRACE_CROSSING_SENSE` (a face reading), and `plane_misses` with
`TRACE_PLANE_LEAD` (another question).

**4.2 The census touch confirm (`census::point::edge_face`).** The kind match and its `_ =>` refusal go.
It reads `cross` on the edge's span against any face surface; the private `Surface` enum stays for
`face_face`.

| Answer | Census result |
|---|---|
| `Tangent` touch on `Hit::On` | candidate `Met`, checked by `in_both` |
| `Apart`, or `Isolated` with crossings only | `Stale` (today's "or cross") |
| Touch on `Hit::End` | `Stale`, as `on_curved_interior` rules today |
| `Lies` | today's typed refusal |
| `Through` | typed refusal |
| Window | `CensusUnsupported` or `CensusEscalated` |

Retired: `circle_plane`, `Locus`, `plane_across`, and the line × sphere arm. `on_sphere` stays for
`face_face`. `pm_census_point_kappa` is Q5.

**4.3 `ring_path::plane_roots`** reads `cross` on `Span::new(p.carrier(), p.span)` against the section's
plane, as `OnSurface::across` does. `Isolated` meetings are the roots, with the slope `|n̂·τ̂|` read at
each point. The arrival pick stays a raw comparison. `Tangent` and pose windows give `None`, as
`split_ring_path_meets_plane` Zero does today. That row and the hand-rolled harmonic retire. **Blocking
(Q7):** today the decision charges `slack`, the stored terms' amplification (`charge`, `slack_of`), and
the plane door charges nothing of the kind.

**4.4 `splitting::classify`.** Both `plane_crossing_lane` reads (one in tests) go through `cross`.
`Lies` and `Apart` mean `continue`. Each `Hit::On` meeting, crossing or touch, splits ascending (the
graze net inserts the graze root, as today); end hits are skipped. A window is
`SplitReduceError::CrossingEscalated`, its `fault` now `CrossWhy` (was `ConicRootFault`; a public type,
re-exported in `topo::lib`), keeping the lever split: pose rung = coincidence, method = crossing. The
`Unlaned` box gate (`edge_clears`) goes: a spline edge is read through `composite` (a gain), a spiric edge
refuses on its window. The line arm is Q8.

**4.5 `curve_pair::on_surface`.** `OnSurface::across` and `::under` read `cross` against a
`Surface::Plane` built from `curve.plane()`. `InPlane` is `Lies`, `Touch` is `Tangent`, and `Roots`
carries `Meeting`s in place of `(SpanRoot, Point3)` (`RootPlace::Inside`/`AtEnd`/`Undecided` read as
`Hit::On`/`Hit::End`/a window). The callers (`region`, `splitting::containment`, `boolean::ops`' sphere
cut-in) move to `Hit` and windows. The `row` parameter that decided the parallel offset goes, so the K
names `"bool_sphere_cut_in_plane"` and `D::COMPONENT` change. For a spline edge `CoSurface::Unlaned` is
now `composite`'s answer, and for a spiric edge the unbuilt window. `region`'s "Unlaned, then `under`" is
a second reading reached because the first declined (Q9).

## 5. Retired in this PR (each with its last reader)
**Retired:** `plane_crossing::span_plane_meet`, `SpanPlaneMeet`, `SpanEnds`, `SpanRoot`;
`CoSurface::Unlaned` (if Q9 allows); `LineRoots::Constant`; `CircleRoots::On` (out of
`first_harmonic_roots`, `constant_residual_roots`, `circle_torus_roots`); `Unsettled::Tangency` from the
first-harmonic and wall arms; `Unsettled::AtApex` and `Degenerate`-on-a-generator from the line arm; §4's items.

**Kept until §2.7's wrappers go.** The wrappers produce these from `pose`, and each names its last reader
in a doc line: `ConicPlaneMeet::Parallel`, `ConicRootFault::{PlaneParallel, BellyGraze}`,
`WallRoots::{AxisParallel, Tangent}`, `SpanVerdict::{Constant, LiesOn, AtApex}`, and `RootPlace` and
`span_place` (for `torus_region`).

## 6. Rows owed (rules: `docs/prompts/implementer-discipline.md` §8)
**An in-band pose escalates.** One `cross` row per class and arm. Each answers `PoseRefusal::InBand` with
its rung, and pins its consumer's lever: coincidence for a pose rung, crossing for a method. All are in
the gap:

| Class | Cases |
|---|---|
| *Lies* | a circle in a plane at an offset; a wall ruling at an offset; a coaxial circle on a wall or sphere, or a torus rim circle, at a constant residual; a torus meridian; a cone generator at a lead |
| *Apart / Tangent* | a plane belly; a wall, sphere or cone discriminant; a first-harmonic extreme with an end in the gap; `a_graze_side_decided_zero_keeps_its_margin`, re-aimed |
| *Through* | a line's apex foot; `bool_conic_cone_apex` |
| *Isolated* | `A₂` in the gap takes the ladder, unrefused (§3) |
**A mutant per retired pose that a door decided itself**, and the row it reds:

| Mutant | Row that reds |
|---|---|
| The pose skips `PlaneParallel` | a circle in its plane answers `Lies`, not a 0/0 phase in `span_roots` |
| The belly's Zero is read as two roots | a graze answers one `Touch` |
| An axis-parallel line is `Isolated` | a ruling is `Lies`, and a parallel line off the wall is `Apart`, through both `cross` and the wrapper |
| The coaxial stage is deleted | the rim circles of a wall, a sphere and a torus are `Lies`, not ladder windows |
| The apex stage reads Positive always | a line and a circle through the apex are `Through` |
| `edge_face`'s kind match is restored | the census serves a circle × sphere touch |

Also owed: `every_door_name_carries_a_listed_prefix` extended so no `curve_face_pose_*` row is decided
outside `curve_face.rs`.

**The census serves every pair `cross` serves.** A table over every built `(CurveKind, SurfaceKind)` in
`route`: a touch reads `Met`, a crossing `Stale`, `Apart` `Stale`, and an unbuilt pair refuses typed.
It reds under the kind-match mutant.

**Wrapper parity:** the sweep's existing pose rows (and ONEDOOR, `in_band_sweep`) answer unchanged
through the wrappers. **Windows:** one row per method window kind, placed, with a meeting outside it
still reported (`window_fuzz` the template).

## 7. Out of scope
U4: the sweep's lanes, `carrier_touch::off_face`, the culls (`conic_clearance`,
`bool_line_cylinder_clearance`) and the face reading's window rule. U5: the ray lane
(`solid_contain::cast_ray`, `bool_ray_cone_*`). U1b: line × cone. Unassigned: `slit` and
`factored_crossing` (the designers' U3; Q6). Not this question: `ops::apply_cut_ins`,
`sphere_region::ray_roots`, `pcurve_cache` incidence.

## 8. Open questions (each decides something this spec does not)
- **Q1.** `Hit::On { t, margin: MarginDiag }` carries an on-carrier reading's margin, and a root a door
  places has none. Options: an optional margin, `Span::place_param_read`'s decision, or
  `curve_pair::Place` for `on_span`.
- **Q2.** Is the singular point a meeting, and of what kind (C5 names crossing, touch, end hit)? Today
  the conic door answers the apex as a double root the sweep places on the apex vertex; trace refuses it.
- **Q3.** The axis-parallel line's constant-residual sign moves into the pose (C5); today each consumer
  decides it (trace: `on_carrier`; the sweep: `SpanVerdict::Constant`). Which spread does it charge?
- **Q4.** Does the torus parallel-axes contour reach in band move to the pose as `Tangent`, or stay?
- **Q5.** Does the census's `pm_census_point_kappa` survive, or does `Tangent`'s arc stand for it?
  Separately, the touch point's accuracy: the plane pose reads `Conic::along` (via
  `curve_pair::pose::height`), plain dot products `û·n̂, v̂·n̂`. That is the form `plane_across`
  documents as `r·u/sin` off, 1.8e-6 at 3e-11 rad. So
  `a_circle_nearly_parallel_to_a_plane_is_touched_where_it_touches` reds, even though the point lies
  inside its in-band arc (≈0.26 rad). Either the census reads the arc, or `along` takes `plane_across`'s
  form.
- **Q6.** Do `slit` and `factored_crossing` join U3 (so the plane door has one entry before U4) or U4?
- **Q7.** Does `cross` charge the stored-term amplification that ring path charges? If it does, every
  consumer's verdicts move; if not, ring path loses a charge it documents as needed. This blocks §4.3.
- **Q8.** `splitting::classify` interpolates its line root from vertex sides (`PlaneSide`), while `cross`
  reads the line's closed form: two predicates for one fact. Should it read `cross` and treat a side
  mismatch as a fault, or keep the sides-derived root as the split's own question?
- **Q9.** Is `region`'s across-then-`under` retired in U3, or kept for spirics? **Q10.** U3 is X-sized: split it into U3a (types, `pose`, `cross`, plane and line arms, trace,
  on_surface) and U3b (conic arms, census, classify, ring path)? The orchestrator's sequencing call.

## 9. Review tier
**Dual** (one Opus, one Fable). The unit fixes the answer and refusal types that U4, U5 and the face
reading build on. It moves self-decided poses out of every closed door at once, which changes what
`Tangency`, `On` and `AtApex` mean to the readers that remain. It changes a public error type, and §3 can
turn into a ratification question. A wrong shape is expensive to change once U4 builds on it.
