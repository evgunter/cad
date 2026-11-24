# BOXFINISH M12 — after PRs 7370 and 7480: the chain's link, the four margins, and where they lose the box (2026-11-24)

Headline: neither PR moved what M11 measured. The chain's link still refuses at plain `Interval` (the tour still ends at
`chaintol.rs:297`); the four margins are undecided as often as before and the hole finish costs the same; the two-parameter
plate still refuses, now mostly on a different predicate. The two untraced things are traced: the clearance's `[0, +inf]` and
the winding's lost angle both come from one division in `geom_brep::quartic::quadratic_roots`, and that division fails
because the wall quadratic's `b`, which is identically 0 at the nearest point, reads ±1e-4.

## 1. The chain's link (tour's exit; `BOXFINISH_TRACE=1`; all measured)
| | box arm off | box arm on (M11's switch-on run, main as of M11) |
|---|---|---|
| the link's body (18 faces, 20 vertices) finished at | `f64` (ok), `Interval` (REFUSED, 7.5 ms), `Sym<Interval>` (REFUSED) | `f64` only, three times, ok; no finish at `Interval` anywhere in the tour |
| chain, 1 link, `Interval` row | "refuses 0.07 s, 1 refused, node 111e1e667832" | "CERTIFIES 0.02 s" |
| command | panics at `src/chaintol.rs:297`, 762 s | runs to its end (109 s at M11) |
- **Which edge:** all 24 of them (`EdgeKey 1v1 .. 24v1`: 18 lines, 12 circles in the body), not one.
- **Which rung:** the edge certificate's `EndpointStart` (check 2), predicate `carrier_endpoint_start`, sample 0, on every
  one: `EdgeCertification { edge: 1v1, error: Escalated { check: EndpointStart, sample: 0, cause: Indeterminate { margin:
  Enclosure { lo: 0.0, hi: 8.9988e-5 }, band (1e-9, 1e-8), predicate: "carrier_endpoint_start" } } }`. The 24 upper ends
  take eight values: 4.80e-5, 9.00e-5, 6.72e-4, 7.20e-4, 7.27e-4, 7.68e-4 (and two within 1e-7 of these); every lower end is 0.
- **With it, in the same finish:** 24 `PlanarFaceEscalated` (`pcurve_point_chart_incident`, e.g. face 1v1 vertex 8v1,
  `[0, 1.80e-6]`) and 24 `PlanarBoundaryEscalated` (`pcurve_line_chart_incident`, e.g. face 3v1 edge 1v1, `[0, 7.25e-4]`).
- **What the edge stores:** edge 1v1 is `Line { origin: ([-4.50e-5, 4.50e-5], [-0.0015, -0.0014993], 0), dir: ([0.99955,
  1.0], [-0.029995, 0.029995], 0) }`, decoration `Com`: a link turned by a joint box of ±0.03 rad. The margin is a
  distance bound, so its lower end is 0 by construction and its upper end is the box (inferred: at plain `Interval` there
  is no tier to answer the identity "the carrier's start is the vertex", which is what decides it at `Sym<Interval>`
  elsewhere; the `Sym<Interval>` finish of the same body refuses on check 7 instead, volume `[-1.08e-7, 2.53e-7]`).
- The `f64` witness of the same body finishes ok (measured); its `carrier_endpoint_start` value was not printed.
- PR 7370 did not cure it: the payload is M11's, to the digit (`hi: 8.998834185533449e-5`).

## 2. The four margins on the tour's first drive (1,023 replays, quiet machine; M11 -> M12; all measured)
| predicate | undecided | ladder time | slow climbs (mean) | undecided reading, M12 |
|---|---|---|---|---|
| `curve_carrier_clear` | 26,948 -> 25,928 (decided 140,248) | 682.6 -> 615.1 s | 3,117 -> 2,981 (206 ms) | `[0, +inf]` `Trv` 13,392; `Def` 10,136; `Com` 2,400 |
| `side_at_infinity_winding` | 11,436 -> 11,212 | 167.2 -> 166.3 s | 2,743 -> 2,730 (61 ms) | `[-0.0072, 0.0646]` `Trv` 8,258 (M11: `[-0.0083, 0.0303]`) |
| `curve_face_pose_disc` | 1,760 -> 1,496 | 100.0 -> 83.4 s | 880 -> 748 (112 ms) | `[-0.00228, 0.00228]` `Def`, all |
| `curve_carrier_gap` | 9,710 -> 9,280 | 33.6 -> 30.2 s | 620 -> 530 | `[0, +inf]` `Trv` 4,464; `Com` 2,560; `Def` 2,256 |
| `azimuth_near_cut_side` (M11's fifth) | 5,452 (decided 7,420) | 22.2 s | 1 | `[-2.4e-7, 5.2e-6]` `Def` |
| | M11 | M12 |
|---|---|---|
| hole finish, mean | 514.5 ms | 517.1 ms |
| drive leaf time | 1,306.5 s | 1,345.7 s |
| wall to the drive's answer | 328 s | 350 s |
| certified / refused / splits | 211 / 301 / 511 | 211 / 301 / 511 |
| decisions (symbolic, numeric, frozen) | 1,832,193 / 2,541,916 / 1,490 | the same three numbers |
So the two PRs moved the winding's enclosure and a few percent of the counts, and nothing else here.

## 3. The two things M11 left untraced (`BOXFINISH_M11=1`, the tour's first leaves; measured unless said)
**a. The clearance's `[0, +inf]`.** The decision is in `topo::solid_contain::curved_edges::passes`, which reads
`carrier.clearance(seed)`; for the rim circle that is `sqrt(h² + (rho − r)²)` (`geom::curves::on_carrier`), and nothing in
it leaves its domain. Its operand does: `seed` is `(-inf, +inf)` in all three coordinates. `seed = q + d·t`, `t` a crossing
parameter of the ray on one of the edge's two faces (`curved_edges::read`, the loop over `ca` and `cb`), and the `t` that is
`(-inf, +inf)` `Trv` is always a crossing of the CYLINDER wall face (320 of 320 traced). It is made in
`geom_brep::quartic::quadratic_roots(a, b, c, disc)`: `root = sqrt(max(disc, 0))`, `minus = −b − root`, `plus = root − b`,
roots `b.select_le_zero(c / plus, minus / a)` and `b.select_le_zero(plus / a, c / minus)`. One traced case:
`a = 0.0301537`, `b = [-9.48e-5, 1.10e-4]`, `c = [-1.638e-6, -1.47e-7]`, `disc = [4.43e-9, 6.14e-8]` (decided Positive),
`sqrt(disc) = [6.66e-5, 2.48e-4]`. Because `b` straddles 0 by more than `sqrt(disc)`, both `plus = [-4.3e-5, 3.4e-4]` and
`minus = [-3.6e-4, 2.8e-5]` contain 0, the select keeps both arms, and `c / plus`, `c / minus` are `(-inf, +inf)`:
roots `[-inf, inf]` `Trv`, slack `[4.7e-18, inf]`. **The operand that leaves its domain is the divisor `√disc − b`.**
**b. Where the pose discriminant's offset takes the box more than once** (`geom_brep::curve_face::line_quadric::
wall_quadratic`, `About::Nearest`; a ray with an exact direction, `a2 = 0.0301537`, `|dp| = 0.1736`):
| quantity | enclosure | width | at f64 |
|---|---|---|---|
| `q`, the ray's start | x `[-0.00206, -0.00149]`, y `[-0.00118, 2.7e-5]` | 0.57 mm, 1.21 mm (the box is 0.1 mm) | (-0.001776, -0.000583) |
| `w0p = wall.offset(q)` | x `[-5.6e-4, 1.1e-4]`, y as `q` | 0.67 mm, 1.21 mm | (-2.26e-4, -5.83e-4) |
| `w0p·dp` | `[-7.50e-5, 1.53e-4]` | 2.28e-4 | 3.93e-5 |
| `shift = clamp(−(w0p·dp)/a2, ±span)` | `[-3.13e-3, 2.49e-3]` `Def` | 5.62 mm (7.6 mm before the clamp) | -1.304e-3 |
| `w = w0p + dp·shift` | x `[-9.6e-4, 4.3e-4]`, y `[-1.48e-3, 3.9e-4]` | 1.39 mm, 1.87 mm | (-3.93e-4, -4.30e-4) |
| `b = w·dp` | `[-1.69e-4, 2.28e-4]` | 3.97e-4 | **0.0** |
Two layers. (i) The ray's start is already 6 to 12 times the box before the wall is read. (ii) `w0p` is then counted twice:
once in `w` directly and once through `shift`, which is `−(w0p·dp)/a2`, a division by `a2 = 0.03` (so a factor
`1/|dp| = 5.76` on the width); the two cancel exactly in `b = w·dp` (0.0 at f64, the point nearest the axis), and over the
box that cancellation is lost. That `b` is what breaks the roots in (a).
**c. The winding's reading.** `solid_contain::loop_turn` sums `cross.atan2(dot)` over the face's edges about the point `p`;
528 of 2,343 traced turns hold an angle of `[-π, π]` `Trv`. One: face 3v1 (the wall), `p = ([-239.5, 264.9], [-10.1, 10.1],
0)` `Def`, so `cross = [-5681, 5692]` and `dot = [-64187, 70881]` both contain 0. The operand is the crossing point, hundreds
of metres wide in the wall's chart; that it is wide because it is `q + d·t` with (a)'s root is inferred, not traced.
So one defect feeds three of the four margins: `b` at the nearest point (measured for the clearance; inferred for the
winding and for the gap's `[0, +inf]`).

## 4. The two-parameter plate (`m10_3 the_two_parameter_drive_certifies_after_bisection_and_accounts_for_all_of_it`)
Still red, 28.8 s (M10: 27.1 s): 31 replays, every one refusing at the finish (735 ms each), certified 0, refused 16,
splits 15; "the box should certify SOMETHING". By first refusal:
- root replay: `RingContactEscalated { face 1v1, ring 3v1, Indeterminate { Enclosure { 0.0, 1.0367 }, band (1e-9, 1e-8),
  "ring_outer_locus_gap" } }` and the same on face 2v1 `{ 0.0, 0.9860 }` (as at M10);
- 21 replays: `VolumeUncomputable { face 7v1, Escalated { Indeterminate { margin: MarginDiag(Invalid), "props_chart_junction_
  closed" }, check: Inventory } }` (NEW: M10 had none of these);
- 9 replays: `VolumeUncomputable { face 7v1, Escalated { Indeterminate { Enclosure { 0.0, 32.302 }, "props_meridian_on_
  surface" }, check: Exact } }` (M10: all 30).
So PR 7370 did not make it certify; it moved 21 of the 30 volume refusals to an earlier check whose margin is not a number.

## 5. editor-core's and sweep's full sets, box arm off (7,159 run, 7,116 pass, 43 fail; M10 had 44 red in these two)
- **Red now, not at M10 (1):** `sweep an_aimed_ray_over_boxed_door_bodies::a_ray_whose_own_crossing_is_as_wide_as_the_box_
  reads_no_pair`: "the door builds a slab: NotFinished [OrientationUndecided { reading: Volume { Enclosure { -4.182, 6.182 },
  "positive_volume_exact" }, aimed: 6, set_aside: [..] }]". Not run with the switch on (inferred box-arm-only: its fixture
  asks the door for a slab over a wide box, which the door only finishes at the box scalar with the arm off).
- **Red at M10, green now (2):** `bisector_nodes::the_near_node_poses_at_the_witness_band`, `section_end_poses::a_section_
  end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds` (main's own, repaired on main).
- Every other row keeps its colour, the rectangle's memory kill (262 s) and the three `m10_3` rows among them.

## Method
- Probe `boxfinish/probe-switch-off`: merge `bf3da4cee8` = M11's probe + `origin/main` `4ddc3942e8` (PRs 7370, 7480). One
  conflict, in `geom-core/src/sym.rs` `with_session_in` (the probe's timer line and main's `mint::SessionRoot::open()`;
  both kept). Box arm off as before. Three probe commits add env-gated prints (`BOXFINISH_M11=1`): the clearance's operands
  where unbounded, the wall quadratic's offset, the winding's turn and any angle that lost its domain, the unbounded
  crossing parameter with its face, and the wall's roots.
- Build and runs as M10/M11. Item 2's times are from runs with nothing else on the machine (an earlier run beside a build
  read twice these times and was discarded; its counts agree). The slow sets ran at 4 workers.
- The arm-on column of item 1 is M11's switch-on run, on main as of M11, not re-taken on this main.

## Not measured, and why
- The switch-on side on this main (one more in-place rebuild); so item 5's new red row is placed by inference.
- The `f64` value of `carrier_endpoint_start` on the link, and which identity `Sym<Interval>` would use to decide it.
- Why the ray's start `q` is 6 to 12 times the box (the aim's step and reach; PR 7480's subject), and (c)'s chain from the
  root to the chart point.
- geom-core's, profile's and topo's sets (the brief asked for editor-core's and sweep's).
