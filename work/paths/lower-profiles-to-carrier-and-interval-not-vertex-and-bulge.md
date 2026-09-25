---
id: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
kind: issue
title: A profile lowers to vertex + bulge, which cannot express a full turn, so every circle is split in two and the lowered pieces diverge from the authored path; lower to one canonical carrier + interval form instead
status: open
opened: 2026-09-25
priority: P1
cost: H
needs_ev: true
---


## Where this came from

EMIT filed this row from Ev's thread on #3202 (2026-09-25). Ev asked:
"we already have lots of ways of expressing arcs in the path algebra; is
this another case of divergence from the lower-level description? …
maybe fixable … by switching to some other single canonical
representation that can express this case". Ev also asked for it to go
to an orchestrator who can give it full attention. No PATHS
orchestrator was active on 2026-09-25.

## The finding

A profile lowers every segment to vertex + bulge. Bulge is tan(θ/4),
which diverges at θ = 2π (`crates/profile/src/path.rs`,
`CircleSplitCount`), so no segment can be a full turn:
- `circle_kernel` splits a circle into two semicircles at ±x;
- `validate` refuses loops with fewer than two vertices
  (`build_loop_segs`).

This is ratified as the "M2 closed-carrier precedent" in
`docs/PATHS-DESIGN.md` §5.1.

The B-rep has no such limit. A full revolve's rims are full-period
self-loop edges (`sweep/src/revolve/mod.rs`), and STEP import builds
them too. Certification, mesh and naming all handle them. Nothing in D1
or topo requires two edges.

What the split forces:
- **Artefacts:**
  - a seam vertex pair and a strut on every cylinder at ±x;
  - the naming anchor's n = 2 orientation case (`eval/anchor.rs`);
  - demos using `circle_split(3)` to dodge semicircle trouble in
    booleans (`demos/tour/src/bossplate.rs`, `twopeg.rs`, `lily.rs`).
- **A divergence:** the lowered pieces no longer line up one-to-one with
  what the author wrote, beyond the merges the geometry itself makes.

## Direction

Lower to one canonical segment form that can express every authored
arc: a carrier (line, or circle with centre and radius) plus a
parameter interval, where a sweep of 2π is legal. Bulge becomes a
derived view rather than the storage.

It needs a survey first. Bulge is read deeply: area sums, joins,
merge_faces, validation and canonicalization. A periodic wall still
needs one seam edge to cut its parameterization (`gate_maximal_faces`,
`boolean/reduce.rs`). Extrude, loft and sweep would build that wall.
The size is roughly 15–25 files across `profile`, `sweep` and
`editor-core`, plus persistence and Python.

This is a design change to ratified PATHS-DESIGN §5.1, so it goes to Ev
as an `[ev]` PR once surveyed.

## Coupling

EMIT's step-id names (`work/emit/profile-pieces-are-named-by-minted-step-ids.md`)
spell a circle as `Piece(0)`/`Piece(1)`. When this lands, a circle is
one `Carrier`, `Piece(1)` vanishes, and the saved format breaks a
second time.

## Survey (PATHS, 2026-09-25)

A read-only lane's census, kept verbatim as the evidence behind the `[ev]` PR. Headings are demoted one level.


Row: `work/paths/lower-profiles-to-carrier-and-interval-not-vertex-and-bulge.md`.
Read-only survey, merge base = the session checkout of 2026-09-25. It is a
shallow clone, so `git log -S` could not confirm when D1's "Profile format"
clause was ratified. Citations are by symbol name. A line number is given
only as a hint.

### 0. Corrections to the row as filed

1. **D1 is touched.** The row says "nothing in D1 or topo requires two edges".
   Topo does not require it. D1 does: `docs/DESIGN.md` D1's **"Profile
   format"** bullet reads "closed carriers split into ≥ 2 vertices", and
   also "zero representation-consistency conditions by construction".
   So the `[ev]` PR changes three texts: D1's profile-format bullet,
   PATHS-DESIGN §2a.1 and §6 PQ4 (the M2 closed-carrier precedent), and
   the profile crate's README and lib.rs header ("θ ∈ (−2π, 2π) exclusive
   by construction … Closed carriers therefore need ≥ 2 vertices
   (ratified)").
2. **The bulge form has a second home in geom-brep.** The B-rep's own
   description layer stores it: `geom_brep::SketchSegment::Arc { a, b,
   bulge }` inside `MappedCurve::PlacedSegment` (`geom-brep/src/mapped.rs`,
   "the ratified zero-redundancy bulge form"). Every profile-derived edge
   is minted through it (`swept::sketch_segment`, `placed_segment_spec`).
   A full-turn rim edge cannot be described in it, so the change reaches
   `geom-brep`, `topo` (`split.rs` via `SketchSegment::restrict`,
   `offset_axial.rs`, `replace_face.rs`, `transform.rs`) and certification
   (`certify.rs`).
3. **The saved format does not store the lowered form.** It stores
   programs. `editor-core/src/persist/mod.rs` says so: "the profile
   programs' REPLAYED SEGMENTS (vertices/bulges/joints…) … re-derive on
   replay". Bodies are not persisted either (`kernel_wire` has no
   descriptions). The format break is therefore purely a NAMES break:
   `ProfileEdgeRef`/`ProfileVertexRef` segment indices today, EMIT's
   `{step, role}` tomorrow. A circle has segments 0 and 1 today and will
   have segment 0 only.
4. **The symbolic tier depends on the unit bulge.** The interval/
   certification tier leans on the circle's literal bulge 1:
   `geom-core/src/sym.rs` and `sym/trig.rs` ("θ = 4·atan(bulge)",
   SketchSegment::eval) and DECIDE's
   `work/decide/rule-d-reaches-the-unit-bulge-only.md` ("every measured
   document authors its arcs through Circle/CircleSplit, whose kernel
   bulge is the literal 1"). A one-segment circle with Δθ = 2π changes
   what those rules see.

### 1. Reader census

Grep patterns, in the first pass:
(p1) `bulge`, counted per file;
(p2) `ProfileVertex|SegmentKind|ValidatedSegment|\.bulge\b|\.bulge\(\)|\.segments\(\)|blend_arcs|bulge_from_(center|via)`
over `crates/*/src demos`;
(p3) `\.vertices\(\)`;
(p4) `SketchSegment|PlacedSegment`;
(p5) `n < 2|n == 2|len\(\) (<|==) 2|>= 2|two-vertex|2-vertex|semicircle`.

**What these cannot match.** They miss closed-form re-derivations spelled
without the word "bulge": `4·atan`, `tan(θ/4)`, apothem formulas. They
also miss readers that depend on a circle being two pieces without
naming bulge: face, edge and vertex counts in census assertions,
`vertex_count == 2`, `Piece(1)`, loft section-count matching.

**Second pass, aimed at that gap.**
(s1) `(4\.0|four|from_f64\(4\.0\))\s*\*\s*…atan\(\)|/\s*4…\)\.tan\(\)`.
This found `geom-core/src/sym.rs`, `sym/trig.rs`, `topo/src/offset_axial.rs`,
`profile/src/path/verbs.rs`, `mesh/src/walk.rs` and the test_support
fixtures, none of which (p1)–(p4) had framed as readers.
(s2) Text search of `work/`, `docs/` for `semicircle|two-vertex|circle_split(3|CircleSplit { n: 4`.
This found three cross-program rows (§6).

**Residual blind spot.** Test assertions that count faces, edges or
vertices on circle-authored bodies. They cannot be grepped by shape
because the numbers are bare literals. The honest census is to run the
suite after unit 3 and read what reddens.

Scale, measured:
- 240 files and 1355 `ProfileVertex::new` call sites build loops through
  the raw fixture door (`RawLoop`). 218 of those files are in `sweep`.
- 102 hand-built "bulge-1 two-arc" pairs sit in 77 files.
- 60 test files call `circle(`.

#### 1a. Construction (writers of the form)

| Site | What it does with bulge | Under carrier + interval |
|---|---|---|
| `profile/src/lib.rs` `ProfileVertex{pos,bulge}`, `ProfileLoop{vertices,tangent_joints}`, `RawLoop::{new,polygon}`, `ProfileLoop::reversed` (b ↦ −b), `map_scalar` | This is the storage itself. | The type changes. `reversed` negates the sweep instead of the bulge. |
| `path.rs` `Core::{seed,set_leaving,push_line,push_arc}`, `step_spans` | Emits the chain. Leaving bulge is set per vertex. | Straightforward rewrite: emit the segment the verb already knows (centre/radius are in hand for Center, fillet and circle). |
| `path.rs` `circle_kernel` (2 vertices at ±x, bulge 1), `circle_split_kernel` (bulge = tan(π/2n), `n < 2` ⇒ `CircleSplitCount`) | The lowering that forces the circle split. | `circle` becomes 1 segment. `circle_split` survives as authored data. `n ≥ 1` or `n ≥ 2` is choice (c). |
| `sugar.rs` `bulge_from_center` (atan2 of endpoints ⇒ tan(θ/4)), `bulge_from_via`, `fillet_bulge` | Encodes an arc whose carrier is known into a bulge. That encoding loses the carrier's bits. | Stores the carrier directly. The bulge becomes a derived view. |
| `path/verbs.rs` (`(signed/4).tan()`), `path/family.rs` `push_arc`/`set_leaving` callers, `path/arc_fillet.rs` `bulge` fields | The same encoding at the Sweep, ArcLen and arc-fillet legs. | Same as above. |
| `path.rs` `fillets_carry_their_tangency` (re-runs `seg::build_seg` on the stored bulge ⇒ `FilletArcFlattenedInStorage` / `…BelowSceneResolution`) | Checks that the stored bulge still is the fillet arc. | Mostly disappears for the radius. Endpoint-on-carrier and joint-tangency checks remain (§4). |
| `lift.rs` `carrier_form`, `chain_form`, `arc_carrier` (a 4th closed-form copy) | Lifts a vertex+bulge loop to a program. `n==2` ⇒ `Step::Circle`. | Needs the one-segment case. Stays an input-form tool if (d) keeps bulge as input. |
| Test fixtures (`RawLoop`, `sweep/src/test_support.rs` 62 hits, `mesh/src/{curved,walk}.rs` tests) | Hand-built chains. | Unaffected if the raw door keeps a vertex+bulge input form (choice (d)). This is the strongest code argument for keeping that door. |

#### 1b. Profile validation (`profile/src/validate.rs`, `seg.rs`)

| Site | Use | Under carrier + interval |
|---|---|---|
| `build_loop_segs` (`n < 2` ⇒ `TooFewVertices`) | Arity. | Becomes `n < 1`. Needs a one-segment case. |
| `seg::build_seg` (`vertex_separation` on the chord, `segment_straightness` via sagitta L·b/2, `arc_diameter_clearance` ⇒ `NearFull`) | Classification. It is entirely chord-based. | A full turn has chord 0 and is refused as `DegenerateSegment`. It needs its own `SegKind` arm with no chord. For chord arcs a *derived* bulge suffices, and the byte identity of margins needs the same arithmetic. |
| `seg::arc_carrier` (centre/radius from L, b) | Derives the carrier. | Removed if the carrier is stored, or kept as the derivation for Bulge-mode input. |
| `seg::{arc_span, line_arc, arc_arc, ray_crossings}` (apex / `span_chord` chordal defect) | Pair simplicity and parity. | A full circle has no apex or span. It needs a "whole carrier" arm, which is simpler: span is always true. |
| `judge_joints` | Joint tangency. | For n = 1 the only joint is a segment against itself. It must be skipped or classified as carrier identity (circle declares nothing today). |
| `loop_orientation` (θ = 4·atan b; r²(θ − sin θ)) | Area sum. | Reads θ from the stored sweep. It works at 2π: chord term 0, area 2πr²/2. |
| `canonicalize_loop` (re-`build_seg` on the reversed chain; `ValidatedSegment{start,end,bulge,kind}`) | Canonical form. | `ValidatedSegment` is already nearly the target: endpoints plus `SegmentKind::Arc{center,radius,turn}`, with the span documented as "θ = 4·atan\|bulge\| … sanctioned re-inspection". Adding `sweep` and a full-turn case makes it the canonical form. |
| `ValidatedSegment::lift` | Rebuilds the carrier at U from endpoints + bulge. | Lifts the stored carrier directly. |
| `ValidatedLoop::blend_arcs` doc | Says "a full circle lowered as two tangent semicircles … is listed too". | That prose goes. |

#### 1c. Sweep (`crates/sweep`)

| Site | Use | Under carrier + interval |
|---|---|---|
| `swept.rs` `SweptSeg{a,b,bulge,kind}`, `SweptChord::bulge`, `swept_segments` (negate on reverse), `sketch_segment` | The traversal record. | Carries the sweep, not the bulge. |
| `swept.rs` `arc_apex(a,b,bulge)`, `arc_span(bulge)`, `placed_segment_spec` (param_end = 4·atan\|b\|), `register_span_identity` | Rim edge specs. | param_end = \|Δθ\|, which is 2π for a circle. Apex = carrier point at mid-sweep. |
| `swept.rs` `cap_points` ("apexes keep 2-vertex loops plane-determining") | Newell cap plane. | A one-segment circle gives vertex plus apex, which is 2 points and not plane-determining. A full-turn arm must add carrier points, or use the sketch normal. |
| `swept.rs` `cosurface` | Carrier identity. | Unchanged. n = 1 compares a segment against itself. |
| `extrude.rs` `extrude` phase 1 and holes (`mvfs` + `mev(Lone)` + `mev(Fan)…` + `mef(Chords)`, `1 % n`) | Lamina chain. | n = 1 needs `mef(MefSite::Lone)` (outer) and `kemr` ring + `mef(Lone)` + `kfmrh` (holes). |
| `extrude.rs` `sweep_loop` ("n == 1 is unreachable (validated loops have ≥ 2 vertices)"), join arm `k_prev == k_next ⇒ describe_at_rest` ("NOT as the chart's SEAM") | Walls and struts. | At n = 1 the lone strut has both halves in one face. It is the u_ref meridian (`side_surface` builds `u_ref` from `qs[j]`), so it must be described `EdgeDescriptionSpec::seam(wall)`, as revolve's `upgrade_meridian_seam` does. |
| `extrude.rs` `wall_segments`, `wall_sense_of`, `WallSeg::bulge` | Wall record. | Straightforward rewrite. |
| `revolve/chain.rs` `build_chain` (same n ≥ 2 mev chain) | Lamina. | Same `mef(Lone)` case. |
| `revolve/axis.rs` `radial_extent` (membership margin `−bulge·((p−a)·n)` on the chord normal), `axis_arc_span` (`π − arc_span(bulge)`), `axis_arc_apex` | Axis classification. | The chord is 0 for a full turn, so the margin is poison. Needs a full-carrier arm. |
| `revolve/tube.rs` (hand-built two-arc traversal, "no profile→bulge→radius arithmetic") | A door that exists because the bulge loses the radius (56 ulps). | Collapses to a one-segment traversal. Its exactness rationale is met by the profile itself once carriers are stored (§4). |
| `skin.rs` `vertex_segment` (classifies by `a.bulge() == 0.0`, **not** by `kind`) and `segment_curve` (a 5th closed-form copy, `mul_add` centre) | Loft walls. | Build the NURBS from the carrier + sweep. Note: `vertex_segment` already breaks `ValidatedSegment`'s "select carriers by kind, never by re-inspecting the bulge" rule. |
| `skin.rs` section-shape check (`vertices().len()` equal across sections); `loft.rs` `assemble` (third copy of the lamina chain) | Loft. | n = 1 works topologically. See §2 for the tessellation interaction. |

#### 1d. geom-brep, topo, geom-core

- `geom-brep/src/mapped.rs` `SketchSegment::{Arc{a,b,bulge}, eval, restrict}`
  (restrict: `tan(atan(b)·(s1−s0))`), `certify.rs`. This is the B-rep
  description layer, so it needs a full-turn form (choice (a)).
- `topo/src/split.rs` (`SketchSegment::Arc` restrict),
  `offset_axial.rs` (`(θ/4).tan()` rebuild), `replace_face.rs`,
  `transform.rs`. These read descriptions. They are straightforward
  rewrites once `SketchSegment` changes.
- **Topo is otherwise agnostic.** `loop_winding` sums per-edge
  `(Δ − sin Δ)` terms and handles one-edge cycles.
  `boolean/reduce.rs::gate_maximal_faces` skips `f1 == f2` ("seam/strut
  inside one face") and accepts same-key curved adjacency. The
  merge_faces bulge hits are 3-D conic terms, not profile bulges.
- `geom-core/src/sym.rs` and `sym/trig.rs`: identity rules keyed on the
  literal-bulge `4·atan b` atoms. Needs re-keying on stored angles
  (§6, DECIDE).

#### 1e. editor-core

- `eval/anchor.rs::derive_naming`: bit-matches canonical against program
  loops on positions, bulges and joints. The **n = 2 paragraph** exists
  because positions cannot decide parity at n = 2. At n = 1 positions
  decide nothing, and the stored sweep's sign alone decides parity. The
  rule survives with the sweep in place of the bulge.
- `eval/anchor.rs::signed_area`: a 6th closed-form copy (r² from
  chord/sin(θ/2)). It divides 0/0 at a full turn and must read
  carrier + sweep.
- `stackup.rs` value digest (`d.scalar(v.bulge())`): an in-memory digest,
  so re-spelling it is straightforward.
- `program.rs` `CheckedRecords::new(…, replayed.vertices().len())`,
  `profile_edges_of`; `LoopProgram::Circle` is 1 step (program-level,
  unaffected).
- `names/emit_sweep.rs::resolve_chain_opt` (`n < 2` ⇒ `NamingError::Emission`):
  revolve naming needs a one-vertex case (rim and meridian are both
  self-loops at one vertex).
- `eval/wire.rs::SWEEP_FRONTIER`: the Node::Sweep refusal's *premise*
  is "a closed chain has two or more segments — even the minimal
  two-vertex loop". With a one-segment circle that sentence is false, so
  the frontier must be re-worded. Whether a circle path should open the
  lane is a SWEEP question.
- `persist/`: no lowered data. The break is names only (§0.3).

#### 1f. Viewer, pncad, pncad-py, demos

- `viewer/src/sketch.rs::flatten`: a 7th closed-form copy. It walks
  vertices and bulges, so it needs the full-turn arm.
- `pncad/src/{prelude,profile}.rs`: re-exports `ProfileVertex`,
  `SegmentKind`, `ValidatedSegment`, `bulge_from_center/_via`. The API
  surface changes and pncad's facade completeness guard sees it.
- `pncad-py`: `ClosedLoop.vertex_count` (the circle's count goes from 2
  to 1: `tests/test_paths.py` asserts `vertex_count == 2` at ~l.155);
  tags `circle_split_count` and `too_few_vertices` (`tags.rs`); the
  `Bulge(p, b)` authoring mode (an input form, which stays);
  `pncad.pyi` `circle`/`circle_split` signatures (unchanged). No Python
  reader of the lowered bulge exists.
- Demos:
  - `demos/tour/src/bodies.rs::circle`, `curvedcut.rs`, `ring.rs`,
    `klein.rs` use `circle()`. Their census numbers move in unit 3.
  - `rocker.rs` uses `blend_arcs`/`SegmentKind`, which is unaffected.
  - `bossplate.rs::boss` and `twopeg.rs::rim` use `circle_split(3)`
    **deliberately**. Their own comments say the three-face rim seam is
    what the stop tests. These are not workarounds and stay.
  - `lily.rs::foot` uses `circle_split(3)` with the reason "a boolean
    operand's curved wall must be maximal-faced". A one-face wall
    satisfies that, so it is a real migration candidate.
  - `teapot.rs` uses `CircleSplit{n:4}` to dodge the loft C⁰ crease. A
    one-segment circle makes this worse (§2).
  - `demos/wild`: no circle authoring found by (p2) or (s1).

### 2. The one-edge loop through the solid builders

**Existing operators that already build full-period self-loop edges.**
- `topo` `MefSite::Lone`: ch. 9.8b "circular edge from a lone vertex",
  documented in `euler.rs` and exercised in `seqgen.rs`/`iso.rs` tests.
- Full revolve: struts are full-period latitude rims (`revolve/full.rs`
  "struts are full-period latitude rims", zipped by `mekr`/`kev`). The
  meridian seam is described by `revolve/upgrade.rs::upgrade_meridian_seam`
  → `EdgeDescriptionSpec::seam(wall)`.
- STEP import: `step-import/src/assemble.rs` builds self-loops at a lone
  vertex, plus the tied self-loop splice.

All are reusable. `mvfs` + `mef(Lone)` is exactly the one-segment
lamina. `upgrade_meridian_seam`'s body is the seam description extrude
and loft need. The one warning is `iso.rs`
`the_circle_route_to_the_pillow_moved_an_edge_off_its_carrier`: an `mev`
fan off a self-loop vertex is refused, so a builder must never re-base
the circle's vertex after the fact.

| Builder | Today (two semicircles at ±x) | Must build (one closed-carrier segment) |
|---|---|---|
| extrude | 2 vertices per rim, 2 struts, 2 wall faces sharing one cylinder key (`side_surface` wrap run), 2 top-rim arcs, 2 bottom-rim arcs; struts `describe_at_rest` | 1 vertex per cap, 1 bottom-rim and 1 top-rim self-loop edge (param 0..2π), 1 wall face, 1 strut whose both halves bound the wall (seam, `EdgeDescriptionSpec::seam`). Cap plane from the sketch normal or ≥ 3 carrier points. The rim upgrade (`upgrade_rim`) witness is at `carrier(π)`, which works. |
| loft | 2 NURBS walls per circle section, each a semicircle skin; 2 struts | 1 wall whose u is a full-period rational NURBS, 1 seam strut. **Interaction:** `segment_curve` splits at `MAX_SUB_ARC = π/2`, so a full circle is 4 quarter Béziers joined at multiplicity-2 knots. That is the C⁰ crease the tessellator refuses (`work/tess/lofted-circle-sections-are-unmeshable…`). Today's two-semicircle loft already hits it. The one-segment loft hits it at 3 interior knots. The option is to let the loft emitter split the wall at knots (the tess row's first remedy), or to fix tess. That is a precondition for any loft over a circle. Section-shape matching becomes 1 against 1. Circle-to-polygon lofts still mismatch, as today. |
| sweep (`sweep_body`) | Same as loft (skin) | Same as loft. |
| revolve partial, circle off-axis | Lamina of 2 arcs; wedge caps; 2 torus faces | Lamina `mvfs`+`mef(Lone)`; 1 strut (latitude arc); 1 torus-patch wall bounded by start meridian (self-loop, shared with the start cap), strut, end meridian, strut. `build_chain` needs the n = 1 arm. |
| revolve full, circle off-axis (torus) | `tube.rs` hand traversal or a 2-arc profile ⇒ 2 faces | 1 vertex, 2 edges (meridian circle and latitude circle, both self-loops, both seams), 1 face. V−E+F = 0, genus 1 ✓. `emit_sweep::resolve_chain_opt` refuses n < 2 today. |
| revolve, circle centred on the axis (sphere) | 2 arcs, each ≤ π, apex tests (`axis_arc_span`) | A full circle crosses the axis, so it keeps refusing `ArcCrossesAxis` via a full-turn arm. A sphere is still authored as a semicircle plus an on-axis chord. |

**Other places checked for ≥ 2-edge assumptions.**
- `gate_maximal_faces` and `reduce.rs`: fine (the `f1 == f2` skip).
- Naming anchor: see 1e. The n = 2 paragraph is replaced by "the sweep
  sign decides parity".
- Certification and mesh operate on the B-rep. They already accept
  revolve's one-face periodic bands with seam and self-loop rims. That
  is the same topology an extruded one-segment cylinder produces.
- `loop_winding`: fine at one edge.
- Not verified: whether the boolean SSI and splitting path has ever
  been exercised on an **extruded** periodic wall with a seam strut
  (revolved ones exist in the corpus). This needs a red-first row in
  unit 2.

### 3. Design choices (questions for Ev)

**(a) Segment shape.**
- A1: keep chord + bulge for every partial arc and add a `FullCircle{centre}` segment at a lone vertex. This is minimal but gives two arc forms, which is not "one canonical form".
- A2: vertices stored verbatim plus a per-segment `Line | Arc{centre, radius, sweep Δθ signed}`. A full turn is one vertex with |Δθ| = 2π, and the bulge is derived as tan(Δθ/4).
- A3: pure carrier + interval, `Arc{centre, radius, θ0, Δθ}`, with endpoints derived by sin/cos.

Code evidence:
- A3 breaks PATHS §2a exactness contract 1 ("authored points are stored verbatim").
- A3 breaks the anchor's bit-matching and shared-vertex adjacency: two carriers' sin/cos endpoints do not coincide bitwise.
- A2's redundancy (endpoints on carrier, Δθ agreeing with endpoints mod 2π) collides with D1's "zero representation-consistency conditions". But `ValidatedSegment` already carries exactly this redundancy (endpoints plus centre, radius and turn) and validation is the gate that verifies it.

Lines: a line's carrier is its chord. Nothing is gained by a (point, dir, t0, t1) parameterization, and the endpoints stay authoritative. **Recommendation: A2.** Endpoints are authoritative. Arc carriers are stored, derived at lowering for Bulge and Via input, and taken from the construction for circle, Center, fillet and the arc-fillet family. The signed sweep is stored. A new validation predicate verifies endpoints on carrier (a length margin) and replaces the chord-derived carrier. D1's clause is re-worded from "zero consistency conditions" to "consistency conditions verified at validate".

**(b) Exactness / D9.**
- Today the bulge is authored (Bulge mode), `tan(θ/4)` of an atan2 difference (`bulge_from_center`), `fillet_bulge`'s closed form, or the literal 1 (circle).
- The carrier is re-derived by `seg::arc_carrier` from (L, b), and re-derived *differently* in `skin::segment_curve` (mul_add), `geom-brep` `SketchSegment::eval`, `anchor::signed_area` and `viewer::flatten`.
- Vertex positions never change under A2, because endpoints are stored.
- Bits move where a consumer switches from a bulge-derived carrier to a stored one. For arcs whose stored carrier is *computed by `seg::arc_carrier` at lowering* (Bulge and Via input), validate-side centres and radii are bit-identical to today. For circle, Center and fillet arcs, storing the constructed carrier moves every downstream radius and centre by ulps (tube.rs's 56-ulp story). That movement is the point.
- `param_end`: if Δθ is computed at lowering as `4·atan(b)`, rim intervals are bit-identical (`arc_span` = `4·atan|b|`; atan is odd).
- Loft walls move once `segment_curve` reads the stored carrier, not its own mul_add derivation.

Recommendation: Unit 1 lands the form with carriers derived exactly as `seg::arc_carrier` derives them and Δθ = 4·atan(b), byte-identical. A later unit switches to constructed carriers and re-baselines. "Endpoints computed from angles" is refused.

**(c) Seam and vertex of a one-segment circle.** `circle(c, r)` must pick a vertex.
- Options: +x (angle 0, today's first vertex, so `Piece(0)`'s start is unchanged); an authored phase; or `circle_split(c, r, 1, phase)`.
- Evidence: `circle_kernel`'s first vertex is `(c.x + r, c.y)`, and extrude's `u_ref` is built from the lamina vertex, so the seam sits there. The lift's `carrier_form` already searches rotations.

Recommendation: +x for `circle`, and `circle_split` survives as authored data with `n ≥ 1`. `n = 1` gives an explicit phase and `n = 0` keeps `CircleSplitCount`. Whether `circle_split(1)` is legal or redundant with `circle` is Ev's call. `circle_split` must survive either way: bossplate and twopeg author three-face seams on purpose, and EMIT names `circle_split` pieces `Piece(k)`.

**(d) The raw vertex + bulge door.**
- Options: keep it as an input form lowering into the canonical form; or retire it.
- Evidence: 240 files and 1355 call sites (218 files in `sweep`) use `RawLoop`/`ProfileVertex::new`. `lift.rs` reads the form. PATHS §6 names "raw vertex+bulge chain" as an authoring form. `Bulge(p, b)` is a public verb in Rust and Python.

Recommendation: keep it as an INPUT form, with `ProfileVertex` as the input record. The emission layer and `RawLoop` lower it to canonical segments, with the carrier from `arc_carrier`. A full turn is not expressible through it, which is fine because `circle` is the door. This keeps the fixture corpus unchurned.

**(e) Persist break and EMIT coupling.**
- Evidence: only names are persisted (§0.3). EMIT's `profile-pieces-are-named-by-minted-step-ids` ships a wire break with circle = `Piece(0)`/`Piece(1)` (Ev, #3202). A second break would re-spell it `Carrier`.
- Options:
  - (e1) Sequence this row's unit 3 before EMIT's wire break lands, so one break covers both.
  - (e2) Land EMIT first and take a second break.
  - (e3) Have EMIT spell the circle `Carrier` from day one. The replay record maps both lowered semicircles to role `Carrier`, which is a 1→2 role that EMIT's "earlier piece names it" rule does not cover.

Recommendation: e1 if EMIT has not started its wire unit, otherwise e2. This needs Ev and the EMIT orchestrator. The pre-EMIT names (`ProfileEdgeRef{segment}`) break regardless when the circle goes 2→1.

**(f) Do lines move too?**
- If A2 is taken, lines are `Segment::Line` with no carrier data. Byte identity is trivial because the chord is the carrier.
- Moving lines to (origin, dir, t0, t1) buys nothing and costs the verbatim-endpoint contract.

Recommendation: lines keep endpoints. The unifying piece is the *segment enum*, not a line parameterization.

**(g) An additional choice: what `SketchSegment` in geom-brep becomes.**
- Options: mirror the profile's segment (`Arc{a, centre, radius, Δθ}` or `Circle{…}`); or add only a `FullCircle` arm.
- `restrict` today computes `tan(atan(b)·(s1−s0))`. With a stored Δθ it becomes `Δθ·(s1−s0)`, which is simpler and exact in structure.
- It is also what the symbolic tier keys on (§0.4).

Recommendation: mirror the profile form. This is a second design page (geom-brep README / D2's scaffold description) and must be listed in the `[ev]` PR.

### 4. What disappears and what stays

**Disappears.**
- The ±x two-semicircle split in `circle_kernel`.
- The `validate` `n < 2` arity (becomes `n < 1`).
- `CircleSplitCount` at n = 1.
- "n == 1 is unreachable" in `extrude::sweep_loop`.
- `cap_points`' apex-for-two-vertex rationale.
- `derive_naming`'s n = 2 paragraph.
- `SWEEP_FRONTIER`'s "minimal two-vertex circle" premise (re-worded).
- `blend_arcs`' semicircle caveat.
- The strut pair and the two-face cylinder on every `circle()` cylinder.
- The duplicate wall faces sharing one key.
- The seven hand re-derivations of the bulge closed forms:
  `seg::arc_carrier`, `path.rs::arc_carrier`, `lift.rs::arc_carrier`,
  `skin::segment_curve`, `SketchSegment::eval`, `anchor::signed_area`,
  `viewer::flatten`. Consumers read the stored carrier.
- `revolve/tube.rs`'s hand-built two-arc traversal. The door itself stays
  for world-frame intent.
- The fillet storage-loss class: `FilletArcFlattenedInStorage` for
  radius, and the "radius recovered from (t1, t2, bulge)" oracle in
  `profile-fillet-radius-off-at-eps-1e-6`, likely.
- `skin::vertex_segment`'s `bulge == 0.0` re-inspection.
- `lily.rs::foot`'s `circle_split(3)`.

**Stays.**
- The periodic wall's seam edge. It becomes a Seam-state strut,
  `gate_maximal_faces`' `f1 == f2` arm.
- `NearFullArc` / `arc_diameter_clearance` for chord arcs whose ends are
  distinct but near-coincident.
- Endpoint-on-carrier verification. This is new under A2 and replaces
  chord-derived carrier trust.
- Joint tangency classification and `FilletCarrierBelowSceneResolution`.
- `circle_split` as authored data.
- The raw bulge input door.
- The Sweep and ArcLen legs' refusal of |θ| ≥ 2π inside a chain: a full
  turn at a chain vertex is non-simple.
- Bossplate's and twopeg's `circle_split(3)`.
- Teapot's `CircleSplit{4}` until the tess crease row lands.

### 5. Unit cut

| # | Unit | Size | Depends | Goldens |
|---|---|---|---|---|
| 0 | `[ev]` PR: re-word D1 "Profile format", PATHS §2a.1 + §6 PQ4 precedent, profile README/lib.rs header, geom-brep `SketchSegment` docs; choices (a)–(g) | D | — | none (docs) |
| 1 | Canonical segment type in `profile` (A2): vertices + `Segment{Line \| Arc{centre, radius, sweep}}`. The emission layer and `RawLoop` lower bulge input with `seg::arc_carrier` and `4·atan b`. The bulge becomes a derived accessor. Every reader in 1a–1f moves to kind + sweep + carrier, except geom-brep. | H | 0 | **byte-identical** by construction (same arithmetic at lowering). Readers that re-derive differently (`skin::segment_curve`, `anchor::signed_area`, `viewer::flatten`) keep their formula through the derived bulge in this unit. |
| 2 | geom-brep `SketchSegment` full-turn form + `restrict`/`eval`/certify, plus the topo description readers (`split`, `offset_axial`, `replace_face`, `transform`) and the sym re-keying | H | 0, 1 | byte-identical for partial arcs if eval keeps its formula. The sym rows (`m10_*_interval`) may move, to be measured. |
| 3 | Admit the one-segment closed loop: validate arm (`SegKind` full turn, pair, parity, orientation, joints), extrude/revolve/loft n = 1 lamina via `mef(Lone)`, seam description for the lone strut, `cap_points` full-turn arm, `axis.rs` full-turn arms, `emit_sweep::resolve_chain_opt` n = 1. Red-first rows use raw fixtures and the circle stays 2 segments. | H | 1, 2 | none (new rows only) |
| 4 | `circle` lowers to one segment; `circle_split` `n ≥ 1`; `lily::foot` → `circle`; re-baseline demo censuses (bodies, curvedcut, ring, klein), `vertex_count`, `too_few_vertices` and `circle_split_count` tags, `SWEEP_FRONTIER` prose | D | 3, and EMIT sequencing (e) | **moves goldens**: every `circle()` body's V/E/F, names, loft walls over circles. It must not ship a loft-over-circle scene until the tess crease is handled (§2). |
| 5 | Constructed carriers: Center, fillet, arc-fillet and circle arcs store their constructed centre and radius, not a bulge-derived one; delete the closed-form copies; tube.rs traversal collapses | D | 4 | **moves goldens** by ulps on every Center/fillet arc radius and centre. Expect `profile-fillet-radius-off-at-eps-1e-6` to change shape. |
| 6 | pncad / pncad-py surface: `SegmentKind`/`ValidatedSegment` re-exports, the .pyi if any accessor surfaces, Python tests | E | 4 | Python pins move |

Units 1–3 are refactors or additions with no golden movement. Units 4
and 5 move goldens. Unit 1 is the largest: about 25 source files plus
`test_support`.

### 6. Slate interactions

**PATHS.**

| Item | Relation |
|---|---|
| `sweep-arclen-legs-fold-an-over-full-angle` | Reshaped. The refusal stays, but its reason changes from "tan diverges" to "a full turn at a chain vertex is non-simple, and `circle` is the door". Its fix could land inside unit 1 as a typed refusal. |
| `profile-fillet-radius-off-at-eps-1e-6` | Probably absorbed by unit 5. The oracle recovers the radius from (t1, t2, bulge), which stops being the storage. Not proven: the error may be in t1/t2. |
| `lift-comparator-misclasses-a-declared-joint-difference` | Reshaped. The lift reads the input form. Its comparator would compare canonical segments. |
| `arc-arc-shallow-corner-legs-escalate-arc-span` | Independent in kind: a chordal `arc_span` conditioning issue. A carrier + sweep form lets `arc_span` be restated angularly, which may help. Say so, but do not claim it. |
| `arc-closer-constructed-from-arrival-tangent`, `via-and-center-arc-closers-declared-arrival` | Independent of storage. They benefit because Center and Via store the carrier. |
| `at-cannot-bind-start-in-the-path-form`, `a-fillet-refusal-names-the-step…`, `sketch-plane-holds-the-affine…`, `arc-carrier-refusal-register-misses-two-format-arms` | Independent. |
| The charter's "three spellings of the arc carrier" (`plan.md`) | Absorbed. There are seven copies, not three (§4). |

**Other programs.**
- EMIT `profile-pieces-are-named-by-minted-step-ids`: coupled (e).
- TESS `lofted-circle-sections-are-unmeshable…`: precondition for any
  loft over a one-segment circle.
- FIXTURE `the-two-vertex-bulge-one-circle-fixture-has-eight-copies`:
  reshaped. The shared fixture should become `circle()`, and the
  two-arc copies can stay as raw input.
- DECIDE/SYM `rule-d-reaches-the-unit-bulge-only`: reshaped, possibly
  dissolved. Stored angles remove the `atan b` atoms that rule D chases.
  Unit 2 must measure `m10_bulge_interval`.
- CHART `revolved-bands-reach-no-clearance-row` (cites `CircleSplit{4}`
  pegs): independent.
- GATHER `loft-path-loses-nine-predicate-families…`: its probe
  population (two-vertex circles) shrinks. Re-measure.
- The SWEEP node frontier (`eval/wire.rs::SWEEP_FRONTIER`): its premise
  changes, so the owner should be told.
