---
id: the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre
kind: issue
title: The certified lane's harmonics and heights of a conic are formed about its centre
status: open
opened: 2026-11-23
priority: P1
cost: H
design: true
parent: one-stored-form-waits-on-the-readers-of-a-conics-centre
refs: [7510, 7258]
---


Cut from `one-stored-form-waits-on-the-readers-of-a-conics-centre` on 2026-11-23, when its census came in at 323 production reads against the row's 90 (PR 7510, the first part: directions, cover, extent and the gate). The functions below are pinned by that PR's gate (`clippy.toml` `disallowed-methods`, an `#[expect]` on each function, `scripts/gates/conic-centre-reads.counts`) as of `bd07a867e7`. **The sorting is the implementer's, from the census line of each read and not from reading each function through**, and a `°` marks a function that reads only `Circle::frame`. **What these readers should ask the conic is a design question in front of three designers** (`analysis/design-fork/conic-reader-interface`); two of the three already say this family is not one family, so the list is where to look, not the unit's shape.

## The functions

- geom-brep `implicit.rs`: `sample_rounding`, `quadric_harmonics`, `conic_cone_harmonics`, `conic_axis_distance_floor`, `conic_torus_harmonics`, `conic_torus_jet`, `conic_residual_harmonics`, `circle_sphere_harmonic`°
- geom-brep `curve_pair/pose.rs`: `line_conic`, `height`, `conic_conic`, `one_curve_gap`, `coplanar_circles`, `coplanar_ellipses`, `jet_at`, `level_on`; `pose/window.rs`: `height_jet`
- geom-brep `carrier_net.rs`: `harmonic`; `curve_face/conic_quadric.rs`: `apex_spread`; `curve_face/spline_face.rs`: `of`; `curve_face/conic_oracle/exact.rs`: `truth`
- geom `curves/on_carrier.rs`: `clearance` (and `distance_within`°)
- topo `ring_path.rs`: `plane`, `misses_in_plane`, `coplanar_apart`, `plane_roots`°, `path_parity`°, `path_resolution`°, `plane_torus`°
- topo `splitting/section.rs`: `plane_section`; `splitting/section_loops.rs`: `whole_conic_principal`, `line_clears_conic`, `conics_clear`; `shell.rs`: `cut_counting`; `census/depth_gap.rs`: `face_read` (the far axis as depth datum: the arc-cover row's probe B); `readback.rs`: `edge_pose`
- topo `boolean/join.rs`: `bool_connect`, `germ_section_loops`, `pk_section`; `boolean/vtxfac.rs`: `read_vertex_on_face`; `injective.rs`: `one_conic`, `one_stated_conic`, `planes_through`
- sweep `blend/open/end_face.rs`: `plan`, `depth`, `outside`, `apart`; `blend/surgery.rs`: `piece_distance`

## In none of the four families

- geom `curves.rs`: `same_content_by`°, `frame` (`Curve3::frame` itself, one of the five gated doors), `data`°, `representability_margins` (the stored-data walks): with whichever unit retires `Curve3::frame`.
- step-export `writer.rs`: `edge_curve` (the derived centre by the owner's decision; listed so the gate holds it, not owed).
- geom `cover.rs`: `arc_of`, PR 7510's own new function, which reads `Conic::frame` to fill the piece's `frame` field. The implementer reported it against itself; it is a fix-pass item of PR 7510, not this row's.
