---
id: symbolic-tier-census
kind: issue
title: "the symbolic tier's census: every identity-shaped predicate name, bucketed, with its evidence"
status: open
opened: 2026-09-04
---

**The long form of M10-7's census** — the full table, one row per name,
which `geom_core::sym`'s module docs summarize and cite. It lives here
rather than in the module because 107 rows of evidence is a reference
and the module needs to stay readable; the module carries the counts,
the argument and the two families that matter, and points here for the
rest.

## The sweep, stated so it can be re-run

Two greps over `crates/` and `demos/`, then a filter:

```
A: grep -rhoE '(decide|decide_bounds|classify)\(\s*"[a-z0-9_]+"' --include=*.rs
B: grep -rhoE '"[a-z0-9_]*(coincid|cosurface|identity|endpoint|on_surface|
                on_locus|on_carrier|_gap|carrier|parallel|matches|circles)
                [a-z0-9_]*"' --include=*.rs
```

then the union, minus the bare filter words themselves (`carrier`,
`parallel`, `matches`, `identity`, `circles`, `coincide`, `coincident`,
`coincidence`, `no_carrier`) and minus test-harness names matching
`matches_loopbuilder`. **107 names.**

**Its blind spots, which are the previous sweep's and are still real.**
A misses a predicate named through a wrapper or a table — and that miss
is not hypothetical here: `carrier_endpoint_start`, the name M10-3's
whole finding turned on, does not appear in A, because its site reaches
the funnel through a crate-local helper. B cannot tell a predicate name
from any other string literal of that shape, which is why the
`pncad-py` tag strings arrive and have to be bucketed out by hand.
Neither sees a name using none of the filtered words — see *Outside the
filter* below, which is the class that costs.

**107, not the 66 the previous sweep reported.** The earlier number is
not reproducible from a rule written down anywhere, so this file states
its own rule rather than quoting a count it cannot re-derive. The
difference is filter width, not new predicates: the buckets' SHAPE is
unchanged, and the one claim that matters — that the IMPLICIT bucket is
exactly S-CERT's four names and no more — holds at either width.

## The counts

| bucket | count |
| --- | --- |
| IMPLICIT (S-CERT's frontier) | 4 |
| NOT A PREDICATE | 8 |
| EXPLICIT | 95 |
| **total** | **107** |

`EXPLICIT` means the margin is a closed form in the parameters over
analytic carriers — a distance, a dot, a cross, a radius difference, a
levered sine — which is what the normal form is made of. Nine of them
carry a MEASURED symbolic/numeric split from
`editor-core/tests/m10_7_census_probe.rs`; the rest carry their site,
and the site is the evidence: a closed form is visible in the
expression the predicate is handed. Many sit on paths that run at `f64`
over no parameter box at all (the `mate`/`coset` family, the boolean
contact verifier), so no fixture in this repository can exercise them
symbolically — that is a statement about the fixtures, not about the
margins.

## Outside the filter, and this is the class that costs

**Five names the sweep's word list cannot see, which the driver's own K
CSV shows deciding or bounding a ceiling.** None contains any filtered
word, so no sweep of this shape would ever have listed them — and two
of them are what bounds the slab today.

| name | why it is not in the sweep | what it does |
| --- | --- | --- |
| `newell_plane_residual` | no filter word | 1,584 symbolic decisions in the driver population; and its INVALID arm is what a one-leaf replay of a wide box fails on |
| `segment_straightness` | no filter word | 1,650 symbolic decisions |
| `witness_at_mid_parameter` | no filter word | 1,377 symbolic decisions |
| `dihedral_wedge` | no filter word | the predicate that SETS the slab's ceiling, by landing in the band |
| `arc_diameter_clearance` | no filter word | `[-3e-4, 1.7e-3]` on a 0.73 mm margin at a ±0.5 mm box (R1's bracket) — dependency widening on a clearance three decades from zero |

**What this says about the census as an instrument.** It was built to
answer "which identity-shaped predicates does the tier reach", and it
answers that. It does not answer "which predicates bound certification",
and the two questions have different populations: the ceiling is set by
a predicate the census cannot name. The follow-on is
`work/m10/real-margin-dependency-widening.md`, whose population is the K
CSV rather than a name filter.

## The table

| name | bucket | evidence | site |
| --- | --- | --- | --- |
| `offset_reanchor_on_carrier` | IMPLICIT | an offset carrier re-anchored through a solve | `crates/topo/src/replace_face.rs` |
| `plane_nurbs_on_locus` | IMPLICIT | a chart-image foot, found by a solve | `crates/geom-brep/src/certify.rs` |
| `ssi_on_locus` | IMPLICIT | a marched intersection point's residual | `crates/geom-brep/src/ssi/certify.rs` |
| `ssi_on_locus_foot` | IMPLICIT | the foot of that point's projection | `crates/geom-brep/src/ssi/certify.rs` |
| `arc_continue_needs_arc_carrier` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` |
| `arc_continue_off_carrier` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` |
| `carrier_in_chain` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` |
| `carrier_kind` | NOT A PREDICATE | a diagnostic name on an `Indeterminate` carrying `MarginDiag::Invalid` | `crates/topo/src/boolean/carrier_eq.rs` |
| `fillet_encloses_leg_carrier` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` |
| `frame_coincidence` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/py/mate.rs` |
| `measure_not_parallel` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` |
| `rebind_identity` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` |
| `arc_apex_identity` | EXPLICIT | closed form at the site | `crates/profile/src/seg.rs` |
| `bool_curved_contain_carrier` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contain.rs` |
| `bool_dir_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/sectors.rs` |
| `bool_face_disc_carrier` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contain.rs` |
| `bool_faces_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/sectors.rs` |
| `bool_germ_frame_axes_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/join.rs` |
| `bool_plane_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` |
| `bool_sphere_escape_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/ops.rs` |
| `bool_sphere_extent_gap` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/ops.rs` |
| `bool_sphere_sphere_gap` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/ops.rs` |
| `carrier_circles_external` | EXPLICIT | closed form at the site | `crates/profile/src/seg.rs` |
| `carrier_circles_identity` | EXPLICIT | MEASURED 6 / 0 (symbolic / numeric) | `crates/profile/src/seg.rs` |
| `carrier_circles_internal` | EXPLICIT | closed form at the site | `crates/profile/src/seg.rs` |
| `carrier_cyl_axis_offset` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` |
| `carrier_cyl_axis_parallel` | EXPLICIT | MEASURED 3 / 0 (symbolic / numeric) | `crates/topo/src/boolean/contact_verify.rs` |
| `carrier_cyl_radius` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` |
| `carrier_endpoint_end` | EXPLICIT | MEASURED 56 / 16 (symbolic / numeric) | `crates/geom-brep/src/certify.rs` |
| `carrier_endpoint_start` | EXPLICIT | MEASURED 56 / 16 (symbolic / numeric) | `crates/geom-brep/src/certify.rs` |
| `carrier_in_seam_halfplane` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` |
| `carrier_line_circle` | EXPLICIT | closed form at the site | `crates/profile/src/seg.rs` |
| `carrier_matches_mapped_source` | EXPLICIT | MEASURED 288 / 72 (symbolic / numeric) | `crates/sweep/tests/m5_s12_curved_ops_interval.rs` |
| `carrier_on_seam_side` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` |
| `carrier_on_surface_1` | EXPLICIT | MEASURED 216 / 72 (symbolic / numeric) | `crates/geom-brep/src/certify.rs` |
| `carrier_on_surface_2` | EXPLICIT | MEASURED 216 / 72 (symbolic / numeric) | `crates/geom-brep/src/certify.rs` |
| `carrier_sphere_center` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` |
| `carrier_sphere_radius` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` |
| `carrier_torus_axis_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/carrier_eq.rs` |
| `carrier_torus_center` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/carrier_eq.rs` |
| `carrier_torus_major_radius` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/carrier_eq.rs` |
| `carrier_torus_minor_radius` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/carrier_eq.rs` |
| `cc_axes_parallel` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `cc_parallel_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `census_backstop_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `chart_region_carrier_tilt` | EXPLICIT | closed form at the site | `crates/topo/src/chart_region.rs` |
| `chart_region_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/chart_region.rs` |
| `compose_seam_gap` | EXPLICIT | closed form at the site | `crates/geom/src/curves/compose.rs` |
| `contact_tangent_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` |
| `enclosing_fillet_swallows_both_leg_carriers` | EXPLICIT | closed form at the site | `crates/profile/tests/review_s2.rs` |
| `endpoint_coincidence` | EXPLICIT | closed form at the site | `crates/geom-core/tests/review_m0_pr3.rs` |
| `extreme_weight_carrier` | EXPLICIT | closed form at the site | `crates/mesh/src/chords.rs` |
| `face_gap_classification` | EXPLICIT | closed form at the site | `crates/geom-core/tests/review_m0_pr3.rs` |
| `fillet_enclosing_carrier` | EXPLICIT | closed form at the site | `crates/profile/src/sugar.rs` |
| `fillet_offset_circles_external` | EXPLICIT | closed form at the site | `crates/profile/src/sugar.rs` |
| `fillet_offset_circles_internal` | EXPLICIT | closed form at the site | `crates/profile/src/sugar.rs` |
| `identity_at_interval` | EXPLICIT | closed form at the site | `crates/geom-core/tests/m10_7_r1_retag_probe.rs` |
| `mate_axes_parallel` | EXPLICIT | closed form at the site | `crates/editor-core/src/mate/coset.rs` |
| `mate_member_rotation_identity` | EXPLICIT | closed form at the site | `crates/editor-core/src/mate/coset.rs` |
| `near_zero_weight_carrier` | EXPLICIT | closed form at the site | `crates/mesh/src/chords.rs` |
| `offset_axial_edge_on_surface` | EXPLICIT | closed form at the site | `crates/topo/src/offset_axial.rs` |
| `offset_torus_carrier_axis` | EXPLICIT | closed form at the site | `crates/topo/src/replace_face.rs` |
| `path_arc_continue_on_carrier` | EXPLICIT | closed form at the site | `crates/profile/src/path.rs` |
| `path_carrier_identity` | EXPLICIT | closed form at the site | `crates/profile/src/path.rs` |
| `path_carrier_meet` | EXPLICIT | closed form at the site | `crates/profile/src/path/arc_fillet.rs` |
| `pc_axis_plane_parallel` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `pc_parallel_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `pm_census_confined_carrier` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `pm_census_ee_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `pm_census_ee_line_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `pm_census_ee_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `pm_census_ef_cut_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `pm_census_span_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `pm_census_ve_line_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `pm_census_vv_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` |
| `props_meridian_on_surface` | EXPLICIT | closed form at the site | `crates/geom-brep/src/props/curved.rs` |
| `props_rim_axis_parallel` | EXPLICIT | closed form at the site | `crates/mesh/src/curved.rs` |
| `ps_center_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `pt_axis_plane_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `pt_cap_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `r2_identity` | EXPLICIT | closed form at the site | `crates/geom-core/tests/m10_7_r2_sym_probes.rs` |
| `r2_sym_identity` | EXPLICIT | closed form at the site | `crates/geom-core/tests/m10_7_r2_sym_probes.rs` |
| `rational_mult_2_carrier` | EXPLICIT | closed form at the site | `crates/mesh/src/chords.rs` |
| `rational_mult_p1_carrier` | EXPLICIT | closed form at the site | `crates/mesh/src/chords.rs` |
| `rim_circle_axis_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/rim_wedge.rs` |
| `ring_outer_locus_gap` | EXPLICIT | closed form at the site | `crates/topo/src/validate.rs` |
| `ring_outer_vertex_gap` | EXPLICIT | closed form at the site | `crates/topo/src/validate.rs` |
| `rounded_square_with_seam_fillet_matches_explicit_hand_chain` | EXPLICIT | closed form at the site | `crates/profile/tests/path_differential.rs` |
| `self_intersection_gap` | EXPLICIT | closed form at the site | `crates/editor-core/src/clearance.rs` |
| `shell_walls_antiparallel` | EXPLICIT | closed form at the site | `crates/topo/src/shell.rs` |
| `side_cylinders_cosurface` | EXPLICIT | MEASURED 4 / 0 (symbolic / numeric) | `crates/sweep/src/extrude.rs` |
| `side_planes_cosurface` | EXPLICIT | MEASURED 0 / 8 (symbolic / numeric) | `crates/sweep/src/extrude.rs` |
| `split_carrier` | EXPLICIT | closed form at the site | `crates/geom/tests/curves/lt_r1_probes.rs` |
| `split_conic_plane_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/splitting/classify.rs` |
| `ss_carrier_external` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `ss_carrier_identity` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `ss_carrier_internal` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` |
| `tangent_locus_axis_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/rest.rs` |
| `tangent_locus_gap` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/rest.rs` |
| `tangent_normal_parallel` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` |
| `tangent_on_surface_1` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` |
| `tangent_on_surface_2` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` |
| `tube_wall_gap` | EXPLICIT | closed form at the site | `crates/sweep/src/revolve/tube.rs` |
| `wall_arcs_cosurface` | EXPLICIT | closed form at the site | `crates/sweep/src/revolve/mod.rs` |
| `wall_lines_cosurface` | EXPLICIT | closed form at the site | `crates/sweep/src/revolve/mod.rs` |
| `witness_on_surface_1` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` |
| `witness_on_surface_2` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` |

## The rule column (M10-8): which atom-algebra rule discharges each row

M10-8 built two functional-identity rules behind the `SymRules` dial
(A `sqrt(X)²=X`, B `sin²+cos²=1`) and filed a third UNBUILT (C
`sqrt(Q²)=Q` by a certified sign — clause 3; its value-reading fold is
at odds with the bit-identity discipline), and measured, per decide site
on the two-hole plate, R2's filleted L-bracket and R1's bracket, which
rule discharges it. The column is uniform and the result is the unit's
headline finding:

**On all three documents, EVERY explicit row that discharges at all is
discharged by the PLAIN quotient form (M10-7); rules A and B add
nothing.** The rules are correct on small forms (geom-core's rule-A/B
unit rows), but the arc-family forms they target — `carrier_on_surface`,
`witness_on_surface`, the moving radial frame, whose `u_ref·u_ref`
carries `sqrt(v·v)²` — are large enough to FREEZE before a top-residual
reduction can reach them, so switched fully on the rules move no ceiling
and change no row's split. `sign_gated` (rule C) is 0 everywhere because
rule C is unbuilt; the receipt and K token stand reserved.

So the rule column reads **"plain"** for every discharging row and
**"none"** for every numeric one; the atom algebra is filed off by
default (`SymRules::shipped()` is empty), the default tier is the M10-7
quotient form bit for bit, and `docs/M10-8-SPEC.md`'s reserve is
measured and not taken. The `carrier_endpoint_start`/`_end` rows remain
the arc family the tier misses (the rim's `|r − sqrt(D)|`, whose radius
sqrt is buried inside the outer distance sqrt — out of a top-residual
fold's reach); an EARLY per-node reduction reaches them and the wider
arc family, but at a cost and with downgrade hazards that filed it.
