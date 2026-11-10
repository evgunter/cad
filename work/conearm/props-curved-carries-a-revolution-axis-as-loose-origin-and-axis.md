---
id: props-curved-carries-a-revolution-axis-as-loose-origin-and-axis
kind: issue
title: props/curved.rs's cylinder, cone and torus readers carry the axis as loose (origin, axis) pairs, about thirty functions in one family
status: open
opened: 2026-11-09
priority: P2
cost: M
---

Filed by ONECOPY's `onecopy/line-placement` lane, which gave every other axis holder it swept a `geom::Line` (`a-line-still-travels-as-loose-arguments-or-a-frame-pair`). This family was left whole on purpose: porting half of it would leave two spellings at one boundary.

## The family

All in `crates/geom-brep/src/props/curved.rs`. Each takes or holds a surface of revolution's axis as a loose `(origin|apex|center, axis)` pair:
- **cylinder:** `cylinder_face`, `cylinder_chart`, `cylinder_windings`, `cylinder_material_sign`, `cylinder_boundary`;
- **cone:** `cone_face_closed_form` (pub), `cone_face_read`, `cone_loops`, `require_on_cone`, `require_cone_circle`, `require_cone_ellipse`, `require_cone_generator`, `cone_material_sign`, `require_cone_loop_vertices_distinct`, `cone_joints`, `cone_loop_winding`, `cone_boundary`;
- **torus:** `torus_parse`, `torus_rims_at_extremes`, `torus_ends`, `torus_chart`, `torus_chart_loop`, `torus_face`, `torus_material_sign`, `sheet_material_sign`, `sheet_apex_jumps`, `torus_boundary`, `torus_classify`, `torus_meridian_orient`;
- **struct:** `SphereArc` (its `center`, `axis`).

## The boundary sites

These are the places where the family meets doors that already take a `geom::Line`. Port the family through them, so no second spelling appears at a boundary:
- `agreed_side`, which calls `chart_singularity::sphere_poles(geom::Line { origin, dir: up }, …)` from loose `(origin, up)`;
- `require_cone_generator`'s three callers (`require_on_cone`, `cone_boundary` and a test row). They pass `(apex, axis, …)`, which `pole_form::cone_generator` now takes as a `Line`.

## The shape of a fix

Follow `onecopy/line-placement`:
- a door takes `axis: geom::Line<T>`;
- an arm that destructures a stored surface builds the line from its own bindings until stage 3 (`analytic-surfaces-spell-their-frame-field-by-field`) gives each kind a `Frame`, and then reads `kind.frame.axis_line()`;
- a holder that also carries `u_ref` is a frame, not a line, and waits on stage 3.
