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
paths: [crates/mesh/src/trimmed.rs, crates/mesh/src/planar.rs, crates/mesh/src/tessellate.rs, crates/mesh/src/validate.rs, crates/mesh/src/walk.rs, crates/mesh/src/chords.rs, crates/mesh/src/nurbs_cert.rs, crates/mesh/src/types.rs, crates/mesh/src/lib.rs, crates/mesh/README.md]
keep_out: [opened 2026-10-28 by TESS's semantic-seam split when TESS measured 146 budget points against 30 - the rows moved by git mv with ids and bodies unchanged, crates/mesh is shared whole with TESS and its other four successors (TESSLIE TESSWALK TESSSIZE TESSLATTICE TESSWORD) and with CHORD, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, the tess-budget document and tools/tess-meter are INSTR's, patch_bound.rs is ENCL's, demos/tour pins are Track X conventions (re-pin in the moving PR with what moved and why), D9's byte-identical replay is ratified ground - a mesh byte that moves is re-baselined with the cause named and never weighed against the change, TESSWALK merged in on 2026-11-20 (docs/doc-ledger/tesswalk-and-tessword-merge-into-tesslie.md) - it held no P0 or P1 row, from TESSWALK: opened 2026-10-28 by TESS's semantic-seam split when TESS measured 146 budget points against 30 - the rows moved by git mv with ids and bodies unchanged, from TESSWALK: crates/mesh is shared whole with TESS and its other four successors (TESSLIE TESSWALK TESSSIZE TESSLATTICE TESSWORD) and with CHORD, from TESSWALK: so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, from TESSWALK: the tess-budget document and tools/tess-meter are INSTR's, from TESSWALK: patch_bound.rs is ENCL's, from TESSWALK: demos/tour pins are Track X conventions (re-pin in the moving PR with what moved and why), from TESSWALK: D9's byte-identical replay is ratified ground - a mesh byte that moves is re-baselined with the cause named and never weighed against the change, TESSWORD merged in on 2026-11-20 (docs/doc-ledger/tesswalk-and-tessword-merge-into-tesslie.md) - it held no P0 or P1 row, from TESSWORD: opened 2026-10-28 by TESS's semantic-seam split when TESS measured 146 budget points against 30 - the rows moved by git mv with ids and bodies unchanged, from TESSWORD: crates/mesh is shared whole with TESS and its other four successors (TESSLIE TESSWALK TESSSIZE TESSLATTICE TESSWORD) and with CHORD, from TESSWORD: so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, from TESSWORD: the tess-budget document and tools/tess-meter are INSTR's, from TESSWORD: patch_bound.rs is ENCL's, from TESSWORD: demos/tour pins are Track X conventions (re-pin in the moving PR with what moved and why), from TESSWORD: D9's byte-identical replay is ratified ground - a mesh byte that moves is re-baselined with the cause named and never weighed against the change]
---

Every row here is a body the mesh gets WRONG without saying so, or panics on: `tessellate` returns Ok on a mesh `check_mesh` refuses (four uses of a pseudomanifold edge's segment, a steep section of a one-face wall), meshes short (a tilted plate across a held pierce), answers an empty patch, or reaches a debug assertion its comment calls vacuous. The chart-coherence detector's Off default came with them as the body-side half of the same question: what stands between malformed input and a confident mesh.

Cut out of TESS (`work/tesslane/`) by its 2026-10-28 semantic-seam split.
TESS's review posture applies (`work/tesslane/plan.md`, Process). Slate:
`work/tesslie/plan.md`; narrative in `work/tesslie/log.md`.

## From TESSWALK (merged 2026-11-20)

TESSWALK — what the walk and the lanes decide: structural premises without a body-level witness, and readings taken off undecided numbers

The rows where a decision in the mesh rests on the wrong thing. Some are structural premises (the branch premise's cone arm, the rim-side unanimity premise, a half-edge read as its edge's minus half, two coincident edges read as two columns) that no body reaches or that a door upstream now makes. The rest decide off numbers that are not decided: the pole-junction unwrap's branch, the planar chart's `u_ref` rejection, a quad diagonal chosen on last-bit UV noise, a certificate rounded to nearest, a zero arm that `next_up` makes dead.

Cut out of TESS (`work/tesslane/`) by its 2026-10-28 semantic-seam split.
TESS's review posture applies (`work/tesslane/plan.md`, Process). Slate:
`work/tesslie/plan.md`; narrative in `docs/doc-ledger/tesswalk-and-tessword-merge-into-tesslie.md`.

## From TESSWORD (merged 2026-11-20)

TESSWORD — tessellation refusal words, mesh docs and contract hygiene

What the mesh says rather than what it does: refusals too long for the viewer, a refusal spelled five times, `MissingEntity` covering four causes, kernel-bug wording on a case not yet built, whether a faceless body refuses at the door, and docs and rows that describe a tree that moved.

Cut out of TESS (`work/tesslane/`) by its 2026-10-28 semantic-seam split.
TESS's review posture applies (`work/tesslane/plan.md`, Process). Slate:
`work/tesslie/plan.md`; narrative in `docs/doc-ledger/tesswalk-and-tessword-merge-into-tesslie.md`.
