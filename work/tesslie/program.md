---
id: tesslie
kind: program
title: TESSLIE — the mesh answers Ok, or panics, where it should refuse or mesh right: non-manifold meshes, short meshes, debug asserts that fire and empty patches
status: ready
opened: 2026-10-28
group: kernel/mesh
prefix: tesslie/
tag: (TESSLIE orchestrator)
ab_band: 15900-15999
paths: [crates/mesh/src/trimmed.rs, crates/mesh/src/planar.rs, crates/mesh/src/tessellate.rs, crates/mesh/src/validate.rs, crates/topo/src/coherence.rs]
keep_out: [opened 2026-10-28 by TESS's semantic-seam split when TESS measured 146 budget points against 30 - the rows moved by git mv with ids and bodies unchanged, crates/mesh is shared whole with TESS and its other four successors (TESSLIE TESSWALK TESSSIZE TESSLATTICE TESSWORD) and with CHORD, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, the tess-budget document and tools/tess-meter are INSTR's, patch_bound.rs is ENCL's, demos/tour pins are Track X conventions (re-pin in the moving PR with what moved and why), D9's byte-identical replay is ratified ground - a mesh byte that moves is re-baselined with the cause named and never weighed against the change]
---

Every row here is a body the mesh gets WRONG without saying so, or panics on: `tessellate` returns Ok on a mesh `check_mesh` refuses (four uses of a pseudomanifold edge's segment, a steep section of a one-face wall), meshes short (a tilted plate across a held pierce), answers an empty patch, or reaches a debug assertion its comment calls vacuous. The chart-coherence detector's Off default came with them as the body-side half of the same question: what stands between malformed input and a confident mesh.

Cut out of TESS (`work/tess/`) by its 2026-10-28 semantic-seam split.
TESS's review posture applies (`work/tess/plan.md`, Process). Slate:
`work/tesslie/plan.md`; narrative in `work/tesslie/log.md`.
