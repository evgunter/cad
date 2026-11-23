---
id: a-circles-centre-is-read-as-a-datum-under-the-conic-gate
kind: issue
title: A circle's centre is read as a datum in about fifty functions pinned under the conic gate
status: parked
opened: 2026-11-23
priority: P2
cost: M
blocked_on: [a-construction-whose-centre-runs-off-still-stores-a-centre]
parent: one-stored-form-waits-on-the-readers-of-a-conics-centre
refs: [7510]
---


Cut from `one-stored-form-waits-on-the-readers-of-a-conics-centre` on 2026-11-23, when its census came in at 323 production reads against the row's 90 (PR 7510, the first part: directions, cover, extent and the gate). The functions below are pinned by that PR's gate (`clippy.toml` `disallowed-methods`, an `#[expect]` on each function, `scripts/gates/conic-centre-reads.counts`) as of `bd07a867e7`. **The sorting is the implementer's, from the census line of each read and not from reading each function through**, and a `°` marks a function that reads only `Circle::frame`. **What these readers should ask the conic is a design question in front of three designers** (`analysis/design-fork/conic-reader-interface`); two of the three already say this family is not one family, so the list is where to look, not the unit's shape.

No construction mints an arc-held circle today, so each of these reads a stored centre and converting them is spelling until one does. **Parked on the four-mints unit**, which is the one to find out whether the STEP import's `recognize_curve` brings an arc-held circle back; if it does, the readers that circle can reach come ahead of that mint.

## The functions (every `°` not placed in another family)

- geom-brep: `curve_face/circle_torus.rs` `axes`°, `meridian`°; `edge_nurbs.rs` `arc_rim_measured`°; `pair_section/gap.rs` `read`°, `critical_gap`°; `pair_section/winds.rs` `azimuth`°, `clear`°; `tangent.rs` `tangent_span_bounds`°; `props/curved.rs` `require_one_chart_branch`°, `cylinder_windings`°, `cylinder_boundary`°, `require_on_cone`°, `cone_boundary`°, `sphere_loop`°, `sphere_boundary`°, `torus_chart_loop`°, `torus_classify`°
- mesh: `sizing.rs` `sphere_face_widest_cos`° (one designer reads this as a linear height harmonic, so an extent read, not a datum)
- step-import: `adopt.rs` `mapped_self_description`°; `normalize.rs` `mint_band`°, `seam_vertex_on_loop`°, `wall_inversion`°
- sweep: `blend/arms.rs` `chamfer_setback_point`°, `frustum_rim_side`°; `blend/battery.rs` `frustum_arm`°, `curved_arm`°; `blend/reach.rs` `circular_reach`°; `blend/surgery.rs` `arc_past`°, `arc_distance`°, `arc_along`°, `of`°, `arc_within`°; `test_support.rs` `arcs_at`°, `circle_arcs_where`°
- topo: `boolean/join.rs` `pt_germ_frame`°; `chart_iso.rs` `traversal_kind`°; `contain.rs` `loop_circle`°; `region.rs` `swept`°, `arc`°, `heading`°; `solid_contain.rs` `wall_crosses_by_turn`°; `surface_group.rs` `coaxial_margins`°; `test_support_fixtures.rs` `arc_cap`°; `validate.rs` `ring_outer_meeting`°, `locus_gap`°
