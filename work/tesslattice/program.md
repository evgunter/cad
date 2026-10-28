---
id: tesslattice
kind: program
title: TESSLATTICE — the NURBS lattice: its share of the budget, its rows and cells, and what it costs and certifies on swept and lofted walls
status: ready
opened: 2026-10-28
group: kernel/mesh
prefix: tesslattice/
tag: (TESSLATTICE orchestrator)
ab_band: 16200-16299
paths: [crates/mesh/src/nurbs_cert.rs, crates/mesh/src/trimmed.rs, crates/mesh/src/memo.rs]
keep_out: [opened 2026-10-28 by TESS's semantic-seam split when TESS measured 146 budget points against 30 - the rows moved by git mv with ids and bodies unchanged, crates/mesh is shared whole with TESS and its other four successors (TESSLIE TESSWALK TESSSIZE TESSLATTICE TESSWORD) and with CHORD, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, the tess-budget document and tools/tess-meter are INSTR's, patch_bound.rs is ENCL's, demos/tour pins are Track X conventions (re-pin in the moving PR with what moved and why), D9's byte-identical replay is ratified ground - a mesh byte that moves is re-baselined with the cause named and never weighed against the change]
---

The NURBS lane after the lattice metric (PR 5531) and the snap's retirement (PR 5551): `LATTICE_SHARE` chosen by measurement, the rational arm's reading pieces that force rows, cell grids that cost more than the whole-patch schedule, a helical wall far under its δ, a swept annulus over its certificate, and a chart swap that leaves a face nothing can re-derive.

Cut out of TESS (`work/tess/`) by its 2026-10-28 semantic-seam split.
TESS's review posture applies (`work/tess/plan.md`, Process). Slate:
`work/tesslattice/plan.md`; narrative in `work/tesslattice/log.md`.
