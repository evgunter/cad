---
id: topo-reexports-the-containment-door-under-two-public-paths
kind: issue
title: topo's public API spells the containment door twice since the component left boolean/: topo::boolean::{ContainError, point_in_solid, …} and topo::{…} / topo::solid_contain::…
status: open
opened: 2026-11-10
priority: P4
cost: E
---


Filed by the review of PR 6775 (S3), which moved boolean's containment
component to the crate root.

## What

To keep every external path resolving, the move left `boolean/mod.rs`
re-exporting the component's public items from their new home
(`crates/topo/src/boolean/mod.rs:216`, `:270`):

- `pub use crate::contain::{ArcEndGaps, CONTAINMENT_RAISED, ContainDecision, ContainError, FaceContainment, contfp, curved_face_containment, placement_ending, placement_lever, placement_subject, unnamed_placement_ending}`;
- `pub use crate::solid_contain::{PointInSolidError, SolidContainment, SolidFaces, point_in_solid, point_in_solid_faces, point_in_solid_of}`.

`lib.rs` re-exports the same items from `contain` and `solid_contain`
directly. So each has two public spellings, `topo::X` and
`topo::boolean::X`, and `topo::solid_contain::…` is a third address for
that module's public items. The old `topo::boolean::solid_contain::…`
module path is gone.

External readers of the `topo::boolean::` spelling:
- `crates/pncad/src/lib.rs`;
- `crates/pncad/tests/all.rs`;
- `crates/sweep/tests/{m5_pr9c_sphere_doors, m5_s12_curved_ops, common/orient, bool3_torus_doors, m5_s10_face_sense, band_planar_mitre, bool2_cone_doors}.rs`.

## The fix

Retire one spelling. The containment door is no longer the Boolean's, so
keep `topo::X`. Point the readers above at it and drop the two
`pub use` blocks in `boolean/mod.rs`. Decide whether
`topo::solid_contain` stays a `pub mod` or goes private behind the root
re-exports.
