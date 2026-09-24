---
id: exact-zero-second-partial-leaves-cell-component-as-subnormal-dust
kind: issue
title: A degree-1 direction's exactly-zero second partial reaches cell_component as 1.5e-323, so the == 0.0 arm split_steps decides on is dead
status: closed
opened: 2026-09-18
priority: P0
cost: D
parent: TESS-3
closed: 2026-09-22
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

## 2026-09-22 — measured, and closed by the ring

**The defect is gone, and SCALAR's RING-2 (PR 3032) closed it.** The
ring is now a newtype over the backend's decorated interval, which pads
only where an operation was inexact: `0 · 0` is exact by `mul_lo`/
`mul_hi`'s zero-factor corner convention and `0 + 0` by `add_lo`/
`add_hi`'s TwoSum witness. So `patch_bound::sq_norm`'s fold
`acc + c.sqr()` from `RingInterval::zero()` keeps `[0, 0]`,
`cell_component`'s `hi == 0.0` arm is reachable, and `split_steps`'
`== 0.0` arms are taken for the faces they were written for.

Measured on the merge base through `nurbs_face_bound`, at `{:.17e}`:

| fixture | `muu` | `muv` | `mvv` | `split_steps(1e-3)` |
| --- | --- | --- | --- | --- |
| planar bilinear, integral 1×1 | `0.00000000000000000e0` | `0.00000000000000000e0` | `0.00000000000000000e0` | `hu = hv = inf`, `cap = false` |
| integral 1×2 (bent in v) | `0.00000000000000000e0` | `0.00000000000000000e0` | `2.80000000000000027e0` | `hu/hv = ρ·16`, `cap = true` |
| integral 1×2, no 3-D extent in u | `0.00000000000000000e0` | `0.00000000000000000e0` | `2.80000000000000027e0` | `hu = hv = 1.88982236504613606e-2`, `cap = false` |
| twisted bilinear, integral 1×1 | `0.00000000000000000e0` | `5.00000000000000111e-1` | `0.00000000000000000e0` | `hu = hv`, `cap = false` |
| rational uniform-w bilinear | `2.84217094304100608e-13` | `9.06463181488535801e-12` | `2.84217094304097327e-13` | generic arm, finite |
| quarter cylinder (rational, deg 1 in v) | `3.03053888509198810e0` | `1.15257297607736328e-1` | `3.48537838053717359e-13` | generic arm, `cap = true` |

`3.85e-162` no longer appears anywhere. The rational arm's dust is
CORRECT and stays: a rational degree-1 direction's cross terms survive
in ℝ, and even where the weight column is constant along the direction
(both rational rows above) the ring chain reports the width its
refinement contributed rather than a zero it never proved.

Pinned, so the mechanism cannot come back silently:

- `planar_bilinear_bounds_collapse` now asserts `muu == 0.0 &&
  mvv == 0.0` instead of `< 1e-100`, and pins the AFFINE arm through
  `hu == hv == inf` with `cap` inactive — an observable no dust can
  produce. `muv` keeps its `< 1e-12` ceiling: its zero is `ΔΔP` exact
  in f64, not structural.
- `twisted_bilinear_mixed_term_is_tight`: the same two exact zeros.
- `one_sided_degenerate_arm_on_a_degree_one_direction` (new): the
  ruled-wall arm on a described face. The windowed variant cannot
  separate the arms (a dusty `muu` clamps to the same `t = ρ·16`), so
  the row carries a windowless variant where the arm's `t = 1`
  fallback is unmistakable.
- `rational_degree_one_direction_is_not_taken_as_degenerate` (new): the
  quarter cylinder's `mvv` bracketed in `[1e-14, 1e-11]`, i.e. strictly
  positive, which IS the claim that `split_steps`' `mvv == 0.0` arms
  are not entered.

The adjacent note is answered and needs nothing: the only integral-arm
domination row is `hessian_hull_dominates_sampled_second_partials` over
`wavy()`, which guards `muu > 0.0 && muv > 0.0 && mvv > 0.0` before it
samples, so no existing row meets a zero bound with `ders`' ~1e-16
`duu` noise. The rows added above compare the exact zero by the ARM
taken, never by a sample, which is the shape any future integral-arm
row on a degenerate direction has to take — `Domination`'s allowance is
relative (`SAMPLER_ULPS`), hence zero against a zero certified side.

Residue filed, not fixed:
`work/tess/chords-m-bound-zero-arm-is-dead-because-the-curve-collapse-has-no-exact-zero-case.md`
— `chords.rs` open-codes the same collapse twice without the
exact-zero case, so its own `m_bound == 0.0` fast path is dead.

Closed by TESS-3.
