---
id: mesh-materialized-form-is-writable
kind: issue
title: mesh::Mesh (with FacePatch and BoundaryPolyline) has all-public fields — any downstream crate can hand-build a mesh whose watertightness contract is false
status: open
opened: 2026-09-08
refs: [2134]
priority: P3
cost: D
---

Found by BOOL-9's class sweep (PR 2134 §7) and filed by the S-MESH
orchestrator: the tessellate → mesh seam's materialized form is writable
from outside the crate, the same shape as the `RawLoop` door BOOL-9
shuts. The type's own docs say `validate::check_mesh` re-derives the
contract and that `tessellate` does not run it, so a hand-built `Mesh`
is a value nothing certified. The other seams the sweep read are sealed
(`topo::Body`, `ValidatedProfile`, `editor-core::Program`) or checked at
construction (`geom::NurbsCurve3::new`).

What a unit decides: whether `Mesh` becomes a sealed type minted by
`tessellate` (and a dev-only door for the fixture writers, per the
BOOL-9 / LoopBuilder→`test_support` precedent — survey the writers
first, as BOOL-9 did), or whether the materialized form stays open with
`check_mesh` as the only contract and the docs say so. The STL / STEP
export lanes and the viewer read `Mesh` and are the consumers to keep
bit-identical. Difficulty M (survey-first). Not scheduled.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.
