---
id: placement-and-part-forwarders-the-census-does-not-weld
kind: issue
title: The part, transform, motion and face-frame refusals NodeErrorKind forwards are not welded to their arms in the feature-tree census
status: open
opened: 2026-10-20
priority: P4
cost: E
---


Found by `wire-census2`, whose PR welds the feature-tree census
(`every_forwarded_arm_has_a_row`, `crates/editor-core/tests/refusal_concision_chains.rs:1389`) to the kernel
and editor-core refusals `NodeErrorKind` forwards. `forward` (`crates/editor-core/tests/refusal_concision_chains.rs:1190`) cites this
row (`UNWELDED_PLACEMENT`) for five arms on placement, datum and part
ground:

- `Part` forwards `parts::PartFault` (nine arms,
  `crates/editor-core/src/eval/parts.rs:191`). `document_arms`' `parts`
  list (`crates/editor-core/tests/refusal_concision_chains.rs:4919`) renders seven; `PartProduct` is in `part_products` (`crates/editor-core/tests/refusal_concision_chains.rs:5229`), and
  `Unresolved` is in the viewer's `instance_authoring.rs`, as the
  comment above the list says. The arms are covered across three
  lists that no roster ties.
- `Transform` forwards `topo::TransformError` (nine arms,
  `crates/topo/src/transform.rs:114`), rendered by `transform()`
  (`crates/editor-core/tests/refusal_concision_chains.rs:2890`), with `ApproxRecertify` through `offset_fit_routes`.
- `Motion` forwards `geom_core::RigidMotionError` (three arms,
  `crates/geom-core/src/linalg/rigid_motion.rs:301`), rendered by
  `motion()` (`crates/editor-core/tests/refusal_concision_chains.rs:2931`).
- `FaceFrameReadback` forwards `topo::readback::ReadbackError` (three
  arms, `crates/topo/src/readback.rs:142`, each with a row).
- `MemberRefused` forwards the `NodeErrorKind` a family's member refused with (`crates/editor-core/src/eval/mod.rs`), one level of the same roster, unwelded as a recursive forward.
- `FrameDirection` forwards `DirectionRefusal`
  (`crates/editor-core/src/eval/wire.rs:1504`).

## Parked on the D10 hold (2026-10-20, WIRE)

Transform, motion and face frames are placement and datum ground, and
`eval/parts.rs` the instance seam D10 stage 3 rebuilds, so
`wire-census2` listed these in `forward` and did not weld them. Parked
on `transform-retires-into-a-placement`, the row whose landing releases
that ground; re-read the arms against the tree when it fires, since
the transform arm is expected to be moot.

## Repair shape

Give each refusal an `f6_variants!` roster in the census's `kernel`
module (or beside the editor-core ones), a row per arm labelled
`{family}/{arm}`, and move its `forward` arm to a welded level. For
`PartFault`, weld over every list that renders it, or name in-file the
arms that render only in the viewer.

## Re-checked after D10 (2026-11-01, WIRE orchestrator)

From the read-only `wirehold-triage` against origin/main 8509df2510. INTENT closed on 2026-10-30, and every trigger this row waited on has fired.

**Changed.**
- `FaceFrameReadback` and `FrameDirection` are gone from `NodeErrorKind`.
- Still unwelded at `refusal_concision_chains.rs:1252-1256`: `Part`, `MemberRefused`, `Transform`, `Mirror` (new, `eval/mod.rs:1513`) and `Motion`. `Transform`/`Mirror` forward `topo::TransformError` (`topo/src/transform.rs:123`).
- Same path hazard as its sibling. One unit with `mate-and-pose-forwarders-the-census-does-not-weld`.
