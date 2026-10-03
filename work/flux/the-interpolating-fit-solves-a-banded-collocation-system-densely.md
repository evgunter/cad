---
id: the-interpolating-fit-solves-a-banded-collocation-system-densely
kind: issue
title: geom fit: interpolate_with_params solves its banded collocation system by dense O(n³) LU, and fit_branch solves the same matrix three times; refinement by certification refits per round
status: open
opened: 2026-10-03
priority: P2
cost: M
---

(SSI implementer on `ssi/step-max-certify`, from measuring the cost of
refinement by certification, 2026-10-03. Filed here because the solver
is flux's ground: `work.py territory` gives `crates/geom/src/curves/fit.rs`
and `crates/geom-core/src/linalg/lsq.rs` to flux.)

## What

`NurbsCurve3::interpolate_with_params` and its 2-D twin
(`crates/geom/src/curves/fit.rs`, the macro body near `solve_square`'s
call) build the square collocation matrix of a degree-`p` interpolation
and hand it to `geom_core::linalg::lsq::solve_square`, a dense
fixed-order Doolittle LU: O(n³) in the sample count. The matrix is
banded (each row has at most `p + 1` nonzeros), and a no-pivot LU of a
banded matrix keeps its fill in the band, so every product it forms
outside the band is an exact `0 · x`.

The SSI fit (`fit_branch` in `crates/geom-brep/src/ssi.rs`) interpolates
the carrier and both pcurves on one shared parameter vector, so it
factors the same matrix three times.

## Why it matters now

Refinement by certification (`march::refine_by_certificate`) refits the
branch every round. Measured in the debug profile on the curved dome's
cuts at ε 1e-9: one round at n ≈ 600 samples spends 2.5–3.8 s in the
fit and 0.8–1.2 s in the certificate. A branch near the fit budget
(n ≈ 1000, the centre-weight-9 rational wall at ε 1e-12) refines for
up to 18 rounds, and `m5_pr7_ssi.rs`'s
`a_rational_walls_plane_three_eps_off_its_edge_meets_the_certificate_limit`
takes about nine minutes at that ε.

## Fix shape

A banded solve (or skipping the structural zeros in the existing
Doolittle, which forms the same nonzero products in the same order) and
one factorization shared by the three right-hand sides. Whether the
banded form reproduces today's bits is the first thing to check; the
argument above says it does where every off-band entry is an exact zero
and every in-band product is finite.

