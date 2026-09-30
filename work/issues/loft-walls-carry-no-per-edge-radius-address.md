---
id: loft-walls-carry-no-per-edge-radius-address
kind: issue
title: Lofted::side_faces and LoftGeometry group walls per loop per segment but reach no flow row, so a loft that declares a per-edge source has no address to attach it at
status: open
opened: 2026-09-17
priority: P1
cost: D
---

Disclosed by the `edit/chain-radius-attach` unit (PR 2804), whose
sweep swept for per-edge flow addresses and found this one correct by
vacuity. Filed here rather than on a program's slate because the two
sites straddle `sweep` and `verbs` and no program owns the pair.

## The finding

`edit/chain-radius-attach` made the sweeps' per-edge radius attach
address a wall by CANONICAL LOOP and CANONICAL SEGMENT, end to end:
`verbs`'s `Extruded::side_faces` and `Revolved::walls` keep both
indices, `editor_core::param_source::attach_swept` takes one token per
(loop, segment) and stamps `walls[i][j]` from `tokens[i][j]`, and a
mismatch between the two shapes is refused loudly.

The LOFT's walls have exactly the same two indices —
`sweep::loft::Lofted::side_faces` is `Vec<Vec<FaceKey>>` per loop per
segment, and `sweep::skin::LoftGeometry::walls` is `Vec<Vec<Arc<NurbsSurface<f64>>>>`
with the same grouping — and reach `attach_swept` from nowhere.
`Node::Loft` declares no per-edge flow row today
(`verbs::VerbKind::param_flow`; the row family is
`RoleFamily::SweptWalls`), so nothing is addressed and nothing is
mis-addressed. That is correct BY VACUITY, not by construction.

The day a loft declares a per-edge source, the address has to be the
one the attach already speaks — and the natural mistake at that site
is the one the sweeps had until this PR: flatten or group by loop
alone, and every wall after the first hole carries the wrong edge's
identity. A loft additionally has TWO section profiles, so which
profile's edge expression a wall is drawn from is a question the
sweeps do not have to answer at all.

## What a taker does

When a loft declares a per-edge flow row: keep both indices through
whatever the verb's record exports, make `attach_swept` the one door
that stamps it — it already refuses a shape mismatch — and settle in
the same change which section profile the per-edge expression is read
from.

## Citations

Accurate at PR 2804's head; the stable halves are
`sweep::loft::Lofted::side_faces` (`crates/sweep/src/loft.rs`),
`sweep::skin::LoftGeometry::walls` (`crates/sweep/src/skin.rs`),
`verbs::VerbKind::param_flow` and `verbs::RoleFamily::SweptWalls`
(`crates/verbs/src/flow.rs`), and
`editor_core::param_source::attach_swept`
(`crates/editor-core/src/param_source.rs`).
