PHASE-1 REPORT — oneround/conic-vertex (measure, then stop). Nothing built, nothing pushed.

HEADLINE
- (a) Evaluation alone, no stored form, recovers every near-gap pose: 7/7/7/7/1 → 40/40/40/40/40 of 40, every root true.
- (b) Evaluation alone recovers NOTHING for the section tables' far-centre conics: their loss is in the minted stored form (1–3.6 u·a off their own surfaces in every evaluation mode).
- (c) Of 13 probed sites, 11 misread on main past the box; only 3 are fixed by evaluating the point from the vertex. The rest read `frame.origin`/semi-axes directly, or need the vertex EXACT, or need neither centre nor vertex.
- One site misreads at the default ε: `split_section_area`'s conic excess, wrong from far 1e4 at ε 1e-9. Every other wrong verdict is at ε 1e-12 from far ≈ 1e5; at 1e-9 they are in-gap only, from far 1e6–1e7.

METHOD
Local patch on merge base b80352cd (`measure-full.patch`), env-switched, one build:
- mode 0: main's centre chain.
- mode 1: `P = (c + u·a) + u·a·(cos θ − 1) + v·b·sin θ`, `cos θ − 1 = −sin²/(1 + cos)` for cos > 0; vertex summed in f64 and charged.
- mode 3: the same with the vertex summed exactly (`two_product` + `two_sum`), charged u·|V|.
`point_at`, `jet_at`'s point and `running_point` all switch together, so the walk and the meter read one chain. Local runs, 4 cores; evidence only.

(a) NEAR-GAP FAMILIES (`the_near_gap_families_keep_their_certified_answers`, ρ = 1500, u·ρ = 1.67e-13)
| family | main | mode 1 | mode 3 |
|---|---|---|---|
| circle | 7 | 40 | 40 |
| ellipse 1.25 / 0.999 / 1.001 | 7 / 7 / 7 | 40 / 40 / 40 | 40 / 40 / 40 |
| ellipse 0.8 | 1 | 40 | 40 |
| meter's error bound (median) | 6.4e-13 (3.8 u·ρ) | 1.28e-13 (0.77 u·ρ) | 6.7e-16 |
| worst root arc / ε | 2.18–2.73 | 0.44–0.55 | 0.002–0.003 |
- Every certified root passed the test's own check against the exact oracle (within ε of a true root).
- Mode 1 certifies with about 2× headroom; mode 3 with about 400×.
- The test hard-codes ε = 1e-9, so there is no 1e-6 or 1e-12 row for these floors. The whole `conic_quadric` module (59 tests, slow set included) stays green in mode 1 at CAD_TOLERANCE_EPS 1e-9, 1e-6 and 1e-12.
- In mode 3 two refusal pins go red at all three ε: `a_root_whose_slack_reads_in_the_gap_is_refused` and `cone_rows::a_root_the_slack_meter_cannot_place_refuses`. I checked the first: both roots it now certifies lie 7e-13 and 2e-13 m from the truth, so the answer is true and the pin's pose no longer refuses. The second (cone) I did not check against an oracle.

(b) SECTION TABLES' FAR-CENTRE CONICS
Probe: random planes against a tilted cylinder (plane 1e-3…1e-6 off the axis) and a cone (1e-3…1e-7 short of parabolic), through `plane_cylinder_section` / `plane_cone_section`; carriers with centre past 1e3 and an in-box part; seamed at the nearest principal vertex; 17 points over ±80 m of arc; exact-rational miss off both surfaces (the `carrier_resolution` test's oracle). Worst miss in units of u·a:
| kind, a | samples | mode 0 | mode 1 | mode 3 |
|---|---|---|---|---|
| cone 1e3 / 1e4 / 1e5 / 1e6 | 680 each | 3.39 / 2.51 / 2.53 / 2.31 | 3.61 / 2.35 / 2.22 / 1.88 | 3.18 / 2.22 / 2.15 / 1.79 |
| cylinder 1e4 / 1e5 / 1e6 | 527 / 680 / 680 | 0.94 / 1.04 / 1.12 | 1.16 / 0.86 / 1.16 | 1.16 / 0.85 / 1.16 |
- Absolute: 7e-10 m at a ≈ 1e6 (cone), 2e-10 (cylinder), in every mode.
- So the stored curve itself, read exactly, is that far from the surfaces it was cut from. The mint has already lost it; no evaluation gets it back.
- The cylinder's in-box part sits 0.42–0.50 rad from the nearest principal vertex, so no vertex is near that arc at all. The cone's sits within 1e-3–1e-2 rad of its major vertex.
- What is paid for this today is `carrier_resolution` (64·u·speed_hi, a fitted constant): `chord_join` refuses a section end as a kernel limitation past x ≈ 2e4 at ε 1e-9.

(c) THE UNCHARGED MARGINS, PROBED
Pose family: conic of semi-axis `far` ∈ 1e4…1e7 seamed at an in-box vertex, centre `far` out, in-box operand swept about the decision boundary; Band(ε, 10ε); truth by exact sums or rationals. "wrong" = definite and false, or definite with the exact margin inside ε.
| site (current home) | main: first wrong | point from vertex, plain sum | exact vertex |
|---|---|---|---|
| `line_clears_conic` (section_loops.rs) | ε 1e-12, far 1e5 (55–80 of 1500) | not reached; hand rewrite still wrong from 1e6 | 0 wrong |
| `conics_clear` (same) — its 8·EPS pad multiplies σ only, not a bound | ε 1e-12, far 1e5 (8–54) | not reached; rewrite wrong from 1e6 | 0 wrong |
| `split_section_area` excess (`Sweep::certify_section_area`, join.rs) | ε 1e-9 AND 1e-12, far 1e4 (48–1500 of 1500) | 0 wrong to 1e5 at 1e-9, wrong from 1e6; unchanged at 1e-12 | not built; re-posed as `(dt − sin dt)` it reads 0 wrong with no centre or vertex |
| `row_interval` (pcurves/mint.rs) | accepts with true gap in (ε, 10ε) from far 1e4 at 1e-12; never past 10ε | no better; one accept past 10ε | not built |
| depth readings via `Cover::arc` (successor of `conic_ring`, `edge_reach_of`, `axial_carrier`) | ε 1e-12, far 1e5 (20–420 of 2800) | unchanged (reads no point) | not built (a rewrite of the arc piece) |
| depth about a far conic's own axis (`depth_gap::face_read`) | ε 1e-12, far 1e5 (37–170 of 2800) | unchanged; no conic point involved | n/a |
| `pm_census_arc_piece`, `bool_factored_crossings_apart` (chords of one carrier) | ε 1e-12, far 1e5 (26–41 of 4000) | 0 wrong, error 4e-15 | same (vertex cancels) |
| `pose::harmonic_roots` ROOT_GAP (a chord) | ε 1e-12, far 1e5; 1222–1300 of 2000 at 1e6–1e7 | 0 wrong, error 6e-17 | same |
| `Polish::closest` `curve_pair_pose_clear` (successor of `Window::read`) | ε 1e-12, far 1e5 (462 of 4000) | unchanged; rewrite about a plain vertex is WORSE than main past 1e6 | 0 wrong |
| `off_the_apex` CONE_APEX | ε 1e-12, far 1e6: 112 of 4000 roots within ε of the apex passed | identical to main | 0 wrong |
| `certify` endpoint residuals | an end minted as `eval(t0)` pins while the true curve is 5e-11 / 5e-10 away at far 1e6 / 1e7 (4000 of 4000 past 10ε at 1e-12) | identical | not built |
| `carrier_harmonic` plane arm (pcurve_cache.rs) | envelope reads 0 against a true gap up to 1e-9, but check 3 refuses first; 0 wrong certifications | unchanged | not built |
| cylinders' listing filter (successor of `conic_ball`) | under-reads only (fail-closed) in the one axis-aligned pose tried | unchanged | — |
UNPROBED, with reason:
- `depth_gap::face_nearest` / `apart_in_depth` end to end, and the root and cut parameters handed to the two chord sites: each needs a far-centre-rimmed Body, outside the time box.
- `certify` at t0 ≠ 0 and its other residuals: no exact truth built for a general parameter.
- The circle arm of `curve_pair_pose_clear` at far ≥ 1e6: the read collapsed to 0 at the targets tried.
- Sites 1–4 of the splitting group were probed at expression level, not through a Body.

ROW PREMISES CORRECTED
- "which this row's vertex evaluation removes at the source": false for every site that reads the frame directly (most of them).
- "Bounds only … which rounding loosens": false. `Cover::arc`'s greatest depth undershoots and its least overshoots the true arc by about u·far, and `depth_gap` decides on them. Same shape in `arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size`: at a = 1e7 the cover misses the true arc by 4e-10, green on main only because `eval` shares the rounding.
- "fail-closed only" for `row_interval` and `certify`'s endpoints: false, both can accept.
- `carrier_harmonic` "mostly widens": in the probe it shrinks to zero; the guard is the sampled schedule.
- "Floor ~1–2 u·ρ" for the vertex term: measured 0.77 u·ρ plain, 0.004 u·ρ exact.
- Retired or renamed: `conic_ring`, `edge_reach_of` (commit f67931961), `axial_carrier` (f097a63b8), `conic_ball` (c35ec02aa), `Window::read`/`ring` (no trace on `curve_pair`); `census::depth_gap` is now a module and no longer reads `speed_hi − speed_lo`.
- Main's counts (7/7/7/7/1) confirmed.

WHICH VERTEX, AND WHERE THE CHOICE LIVES
- The seam vertex, θ = 0: `c + u_ref·semi_u`. It is the only anchor that is exact in the stored terms; at any other θ₀ the anchor carries u·ρ from `cos θ₀` itself.
- The choice lives in the carrier (a `Conic` reading beside `running_point`), resting on the existing convention that a caller seams at the principal vertex nearest the part in use (`Ellipse::seamed_near`, which `chord_join` applies).
- Two gaps in that convention:
  - A circle has no re-seaming door (`Curve3::seamed_near` is `None` for one), so a far circle's arc is near its seam only if its mint put it there.
  - The cylinder's shallow sections have no principal vertex near the in-box arc. They need an anchor at a general arc point with conjugate semi-diameters, which is a stored form.

DOES EVALUATION ALONE RECOVER IT?
- The near-gap families: yes, fully.
- Chords of one carrier: yes, and a plain vertex sum is enough.
- Anything that compares the conic's point to something else in the box (apex, clearance, line, endpoint): only with the vertex exact. A plain `c + u·a` rounds at u·far like the centre form.
- The section tables: no. A stored form anchored in the box, minted well-conditioned, is needed.

WHAT I WOULD BUILD (my recommendation: 1 now, 2 and 3 as their own rows)
1. This unit: a vertex-relative point chain on `Conic`, vertex summed exactly, with its running bound; the three residual mirrors (`conic_quadric_residual`, `conic_cone_residual`, `conic_torus_residual`) and the door's walk read it; floors raised to 40; the two refusal pins re-posed at poses that still cannot be placed; `off_the_apex` reads it.
   - Decision needed: does `point_at` itself become this chain, or is it a second reading beside it?
     - One chain everywhere satisfies "every reader evaluates the point one way", but changes the bits of every conic point. Measured cost of the crude form: 8 of 4,531 lib tests move in `topo`/`geom`/`geom-brep` (ci profile; integration tests, sweep, verbs, demos and renders not run). One is an Interval enclosure that gets wider away from the vertex (`a_parameter_derived_circle_on_its_cylinder_certifies_the_box_in_one_leaf` stalls), so the Interval form needs `−2 sin²(θ/2)` rather than `−sin²/(1 + cos)`.
     - A second reading leaves every existing bit alone, and the door's walk and meter still share one chain, but the tree then has two point evaluators.
   - The exact vertex needs a product-with-residual at the scalar. `Real` has no `mul_add`; `geom_core::exact::two_product` is f64 only. At f64 it can be used directly; at other scalars the plain sum.
2. Separate rows, one per site group, each re-posed about the vertex or without it: `split_section_area` (the default-ε one, `dt − sin dt`), `line_clears_conic`/`conics_clear`, `Cover::arc` and the depth readings, `curve_pair_pose_clear`, `certify`'s endpoints, `row_interval`, `carrier_harmonic`.
3. The stored form (Ev's B) for the section tables' mint: anchor point in the box plus conjugate semi-diameters, behind `Conic`. Measured necessary for (b); not this unit.

SEAM WITH PR 7131 (carrier frames as RigidMotion, head ebf92776, waiting on Ev)
- Item 1: textual overlap only (`conic.rs`, `implicit.rs`, `conic_quadric/*`: field reads become `frame.origin()`, `frame.u_ref()`), with one dependency. 7131's decision 1 makes the true frame the polar factor of the stored entries, not the entries. For a frame that is not exactly orthonormal the true `u` then differs from the stored one by about an ulp, so the true vertex differs from the "exact" one by about u·a. "The frame and the sizes are taken as exact" in the running bound would no longer hold at the far scale, and the bound would have to charge it. If Ev rules for decision 1, item 1's gain shrinks toward mode 1's (0.77 u·ρ) on such frames; if the entries are read as points, it stands as measured.
- Item 2: independent, textual overlap in the same files.
- Item 3: dependent. A vertex- or arc-anchored form changes what the frame's origin is, which is exactly what 7131 is deciding the type of. It also removes the dependency above, since the frame's error then multiplies only the small offsets.

UNSURE OF
- The cone refusal pin's newly certified roots (not oracle-checked).
- The (b) probe covers plane × cylinder and plane × cone only; other tables that can mint a far-centre conic were not swept.
- Whether production circles reach the door seamed near their arc; the circle mints were not surveyed.

FRICTION
- The fifteen-site probe took about 25 minutes of wall time across three parallel workers in separate worktrees; done serially it would have been well over the 15-minute line per group.
- `Real` has no door to an f64 point value from generic code in `geom`, so the exact-vertex measurement went through a downcast in the scratch patch.

---

APPENDIX (not in the report as sent)

A. The two refusal pins the exact vertex turns into certifications — evidence for the stored-form unit, not defects.
- `in_band_sweep::a_root_whose_slack_reads_in_the_gap_is_refused` (ρ = 1500, depth 5.44e-8, ε 1e-9). Mode 0 and mode 1: refused, the row green. Mode 3: `Certified`, roots −9.139608097293848e-7 and 9.139605262399492e-7; against `exact::truth` they lie 7.09e-13 m and 2.35e-13 m of arc from a true root (`GAPROOT` lines). The row's own doc says a meter that accepted the gap reading on main's chain certifies roots 2.05e-9 m off; on the exact-vertex chain the root itself is placed three orders better, so the certification is true.
- `cone_rows::a_root_the_slack_meter_cannot_place_refuses`, at d = 1e-5: "a root charged 4.44e-11 is refused, got Certified, count 4, roots −1.5707963267948968, −8.24e-17, 1.4142235625699017e-5, 1.570782184559271". NOT checked against an oracle.
- Both red at CAD_TOLERANCE_EPS 1e-9, 1e-6 and 1e-12 in mode 3; both green in modes 0 and 1.

B. What moves with every conic point in mode 1 (lib suites, ci profile; `logs/*_failures.txt`).
- topo (2,942 run): `ruled_region_rows::a_cones_circle_legs_read_no_chart_frame_at_the_escalation` (query count 35 → 38), `torus_region::tests::a_torus_faces_circle_legs_read_no_chart_frame_at_the_escalation` (31 → 27), `splitting::containment::arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size` (point 4.2e-10 past the cover's ball).
- geom + geom-brep (1,589 run): `conic::tests::the_conic_readings_are_the_carriers_own` (bits), `carrier_resolution::tests::the_resolution_bounds_each_section_tables_carrier` (a sphere's circle at 0.106 of the allowance against the 0.1 line), `ellipse_torus::torus_rows::the_slack_meter_charges_every_term`, `curve_pair::touch_spread_rows::the_subdivisions_touch_whose_spread_exceeds_the_band_escalates`, `edge_nurbs::tests::a_parameter_derived_circle_on_its_cylinder_certifies_the_box_in_one_leaf` (`ssi_hull_sup` stalls, enclosure [0, 2.42]).

C. How the workers' tables in (c) were taken: see `README.md` for the commands. Probe A's truth is computed offline from dumped f64 bits; probes B and C compute it in the test with `two_sum`/`two_product` or a closed form. All counts are per cell of (ε, far[, ratio]) with the pose counts stated in the table.
