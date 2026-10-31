# GERM plane × torus — the boolean join's germ frame for a plane × torus pair (unit spec)

This spec carries what GERM still owes on
`work/sect/c5-plane-torus-cone-cylinder-arms.md` (P1, H): the germ frame
for a plane × torus germ pair, which `boolean::join`'s
`pair_section_frame_at` refuses as
`GermFrameUnsupported { Plane, Torus }`. It measures the chain of doors
that pair meets (§0), specifies the frame for each configuration the
program can serve exactly and a named refusal for the rest (§1), checks
each arm against the D10 hold (§2), and cuts the units (§3). It follows
the method of `docs/GERM-CONE-SECTOR-SPEC.md`. The spec is deleted when
its last unit merges.

Read on `a2e0725b5e`. Line numbers ride beside names and may rot.

## 0. Measured: the chain on main

**How.** A scratch tree on `a2e0725b5e`, never committed and reverted:

- every `BooleanError::CurvedBooleanUnsupported` construction in
  `crates/topo/src/boolean/` and `crates/topo/src/splitting/`, and the
  frame's `NoArm`, wrapped to print its `file:line`. Each refusal below
  names its raising site, not a theory of it;
- three scratch arms behind environment switches, so one binary peels
  the chain one door at a time:
  - `PT_FRAME`: the frame of §1.1, with the general pose refused;
  - `PT_DISPATCH`: `kind_arm` admits the torus as a wall;
  - `PT_CHORD`: `bool_planar_chord_spec`'s guard admits the torus;
- a fourth switch, `PT_NOLOOP`, the mutant "every germ takes the first
  loop's frame".

Every pose ran under ∪ and ∩ in both member orders and under both ∖
(six results per pose; the lily ran ∪ only, three ways). The harness checked every returned body three
ways: `mass_properties` volume against a Monte Carlo of the analytic
operands, `validate_geometric` (tier 3), and `point_in_solid` at 4000
sampled points. **No pose returned a body** with any combination of
switches, so those checks never ran (§0.2).

**The fixtures.** `D` is the donut of `germ_torus_doors` and
`germ_interior_oval`: `R = 2`, `r = 0.5` about `y`, two torus faces
(inner and outer) split on the circles `ρ = 2`, `y = ±0.5`. A
"half-space" is a 12 m square bar whose start cap is the named plane.

| class | pose | plane against `D` |
|---|---|---|
| **meridian** (holds the axis) | M1 | `x = 0` (half-space) |
| | M2 | the meridian plane 30° round the axis |
| | M3 | the box `[0, 1] × [−1, 1] × [1.2, 2.8]`: a meridian face that cuts one circle |
| **parallel** (square to the axis) | P1 | `y = 0.2`: two concentric circles |
| | P2 | `y = 0`: the circles `ρ = 1.5`, `2.5` |
| | P3 | `y = 0.5`: tangent along the seam circle |
| | P4 | the box `[−1, 1] × [0.2, 1] × [1.2, 2.8]` |
| **axis-parallel, off the axis** | S1 | `x = 0.3`: two spiric ovals |
| | S2 | `x = 1.5`: tangent to the inner equator (a node) |
| | S3 | `x = 1.8`: one folded loop |
| | S4 | `x = 2.5`: tangent to the outer equator |
| **oblique** | O1 | normal 20° off the axis, through `(0, 0.1, 0)` |
| | O2 | normal 20° off the meridian normal, through the centre |
| | V1 | the bitangent plane through the centre, `asin(r/R)` off the equator (Villarceau) |
| **the row's consumers** | L1 | the lily's stem glue (`mate7a_r1_probes::{stem, arch}`): `stem ∪ arch` declared, and undeclared in both orders |
| | B1 | `germ_torus_doors`' chord across the hole, `[−0.1, 0.1]² × [−2, 2]` |
| | B2 | its bar through the tube, `[−0.05, 0.05] × [0.25, 0.35] × [1, 3]` |
| | B3 | its tilted bar (the Ferrari arm's), `framed_bar((0, 0.25, 2), (0, 0.3, 1), −1, 1, 0.02)` |
| | B4 | `germ_interior_oval`'s corner bar: `[−0.1, 0.1]²` cut so its eight corners lie on the inner face |
| | B5 | the same corner bar on a **fat** torus, `R = 1`, `r = 0.6` (and B5′ at half-width 0.3) |
| | T1 | TANG's tube on a latitude, turned to `y`: torus `R = 2`, `r = 1`; a tube of radius `2 + 1/√2` standing at `y = 1/√2`, its rim on the 45° latitude |

**Which poses are easy and which are general.** A plane holding the axis
cuts the ring torus in two circles of radius `r` (the meridian circles).
A plane square to the axis cuts it in two concentric circles about the
axis, or touches it along one, or misses it. Both are closed forms in
`geom_brep::plane_torus_section` (`intersect.rs` :2387), as is a third:
a plane parallel to the axis, off it but short of the inner equator,
cuts two spiric ovals (`PlaneTorusSection::SpiricOvals`, each a
`Curve3::Spiric`). Every other plane cuts a **general** spiric quartic,
which that table routes to the general rung. The general poses here are
S2–S4, O1, O2, V1, B3, and B4's end squares. The section certificate
(`section_cert::torus_plane`, :882) classifies every pose, general or
not, by its components: two loops round the axis, two loops round the
tube, one null oval, or a tangency.

### 0.1 The door chain

The doors in the order a pose meets them. **F1** is where main stops for
every transverse pose; each later door was reached by switching on the
arms before it.

| # | door | raising site (`a2e0725b5e`) | payload |
|---|---|---|---|
| F0 | an on-carrier torus sector (touching poses) | `vtxfac::classify_vertex_on_face`, the C8 rule "a CURVED on-carrier sector is opened by a VERIFIED declaration and by nothing else" (`vtxfac.rs` :721) | `CurvedBooleanUnsupported { kind: Torus }` |
| F1 | the germ pair's section frame | `join::pair_section_frame_at`'s `_ => NoArm` (`join.rs` :2499), mapped by `frame_refusal` (:2231) | `GermFrameUnsupported { Plane, Torus }`, either order |
| F2 | the germ-pair dispatch | `bool_connect`'s `no_arm` closure (`join.rs` :998, the measured print). `kind_arm` (:268) gives `KindArm::None`, so `edge_lane` runs. Which `return` calls the closure is read off the germ, not printed: :1025 for a germ along no edge, :1037 for the lily, whose edge lies in its planar holder | `CurvedBooleanUnsupported { kind: Torus }` |
| F3a | the planar side's chord | `chord_join::bool_planar_chord_spec`'s guard (:1610) | `Join(SectionUnarmed { divided: Plane, wall: Torus })` |
| F3a′ | the same side, past that guard | `section_case`'s fall-through to `plane_cylinder_section` (:1062) | `Join(Section { WrongLane { "plane×cylinder" } })` |
| F3b | the wall side's section | `chord_join::wall_section` (:1505) | `Join(SectionUnarmed { divided: Torus, wall: Torus })` |

F3a and F3b are one door, the chord layer's plane × torus section, met
from either side. **The member order decides which side meets it
first.** Torus-first orders (`T ∪ P`, `T ∩ P`, `T ∖ P`) meet F3b.
Plane-first orders meet F3a, then F3a′.

| class | poses | main | `PT_FRAME` | `+ PT_DISPATCH` | `+ PT_CHORD` |
|---|---|---|---|---|---|
| meridian | M1, M2, M3 | **F1**, all six | F2, all six | F3a / F3b by order | F3a′ / F3b |
| parallel, two circles | P1, P2, P4 | **F1** | F2 | F3a / F3b | F3a′ / F3b |
| axis-parallel, two ovals | S1 | **F1** | F2 | F3a / F3b | F3a′ / F3b |
| bars on those planes | B1, B2 (spiric sides, concentric top and bottom) | **F1** | F2 | F3a / F3b | F3a′ / F3b |
| the fat corner bar | B5, B5′ | **F1** | F2 | — | F3a′ / F3b |
| TANG | T1 | **F1**, both orders | **F3b in all six**: past the frame, the tube's rim is an edge lying on the torus, so `edge_lane` takes it (`GermLane::EdgePlane`) and the wall side's `wall_section` refuses | — | — |
| the lily | L1 | **F2 (:1037)**, not F1, declared or not | F2 | stem first: F3a; arch first: F3b | F3a′ / F3b |
| general | S3, O1, O2, B3, B4 | **F1** | the scratch general-pose refusal, all six | — | — |
| touching | P3, S2, S4, V1 | **F0**, all six | F0 | — | — |

**Corrections to the row's chain.**

- **The lily is not at the frame.** Its germ runs along the arch's rim,
  which lies in the weld cap, so `germ_section_frame` reads that edge's
  own circle (`along_edge_conic`) and never asks the face pair. Main
  stops at the dispatch: `edge_lane` refuses because the edge's holder is
  the stem's planar cap (`join.rs` :1037). The lily's own test already
  says so (`germ_circle_torus.rs`,
  `the_lily_stem_glue_is_past_the_circle_torus_pairs`). The lily needs the
  dispatch and the chord, not the frame.
- **The lily's declaration is not on this path.** The undeclared union
  reaches the same doors, in both member orders.
- **TANG's pose stops at F1 on main**, in both orders, as the row says.
  Past it, it takes the edge-plane lane and stops at F3b, not at the
  dispatch.
- **The corner bar.** With the exact frame arms open, it still stops at
  the frame, on its end square `z = −1.5068`, in all six ops: that plane
  lies past the inner equator (`R − r = 1.5`), so it cuts one null oval,
  a general pose. §1.4 shows this holds for every corner bar on this
  donut, and fails on a fat torus (B5).

### 0.2 Bodies returned, and the doors behind F3

No pose returned a body before U-T3, because F3 stood behind every
served frame. Measured in U-T3 with the circles' chord in, every op of
every pose:

- **Bodies.** M1, M2, P1, P2, `y ≥ −0.3`, a box face `y = 0.2` and one
  `y = 0.499`, on the donut, the one-face torus and the seamed torus,
  and T1, build in all six ops. Each holds to its closed form, to a
  Monte Carlo of the analytic operands, to tier 3 and to
  `point_in_solid` (`germ_plane_torus_chord`). So do three older pins
  that stood at F3: the notched half donut (`3π²/8`), the vase's brick
  in its torus band and TANG's tube in `pi_seam_and_kiss`.
- **M3 and P4** stood at F3 until U-T4: their box faces `z = 1.2` cut
  two spiric ovals. They build since (§1.3). Past U-T4's chord, ops
  stop at three doors past the join, each filed (§1.3).
- **The interior-loop guard** stops the lily (both orders, ∪): past the
  join, the certificate has no arm for the stem's tube wall against
  the arch's, two tori with parallel axes apart (filed on ORBIT,
  `the-lilys-stem-and-arch-walls-have-no-certificate-arm`). The lily's
  ∩ and ∖ refuse earlier, on the no-crossings path.
- **A section circle that is an edge of the torus** (`y = 0` against
  the one-face torus with its vertex on an equator, and against the
  torus seamed there) refuses `SingleSiteSectionLoop` in every op: the
  in-face circle class of SECT's
  `closed-in-face-section-loop-has-one-site`, where the evidence is
  added.
- **A ring on a torus face** is reached through the public ops: the box
  `x ≥ 0 ∧ y ≥ h`, `h < 0`, against the one-face torus with its vertex
  on the inner equator puts a pierce ring on the torus face, and so
  does the bar `[−3, 3] × [0.2, 1.2] × [0, 3]` against the donut, torus
  first. The ring's point lies in the planes of run arcs (a box edge
  pierces the tube on two box faces' planes), and on the one-face
  torus the face's outer loop lies on the other arcs' carriers, so
  every path between them runs along an arc's plane; the ring lane
  reads such a piece apart from the arc (§1.3), and both build their
  true bodies in every op. So does the box `x ≥ 0 ∧ y ≥ −0.3` against the
  one-face torus with its vertex at `π + asin 0.6`, whose wrap parallel
  and vertex lie on the box's face: an edge and a vertex on a face that
  cuts the tube transversally, decided Zero by the crossing layer's
  margins, as TANG's rim is (§2, not held). Containment reads their ∪
  and `T ∖ P` bodies, whose torus walls are no chart rectangle, by the
  torus region (RESTREAD, `boolean::torus_region`).
- **A slab across the one-face torus** refuses `JoinDesync` in
  `choose_roles` at some vertex phases (filed on GERM,
  `a-slab-against-a-one-face-torus-desyncs-its-chord-roles`).
- **Containment on a body the notch mints** reads it: its torus walls
  are L-shaped in the chart, which the torus region reads (RESTREAD,
  `point-in-solid-refuses-an-l-shaped-torus-face`, closed).

**The section certificate on the corner bars** (`section_report`, on
main):

- B4's twelve face pairs clear as `Essential` (8), `Out` (2) and
  `LoneEvented` (2). The last two are its end squares, cleared by W4 on
  the corners' events alone.
- B5's twelve pairs all clear as `Essential`. Its end squares cut two
  ovals round the tube, which are essential on the torus (W2).

### 0.3 The loops' geometry, measured outside the kernel

What the facing test needs of a frame is a centre and an axis about
which each loop of the section turns strictly monotonically
(the argument the cylinder × sphere transverse frame made, retired with
that frame by SECT step 1a: the pair now matches by foot parameter on
its section). A numeric tracer
(marching squares on the implicit section in the plane) read each loop's
turn about a candidate centre:

- **Round the axis** (P1, and 12 loops from random poses of that
  class): every loop is monotone about the torus axis.
- **Round the tube**: monotone about the point where the core circle
  crosses the plane, on S1, the spiric ovals out to `x = 1.49`, O1, O2,
  and 422 loops from random poses. **Not** monotone on a plane 75° off the meridian
  normal, near the Villarceau tilt (its `s·R − r` is 0.018 m). So the
  class has no frame in general.
- **Null ovals**: the axis-parallel ones (S3, `x = 1.52`, B4's end
  square, `x = 2.3`) are monotone about their centroid. Tilted ones are
  not (10° off the axis at `h = 0.3`, and 40° at `h = 1.2`), about any
  centre the tracer tried.

§1.1 proves the cases the frame serves; it does not rely on this
sampling.

## 1. The arms

**Notation.** The torus is `(c, â, R, r)` with `R > r > 0`
(`TorusConvention`). The plane is `(q, n̂)`. The stand-off is
`d = n̂·(q − c)`, and `ŵ = â × n̂` (a unit vector once `â·n̂` is decided
Zero). For a point `p`, `ρ` is its distance from the axis. A **loop** is
one connected component of the section.

### 1.1 The frame per configuration

`pair_section_frame_at` gains the plane × torus arm, in either member
order, through the table's own `geom_brep::plane_torus_section`:

| `PlaneTorusSection` | frame of the loop the germ lies on | loop decided by |
|---|---|---|
| `MeridianCircles { c1, c2 }` | that circle's `(centre, n̂)` | `σ = sign((p − c)·ŵ)` |
| `SpiricOvals { s1, s2 }` | `(k_σ, n̂)`, with `k_σ = c + n̂d + σŵ√(R² − d²)`: the core circle's crossing of the plane inside oval `σ` | the same `σ` |
| `ConcentricCircles { c1, c2 }` | `(c + âh, â)`, one frame for both | `sign(ρ − R)`: outer or inner |
| `TangentCircle`, `Empty` | `FrameError::Desync`: a touching configuration the reduction should not have paired, as the sphere arm's `TangentPoint` | — |
| `Err(RoutesToGeneralRung)` | the general-pose refusal (§1.2); where the certificate's unlevered margins read the pose touching or empty, or leave a margin undecided, `FrameError::Escalated`: the two readings straddle the band about one pose | — |
| `Err(Escalated)`, `Err(Spiric(Escalated))` | `FrameError::Escalated` | — |
| `Err(DegenerateTorus)` | the ring-torus convention's refusal, as `plane-torus-section-degenerate-torus-ends-as-corruption` settles it | — |

**Why each frame is a frame.**

- **The meridian circles** are circles.
- **The concentric circles** share their centre and axis. The facing
  test reads only `â·((p − c₀) × ṗ)`, and sliding `c₀` along `â` leaves
  it unchanged, so one frame serves both loops. This was the cylinder ×
  sphere coaxial frame's argument (that frame retired in SECT step 1a).
- **The spiric ovals** turn strictly monotonically about `k_σ`, which
  lies inside them. The proof's one home is `join::pt_germ_frame`'s
  doc. The meridian circles are its case `d = 0`.

**The loop is the germ's, decided by a margin.** On a meridian circle
or a spiric oval, `|(p − c)·ŵ| = √(ρ² − d²) ≥ √((R − r)² − d²)`. That is
strictly positive wherever the table served the pose, so `σ` is decided
definite at every site on the section. On the concentric circles,
`|ρ − R| = √(r² − h²) > 0`, which the table's `pt_cap_gap` decided. A
site whose loop margin falls in the band is not on the section, which is
a desync.

**The dispatch names loops, and the caller picks one.** The dispatch's
contract is a statement about two surfaces, never about a germ ("keyed
on the germ pair's two surface KINDS and nothing else"). So
`pair_section_frame_at` returns the section's loops, one frame or two,
each with a loop identity. `germ_section_frame` picks the germ's loop by
its site (`half_edge_start_point(germ.he)`). Four consumers read the
frame:

- **`germs_face_each_other`** (`join.rs` :2861) reads both germs'
  loops. **Two germs on different loops never face**, and the result is
  `Ok(false)`, not a desync. On M1 each of the donut's two faces carries
  arcs of both meridian circles, so a face pair carries germs on both
  loops, and that is legitimate.
- **`nearer_along`** (:1399) never ranks a candidate on another loop.
  Its half-turn and travel read a turn about the germ's own frame, which
  means nothing for another loop's site.
- **`one_site`'s antipode** (:683) reads `centre + (centre − p)` as a
  point of the conic. That holds for a circle. **It fails for a spiric
  oval**, which is not centrally symmetric. A one-site loop on an oval
  takes the oval's closed-form extent for its ball instead: `t` between
  `√((R − r)² − d²)` and `√((R + r)² − d²)`, and `|y| ≤ r`. A loop
  identity therefore carries whether its centre is a centre of symmetry.
- **`wrap_site_segments`** takes a one-site loop on a wrap edge as one
  segment round the whole loop, and refuses a second record **on the
  same loop** of its face pair as a desync. A second record of the same
  face pair on the other loop is legitimate: on a one-face torus each
  meridian, concentric or spiric loop crosses a wrap edge once, and on a
  torus seamed on `y = 0` both circles of a plane `y = h`, `|h| < r`,
  lie on one face and cross its wrap edge once each.

**The extent.** This is `FrameExtent::Reach`, the torus face's farthest
distance from the reading point (`face_reach_from`), handed to the table
as its `extent`. It is the cone arm's measure (U-S3), and `frame_extent`
gains the torus arm beside `cone_reach`. A definite verdict at that
lever holds over the whole face, and so over the section the join
consumes there. Any other extent is a desync. The extent levers only the
axis tilt's fork; no served circle leans on it being long:

- **A meridian circle** is minted on the torus about a centre `d` off
  the plane, and the tilt leans its points off the plane by up to
  `r·|â·n̂|` more. The table decides the sum (`pt_axis_plane_gap`
  beside its floor), so a served circle stands under the band off its
  plane at any lever. Decided one at a time, `d = 0.9ε` beside a lean
  of `0.9ε` stood `1.8ε` off on the donut and on `(1, 0.95)`, and
  `1.32ε` on the arch `(1.1, 0.052)`.
- **The concentric circles** are read at the plane's crossing of the
  axis, depth `h = d/(â·n̂)`, wherever the plane's origin is stored. A
  circle of radius `ρ ≤ R + r` stands at most `ρ‖â × n̂‖` off the plane,
  under the tilt levered at `R + r`, and the plane's depth moves by at
  most as much across the circles, which `pt_cap_gap` reads beside its
  gap. Read at the stored origin, a circle stood off the cut by the
  origin's distance from the axis times the tilt (19 bands at 50 m, 181
  at 500 m), and a plane 41 bands over the tube's crest was served
  circles. At `R` rather than `R + r` the outer circle stood `1.1ε`
  off its plane (`chord_join::torus_chord_rows`).

The frame reads only the circles' centres and axes. The chord reads
the circles, at the divided face's reach, which can be shorter than `r`
(the lily's weld cap reads `0.112 m`); the sum above bounds them there
too.

### 1.2 The general pose: a typed refusal named for its configuration

Every `RoutesToGeneralRung` from the table refuses with a new
`BooleanError::GermSectionSpiric`. It carries:

- the faces and kinds;
- the table's `SectionError`;
- `loops: SpiricLoops`, which is `AroundAxis`, `AroundTube` or `Null`.

`loops` is the section certificate's own classification
(`section_cert::torus_plane`'s NEAR, FAR and BETWEEN margins), exposed
as a crate-private reading so that the refusal and the certificate name
one decision. The measured poses take:

| pose | `loops` |
|---|---|
| S3, and B4's end square | `Null` |
| O1, O2 | `AroundTube` |
| a plane a few degrees off axis-normal that still cuts both tube circles | `AroundAxis` |

The recourse is the one the dispatch's torus refusal gives
(`meeting_recourse`), which names the planes a torus face is joined
against since U-T3 built them: a plane through the torus's axis or
square to it.
A tangency (S2, S4, V1, P3) never reaches this refusal: F0 refuses it
first, and the certificate's own tangent verdict would refuse it at the
guard.

**Why not serve a frame for loops round the axis.** That class has a
proved frame:

- each half-plane at azimuth `φ` meets the plane in a ray, which meets
  the tube circle at most twice;
- two loops each winding once round the axis therefore cross every
  half-plane exactly once;
- a loop tangent to its half-plane would make that ray tangent to the
  tube circle, which joins the two loops, and the certificate's definite
  NEAR and FAR margins exclude that;
- so `â·((p − c) × ṗ) = ρ²φ̇ ≠ 0`, and `(c, â)` is the frame.

The frame is not served anyway. The chord layer has no arm for the
quartic (the table routes it to the general rung, and `Curve3::Spiric`
carries only the axis-parallel ovals), so a served frame would only
move the refusal one door later, under a vaguer name. The proof is kept
here for the day the general rung retires (Q2).

### 1.3 What the chain needs past the frame

These are not the frame, but they are what turns the frame into
bodies, measured in §0.1:

- **The dispatch (F2).** `kind_arm` gives `(Plane, Torus)`
  `KindArm::PlaneWall` and `(Torus, Plane)` `KindArm::WallPlane`, beside
  the sphere, cylinder and cone. The lily's germ then takes the planar
  lane rather than `edge_lane`, because a kind arm comes first.
- **The torus face's ring closure.** Measured in U-T2 with the kind arm
  in, over every pose of §0 in all six ops, the one-face and seamed
  tori's two-loop poses and the lily both ways: 266 runs, each stopping
  at F3a or F3b. No torus face reaches `choose_roles`' ring lane: the
  lane needs a segment whose two germs lie on one ring of the face,
  which only an island's closing segment has, and every op stops at its
  first segment's chord. Read in-crate, the lane would refuse: the
  torus face took `RingFace::Wall`, and `chart_island_winding` reads
  only a cylinder's chart (`RingIslandUnread { Torus }`). A chart
  winding on the torus would need closed-form chart integrals of every
  edge image, spiric ones included. The closure built instead is the
  sphere's and the cone's: the torus is a `ring_path::Quadric`, its
  island read by a path's parity across the run closed by the segment's
  curve (`RingFace::Quadric`). The paths run on meridians and parallels.
  A crossing of a loop arc's plane counts only on that arc's own loop
  of the plane (`ArcLoop`, on the join's own `LoopSplit`), because a
  plane cuts a torus in two. A piece that runs along an arc's plane —
  the meridian or parallel through a point of the arc's carrier or of
  its plane's other loop, as a box edge's pierce point is — meets the
  arc nowhere where the two are decided apart: its circle clear of the
  arc's, or the arc's own circle with the two spans disjoint
  (`ring_path::misses_in_plane`); so does a lead with an end on the
  plane, decided farther from the arc than half its length. The
  parity does not depend on the path, because tier 3 lifts every face
  to a chart disc, so the island's curve bounds on the torus. Loop arcs
  read are meridians and parallels, but for two parallels: the tube's
  top and bottom circles (radius `R`), whose plane touches the torus
  along them, so a path meets that plane only where it meets the
  circle and never crosses it, and plane crossings cannot count them.
  They are read across the cylinder `ρ = R` instead, which cuts the
  torus transversally in exactly those two circles, told apart by
  their height along the axis (`ring_path::LoopArc::Crest`, U-T3): a
  meridian piece crosses it at the top and bottom of the tube, a
  parallel piece never. A spiric oval's arc is read as a meridian's
  is (U-T4): a crossing of its plane, parallel to the axis, counts on
  its own oval's side of the axis's trace, inside its span on its own
  parameter.
- **The chord for the circles (F3, meridian and concentric).**
  `section_case` (`chord_join.rs` :957) and `wall_section` (:1487) gain
  the torus arm. **Both sections have two loops**, and `SectionCase` has
  no two-conic case. The arm picks the loop the chord's sites lie on,
  with the frame's margin, as `SectionCase::Straight([l1, l2])` is
  resolved at the sites for two rulings. T1 needs only this: it takes
  the concentric circles through `wall_section` (its outer circle is
  the rim) and builds. The lily takes the arch's meridian circle on
  both sides, and then stops at the interior-loop guard (§0.2).
- **The chord for the ovals (F3, spiric).** `SectionCase` gains a spiric
  arm: an arc of a `Curve3::Spiric`, chosen by its own parameter, which
  turns monotonically about `k_σ` (§1.1). B1, B2 and B5 need it, and,
  measured in U-T4, both ring lanes behind it:
  - **the torus face's**, for the pierce islands of B1, B2 and B5 (their
    rings stand on the plane of an oval of the face's run, on its other
    loop, and on a concentric circle's plane out of the run's span: a
    path's lead of no length from such a ring is read as the point it
    is, and passed over where it is decided off the arc);
  - **the plane face's**, for S1's ovals, each an island in the plane
    face: its winding sum takes a spiric term, an elliptic integral
    read through `geom_brep`'s certified enclosure of the arc's vector
    area (`props::spiric`, the spiric face lane's own term), summed at
    the interval scalar and refined until the sign is decided (`topo`'s
    `loop_winding`). Ovals pinched toward the inner equator, `1e-6`
    short of it on the donut and `0.01` from the axis on the fat tori
    `(1, 0.9)` and `(1, 0.95)`, wind and build.

  Past the join, a result's torus face bounded by ovals is no chart
  rectangle; `point_in_solid` reads it by the torus region (RESTREAD),
  and the spiric face lane measures it, a torus face holding a ring
  included (every loop read, each ring wound against the face). One
  door behind it stops some ops: the spiric face lane's
  `QuadratureBudget` (`VolumeUnmeasured`), measured at ε `1e-12` on
  `P ∖ T` of the corner bar of half-width `0.05` on the fat torus and
  its spellings
  (`work/spiricarm/a-thin-corner-bars-spiric-cap-exhausts-the-quadrature-budget-at-1e-12.md`).

### 1.4 The corner bar's lens

**On the donut (B4) the frame stays the barrier.** A corner bar of
half-width `w` with its corners on the inner face has its end squares at
`z² = ρ² − w²`, with `ρ = R − s` and `s = √(r² − w²)`. Then

`z² − (R − r)² = 2(r − s)(R − r − s)`.

On this donut, `R − r − s ≥ 1 > 0`, so the end squares always lie past
the inner equator. They cut one null oval, the general pose, and the
frame refuses (`GermSectionSpiric { loops: Null }`) in every op,
measured on B4. The row
`germ_interior_oval::the_corner_bar_never_comes_back_a_body` stays green
under every unit here. U-T1 re-pins its refusal to that named one at the
end square, so it goes red the day anything serves that section.

**On a fat torus the lens reaches the chord layer.** Where
`R < r + s` (`R < 2r` is necessary), the end squares lie short of the
inner equator and cut two spiric ovals. The frame serves them. Measured
on B5 (`R = 1`, `r = 0.6`, `w = 0.1`, end squares at `z = 0.396`
against `R − r = 0.4`) and B5′ (`w = 0.3`): every op passes the frame,
then the dispatch, and stops at F3.

Each oval runs from corner to corner across the end square:

- on B5 the +`x` oval reaches `x = √((R − r)² − z²) ≈ 0.057` inside the
  square's `|x| ≤ 0.1`;
- so the square holds a lens of the tube on each side, bounded by oval
  arcs whose only events are the corners.

The certificate clears those pairs as `Essential` (W2), and that is
its whole contract: a component evidenced by an event is left to the
join to trace (its lemma L1). Nothing upstream needs to guard the lens,
because the join already pairs it. Measured on main, B5 and B5′ in
all six ops and both orders: the crossing layer records eight corner
events, the matcher consumes all eight, and each segment joins the two
corners of one oval, `(±w, w, ±z) → (±w, −w, ±z)`. Two face pairs
share each site pair (the end square and the bar's side `x = ±w`, each
against the torus), because the corner edge dips into the tube. So the
pairing is decided before any chord exists. U-T4 cannot mis-pair the
lens; it can only mis-curve it.

The lens is then traced by the rule every curved chord uses, with no
mechanism of its own (Q1):

- which arc of a closed section the chord takes is the pairing's
  answer, read as the germ's departure (`chord_join::arc_leaving`
  decides `leave · tangent(p₁)`, and `oriented_arc` runs from the start
  site's parameter to the matched site's). At a corner the departure
  points into the end square along the oval;
- a curved wall never defaults to a straight chord: an unarmed section
  is a typed refusal, so chording the end square along its own edges is
  a mutant, not a state the lane can drift into.

The same class builds true bodies on main for the other kinds: a box
edge dipping into a sphere, an oblique cylinder or a cone frustum, its
two ends on the curved face and nothing else touching, builds the true
body in all six ops (volume against a Monte Carlo of the operands, tier
3, `point_in_solid`). Whether such a component carries only vertex
events depends on where the other body's seam falls: on a cylinder with
its seam turned across the lens the same lens carries an edge event and
builds the same bodies.

## 2. The D10 check, per arm

The hold (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`, and
`work/germ/log.md`, 2026-10-03) bars a new unit from meaningfully using
declared pairs and declared contact, the undeclared-coincidence and
undeclared-contact refusals, axis declarations, `ParamSource`, and the
rest of that list. D10's Booleans paragraph keeps what these arms are:
a boolean "glues what its verdicts decide Zero where an arm exists for
the carrier pair … and refuses what falls in the sliver band".

| arm | verdict | why |
|---|---|---|
| the frame's exact arms, and their loop margins (U-T1) | **not held** | the C5 table's classification, and margins on transverse sections; no declaration is read or offered |
| the general-pose refusal (U-T1) | **not held** | a typed refusal reading the certificate's margins |
| the dispatch (U-T2) | **not held** | pure geometry: a germ plane and a wall. **The lily's declared `Rest` cap pair is not read on this path**: the undeclared union reaches the same doors in both orders (§0.1), so U-T2's rows run the lily undeclared |
| the chord arms (U-T3, U-T4) | **not held** | sections of the C5 table; a tangent section keeps today's touching refusal |
| TANG's rim (the edge-plane lane, F3b) | **not held** | the rim is ON the torus by the crossing layer's margined verdict, not by a declaration |
| the lens (U-T4) | **not held** | a traced join on the undeclared path (Q1): margined geometry, no declaration read |
| **a torus sector ON the plane: the touching poses P3, S2, S4, V1 (F0)** | **HELD** | `vtxfac.rs` :721: the C8 rule opens a curved on-carrier sector "by a VERIFIED declaration and by nothing else". Under D10 that coincidence is a margined verdict at stage 4's coincidence door, built there, not here |
| **the second-order reading at a tangency** (`sectors::tangent_lump` for a torus) | **HELD** | runs only for a declared-`Tangent` pair; D10 makes tangency a construction (stage 6) |

Nothing on the units' path is held. The two held rows are the torus's
touching configurations, and they stay typed refusals through every unit
here. They are parked as U-H1 (§3), the cone spec's U-H1 and U-H2 over
again.

## 3. Units, in dependency order

The torus is on every op's operand roster: all six ops reached the frame
in §0. So the rows run whole poses through the public ops, and no
testing door is needed (the cone spec's U-S0 has no counterpart).

**Difficulty and cost.** Difficulty is the brief's H/M/L. It maps to
the item header's `cost:` as H→H, M→M, L→E. Cost is the size of the lane.

| order | unit | scope | depends on | difficulty | cost | review |
|---|---|---|---|---|---|---|
| 1 | U-T1 `plane-torus-germ-frame` | F1: the frame per §1.1 (loops and their identity through `germ_section_frame`, `germs_face_each_other`, `nearer_along` and `one_site`); `FrameExtent::Reach` for the torus face; the general-pose refusal `GermSectionSpiric` (§1.2); re-pins B4 | — | M | one lane, about 14 rows | single |
| 2 | U-T2 `plane-torus-join-dispatch` | F2: `kind_arm`'s torus arms; measure, then make, the torus face's ring closure (§1.3: path parity, as the sphere's and the cone's) | U-T1 (the lily alone needs only this unit) | M | one lane; the ring closure is the unknown | single |
| 3 | U-T3 `plane-torus-chord-circles` | F3 for the meridian and concentric sections: the torus arm of `section_case` and `wall_section`, with the two-loop pick at the chord's sites; the ring lane's reading of the tube's top and bottom circles (§1.3); then measure the doors behind F3 (§0.2) for T1, the lily, M1–M3 and P1–P4 | U-T2 | H | one lane; dual review | dual |
| 4 | U-T4 `plane-torus-chord-spiric` | F3 for the spiric ovals: a spiric arm in `SectionCase`, an arc chosen on the oval's own parameter; **owns the corner bar's lens on a fat torus** (§1.4, Q1) | U-T3 | H | one lane; dual review | dual |
| — | U-H1 `torus-on-carrier-sector` | **HELD by D10**: the touching poses at F0 (a torus sector on the plane, the tangent circle, the Villarceau pair) | `d10-one-way-to-say-intent-is-unbuilt` | H | — | dual |

**Order.** U-T1 lands first. U-T2 needs it for every pose but the
lily's. The lily's glue (`L1`) passes the chord with U-T2 and U-T3 and
then needs a certificate arm for its two tube walls (§0.2). TANG's pose
needs U-T1 and U-T3, and builds. `germ_torus_doors`' bars B1 and B2
pass the join with all four units, and build or stop at the two doors
behind it (§1.3). B3, and the near-perpendicular bar, are general poses and refuse
by name once U-T1 lands, until the general rung retires.

### U-T1 — plane-torus-germ-frame

Rows run in-crate, against `pair_section_frame_at` and
`germ_section_frame`, and through the public ops.

1. **The meridian loop is the germ's.** On M1's plane, a germ site on
   `c2` gets `(c2.centre, n̂)`. Red on main: `NoArm`. **Red against the
   mutant "every germ takes the first loop's frame"** (the scratch's
   `PT_NOLOOP`).
2. **Loops never face across.** On M1's face pair, two germs of opposite
   sense on different circles do not face. **Red against the mutant
   "the facing test ignores the loop"**, which pairs them.
3. **`nearer_along` stays on its loop.** A candidate on the other
   meridian circle, nearer by turn about the germ's frame, is never
   chosen. Red against the same mutant.
4. **The concentric circles** (P1's plane): `(c + âh, â)` for a site on
   either circle, with each circle as its own loop. Red on main.
5. **The spiric ovals** (S1's plane, and B1's sides): the frame is
   `(k_σ, n̂)`. Every site sampled along the oval in closed form decides
   one nonzero sense. **Red against the mutant "centre at the plane's
   foot `c + n̂d`"**: that point lies outside both ovals, so the sense
   changes along each.
6. **The loop margin.** A site on the axis's trace in the plane
   (`(p − c)·ŵ = 0`), which no point of a served section comes near,
   decides its loop in band: a desync, never a loop.
7. **The general pose is named.** S3 and B4's end square give
   `GermSectionSpiric { loops: Null }`. O1 and O2 give `AroundTube`. A
   plane 3° off axis-normal, through `(0, 0.2, 0)`, gives `AroundAxis`.
   B3 refuses by name in all six ops. **Red against the mutant "the
   general rung folded into `NoArm`"**, which names the wrong cause.
8. **Touching under a germ:** `TangentCircle` and `Empty` are the desync.
9. **The lever:** a plane whose axis tilt is in band at the torus face's
   reach and definite at ten times it escalates. It never snaps to a
   meridian or a spiric.
10. **TANG** (T1): past the frame in all six ops, stopping at F3b until
    U-T3. Red on main: F1.
11. **The corner bar on the donut** (B4): the refusal is
    `GermSectionSpiric { loops: Null }` at the end square, in all six
    ops. The row `the_corner_bar_never_comes_back_a_body` stays.
12. **The fat corner bar** (B5, B5′): past the frame in all six ops.
    Pinned at the next door until U-T4, so that each later unit's move
    is a red row.
13. **The touching poses** (P3, S2, S4, V1) still refuse at F0. These
    are pins.
14. `germ_torus_doors`' `a_chord_across_the_hole_is_pierced_not_passed`
    and its siblings move off `GermFrameUnsupported` to F2.

### U-T2 — plane-torus-join-dispatch

1. **Measure first:** whether a torus face's ring island winds along the
   section plane on the torus chart (`RingFace::Wall`). Measured: it
   does not (§1.3). The closure is path parity, held to an oracle on a
   torus sheet (`chord_join::torus_ring_rows`), as
   `chord_join::cone_ring_rows` holds the cone's.
2. **The lily, undeclared, both orders:** past F2, then meeting F3a
   (stem first) or F3b (arch first), as measured. Red on main: F2 at
   :1037.
3. **The sides:** **the mutant "`PlaneWall` for `(Torus, Plane)`"** must
   turn a row red.
4. M1–M3, P1, P2, P4, S1, B1, B2, B5: past F2 in every op.

### U-T3 — plane-torus-chord-circles

1. **Red first:** F3a′ and F3b's payloads, from both sides.
2. **The two-loop pick:** on M1, a chord whose sites lie on `c2` is an
   arc of `c2`. **Red against the mutant "the first circle always"**.
3. **Measure the doors behind F3** (§0.2) on T1, the lily, M1–M3 and
   P1–P4, and land what they need, or file it on the owner's slate.
4. **Bodies**, each against the truth: volume against a Monte Carlo of
   the analytic operands, with a closed form where one exists (M1's ∩ is
   half the donut, `π²Rr²`); tier 3; and `point_in_solid` at sampled
   points.
5. **The tube's top and bottom circles in the ring lane** (§1.3): a
   run or a closing chord on either reads its island and re-homes its
   rings, held to the torus islands' oracle.
   `torus_ring_rows`' `a_run_on_the_tubes_top_or_bottom_circle_is_a_ring_the_lane_does_not_read`
   goes red, and is rewritten as the reading
   (`a_run_on_the_tubes_top_or_bottom_circle_winds_its_island_as_the_oracle_does`).
6. **The minted circles' planarity** (§1.1): at the thinnest face
   reach among the rows, with the meridian plane's stand-off and lean
   in band together, and with the plane's origin stored off the axis.

### U-T4 — plane-torus-chord-spiric

1. **Red first:** S1, B1 and B2 at F3, with spiric payloads. Since
   U-T3, a torus's `SectionUnarmed` is raised only for its spiric
   ovals.
2. **Arcs on the oval's own parameter**, held to the closed form.
   **Red against the mutant "arc chosen by angle about the plane's
   foot"**.
3. **The lens (the obligation):** B5 and B5′, all six ops, each either
   the true body (volume, tier 3 and `point_in_solid` against the truth)
   or a typed refusal, **never a wrong body** (`point_in_solid` refuses
   every body an oval bounds, §1.3, so the lens's arcs on the built
   bodies witness it at every ε). **Red against the mutant
   that chords the end square along its own edges**, skipping the oval
   arcs between the corners: that body misses the lens. Measure the lens
   volume first and show it well above the rows' tolerance, as
   `the_oval_lens_is_well_above_the_rows_tolerance` does for the oval
   (measured: `7.08e-5 m³` on B5, `9.40e-3` on B5′, by a box oracle
   held to a Monte Carlo; the earlier `6.6e-5` and `9.3e-3` were off).
   The spiric arc goes
   through `arc_leaving` and `oriented_arc`, not a rule of its own: the
   oval exposes its parameter and tangent, and a lever for the departure
   cosine in place of a conic's semi-major axis (the oval's own extent).
   Pin B5's segment listing (eight records, corner to corner on one
   loop) beside `every_segment_joins_two_sites_of_one_loop`, and name
   in review that the matcher's `off_conic_slack` assumes a conic.
4. **Bodies** for S1, B1 and B2, against the truth.

## 4. Open questions (⚑ = design fork)

- **Q1 The fat corner bar's lens** (answered: traced, through the
  chord's arc rule; no guard in any layer). On B5 the frame and the
  section certificate both pass a lens whose arcs carry no event but its
  corners (§1.4). The join already pairs its corners on one loop, and
  U-T4's spiric arc follows the departure rule every curved chord uses.
  No certificate, frame or join rule reads whether a component's events
  are vertex contacts: after the reduction every event is a vertex
  contact, and whether one is minted or brought by the operand depends
  on how the input is meshed and where a seam falls, not on the pose.
  A margined rule refusing such components was measured: it would
  refuse the sphere, cylinder and cone corner lenses that build true
  bodies today, and, by a census over the sweep suite, up to 93 rows
  and 634 six-op runs that return bodies now (an upper bound: the
  census's criterion also catches rims and flush contacts). U-T4's row 3
  is the obligation's pin.
- **Q2 Where the general pose refuses** (answered: at the frame). A
  frame exists for two loops round the axis (§1.2's proof), and the
  cylinder × sphere arm served a frame for a quartic and refused at the
  dispatch (its frame retired in SECT step 1a; the pair now reads its
  section). Here no later door can use the
  frame: the table routes the quartic to the general rung and no carrier
  holds it. So the first door that reads the table owns the refusal, as
  the cone's hyperbola refuses at its frame. The proof is kept for the
  day the general rung retires.
- **Q3 The general pose's refusal type** (answered: a new
  `GermSectionSpiric`). Not `GermFrameUnsupported`, which says the kind
  pair has no arm, and it will have one. Not `GermSectionOpenConic`:
  the hyperbola's arc is minted (C1) and only the Boolean's crossing
  layer for a NURBS carrier is missing, while the spiric quartic's
  general rung is unbuilt (the table's own text: "blocked on the
  torus's exact meters conversion"). Its recourse differs too: the
  spiric quartic's is the torus's meeting recourse, the hyperbola's
  says cut all the way round.
- **Q4 Two loops on one face pair** (answered: the germ's loop, by
  margin). The cylinder pair's intersecting axes need a frame per
  ellipse, because no one frame serves both: each germ carries its
  branch, decided where it is minted.
  Here each loop has a proven frame and the germ's loop is decided
  definite at every site (§1.1), so refusing would turn away M1, B1 and
  B2 with no gain. The alternative, a frame chosen at the face's
  reading point `at`, is the mutant of U-T1's row 1.
- **Q5 The frame's extent** (answered: the torus face's reach, as the
  cone's). Answered on the cone spec's Q4 argument: a lever is sound if
  it bounds the consumed region, and the face's reach does.
- **Q6 The same loop question on the cylinder × sphere coaxial pair**
  (answered by measurement: not live; the frame retired in SECT step 1a,
  and the pair now matches by foot parameter on its two circles). The
  coaxial frame served one frame
  for two circles, and nothing in the frame stops `nearer_along` ranking
  across them where both lie on one face pair. Measured in U-T1 on a
  `0.5` rod (`germ_pair::cyl`) through a ball centred on its axis, at
  radii `1` and `0.52` (circles `1.73` and `0.29` apart, the second
  nearer than the half-turn along either), the ball also moved `0.3` up
  the axis, the rod spun 1 rad, and the ball's chart turned a
  quarter-turn: each circle holds two sites, and every segment of
  ∪, ∩ and both ∖, in both member orders, joins two sites of one circle.
  No row is filed. `germ_plane_torus_frame`'s
  `the_coaxial_cylinder_sphere_pair_keeps_each_circle_to_itself` pins
  the nearest pose.
- **Q7 The row's chain is stale in three places** (§0.1): the lily
  stops at the dispatch, not the frame; TANG's pose stops at the wall
  side's section once past the frame; and the corner bar's barrier is
  the general pose only on tori with `R ≥ r + s`. The row should cite
  §0.1.
