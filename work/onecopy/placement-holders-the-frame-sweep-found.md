---
id: placement-holders-the-frame-sweep-found
kind: issue
title: Types holding a carrier's placement field by field, found by the frame-readings sweep, owed to the kind-struct stage
status: open
opened: 2026-11-03
priority: P2
cost: M
refs: [analytic-surfaces-spell-their-frame-field-by-field]
---


Found by the second pass of the frame-readings unit's sweep (`the-frame-carries-every-placement-reading`): a type whose fields hold a point beside an axis (`center`/`centre`/`origin`/`apex`/`anchor`/`base` with `axis`/`normal`/`dir`, and often `u_ref`), the field-shape grep the placement decision keeps as its sweep's second pass. The unit retired the copies its scope named (`surgery::CircleFrame`, `ring_path::CircleArc`, `LoopArc::Line`, `contain::LoopCircle`, `ChartInverse`'s fields, `topo::Chart`'s accessors, `chart_region::CylFrame`, step-import's tuple, `TangentLocus::Line`, `RuledSection::TangentLine`, `DecidedLine`, `gap::Spine::Line` and `gap::Core`, `depth_gap::Depth::Axis`); these remain, each with the cause the decision names (cause 1: it memoizes a `Surface` match because the kind has no type; cause 2: a door took loose vectors):

- **A full frame (`u_ref` beside the point and axis), cause 1, so the kind structs retire them:** `geom-brep/src/pair_section.rs` `CylinderSpherePose`; `geom-brep/src/pair_section/torus.rs` `Ring`; `geom-brep/src/ssi/boundary.rs` `PassPlane`; `topo/src/boolean/section_cert.rs` `Cone`; `topo/src/boolean/torus_region.rs` `TorusFaceRegion`; `topo/src/chord_join.rs` `WallChart`; `topo/src/sector_face.rs` `ConeApex`; `topo/src/shell.rs` `PlanarFace`; `topo/src/boolean/ruled_region.rs` `Ruled::{Cylinder, Cone}`; `step-import/src/normalize/singularity.rs` `Sphere`/`Cone`; `topo/src/test_support_fixtures.rs` `CylFrame` (a fixture's spelling of a cylinder's datum).
- **A conic's frame, held loose:** `topo/src/ring_path.rs` `Oval` and `topo/src/splitting/spiric_arc.rs` `Oval` (a spiric's torus frame: the spiric row, stage 3); `topo/src/validate.rs` `Arc`; `topo/src/boolean/rim_wedge.rs` `Rim` (waits on PR 6443, `circle-frame-structs-re-spell-geom-circle`).
- **An axis line (a point and a direction, no seam):** `sweep/src/blend/arms.rs` `Meridian`; `geom-brep/src/chart_singularity.rs` `Shape` (`anchor`, `axis`); `geom-brep/src/offset.rs` `ConeOffset`; `topo/src/offset_axial.rs` `Frame`; `sweep/src/swept.rs` `Turn`; `topo/src/boolean/join.rs` `Turn`; `geom-brep/src/pair_section/ruling.rs` `RuledCylinders`; `topo/src/boolean/surface_group.rs` `WrapRims`; `topo/src/census/depth_gap.rs` `Ring`; `geom/src/cover.rs` `Slab`/`Frustum` (a unit-axis solid, possibly legitimately its own).
- **A circle with no seam (centre, axis, radius):** `geom-brep/src/pair_section/gap.rs` `Spine::Circle`, `geom-brep/src/pair_section/winds.rs` `Core`, `topo/src/census/depth_gap.rs` `Depth::Core`, `topo/src/boolean/contain.rs` `TorusCarrier`, `topo/src/boolean/ops.rs` `SphereRecut`: a narrower reading than `geom::Circle` (no seam is read), which is a decision for each.
- **A plane (point and normal):** `geom-brep/src/intersect.rs` `PlaneSite`, `topo/src/boolean/plane_eq.rs` `PlaneDesc`, `topo/src/splitting/mod.rs` `SplitPlane`, `topo/src/vertex_section.rs` `SectionPlane`, `topo/src/shell.rs` `InPlane`, `topo/src/census.rs` `FaceGeo`: these name a plane with no seam, so `Frame` is not their home; whether a seamless plane type is wanted is the stage-3 implementer's question.
- **Also named on the parent row and not taken here:** `ring_path::LoopArc::Crest.core`, `mesh::planar::ChartFrame`, `topo::readback::Pose`, `separating::Circle`, revolve's `wall_surface`.

Blind spot of the field grep: a holder whose fields are named otherwise (`c`, `o`, `n`, `a`), a tuple (`plane-and-line-tuples-restate-the-placement`), and a struct split over more than eight lines between its point and its axis. The pass read eight lines either side of every point-typed field named for a placement point; a second pass on single-letter point fields (`c: Point3`, `o: Point3`) beside a `Vec3` finds the pair-section poses (`PlaneCylinder { q, n, o, a, r }`), which are a pose's decided data rather than a carrier's.

Fix: with the kind structs (`analytic-surfaces-spell-their-frame-field-by-field`), each holds the kind's struct, a `Frame` or a `geom::Line` beside what it adds.
