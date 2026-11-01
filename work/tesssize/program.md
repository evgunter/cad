---
id: tesssize
kind: program
title: TESSSIZE — the curved and trimmed lanes' sizing: which density each lattice emits, and the theorem behind it
status: ready
opened: 2026-10-28
group: kernel/mesh
prefix: tesssize/
tag: (TESSSIZE orchestrator)
ab_band: 16100-16199
paths: [crates/mesh/src/sizing.rs, crates/mesh/src/curved.rs, crates/mesh/src/trimmed.rs, crates/mesh/src/chords.rs]
keep_out: [opened 2026-10-28 by TESS's semantic-seam split when TESS measured 146 budget points against 30 - the rows moved by git mv with ids and bodies unchanged, crates/mesh is shared whole with TESS and its other four successors (TESSLIE TESSWALK TESSSIZE TESSLATTICE TESSWORD) and with CHORD, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, the tess-budget document and tools/tess-meter are INSTR's, patch_bound.rs is ENCL's, demos/tour pins are Track X conventions (re-pin in the moving PR with what moved and why), D9's byte-identical replay is ratified ground - a mesh byte that moves is re-baselined with the cause named and never weighed against the change]
---

Rows about the density the analytic lanes emit and what proves it is enough: the one-element grid axes that drop the other axis' schedule (P0), the lanes that size without a worst-triangle theorem (trimmed cylinder, a cone with a split side, a thin torus past the aspect lemma), a split rim's sub-arc joints, one δ for a whole body, and the twopeg triangle growth the nightly gate reads.

Cut out of TESS (`work/tess/`) by its 2026-10-28 semantic-seam split.
TESS's review posture applies (`work/tess/plan.md`, Process). Slate:
`work/tesssize/plan.md`; narrative in `work/tesssize/log.md`.
