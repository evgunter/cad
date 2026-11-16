---
id: analytic-surfaces-spell-their-frame-field-by-field
kind: issue
title: Plane, Cylinder, Cone (and sphere, torus) each spell the placement (point, axis, u_ref) field by field, beside the geom::Frame the conics hold
status: parked
opened: 2026-11-02
priority: P1
cost: H
blocked_on: [6371]
---

Found by the ONEHOME orchestrator while adjudicating PR 6328's style review (2026-11-02).

The conic unit (`circle-and-ellipse-frame-arms-re-spell-conic-of`, branch `onehome/conic-frame`) introduces `geom::Frame`: a rigid placement, i.e. a point and an oriented orthonormal basis (`axis`, `u_ref`, `v_ref = axis × u_ref`), held by `Curve3::Circle` and `Curve3::Ellipse`. The analytic surfaces carry the same triple under their own names: `Surface::Plane { origin, normal, u_ref }`, `Cylinder { origin, axis, u_ref }`, `Cone { apex, axis, u_ref }`, and likely the sphere and torus. Under DESIGN `[one-way]` ("Two types carrying the same fields are a sign of a cleaner arrangement to find…", Ev, PR 6325), each should hold one `Frame`. Readers of the placement alone (azimuth, the seam, transforms, the axis line) would then read it there rather than per kind.

Parked until the conic unit lands and fixes `Frame`'s shape. Then sweep every surface kind and every reader that assembles the triple, including the surfaces' axis-line readings that PR 6328's review found built by hand as `geom::Line { origin, dir: axis }`.

## A further holder (PR 6349 review, 2026-11-02)

`geom::chart_point::ChartInverse { anchor, axis, u_ref, v_ref, arm }` holds the same placement triple in the same crate, plus a stored `v_ref` that `Frame::v_ref` derives. It belongs in this row's sweep: it holds a `geom::Frame`, or its readers read the surface's.

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.

## Decided (2026-11-03, three designers, agreed after two rounds)

This is weighed by the design-fork procedure; the draw is on `analysis/design-fork/onecopy-placement`. All three designers agree on the final state below and on its being an elaboration of `[one-way]` and the PR 6325 conic precedent, changing no ratified text. So it lands without an `[ev]` PR, with the full argument in the units' PR bodies.

**Why the copies arose.** The cause is not that the fields were in the wrong place. It is three things:
1. `Surface`'s kinds are struct-like variants. An anonymous payload is not a type, so nothing can hold, return or give methods to "a cylinder", and every reader that needs one beyond a match arm memoizes the match in a struct of its own (`CylFrame`, `CircleFrame`, `ChartInverse`, step-import's tuple).
2. The homes' own doors took loose vectors (`azimuth::frame(axis, u_ref, u)`, `Curve3::circle_at`, the cover constructors), so even a holder of a `Frame` took it apart.
3. `Frame` answered almost nothing (`in_plane` only), was documented as a conic's, and is a day old.

**Final state.**
- Each analytic surface kind is a named `geom` struct holding one `Frame` beside its shape: `Surface::Plane(Plane { frame })`, `Cylinder(Cylinder { frame, radius })`, `Cone(Cone { frame, half_angle })`, `Sphere(Sphere { frame, radius })` and `Torus(Torus { frame, major_radius, minor_radius })`.
- `Curve3::Spiric(Spiric { torus: Torus, offset })`. The spiric's docs say the held torus is the cut's own torus, seamed on the cutting plane's normal, and never the face's surface. Before landing, grep the spiric readers for any that compare the spiric's frame bit-for-bit with the face's surface or build its pcurves through it; one such reader would flip this to `{ frame, major_radius, minor_radius, offset }`.
- Every reading of a placement is a method on `Frame` (the stage-1 list on `the-frame-carries-every-placement-reading`).
- `RigidMotion` stays a distinct type, a proven motion certified at mint, against a carrier's stated frame certified at rest. They share one bridge.
- The data field is `origin`; there is no `Plane::origin()` and no other re-spelling of `frame.origin`. Per-kind name methods exist only where the name is a fact of the kind: `Cone::apex()`, and `center()` on the circle, sphere and torus. Refusal text keeps the kind's word. Whether `SurfaceDatum` keeps `Apex`/`Center` or merges them into `Origin` is the stage-3 implementer's call, after grepping the pinned refusal strings, provided the user-facing text keeps the kind's word (orchestrator's ruling; the designers called it low stakes).
- **One gate**: no struct-like variant in `Surface` or `Curve3`, under `scripts/gates/`, armed in the stage-3 PR. A field-shape gate was rejected, because `[one-way]` calls a shared field shape "a sign", not a ban, and every legitimate holder would need an allowlist entry. The field-shape grep stays the sweep's second pass.
- Rejected: a `frame` field inside the inline variants (cause 1 stays); one `Surface::Analytic { frame, shape }` (nested dispatch, the shape passed over on 6325); carriers holding `RigidMotion` (moves certification from rest to mint).

**Stages.**
1. The `Frame` module and readings, and the doors taking `&Frame`.
2. The consumer structs.

Stages 1 and 2 are `the-frame-carries-every-placement-reading`.

3. This row: the kind-struct sweep (~4,000 destructures, one PR), the spiric, and the gate. It runs after PR 6371 (709 files) and PR 6443 (`rim_wedge.rs`) land, and `Rim` becomes a `Circle` with it.

**Holders the original row missed, all in scope of stages 2–3:** `Curve3::Spiric`, `ring_path::LoopArc::Crest.core`, step-import's `type Frame`, `mesh::planar::ChartFrame`, `topo::readback::Pose`, `separating::Circle`, `cover::{Arc, Tube}` storing `v_ref`, `gap.rs`'s loose-argument door, step-export's `axis2_placement`, and revolve's `wall_surface`.

**Sites that rebuild a carrier's frame from its fields and wait for the kind structs** (PR 6463's final sweep for `Frame::new(` and `Circle::new(Frame::new(` outside test code; each is inside a match on a kind that stores no `Frame` yet, so it becomes `kind.frame` in the sweep):
- `geom::Surface::frame` itself and `Curve3::frame`'s spiric arm (the door, reading the loose fields);
- `geom::Surface::eval` and `jet`, whose azimuth arms build `Frame::new(origin, axis, u_ref)`;
- `geom::cover::readings`' sphere, torus and spiric arms;
- `Curve3`'s spiric evaluator arms (`azimuth::basis(&Frame::new(..))`);
- `topo::boolean::section_cert::Cone::new` (a loose-argument door whose second caller passes a turned axis, so it waits for the cone struct);
- geom-brep `pair_section/torus.rs` `Ring` (`Frame::new(self.center, self.axis, self.u_ref)`, once, in `Ring::frame`) and `topo::boolean::section_cert::square_search`'s ring (`Frame::new(ring.c, ring.a, ring.b1)`): torus holders;
- `pair_section.rs` `CylinderSpherePose`.

The other `Frame::new` sites outside test code mint a new frame rather than rebuild a carrier's: the constructors (`Curve3::circle`, `Curve3::ellipse`, `Round::new`), the squared core (`gap::Core::of`), a reflection's image (`topo::transform`), step-import's placements, ring_path's new arcs, and a section's shape ellipse at the origin (`sweep::blend::open::end_face`).

## Plane holders waiting on this stage (2026-11-09)

`plane-and-line-tuples-restate-the-placement` lists the doors that carry
a plane as a `(point, normal)` tuple, as loose arguments, or as struct
fields (`SectionPlane::site`, `FaceGeo::Plane`, `SplitPlane`,
`PlanarFace`, `SectionCtx`, …). Once `Plane { frame }` lands, they port
onto it. That row also asks whether a seamless plane needs a reading of
its own.

## Frame holders found by the axis sweep (2026-11-09, `onecopy/line-placement`)

The axis-line sweep found these holders. Each carries a full placement (a point, an axis and a seam), not just a line, so they were left for this stage's `Frame` rather than split into a `Line` plus a loose seam:
- `topo`: `boolean/ruled_region.rs` `Ruled`, `boolean/torus_region.rs` `TorusFaceRegion`, `chord_join.rs` `WallChart`, `boolean/section_cert.rs` `Cone`, `ring_path.rs` `Oval`;
- `geom-brep`: `pair_section/torus_arms.rs` `SphereMeridian`, `curve_face/circle_torus.rs` `roots_of`, `tangent.rs` `circle_span_bounds`;
- `sweep`: `test_support.rs` `tube_frame`.

`pair_section/torus.rs` `Ring` is not on this list. It has one door to its placement, `Ring::frame()`, and every axis reader in the file now reads `ring.frame().axis_line()` (PR 6665's review fix). So `Ring` waits on nothing here: its readers already hold the one spelling.

`geom/src/cover.rs`'s `Slab` and `Frustum` hold the axis as a `UnitVec3`. A `Line` would drop that unit typing, so they stay too.

## Axis lines built from an arm's bindings (2026-11-09, PR 6665)

PR 6665 gave the doors and holders below a `geom::Line`. Where the line came from a whole stored surface it was read through `Surface::axis_line()`, and where a `Frame` was in hand through `Frame::axis_line()`. The sites below instead build `geom::Line { origin, dir }` from the bindings of an arm that destructures a stored surface (or a carrier's or revolve's frame fields). Once each kind holds its `Frame`, each one becomes `kind.frame.axis_line()`, which leaves no second spelling of the axis. By file and function:
- `geom-brep/src/certify.rs`: `plane_nurbs_pair`, `tube_separation` ×2
- `geom-brep/src/chart_singularity.rs`: `singular_points` ×2
- `geom-brep/src/curve_face/circle_torus.rs`: `circle_torus_roots` ×3
- `geom-brep/src/curve_face/ellipse_torus.rs`: `torus_walk` ×4
- `geom-brep/src/implicit.rs`: `conic_form_sup` ×3, `conic_residual_harmonics` ×10, `implicit_residual`
- `geom-brep/src/mapped.rs`: `place_point`
- `geom-brep/src/offset.rs`: `offset_surface`
- `geom-brep/src/pair_section.rs`: `line_tilt`
- `geom-brep/src/pair_section/gap.rs`: `read`
- `geom-brep/src/pcurve_cache.rs`: `chart_distance`
- `geom-brep/src/pcurve_cache/projected.rs`: `projected_envelope`
- `geom-brep/src/props/curved.rs`: `agreed_side`
- `step-import/src/adopt.rs`: `mapped_self_description`
- `step-import/src/entities.rs`: `as_bool`
- `step-import/src/normalize/singularity.rs`: `meridian_rays`, `strut_tip`
- `sweep/src/blend/battery.rs`: `turn_reading` (the mitre plane's cylinder pair, for `line_meet`; `onecopy/line-holders`)
- `sweep/src/blend/reach.rs`: `circular_reach` (the torus band's axis, for `canonical_axis`; `onecopy/line-holders`)
- `sweep/src/revolve/full.rs`: `build_wire`
- `sweep/src/revolve/surfaces.rs`: `revolved_strut_spec`
- `sweep/src/revolve/turn.rs`: `sweep_turn`
- `topo/src/boolean/conic_quadric/mod.rs`: `cone_roots` ×2, `quadric_ladder`, `through_the_apex`
- `topo/src/boolean/contain.rs`: `curved_interior_placement` ×2
- `topo/src/boolean/join.rs`: `pt_germ_frame` ×2, `region_halves` ×5
- `topo/src/boolean/ops.rs`: `pinched_at_cone_apex`
- `topo/src/boolean/reduce.rs`: `conic_clearance`, `line_wall_roots_of` ×3, `requeue` ×6
- `topo/src/boolean/section_cert.rs`: `cone_cylinder`, `cone_pair` ×3, `torus_pair`
- `topo/src/boolean/section_cert/ruling.rs`: (closure), `assemble`
- `topo/src/boolean/solid_contain.rs`: `face_geo_arms` ×4, `root_outward`, `sphere_chart_trim` ×2, `unread_refusal` ×9
- `topo/src/boolean/surface_group.rs`: `wrap_rims`
- `topo/src/census/depth_gap.rs`: `across` (its `carrier_depth` ×2 left with main's BOXES redesign, which reads a carrier's site through `geom::cover`'s `Cover::about`)
- `topo/src/face_normal.rs`: `face_outward_normal_at` ×2, `face_outward_normal_at_foot`
- `topo/src/link.rs`: `Round::of` (the cylinder's axis, for `Round::cylinder`; `onecopy/line-holders`)
- `topo/src/offset_axial.rs`: `clamp_unit` ×3, `surface_residual`
- `topo/src/offset_nappe.rs`: `corner_stations`
- `topo/src/replace_face.rs`: `apex_shift`, `group_cone_v_window`, `offset_charts_staged` ×2, `transport_curve`
- `topo/src/ring_path.rs`: (closure) ×6
- `topo/src/sector_face.rs`: `resolve`
- `topo/src/transform.rs`: `map_mapped_curve`
- `topo/src/vertex_section.rs`: `read_kind`
