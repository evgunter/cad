# GERM crossing U3 — `curve_face::pose`, `curve_face::cross`, and the small consumers (unit spec)
This spec carries U3 of `work/germ/curve-surface-crossing-has-no-kind-pair-table.md` (its Ratified
section), building C5's curve × face paragraph as PR 7167 ratified it (`crates/geom-brep/README.md`). U3
lands as two PRs.
- **U3a** builds the types, `curve_face::pose` and `curve_face::cross` (dispatched through
  `curve_face::route`), and moves `boolean::trace` and the census touch confirm onto them.
- **U3b** moves `ring_path::plane_roots`, `splitting::classify`, `curve_pair::on_surface` and `region`.

The sweep, `slit` and `factored_crossing` are U4's, and the ray lane's remaining reads are U5's. Until U4
the sweep reads the doors through wrappers (§a.4). The questions this spec first left open were answered by two designers. The GERM orchestrator reconciled
their answers on 2026-11-20, and they are stated below as the spec. Read on `c093993bb2`, with U1 read on
the `germ/line-roots-door` lane's uncommitted tree. Names are cited, not lines. This spec is deleted when
U3b merges.

## 0. Prerequisites
**U1** delivers `curve_face::line_quadric`: `line_quadric_roots`, `LineRoots`, `LineQuadricRefusal`,
`WallRoots`, `About`, the per-kind finders, `line_cone_roots` and `apex_root_spread`. It also deletes
`reduce::line_wall_roots_of` and puts trace on `line_quadric_roots`, which drops `ray_crossings`'
`Some(Err(_)) => unreachable!`. U3a needs the axis-parallel, cone apex, lead and spread rungs callable on
their own, so that the pose and the wrappers share one predicate per row.

**U2** delivers `curve_face::conic_quadric`, refusing `DoorRefusal` (not `BooleanError`). Beside it sit
`first_harmonic_arm`, `quadric_ladder`, `cone_roots`, `through_the_apex` and `off_the_apex`, and its rows
are under a `DOOR_K_PREFIXES` prefix.
Two units must never be in flight with U3a. U1b rewrites the apex rungs, as U3a does. The first-harmonic
arm's Part 2 (`germ-rounds` P/Q) rewrites `first_harmonic_roots`' decide and count, as U3a does.

# U3a — the types, `pose`, `cross`, trace, census (PR 1)
## a.1 The types
**`curve_face::Pose<T>`** is new. `curve_pair::pose::Pose` is a different type and stays.

| Class | Meaning |
|---|---|
| `Lies` | over the whole span; a reading of values only |
| `Apart` | no meeting |
| `Tangent(Touch<T>)` | `Isolated`'s pose-decided special case. `Touch { at, point, arc: (T, T) }` holds the double root and an arc of span parameters that encloses the in-band arc and never understates it (§a.2) |
| `Through(Singular<T>)` | `Singular { at, point, spread: (T, T) }`. In U3 only a cone's apex reaches it, because the spindle route is unbuilt |
| `Isolated(Form<T>)` | `Form` is private per arm: the harmonics, quadratic or candidates the pose computed, which the method takes without re-deciding. Answered only once `Lies` is excluded (§a.2) |

**A singular point is not a meeting.** It is the pose's datum, and consumers match the pose first: the
sweep places it on the apex vertex, the trace answers `Apex`, and the census refuses typed.
**`PoseRefusal`** is `InBand { rung: PoseRung, diag } | Enclosed` (a spline arm, no closed pose) `|
Unbuilt(Route) | Torus(TorusRefusal) | Fault(&'static str)`. `PoseRung` is `PlaneParallel | BellyGraze | AxisParallel | Coaxial | Extreme | Kappa | Apex | ConeLead |
Meridian | NotLies`, plus the torus contour rung if it moves (§a.2). A consumer maps a pose rung to its
coincidence lever (today `BooleanDecision::of_conic_root`'s `Coincide::EdgeOnPlane`), and a method fault
to its crossing decision.

**The meeting** is new and one-sided: `curve_face::Meeting<T> { point, on_span: OnSpan<T>, kind }`, with
`OnSpan<T> = Inside { t } | End(SpanEnd)` (no `Past`, no margin: a root placed by parameter has no
on-carrier reading; `curve_pair::Hit` is not reused) and `kind: Cross | Touch { arc: (T, T) }`.

**The answer** is `Crossings<T> { pose: Pose<T>, meetings: Vec<Meeting<T>> }`, the meetings ascending along
the span and decided apart by their chord (two in band of each other are a window). `Lies` and `Apart`
hold none, `Tangent` its one touch, `Through` the meetings other than the singular point.

**The refusal** is `CrossRefusal<T> { found: Vec<Meeting<T>>, windows: Vec<Undecided<T, CrossWhy>> }`,
shaped like `curve_pair::MeetRefusal` (meetings decided before it, never the complete set) and read like
`curve_face::Located` (exact outside the windows, silent inside). `curve_face::Undecided` becomes generic
in its why (`Undecided<T, W = BoundaryRefusal>`), so the boundary pass is unchanged;
`Located::undecided_over` lifts to a function over `&[Undecided<T, W>]`. `CrossWhy = Pose(PoseRefusal) |
Method { door: Door, fault: MeetFault }`. A pose refusal, an unbuilt route's included, is one whole-span
window.

**Entry points** (`curve_face.rs`): `pose(span: &Span<'_, T>, surface: &Surface<T>, band) ->
Result<Pose<T>, PoseRefusal>` and `cross(span, surface, band) -> Result<Crossings<T>, CrossRefusal<T>>`.
`cross` matches `route(..).door` exhaustively, with no wildcard: `pose` first, then the door's one
method. A wrong kind reaching an arm is `MeetFault::Desync`. `Span::endless` replaces
`plane_crossing::SpanEnds`.

## a.2 Per arm (`route`'s `Door`)
Each rung moves out of its door into `pose` as a `curve_face_pose_*` row. Under `TABLE_K_PREFIX`, a row
is named for the module that decides it. The door keeps only its method.

**Plane** (`plane_crossing.rs`).

| | |
|---|---|
| Pose | From `conic_plane_candidates`: `plane_crossing_belly_graze`, where Negative is `Apart`, Zero goes on to `Kappa`, and in band is `BellyGraze`; `_plane_parallel`, where Zero with the offset Zero is `Lies`, else `Apart`, and in band is `PlaneParallel`; and `_graze_side`. The line arm's `_line_parallel` is the axis-parallel rung below. |
| Method | `span_roots` (`Span::place_param`, `_roots_apart`, `_root_order`), and a line's closed form |
| Windows | a root in band of an end; two roots in band of each other |

**Line** (`line_quadric.rs`).

| | |
|---|---|
| Axis-parallel | For a wall, or a line parallel to a plane, decided like `curve_pair_pose_parallel`. `r(mid)` is the residual at the span's middle, and `drift = \|slope\|·(t1 − t0)` is the drift the rung decided over the span. `Lies` where `\|r(mid)\| + drift/2` is Zero; `Apart` where `\|r(mid)\| − drift/2` is definite past the band; otherwise `InBand(AxisParallel)`. The span is the reading's own, so the trace's `on_carrier` and the sweep's `Constant` become this one predicate. |
| Discriminant | Wall and sphere: Zero goes on to `Kappa`, Negative is `Apart`. |
| Cone | The apex foot Zero with the lead Zero is `Lies` (a generator). The apex Zero with the lead definite is `Through`, its spread `_cone_apex_spread`. In band is `Apex`. The lead Zero off the apex stays the method's `Degenerate`, a whole-span window (`a-line-parallel-to-a-cone-generator-has-no-root-lane`). |
| Lies by kind | A line never lies on a sphere or a torus. |
| Method | `Centred::roots` and `line_torus_roots`; the cone's `line_quadric_cone_root_slack` meter. Wall and sphere roots carry no root-position meter today (`line-roots-carry-no-root-slack-meter`, a `design` row U3a does not settle). An `AxisParallel` met in the method is `MeetFault::Contradiction`. |

**Conic × quadric** (U2).

| | |
|---|---|
| Zero lane (§a.3) | From `first_harmonic_roots_in_band`: the noise rung; `coaxial` through `constant_residual_roots` (`CircleRoots::On` becomes `Lies`, a Miss `Apart`); and `extreme`, where one-signed is `Apart` and the zero band goes on to `Kappa`. The escalated-coaxial fall-through to the extremes stays as a stage of one rule. **The PR body states the proof that both stages decide the same class** (D3); reviewers will ask for it. |
| Cone | `bool_conic_cone_apex` Zero is `Through`, its spread `_apex_spread`. `off_the_apex`'s per-root `AtApex` becomes `Apex` in band. `bool_conic_cone_on_surface` is `Lies`. |
| Method | On the Zero lane, the core's slack and placement; otherwise `door_half_angle_roots` through `certified_subdivision`. Through the apex, the deflated `G` roots. |

**Conic × torus** (`circle_torus.rs`, `ellipse_torus.rs`).

| | |
|---|---|
| Pose | `circle_torus_coaxial_*` is `Lies` (a rim) or `Apart`. `circle_torus_meridian` is `Lies`. `DoorRefusal::Torus` becomes `PoseRefusal::Torus`, read before any pose. |
| Parallel axes | The contour-reach predicate moves to the pose as a `curve_face_pose_*` row if its Zero changes the pair's class; the pose may call the arm's contour machinery. If it only meters where roots fall, it stays in the method. The implementer applies that test and says so in the PR body. |

Ellipse × torus has no special pose.
**Not-`Lies`.** An arm with no rung that excludes `Lies` gets the general rung `NotLies`: one carrier point
whose residual is decided past the band. Found, the class is `Isolated`; not found, the pose escalates
`InBand(NotLies)`. It runs on circle × torus past the coaxial and meridian rungs, on ellipse × torus, and
on conic × quadric's ladder lane only for `A₂` in the gap (a definite `A₂` already excludes `Lies`). A
Villarceau circle therefore escalates rather than answering `Uncertain`; its own `Lies` rung stays with
`work/germ/a-villarceau-circle-lying-on-a-torus-is-unsettled.md`, which U3a touches but does not close.

**`Tangent`.** Every arm's double root reads `Kappa` (formerly `pm_census_point_kappa`): the relative
curvature across the point definitely positive, a nondegenerate double root, is `Tangent`; in band is
higher-order contact and escalates `InBand(Kappa)`. The arc is `√(2z/κ)` about the root at the extremum's
own curvature radius (a circle's formula understates a thin ellipse's flat end); each arm's arc encloses
the in-band arc and never understates it. `Conic::along` is re-posed in `plane_across`'s cancellation-free
form (`[simple.conditioned]`: a form that loses digits is re-posed), so the plane arm's touch point at
3e-11 rad off parallel is placed as the census row requires; reading the arc instead would hide the error.

**Spline arms** (`composite::crossings`, `spline_face::settle`) answer `Enclosed`, and their enclosures
are the method. Their in-band pieces and floor boxes are windows, and so is a spline run in band (Ev,
PR 7167). **Unbuilt routes** (spiric, sheet, periodic) answer `Unbuilt(route)`, a whole-span window.

**Windows in U3.** A closed form's `Unsettled` count and `half_angle`'s refusal are whole-span windows,
which is acceptable in U3; local windows are a U4 prerequisite (§7). A refused root slack is a window of
the root ± its slack.

**Apart's cull stage.** `conic_clearance` takes no `Body`, so it moves from `reduce` into the pose as
`Apart`'s cull stage, ahead of the closed rungs; in band it hands on, never refusing alone. It is one match
on surface kind, replacing `implicit::conic_residual_extremes`' harmonic-else-sampled path.
`bool_line_cylinder_clearance` waits for U4: it reads the edge's vertex points (`pu`, `pv`), gated on
their side readings, which are body records.

## a.3 The conic × quadric lane choice
C5's rule is a lane choice (`[verdicts.consumers]`) and records nothing.
`bool_conic_quadric_second_harmonic` decides `A₂`: Zero takes the first-harmonic core, with `A₂` charged to
the extremes' noise; in band and definite take the ladder. The pose decides `Lies`, `Apart` and `Tangent` from the extremes on the Zero lane only. On the ladder lane
those come from `NotLies` and the subdivision. Circle × sphere (`A₂ ≡ 0` by kind) takes the Zero lane
with no decision.

**The gate.** D3 says "the general lane's certificate does not rest on the answer", so the ladder must be
sound with `A₂` in the gap and at 0. By construction it is: `certified_subdivision` decides on F itself,
and a constant F in band never clears. But PR 4600 measured only definite `A₂`.

Owed: a fuzz row that forces the ladder lane through a test-only switch, judged against
`conic_oracle::exact` (no definite answer the oracle contradicts), with `in_band_sweep`'s mutant. It covers
circles within the gap of square to a wall (coaxial ones included), near-circular ellipses on a sphere, a
circle exactly square to a wall at depths across the band, and a cone's F. **If the row reds, U3a stops on
an `[ev]` blocker.**

## a.4 The wrappers and the sweep
The old entries stay only for readers U3 does not move. Each becomes a wrapper: `pose`, then the method,
mapped back to the old answer. A wrapper decides nothing, and a doc line names its last reader.

| Wrapper | Last reader |
|---|---|
| `plane_crossing_lane` | the sweep (`reduce`), `slit`, `factored_crossing`, `offer_rows`: U4 |
| `line_quadric_roots` | the sweep: U4 |
| `conic_quadric_roots`, `circle_torus_roots`, `ellipse_torus_roots` | `reduce::wall_crossing`: U4 |
| `line_wall_roots`, `line_cone_quadratic`, `WallRoots` | the ray lane: U5 |

**Only the wrappers produce the pose-deciding variants**, and they build them from `Pose`, each carrying
the class the pose decided: `ConicPlaneMeet::Parallel`, `ConicRootFault::{PlaneParallel, BellyGraze}`,
`WallRoots::{AxisParallel, Tangent}`, `SpanVerdict::{Constant, LiesOn, AtApex}`.
U3a deletes the sweep's own re-decision of `ConicPlaneMeet::Parallel { offset }` and the sweep's own
reading of `SpanVerdict::Constant`; both read the class instead. The re-decision is
`bool_conic_face_plane_offset`, in `reduce` and, as a copy, in `slit`. C5's "the plane door has one entry
point" holds once U4 retires the wrapper.

## a.5 Consumers
**`boolean::trace`.** `ray_crossings` reads `cross` on `(ray.start(), ray.from_start(reach))` for every
kind, the plane included, skipping `End(Start)`; `circle_crossings` reads `(0, τ)`, skipping both ends.
The plane arms, the torus-vs-`_ => conic_quadric_roots` match and the outer `_ =>` go. `Nurbs` and
`Approx` stay `Unsupported`, read off a `Pose(Unbuilt|Enclosed)` window. Trace matches the pose first:

| Answer | Trace |
|---|---|
| `Lies` | `OnCarrier` |
| `Through` | `Apex` |
| `Tangent`, a `Touch`, or a two-in-band window | `Grazing` |
| `Pose(InBand)` | `Escalated` |
| `Pose(Torus)` | `Torus` |
| `Method` | by `MeetFault::kind`: `Escalated` stays `Escalated`, `Touch` is `Grazing`, `Unread` is `Unsettled`, `Fault` is a defect panic |

Retired: `door`, `door_refusal`, `line_door_refusal`, `roots_of`, `unsettled` and their `unreachable!`s,
and the rows `TRACE_ON_CARRIER`, `_PLANE_DEPTH`, `_CROSSING_AHEAD`, `_CROSSING_APEX` and
`_CROSSING_ORDER`. Kept: `TRACE_CROSSING_SENSE`, and `plane_misses` with `TRACE_PLANE_LEAD`.

**Census touch confirm (`census::point::edge_face`).** The kind match and its `_ =>` refusal go. It reads
`cross` on the edge's span against any face surface, and matches on `Tangent` and reads its point:

| Answer | Census |
|---|---|
| `Tangent`, touch `Inside` | a candidate `Met`, checked by `in_both` |
| `Apart`, or `Isolated` with crossings only | `Stale` |
| a touch at an `End` | `Stale` |
| `Lies`, `Through` | a typed refusal |
| a window | `CensusUnsupported` or `CensusEscalated` |

Retired: `circle_plane`, `Locus`, the line × sphere arm, `plane_across` (its form moves into
`Conic::along`), `separates` and `pm_census_point_kappa` (now the pose's `Kappa`). `on_sphere` and the
private `Surface` stay for `face_face`.

## a.6 Retired, and rows owed
**Retired in U3a**, each with its last reader: `LineRoots::Constant`; `CircleRoots::On` (out of
`first_harmonic_roots`, `constant_residual_roots`, `circle_torus_roots`); `Unsettled::Tangency` from the
first-harmonic and wall arms; `Unsettled::AtApex` and generator `Degenerate` from the line arm;
`bool_conic_face_plane_offset`; §a.5's items.

**Rows** (`docs/prompts/implementer-discipline.md` §8). Each pose escalates in band: one `cross` row per
class and arm, answering `InBand` with its rung and pinning the consumer's lever. Every case is in the
gap:

| Class | Cases |
|---|---|
| `Lies` | a circle in a plane at an offset; a wall ruling (`\|r(mid)\| ± drift/2`); a coaxial circle on a wall or sphere; a torus rim; a torus meridian; a cone generator |
| `Apart` / `Tangent` | a plane belly; a wall, sphere or cone discriminant; a first-harmonic extreme; `a_graze_side_decided_zero_keeps_its_margin`, re-aimed |
| `Kappa` | higher-order contact on each arm with a `Tangent` |
| `Through` | a line's apex foot; `bool_conic_cone_apex` |
| `NotLies` | a Villarceau circle escalates; `A₂` in the gap on the ladder lane |

Each mutant below must turn its row red:

| Mutant | Row that reds |
|---|---|
| the pose skips `PlaneParallel` | a circle in its plane answers `Lies` |
| belly Zero read as two roots | a graze answers one `Touch` |
| axis-parallel answers `Isolated` | a ruling is `Lies`, and a parallel line off the wall is `Apart`, via `cross` and the wrapper |
| the coaxial stage deleted | rim circles of a wall, a sphere and a torus are `Lies` |
| the apex stage always Positive | a line and a circle through the apex are `Through` |
| `NotLies` skipped | a Villarceau circle answers `Isolated` |
| `Kappa` skipped | higher-order contact answers `Tangent` |
| `edge_face`'s kind match restored | the census serves a circle × sphere touch |
| the sweep re-decides `Parallel`'s offset | wrapper parity |

Also owed: `every_door_name_carries_a_listed_prefix` extended so no `curve_face_pose_*` row is decided
outside `curve_face.rs`; the §a.3 fuzz row; **Tangent arcs**, one oracle row per arm showing every span
point within the band of the surface lies inside `arc`; **touch accuracy**,
`a_circle_nearly_parallel_to_a_plane_is_touched_where_it_touches` re-aimed at the pose's `Tangent` point
through the re-posed `Conic::along`; **the census serves every built pair** of `route` (a touch `Met`, a
crossing or `Apart` `Stale`, an unbuilt pair refused typed); **wrapper parity** (the sweep's pose rows,
ONEDOOR and `in_band_sweep` unchanged); and windows placed, with meetings outside them still reported
(`window_fuzz`).

## a.7 Review tier
**Dual** (one Opus, one Fable). U3a fixes the types that U3b, U4, U5 and the face reading build on. It
moves every closed door's self-decided poses at once and carries the §a.3 soundness gate. A wrong shape
here is expensive to change once U4 builds on it.

# U3b — ring path, classify, on_surface, region (PR 2, after U3a)
**`ring_path::plane_roots`** reads `cross` on `Span::new(p.carrier(), p.span)` against the section's
plane. `Isolated` meetings are the roots, with the slope `|n̂·τ̂|` read at each point; the arrival pick
stays a raw comparison; `Tangent` and pose windows give `None`. `split_ring_path_meets_plane` and the
hand-rolled harmonic retire. `cross` charges no rounding (`[simple.conditioned]`: "nothing charges, pads
or estimates float rounding"), so the ring path drops its `slack_of` charge from these readings.
**Dependency**: if that reds ring-path rows, U3b waits on
`work/germ/ring-path-readings-decide-past-their-resolution-at-far-frames.md`, which re-poses the frames.

**`splitting::classify`** reads `cross` at both `plane_crossing_lane` sites (one in tests). A vertex
already recorded ON the plane is a record read and stays `classify`'s, ahead of `cross`. `Lies` and
`Apart` mean `continue`; each `Inside` meeting, crossing or touch, splits ascending (the graze net inserts
the graze root); end hits are skipped. Vertex sides decided opposite past the band while `cross` finds no
interior root and no end hit is `MeetFault::Contradiction`; the sides-derived line interpolation retires.
A window is `SplitReduceError::CrossingEscalated`, its `fault` now `CrossWhy` (public, re-exported in
`topo::lib`; the lever split kept). The `Unlaned` box gate (`edge_clears`) goes: a spline edge is read
through `composite`, a spiric edge refuses on its window.

**`curve_pair::on_surface`.** `OnSurface::across` reads `cross` against a `Surface::Plane` built from
`curve.plane()`. `InPlane` is `Lies`, `Touch` is `Tangent`, and `Roots` carries `Meeting`s in place of
`(SpanRoot, Point3)` (`RootPlace::Inside`/`AtEnd` become `OnSpan::Inside`/`End`, `Undecided` a window).
The caller's `row` for the parallel offset goes (the pose owns it), so the K names
`"bool_sphere_cut_in_plane"` and `D::COMPONENT` change. `CoSurface::Unlaned` retires; a spline edge is
`composite`'s answer. `region`, `splitting::containment` and `boolean::ops`' sphere cut-in move to
`OnSpan` and windows.

**`region`'s across-then-`under`** retires. If `under` (the curve read across a spiric edge's own plane)
is a correct general reading of a spiric span against a plane, it becomes the `(Spiric, Plane)` arm's one
method, fixed in `route`. Otherwise a spiric edge is the unbuilt route's whole-span window, refused typed.
The implementer proves or refutes that in the PR body. A plane tangent to the surface along the edge is a
pose fact, and escalates in band.

**Retired in U3b**: `plane_crossing::span_plane_meet`, `SpanPlaneMeet`, `SpanEnds`, `SpanRoot`;
`CoSurface::Unlaned`; `OnSurface::under` as a fallback; `slack_of`'s charge on these readings.
`RootPlace` and `span_place` remain for `torus_region`.

**Rows owed**: the ring path's rows pass without `slack_of` (or U3b waits, above); in classify, a mutant
restoring the interpolation reds a row, and sides opposite with `cross` empty answer `Contradiction`; a
spiric edge is read by its arm or refuses typed; a plane tangent along an edge escalates in band.

**Review tier: single FULL.** The moves can be read, but the classify contradiction rule and `under`'s
soundness are correctness claims.

## 6. Tracker rows U3 touches

Rows that recently arrived on GERM's slate (from GERMTAIL and HONEROOT) and that U3 touches. U3 closes
none of them.
- `a-villarceau-circle-lying-on-a-torus-is-unsettled`: U3a's `NotLies` turns the `Uncertain` into an
  in-band escalation. The `Lies` rung is still owed.
- `line-roots-carry-no-root-slack-meter`: the line arm's method carries this design question unchanged.
- `boolean-in-span-readings-of-grazing-roots-are-levered-by-arc-length`: `cross` places roots by
  `Span::place_param` as `wall_crossing` does, so the root-slack bound on the in-span reading is still
  owed.
- `root-slack-meters-roots-outside-the-arc` and
  `a-circle-touching-a-torus-off-the-arc-escalates-the-arcs-side-decision`: whole-span windows (§a.2)
  keep the refusal. Both close with U4's local windows.
- `ring-path-reads-curve-meetings-and-one-curve-outside-meet`: U3b moves `plane_roots` only.
  `misses_in_plane` and `coplanar_apart` still run after it and stay curve-pair work.
- `a-ring-path-along-a-ruling-or-crest-arcs-carrier-reads-nothing`: U3b's `Lies` maps to `None`, as
  today.

## 7. Out of scope, and what U4 needs
**U4** takes the sweep's lanes, `carrier_touch::off_face`, `bool_line_cylinder_clearance`, the face
reading's window rule, `slit` and `factored_crossing`. **A U4 prerequisite**: `half_angle`'s undecided
pieces and the closed forms' unsettled spreads plumbed out as local windows (U3 answers whole-span ones).
**U5** takes the ray lane, **U1b** line × cone; `ops::apply_cut_ins`, `sphere_region::ray_roots` and
`pcurve_cache` incidence are not this question.
