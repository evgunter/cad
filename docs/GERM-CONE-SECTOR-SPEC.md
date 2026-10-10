# GERM cone sector — the boolean's sector, pierce and join arms for a cone face (unit spec)

This spec carries `work/germ/boolean-sector-algebra-has-no-cone-arm.md`
(P1, H). It measures the whole chain of doors a cone operand meets past
the pair gate (§0), specifies each door's arm (§1), checks each against
the D10 hold (§2), and cuts the units (§3). Every unit here lands below
the operand gate and precedes the cone admission spec's U7 (`docs/doc-ledger/germ-verbs-cone-spec.md`) (the
roster flip). The spec is deleted when its last unit merges.

Read on `c333c6ac65`. Line numbers ride beside names and may rot.

## 0. Measured: the chain on main

**How.** A scratch tree on `c333c6ac65`, never committed and reverted:

- `Cone` on both rosters (`reduce::boolean_arm_exists` :199,
  `revert_arm_exists` :220);
- every `BooleanError::CurvedBooleanUnsupported` construction in
  `crates/topo/src/boolean/` wrapped to print its `file:line`, so each
  refusal below names its raising site rather than a theory of it;
- each door given a scratch arm behind an environment switch, so one
  binary peels the chain one door at a time, and a second switch turns
  any one arm back off to confirm that door stands where the chain says.

Every pose ran under ∪ and ∩ in both member orders and under both ∖
(six results per pose). The fixtures:

- **REACH's poses** (`sweep/tests/reach_cone_root_lane.rs`; frustums over
  `y ∈ [0, 1]`: `WIDENING` r 0.5→1, `NARROWING` r 1→0.5, `FULL` r 1→0):
  R1 a cube turned onto the widening wall; R2 the same on the narrowing
  wall; R3 a tilted rod across the narrowing wall; R4 a steep rod across
  the widening wall; R5 a coaxial rod through the top cap; R6 a thin
  brick past the full cone's apex; R7 a thin brick through the apex
  region below the widening frustum; R8 a brick cornered at the apex;
  R9 a brick grazing the full cone's wall.
- **TANG's pose** (`sweep/tests/a_ring_on_a_cone_face.rs`): T1, the π/6
  cone against the box turned −50°, at scale 0.6 and 0.1, as built.
- **Against a box:** B1 an axis-aligned brick across the widening wall;
  B2 a coaxial rod through both caps; B3 a tilted rod across the
  widening wall; B4 the axis-normal slab `y ∈ [0.3, 0.6]` across the
  full cone.
- **A plane cutting the widening frustum** (a 6 m half-space brick):
  C1 tilted 10° off axis-normal (an ellipse); C2 the plane `x = 0.3`,
  parallel to the axis (a hyperbola); C3 parallel to a generator (a
  parabola); C4 20° off the axis (a hyperbola).
- **The apex** of the full cone: X1 the plane `x = 0` through it (the
  generator pair); X2 a plane 10° off axis-normal touching it alone;
  X3 the same plane 0.05 below it (an ellipse round the apex).

**Every returned body was checked three ways:**

- its `mass_properties` volume against the op's truth from
  `V(cone)`, `V(other)` and `I = V(cone ∩ other)`, with `I` by Monte
  Carlo (the frustum analytic, the other operand by its own
  `point_in_solid`) and, where Monte Carlo was too coarse to judge, by a
  500³ grid integral outside the kernel;
- `validate_geometric` (tier 3);
- `point_in_solid` on the result at 1500 sampled points, against the
  same truth.

### 0.1 The door chain, per pose class

The doors in the order a pose meets them. **Bold** is where main stops
(rosters open, no other change); each later door was reached by giving
the ones before it a scratch arm, and each was confirmed by switching
its own arm back off with the rest left on.

| # | door | raising site (`c333c6ac65`) | payload |
|---|---|---|---|
| D1 | the sector's normal | `sectors::sector_face`'s `SectorCarrier::Cone` arm (`sectors.rs` :383) | `CurvedBooleanUnsupported { kind: Cone }` |
| D2 | the pierce normal | `vtxfac::classify_vertex_on_face` (:414) on `face_outward_normal_at`'s `Ok(None)` (`face_normal.rs` :275) | `CurvedBooleanUnsupported { kind: Cone }` |
| D3 | the germ pair's section frame | `join::pair_section_frame_at`'s `_ => NoArm` (`join.rs` :2012), raised at :1762 | `GermFrameUnsupported` |
| D4 | the germ-pair dispatch | `bool_connect`'s `no_arm` (`join.rs` :729, :808) | `CurvedBooleanUnsupported { kind: Cone }` |
| D5 | the planar side's chord | `chord_join::bool_planar_chord_spec`'s wall guard (`chord_join.rs` :1577) | `Join(SectionInvariant { "… neither a cylinder nor a sphere (arm not wired)" })` |
| D6 | the interior-loop guard | `ops::interior_loop_verdict` (:963, raised at :987): `section_cert::classify`'s `_ => Intractable` (:595) for every cone pair | `CurvedPairUnsupported { site: InteriorLoopGuard }` |
| D7 | the result door's volume | `props::face_flux` (:2387): a ring on a cone face | none since its arm landed (§1.7); `ResultInvalid { VolumeUncomputable { RingOnCurvedFace } }` before |

| class | poses | chain | end of the road with every door peeled |
|---|---|---|---|
| **plane × cone, ellipse or circle** | R1, R2, R7, B4, C1, X3, T1 | **D1** (cone-first orders) or **D2** (other-first) → D1/D2 → D3 → D4 → D5 → D6 → D7 | **bodies**, all correct (§0.2), ∪ and cone ∖ other included where the result keeps a ring on the cone face (R2, T1): D7's arm reads it (§1.7). |
| **plane × cone, hyperbola or parabola** | B1, C2, C3, C4 | **D1/D2** → D3, where `plane_cone_section` refuses the conic | the conic refusal (by decision, below) |
| **cone × cylinder, general pose** | R3, R4, B3 | **D1/D2** → D3, `(Cone, Cylinder)` has no frame | `GermFrameUnsupported`: `cone-pairs-in-general-pose-have-no-section-arm` |
| **coaxial, events on the caps only** | R5, B2 | **D6** first: no germ lands on the cone face | **bodies**, all correct, once D6 lets the cone pairs through |
| **the apex** | R8; R9; X1, X2; R6 | R8 `CrossingAtConeApex`, R9 `CurvedPierceUnsupported`, both in the sweep, by design. **X1, X2:** past D1, `Escalated { Corner(Straight) }` at `sector_straight`, the apex vertex's cone sector read with a poisoned normal. **R6:** past D1–D5, `Join(Escalated { chord_arc_leave_germ, MarginDiag::Invalid })` | typed refusals; X1, X2 and R6 refuse under the wrong name (§1.2, §1.6) |

**Corrections to the item's chain.** The item lists four sites:
`sector_face`, then `face_outward_normal_at`, then `bool_connect`'s
`no_arm`, then `bool_planar_chord_spec`. Measured:

- D1 and D2 are not ordered. Which one fires first depends on the member
  order, because the vertex-vertex lane and the vertex-on-face lane run in
  operand order.
- **D3 stands between D2 and D4.** `pair_section_frame_at` is read when
  germs are matched, before `bool_connect` dispatches the pair. TANG's
  "plane×cone germ lane" opened the frame and the dispatch together.
- **Two more doors stand behind the chord: D6 and D7.** D6 is the
  section certificate's cone rows (the cone admission spec's U4). It is
  decided before the join and raised after it, only where a body would be
  returned. So D6 is the last refusal on the crossings path, and it fires
  even when every join door is open.

### 0.2 Bodies returned: none is wrong

**No pose returned a body on main, rosters open or not.** With D1–D5
given scratch arms and the cone pairs let through D6 — the mutant the
certificate exists to stop — 51 bodies came back. All are tier-3 valid,
and none disagreed with the truth at any of the 1500 points. Every
volume matches:

| pose | ops answered | check |
|---|---|---|
| B4 slab | all six | ∩ = `0.29217`, the closed form `π·0.3/3·(0.49 + 0.28 + 0.16) = 0.093π` |
| R5 coaxial rod | all six | ∩ = `0.08482 = π·0.3²·0.3` exactly |
| B2 coaxial rod | all six | ∩ = `0.12566 = π·0.2²·1` exactly |
| T1, scale 0.6 | ∩ both orders, box ∖ cone | ∩ = `0.05715`; grid integral `0.05715` |
| T1, scale 0.1 | the same | ∩ = `2.65e-4`, the 0.6 result × (1/6)³: the pose is self-similar about the apex |
| R2 | ∩ both orders, cube ∖ cone | ∩ = `0.03859`; grid integral `0.03855` (the grid's error is about 1e-4) |
| R1, R7, C1, X3 | all six | within 1σ of Monte Carlo (σ ≤ 0.02 on C1, much smaller elsewhere) |

Two things the probe saw that are not wrong answers:

- `point_in_solid` on R2's result refused at 872 of the 1500 points,
  `PartialConeFace`. The ∩ leaves an apex-closed cone sector, and
  `cone_trimmed_window` (`solid_contain.rs` :1715) refuses it. That is
  the cone admission spec's U3 (`cone-apex-closure`).
- The ∪ and cone ∖ other results that keep a ring on the cone face
  refused at D7: built, then refused typed at the result door.

**What this does not cover.** These are 20 poses, not a search. In
particular, the cone admission spec's P3 is not among them (the edges
from nappe to nappe that `(Negative, Negative)` clears silently).
REACH's lane closed that arm for the cone (R6 is its pose here); its
certificate-row consequence is U4's to pin. The bypass also stopped
nothing else: the guard walked every pair, with `stop` off, and only
cone pairs were let through.

## 1. The arms, per door

**Notation.** The cone is `(A, â, α)`. For a point `p`, `q = p − A`,
`h = q·â` and `ρ = |q − hâ|`. The face's nappe is `sign h` (the chart's
`v`). The elevation is `f = ρ·cos α − |h|·sin α`
(`geom_brep::cone_elevation`). The chart normal is
`n = ŵ·cos α − â·sin α·sign h`, with `ŵ` the radial unit
(`geom_brep::implicit_gradient`).

### 1.1 D2: the pierce normal (`face_outward_normal_at`'s cone arm)

**Geometry.** The normal is the implicit gradient folded through the
face sense, as for the other curved kinds. **The cylinder arm's
certificate cannot be reused.** That arm certifies `p` on the chart by
`‖∇F‖ − 1`, which is zero exactly on a cylinder, sphere or torus. A
cone's `F` is a distance (the elevation), so `‖∇F‖ = 1` at **every**
point off the axis. Read that way, the margin is identically Zero and
certifies any point at all. The cone arm needs two margins of its own:

1. **On the face's nappe:** `cone_elevation(…, Some(nappe), p)`, decided
   Zero against the band, unlevered (it is metres). A definite sign is
   `OffSurface`, and a point on the mirror nappe reads definitely
   positive there. In band, the arm escalates `NormalDecision::OnSurface`.
2. **Off the apex:** `ρ` decided positive against the band. Zero or in
   band refuses, typed: a new `NormalAtError` arm for the apex (`p`
   is at the apex, where there is no tangent plane), carried through
   `of_pierced_normal`. It never returns a poisoned normal.

**The lever beside it** (`vtxfac.rs` :434). `side_code` charges a bound's
sagitta against `min_radius_of_curvature(pierced, p)`, which is `ρ(p)`
for a cone. `geom_brep` documents that as a bound **at `p` only**
(`implicit.rs` :374): the normal curvature `cos α·(d·φ̂)²/ρ` grows
toward the apex, along the generator. A bound leaving `p` toward the
axis meets a tighter bend than `ρ(p)` reports. **The cone's lever is
`max(ρ(p) − reach, 2ρ(p)/3)`**: the smallest `ρ` within the bound's
reach, or within the stretch the charge reads where that is shorter.
The charge reads no further than `slope·lever/2 ≤ ρ/3` at a lever of
`2ρ/3`. Measured, the literal `ρ − reach`, decided positive, refused
T1 in every op (six of §0.2's bodies) with no soundness gain: across
2·10⁵ random poses the `ρ(p)` reading never decided a wrong side. It
is a lever, not a new trilean.

**Certified pieces:** the elevation margin, the apex margin, the lever's
positivity. **Refusals:** off the face (an invariant: the sweep put the
vertex there), in band (escalation), the apex.

### 1.2 D1: the sector's normal (`sector_face`'s cone arm)

**Geometry.** The shared walk already mints the cone's normal
(`sector_face.rs` :197); the boolean's wrapper refuses it. The arm lets
it through, and adds what the walk leaves out: **an apex check.** A
sector based at the apex has no tangent plane, and today its normal is
poison. Measured on X1 and X2, that poison escalated at the corner rung
as `Corner(Straight)`, whose recourse names a tolerance that no tolerance
reaches. In the wrapper, `ρ(vertex)` is decided positive against the
band. Zero or in band refuses with a typed apex refusal naming the face
(the payload of `CrossingAtConeApex`: operand, face, band).

**The algebra downstream** (`within`, `side_code`, `sector_overlap`, the
bisector, `insert::germ_dir`, `vtxfac::pierce_germ_dir`) reads the
normal at the base vertex and the bounds as chords, and it is
kind-blind. Measured: with D1 and D2 open, no pose refused anywhere in
it, and every body that came back was right. **No cone-specific
algebra is needed there**, except the D2 lever above.

**What stays refused.** The coplanar-sector lump (`vtxfac.rs` :475–:640)
is reached only where both bounds of a sector read On the pierced face's
tangent plane: a tangency. With D10 in view, the cone takes none of its
declared branches (§2). Its undeclared in-band case refuses
`Coincide::Sectors` as it does today. The carrier lump (:652, :671)
refuses a cone through `rest::face_carrier`'s `None`, as now.

### 1.3 D3: the plane × cone section frame

**Geometry.** `pair_section_frame_at` gains the plane×cone arm, through
the table's own `geom_brep::plane_cone_section` (the C5 arm, exact
ellipse per Ev's 2026-10-01 ruling):

| `PlaneConeSection` | frame |
|---|---|
| `TiltedEllipse`, `AxisNormalCircle` | `Some((center, axis))`, the conic's own |
| `ApexLinePair` | `Ok(None)`, proved straight (two generators) |
| `ApexTangentLine`, `ApexPoint` | `FrameError::Desync`: a touching configuration the reduction should not have paired, exactly as the sphere arm's `TangentPoint` |
| `Err(Escalated)`, or the ellipse constructor's `Escalated` or `CircularAxes` | `FrameError::Escalated`: the classifier read a definite tilt, but the carrier cannot tell the curve from a circle |
| parabola, hyperbola (`Err`, by decision R1) | **a typed refusal naming the conic**, not a desync |

**The extent** `plane_cone_section` meters its aperture and conic-type
margins at is the frame's own `FrameExtent::Reach`: the cone face's
farthest distance from the reading point (`face_reach_from`), the
measure the cone's split lane levers the same section at
(`chord_join::section_reach`). It is the only extent the arm takes; any
other is a desync (Q4).

**The by-decision refusal.** Measured, the commonest pose of all — an
axis-aligned brick across a cone wall (B1, the brick
`[0.6, 2] × [0.3, 0.8] × [−0.2, 0.2]`) — cuts hyperbolas and stops
here, in every op and member order. So does any plane parallel to the
axis (C2). A brick one of whose edges grazes the wall stops earlier, at
the sweep's tangency refusal: `[0.6, 2] × [0.2, 0.8] × [−0.2, 0.2]`,
whose edge along `z` at `(0.6, 0.2)` touches the wall's circle of
radius `0.6` at `z = 0`, refuses
`CurvedPierceUnsupported { edge: EdgeKey(5v1) }` in every op, and
moving that face to `y = 0.25` or `x = 0.7` brings back the conic
refusal. This is
`crates/geom-brep/README.md` C1/C5's ruling (the hyperbola and parabola
are out of the conic inventory), not a missing arm. The refusal must say
so: `BooleanError::GermSectionOutsideInventory`, carrying the
`SectionError` that names the conic, not `JoinDesync` (Q3).

**Cone × cylinder** (R3, R4, B3) stays `NoArm`. Its frame is
`cone-pairs-in-general-pose-have-no-section-arm`, whose spec it is.

### 1.4 D4: the germ-pair dispatch

`(Plane, Cone)` takes `GermLane::PlaneWall` and `(Cone, Plane)` takes
`GermLane::WallPlane`, beside the cylinder's and the sphere's, with the
germ normal minted as theirs is. The wall side is the split lane's
`split_curve` against the germ plane, which already reads a cone
(`chord_join::section_case` :1011). The ring closure is
`RingClosure::Wall`, whose cone island winds by the segment's curve
(`chord_join::path_island_winding`, held to its oracle on a cone sheet
in `chord_join::cone_ring_rows`). Measured: with D4 and D5 open, every
ellipse and circle pose joined, R1, R7, B4, C1 and X3 in all six ops.

### 1.5 D5: the planar side's chord

`bool_planar_chord_spec`'s guard admits `Cone`. Everything past it is
kind-generic over `section_case`, which has the cone arm:
`TiltedEllipse`/`AxisNormalCircle` → a conic arc;
`ApexLinePair` → straight chords; `ApexTangentLine` → the existing
touching refusal (:1620); `ApexPoint` → its invariant. The aux wall
surface is the cone's full copy, minted once per germ face as the
cylinder's is.

### 1.6 The apex in the join (R6)

R6's thin brick past the full cone's apex got through D1–D5 and refused
`Join(Escalated { chord_arc_leave_germ, MarginDiag::Invalid })` at
`chord_join::arc_leaving` (:1218). The margin is `NaN`. **Measured: the
poison is the section, not a normal.** The brick's side plane `x = −0.03`
runs along the axis, so it cuts a hyperbola. But
`geom_brep::plane_cone_section` levered its `pn_axis_normal` sine at
`|h|·tan α`, where `h` is the axial height of the plane's *stored
origin*. That origin sat level with the apex, so the lever was 0 and the
section came back as a radius-0 `AxisNormalCircle`. That circle's zero
tangent gave `0/0` in `arc_leaving`, against a finite germ direction.
The sine is now levered at `max(|δ/c|·tan α, extent)`. Here `δ` is the
apex's distance off the plane and `c = â·n̂`. The first term is the
would-be circle's radius; the second is the reach that `pn_conic_type`
is metered at. The circle is built where the axis meets the plane.
Neither term reads the stored origin. R6 now refuses with the
hyperbola's typed refusal (R1).

### 1.7 D6 and D7: not this spec's arms

- **D6** is the cone admission spec's U4 (the certificate's cone rows:
  cone × plane, the coaxial pairs). §0.2 says what the mutant that skips
  it returned on these poses. It says nothing about the poses U4 exists
  for: the premise-S pose P3, and the null loops.
- **D7** is its own arm, the ring on a cone face in
  `props::face_flux`, built after U7 (Q5). The cylinder and torus read a
  ring there in closed form only when its edges are lines and circles; a
  cone face's rings in these poses are ellipse arcs (T1, R2), and the
  cone's closed form reads them anyway: its flux and area are the
  boundary's vector area, summed over every loop, its rings certified
  holes in the face (`geom_brep::props::cone_face_closed_form`). So ∪
  and cone ∖ other with a ring on a cone face build.

## 2. The D10 check, per door

The hold (`work/germ/log.md`, 2026-10-03) bars a new unit from
meaningfully using:

- declared pairs and declared contact (`Boolean`/`Union` `declare`,
  `ContactClass`, continuations, seams);
- the undeclared-coincidence and undeclared-contact refusals;
- axis declarations, `ParamSource`, and the rest of that list.

D10's Booleans paragraph keeps what these arms are: a boolean "glues
what its verdicts decide Zero where an arm exists for the carrier pair
… and refuses what falls in the sliver band".

| door | verdict | why |
|---|---|---|
| D1 sector normal + apex refusal | **not held** | a transverse normal and a margined apex verdict; reads no declaration |
| D2 pierce normal, its certificate and lever | **not held** | the same. `classify_vertex_on_face` passes `declared` through, but no cone pair can be declared (C8's inventory excludes the cone, `validate_declarations` `mod.rs` :5603), so every read on a cone pair finds no declaration and the arm adds no route that offers one |
| D3 plane × cone frame | **not held** | the C5 table's own classification. The touching outcomes keep today's desync/refusal shapes and offer no declaration |
| D4 dispatch | **not held** | pure geometry: a germ plane and a wall |
| D5 planar chord | **not held** | the conic arm. Its `SectionCase::Tangent` branch is the touching frontier, unchanged |
| the near-apex join poison (R6) | **not held** | a typed refusal at the apex |
| D6 certificate rows (VERBS-CONE U4) | **not held** | the guard exempts DECLARED pairs, and a cone pair never is one |
| D7 ring volume | **not held** | mass properties |
| **the second-order reading at a TANGENCY** | **HELD** | `sectors::tangent_lump` (:641) and `insert::record_germ_dir` (:2065) run only for a declared-`Tangent` pair (`BooleanCoincidence::TANGENT`, `vtxfac.rs` :621), through `geom_brep::tangent_locus`. A cone arm there is the declared-tangency seat D10 retires; tangency becomes a construction (D10 stage 6) and a contact an `unproven-coincidence` finding (stage 4) |
| **a cone sector ON a cone (or other curved) face** | **HELD** | `vtxfac.rs` :671: "a CURVED on-carrier sector is opened by a VERIFIED declaration and by nothing else"; `recl::carrier_of` (:52) through `rest::face_carrier`, the `Rest` ladder's declared rung. Opening it for the cone means a cone `CarrierDesc` and a declaration that covers it. Under D10 that coincidence is a margined verdict at the coincidence door (stage 4), built there, not here |

**What the held rows mean for U7.** Nothing on U7's path is held. Every
arm that turns a refusal into a body (D1–D5, then U4's D6) is
transverse. The held rows are the cone's touching and coincident
configurations, and they stay typed refusals through U7:

- a plane resting tangent along a generator (`ApexTangentLine`, and
  R9's graze in the sweep);
- a cone sector lying on a cone face.

That is the torus's standing since PR 3265. They are parked on
`d10-one-way-to-say-intent-is-unbuilt` (§3, U-H1 and U-H2), and U7's
rows pin them as refusals, never as bodies.

## 3. Units, in dependency order

**How the units stay safe.** Like the cone admission spec's, every
unit here leaves `boolean_arm_exists` untouched. The cone keeps refusing
at the operand gate, and the gate pins stay green:

- `reach_cone_root_lane.rs`'s
  `every_op_refuses_a_cone_operand_at_the_pair_gate`;
- `a_ring_on_a_cone_face.rs`;
- `verbs_germarms.rs`, `verbs_gate_r1_probes.rs`, `review_m3_pr4.rs`.

The rows are therefore verdict-level and in-crate. They call
`face_outward_normal_at`, `sector_face`, `pair_section_frame_at`,
`bool_planar_chord_spec` and `classify_vertex_on_face` directly. Or they
take `topo::sweep_split`'s door one stage further: a
`sweep-testing` door that runs the join on the split operands is U-S0,
optional.

| unit | scope | depends on | cost | review |
|---|---|---|---|---|
| U-S0 `cone-join-testing-door` (optional) | a `sweep-testing` door that runs the op with `Cone` on the roster, so the rows below can run whole poses below the public gate | — | E | orchestrator read |
| U-S1 `cone-pierce-normal` | D2: the cone arm of `face_outward_normal_at` (elevation on the face's nappe, apex refusal), and the cone's lever in `classify_vertex_on_face` (§1.1) | — | M | single |
| U-S2 `cone-sector-normal` | D1: `sector_face` lets the cone through; the apex refusal for a sector based at the apex (§1.2) | — | E–M | single |
| U-S3 `plane-cone-germ-frame` | D3: the plane×cone arm of `pair_section_frame_at`; the conic refusal typed (§1.3) | — | M | single |
| U-S4 `plane-cone-join-lane` | D4 + D5: the dispatch arms and the chord guard (§1.4, §1.5) | U-S3 | M | dual |
| U-S5 `cone-ring-volume` (not on U7's path) | D7: rings on a cone face in `face_flux`, the quadrature lane reading rings (§1.7) | — | H | dual |
| U-S6 `near-apex-join-poison` | R6: measure the poison input, refuse at its source (§1.6) | U-S1, U-S2 | E–M | single |
| — then the cone admission spec U4 (D6), U6, **U7** | | U-S1–U-S4, U-S6 | | |
| U-H1 `cone-tangent-lump` | **HELD by D10**: the second-order lump on a cone pair | `d10-one-way-to-say-intent-is-unbuilt` | M | dual |
| U-H2 `cone-on-carrier-sector` | **HELD by D10**: a cone sector on a curved face | `d10-one-way-to-say-intent-is-unbuilt` | H | dual |

U-S1, U-S2 and U-S3 are independent and may land in any order. U-S4
needs the frame. This spec's U-S3 + U-S4 is the cone admission spec's
U8 (the axis-normal join, "after the flip") and its Q1's ellipse, moved
before the flip: the item says it must precede U7, and §0 measured that
U8's own fixture (P5, here B4) answers once these land. Q1 asks how to
reconcile the two specs.

### U-S1 — cone-pierce-normal

1. **On the face:** at points on the widening and narrowing frustums'
   walls, the outward normal equals `(ŵ cos α ∓ â sin α)` folded through
   the sense, within an ulp-scaled bound.
2. **Off the face:** a point 1e-6 off the wall reads `OffSurface`.
   **Red against the mutant "certify by `‖∇F‖ − 1`"**, which certifies
   it on the chart. That margin is identically zero for the cone.
3. **The mirror nappe:** a point on the double cone's other nappe reads
   `OffSurface`, never a normal.
4. **The apex:** the apex itself, and a point `0.5·band` from it,
   refuse with the apex arm, never a poisoned normal.
5. **In band:** a point `0.5·zero` off the wall escalates `OnSurface`.
6. **The lever:** a bound leaving a pierce at `ρ = 0.1` toward the axis,
   reach 0.08. Read at `ρ(p)` its sagitta charge clears; read at
   `ρ − reach` it does not. **Red against the mutant "lever `ρ(p)`"**:
   the side is then decided where the bend near the axis puts it in
   band. Construct the pose so the decided side is wrong under the
   mutant. Measure it first; if no pose makes it wrong, say so and keep
   the row as an anti-loosening row.

### U-S2 — cone-sector-normal

1. **A cone sector off the apex:** `sector_face` returns the cone
   face's outward normal at a rim vertex of the widening frustum, both
   senses (a subtracted cone flips it).
2. **The apex:** the full cone's apex vertex refuses with the apex
   refusal, typed, naming the face. Red on main: the wrapper's
   `CurvedBooleanUnsupported`. **Red against the mutant "no apex
   check"**, whose poison reaches `sector_straight` as
   `Corner(Straight)`: X1's measured payload.
3. **The exhaustive match stays exhaustive.** The two lanes' wrappers
   still name every `SectorCarrier` arm.

### U-S3 — plane-cone-germ-frame

1. **Ellipse and circle:** the frame is the conic's centre and axis
   (C1's and B4's planes against the widening cone), against the
   Dandelin closed form in `PlaneConeSection`'s docs.
2. **The generator pair** (X1's plane): `Ok(None)`.
3. **Hyperbola and parabola** (C2, C3, C4, B1's faces): the typed conic
   refusal. **Red against the mutant "refusal folded into `NoArm`"**,
   which names the wrong cause.
4. **Touching:** `ApexTangentLine` and `ApexPoint` are the desync, as
   the sphere arm's `TangentPoint` is.
5. **The near-parabola band:** a plane within the band of a generator's
   direction escalates. It is never snapped to an ellipse.
6. **The lever:** a frustum far from its apex escalates a near-parabola
   whose margin is in the band at the face's reach and clear of it at
   `|at − apex|` or at ten times the reach (Q4).
7. **The near-circular tilt:** a tilt the classifier reads as definite
   but the carrier cannot tell from a circle escalates
   `ellipse_axes_distinct`, on the cone and on the cylinder arm alike.

### U-S4 — plane-cone-join-lane

The rows run whole poses through U-S0's door,
`topo::test_support::boolean_through_the_join`, which stops the
production pipeline after the join (`sweep/tests/cone_join_lane.rs`):

1. **B4's slab:** both sides' chords are the circle `y = 0.3` and
   `y = 0.6` arcs, on the cone's aux copy.
2. **C1's tilted plane:** the planar side's arc is the ellipse arc the
   matched germs name, in both member orders.
3. **T1's ring:** the cone face's island winds by the segment's curve
   (the ring lane's first boolean row): the island's only loop winds
   counter-clockwise about the cone's outward normal, the ring its
   remainder holds clockwise, and each of the box's two faces, in closed
   form, carries its ellipse on both sides.
4. **Red on main:** D4's `CurvedBooleanUnsupported` and D5's
   `SectionInvariant`. The mutant "`PlaneWall` for `(Cone, Plane)`"
   (the sides swapped) must turn a row red. That is the swap the arms'
   symmetry hides.
5. **Through U-S0, the whole poses,** joined in every op and member
   order, their chords against the closed-form section; C2, B1 and C3
   refuse at the frame with the conic named. No body is built
   below the join, so the volume, tier-3 and `point_in_solid` checks of
   §0.2 wait for U4's door. Every pose's interior-loop verdict is D6's
   refusal, and the rows say so. Below the gate, the join's output is
   pinned by the chords it mints.

### U-S5 — cone-ring-volume (off U7's path)

- **Red first:** T1's ∪ (`ResultInvalid { RingOnCurvedFace }` today,
  measured) returns its body, with the volume equal to `V(cone) + V(box)
  − 0.05715` and the grid integral agreeing.
- **R2's ∪:** the same.
- **The mutant "ring skipped"** returns the face's area without the
  hole, and must turn the row red.

### U-S6 — near-apex-join-poison

- **Measure first:** which input to `arc_leaving` is `NaN` on R6. It
  is the section conic (§1.6).
- **Red first:** a plane along the axis is a hyperbola wherever its
  stored origin sits; R6 refuses typed, never with an `Invalid` margin.

## 4. Open questions (⚑ = design fork)

- **Q1 Reconciling with the cone admission spec.** Its U8 (the
  axis-normal join, after the flip) and Q1 (the tilted ellipse, ruled
  in by Ev on 2026-10-01) are this spec's U-S3 + U-S4, before the flip.
  Recommendation: retire U8 there and point it here. The orchestrator
  edits that spec. Not a fork: Ev's ruling already admits the ellipse.
- **Q2 ⚑ The apex sector, X1.** A plane through the apex cuts the cone
  in two generators (`ApexLinePair`, in the inventory). Halving a cone
  through its apex is a natural user pose. This spec refuses it at the
  apex vertex's sector (U-S2). An arm would classify the plane against
  the apex's cone of material directions, an algebra the sector lane
  does not have. Does the apex vertex get an arm (a new unit, after U7),
  or is a cut through the apex a permanent typed refusal, like
  `CrossingAtConeApex`? That is a design choice about which poses the
  kernel answers, so it is Ev's.
- **Q3 The conic refusal's type.** A new arm,
  `BooleanError::GermSectionOutsideInventory`, carrying the
  `SectionError`. `CurvedBooleanUnsupported` says "not supported yet",
  and its recourse names other kinds of face; the conic is refused by
  decision, and its recourse is to cut the cone all the way round.
- **Q4 The frame's extent** for `plane_cone_section`'s aperture and
  conic-type margins: the face's reach (`FrameExtent::Reach`), and
  nothing else. The levered margin is `L·sin θ` (C3's plane turned by
  `θ`, so `D = −sin θ`), so the frame escalates exactly for
  `θ ∈ (ε/L, Kε/L)`. A longer lever moves a reading toward the definite
  side: a shorter arm never decides positive where the full reach would
  not (`UnitVec3::levered`). So the longer lever is the less
  conservative one for a definite verdict, and what makes a lever sound
  is that it bounds the consumed region, not that it is long. The
  face's reach does: every point of the face, and so of the section the
  join consumes on it, lies within it of the reading point, so a
  definite verdict at that lever holds over the whole face. It
  overstates the best hinge's reach by at most about 2×, which K = 10
  absorbs, and it is the measure the wall side's chord already levers
  at. `|at − apex|` is not a bound: a frustum far
  from its apex overstates by its distance from the apex (100 m to a
  face 1 m long in `a_frustum_far_from_its_apex_is_levered_at_its_own_reach`),
  deciding margins the face cannot. It is not kept as a fallback; a
  plane×cone frame handed any other extent is a desync.
- **Q5 U-S5's place.** The ring on a cone face was built and then
  refused at the result door, which is safe, so U-S5 was off U7's path
  and queued after it, U7's rows pinning T1's ∪ as `ResultInvalid` so
  the door's opening would be a red row. U-S5 has landed (§1.7).
- **Q6 The hold's two rows.** U-H1 and U-H2 park with
  `blocked_on: [d10-one-way-to-say-intent-is-unbuilt]`. Under D10 the
  second, a coincident cone sector, may not be a GERM unit at all: it
  belongs to stage 4's coincidence door. Whether GERM keeps it is for
  the D10 program to decide when it gets there.
- **Q7 The item's chain** (D1 → D2 → `no_arm` → chord) is stale in two
  places: D3 stands before the dispatch, and D6 and D7 stand after the
  chord (§0.1). The item should cite §0.1.
