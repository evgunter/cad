---
id: the-approximation-refit-solves-banded-normal-equations-densely
kind: issue
title: geom fit: approximate's least-squares refit builds dense rows and solves banded normal equations by dense Cholesky
status: open
opened: 2026-10-04
priority: P4
cost: M
refs: [the-interpolating-fit-solves-a-banded-collocation-system-densely]
---


(SSI implementer on `ssi/banded-fit`, 2026-10-04: the sibling the
banded-collocation unit's sweep found. Same class, a different system.)

The final least-squares refit of `approximate_core`
(`crates/geom/src/curves/fit.rs`, the `rational_row` loop ahead of
`lsq::solve_normal`) writes each data row as a dense `n_ctrl − 2`
vector holding `p + 1` nonzeros and hands it to `solve_normal`, which
forms `AᵀA` densely (`O(m·n²)`) and factors it by dense Cholesky
(`O(n³)`). `AᵀA` of a B-spline basis is banded with half-bandwidth
`p`, and so is its Cholesky factor.

Why it is P4: the refit runs on the structure knot removal selected, so
`n_ctrl` is the compacted count, and no hot path reaches it today —
`fit_branch` interpolates and does not approximate. It matters on the
day a consumer approximates thousands of samples (the comment in
`fit_branch` names compacting a carrier through
`approximate_with_params` as a later optimization).

Fix shape: a banded normal-equations assembly and a banded Cholesky
beside the fit's banded LU (`crates/geom/src/curves/banded.rs`).
Whether the bit-identity argument that module's doc makes carries over (dropped terms exact `0 · x`, no sum
starting at `−0.0`) is the first thing to check.

## Re-homed from FLUX to KNOT (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. KNOT collects the spline, net and fit doors in `geom-core` and `geom`: refinement inside an enclosure, the net-state reads, the NURBS derivative and projection doors, and the fit's refusals and solves. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.
