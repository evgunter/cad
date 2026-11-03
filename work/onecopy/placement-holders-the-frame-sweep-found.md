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

- **Found by the review's type sweep** (any `Point3` field beside any `Vec3`/`UnitVec3` field, names ignored, struct-like enum variants included):
  - `topo/src/boolean/region.rs` `Round` and `topo/src/boolean/circle_torus.rs` `ParallelPose`: circles, fixed in PR 6463 (each now holds a `geom::Circle`).
  - `topo/src/boolean/boxes.rs` `FaceBoxRule::{TorusWindow, CylinderSlab, ConeSlab}`: a box rule's kind data, cause 1. They go with the kind structs (stage 3).
  - `topo/src/boolean/ops.rs` `SphereCutIn`: a sphere and a cut plane, cause 1, stage 3.
  - `topo/src/boolean/join.rs` `SectionLoop`: a section circle's plane and centre. It is a conic reading, so it should hold the section's `Curve3` or a `geom::Circle`; this row.
  - `topo/src/ring_path.rs` `SectionArc` and `LoopSplit::Radius`/`Side`: a section plane with a point and a direction. These are seamless planes and an axis line, so this row's seamless-plane question.
  - `topo/src/boolean/solid_contain.rs` `FaceGeo::{PartialCone, PartialWall, TorusPatch}`: a face's carrier fields, cause 1, stage 3.
  - geom-brep `props/curved.rs` `SeamLoops`, `pair_section/torus_coincidence.rs` `TorusData`: a torus's fields, cause 1, stage 3.
  - geom-brep `mapped.rs` `MappedSource::RevolvedPoint`: a revolve's axis (an axis line), so it should hold `geom::Line`; this row.
  - `topo/src/shell.rs` `EdgeArc::Conic`: a conic edge's fields, which should be read through `Curve3::conic`; this row.
  - sweep `blend/reach.rs` `Bound::{Sheet, Tube}`: a sheet's plane and a tube's spine; seamless plane and axis line, this row.

Blind spot of the field grep: it matched a placement-point field by name, so it missed holders whose fields are named otherwise (`c`, `o`, `n`, `a`, `e1`, `e2`) and struct-like enum variants whose header it did not read as a type. Those are the review's list above, which swept by type. It also misses a tuple (`plane-and-line-tuples-restate-the-placement`) and a struct split over more than eight lines between its point and its axis. The pass read eight lines either side of every point-typed field named for a placement point; a second pass on single-letter point fields (`c: Point3`, `o: Point3`) beside a `Vec3` finds the pair-section poses (`PlaneCylinder { q, n, o, a, r }`), which are a pose's decided data rather than a carrier's.

Fix: with the kind structs (`analytic-surfaces-spell-their-frame-field-by-field`), each holds the kind's struct, a `Frame` or a `geom::Line` beside what it adds.
