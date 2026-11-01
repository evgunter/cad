---
id: tensor-residual-degree-guard-has-no-row-at-its-edge
kind: issue
title: The tensor residual's exact-binomial guard survives a +2 mutant across geom-core
status: closed
opened: 2026-10-20
closed: 2026-10-20
---


**P3.** Found by #5414's FULL review, as a NOTE (https://github.com/evgunter/cad/pull/5414). The finding predates that PR.

**What.** The tensor residual's guard refuses a cell whose surface degrees overflow the exact binomials: `if mu + mv > super::BINOM_EXACT_MAX` (`crates/geom-core/src/spline/compose/tensor.rs:429`). The comment above it says this is the one budget case the automatic NaN row can't reach: a degree-0 curve pair whose `C(m_u,i)·C(m_v,j)` still could overflow. Weakening it to `BINOM_EXACT_MAX + 2` survives the whole of geom-core, so no row reads a degree-0 curve pair at or just past the cap.

**Done when** a row composes a degree-0 curve pair onto a surface with `mu + mv` equal to `BINOM_EXACT_MAX` and to `BINOM_EXACT_MAX + 1`: served at the first, refused at the second. The +1 and +2 mutants must both go red.

**Closed (2026-10-20)** by the row
`a_degree_zero_pair_is_served_at_the_binomial_cap_and_refused_past_it`
in `tensor.rs`'s tests. It reaches `cell_residual` directly with
degree-0 rows because `KnotVector` refuses degree 0, so no public
composition gets to the guard. `28 + 28` is served and `28 + 29`
refused, and the row goes red on the guard's `+ 1` and `+ 2` mutants.
