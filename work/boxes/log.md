# BOXES — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

11 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7300-7399 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

**Seam note (CONTACT-11, 2026-09-29).** A dated section on `chart-window-walk-written-twice` records where the torus walk now stands: consecutive continuity, and the box through `solid_contain::chart_polygon_box`.

Signed: (CONTACT-11 implementer)
- 2026-09-29 — Seam note from ORIGIN: filed `box-door-census-misses-a-point-free-read` on this slate, from PR 3425's sweep of `(`-terminated source needles. (ORIGIN orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/boolean/boxes.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513's second fix pass (branch `topo/every-escalation-names-its-decision`), `crates/topo/src/boolean/boxes.rs`'s test module is `pub(crate)` and its `torus_wall` fixture `pub(crate)`, so `reduce::declaration_order_rows` can build a torus sheet for the circle × torus row; nothing else in the file moves. (TOPO implementer)
- 2026-10-10 — Seam note from PIPE (PR 4486, `pipe/census-arms-and-witness-caps`): `boolean::boxes::edge_box_rule` now takes the certified `Option<&geom_brep::EdgeCurve<T>>`, and `EdgeBoxRule::ConicAmplitude` carries `params: (T, T)`. `AxialCarrier::Conic::params` is `(T, T)`, not `Option`, and the full-turn `None` arms (unreachable by construction) are gone. No box moved. (PIPE implementer, D291 lane)

## Seam note from PIPE (2026-10-10): `#757` is retired by D10

PIPE closed `S79`, which pointed at `#757`, and records this here for
BOXES to act on. `boolean-declarations-has-no-geometric-producer` asks
for a geometric producer of `BooleanDeclarations`. D10 (ratified, PR
#3990) retires the type: INTENT's `declared-pairs-retire` deletes it,
its prelude export and the declared seats on booleans. Once that row
lands, the hand-written declarers at `demos/tour/src/booleans.rs` and
`crates/topo/tests/common/mod.rs` have nothing left to produce. BOXES
decides whether to close the row now or park it on
`declared-pairs-retire`. Signed (PIPE orchestrator).
- 2026-10-10 — Seam note from PIPE S350 (PR 4482): comment-only edit in `crates/topo/src/boolean/boxes.rs`'s `every_door_that_reads_a_box_is_inventoried` prose — the two census `face_box(` calls are now described as test-module rows pinning a described net carrying poison (one of them gates). The pinned count (`census.rs`, 7) is unchanged. (PIPE S350 lane)
- 2026-10-18 — Seam note from RESTREAD (branch `restread/one-edge-box-map`, row `census-edge-reach-of-is-a-second-edge-box-map`): `boolean::boxes` gains `edge_rule_box`, the one `EdgeBoxRule → SpanBox` map, which `edge_box` (at `f64` brackets) and `census::edge_reach_of` (at the census's scalar, in its frame) both read. Its spiric arm is `geom::spiric_period_box`, the construction `geom::curves::boxes::spiric_arc_aabb` also reads now. That construction charges its root of a cancelling difference for rounding, so it encloses the period with no pad; both unrounded copies missed the arc near the fold. `EdgeBoxRule::ConicAmplitude` is now a tuple variant over `ConicArc`, and `EdgeBoxRule::Spiric` drops its `carrier` field. Both lanes' spiric boxes widen: about `32·u·S` off the fold, and near it by the root of a radicand widened at its own scale, measured at 4.7e-12 to 2.0e-7 m over the gate's fixtures. No public door reaches either lane with a spiric. The door inventory now counts `edge_rule_box(` and turbofish-spelled calls (`name::<`), and `census.rs` pins 10. Filed here: `census-lane-span-extents-are-not-self-enclosing`. (RESTREAD implementer)
- 2026-10-25 — Seam note from PRED `D292` (branch `pred/d292-directional-extremum`): `crates/topo/src/boolean/boxes.rs` stops spelling a conic arc's amplitude. `edge_axial_span`'s conic arm (`AxialCarrier::Conic` is now the lane's span) and the census's conic edge box read `geom::curves::boxes::conic_arc_extent_along` (the bracket lane its certified twin); `arc_extent` and its adversarial probe row are deleted, `ConicArc` drops `u_ref`, `BoxFrame` gains `rows`, and `the_two_box_lanes_agree_face_for_face` pins the lanes equal to rounding rather than to the subdivision charge. Census boxes and axial windows on arcs tighten by up to that charge (`a·(Δt/16)²/8`). (PRED orchestrator)
- 2026-10-26 — Seam note from WIRE (branch `wire/re-chart-rows-name-input-cells`, row `a-re-charted-operands-rows-may-name-cells-the-re-chart-minted`): `boolean/ops.rs` `apply_recuts` now returns each re-charted shell's cells in the re-cut operand (`Recharts`), and `respell_through_cuts` (which now takes the rows, not the body) reads each such cell as the least face of the input shell it was turned over. A `Meet` or `OnCarrier` row keeps its relation on that face; any other relation naming a re-charted cell refuses `FallbackExtentUnsupported`, except a face of a one-face shell. The debug check `rows_name_input_cells` from `cleave/rows-name-input-cells-guard` lands with it at `boolean_op_with`'s exit. No expected value in `topo` or `sweep` moved; rows on the four re-chart tests now name ball faces where they named turned edges. (WIRE implementer, wire-rechart)
- 2026-10-27 — **Claimed from FLUXTAIL:** `an-edges-extent-has-four-dispatchers` (P1, M), at the re-cut that closed FLUX and FLUXTAIL (`docs/doc-ledger/flux-leaves-the-tracker.md`). It is BOXES' class, one question with four homes, and was re-banded P3 → P1 on Ev's instruction. Three folds wait on it: CHARTTAIL's `chart-region-carrier-gates-read-vertices-not-arcs`, HONE's `splitting-face-extent-lever-reads-vertices-not-arcs` and LEVER's `tangent-witness-span-reads-vertices-not-arcs`.

- 2026-10-27 — **Seam (ATREST):** `atrest/aligned-arc-or-curved-faces-that-abut-are-flush` touches `crates/topo/src/boolean/boxes.rs` in its docs only: the module docs' door list says `flush_at_rest`'s one `face_box` read now boxes cylinder and sphere faces beside the planar ones.

## 2026-11-02 — picked up; cut on the extent seam

(BOXES orchestrator) Picked up; `status: active`. The slate measured 62
points against 30, so it was cut on its semantic seam first: every row
asking *what region can a face or edge occupy* stays; seven one-home
doors that ask something else went to a new program, ONEDOOR (`D95`,
`G9`, `S173`, `chart-window-walk-written-twice`,
`description-staleness-ladder-three-spellings`,
`sweep-trace-has-two-spellings`,
`the-shell-of-a-face-is-scanned-for-where-a-back-pointer-answers`), and
`signed-penetration-depth`, a clearance-engine feature, to CLEAR.
Closed `boolean-declarations-has-no-geometric-producer`: the type is
gone (D10). Re-priced `D280` M with `design: true`, and
`the-chord-dip-charge-has-two-homes` C riding with
`split-gate-sphere-zone-folds-into-face-box-rule`: its remaining home in
`boxes.rs` serves only the sampled torus window, which the fold
replaces by the closed form the split gate already reads. The P3/P4
tail stays here though it keeps BOXES over budget: those rows are arms
of the rule the step-2 design will restate, and cutting them away
would separate them from their answer.

The diagnosis behind the order (plan.md): `[one-way.places]` names no
home for a face's or edge's extent, so each site that needed one built
its own. TWICE's `extent-ball-and-reach-are-two-statements-of-one-extent`
and ONEHOME's cluster C (`three-readings-of-a-points-rounding-charge-it-three-ways`)
are the same gap seen from the verdict side; the step-2 survey reads them.

## 2026-11-02 — the extent survey (main `b3448a04bd`)

(BOXES orchestrator) A read-only survey of every door that answers *what region can a face or an edge occupy*, verbatim as delivered. It is the evidence the extent-home design fork is dispatched on; it asserts a class, so it lives here rather than in a session.

> BOXES survey: what region can a face or an edge occupy?
> 
> Checkout `baf3fe65f5`. It differs from origin/main `b3448a04bd` only under `work/`, so the code below is main's. I read the tree, not the row bodies. Nothing was edited; one scratch counting script is in the scratchpad.
> 
> **The diagnosis holds.** `[one-way.places]` has no row for this question. `[one-way.readings]` also exempts "kind-agnostic methods (boxes, BVH, subdivision, clearance)", yet every door below is a per-kind match. So the box doors sit outside both the canonical-places table and the "one reading" rule.
> 
> ## 1. Inventory
> 
> Production code only (`#[cfg(test)]` modules, `*_rows.rs` and fuzz/probe/search files excluded). Caller counts are approximate. Abbreviations: P=Plane, Cy=Cylinder, Co=Cone, S=Sphere, T=Torus, N=Nurbs/Approx, L=Line, C=Circle, E=Ellipse, Sp=Spiric.
> 
> **Face doors**
> - `topo::boolean::boxes::face_box_rule` → `FaceBoxRule`, the kind→construction rule, under `Decide`. Arms: P→BoundaryHull; Cy→CylinderSlab (falls to BoundaryHull when the axis is not decided unit); Co→ConeSlab; S→SphereWindow; T→TorusWindow; N→ControlNet. Callers: `face_box`, `census::face_reach_in`, `census/depth_gap.rs` (2).
> - `boxes::face_box` (`Decide + Bounds`) → world `Aabb`, f64 brackets, padded, poison on no claim. Arms: ControlNet→`nurbs_surface_aabb`; Sphere→`sphere_reach`; Torus→sampled `torus_window_extent` ∩ `torus_extent`; Cy→`slab_extent` clipped to the boundary hull; Co→`cone_frustum_extent` with tan = `sin.hi()/cos.lo()`; hull→per-edge `edge_box`. About 29 calls: ops.rs (8, `face_rows`), pieces.rs (7), reduce.rs (3, the C10 tree and gate), separation.rs (3), census.rs (3, Trees), glue.rs (2), `boolean/mod.rs` held-pierce, `face_boxes.rs`.
> - `topo::FaceBoxes::of` (pub) → every face's `face_box` in one BVH. The only face box `topo` exports. Caller: `sweep/blend/reach.rs` (3).
> - `census::face_reach_in` / `face_reach` (World frame), `Decide`, pub(crate) → `(lo, hi)` in any `BoxFrame`; `None` on no claim. Same rule and the same `Span` extents at T, but ControlNet→`point_hull` of the control points, conic arcs via `conic_arc_extent_along` (no outward rounding), cone tan via `half_angle.tan()`. `face_reach` callers: separating.rs (10, narrow phase), join.rs (2), carrier_pair.rs, reduce.rs, census/in_plane.rs. `face_reach_in` callers: separating.rs, `boolean/mod.rs` `witness_span`, `classify::gate_face_reach`, census.
> - `census::held_reach_in` (private) → `face_reach_in`, with a sphere's first frame row narrowed where its pole reads `Out`. Census-only tightening; one caller (census.rs:8819).
> - `boxes::sphere_reach` → a sphere face's box (Rect, Zone or whole ball), `Decide`. Read by both box lanes.
> - `splitting::classify::gate_face_reach` / `torus_window_reach` → aimed-frame box. Torus: closed-form `torus_rect_extent` once `R > r` is decided; otherwise `face_reach_in`. Split gate only.
> - `boolean::carrier_pair::face_ball` → `ExtentBall`: `ExtentBall::of_carrier` (the whole S or T ball) when there is one, else `of_box(census::face_reach)`. Callers: the carrier doors (`pair_extent`).
> - `geom_brep::ExtentBall::of_carrier` (S/T whole ball; `None` for others) and `of_torus` (R+r, no abs). `of_torus` callers: `of_carrier`, `pair_section/torus.rs`.
> - `splitting::rules::face_reach_from` → scalar reach from a point. P/Cy: each edge over its span (`Reach::span_reach_from`); every other kind: each edge round its whole carrier (`Reach::lever_from`). Plus vertices. Callers: rules.rs `face_extent` (the split lever), join.rs `frame_extent` (plane×torus), chord_join.rs, link.rs.
> - `rules::face_reach_round_from` (always round the whole carrier); `face_axial_range` (extent along an axis through `Reach::range_along`); `cone_axial_window` (widens to hold the apex unless the azimuth window is under 3/4 turn); `cone_face_reach` → `Reach::Face`. Callers: join.rs, chord_join.rs.
> - `boolean/mod.rs::witness_span` + `conic_reach` → extent along a tangent line: vertices, plus each C/E edge's whole-carrier extremes along `dir`; falls back to `face_reach_in` aimed when an edge is another kind. Ignores the face's kind. Callers: `locus_through_both_faces`, the census tangent pair.
> - `boolean::solid_contain::selection_reach` (+ `splitting::containment::loop_extent_from`) → ball about q holding the loops (vertices plus each edge's `carrier_ball`/`conic_arc_ball`). Its docs say it does not hold S/T faces. Callers: solid_contain, side_reader.
> - `side_reader::beyond` → `selection_reach`, widened by the whole S/T ball spelled `|c−aim| + |r|` and `+ |R| + |r|`.
> - `editor-core::mate::reach::face_reach` → scalar reach from the origin. P/Cy: boundary (`measure::reach_of`); Co: boundary ∨ apex; S: `|c−o| + r`; T: `+ R + r` (no abs); N: control net. Caller: `body_reach`/`part_reach` (mate lever, `eval/wire.rs` `copy_reach`).
> - `editor-core::eval::measure::reach_of` → boundary walk with `curve_reach`, kind-blind. Callers: measure.rs (2), mate (3), `checks/at_rest.rs` (4; its line-1963 caller guards P/Cy first).
> - `sweep::blend::reach::sheet_clip` → face box in a revolve's (r, z) sheet frame from per-edge boxes. L: hull of `eval(t0..t1)`; C/E: `{circle,ellipse}_arc_aabb`; N: `nurbs_curve_aabb`; Sp: `None`.
> - `topo::boolean::trace::carrier_extent` (pub) → whole-carrier (anchor, reach) across a normal. P/Co: 0; Cy, S: radius; T: `R·|a×n| + r`; N: `None`. Callers: `plane_misses`, editor-core trace.rs.
> - `geom_brep::ssi::analytic_box` → whole S/T cube (`|R| + |r|`), f64. Quadric readout.
> - `chart_region::face_boundary_points` (vertices only), read as an extent by `carrier_agreement` and `cylinder_pair_overlap`. Open row `work/charttail/chart-region-carrier-gates-read-vertices-not-arcs.md`.
> - `census::distance` (planar face) → hull of the boundary vertices. Unsure whether its `planar_face` admits curved edges.
> 
> **Edge doors**
> - `boxes::edge_box_rule` → `EdgeBoxRule`: L→Chord; C/E→ConicAmplitude; Sp→Spiric; N→ControlHull; null→NoSoundBox. Callers: census (3), depth_gap (2), in_plane (2), boxes.
> - `boxes::edge_rule_box` → `SpanBox`, generic over the lane's scalar. ControlHull folds the control points with the chord; Spiric→`geom::spiric_period_box`. Callers: `edge_box`, census `edge_reach_of`.
> - `boxes::edge_box` (`Decide + Bounds`) → `Aabb`; conic via `geom::conic_arc_aabb` (Interval, outward). About 10 calls: census (4), reduce (2), validate ring_pairs, carrier_touch, ops, `boundary_hull`.
> - `census::edge_reach_in` (`Decide`, frame) → conic via `conic_arc_extent_along` per frame row at T. Callers: separating, in_plane, `classify::edge_clears`, reduce `gate_maximal_faces`.
> - `census::Candidates::build`'s `curved_box` → N: `nurbs_curve_aabb` (no chord hull); else `edge_box`. Pass 2′ tree.
> - `boxes::axial_carrier` + `edge_axial_span` → an edge's extent along an axis. L chord; C/E via `conic_arc_extent_along(_bracketed)`; N control points; Sp and NoSoundBox **Unclaimable**. Callers: the Cy/Co arms of both lanes.
> - `geom::curves::boxes` (sole `Bounds`, Interval-lifted): `circle_arc_aabb`, `ellipse_arc_aabb`, `spiric_arc_aabb` (= `spiric_period_box` ∪ ends), `conic_arc_aabb` (dispatcher, `None` for L/N), `nurbs_curve_aabb`. Plus `conic_arc_extent_along` (Real) and its `_bracketed` twin. Callers: topo boxes and census, sweep reach and surgery, step-import `knot_split::piece_box` (which maps kinds itself: "there is no body yet").
> - `geom::spiric_period_box` (Real) → the box of an oval's whole period. Callers: topo boxes, geom `spiric_arc_aabb`.
> - `geom_brep::extent::edge_reach_from` / `edge_extent_sup` / `edge_reach_bounds` (`SpanLocate`) → reach from a pivot, or a diameter. L ends; C exact arc; E `conic_arc_reach`; Sp/N the lesser of the whole torus or net and a 16-sample Lipschitz bound. Callers: carrier_pair (rest locus), contact_verify, offset_derive, fate, census/curved, validate.
> - `geom_brep::Reach::{lever_from, span_reach_from, range_along}` (Real) on `Reach::Span`. `lever_from`: C/E whole carrier; Sp `|c−p| + |R| + |r|`; L/N span points. `range_along`: C/E exact arc; Sp torus support `R·|dir×k| + r`; N whole net. Callers: rules.rs, chord_join, intersect.
> - `geom_brep::control_net_reach` → N net reach from a point. Callers: reduce, face_normal, sector_face, dihedral.
> - `splitting::containment::carrier_ball` (`Decide`) → ball. L `None`; C `(center, radius)` (no abs); E (center, larger semi-axis); Sp speed-bound ball about the midpoint; N ball about the centre of the control box. Callers: `step_ball`, `arc_distance_floor`, shell.rs (`carrier_box`, `moved_walls`).
> - `containment::conic_arc_ball` → the arc's own ball (midpoint, speed×half-width, plus a rounding charge). `step_ball` keeps both balls and `extent_from` takes the nearer.
> - `shell::carrier_box` → 2-D in-plane box, the projection of `carrier_ball`. Shell footprint gate.
> - `shell::moved_walls` (inline) → cavity wall box. Conic: cube `centre ± max(ru, rv)`; Sp/N: `ArcPiece::ball` (the lesser of the speed ball and the net ball).
> - `reduce::gate_maximal_faces` (inline) → reach from the edge's start. L chord; C `2|r|·sin(min(span/2, π/2))`; N `control_net_reach`; else `ExtentBall::of_box(edge_reach_in).lever_from(at)`.
> - `sweep::blend::surgery::boxed_reach` (`Bounds`) → near/far from c, or a range along u, from box corners. E/Sp/N via geom boxes; L/C `None`.
> - `editor-core::eval::measure::curve_reach` → reach from a point, whole carrier. C `|c−o| + radius` (no abs); E larger semi-axis; Sp `+ R + r`; N net.
> - Subdivision piece balls (`spline_arc`, `spiric_arc`, `ArcPiece`) are kind-specific but are refinement, not doors.
> 
> **Literal torus balls in region-bounding code:** `extent.rs:173` (`of_torus`, no abs) and `:779` (with abs, same file); `side_reader.rs:434`; `ssi.rs:3423`; `mate/reach.rs:216`; `measure.rs:730`; `join.rs:4208` and `containment.rs:2083` (spiric levers, R+r+|d|); `solid_contain.rs:2794` (`ext`); `circle_torus.rs:373` (lever). Torus supports written differently: `boxes::torus_extent` `(R+r)·perp + r·|a_i|`; `trace::carrier_extent` `R·|a×n| + r`; `Reach::range_along` `R·|dir×k| + r`.
> 
> **What my search could not match.** My regexes caught `fn …(aabb|box|extent|reach|ball)`, `Curve3::`/`Surface::` arms and `from_points`/`point_hull`. They miss inline per-kind matches with no such name (as `moved_walls` and `gate_maximal_faces` were found only by reading), folds that use vertices as extents under generic names (as `chart_region` was), levers that carry a region bound under "lever"/"arm"/"size" names (e.g. `pcurve_cache` `azimuth_lever`, mesh sizing; I judged those not to be this question), and the narrower blend doors (`arms::corner_ball`, `battery::extent_of`), which I did not open.
> 
> ## 2. Where they disagree
> 
> - **Torus face**: `face_box` and `face_reach_in` use a sampled chart window plus a subdivision charge; the split gate uses the closed form `torus_rect_extent`, the cause its docs name for having its own torus arm. `carrier_pair::face_ball` uses the whole R+r ball; mate and `beyond` use R+r. **sure**
> - **Torus support along a direction**: `torus_extent` ≥ `trace::carrier_extent` = `range_along` for any tilted axis, because r·(√(1−a²) + |a|) ≥ r. **sure** (row `torus-extent-overreaches-…`)
> - **Sphere face**: `sphere_reach` (rect, zone or ball) in both box lanes; `held_reach_in` narrows further in the census only; `face_ball`, `beyond`, mate and ssi take the whole ball. **sure**
> - **Sphere/torus face reach from a point**: `rules::face_reach_from` levers boundary edges round their own carriers and never the surface's ball. On a sphere cap past a hemisphere, measured from a rim point, the antipode (2r) exceeds the rim's 2ρ. Mate and `beyond` add the carrier ball instead. Whether a split or join caller ever hands it such a face: **unsure**. The difference in arithmetic: **sure**.
> - **Conic arc box**: Interval-outward `conic_arc_aabb` (`edge_box`) against `conic_arc_extent_along` at T with no outward charge (`edge_reach_in`). **sure**. Whether the latter fails to enclose itself: row `census-lane-span-extents-are-not-self-enclosing`.
> - **Conic edge tightness**: the exact arc (boxes lanes, sweep, `span_reach_from`, `edge_reach_from`) against the whole carrier (`carrier_ball`, `shell::carrier_box`, `moved_walls` cube, `witness_span::conic_reach`, `measure::curve_reach`, `Reach::lever_from`). **sure**
> - **Ellipse reach from a point**: `gate_maximal_faces` takes a box-corner ball lever from `edge_reach_in`; `edge_reach_from` takes `conic_arc_reach`; `carrier_ball` takes the larger semi-axis ball. **sure**
> - **Spiric**: period box (topo boxes, geom, sweep surgery); speed-bound midpoint ball (`carrier_ball`); torus ball (`lever_from`, `curve_reach`); torus support (`range_along`); min(torus, Lipschitz) (`edge_reach_from`); `axial_carrier` says Unclaimable although `edge_rule_box` has a box; `sheet_clip` refuses it. **sure**
> - **NURBS edge**: `edge_rule_box` ControlHull (net ∪ chord) against census `curved_box` `nurbs_curve_aabb` (net only; poison on any poisoned channel). The census comment "where `edge_box` has no sound box" is stale, since `edge_box_rule` maps Nurbs→ControlHull. **sure** stale, **likely** redundant.
> - **NURBS face**: `face_box` poison box against `face_reach_in` `None` for a poisoned net (documented as intended). **sure**
> - **Cone face**: the ConeSlab axial window is read off the boundary only, so a tip face bounded by its rim alone would leave out the apex. `rules::cone_axial_window` widens for exactly that case. Valid bodies seem to carry apex struts. **unsure**
> - **Sign handling**: `of_torus` R+r against `lever_from` |R|+|r| (one file); `carrier_ball` and `curve_reach` circle `radius` against `lever_from` `radius.abs()`. At rest, tier 3 refuses non-positive radii, so this differs only on bodies that have not been through tier 3. **likely**
> - **Line edge ends**: vertex points (topo) against `carrier.eval(t0/t1)` (sweep `sheet_clip`, `curve_reach`). **sure**, small.
> - **Face boundary reach**: `measure::reach_of` is kind-blind, and some callers guard P/Cy (`at_rest.rs:1963`) while others hand any kind (`at_rest.rs:1079` `pairing_read`). **unsure**
> 
> ## 3. Why there are several
> 
> 1. **Scalar walls.** The `Bounds` scope rule (`geom-core/src/real.rs:1117`, "brackets never decide") puts a compound `Decide + Bounds` only in per-file allowlisted driver code (`scripts/gates/bounds-allowlist.sh`: boxes.rs 3, census.rs 8, reduce.rs 11, shell.rs 2, sweep reach and surgery …).
>    - Pruning lanes therefore box at f64 brackets (`face_box`, `edge_box`, `FaceBoxes`). Deciding lanes (split gate `reach_clears`, narrow phase, carrier doors, join levers) must hold the answer at T under `Decide` only, so they get `face_reach_in`, `edge_reach_in`, `carrier_ball` and `face_reach_from`.
>    - geom's arc boxes are sole-`Bounds`, so a `Decide`-only lane cannot call them; `conic_arc_extent_along` (Real) was added as a twin of `_bracketed`. geom-brep's `edge_reach_from` needs `SpanLocate` (sampling), a third wall.
>    - `boxes.rs`'s `Span<T>` arithmetic is the one place that collapsed this: one formula, two lane entries.
> 2. **Crate layering and visibility.** A face's region needs the Body (loop walk, stored pcurves for the torus window, `sphere_chart_trim`, chord_join azimuth windows), so it lives in topo. The edge pieces that need no Body live below: geom has boxes, geom-brep has reaches and balls. topo's doors are pub(crate), and the only export is the f64 world `FaceBoxes` (plus `trace::carrier_extent`). So sweep, editor-core (mate, measure) and step-import ("there is no body yet") rebuild per-kind matches from geom's public boxes or by hand.
> 3. **Answer shape: genuinely different consumers.**
>    - BVH and pruning want a world Aabb: looseness costs work, or a refusal where non-overlap is the grant (the boxes.rs module docs list thirteen doors and which way each reads).
>    - Gates want an extent along one direction in an aimed frame (`BoxFrame`).
>    - Angular levers want a reach from a pivot. `extent.rs` says a ball is not a lever ("no longer than the caller's own measure"), so a box or ball door cannot serve a two-sided lever row.
>    - The shell wants a 2-D in-plane box; the blend wants an (r, z) sheet box; the mate wants a reach from the part origin.
> 4. **Per-kind tightening lands where the need is felt.** The sphere zone (split gate, later folded into the rule), the closed-form torus (split gate, not folded), the sphere pole `Out` (census only) and the cone apex (rules) each have their data in a different module. The rule itself records a territory fence: ConeSlab "No boundary clip … jurisdiction: the cone lanes are a different unit's territory."
> 5. **Fail-loud direction per door.** Poison box (prunes nothing), `None` (refuse), `unreachable!`, or `Err(UnboundedFace)`. The same no-claim answer is spelled per consumer, which keeps census and boolean lanes apart even where their arithmetic is shared.
> 6. **At-rest vs mid-op vs pre-body.** `face_reach_in` and `carrier_pair` serve at-rest and mid-op bodies (`OPERATORS_KEEP_LINKS`); sweep surgery reads mid-op bodies through `Option`; step-import has curves and no body; editor-core reads evaluated bodies. Sign conventions (abs or not) differ along this axis.
> 7. **DESIGN gives no home.** No `[one-way.places]` row exists. `[one-way.readings]` exempts "boxes" as kind-agnostic although every door matches on kind, and its "lowest crate" placement (geom for one carrier) conflicts with the scalar wall in 1.
> 
> ## 4. Axes a single home would carry
> 
> - **Genuine** (consumers need different things):
>   - answer shape: world box, frame box, extent along a direction, reach from a point (scalar), ball;
>   - face vs edge (a face needs the Body and trim, an edge only its carrier and span);
>   - frame: world, aimed, in-plane 2-D, (r, z) sheet;
>   - pruning vs deciding direction: bracketed f64 vs T;
>   - lever constraint: an upper bound that is also no longer than the region's own measure (rules out balls);
>   - no-claim spelling, as far as each door's fail-loud direction differs.
> - **Accidental:**
>   - the tightness choice per kind (whole carrier vs arc, sampled vs closed-form torus, sphere ball vs window, period box vs speed ball), made per site with no consumer reason recorded;
>   - torus and sphere literals and their abs handling;
>   - the stale NURBS `curved_box`;
>   - the spiric being Unclaimable along an axis but boxable;
>   - vertex points vs `eval` endpoints;
>   - the topo `pub(crate)` wall that forces sweep, editor-core and step-import re-derivations;
>   - outward rounding present at one scalar and absent at the other.
> - **Mixed:** the scalar bound (`Bounds` / `Decide` / `SpanLocate` / `Real`). The wall itself is ratified policy, but `Span<T>` shows the arithmetic need not split along it; at-rest vs mid-op vs no-body, real for which records exist, accidental for sign conventions.
> 
> **Stale rows.** D280's `reach_box` no longer exists: `face_reach_in` reads the rule and the shared `Span` extents, and the remaining gaps are the cone tan spelling, the ControlNet poison/None split and conic rounding. `boxes::{conic_extent, arc_extent}` from the four-dispatchers row: `arc_extent` is gone, and `conic_extent` now serves only full circles (`circle_box`, `separating`). The witness_span vertices-only row is partly overtaken: `conic_reach` adds the whole conic carriers.

Dispatched on it: the extent-home design fork, three designers (protocol `1b9ac429b1`, blinding on `analysis/design-fork/boxes-extent-home`). The survey also reports `D280`'s premise stale (`reach_box` is gone; what is left is the cone tan spelling, the ControlNet poison/None split and the conic rounding) and the census `curved_box` NURBS comment stale; both go to the design.
