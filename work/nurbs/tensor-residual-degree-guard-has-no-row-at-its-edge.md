---
id: tensor-residual-degree-guard-has-no-row-at-its-edge
kind: issue
title: The tensor residual's exact-binomial guard survives a +2 mutant across geom-core
status: open
opened: 2026-10-20
---


**P3.** Found by #5414's FULL review, as a NOTE (https://github.com/evgunter/cad/pull/5414). The finding predates that PR.

**What.** The tensor residual's guard refuses a cell whose surface degrees overflow the exact binomials: `if mu + mv > super::BINOM_EXACT_MAX` (`crates/geom-core/src/spline/compose/tensor.rs:429`). The comment above it says this is the one budget case the automatic NaN row can't reach: a degree-0 curve pair whose `C(m_u,i)·C(m_v,j)` still could overflow. Weakening it to `BINOM_EXACT_MAX + 2` survives the whole of geom-core, so no row reads a degree-0 curve pair at or just past the cap.

**Done when** a row composes a degree-0 curve pair onto a surface with `mu + mv` equal to `BINOM_EXACT_MAX` and to `BINOM_EXACT_MAX + 1`: served at the first, refused at the second. The +1 and +2 mutants must both go red.
