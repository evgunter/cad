# BOXFINISH measurement lane — report (base main b363f5fa, 2026-11-14)

Headline: with the switch off today, the slab-class refusals come from TWO artefacts of interval arithmetic, neither
inherent to the question; removing both by hand makes the +-0.05 slab certify in one leaf through the rays. The tour's
stop is held by a different finding (check 1, a cylinder's u_ref length), not by the side reader. The NURBS quadrature
refusal needs no box at all. Most of the cost multiple is bisection forced by those refusals.

## Probe
Branch `boxfinish/probe-switch-off`, head 77e5f7ff (e736651e switch off + tallies, saved by the orchestrator after the
restart; f8794fa4 traces; 77e5f7ff experiments). My two commits carry no trailer.
- Edit: `crates/topo/src/props.rs`, `impl AtRestPolicy for Interval` and `for Sym<T>`: `type Leaf = AtRestBody<Self>`,
  `TAKE_FINISHES = false`, `leaf` and `take` copied from the `f64` arm (finish inside a `k_stats::Bracket`, record held on
  the body, relogged by `take` or on refusal). Faithful because the leaf is the finished body, finished at the door, and both
  lanes of a drive still log the finish at the same site (logged at the building node in one lane only, every M10 drive
  refused FlipCrossing on PR 6371). Nothing else had to change for editor-core or the tour to compile.
- It does break one file: `crates/sweep/tests/a_boxed_motion_moves_a_finished_body.rs` names `topo::Replay<Interval>` as the
  leaf (E0308, E0599); gated out in `crates/sweep/tests/all.rs` on the probe. The deletion row does not list it.
- Instrumentation (env `BOXFINISH_TRACE=1`): per-finish, per-aimed-ray, per-leaf lines and a (phase, predicate, kind)
  tally. Experiments: `BOXFINISH_AIM={dir,midaim,mid}`, `BOXFINISH_PLANE=mid` (in `side_reader.rs`).
- Build: ci.yml's env block plus `scripts/rustc-test-crates-opt0.sh` (integration binary opt 0, libraries opt 1), LTO off.
  All times are that shape on this 4-vCPU box. Logs: `~/.local/share/cad-work/boxfinish-m1/`.

## Baseline, switch on (unmodified b363f5fa): all 16 rows pass, none red
docm9 three rows 0.13-0.17 s; slab row 0.05 s; driving_writes_nothing 0.26 s; m10_8 pins 0.26/0.86 s (plain 2835/2803/0,
shipped 2891/2747/0); derived-frame rows 0.10/0.36 s; eval10 row 0.05 s; corpus_finish_count 75.5 s. Tour not re-run on.

## 1. Side reader over a box
**Slab +-0.05** (`m10_3_driver_interval::a_macroscopic_box_certifies_at_the_shipped_depth_budget`): FAILS `certified == 1`;
receipt certified=4 refused=0 splits=3, 5852/7193/126, 3.2 s with the trace on. Root payload: `Extrude(NotFinished{[OrientationUndecided{
reading: Volume{margin Enclosure{-1.617, 4.129}, band (2.2e-308, 4.5e-308), "positive_volume_exact"}, aimed 6, set_aside 6}]})`.
Per ray at the root (Sym<Interval>, depth [0.95, 1.05]), raised in `ray_crossings` unless said:
- top cap: stored normal ([-0.2,0.2],[-0.2,0.2],[0.962,1]); aim z [0.863,1.137]; `from` width 1.6; `d` width 0.8-1.5;
  set aside on `side_at_infinity_denom` of face 3, [-1.808, 5.978] vs (1e-9, 1e-8).
- bottom cap: aim, normal, `out` EXACT (width 1e-16); `beyond` [1.392,1.544]; `from` width (0.04,0.04,0.30);
  `d = (aim - from).normalize()` width (0.026,0.024,0.204) though it equals `-out`; crossing point width (0.23,0.21,1.80)
  though it equals the aim; six `side_at_infinity_edge` gaps [0, 1.6..3.0]; set aside on `side_at_infinity_winding`
  `MarginDiag(Invalid)`, raised by `turned_inside` over `loop_turn`'s atan2 sum.
- four sides: stored normal exact in direction, length [0.818, 1.222]; first decision `side_at_infinity_denom` of face 1,
  [-12.29, 7.28].
- quarter leaves (width 0.025): all rays still set aside; the VOLUME reads; 465-540 ms per leaf.
Where the width enters (measured):
1. `geom_brep::newell_plane`, called by `sweep::extrude` for the caps (`cap_plane`) and the line walls: centroid Newell over
   boxed corners reads z - z as [-0.1, 0.1]. The cap's true normal is (0,0,1) at every depth.
2. `side_reader::aimed_ray`: `beyond` (boxed reach) -> `from` -> `d` -> `p = from + d*t`; the aim/from correlation is lost.
Hand substitutions (same binary, env only), slab +-0.05:
| normals | ray | result |
|---|---|---|
| stored | as built | 4 leaves, quarters by the volume |
| stored | `d = -out` (dir) | 4 leaves, quarters by rays, 205-220 ms/leaf |
| stored | constant (mid) | 2 leaves, halves by rays, 55-61 ms/leaf; root: denom of face 1 [-1.357, 0.437], winding Invalid |
| midpoint | as built | 4 leaves, quarters by the volume |
| midpoint | constant | ROOT reads by rays: certified=1, 836/969, test PASSES, 0.15 s |
So a ray exact in the parameters removes one bisection level and no more; the stored normals' width is the other half.
What it does not establish: that a midpoint normal or aim is sound in general (here the true normal is parameter-free).
**Derived-frame rows**: both FAIL at 5e-2 only (eps/8, 1e-6, 1e-3 pass). The refusing node is the CUBE (node 17, the box
whose height is widened), not the boss and not the placement: same payload shape (volume [-5.163, 15.214]; cap normal
+-0.1; `side_at_infinity_denom` [-10.71, 4.49]; winding Invalid). The boss finishes ok every time. With both
substitutions the twin row PASSES; the freeze row certifies and then trips `frozen == 0` (1060), a probe artefact.
**Tour** (`demo-tour certified`, ci-tour profile): 98 s, exit 101; 0 certified, 512 refused, all "budget"; certified
0.0000%, refused 99.4607%, tail 0.5393%. Traced: 1023 replays, 3 finishes each. The plate FINISHES in every replay and its
side reads by rays (Side x 1032, never Unread). The refusal is check 1 on the two hole extrudes (2046 finishes):
`SurfaceDatumUndecided{kind: Cylinder, datum: URef, measure: Length, end: Upper, margin Enclosure{-2.971e-4, 2.412e-4},
band (1e-9, 1e-8), predicate None}`, raised by `topo::validate::surface_datum_errors` over
`Surface::representability_margins`. Minted by `sweep::extrude`'s arc arm, `u_ref: rim.normalize()` (inferred: x/|x| over
a box is not recognisably unit; the width scales with the box, so 512 leaves never reach 1e-9).

## 2. Quadrature on Interval NURBS walls
- eval10 row: FAILS, `Loft(NotFinished{[VolumeUncomputable{face 3v1, QuadratureUnsupported{"... minted exact 0/1 ..."}}]})`,
  raised by the `exact` closure in `topo::props::quad_lane::nurbs_face`.
- sweep, through the door (two probe tests added to `a_loft_computes_its_walls_in_t.rs`): the boxed loft refuses the same
  way, and so does the SAME loft with every section a point (no box at all).
- Ends: the stored image is `IsoLine{p0, pl}` over the edge's own parameter, read as `image.eval(t1) = p0 + pl*t1`.
  eval10: `pl.x = [1.999999999984, 2.000000000016]`, `t1 = 0.5 +- 3.9e-12` -> end [1 +- 1.56e-11] (box-wide);
  another rim `pl.x = [0.9999999999999997, 1]` -> end width 5.6e-16. Point loft: `pl.x = 1/2.2`, end width 2.2e-15.
  Ten of the thirteen printed half-edges read exact 0/1.
- Minted by `topo::pcurves::side_image_of` (`slope = (b - a)/span; p0 = a - slope*t0`) and
  `sweep::loft::describe_section_rim` (`slope = 1/(t1 - t0)`). The end is an exact chart 0/1 that went through
  `(1/span)*t1` with outward rounding; it is not a boxed quantity (measured: it refuses with no box).
- Why check 7 reaches the quadrature: `[bf-side] aimed=0 set_aside=[Limit(KindUnsupported{face 3v1, kind: Nurbs})]`:
  `cast_aims`' `uncrossable` loop casts no ray at a selection holding a NURBS face, so a loft always goes to the volume.

## 3. Cost
| row | on | off, as built | off, both substitutions (binary's own time) |
|---|---|---|---|
| m10_8 plain decisions | 2835/2803/0 | 10851/12675/42 (3.83x/4.52x) | 4227/4703 (1.49x/1.68x) |
| docm9 a_certified_side_stops_at_the_seeds_edge | 0.137 s | 9.96 s PASS | 0.35 s |
| docm9 a_parameter_field_boxes_directly | 0.132 s | 9.92 s PASS | 0.36 s |
| docm9 the_query_does_not_touch_the_input_document | 0.171 s | 10.22 s PASS | 0.39 s |
| m10_3 driving_writes_nothing | 0.26 s | 35.5 s PASS | 0.69 s |
Attribution, m10_8 plain (slab(1.0, 0.25), 8 leaves), all measured from the tally:
- on: 7 replays = 3 refusing in the build (305/265 each) + 4 certifying quarters (480/502).
- off: 15 replays = those 3 + 12 that refuse AT THE FINISH (828/990 each); receipt certified=0 refused=8 (leaf budget).
- one finish per finishing leaf, none repeated: 348 symbolic + 488 numeric each.
- of the extra decisions, the 8 extra replays are 82% (sym) / 80% (num); the finish on the 4 quarters is 18% / 20%.
- per-finish rows: dihedral_arm 168, dihedral_wedge 168, dihedral_sign 84, carrier_on_surface_1 108, _2 108,
  ssi_hull_sup 24, witness_* 3x12, carrier_endpoint_* 2x12, interval_span_forward 12: the door's own build decides each
  of these the same number of times on the same body in the same leaf (about 732 of 836; equal names and counts measured,
  equal margins inferred). New in the finish: pcurve_*_chart_incident 48, bool_ring_run_winding 6, session_box_reach 1,
  side_at_infinity_* about 49.
docm9 (first row traced): 31 replays against main's 7 (m10_8's own doc comment: root, two halves, four quarters); 28
finishes, none repeated; every ray set aside at all 28; the volume refuses at 12 (254 ms avg) and reads at 16 (354 ms avg);
finishes are 8.7 s of 10.4 s. driving_writes_nothing: 127 replays, 61 finishes; 37 refusing ones average 787 ms (29.1 s
of 36.7 s). A finish whose rays read costs 50-110 ms on a 6-face slab at Sym<Interval> (2 ms at f64); the rest of that
leaf is 20-40 ms.
Repeats: none in a one-parameter slab drive. With a move, 3 finishes per leaf (door, the moved leaf's re-finish through
`LeafBody for AtRestBody::moved_by`, the union's result). In the tour, 3069 box-lane finishes cover 255 distinct vertex
fingerprints: the plate is ONE identical body finished 1023 times, the two holes 2046 times over 254; 92% of the tour's
finishes repeat a body already finished in the same drive, and finishes are 38% of its leaf time (surfaces not hashed).

## Corrections to the rows' premises
1. Side-reader row: the slab does not spend the 32-leaf budget and certify 0; it certifies in 4 leaves, by the volume.
   The quoted margins ([-0.91, 2.99], [-5.22, 3.13]) are not today's.
2. Side-reader row: "aim at exact points" alone is measured insufficient; the stored plane normals are the other cause,
   and they are minted outside the program's paths.
3. Side-reader row: the tour stop is not a side-reader failure; it is check 1's `SurfaceDatumUndecided`. No row holds it.
4. Side-reader row: the derived-frame rows refuse at the widened cube, which is the slab case again.
5. Cost row: docm9 is 10 s, not 157-299 s, and passes its main-era assertions; main's pins are 2835/2803, not 2898/1612.
   The multiple is mostly forced bisection plus the volume fallback, not tier 3 per leaf.
6. Quadrature row: the refusal is not specific to a boxed loft; a point-interval loft refuses on rounding alone.
7. Deletion row: one more file names the Replay leaf (the sweep test above).

## Off-fence, noticed, not filed
- `geom_brep::newell_plane` / `sweep::extrude`: plane normals minted over a box carry tilt and non-unit length.
- `sweep::extrude` arc arm `u_ref: rim.normalize()` against check 1 over a box.
- `topo::pcurves::side_image_of`, `sweep::loft::describe_section_rim`: the chart end goes through `(1/span)*t1`.
- Check 7's volume fallback encloses a volume of [0.95, 1.05] as [-1.62, 4.13] at +-0.05, and reads only at width <= 0.03.
- f64 witness of `a_slab_placed_then_unioned_certifies_in_one_leaf`: the slab is finished twice with one fingerprint
  before the move (4 finishes against the box lane's 3). Cause not traced.
- A stray 5-byte file `/build-on.pid` (my launcher mistake); the harness refused its removal. Please delete it.

## Not measured
- The tour with the switch on today (the 114 s figure was taken as given), and the tour under the substitutions (its
  refusal is check 1, which they do not touch).
- Whether the finish's repeated rows have bit-equal margins to the build's; why the winding margin is Invalid (inferred:
  an atan2 box containing the origin); where the volume fallback's width enters.
- No sampling profile (the per-finish timers answered the question). pncad-py was not built with the switch off.
- Container restart at about 02:55 UTC: nothing of this lane's was running; no run looked OOM-killed.

## 2c. Sweep lofts through the door (sweep-probe_*.log)
a_loft_computes_its_walls_in_t::probe_a_boxed_interval_loft_through_the_door (box 1.2 +- 3.9e-12 on the middle section)
and ::probe_a_point_interval_loft_through_the_door (no box) both refuse:
NotFinished{[VolumeUncomputable{face 4v1, QuadratureUnsupported{"a NURBS-face pcurve endpoint is not exact structure ..."}}]}.
Both: [bf-side] aimed=0 set_aside=[Limit(KindUnsupported{face 3v1, kind: Nurbs})]. Same vertex fingerprint 06869d0021d15c04.
Inexact ends in both: he 20v1 pl.x=[0.49751859510499435, 0.4975185951049948], t1=2.009975124 (w 1.3e-15) -> b.x w 2.0e-15;
he 22v1 pl.x=[0.45454545454545425, 0.45454545454545475], t1=2.2 (w 1.8e-15) -> b.x w 2.2e-15.
The sweep test binary needed crates/sweep/tests/a_boxed_motion_moves_a_finished_body.rs gated out (it names
topo::Replay<Interval> as the leaf: E0308 at cylinder_leaf, E0599 at .moved).

## 1f. Tour repeats (tour-off-trace.err)
Box-lane finishes: 6-face plate 1023 finishes, 1 distinct vertex fingerprint (avg 46.3 ms, all ok); 4-face holes 2046
finishes, 254 distinct (avg 49.6 ms, all REFUSED at check 1). Leaf time avg 381 ms over 1023 replays. No [bf-move].

# Appendix: running notes, in the order found
# BOXFINISH measurement lane — running notes (appended as found)

Base: main b363f5fa. Build: CI's env block + scripts/rustc-test-crates-opt0.sh wrapper
(integration binary at opt 0, libraries at opt 1), LTO off, target /home/user/boxfinish-m1-target.
Wrapper for every cargo call: ~/.local/share/cad-work/boxfinish-m1/c.sh

## Baseline (switch ON, unmodified main b363f5fa), editor-core `all` binary

c.sh nextest run -p editor-core --test all --no-fail-fast -j 2 -E '<rows>' : 16 run, 16 passed

- docm9_range::a_certified_side_stops_at_the_seeds_edge 0.137 s
- docm9_range::a_parameter_field_boxes_directly 0.132 s
- docm9_range::the_query_does_not_touch_the_input_document 0.171 s
- eval10_section_reads_the_nominal::a_box_over_a_section_width_lofts_the_family_it_spans 0.051 s
- m10_3_driver_interval::a_macroscopic_box_certifies_at_the_shipped_depth_budget 0.050 s
- m10_3_driver_interval::driving_writes_nothing 0.260 s
- m10_8_pins_interval::m10_8_the_a0_set_is_a0_alone 0.862 s
- m10_8_pins_interval::m10_8_the_shipped_set_is_inert_on_straight_geometry 0.260 s
  (main's pins today: plain 2835/2803/0, shipped 2891/2747/0; the cost row quotes 2898/1612/0)
- m10_derived_frame_interval::m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width 0.355 s
- m10_derived_frame_interval::m10_the_derived_frames_refusal_is_not_a_freeze 0.098 s
- onehome_switch_parity_interval (5 rows) 0.04-0.88 s
- corpus_finish_count::the_corpus_runs_its_pinned_count_of_finishes 75.5 s

None is red on today's main.

## Events
- ~02:55 UTC container restart (cause unknown). Nothing of this lane's was running detached then
  (baseline build and rows had completed). Orchestrator saved the WIP as e736651e.
- A stray 5-byte file /build-on.pid was written by mistake (launcher line with an unset variable);
  the harness refused its removal. Harmless; please delete.

## Probe
Branch boxfinish/probe-switch-off: e736651e (switch off + first instrumentation), f8794fa4 (ray/leaf/pcurve traces).
Switch-off edit (crates/topo/src/props.rs, `impl AtRestPolicy for Interval` and `for Sym<T>`): `type Leaf = AtRestBody<Self>`,
`TAKE_FINISHES = false`, `leaf` = the f64 arm's (finish inside a k_stats Bracket, record held on the body, relogged on refusal),
`take` = relog_held + Arc::clone. Why this shape: logging the finish at the building node in one lane only made every M10 drive
refuse FlipCrossing on PR 6371 (its body, "Ev's ruling" section); the f64-arm shape keeps both lanes logging at the taker.
It compiled with no other change. Env BOXFINISH_TRACE=1 turns the traces on; BOXFINISH_AIM={dir,midaim,mid} are aim experiments.

## 1a. Slab +-0.05 (m10_3_driver_interval::a_macroscopic_box_certifies_at_the_shipped_depth_budget), switch off
Logs: slab-off.log, slab-off-{dir,midaim,mid}.log.
FAILS its assertion (certified == 1): receipt certified=4 refused=0 splits=3 (root BISECT, two halves BISECT, four quarters
CERTIFIED). NOT "spends the 32-leaf budget, certifies 0" as the row says. decisions sym=5852 num=7193 frozen=126; 2.0 s.
Root leaf refusal payload: Extrude(NotFinished{[OrientationUndecided{ reading: Volume{ margin: Enclosure{lo:-1.6167, hi:4.1293},
band{zero:2.2e-308, escalate:4.5e-308}, predicate "positive_volume_exact"}, aimed: 6, set_aside: 6 }]}).
Per ray at the root (Sym<Interval>, depth in [0.95,1.05]):
- face 1 (top cap). aim z=[0.863,1.137] (w 0.273), stored plane normal ([-0.2,0.2],[-0.2,0.2],[0.962,1]) (w 0.4 tilt),
  out w 0.42, beyond [1.30,1.82], from w 1.6-1.7, d w 0.8-1.5, plane-1 t=[0.67,22.0], p w 18-45.
  Set aside: side_at_infinity_denom on face 3, enclosure [-1.808, 5.978] vs band (1e-9,1e-8) (ray_crossings' plane loop).
- face 2 (bottom cap). aim exact (w 1e-16), normal exact (0,0,-1), out exact; beyond=[1.392,1.544] (w 0.15);
  from w (0.039,0.036,0.298); d=(aim-from).normalize() w (0.026,0.024,0.204) though d == -out exactly;
  plane-2 crossing p w (0.23,0.21,1.80) though p == aim exactly. 6 edge gaps' lower ends reach 0 (side_at_infinity_edge
  enclosures [0, 1.6..3.0]), edge_span [-45,47]; set aside on side_at_infinity_winding MarginDiag(Invalid) (NaN) raised by
  turned_inside(crosses_by_turn/loop_turn's atan2) on face 1.
- faces 3-6 (sides). stored plane normal axis-exact in direction, length [0.818,1.222] (w 0.404); the FIRST decision,
  side_at_infinity_denom on face 1, is [-12.29, 7.28] in band.
- volume fallback: positive_volume_exact [-1.617, 4.129] (true volume in [0.95,1.05]).
Quarter leaves (w 0.025): all 6 rays still set aside; the VOLUME reads Out; 465-540 ms per leaf.
Where the width enters (measured): the stored plane normals minted by the extrude door through geom_brep::newell_plane
(translate-to-centroid Newell over boxed corner points: the cap's z-z differences are [-0.1,0.1], not 0), and the ray's own
build in side_reader::aimed_ray (beyond -> from -> d=(aim-from).normalize() -> p=from+d*t loses the aim/from correlation).
Aim experiments (same binary, env only):
- BOXFINISH_AIM=dir (d read off -out): root and halves still BISECT; quarters read by RAYS (Side(Out)), 205-220 ms/leaf.
  certified=4 splits=3.
- BOXFINISH_AIM=midaim (aim collapsed to its midpoint, normal as stored): same tree; quarters read by the volume; 5.4-6.1 s/leaf.
- BOXFINISH_AIM=mid (aim, normal, beyond collapsed: a constant ray): root still BISECT; the two HALVES read by rays
  (55-61 ms/leaf); certified=2 splits=1; drive 0.40 s. At the root with a constant ray: rays 3-6 set aside on
  side_at_infinity_denom of face 1 = [-1.357, 0.437] (the top cap's stored normal, +-0.2 tilt); rays 1,2 on
  side_at_infinity_winding Invalid.

## 1b. Derived-frame rows, switch off (derived-off.log)
Both FAIL. m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width: half = eps/8, 1e-6, 1e-3 pass
(every finish ok, rays read Side(Out)); half=5e-2 fails "the derived-frame boss must certify on the symbolic lane".
m10_the_derived_frames_refusal_is_not_a_freeze fails at "plain form alone certifies".
The refusing node is node 17, the CUBE (the 2x2 box whose height is the widened parameter), not the boss and not the
derived placement: Extrude(NotFinished{[OrientationUndecided{reading: Volume{positive_volume_exact [-5.163, 15.214]},
aimed 6, set_aside 6}]}). Same mechanism as the slab: top cap stored normal ([-0.1,0.1],[-0.1,0.1],[0.990,1]), side normals
of length [0.818,1.222]; rays 3-6 set aside on side_at_infinity_denom [-10.71, 4.49]; rays 1,2 on side_at_infinity_winding
Invalid. The boss (exact body, fp 677cd0533d89c6c8) finishes ok every time (rays Side(Out), 48-84 ms).

## 2. eval10 boxed loft, switch off (eval10-off.log)
FAILS: Loft(NotFinished{[VolumeUncomputable{face 3v1, QuadratureUnsupported{"a NURBS-face pcurve endpoint is not exact
structure ..."}}]}), raised by the exact closure in topo::props::quad_lane::nurbs_face.
Why the volume is reached: [bf-side] scalar=Interval faces=6 rays=Unread aimed=0 set_aside=1 -> no ray is cast at all
(cast_aims' first loop: a selection holding a spline face casts none), so check 7 goes straight to the quadrature.
Pcurve ends on the walls (image.eval(t1), image = the stored IsoLine p0 + pl*t over the edge's own params):
- he 1v1, 5v1: IsoLine{p0:(0,0), pl:([1.999999999984374, 2.000000000015626], 0)}, t1=[0.5 +- 3.9e-12] -> b.x = [1 +- 1.56e-11]
  (w 3.1e-11): box-wide. slope = (b-a)/span with span = the rim's own parameter span (the boxed section width).
- he 3v1: pl.x=[0.9999999999999997, 1.0], t1 w 2.2e-16 -> b.x w 5.6e-16: ROUNDING-wide, no box needed.
- the other ten half-edges read exact 0/1.
Minted by topo::pcurves::side_image_of (slope = (b - a) / span; p0 = a - slope*t0) and sweep::loft::describe_section_rim
(slope = 1/(t1-t0)): the end value is an exact chart 0/1 that went through (1/span)*t1, not a boxed quantity.

## 3. Cost, switch off (cost-a-off.log, m10_8-off.log)
- m10_8_the_shipped_set_is_inert_on_straight_geometry: plain 10851/12675/42 vs main 2835/2803/0 (3.83x sym, 4.52x num);
  shipped 10971/12555/42. 2.9 s vs 0.26 s.
- m10_8_the_a0_set_is_a0_alone 6.77 s (0.86 s on); m10_3 driving_writes_nothing 35.5 s (0.26 s on); both PASS.
Attribution of m10_8 plain (slab(1.0,0.25), max_leaves 8):
  main: 7 replays = 3 early-refusing (root, halves: 305 sym/265 num each, no finish reached) + 4 certified quarters (480/502).
  off: 15 replays = the same 3 + 4 quarters + 8 eighths, every one of the 12 refusing AT THE FINISH (side reader + volume),
  828/990 each; receipt certified=0 refused=8 splits=7 (leaf budget). One finish per finishing leaf, none repeated.
  Per finish: 348 symbolic + 488 numeric. Extra decisions: 8 extra replays = 6624 sym/7920 num (82%/80% of the extra);
  the finish on the 4 quarters = 1416/1928 (18%/20%).
  Per-finish rows (count; "=" where the build phase of the same leaf decides the same row the same number of times):
  dihedral_arm 168 (=168), dihedral_wedge 168 (=168), dihedral_sign 84 (=84), carrier_on_surface_1 108 (=108),
  carrier_on_surface_2 108 (=108), ssi_hull_sup 24 (=24), witness_at_mid_parameter/on_surface_1/on_surface_2 12 each (=12),
  carrier_endpoint_start/end 12 each, interval_span_forward 12; NEW in the finish: pcurve_point_chart_incident 24,
  pcurve_line_chart_incident 24, bool_ring_run_winding 6, session_box_reach 1, side_at_infinity_* ~49.
  So ~732 of a finish's 836 decisions are rows the door's own build decided on the same body in the same leaf.

## 3b. docm9 rows and driving_writes_nothing, switch off, as built
- docm9_range::{a_certified_side_stops_at_the_seeds_edge, a_parameter_field_boxes_directly,
  the_query_does_not_touch_the_input_document}: PASS, 9.96 s / 9.92 s / 10.22 s (0.137 / 0.132 / 0.171 s on main).
  The row's 157-299 s is not today's number.
  Trace of the first (docm9-trace-off.log): 31 replays (15 BISECT, 16 CERTIFIED at width 0.03125), receipt certified=16
  splits=15, decisions 24347/29104/126. 28 leaves reach the finish, one finish each, no fingerprint repeated; all 28 have
  every ray set aside; the volume fallback refuses at 12 (widths 0.125, 0.0625: avg 254 ms/finish) and reads at 16 (width
  0.03125: avg 354 ms/finish). Finishes are 8.72 s of the 10.35 s of leaf time. Main: 7 replays (inferred from m10_8's
  pin arithmetic: root, two halves, four certifying quarters), no finish at Sym<Interval>.
- m10_3 driving_writes_nothing (slab(20 eps, 40 eps), 64 leaves): PASS 35.5 s. Trace: 127 replays, 61 finishes, none
  repeated; 37 refusing finishes average 787 ms (29.1 s of 36.7 s), 24 passing average 103 ms; side reads: 25 Side,
  18 InBand pair, 19 Unread with the volume refusing.

## 1c. The two artefacts removed by hand (same binary, env only): BOXFINISH_PLANE=mid BOXFINISH_AIM=mid
PLANE=mid collapses each planar face's stored normal, as the side reader reads it, to its enclosure's midpoint (unit);
AIM=mid makes the ray a constant.
- slab +-0.05: plane=mid alone -> root, halves BISECT; quarters CERTIFIED by the volume (4 leaves).
  plane=mid + aim=dir -> root, halves BISECT; quarters by RAYS (88-92 ms/leaf).
  plane=mid + aim=mid -> the ROOT reads by rays: certified=1 splits=0, 836 sym/969 num, test PASSES, 0.15 s.
- m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width: PASSES (1.03 s).
- m10_the_derived_frames_refusal_is_not_a_freeze: certifies (passes line 568), then fails `frozen == 0` (1060): the probe's
  constants freeze forms; an artefact of the hand substitution.
- m10_8 plain: 4227/4703/1985 = 3 x (305,265) + 4 x (828,977): main's 7 replays, 1.49x/1.68x main's decisions.
- docm9 three rows: PASS in 0.35 / 0.36 / 0.39 s. driving_writes_nothing: PASS 0.69 s.

## 2b. eval10 no-aim reason (eval10-off-2.log)
[bf-side] Interval faces=6 rays=Unread aimed=0 set_aside=1 [SetAside{aimed: FaceKey(3v1), aim: None,
why: Limit(KindUnsupported{face: FaceKey(3v1), kind: Nurbs})}] -> cast_aims' `uncrossable` loop: a selection holding a
NURBS face casts no ray, so check 7 always reaches the quadrature on a loft.

## 3c. Finishes per leaf with a taker / a move (onehome_switch_parity_interval, switch off; all 5 rows PASS)
- a_slab_under_a_union: 1 finish per leaf at Sym<Interval> (the door's; the union of one member runs none), 72-110 ms;
  leaf 836 sym / 969 num, 102-152 ms. f64 witness: 1 finish, 1.6-2.1 ms.
- a_slab_placed_then_unioned: 3 finishes per leaf at Sym<Interval>: the door's (75-82 ms), the moved leaf's re-finish
  ([bf-move], LeafBody for AtRestBody::moved_by -> T::leaf; 83-92 ms), the union result's (12 faces, 50-60 ms);
  leaf 2122/3033, 377-410 ms. No fingerprint repeats at Sym<Interval>.
  f64 witness: 4 finishes; the slab (fp 3e3dba330ae97de7) is finished TWICE before the move. Cause not traced.

## 1d. Tour certified stop, switch off (tour-off.out/.err): `demo-tour certified`, ci-tour profile
98 s, exit 101 (panic at tolerance.rs "the real study certified nothing"). Two-hole plate, 3 axes, 512 leaves:
0 certified, 512 refused (511 splits), every refusal "budget"; certified 0.0000%, refused (budget) 99.4607%, tail 0.5393%.
3841365 decisions, 47.4% symbolic, 1909 forms frozen.

## 1e. Tour certified stop, switch off, traced (tour-off-trace.err; 99 s)
1023 replays, every one BISECT, 3 finishes per replay. The plate (6 faces) FINISHES at every replay (1023 ok, avg 46 ms)
and its side reads by rays: [bf-side] Side x 1032, never Unread. The refusal is NOT check 7:
both hole extrudes (4-face bodies, nodes 29 and 37) refuse at tier 3's CHECK 1 at every replay (2046 refusing finishes):
  Extrude(NotFinished{[SurfaceDatumUndecided{face 3v1, kind: Cylinder, datum: URef, measure: Length, end: Upper,
  verdict: Indeterminate{margin: Enclosure{lo: -2.971e-4, hi: 2.412e-4}, band (1e-9, 1e-8), predicate: None}}, same on 4v1]})
at the root box (spacing +-5e-5, radii +-3e-5 m). Raised by topo::validate::surface_datum_errors over
geom Surface::representability_margins (geom::convention::frame_margins). The datum is minted in sweep::extrude's arc arm:
`u_ref: rim.normalize()` with rim = a lamina vertex minus the extruded centre (inferred: x/|x| over a box is not
recognisably unit length; the margin's width scales with the box, so bisection to 512 leaves never brings it under 1e-9).
The row's attribution of the tour stop to the side reader is contradicted by this payload.
