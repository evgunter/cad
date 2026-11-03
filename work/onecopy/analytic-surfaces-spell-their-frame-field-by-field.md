---
id: analytic-surfaces-spell-their-frame-field-by-field
kind: issue
title: Plane, Cylinder, Cone (and sphere, torus) each spell the placement (point, axis, u_ref) field by field, beside the geom::Frame the conics hold
status: parked
opened: 2026-11-02
priority: P1
cost: H
blocked_on: [the-frame-carries-every-placement-reading, 6371, 6443]
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

**Sites that rebuild a carrier's frame from its fields and wait for the kind structs** (found by PR 6463's review, which repointed the readers outside `geom` at `Surface::frame()`/`Curve3::frame()`):
- `geom::Surface::eval` and `jet`, whose azimuth arms build `Frame::new(origin, axis, u_ref)`;
- `geom::cover::readings`' sphere, torus and spiric arms;
- `Curve3`'s spiric evaluator arms (`azimuth::basis(&Frame::new(..))`);
- step-export's writer arms (`axis2_placement(&Frame::new(..))`);
- `topo::boolean::section_cert::Cone::new` (a loose-argument door);
- geom-brep `pair_section/torus.rs` `Ring` (`Frame::new(self.center, self.axis, self.u_ref)` three times);
- `pair_section.rs` `CylinderSpherePose`.
