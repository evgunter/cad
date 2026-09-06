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

| name | bucket | evidence | site | rule (M10-8) |
| --- | --- | --- | --- | --- |
| `offset_reanchor_on_carrier` | IMPLICIT | an offset carrier re-anchored through a solve | `crates/topo/src/replace_face.rs` | not in the M10-8 documents |
| `plane_nurbs_on_locus` | IMPLICIT | a chart-image foot, found by a solve | `crates/geom-brep/src/certify.rs` | not in the M10-8 documents |
| `ssi_on_locus` | IMPLICIT | a marched intersection point's residual | `crates/geom-brep/src/ssi/certify.rs` | not in the M10-8 documents |
| `ssi_on_locus_foot` | IMPLICIT | the foot of that point's projection | `crates/geom-brep/src/ssi/certify.rs` | not in the M10-8 documents |
| `arc_continue_needs_arc_carrier` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` | not in the M10-8 documents |
| `arc_continue_off_carrier` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` | not in the M10-8 documents |
| `carrier_in_chain` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` | not in the M10-8 documents |
| `carrier_kind` | NOT A PREDICATE | a diagnostic name on an `Indeterminate` carrying `MarginDiag::Invalid` | `crates/topo/src/boolean/carrier_eq.rs` | not in the M10-8 documents |
| `encloses_leg_carrier` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` | not in the M10-8 documents |
| `frame_coincidence` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/py/mate.rs` | not in the M10-8 documents |
| `measure_not_parallel` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` | not in the M10-8 documents |
| `rebind_identity` | NOT A PREDICATE | a `pncad-py` tag string, never a classified margin | `crates/pncad-py/src/tests.rs` | not in the M10-8 documents |
| `arc_apex_identity` | EXPLICIT | closed form at the site | `crates/profile/src/seg.rs` | — |
| `bool_curved_contain_carrier` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contain.rs` | not in the M10-8 documents |
| `bool_dir_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/sectors.rs` | not in the M10-8 documents |
| `bool_face_disc_carrier` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contain.rs` | not in the M10-8 documents |
| `bool_faces_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/sectors.rs` | not in the M10-8 documents |
| `bool_germ_frame_axes_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/join.rs` | not in the M10-8 documents |
| `bool_plane_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` | not in the M10-8 documents |
| `bool_sphere_escape_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/ops.rs` | not in the M10-8 documents |
| `bool_sphere_extent_gap` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/ops.rs` | not in the M10-8 documents |
| `bool_sphere_sphere_gap` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/ops.rs` | not in the M10-8 documents |
| `carrier_circles_external` | EXPLICIT | closed form at the site | `crates/profile/src/seg.rs` | not in the M10-8 documents |
| `carrier_circles_identity` | EXPLICIT | MEASURED 6 / 0 (symbolic / numeric) | `crates/profile/src/seg.rs` | plain |
| `carrier_circles_internal` | EXPLICIT | closed form at the site | `crates/profile/src/seg.rs` | not in the M10-8 documents |
| `carrier_cyl_axis_offset` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` | not in the M10-8 documents |
| `carrier_cyl_axis_parallel` | EXPLICIT | MEASURED 3 / 0 (symbolic / numeric) | `crates/topo/src/boolean/contact_verify.rs` | plain |
| `carrier_cyl_radius` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` | not in the M10-8 documents |
| `carrier_endpoint_end` | EXPLICIT | MEASURED 56 / 16 (symbolic / numeric) | `crates/geom-brep/src/certify.rs` | plain |
| `carrier_endpoint_start` | EXPLICIT | MEASURED 56 / 16 (symbolic / numeric) | `crates/geom-brep/src/certify.rs` | plain |
| `carrier_in_seam_halfplane` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` | not in the M10-8 documents |
| `carrier_line_circle` | EXPLICIT | closed form at the site | `crates/profile/src/seg.rs` | — |
| `carrier_matches_mapped_source` | EXPLICIT | MEASURED 288 / 72 (symbolic / numeric) | `crates/sweep/tests/m5_s12_curved_ops_interval.rs` | plain |
| `carrier_on_seam_side` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` | not in the M10-8 documents |
| `carrier_on_surface_1` | EXPLICIT | MEASURED 216 / 72 (symbolic / numeric) | `crates/geom-brep/src/certify.rs` | plain+A0, A0 |
| `carrier_on_surface_2` | EXPLICIT | MEASURED 216 / 72 (symbolic / numeric) | `crates/geom-brep/src/certify.rs` | plain, A0 |
| `carrier_sphere_center` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` | not in the M10-8 documents |
| `carrier_sphere_radius` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` | not in the M10-8 documents |
| `carrier_torus_axis_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/carrier_eq.rs` | not in the M10-8 documents |
| `carrier_torus_center` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/carrier_eq.rs` | not in the M10-8 documents |
| `carrier_torus_major_radius` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/carrier_eq.rs` | not in the M10-8 documents |
| `carrier_torus_minor_radius` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/carrier_eq.rs` | not in the M10-8 documents |
| `cc_axes_parallel` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `cc_parallel_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `census_backstop_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `chart_region_carrier_tilt` | EXPLICIT | closed form at the site | `crates/topo/src/chart_region.rs` | not in the M10-8 documents |
| `chart_region_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/chart_region.rs` | not in the M10-8 documents |
| `compose_seam_gap` | EXPLICIT | closed form at the site | `crates/geom/src/curves/compose.rs` | not in the M10-8 documents |
| `contact_tangent_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/contact_verify.rs` | not in the M10-8 documents |
| `enclosing_fillet_swallows_both_leg_carriers` | EXPLICIT | closed form at the site | `crates/profile/tests/review_s2.rs` | not in the M10-8 documents |
| `endpoint_coincidence` | EXPLICIT | closed form at the site | `crates/geom-core/tests/review_m0_pr3.rs` | not in the M10-8 documents |
| `extreme_weight_carrier` | EXPLICIT | closed form at the site | `crates/mesh/src/chords.rs` | not in the M10-8 documents |
| `face_gap_classification` | EXPLICIT | closed form at the site | `crates/geom-core/tests/review_m0_pr3.rs` | not in the M10-8 documents |
| `fillet_enclosing_carrier` | EXPLICIT | closed form at the site | `crates/profile/src/sugar.rs` | not in the M10-8 documents |
| `fillet_offset_circles_external` | EXPLICIT | closed form at the site | `crates/profile/src/sugar.rs` | not in the M10-8 documents |
| `fillet_offset_circles_internal` | EXPLICIT | closed form at the site | `crates/profile/src/sugar.rs` | not in the M10-8 documents |
| `identity_at_interval` | EXPLICIT | closed form at the site | `crates/geom-core/tests/m10_7_r1_retag_probe.rs` | not in the M10-8 documents |
| `mate_axes_parallel` | EXPLICIT | closed form at the site | `crates/editor-core/src/mate/coset.rs` | not in the M10-8 documents |
| `mate_member_rotation_identity` | EXPLICIT | closed form at the site | `crates/editor-core/src/mate/coset.rs` | not in the M10-8 documents |
| `near_zero_weight_carrier` | EXPLICIT | closed form at the site | `crates/mesh/src/chords.rs` | not in the M10-8 documents |
| `offset_axial_edge_on_surface` | EXPLICIT | closed form at the site | `crates/topo/src/offset_axial.rs` | not in the M10-8 documents |
| `offset_torus_carrier_axis` | EXPLICIT | closed form at the site | `crates/topo/src/replace_face.rs` | not in the M10-8 documents |
| `path_arc_continue_on_carrier` | EXPLICIT | closed form at the site | `crates/profile/src/path.rs` | not in the M10-8 documents |
| `path_carrier_identity` | EXPLICIT | closed form at the site | `crates/profile/src/path.rs` | not in the M10-8 documents |
| `path_carrier_meet` | EXPLICIT | closed form at the site | `crates/profile/src/path/arc_fillet.rs` | not in the M10-8 documents |
| `pc_axis_plane_parallel` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `pc_parallel_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `pm_census_confined_carrier` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `pm_census_ee_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `pm_census_ee_line_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `pm_census_ee_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `pm_census_ef_cut_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `pm_census_span_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `pm_census_ve_line_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `pm_census_vv_gap` | EXPLICIT | closed form at the site | `crates/topo/src/census.rs` | not in the M10-8 documents |
| `props_meridian_on_surface` | EXPLICIT | closed form at the site | `crates/geom-brep/src/props/curved.rs` | not in the M10-8 documents |
| `props_rim_axis_parallel` | EXPLICIT | closed form at the site | `crates/mesh/src/curved.rs` | not in the M10-8 documents |
| `ps_center_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `pt_axis_plane_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `pt_cap_gap` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `r2_identity` | EXPLICIT | closed form at the site | `crates/geom-core/tests/m10_7_r2_sym_probes.rs` | not in the M10-8 documents |
| `r2_sym_identity` | EXPLICIT | closed form at the site | `crates/geom-core/tests/m10_7_r2_sym_probes.rs` | not in the M10-8 documents |
| `rational_mult_2_carrier` | EXPLICIT | closed form at the site | `crates/mesh/src/chords.rs` | not in the M10-8 documents |
| `rational_mult_p1_carrier` | EXPLICIT | closed form at the site | `crates/mesh/src/chords.rs` | not in the M10-8 documents |
| `rim_circle_axis_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/rim_wedge.rs` | not in the M10-8 documents |
| `ring_outer_locus_gap` | EXPLICIT | closed form at the site | `crates/topo/src/validate.rs` | not in the M10-8 documents |
| `ring_outer_vertex_gap` | EXPLICIT | closed form at the site | `crates/topo/src/validate.rs` | not in the M10-8 documents |
| `rounded_square_with_seam_fillet_matches_explicit_hand_chain` | EXPLICIT | closed form at the site | `crates/profile/tests/path_differential.rs` | not in the M10-8 documents |
| `self_intersection_gap` | EXPLICIT | closed form at the site | `crates/editor-core/src/clearance.rs` | not in the M10-8 documents |
| `shell_walls_antiparallel` | EXPLICIT | closed form at the site | `crates/topo/src/shell.rs` | not in the M10-8 documents |
| `side_cylinders_cosurface` | EXPLICIT | MEASURED 4 / 0 (symbolic / numeric) | `crates/sweep/src/extrude.rs` | plain |
| `side_planes_cosurface` | EXPLICIT | MEASURED 0 / 8 (symbolic / numeric) | `crates/sweep/src/extrude.rs` | — |
| `split_carrier` | EXPLICIT | closed form at the site | `crates/geom/tests/curves/lt_r1_probes.rs` | not in the M10-8 documents |
| `split_conic_plane_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/splitting/classify.rs` | not in the M10-8 documents |
| `ss_carrier_external` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `ss_carrier_identity` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `ss_carrier_internal` | EXPLICIT | closed form at the site | `crates/geom-brep/src/intersect.rs` | not in the M10-8 documents |
| `tangent_locus_axis_parallel` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/rest.rs` | not in the M10-8 documents |
| `tangent_locus_gap` | EXPLICIT | closed form at the site | `crates/topo/src/boolean/rest.rs` | not in the M10-8 documents |
| `tangent_normal_parallel` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` | — |
| `tangent_on_surface_1` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` | A0 |
| `tangent_on_surface_2` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` | — |
| `tube_wall_gap` | EXPLICIT | closed form at the site | `crates/sweep/src/revolve/tube.rs` | not in the M10-8 documents |
| `wall_arcs_cosurface` | EXPLICIT | closed form at the site | `crates/sweep/src/revolve/mod.rs` | not in the M10-8 documents |
| `wall_lines_cosurface` | EXPLICIT | closed form at the site | `crates/sweep/src/revolve/mod.rs` | not in the M10-8 documents |
| `witness_on_surface_1` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` | plain+A0, A0 |
| `witness_on_surface_2` | EXPLICIT | closed form at the site | `crates/geom-brep/src/certify.rs` | plain, A0 |

## The rule column (M10-8): which mechanism discharges each row

The table's fifth column, per row, from M10-8's fixed shape-report
instrument (`editor-core/tests/m10_8_arc_family_interval.rs`, the
per-predicate table at the nominal of the two-hole plate, R2's filleted
L-bracket and R1's bracket, one column per rung of the rule ladder
`none → A0 → A0+AB_top → A0+C_early → +AB_top`), read as:

- **plain** — discharged by the plain quotient form (M10-7's tier),
  every rung the same;
- **A0** — first discharged by the constant fold (`sqrt(1)^k`,
  `sqrt` of an exact-square dyadic constant), the rule that SHIPS
  (`geom_core::SymRules::shipped`), on one or more of the documents;
  **plain+A0** where the row was partly plain already and A0 moved the
  rest (e.g. `carrier_on_surface_1` on the plate: 108/72 → 180/0
  theorem/numeric);
- **—** — numeric on every rung, or not decided at the nominal;
- **not in the M10-8 documents** — the row's predicate never fired on
  the three documents (the column says nothing about it).

No row reads **C**: rule C's clause-3 fold (`sign_gated`) folds on none
of the three documents at the shipped 256-bit coefficient bound and
moved no ceiling at any bound, so it is built, dial-selectable and off.
No row reads **A/B**: over the top residual they add nothing once A0
has run; per node they reach the plate's nested `sqrt(…)²` at minutes
per replay and are off on cost. The rows that stay numeric on every
rung are the arc family's remainder — `carrier_endpoint_start` /
`_end` on the plate (the rim's `‖q − c‖ = r`, an outer `sqrt` over
nested even-power atoms:
`work/m10/plate-rim-residual-needs-the-wide-coefficient-ring`), the
declared tangency `carrier_line_circle` on R2's pad
(`work/m10/declared-tangency-needs-the-registered-identity-door`), and
the real-margin class M10-7 named (`arc_diameter_clearance`,
`line_span`: `work/m10/real-margin-dependency-widening`).

The counts behind the column, at the nominal, theorem/gated/numeric:

| document | predicate | none | A0 (shipped) |
| --- | --- | --- | --- |
| plate | `carrier_on_surface_1` | 108/0/72 | 180/0/0 |
| plate | `newell_plane_residual` | 32/0/8 | 40/0/0 |
| plate | `witness_on_surface_1` | 12/0/8 | 20/0/0 |
| plate | `carrier_endpoint_start` | 32/0/16 | 32/0/16 |
| R2 bracket | `carrier_on_surface_1` | 0/0/243 | 108/0/135 |
| R2 bracket | `carrier_on_surface_2` | 0/0/243 | 135/0/108 |
| R2 bracket | `newell_plane_residual` | 8/0/48 | 36/0/20 |
| R2 bracket | `carrier_endpoint_start` | 44/0/22 | 42/0/24 |
| R2 bracket | `carrier_matches_mapped_source` | 234/0/99 | 225/0/108 |
| R1 bracket | `carrier_on_surface_1` | 72/0/108 | 180/0/0 |
| R1 bracket | `carrier_on_surface_2` | 0/0/180 | 108/0/72 |

The two R2 rows that LOSE theorems under A0 REPLACING the plain form
(`carrier_endpoint_start` 44 → 42, `carrier_matches_mapped_source`
234 → 225) lose them to coefficient freezes at the 256-bit bound — the
folded constants' products grow past it where the opaque atoms' did
not. That variant does not ship: A0 runs in a second walk ALONGSIDE
the plain form, so every plain theorem stays and the fold only adds,
at ~1.8× the cost per leaf where the plain form does not answer; the
bracket's whole-certifying ceiling moves 10.4× (`3.7e1 · ε` →
`3.9e2 · ε`) either way. At 4096 bits nothing freezes even replacing,
at 229 s per leaf against 5.9 s — coefficient growth, not the ring's
heap traffic, which the `i128`-inline integer removed. The bound is the
measured trade (`geom_core::sym::COEFF_BITS`).
