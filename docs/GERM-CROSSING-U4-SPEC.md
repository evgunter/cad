# GERM crossing U4 — the sweep's lanes onto `curve_face::cross` (unit spec)
This spec carries U4 of `work/germ/curve-surface-crossing-has-no-kind-pair-table.md` (its Ratified
section): the boolean's sweep reads every edge × face pair through `curve_face::cross` and one face
reading, as C5's curve × face paragraph states (`crates/geom-brep/README.md`). Its shape follows
`docs/GERM-CROSSING-U3-SPEC.md`, which it builds on. It was read on main at `739ffae37c`, after U3a
(PR 7226), U3b (PR 7299), the far-frame re-pose (PR 7411) and the direction table and spiric × plane
door (PR 7431). Names are cited with their line on that commit; the line may go stale, the name may not.
This spec is deleted when U4's last stage merges.

U4 lands as four PRs:
- **U4a** gives `pose` and `cross` local windows (geom-brep only);
- **U4b** adds the face reading and moves the sweep's curved lane onto `cross`;
- **U4c** moves the planar lane onto `cross`, which merges the two lanes, and moves `slit`,
  `factored_crossing` and `offer_rows` with it;
- **U4d** reads the spline arms through `cross`, once Ev has answered the fork below.

U4a has no prerequisite. U4b follows U4a, U4c follows U4b, and U4d follows the fork's answer and U4b.

# 1. What the code does today
## 1.1 The readers U4 owns

| Reader | Where | What it decides | Undecided windows today | Through `cross`? |
|---|---|---|---|---|
| The planar lane, conic | `reduce::sweep_direction` (`reduce.rs:907`) via `plane_crossing_lane` (`reduce.rs:1191`) | a conic edge's roots against a plane face, each placed by `contfp_decided`; the first the trim holds is split | the wrapper's first refusal becomes a `ConicRootFault`, escalated whole (`reduce.rs:1292`); two roots within the band are inserted once (`PlaceWhy::Together`, `plane_crossing.rs:180`) | no: wrapper |
| The planar lane, line | `sweep_direction`, the endpoint-sign match (`reduce.rs:1442`–`1452`) | a line's crossing at `t0 + (t1 − t0)·d₁/(d₁ − d₂)`, `d` the plane offsets at the vertices | none: the vertex sides decide | no: its own formula |
| The planar lane, spline | `spline_plane_crossings` (`reduce.rs:4729`) → `composite_count` (`:4793`) → `T::spline_face_crossings` | a spline span's crossings of a plane | a refusal is read whole, then localized by `carrier_touch::off_face` (`:4817`) | no: the lane directly |
| The curved lane | `curved_face_arm` (`reduce.rs:2287`) | the event of an edge against a cylinder, cone, sphere, torus or spline face | per arm, below | no |
| The conic clearance | `conic_clearance` (`reduce.rs:3272`), called at `:2505` | `bool_conic_curved_clearance`: a conic arc definitely clear of the surface | in band falls through to the roots | no: a ladder stage the pose does not hold |
| The line clearance | the `(Positive, Positive)` arm (`reduce.rs:3130`–`3226`) | `bool_line_cylinder_clearance`: a line's endpoint residuals less a dip charge, read from the vertex points | not clear goes to the roots | no: reads body records |
| The wall crossing | `wall_crossing` (`reduce.rs:5102`), five calls in `curved_face_arm` | an analytic span's roots against a curved surface, through `line_quadric_roots` (`:5138`), `conic_quadric_roots` (`:5158`), `circle_torus_roots` (`:5164`) and `ellipse_torus_roots` (`:5173`), each placed by `place_crossing` (`:5047`) | any escalation or `Unsettled` asks `off_face` (`:5126`, `:5197`); `AtApex` refuses | no: wrappers |
| The off-face localizer | `carrier_touch::off_face` (`carrier_touch.rs:85`), `clusters` (`:222`), `ball_off_face` (`:308`) | whether every meeting of the span with the carrier is off the face: it bisects the span itself into clusters, then reads each cluster's ball against the trim | is itself the window reading, on clusters of its own | no: a second method for the meetings' places |
| The spline walls | `spline_wall_event` (`reduce.rs:4894`) via `composite_count` | a spline span against an analytic curved face | as the planar spline lane | no |
| The spline face | `spline_face` (`reduce.rs:3360`), `spline_interior` (`:3479`) via `T::spline_chart_meetings` | an analytic or spline span against a spline face, past the boundary pass | a `SplineFaceRefusal` refuses whole | no |
| `slit` | `slit::conic_or_line_on` (`slit.rs:1359`) | whether an edge's carrier lies on a face's surface: a conic on a plane through `plane_crossing_lane` (`:1379`); a line on a cylinder or cone through `census::line_on_quadric_carrier` (`:1395`); a conic on a quadric or torus at five or nine samples (`Variety::read_piece`); a spiric on a plane through `pose` (`:1449`) | an in-band offset escalates; a spiric pose refusal other than `InBand` is `unreachable!` (`:1459`) | partly: the spiric arm reads `pose` |
| `factored_crossing` | `factored_crossing.rs:65`, reached from `curved_face_arm` (`reduce.rs:2367`) | an edge on one of two equal cylinders with meeting axes, against the other, read through the bisector planes | a conic in a bisector plane, a node, or an unlaned carrier returns `Ok(None)` (`:123`, `:134`, `:145`, `:158`), and the caller goes on to the quadric door | no: `plane_crossing_lane` (`:109`), and a line's root `plane_offset(origin, n, centre)/dir·n` (`:125`) |
| `offer_rows` | `offer_rows.rs:1358` (test) | a probe reading a conic's plane roots for an executed tolerance offer | maps `ConicRootFault` to a `BooleanDecision` | no: wrapper |
| The refusal mappers | `BooleanError::of_line_quadric_refusal`, `of_conic_quadric_refusal`, `of_torus_door_refusal` (`boolean/mod.rs:4815`–`4867`), `BooleanDecision::of_conic_root` (`refusal_routes.rs:1285`), `pub use ConicRootFault` (`lib.rs:1608`) | the wrappers' refusals as boolean decisions | | wrapper shapes |
| Test callers | `reduce.rs:7276`, `:9458`, `:9556`, `:9690` | rows that call `line_quadric_roots` and `conic_quadric_roots` directly | | wrappers |

## 1.2 What the code now forces where U3 left a choice
- **Pose refusals are whole-span today.** `PoseRefusal` (`pose.rs:181`) is `Copy` and has no scalar.
  It rides inside `CrossWhy`, which is public in topo (`lib.rs:1607`) and sits in
  `SplitReduceError::CrossingEscalated`. A window's extent therefore cannot ride on `PoseRefusal`
  without putting `T` into public error types. It rides on `Undecided` (§2.2).
- **Method refusals are whole-span today.** `cross` maps every method refusal through `whole`
  (`cross.rs:150`). A refused root slack is `MeetFault::Unsettled(Meter)` over the whole span, not
  the root ± its slack that U3 §a.2 asked for. Only `place` (`cross.rs:368`) and a `Tangent` past the
  span (`cross.rs:170`) answer local windows.
- **The certified subdivision stops at its first refusal** (`half_angle.rs:685`, `subdivision`), and
  walks the whole turn: a refused piece the arc never reaches refuses the arc. This is
  `root-slack-meters-roots-outside-the-arc` and
  `a-circle-touching-a-torus-off-the-arc-escalates-the-arcs-side-decision`.
- **The enclosure cull never moved.** U3 §a.2 put `conic_clearance` into the pose as `Apart`'s cull
  stage in U3a. It is still in `reduce.rs`, filed as `the-curve-face-pose-has-no-enclosure-cull-stage`.
  It charges rounding (`geom_core::rounding_charge`, and `conic_residual_extremes`' charges), which
  `[simple.conditioned]` forbids at `f64`. Its extremes `c₀ ± swing` cancel for a far-centred conic
  (the harmonic belly). It cannot move as it stands; U4b re-poses it.
- **The wrappers lose the `Tangent`.** `line_quadric_roots` (`line_quadric.rs:128`) turns a decided
  touch into `Unsettled::Tangency`. The curved lane then re-localizes it with `off_face`, while the
  planar lane splits at the same touch (`plane_crossing.rs:161`, the graze net). The two lanes read one
  pose two ways, and the merge reads the `Touch` (§2.3).
- **`off_face` is the face reading, run on clusters of its own.** Its ball rule (`ball_off_face`,
  `edge_clear_of_ball`, `reach`, `distance`) is C5's window rule. Its `clusters` are a second method
  for where the span meets the carrier, which the parent row's Owed section names as a fallback. With
  local windows the balls are the windows', and `clusters` goes.
- **`factored_crossing` hands its declines back.** C5: "where it declines (at a node, or for a conic in a
  bisector plane) it refuses typed and never hands the span back to the quadric door" (PR 7167). The
  code returns `Ok(None)` and the caller reaches the quadric door. U4c makes the code match C5.
- **The spline arms need certification arithmetic.** `cross` is `T: Decide`. The arms are reached
  through `NurbsLane<T>` (`certify.rs`) and topo's `AtRestPolicy::spline_face_crossings` and
  `spline_chart_meetings` (`props.rs:7246`, `:7259`), which answer `None` at a dual. How `cross` reaches
  them is the fork (§Fork for Ev).

## 1.3 Consumers U3's table missed
U3's table (§a.4) gave U4 `plane_crossing_lane` (the sweep, `slit`, `factored_crossing`, `offer_rows`),
`line_quadric_roots` and the conic and torus wrappers in `wall_crossing`. The code also shows these:
1. **The planar lane's line formula** (`reduce.rs:1452`): a crossing found outside `cross`, from offsets
   taken at the vertices (the far-frame offset's class).
2. **`carrier_touch::off_face`'s three call sites** (`reduce.rs:4817`, `:5131`, `:5197`) and
   `SpanVerdict::OffFace`. U3 §7 named `off_face`, but not that it is the window rule.
3. **`conic_clearance` and `bool_line_cylinder_clearance`**: one was U3a's and did not land, the other
   was deferred to U4.
4. **The sweep's spline arms** (`spline_plane_crossings`, `composite_count`, `spline_wall_event`,
   `spline_interior`), which read the arms directly. Open as
   `cross-answers-a-spline-arm-with-a-whole-span-window`.
5. **`slit`'s curved `Lies` readings**: `census::line_on_quadric_carrier` (`census.rs:4518`) and the
   sampled `Variety::read_piece` for a conic on a quadric or torus. Both are second predicates for the
   pose's `Lies` (D3, "one reading per question"). The census's own read of `line_on_quadric_carrier`
   (`census.rs:4707`, `pm_census_ef_curved_residual`) is U5's.
6. **`factored_crossing`'s line root** (`factored_crossing.rs:125`), a second plane × line closed form.
7. **The refusal mappers and the public `topo::ConicRootFault`** (`lib.rs:1608`).
8. **Test rows** that call the wrappers directly (`reduce.rs:7276`, `:9458`, `:9556`, `:9690`).

# 2. Local windows (the prerequisite)
## 2.1 What a window is
A window is `curve_face::Undecided<T, W> { window: (lo, hi), why: W }` (`curve_face.rs:282`, unchanged):
an interval of the span's parameter outside which the reading has decided everything, with why it did
not decide inside. A `cross` answer with windows obeys these rules. Each is a claim a reviewer falsifies.
- **W1, exact outside.** Every meeting of the span with the surface whose parameter lies outside every
  window is in `found`, placed as `Crossings::meetings` would place it. No found meeting lies inside a
  window.
- **W2, never understated.** A window holds every parameter of the span where a meeting the arm did
  not read can lie, widened by the band's escalation `Kε` (metered at the span's least speed), which is
  what `undecided_over` already assumes of a window (`curve_face.rs:386`).
- **W3, on the span.** `t0 ≤ lo < hi ≤ t1` (in the span's own order, on a conic within the turn about
  the span's middle). A stretch that misses the span is no window: it is dropped, with the meetings it
  could hold. Whether it misses is decided as `undecided_over` decides a run past a window
  (`curve_face_window`), and a gap in band keeps it.
- **W4, disjoint and ascending.** Two windows that overlap, or touch within the band, are one window
  over their hull. Its why is the first one a reader must act on: a fault (`CrossWhy::fault`) ahead of
  anything else, otherwise the earlier window's. A fault is never merged away.
- **W5, local.** A window is *local* when its interval is a proper part of the span, given by the arm's
  own data. An arm with nothing local answers the span's whole interval, as today.

## 2.2 What `pose` and `cross` return
- `pose(span, surface, band) -> Result<Pose<T>, Undecided<T, PoseRefusal>>`. `PoseRefusal` keeps its
  variants and stays `Copy`. Every reader that matched the bare refusal reads `.why` (trace, census,
  `ring_path`, `slit`, classify, `region`). The `PoseRefusal` doc line "each is one window over the whole
  span" becomes the rung list below.
- `cross` keeps its signature and answer types. `CrossRefusal::windows` obeys §2.1. A pose refusal is
  the pose's window, its why `CrossWhy::Pose(why)`.
- **The methods return what they found and where they could not read.** The doors' methods
  (`line_method`, `conic_quadric_method`, `torus_method`, `spiric_plane_method`, `through_the_apex`)
  answer one shape, `Walked<T> { roots: Vec<T>, windows: Vec<((T, T), MeetFault)> }`, in place of
  `Result<Result<CircleRoots, Unsettled>, DoorRefusal>`. A kernel fault stays an `Err`, read by `cross`
  as `MeetFault::Contradiction`. `cross` places the roots (`place`) and merges every window (W4).

**Which refusals are local.** Each window below is the arm's own data, never a second reading.

| Source | Window |
|---|---|
| a graze rung in band: `BellyGraze`, `Discriminant` (line × wall, line × sphere), `Extreme` (first harmonic), `Kappa` | the graze's reach at `Kε` about its extreme, `pose::arc` at `Kε` (`pose.rs:431`), clipped to the span. In each of these rungs a rung run before it has excluded `Lies`, so outside the reach the residual is decided past `Kε` and holds no meeting. A reach that definitely misses the span is `Apart`, as `reaches` decides today for the zero band (`pose.rs:473`); in band it keeps the window. |
| `Apex`, `ApexSpread` | the singular point's spread (`Singular::spread`) |
| a root whose slack the meter refuses | the root ± its slack, clipped |
| a walk's refused piece: `Touch`, `Tangency`, `Floor` | the piece, clipped; a piece that misses the arc is dropped (W3) |
| `place`'s `End`, `Together`, `Apart` | as today (`cross.rs:368`) |
| a `Tangent` past the span | as today (`cross.rs:170`) |
| `PlaneParallel`, `AxisParallel`, `Coaxial`, `Noise`, `NotLies`, `ConeLead`, `Meridian`, `Contour`, `SpiricRegime`, a pose `Meter`, `Enclosed`, `Unbuilt`, `Torus` | whole span: each decides the class over the whole span |
| `Exhausted`, `Resolvent`, `CountDisagrees`, `Degenerate`, the line × torus quartic ladder's refusals | whole span: nothing local |

**The walk reads past what it refuses.** `half_angle::subdivision` keeps walking after a refused piece,
records it, and answers its roots and refused pieces. A piece wholly off the arc decides nothing the
caller reads, and is neither walked to the floor nor a window (W3). The turn's parity and degree checks
(`RootFault::ImpossibleCount`) read only a walk with no refused piece: with a window, the count inside
it is unknown, and the check is skipped. A parity check that runs with a window open would read the
unknown count as a contradiction, failing loud on data. The same holds for `ellipse_torus`'s
`torus_walk` and for every caller of `certified_subdivision` and `certified_subdivision_touching`.

**The touch arc.** A local window at a graze rests on `pose::arc` never understating the reach.
`arc`'s harmonic branch reads `acos(1 − z/bend)` (`pose.rs:437`), which loses digits as `z/bend` falls
towards the rounding: at `z/bend = 1e-12` the subtraction keeps four of `z/bend`'s digits. U4a re-poses
it as `2·asin(√(z/(2·bend)))`, which has the same value with no subtraction, before any window rests
on it.

## 2.3 How a consumer reads a window against its trim (the face reading)
C5: "the face's interior is `cross`'s answer read through the trim, every time … A window whose region
on the surface the face's own placement certifies wholly outside the face, with no boundary of the face
reaching it, holds no point of the face; any other window refuses the face reading, naming the window
and its why." U4b builds this once in topo, as `face_crossings(y, face, span, answer, band)`. It lives
in `boolean/face_reading.rs`, and moves to `topo`'s root if a reader outside the boolean takes it.
- **A found meeting** is placed by `contain::placement_on_carrier` (`contain.rs:844`), the one door
  for every face kind (on a plane its arm is `contfp_decided`). On a cone it is told off the far nappe
  first (`place_crossing`, `reduce.rs:5047`).
- **A window** holds no point of the face only when every meeting it can hold is off the face. Its
  meetings are localized into clusters by bisection, as `off_face`'s clusters localized a span. Let
  `m` be a piece's middle, `ℓ` its reach along the carrier at the carrier's speed bound
  (`bisect::reach`), and `(dist, foot, n)` the surface's distance, foot and unit normal at `C(m)`
  (`face_reading::distance`). A piece holds no meeting within `escalate` where the larger of two
  lower bounds on its distance, less `escalate`, decides positive (`curve_face_window_piece_clear`,
  in metres):
  - `|dist| − ℓ`, the distance being 1-Lipschitz;
  - where the piece keeps clear of the carrier's medial axis (`curve_face_window_piece_off_axis`),
    the second-order `|dist| − ℓ·|g| − ℓ²·H/2`, `g` the distance's derivative along the curve and `H`
    `1/(ρ − |dist| − ℓ)` plus the curve's own curvature.

  A piece neither clears is halved down to the tangency floor of the carrier's reach
  (`bisect::tangency_floor`); the pieces left there run together into clusters, at most eight (the
  most meetings a line or a conic has with these carriers), within the walk's piece budget. A
  cluster's meetings lie within `ℓ + |dist|` of its foot; that radius, widened by `escalate`, must
  decide below the carrier's reach ρ, and `ball_off_face` then needs the foot placed `Out` and every
  boundary vertex and edge decided clear of the ball. A window with a cluster whose ball does not
  clear, or that does not localize, refuses the reading. The clusters are what read a window the
  arm's method answers whole, a circle × torus contour rung's whole-span refusal among them
  (`mate7a_torus_rest`'s G1 chain). A refused window's why is read once for the lane
  (`reduce::read_why`): a fault fails loud, an apex reading is the apex's before any zero-band
  reading, a pose rung to its coincidence lever, a method's fault by `MeetFault::kind`. A kind with
  no distance and foot (a cone, a spline face) refuses every window, as `off_face` does today.
- **A window whose why is a fault** fails loud (`BooleanError::ClassificationInvariant`) and is never
  set aside.
- **A touch** (`MeetingKind::Touch { arc }`) is one meeting, the in-band arc, so the trim reads the
  arc as a window, not the point alone. A touch at an end of the span is the end vertex's at its
  point, but its arc's stretch inside the span is the span's, read the same way. If the ball is off the face there is no event.
  Otherwise the lane's touch treatment applies: on a plane the graze net splits at the touch point, as
  today; on a curved face the typed frontier (`CurvedPierceUnsupported`) stands, as today. U4 widens
  neither.
- **The ends.** A meeting at an `End` is the end vertex's: the vertex rows own it (C5, "dropping a root
  at an end of the span is the consumer's choice"). The vertex sides stay the sweep's record-aware
  readings (`vertex_face_side`, `bool_vertex_face_side`). Where they straddle past the band, and `cross`
  answers no interior crossing, no window and no end hit, two readings of one fact disagree:
  `MeetFault::Contradiction`, failing loud, as U3b's classify rule does (`classify.rs:401`).

# U4a — local windows (PR 1)
**Scope.**
- In `curve_face/{pose,cross}.rs`: `pose` refuses `Undecided<T, PoseRefusal>`, with the graze, apex and
  slack rungs local (§2.2). `pose::arc`'s harmonic branch and `plane_conic`'s near-graze roots are
  re-posed (standing claim 1).
- In `half_angle.rs`: the subdivision reads past refused pieces, drops pieces off the arc, and skips the
  parity and degree checks when a window is open.
- The doors' methods answer `Walked`, and `cross` merges windows (W4).
- Readers of `pose` read `.why`. The wrappers map back to their old shapes from the first window that
  meets the span, so the sweep's behaviour is unchanged, except that a refusal off the arc no longer
  refuses.
- C5's sentence "a closed form's unsettled count is one window over the whole span too" becomes the
  §2.2 table, as an approved code change rewording what it describes (CLAUDE.md, the ratification test).

**Retires**: the whole-span mapping of method refusals (`cross.rs:150`'s `whole` for methods); the
subdivision's first-refusal return; `acos(1 − z/bend)` and `acos(−d₀/R)`.
**Closes**: `root-slack-meters-roots-outside-the-arc`,
`a-circle-touching-a-torus-off-the-arc-escalates-the-arcs-side-decision`, and the window-placement
item of `cross-owes-census-window-and-conic-touch-arc-rows`.

**Claims a reviewer must falsify.**
- W1–W5 hold for every arm, on every source in §2.2's table.
- A graze rung's reach encloses every root of the span that the in-band reading leaves open. Falsify it
  against the exact oracle at grazes across the gap, not at the zero band alone.
- The walk reading past a refusal never answers a root inside a refused piece, and never skips a piece
  the arc reaches.
- The parity and degree checks never run while a window is open, and still run on every walk with none.
- No `PoseRefusal` reader dropped a case when it moved to `.why`, and the trace's and census's tables
  are unchanged.
- The wrappers change the sweep's answers only where a refused stretch misses the arc.

**Rows owed, each with the mutant that reds it.**

| Row | Mutant that reds it |
|---|---|
| a graze in band on each graze rung (plane belly, line × wall, line × sphere, first-harmonic extreme, `Kappa`) answers a local window holding every exact root | the rung's window widened to the span; the reach read at `ε` instead of `Kε` |
| graze-window fuzz: random grazes across the gap; every exact root (`conic_oracle::exact`, double-double) outside the windows is found, and none found inside one | the walk drops a refused piece's window; `cross` keeps a found meeting inside a window |
| the measured circle × torus case (`circle_torus_roots(&ring, 1.0, 2.0, …)`, the touch at `θ = 0`) and the root-slack frustum case answer with no window | a window that misses the arc is kept |
| a turn with one touch window and one decided crossing answers its window and the crossing, with no fault | the parity check runs with a window open |
| a refused root slack answers the root ± its slack | the slack window widened to the span |
| `arc`'s harmonic branch against `2·asin(√(z/2b))` in double-double, `z/b` from `1e-4` down to `1e-16` | `acos(1 − z/b)` restored |
| a circle crossing a plane at depths from `1e-4·R` down to `10 Kε` below its belly: each root within the band of the double-double root | `acos(−d₀/R)` restored |
| two overlapping windows, one a fault, answer one window whose why is the fault | the merge keeps the earlier why |
| wrapper parity: `in_band_sweep`, ONEDOOR and the sweep's rows unchanged | a wrapper reads every window, not the first that meets the span |

**Goldens and pins.** None expected. A corpus row that stops refusing because its refusal was off the
arc is judged by its answer against the exact oracle, never by the old refusal. The `K` names are
unchanged, and any new name (`curve_face_window_*`) is listed in `DOOR_K_PREFIXES`' table row.

**Letter H, tier dual.** H: it changes the certified walk's control flow and the soundness of every
window. Dual: it fixes the answer shape U4b–U4d and U5 build on, which is hard to change once they do.

# U4b — the face reading, and the curved lane onto `cross` (PR 2)
**Scope.**
- `face_crossings` (§2.3) is built from `off_face`'s ball rule, and `clusters` goes.
- `wall_crossing` reads `cross` on the edge's span and `face_crossings`. `SpanVerdict` becomes a
  reading of that answer: `Pierce`, `NoInterior`, `Elsewhere`, `LiesOn` (`Lies`), `Miss` (`Apart`),
  `AtApex` (an apex window: a spread or an apex the band cannot call), and `OffFace` (every window
  read off the face). A `Through` pose's apex is a meeting like any other: `face_crossings` places it
  in span order among the others, so a span through a cone's apex pierces the face there or crosses
  off it, and is not refused.
- The covered arm reads its posture off `cross`'s whole answer (`reduce::covered_posture`), every
  window and every found meeting by the lane's one reading: a crossing found inside the span, or a
  limit, an apex or a non-ring torus in any window, is the frontier or the face's refusal; a pose
  rung in its gap the coincidence refusal; and only windows that are grazes, method signs in band or
  tangencies the method could not settle the zero posture, whose covered rung reads the ends and
  defers the interior (`settle_deferred`).
- The cull moves into the pose as `Apart`'s first stage on the conic arms
  (`curve_face_pose_clearance`), re-posed as standing claim 1 requires. In band it hands on and never
  refuses alone, and `conic_clearance` goes. The re-posed cull is a ball's: the whole conic lies in
  the ball about its centre `m` (the midpoint of two antipodal running points) of its larger
  semi-axis `ℓ`, and each kind has a signed function `g`, `L`-Lipschitz with the residual's sign
  (`|p − S| − R`, `ρ − R`, `ρ cos α − |h| sin α`, `√((ρ − R)² + h²) − r`), so `|g(m)| − L·ℓ −
  escalate` decided past the band is `Apart`. It reads no harmonic extremes and charges nothing:
  `g(m)` is read from `m − origin` (an axis's offset as `|e × n|`). The question is past `Kε`, not
  past zero, because `Apart` claims the conic stands past `Kε` (W2), as the face reading's piece rule
  asks (§2.3). Asked past zero, a 363-m circle's rounding read a graze at 0.999·Kε past Kε. Asked past
  `Kε`, the session box bounds the bound's rounding to about 7.3e-12 m (sixteen ulps of its 3.46-km
  diagonal), against `Kε` = 1e-11 at
  ε = 1e-12. The torus arm asks the ring convention before it.
- `bool_line_cylinder_clearance` goes: the pose's discriminant decides that line's class exactly.
- The straddle arm's `NoInterior` with no window becomes `Contradiction` (§2.3).
- The walk does not drop a piece wholly off the arc at its split (§2.2): it walks such a piece until
  it settles or refuses, as U4a does. `cross` reads only the meetings and windows on the span
  (`on_span`), so a piece off the arc costs its walk and never its answer; dropping it unwalked would
  move the walk's refusals with no reader to gain.

**Retires**: `line_quadric_roots`, `conic_quadric_roots`, `circle_torus_roots`, `ellipse_torus_roots`
from the API (each stays the door rows' test reader, `cfg(test)`); `carrier_touch::off_face` and `clusters`; `composite_count`'s and
`wall_crossing`'s `off_face` hooks; `conic_clearance`; `bool_conic_curved_clearance`;
`bool_line_cylinder_clearance`; `of_line_quadric_refusal`, `of_conic_quadric_refusal` and
`of_torus_door_refusal`, replaced by one map from `CrossWhy`; the test rows' direct wrapper calls,
re-aimed at `cross`.
**Closes**: `the-curve-face-pose-has-no-enclosure-cull-stage`,
`a-straddling-span-whose-roots-do-not-settle-keeps-its-door-off-the-face`, and
`splice/the-analytic-crossing-doors-are-not-dispatched-through-the-curve-face-table` for the curved
lane.

**Gate before review: every rescue still rescued.** Instrument `off_face` on main: log every call that
answers `true`, with its arm and refusal, across the suite at every ε row and the corpus. Each logged
pair must answer `OffFace` through `face_crossings` on U4b's head. A pair that `off_face` rescued from a
whole-span refusal (the line × torus quartic, `Exhausted`, `CountDisagrees`) and that U4a gave nothing
local is a regression. U4b stops on it. If the only way to give that arm a local window is to change the
arm's fixed method (for line × torus, a certified walk in place of the quartic ladder), that is a
change to C5's fixed method per arm, and goes to Ev as an `[ev]` question on U4b's PR.

**Claims a reviewer must falsify.**
- `face_crossings` decides no window `OffFace` that holds a point of the face. Reach for a window whose
  ball touches a boundary edge, a seam, or the far nappe.
- The sweep's events are unchanged where `cross` answers without windows, apart from rows the PR body
  lists, each judged.
- The cull decides `Apart` only where the arc is past the band. Test it at far centres, and at a cone,
  whose form bounds the residual only from below.
- Every pair `bool_line_cylinder_clearance` cleared, the pose answers `Apart` or meetings past the span.
- The covered arms' deferrals are as before.

**Rows owed.**

| Row | Mutant that reds it |
|---|---|
| a window off the face, its ball clear of the boundary, answers no event | the boundary check skipped |
| a window whose ball meets a boundary edge refuses, naming the window's decision | as above |
| a window on a cone face refuses | a cone given a distance |
| a window whose ball radius reads in band against ρ refuses | the gap read as fitting |
| a touch whose point is `Out` but whose arc reaches the boundary refuses | the touch read at its point |
| the rescue list (gate above), each `OffFace` | `face_crossings` reads every window as refusing |
| a definite clearance answers `Apart` where the closed rungs escalate | the cull skipped |
| a circle 1e6 m from its sphere's centre, 10 `Kε` clear, against a double-double oracle | the cull reads `c₀ − swing` at the far centre |
| large circles grazing a small wall inside `Kε`, against a double-double oracle, never culled | the question asked past zero |
| the line-clearance fuzz: line × wall and line × sphere, both endpoint residuals positive, the answer against the closed-form quadratic in double-double | the pose's discriminant read on the endpoint chord |
| a straddle with no interior crossing fails loud | the contradiction read as the frontier |

**Goldens and pins.**
- `scripts/gates/any-climb-pins.counts` (`off_face`, `ball_off_face`) and `scripts/gates/bounds-allowlist.sh`
  (the `Bounds` counts riding to `off_face`) move with the code.
- The k-lint baseline loses `bool_conic_curved_clearance` and `bool_line_cylinder_clearance`, and gains
  `curve_face_pose_clearance`; re-derive it per the K-REPORT runbook.
- Refusal-text pins whose decision moves from a wrapper's to `CrossWhy`'s are judged by whether the new
  lever is the decision's own.
- Corpus goldens (`perf12_census_goldens`, `bool13_goldens`, `revalue_corpus`): no outcome, count or
  verdict may move without an oracle that says the new one is right. A split point that moves by ulps
  is judged as U3b's was: the new value against the exact crossing.

**Letter H, tier dual.** H: it changes the structural act on every curved pair, and retires a localizer
whose rescues must each be shown kept. Dual: the face reading is the rule every later consumer of
`cross` against a trimmed face reads.

# U4c — the planar lane, `slit`, `factored_crossing`, `offer_rows` (PR 3)
**Scope.**
- The planar lane reads `cross` and `face_crossings` for a line and a conic alike, and the line
  formula (`reduce.rs:1452`) goes. With both lanes reading one answer, the `face_plane` gate
  (`reduce.rs:992`) stops choosing a method: one edge × face event function matches on the answer.
  Only the vertex rows (the plane's vertex-section rule, the curved face's `vertex_on_curved_face`) and
  the touch treatment (§2.3) differ by face kind.
- `slit::conic_or_line_on` reads `pose` for every built analytic pair (`Lies` is on; `Apart`,
  `Tangent`, `Through` and `Isolated` are off; in band escalates). `line_on_quadric_carrier` and the
  sampled `Variety` arms go from `slit`. A spline or spiric against a curved face keeps
  `clears_plane`/`spiric_on_curved` until U4d.
- `factored_crossing` reads `cross` against `Surface::Plane` built from each bisector. Its line root
  comes from that `cross`. Every decline that is not "no such pair" refuses typed: a node is
  `NodeUnbuilt`, and a conic in a bisector plane names the plane (C5).
- `offer_rows`' probe reads `cross`.

**Retires**: `plane_crossing_lane`, `PlaneCrossingLane`, `ConicPlaneMeet`, `ConicRootFault` and its
`topo` re-export; `of_conic_root`; the plane wrapper's two `unreachable!`s; `slit`'s
`unreachable!` (`slit.rs:1459`), now a typed refusal; `factored_crossing`'s `Ok(None)` hand-backs.
**Closes**: `splice/the-analytic-crossing-doors-are-not-dispatched-through-the-curve-face-table`, and
the parent row's "dispatch is an Option gate, not a match".

**Claims a reviewer must falsify.**
- The line's crossing is placed where the plane crosses the carrier, at a far frame and a small tilt,
  as well as the interpolation placed it or better.
- `slit` reads `Lies` where the samples did, and reads `Lies` nowhere the pose does not.
- A `factored_crossing` decline never reaches the quadric door.
- The one event function changes no curved-lane answer U4b fixed.
- Removing the public `ConicRootFault` breaks no root the gate does not build. Run clippy in
  `demos/tour` and `demos/wild`.

**Rows owed.**

| Row | Mutant that reds it |
|---|---|
| a line 1e6 m from the origin crossing a plane at 1e-6 rad, split within the band of the double-double crossing | the interpolation restored |
| two conic roots within the band of each other in the face's trim refuse (C5's window), where the planar lane used to insert one | `Together` inserted once |
| a planar graze in the trim is split at the touch point, as before | the planar touch treated as the curved frontier |
| a conic in a bisector plane refuses typed | the decline returns `Ok(None)` |
| `slit` reads a circle on a sphere, a ruling on a wall and a generator on a cone as on, and a circle near (in band of) a torus escalates | `slit` reads its own samples |
| a spiric pose refusal other than in band is a typed refusal in `slit` | the `unreachable!` restored |

**Goldens and pins.** Planar split points move by ulps where the line's crossing moves from the
interpolation to the pose's root; judge each against the exact crossing. The two `Together` cases
change from a split to a refusal. That is C5's rule; the PR body lists each case with its geometry.
Executed offers in `offer_rows` keep their keys, and a decision that renames is judged by its lever.

**Letter M, tier single FULL.** M: it applies U4b's reading to the planar lane, and the hard rule is
already built. FULL: the line formula's replacement, the typed declines and `slit`'s `Lies` are
correctness claims, not reading.

# U4d — the spline arms through `cross` (PR 4, after the fork)
**Scope.**
- `cross` reads `SplineComposite` and `SplineFace` by their enclosures, through whatever the fork's
  answer gives it.
- Their windows are the arm's own: `composite`'s in-band pieces and creases, and `spline_face`'s
  undecided floor boxes, each read past as U4a's walk is (W1–W5).
- The sweep's `spline_plane_crossings`, `composite_count`, `spline_wall_event` and `spline_interior`
  read `cross` and `face_crossings`.
- The trace and the census read spline pairs, and classify reads a spline edge through `cross`.

**Retires**: the sweep's direct `T::spline_face_crossings` and `T::spline_chart_meetings` reads;
`PoseRefusal::Enclosed` as a whole-span window.
**Closes**: `cross-answers-a-spline-arm-with-a-whole-span-window`;
`parting/the-plane-split-reads-no-crossing-of-a-spline-edge`, if classify's spline read lands here.

**Claims a reviewer must falsify.**
- No spline answer that is definite today becomes a refusal, except where a window holds a point of the
  face.
- A refused piece or floor box is a window that encloses every meeting it can hold, a crease's included.
- Whatever the fork answers, no scalar reaches a spline arm without a sentence saying what its answer
  means there (DESIGN `[unrepresentable]`).

**Rows owed**: a spline span across a plane and across a wall, through `cross`; a spline span against a
spline face, through `cross`; a refused piece read off the face answers no event, and one in the trim
refuses; the trace and the census serve a spline pair. The mutants: an in-band piece read as clear; a
window dropped; the dual's refusal skipped.

**Goldens and pins**: the spline rows' answers, unchanged unless a window was read off the face; the
`AtRestPolicy` method counts in the gates, if those methods go.

**Letter H, tier dual.** H: it reads the certified enclosures past their refusals. Dual: it sets how a
scalar's certification reaches the one reading, which every reader of `cross` inherits.

# Standing claims (every stage)
Every review of every stage carries these five, with the stage's instances.
1. **No formula that cancels.** Re-pose rather than charge (`[simple.conditioned]`). The classes the
   last units' reviews found are the far-frame offset (PR 7411), the harmonic belly
   (`the-plane-arms-harmonic-amplitude-rounds-at-a-far-centred-conics-reach`) and the `P ∩ Q` line at
   small tilt (PR 7431). U4's instances:
   - `pose::arc`'s `acos(1 − z/bend)` (U4a);
   - `pose::plane_conic`'s roots `φ ± acos(−d₀/R)` (`pose.rs:663`), which lose digits near a graze,
     where `−d₀/R` nears 1: re-posed from the belly's own depth (U4a, unless the harmonic-belly lane
     has re-posed it first);
   - the cull's `c₀ ± swing` at a far centre (U4b);
   - each window ball's `dist(m)` and foot per kind (U4b);
   - `factored_crossing`'s `plane_offset(origin, n, centre)/dir·n` at a far line origin (U4c, read from
     the span's middle);
   - any new spiric window, which reads `P ∩ Q` in the spiric's own plane (U4a).
   A charge, pad or estimate of rounding added at `f64` is a defect, whatever it passes.
2. **Oracles independent of the implementation.** An oracle is computed from stored data with its own
   arithmetic: `conic_oracle::exact` in double-double, closed-form roots in double-double, a dense
   sample of the distance. Never `place`, `arc`, the pose's own forms, or `off_face`'s clusters.
3. **A gap inside the band reads in band.** It never reads as a definite answer:
   - a window end in the gap keeps the window;
   - a ball radius in the gap against ρ refuses;
   - a graze reach whose overlap with the span is in the gap keeps the window, never `Apart`;
   - the cull in its gap hands on.
4. **Faults fail loud; data refuses typed.** Never `unreachable!` on data a literal can build. In U4's
   ground:
   - `cross.rs:425` (`curve_face_cross_order` reading roots apart past the band as level);
   - `carrier_touch.rs:361`;
   - `pose.rs:732` and `:1690`;
   - `spiric_plane.rs:142`;
   - `line_quadric.rs:343`, `:603` and `:841` (their arms change in U4a);
   - `slit.rs:1459`.
   Each is either argued unreachable from every literal in the PR body, or made a typed refusal. A fault
   window is never set aside.
5. **Every new or changed code path has a row that a named mutant reds.** The PR body lists each path,
   its row and its mutant, and says which mutants survived and why.

# Tracker rows U4 touches
- **Closed by U4**: those named under each stage.
- **Touched, not closed**:
  - `line-roots-carry-no-root-slack-meter` and
    `the-edge-sweeps-line-wall-and-line-sphere-roots-carry-no-root-slack`: a wall or sphere root with
    no meter has no slack window, and keeps its whole-span refusal.
  - `boolean-in-span-readings-of-grazing-roots-are-levered-by-arc-length`.
  - `a-villarceau-circle-lying-on-a-torus-is-unsettled` (`NotLies` stays whole-span).
  - `splitdoor/a-conics-belly-graze-on-a-plane-is-not-read-against-the-face`: after U4a and U4c, a belly
    graze in band is a local window that `face_crossings` reads; it closes there if its row reads off
    the face.
  - `the-off-face-touch-decisions-have-uncalibrated-margins`: its four decisions become the face
    reading's.
  - `orbit/a-rod-touching-a-big-donut-off-its-face-stops-at-the-extent-scan-at-1e-12`: U4b's gate
    measures it.
- **Not U4's**: U5's ray lane (`line_wall_roots`, `line_sphere_roots`, `line_cone_roots`,
  `solid_contain::cast_ray`); `ring_path::plane_roots`' route through the co-surface arm; the census's
  `pm_census_ef_curved_residual`; the general A ∩ B rule
  (`a-span-on-one-surface-may-cross-another-through-their-section`).

# Territory
From `python3 scripts/work.py territory --files -` on each stage's files:
- **U4a**: `curve_face/{cross,pose,line_quadric,ellipse_torus,plane_crossing,spiric_plane}.rs` are
  SPLICE's, and `circle_roots.rs`, `circle_torus.rs` and `conic_quadric.rs` are claimed by SPLICE and
  GERM. The `.why` move touches `boolean/trace.rs` (the boolean family's and TRACE's),
  `census/point.rs` (ATREST's, HELDRECORDS'), `splitting/classify.rs` (the splitting family's and
  GAUGE's) and `ring_path.rs` (shared with ONEHOME and ONEROUND).
- **U4b**:
  - `boolean/{mod,refusal_routes,r2_probes,carrier_touch_rows}.rs` belong to the boolean family:
    CLEAVETAIL, HELDRECORDS, HONE, HONEONE, ONECOPY, ONEDOOR, ONEHOME, PARTFRAMES, PARTING, SEAL,
    SPLITDOOR and TANG.
  - `reduce.rs` and `carrier_touch.rs` are claimed by that family and GERM; `reduce.rs` by SPLICE too.
  - `contain.rs` is INSIDE's and REACHHOLD's.
  - `pose.rs` is SPLICE's.
  - `scripts/gates/{any-climb-pins.counts,bounds-allowlist.sh}` are CIW's.
- **U4c**: `boolean/{slit,factored_crossing,node,offer_rows,refusal_routes}.rs` belong to the boolean
  family; `reduce.rs` as in U4b; `census.rs` is ATREST's, CONTACTHOLD's, HELDRECORDS' and ONEDOOR's;
  `plane_crossing.rs` is SPLICE's.
- **U4d**: `curve_face/{composite,spline_face,cross}.rs` are SPLICE's; `certify.rs` is PARTFRAMES' and
  SELECT's; `props.rs` is BOXFINISH's, MEASUREDOOR's and TALLY's; `trace.rs`, `census/point.rs` and
  `classify.rs` are as in U4a.

GERM has carried each of these files' curve × face code since U1. The crossings are routine, except
`reduce.rs`, where ONEROUND's dispatched `an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre`
and SPLICE's rows also edit. Each stage merges main before review, and its PR names the rows it
overlaps.

## The spline arms' lane
`curve_face::cross` is generic over every deciding scalar, the dual included. Its spline arms need a certifying lane.

**At every scalar that certifies, the lane is read off the scalar.**
- The scalar's NURBS lane comes from a sealed per-scalar trait in geom-brep. `AtRestPolicy` takes that trait as a supertrait.
- `NurbsLaneOf` carries the two arm readings.
- `cross` takes no lane argument.
- `AtRestPolicy::spline_face_crossings`, `spline_chart_meetings`, the `NurbsLane` holder and `classify`'s fallback go. Every consumer reads `cross` alone.

**At a dual, a spline arm refuses with a typed not-implemented error.**
- The error is a variant of its own, naming the arm and the scalar, so the cases are found by type later.
- It is not a window that claims the scalar has no meaning there. A spline crossing at a dual does mean something: the certified search runs on the dual's value channel, so its decisions are the f64 run's bit for bit. Each admitted root then carries its tangent by the implicit-function theorem, `dt/dp = −(∂F/∂p)/(∂F/∂t)`, or `−J⁻¹·∂R/∂p` with the Jacobian the Krawczyk test already proves invertible.
- That lift is filed as its own row. It needs every value channel stated `Certifies`, and a dual half of the admit door (DL5.2's sentence on the dual changes with it).

**Why not exclude the dual by type:** the carrier's kind is run-time data in a `Body<T>`. A dual body holding spline walls is meaningful (`loft_prism` builds at `Dual64`), and booleans run at a dual for sensitivities.

**C5** gains one sentence: at a dual, a spline arm is not yet built, and refuses typed as such.
