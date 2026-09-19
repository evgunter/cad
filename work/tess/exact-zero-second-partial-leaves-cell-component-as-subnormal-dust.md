---
id: exact-zero-second-partial-leaves-cell-component-as-subnormal-dust
kind: issue
title: A degree-1 direction's exactly-zero second partial reaches cell_component as 1.5e-323, so the == 0.0 arm split_steps decides on is dead
status: open
opened: 2026-09-18
---


Found by TESS's diagnostic lane on the NURBS face bound
(`tess/nurbs-bound-diag` at `db4cb45a8`), measured there and confirmed
by the orchestrator's read of the three functions.

## The defect

`nurbs_cert::cell_component`'s doc: *"An exactly-zero enclosure
collapses to exactly `0.0` — … load-bearing: the split selection's
degenerate-direction predicates (`NurbsFaceBound::split_steps`) are
decided on `== 0.0`, so the structurally-exact zero of a degree-1
direction must not leave here as subnormal dust."* It leaves as
subnormal dust, one step EARLIER:

- `geom_brep::patch_bound::sq_norm` folds `acc + c.sqr()` from the ring
  zero;
- `RingInterval`'s `Add` widens unconditionally — `up1(self.hi +
  rhs.hi)` — so `0 + 0` has `hi` = the least subnormal, and three
  exact-zero channels give `hi = 1.5e-323`;
- `cell_component` then takes the `else` arm: `√1.5e-323` rounded up.

**Measured**: `muu = mvv = 3.85e-162` on a bilinear integral patch,
where `NurbsFaceBound::muu`'s doc promises "EXACTLY 0.0". So
`cell_component`'s `hi == 0.0` arm is unreachable through `sq_norm`,
and `split_steps`' `== 0.0` degenerate arms are not taken for the
faces they were written for (the second half from reading; what
`split_steps` DOES answer for such a face was not measured — 3.85e-162
may land on the same steps by arithmetic, which is the first thing the
unit measures).

## Why no test sees it

`planar_bilinear_bounds_collapse` asserts `< 1e-100`, and its own
doc-comment says the exact zero is preserved. A row that would go red:
`assert_eq!(b.muu, 0.0)` on that fixture.

## Whose

The promise and its consumers are `mesh`'s; `sq_norm` and
`RingInterval::add` are PROPS' (`patch_bound.rs`, `geom-core`). The
unit decides where the zero is kept: an exact-zero short-circuit in the
ring's `add` is a `geom-core` change with every interval consumer
behind it (PROPS' call, a design question about whether the ring may
know an exact zero); a `sq_norm` that skips exact-zero channels is
local to `patch_bound`; deciding degeneracy in `mesh` from STRUCTURE
(degree 1, single span, integral) rather than from a float `== 0.0` is
inside this fence and is the never-infer doctrine's answer.

Adjacent, same lane, recorded here rather than separately:
`NurbsSurface::ders` returns ~1e-16 for `duu` on an integral degree-1
direction whose true value is 0, so a future INTEGRAL-arm domination
sweep would red against a near-zero bound on sampler noise
(`r1_random_rational_soundness_sweep` draws no integral surface today).
