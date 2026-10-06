---
id: nurbs-lane-absence-has-three-spellings-across-the-doors
kind: issue
title: "this scalar holds no NURBS lane" is spelled three ways across transform, euler and replace_face (NurbsLaneUnsupported twice, RechartFalsifies { NurbsLaneNotSupplied } once)
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Filed by the `shell/lofted-wall-seam` lane (PR 4117), from its review's
Q1. One fact — `AtRestPolicy::nurbs_lane()` answers `None` at this
scalar (a dual, DL1) — reaches a caller in three shapes:

- `crates/topo/src/transform.rs`, `TransformError::NurbsLaneUnsupported { edge, scalar }`;
- `crates/topo/src/euler.rs`, `EulerOpError::NurbsLaneUnsupported { edge: Option<EdgeKey>, scalar }`;
- `crates/topo/src/replace_face.rs`: `ReplaceFaceError::NurbsLaneUnsupported { edge, scalar }`
  from the re-anchor (PR 4117 took transform's shape), but the SAME
  door's re-chart reports it as `RechartFalsifies { NurbsLaneNotSupplied }`
  (`geom_brep::CertifyError`, passed through without
  `policy_lane::ByPolicy`'s naming) — measured on the imported M7-8
  instance in `shell-refuses-every-lofted-body-at-a-wall-seam-carrier`'s
  PR-3720 table.

## Fix

Route `replace_face`'s re-chart refusal through `ByPolicy::NoLane` into
its own `NurbsLaneUnsupported`, and decide whether `euler`'s
`Option<EdgeKey>` is a real difference or the same shape. Every
`nurbs_lane()` consumer's error enum is the sweep.
