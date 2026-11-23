# Wedge-measure reports (`embed/wedge-measure`, not for merge)

The reports as sent to the EMBED orchestrator, in order. Commands and raw
rows: `README.md` beside this file. Band (`Tol::witness`): zero 1e-9,
escalate 1e-8.

## 1. Final report (items 1–5)

Branch cut at 1a640b685 (PR 7329 head); `embed/gate-measure` merged.

**Item 1 — the sliver through `sign_on`** (`injective::wedge_measure_rows`).
Geometry: cylinder r = 1e-3 leaving the plane at s = 1e-4; second cut at
chord 2.000e-7 (= 2r sin θ), deepest 5.000e-12 (= rθ²/2). Sliver
u ∈ [−s + 2e-8, 0]: width 9.998e-8, far boundary 5.000e-12 off, far rate
2.00002e-8. Consistent. Rate enclosure over the whole cell
[−1.0000e-4, −2.00002e-8]; check 4 at this pair: fold 1e-3, Corner.
(a) lever 1.0: margin lo 2.00e-8, CERTIFIES. (b) lever 1e-3: margin
[−1e-7, −2.0e-11], Indeterminate, REFUSES. (c) sign alone: CERTIFIES.

**Item 2 — built bodies** (`crates/sweep/tests/wedge_measure.rs`, `wm2-rows.txt`).
No door builds the sliver: only the chord plane goes through both rulings.
Profile + extrude: every short-arc lens NonSimple{Overlap}; a single long
arc NearFullArc. Boolean rod r = 1e-3 ∩/− brick 5e-12 (also 1e-11, 1e-13)
deep: ∩ Empty, − returns the rod unchanged. Plain long-arc case (two arcs):
tier 3 PASS, struts Crease, Intersection, lever = fold 1e-3 (reach 0.5),
margin 1.0e-7; `embedded` Refused Uncertified/Departure at the cap–flat
plane/plane edge (f64 and Interval). Moved in (depth 1e-11): Crease
1.41e-7, Refused at the flat–cylinder strut. Moved out (depth 1e-13):
Crease 1.414e-8, Refused as plain.

**Items 3–5 population**: census list (1,313; 1,301 ran), 5-min cap. P0
(PR 7329): 60 pass, 1,217 fail, 24 time out. Tour: `embed_walk` with
`CAD_GATE_PASS=1`.

**Item 3 — sign alone beside creases.** P walk: 254,544 crease cell pairs
uncertified by PR 7329 where sign alone certifies (240 tests, 1,027 face
pairs; 279 refused for good). Mutant walk: 17,299 flips. Far-side gaps:
<0 1,926 (104 in (−1e-8, 0); 453 below −1, blown-up enclosures);
0..1e-9 383; 1e-9..1e-8 0; 1e-8..1e-6 6; 1e-6..1e-3 3,778; ≥1e-3 11,044;
not read 162. Face pairs alone (7329, mutant): 891 E/E, 130 R/R, 6 R→E.
One test FAIL→PASS (a_cap_moved_off_a_needle_apex_refuses_the_hyperbola).
Tour verdicts identical (78/85).

**Item 4 — bisector sum.** 297,886 crease cell pairs refused for good
(294,098 at depth 40, 3,788 Refuse); 28,114 reach the bisector, 250 a conic
plane, 269,522 no plane separator. The sum clears where not both alone in
16; where neither alone in 0. Both definite alone but on the same side:
19,190 (see §4 below). Tour: 58, none reach a plane.

**Item 5 — reach vs fold.** 210,058 per-ask edge readings: within 2× 175,260;
reach > 2×fold 33,581; fold NaN 1,217. 4,313 cell pairs differ, all
7329-certifies/fold-refuses; 4,185 at a NaN fold; finite: 128 on 27 edges
(23 past 2×). Tour: 13,580, all NaN fold.

## 2. The table and the summed-rate row

| case | reach (PR 7329) | check 4's fold | narrower face's run |
|---|---|---|---|
| cylinder sliver (unit) | 2.0e-8 certifies | 2.0e-11 refuses | 2.0e-15 refuses |
| flat twin (unit) | 5.0e-5 certifies | 5.0e-5 certifies | 5.0e-12 refuses |
| apex cone (built) | Embedded (quarter), Refused at the apex ruling, depth 40 (half) | same | same |

All unit entries match the designers' predictions. Check 4: sliver Corner,
lever 1e-3, margin 1e-7; flat twin Transverse, lever 1, margin 5e-5.
Neither the sliver nor the flat twin builds (NonSimple Overlap / Touch).
Apex cone (revolve Partial π/2, π): check 4 Crease, margin 0.0625; along the
apex ruling fold 0 → 0.5 linear, sine 1, sine×fold 0 at the apex station
(in band, rung Arm), least excluding the apex 0.05. Oblique cone: least
station sine×fold 0.232 against least sine × least fold 0.128.

Summed rate (wedge sine 1.2e-8, reach 1): carrier arm certifies (1.2e-8 +
0); bisector each alone 6e-9 refuses, summed 1.2e-8 decides.

## 3. The narrower face's run with summed rates — body-level moves

Census (P0 vs N): 1,332 sequences, 1,311 unmoved, 21 moved. Embedded→Refused 3,
Refused→Embedded 5, Refused→Crosses 1, Refused→Refused 12. The three new
refusals:
1. `a_split_through_a_strut_vertex_keeps_one_face_where_the_piece_goes_round`,
   Cylinder/Plane crease, narrower face a Plane 6.46e-10 across, refused at
   depth 40 (6,628 cells).
2. `flat_wedge_sunk_at_20_degrees`, Plane/Plane crease, narrower Plane
   1.71e-10, 19 cells at depth 40.
3. `into_a_slab_in_band`, Budget(Pairs), no edge named.
No new refusal on a narrower face wider than 1e-9. Tour: 85 bodies, 2,067
asks unmoved. The closed-rim fallback (half the boundary's diameter) is
this lane's choice, not the design text's. Full list: `moves-N.txt`.

## 4. Both faces definite on one side of a separator through the shared cell

The 19,190 (PR 7329 targeted run, crease cell pairs refused for good)
split as **19,164 at the bisector of one straight edge, all in one test**
(`sweep parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region`),
and **26 at a conic's own plane** (`pl`) in four tests. Probe: the five
tests re-run with `wm_same_side_probe` (`injective.rs` on this branch;
rows `log-F`, summary below). ux, uy = each face's departure at the edge's
middle (its chart derivative across the zero side, inward, perpendicular
to the edge); n1 = ux − uy and n2 = ux + uy are the two planes bisecting
the tangent planes; `sign_on` is read under each, at PR 7329's arm.

| test (edge, faces) | tier 3 | angle(ux,uy) | chosen plane | sign_on under chosen | sign_on under the other |
|---|---|---|---|---|---|
| parallel_cylinder_join::seams_lying… (14v1, 3v1/8v3; also 8v1, 13v1, 15v1, 3v1) | Cylinder/Cylinder Crease, Concave | 89.9–90.1° | bisector = n2 (cos 1.000) | +/+ | n1: +/− (opposite) |
| coincidence_door::the_coaxial_stage6_unions… (18v1, 5v1/8v3) | Cylinder/Sphere Crease, Concave | 51.1° | conic plane | +/+ | n1: +/− |
| coincidence_door::every_band_a_revolve_mints… (15v7, 5v1/12v1) | Crease, Concave (key also a Seam in another body) | 0.2° | conic plane | +/+ | n1: None/− |
| germ_plane_torus_chord::tangs_tube_on_a_latitude… (6v3, 3v1/9v3) | Cylinder/Torus Crease (Concave or Convex by body) | 0.1° | conic plane | +/+ | n1: +/None |
| bisector_nodes::a_rim_vertex_on_a_node… (21v1, 5v3/8v7) | Cylinder/Cylinder Crease (Concave or Convex by body) | 16.2° (0.3–0.4° deeper) | conic plane | +/+ | n1: +/− (None at 0.3°) |

(Edge keys repeat across the bodies of one test, so two classes can show
for one key.)

**One cause for the 19,164:** `planes_through` builds the line edge's
plane from the faces' headings (`heading`, the direction from the shared
edge's start to the rectangle's middle; PR 7329 head `injective.rs:5987`,
read at `:6086–6087`, bisected at `:6106` by `bisector`, `:710`). There one
face's rectangle has almost no width across the edge (u 1.5708–1.5708 at
four decimals), so its middle lies 2.3e-10 to 3.6e-7 m off the edge line,
while the anchor is 0.5 m along it: the heading is (along 1.0, perpendicular
2.3e-10). Its perpendicular part is anti-aligned with that face's
departure (cos −1.000), so `bisector` returns the other bisecting plane,
n2, which both faces leave at 45° on one side (rates +0.707/+0.707). Under
n1 the same `sign_on` reads Positive/Negative at rates ±0.707. Every
probed cell of this test reads so (19,184 probed rows).

**The 26 at a conic plane** are a different thing: the conic's own plane
is not a bisector, and both faces leave the edge on one side of it (angles
0.1°–51° between departures). Under n1 the faces read opposite where the
angle is 16°–51°, and one face undecided where they are within 0.4° of
tangent.
