---
id: tesssize
kind: program
title: TESSSIZE — the lanes' sizing: which density each lattice emits, the theorem behind it, and the NURBS lattice's share, rows and cells
status: ready
opened: 2026-10-28
group: kernel/mesh
prefix: tesssize/
tag: (TESSSIZE orchestrator)
ab_band: 16100-16199
paths: [crates/mesh/src/sizing.rs, crates/mesh/src/curved.rs, crates/mesh/src/trimmed.rs, crates/mesh/src/chords.rs, crates/mesh/src/nurbs_cert.rs, crates/mesh/src/memo.rs]
keep_out: [opened 2026-10-28 by TESS's semantic-seam split when TESS measured 146 budget points against 30 - the rows moved by git mv with ids and bodies unchanged, crates/mesh is shared whole with TESS and its other four successors (TESSLIE TESSWALK TESSSIZE TESSLATTICE TESSWORD) and with CHORD, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, the tess-budget document and tools/tess-meter are INSTR's, patch_bound.rs is ENCL's, demos/tour pins are Track X conventions (re-pin in the moving PR with what moved and why), D9's byte-identical replay is ratified ground - a mesh byte that moves is re-baselined with the cause named and never weighed against the change, TESSLATTICE merged in on 2026-11-02 (docs/doc-ledger/tesslattice-leaves-the-tracker.md)]
---

Rows about the density the analytic lanes emit and what proves it is enough: the one-element grid axes that drop the other axis' schedule (P0), the lanes that size without a worst-triangle theorem (trimmed cylinder, a cone with a split side, a thin torus past the aspect lemma), a split rim's sub-arc joints, one δ for a whole body, and the twopeg triangle growth the nightly gate reads.

Cut out of TESS (`work/tess/`) by its 2026-10-28 semantic-seam split.
TESS's review posture applies (`work/tess/plan.md`, Process). Slate:
`work/tesssize/plan.md`; narrative in `work/tesssize/log.md`.

## Merged in 2026-11-02

The NURBS lattice's share, rows and cells are the same question as the analytic lanes' sizing: what density a lane emits and what proves it is enough. The track budget now counts only dispatchable P0, P1 and unbanded rows (`work/README.md`, Track size), so these tracks had fallen far under it; their rows moved here by `git mv` with ids and bodies unchanged.

**From TESSLATTICE** (TESSLATTICE — the NURBS lattice: its share of the budget, its rows and cells, and what it costs and certifies on swept and lofted walls):

The NURBS lane after the lattice metric (PR 5531) and the snap's retirement (PR 5551): `LATTICE_SHARE` chosen by measurement, the rational arm's reading pieces that force rows, cell grids that cost more than the whole-patch schedule, a helical wall far under its δ, a swept annulus over its certificate, and a chart swap that leaves a face nothing can re-derive.
