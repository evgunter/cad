# GERM crossing U3 — `curve_face::pose`, `curve_face::cross`, and the small consumers (unit spec)

This spec carries U3 of `work/germ/curve-surface-crossing-has-no-kind-pair-table.md` (its Ratified
section), building C5's curve × face paragraph as PR 7167 ratified it (`crates/geom-brep/README.md`). It
lands as two PRs:
- **U3a** builds the types, `curve_face::pose` and `curve_face::cross`, dispatched through
  `curve_face::route`. It moves `boolean::trace` and the census touch confirm onto them.
- **U3b** moves `ring_path::plane_roots`, `splitting::classify`, `curve_pair::on_surface` and `region`.

The sweep and `slit`/`factored_crossing` are U4's, and the ray lane's remaining reads are U5's. Until U4
the sweep reads the doors through wrappers (§a.4).

The questions this spec first left open were answered by two designers and reconciled by the GERM
orchestrator on 2026-11-20. Their answers are stated below as the spec. The spec was read on
`c093993bb2`, with U1 read on the `germ/line-roots-door` lane's uncommitted tree. It cites names, not
lines. Delete it when U3b merges.

## 0. Prerequisites

**U1** delivers:
- `curve_face::line_quadric`: `line_quadric_roots`, `LineRoots`, `LineQuadricRefusal`, `WallRoots`,
  `About`, the per-kind finders, `line_cone_roots` and `apex_root_spread`;
- `reduce::line_wall_roots_of` deleted;
- trace on `line_quadric_roots`, which drops `ray_crossings`' `Some(Err(_)) => unreachable!`.

U3a needs the axis-parallel, cone apex, lead and spread rungs callable on their own. Then the pose and
the wrappers share one predicate per row.

**U2** delivers `curve_face::conic_quadric`, refusing `DoorRefusal` (not `BooleanError`), with
`first_harmonic_arm`, `quadric_ladder`, `cone_roots`, `through_the_apex` and `off_the_apex`. Its rows sit
under a `DOOR_K_PREFIXES` prefix.

Never in flight with U3a:
- U1b, because both rewrite the apex rungs;
- the first-harmonic arm's Part 2 (`germ-rounds` P/Q), because both rewrite `first_harmonic_roots`'
  decide and count.

# U3a — the types, `pose`, `cross`, trace, census (PR 1)

## a.1 The types

**`curve_face::Pose<T>`** is new. `curve_pair::pose::Pose`, the curve-pair pose, stays.
- `Lies`: holds over the whole span; a reading of values only.
- `Apart`.
- `Tangent(Touch<T>)`: `Isolated`'s pose-decided special case. `Touch { at: T, point, arc: (T, T) }` is
  the double root's point and an arc of span parameters. The arc encloses the in-band arc and never
  understates it (§a.2, "Tangent").
- `Through(Singular<T>)`: `Singular { at, point, spread: (T, T) }`. In U3 only a cone's apex reaches
  it, because the spindle route is unbuilt. **The singular point is not a meeting.** It is the pose's
  datum, and consumers match the pose first: the sweep places it on the apex vertex, the trace answers
  `Apex`, and the census refuses it typed.
- `Isolated(Form<T>)`: `Form` is private per arm (the harmonics, quadratic or candidates the pose
  computed), and the method takes it without re-deciding. **`Isolated` is answered only once `Lies` is
  excluded** (§a.2, "Not-`Lies`").

**`PoseRefusal`** is `InBand { rung: PoseRung, diag } | Enclosed | Unbuilt(Route) |
Torus(TorusRefusal) | Fault(&'static str)`. `Enclosed` means a spline arm, which has no closed pose.

**`PoseRung`** is `PlaneParallel | BellyGraze | AxisParallel | Coaxial | Extreme | Kappa | Apex |
ConeLead | Meridian | NotLies`, plus the torus contour rung if it moves (§a.2). A consumer maps a pose
rung to its coincidence lever (today `BooleanDecision::of_conic_root`'s `Coincide::EdgeOnPlane`), and a
method fault to its crossing decision.

**The meeting** is new and one-sided: `curve_face::Meeting<T> { point, on_span: OnSpan<T>, kind:
Kind<T> }`.
- `OnSpan<T>` is `Inside { t } | End(SpanEnd)`. It has no `Past` and no margin, because a root placed by
  parameter has no on-carrier reading. `curve_pair::Hit` is not reused.
- `Kind<T>` is `Cross | Touch { arc: (T, T) }`.

**The answer** is `Crossings<T> { pose: Pose<T>, meetings: Vec<Meeting<T>> }`. The meetings ascend along
the span, and every pair is decided apart by its chord; two in band of each other are a window. `Lies`
and `Apart` hold none. `Tangent` holds its one touch. `Through` holds the meetings other than the
singular point.

**The refusal** is `CrossRefusal<T> { found: Vec<Meeting<T>>, windows: Vec<Undecided<T, CrossWhy>> }`.
- It is shaped like `curve_pair::MeetRefusal`: the meetings decided before it, never the complete set.
- It is read like `curve_face::Located`: exact outside the windows, silent inside them.
- `curve_face::Undecided` becomes generic in its why (`Undecided<T, W = BoundaryRefusal>`), so the
  boundary pass is unchanged. `Located::undecided_over` lifts to a function over `&[Undecided<T, W>]`.
- `CrossWhy` is `Pose(PoseRefusal) | Method { door: Door, fault: MeetFault }`.
- A pose refusal, an unbuilt route included, is one whole-span window.

**Entry points** (in `curve_face.rs`):
- `pose(span: &Span<'_, T>, surface: &Surface<T>, band) -> Result<Pose<T>, PoseRefusal>`;
- `cross(span, surface, band) -> Result<Crossings<T>, CrossRefusal<T>>`.

`cross` matches `route(..).door` exhaustively, with no wildcard: `pose` first, then the door's one
method. A wrong kind at an arm is `MeetFault::Desync`. `Span::endless` replaces
`plane_crossing::SpanEnds`.

## a.2 Per arm (`route`'s `Door`)

Each rung moves out of its door into `pose`, as a `curve_face_pose_*` row (`TABLE_K_PREFIX`: the row is
named for the module that decides it). The door keeps only its method.

**Plane** (`plane_crossing.rs`).
- `conic_plane_candidates`' rungs:
  - `plane_crossing_belly_graze`: Negative is `Apart`, Zero goes on to `Kappa` (below), in band is
    `BellyGraze`.
  - `_plane_parallel` Zero, with the offset Zero, is `Lies`; otherwise `Apart`. In band is
    `PlaneParallel`.
  - `_graze_side`.
- The line arm's `_line_parallel`, decided as the axis-parallel rung below.
- Method: `span_roots` (`Span::place_param`, `_roots_apart`, `_root_order`), and a line's closed form.
- Windows: a root in band of an end, and two roots in band of each other.
- Apart: the belly.

**Line** (`line_quadric.rs`).
- **The axis-parallel rung** (a wall, or a line parallel to a plane) is decided like
  `curve_pair_pose_parallel`. Take the residual at the span's middle, `r(mid)`, with
  `drift = |slope|·(t1 − t0)`, the drift the rung decided over the span:
  - `Lies` where `|r(mid)| + drift/2` is Zero;
  - `Apart` where `|r(mid)| − drift/2` is definite past the band;
  - `InBand(AxisParallel)` otherwise.

  The span is the reading's own, so trace's `on_carrier` and the sweep's `Constant` become this one
  predicate.
- Wall and sphere discriminants: Zero goes on to `Kappa`, Negative is `Apart`.
- Cone:
  - the apex foot Zero with the lead Zero is `Lies` (a generator);
  - apex Zero with the lead definite is `Through`, its spread from `_cone_apex_spread`;
  - in band is `Apex`.
- The lead Zero off the apex stays the method's `Degenerate` whole-span window
  (`a-line-parallel-to-a-cone-generator-has-no-root-lane`).
- Sphere and torus exclude `Lies` by kind: a line never lies on either.
- Method: `Centred::roots` with the slacks, and `line_torus_roots`. An `AxisParallel` met in the method
  is `MeetFault::Contradiction`.

**Conic × quadric** (U2).
- **Zero lane** (§a.3), `first_harmonic_roots_in_band`'s rungs:
  - noise;
  - `coaxial` through `constant_residual_roots`: `CircleRoots::On` becomes `Lies`, and a Miss `Apart`;
  - `extreme`: one-signed is `Apart`, and in the zero band it goes on to `Kappa`.
- **The escalated-coaxial fall-through** to the extremes stays only as a stage of one rule. **The PR body
  states the proof that both stages decide the same class** (D3); reviewers will ask for it.
- **Cone:**
  - `bool_conic_cone_apex` Zero is `Through`, its spread from `_apex_spread`;
  - `off_the_apex`'s per-root `AtApex` becomes `Apex` in band;
  - `bool_conic_cone_on_surface` is `Lies`.
- Method: the Zero lane's slack and placement; otherwise `door_half_angle_roots`, through
  `certified_subdivision`; through the apex, the deflated `G` roots.

**Conic × torus** (`circle_torus.rs`, `ellipse_torus.rs`).
- Rungs:
  - `circle_torus_coaxial_*` is `Lies` (a rim) or `Apart`;
  - `circle_torus_meridian` is `Lies`;
  - `DoorRefusal::Torus` becomes `PoseRefusal::Torus`, read before any pose.
- The parallel-axes contour-reach predicate moves to the pose as a `curve_face_pose_*` row if its Zero
  changes the pair's class (the pose may call the arm's contour machinery). If it only meters where roots
  fall, it stays in the method. The implementer applies that test and states the result in the PR body.
- Ellipse × torus has no special pose.

**Not-`Lies`.** Every arm without a rung that excludes `Lies` gets the general rung `NotLies`: one
carrier point whose residual is decided past the band. Found, the class is `Isolated`; not found, the pose
escalates `InBand(NotLies)`.
- Where it runs: circle × torus past the coaxial and meridian rungs, ellipse × torus, and conic × quadric
  on the ladder lane.
- On the ladder lane it runs only for `A₂` in the gap. `A₂` definite already excludes `Lies`, since a
  zero residual has every harmonic zero.
- It covers Villarceau circles with no special rung: they escalate. Their `Lies` rung is filed P3 as
  `work/germ/a-villarceau-circle-has-no-lies-rung.md`.

**`Tangent`.**
- Every arm's double root reads `Kappa`. This was `pm_census_point_kappa`: the relative curvature across
  the point definitely positive, which is a nondegenerate double root. Definite is `Tangent`. In band is
  higher-order contact and escalates `InBand(Kappa)`.
- The arc is `√(2z/κ)` about the root, at the extremum's own curvature radius. A circle's formula
  understates a thin ellipse's flat end. Each arm's arc encloses the in-band arc and never understates it
  (rows in §a.6).
- `Conic::along` is re-posed in `plane_across`'s cancellation-free form (`[simple.conditioned]`: a form
  that loses digits is re-posed), so the plane arm's touch point at 3e-11 rad off parallel is placed as
  the census row requires. Reading the arc instead would hide the error.

**Spline arms** (`composite::crossings`, `spline_face::settle`): `Enclosed`, with the enclosures as the
method. Their windows are their in-band pieces and floor boxes. A spline run in band is a window (Ev,
PR 7167).

**Unbuilt routes** (spiric, sheet, periodic): `Unbuilt(route)`, one whole-span window.

**Windows.** A closed form's `Unsettled` count and `half_angle`'s refusal are whole-span windows in U3.
That is acceptable now; local windows are a U4 prerequisite (§7). A refused root slack is the root ± its
slack.

**Apart's cull stage.**
- `conic_clearance` takes no `Body`, so it moves from `reduce` into the pose as `Apart`'s cull stage,
  run before the closed rungs. In band it hands on and never refuses alone. It becomes one match on
  surface kind, replacing `implicit::conic_residual_extremes`' harmonic-else-sampled path.
- `bool_line_cylinder_clearance` waits for U4. It reads the edge's vertex points (`pu`, `pv`) and is
  gated on their side readings, which are body records.

## a.3 The conic × quadric lane choice

C5: `bool_conic_quadric_second_harmonic` decides `A₂`. Zero takes the first-harmonic core, with `A₂`
charged to the extremes' noise. In band and definite take the ladder. This is a lane choice
(`[verdicts.consumers]`), and it records nothing.
- The pose decides `Lies`, `Apart` and `Tangent` from the extremes on the Zero lane only.
- On the ladder lane, those come from `NotLies` and the subdivision.
- Circle × sphere (`A₂ ≡ 0` by kind) takes the Zero lane, with no decision.

**The gate.** D3 says "the general lane's certificate does not rest on the answer", so the ladder must be
sound with `A₂` in the gap and at 0. By construction it is: `certified_subdivision` decides on F itself,
so a constant F in band never clears. PR 4600 measured only `A₂` definite.

Owed: a fuzz row forcing the ladder lane through a test-only switch, judged against
`conic_oracle::exact` (no definite answer the oracle contradicts), with `in_band_sweep`'s mutant. Its
cases:
- circles within the gap of square to a wall, coaxial included;
- near-circular ellipses on a sphere;
- a circle exactly square to a wall at depths across the band;
- a cone's F.

**If it reds, U3a stops on an `[ev]` blocker.**

## a.4 The wrappers, and the sweep

The old entries stay only for readers U3 does not move. Each becomes a wrapper: `pose`, then the method,
mapped to the old answer. It decides nothing, and a doc line names its last reader.

| Wrapper | Last reader |
|---|---|
| `plane_crossing_lane` | the sweep (`reduce`), `slit`, `factored_crossing`, `offer_rows`: U4 |
| `line_quadric_roots` | the sweep: U4 |
| `conic_quadric_roots`, `circle_torus_roots`, `ellipse_torus_roots` | `reduce::wall_crossing`: U4 |
| `line_wall_roots`, `line_cone_quadratic`, `WallRoots` | the ray lane: U5 |

**Only the wrappers produce the pose-deciding variants, and they build them from `Pose`.** Each variant
carries the class the pose decided:
- `ConicPlaneMeet::Parallel`;
- `ConicRootFault::{PlaneParallel, BellyGraze}`;
- `WallRoots::{AxisParallel, Tangent}`;
- `SpanVerdict::{Constant, LiesOn, AtApex}`.

U3a deletes the sweep's own re-decision of `ConicPlaneMeet::Parallel { offset }`
(`bool_conic_face_plane_offset` in `reduce`, and its copy in `slit`), and the sweep's own reading of
`SpanVerdict::Constant`. Both read the class instead. C5's "the plane door has one entry point" holds once
U4 retires the wrapper.

## a.5 Consumers

**`boolean::trace`.**
- `ray_crossings` reads `cross` on `(ray.start(), ray.from_start(reach))` for every kind, the plane
  included, and skips `End(Start)`.
- `circle_crossings` reads `(0, τ)` and skips both ends.
- The plane arms, the torus-vs-`_ => conic_quadric_roots` match and the outer `_ =>` all go.
- `Nurbs` and `Approx` stay `Unsupported`, read off a `Pose(Unbuilt|Enclosed)` window.

Trace matches the pose first:

| Answer | Trace |
|---|---|
| `Lies` | `OnCarrier` |
| `Through` | `Apex` |
| `Tangent`, a `Touch` meeting, or a two-in-band window | `Grazing` |
| `Pose(InBand)` | `Escalated` |
| `Pose(Torus)` | `Torus` |
| `Method` | by `MeetFault::kind`: `Escalated`, `Grazing` (from `Touch`), `Unsettled` (from `Unread`), or a defect panic (from `Fault`) |

- Retired: `door`, `door_refusal`, `line_door_refusal`, `roots_of`, `unsettled` and their
  `unreachable!`s; `TRACE_ON_CARRIER`, `_PLANE_DEPTH`, `_CROSSING_AHEAD`, `_CROSSING_APEX`,
  `_CROSSING_ORDER`.
- Kept: `TRACE_CROSSING_SENSE`, and `plane_misses` with `TRACE_PLANE_LEAD`.

**Census touch confirm (`census::point::edge_face`).** The kind match and its `_ =>` refusal go. It reads
`cross` on the edge's span against any face surface, and matches on `Tangent`, reading its point.

| Answer | Census |
|---|---|
| `Tangent` with the touch `Inside` | candidate `Met`, checked by `in_both` |
| `Apart`, or `Isolated` with crossings only | `Stale` |
| a touch at an `End` | `Stale` |
| `Lies`, or `Through` | typed refusal |
| a window | `CensusUnsupported` or `CensusEscalated` |

- Retired: `circle_plane`, `Locus`, `plane_across` (its form moves into `Conic::along`), `separates` and
  `pm_census_point_kappa` (now the pose's `Kappa`), and the line × sphere arm.
- Kept: `on_sphere` and the private `Surface`, for `face_face`.

## a.6 Retired, and rows owed

**Retired in U3a**, each with its last reader:
- `LineRoots::Constant`;
- `CircleRoots::On`, out of `first_harmonic_roots`, `constant_residual_roots` and `circle_torus_roots`;
- `Unsettled::Tangency`, from the first-harmonic and wall arms;
- `Unsettled::AtApex` and the generator `Degenerate`, from the line arm;
- `bool_conic_face_plane_offset`;
- §a.5's items.

**Rows** (`docs/prompts/implementer-discipline.md` §8).

*A pose in band escalates*: one `cross` row per class and arm, answering `InBand` with its rung and
pinning the consumer's lever. Each case is in the gap:
- `Lies`: a circle in a plane at an offset; a wall ruling (`|r(mid)| ± drift/2`); a coaxial circle on a
  wall or sphere; a torus rim; a torus meridian; a cone generator.
- `Apart`/`Tangent`: a plane belly; a wall, sphere or cone discriminant; a first-harmonic extreme;
  `a_graze_side_decided_zero_keeps_its_margin`, re-aimed.
- `Kappa`: higher-order contact, on each arm with a `Tangent`.
- `Through`: a line's apex foot; `bool_conic_cone_apex`.
- `NotLies`: a Villarceau circle escalates; `A₂` in the gap on the ladder lane.

*Mutants*:

| Mutant | Row that reds |
|---|---|
| the pose skips `PlaneParallel` | a circle in its plane answers `Lies` |
| the belly's Zero is read as two roots | a graze answers one `Touch` |
| axis-parallel answers `Isolated` | a ruling is `Lies`, and a parallel line off the wall is `Apart`, via `cross` and the wrapper |
| the coaxial stage is deleted | rim circles of a wall, a sphere and a torus are `Lies` |
| the apex stage reads Positive always | a line and a circle through the apex are `Through` |
| `NotLies` is skipped | a Villarceau circle answers `Isolated` |
| `Kappa` is skipped | higher-order contact answers `Tangent` |
| `edge_face`'s kind match is restored | the census serves a circle × sphere touch |
| the sweep re-decides `Parallel`'s offset | the wrapper-parity row |

*Also owed*:
- `every_door_name_carries_a_listed_prefix`, extended so no `curve_face_pose_*` row is decided outside
  `curve_face.rs`.
- The §a.3 fuzz row.
- **Tangent arcs**: an oracle row per arm, showing every span point within the band of the surface lies
  inside `arc`.
- **Touch accuracy**: `a_circle_nearly_parallel_to_a_plane_is_touched_where_it_touches`, re-aimed at the
  pose's `Tangent` point through the re-posed `Conic::along`.
- **The census serves every pair**: a table over every built `(CurveKind, SurfaceKind)` in `route`. A
  touch reads `Met`, a crossing and `Apart` read `Stale`, and an unbuilt pair refuses typed.
- **Wrapper parity**: the sweep's pose rows, ONEDOOR and `in_band_sweep` all answer unchanged.
- **Windows**: placed, with meetings outside them still reported (`window_fuzz`).

## a.7 Review tier

**Dual** (one Opus, one Fable):
- U3a fixes the types U3b, U4, U5 and the face reading build on.
- It moves every closed door's self-decided poses at once.
- It carries the §a.3 soundness gate.
- A wrong shape is expensive once U4 builds on it.

# U3b — ring path, classify, on_surface, region (PR 2, after U3a)

**`ring_path::plane_roots`** reads `cross` on `Span::new(p.carrier(), p.span)` against the section's
plane.
- `Isolated` meetings are its roots, each with the slope `|n̂·τ̂|` read at its point.
- The arrival pick stays a raw comparison.
- `Tangent` and pose windows give `None`.
- Retired: `split_ring_path_meets_plane` and the hand-rolled harmonic.
- `cross` charges no rounding (`[simple.conditioned]`: "nothing charges, pads or estimates float
  rounding"), so ring path drops its `slack_of` charge from these readings.
- **Dependency**: if that reds ring-path rows, U3b waits on
  `work/germ/ring-path-readings-decide-past-their-resolution-at-far-frames.md`, which re-poses the
  frames.

**`splitting::classify`** reads `cross` at both `plane_crossing_lane` sites (one is in tests).
- A vertex already recorded ON the plane is a record read: it stays `classify`'s, ahead of `cross`.
- `Lies` and `Apart` are `continue`.
- Each `Inside` meeting, crossing or touch, splits ascending. The graze net inserts the graze root.
- End hits are skipped.
- Vertex sides decided opposite past the band, with no interior root and no end hit from `cross`, is
  `MeetFault::Contradiction`.
- The sides-derived line interpolation retires.
- A window is `SplitReduceError::CrossingEscalated`. Its `fault` becomes `CrossWhy`, a public type
  re-exported in `topo::lib`, and the lever split is kept.
- The `Unlaned` box gate (`edge_clears`) goes. A spline edge is read through `composite`; a spiric edge
  refuses on its window.

**`curve_pair::on_surface`.** `OnSurface::across` reads `cross` against a `Surface::Plane` built from
`curve.plane()`.
- `InPlane` is `Lies` and `Touch` is `Tangent`.
- `Roots` carries `Meeting`s, replacing `(SpanRoot, Point3)`. `RootPlace::Inside` and `AtEnd` map to
  `OnSpan::Inside` and `End`; `Undecided` maps to a window.
- The caller's `row` for the parallel offset goes, since the pose owns it. That renames the K names
  `"bool_sphere_cut_in_plane"` and `D::COMPONENT`.
- `CoSurface::Unlaned` retires; a spline edge is `composite`'s answer.
- The callers `region`, `splitting::containment` and `boolean::ops`' sphere cut-in move to `OnSpan` and
  windows.

**`region`'s across-then-`under`** retires.
- If `under` (the curve read across a spiric edge's own plane) is a correct general reading of a spiric
  span against a plane, it becomes the `(Spiric, Plane)` arm's one method, fixed in `route`.
- Otherwise a spiric edge is the unbuilt route's whole-span window, refused typed.
- The implementer proves or refutes `under`'s generality in the PR body.
- A plane tangent to the surface along the edge is a pose fact; in band it escalates.

**Retired in U3b**:
- `plane_crossing::span_plane_meet`, `SpanPlaneMeet`, `SpanEnds` and `SpanRoot`;
- `CoSurface::Unlaned`;
- `OnSurface::under` as a fallback;
- `slack_of`'s charge on these readings.

`RootPlace` and `span_place` remain for `torus_region`.

**Rows**:
- ring path's existing rows pass without `slack_of`, or U3b waits on the dependency above;
- classify: a mutant restoring the interpolation, and a row with sides opposite and `cross` empty
  answering `Contradiction`;
- region: a spiric edge reads by its arm or refuses typed;
- a plane tangent along an edge escalates in band.

**Review tier**: single FULL. The consumer moves can be read directly, but the classify contradiction
rule and `under`'s soundness are correctness claims.

## 7. Out of scope, and what U4 needs

**U4** owns:
- the sweep's lanes;
- `carrier_touch::off_face`;
- `bool_line_cylinder_clearance`;
- the face reading's window rule;
- `slit` and `factored_crossing`.

**A U4 prerequisite**: `half_angle`'s undecided pieces and the closed forms' unsettled spreads must be
plumbed out as local windows. U3 answers whole-span windows.

**Elsewhere**:
- U5: the ray lane (`solid_contain::cast_ray`, `bool_ray_cone_*`);
- U1b: line × cone;
- not this question: `ops::apply_cut_ins`, `sphere_region::ray_roots`, and `pcurve_cache` incidence.
