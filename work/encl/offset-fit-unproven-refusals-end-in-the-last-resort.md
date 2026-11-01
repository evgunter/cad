---
id: offset-fit-unproven-refusals-end-in-the-last-resort
kind: issue
title: geom-brep: OffsetFitError::Structure, Fit past its narrow-span arms, and MintLimb end in the last resort because neither input reaching them nor only a bug reaching them is shown
status: open
opened: 2026-10-20
parent: offset-fit-and-patch-bound-kernel-defect-endings-split-by-class
priority: P3
cost: M
---


(Filed by the ENCL implementer of PR 5476 at its review's request.)

## What

Three `OffsetFitError` refusals in `crates/geom-brep/src/offset_fit.rs`
end in `geom_core::KERNEL_LIMIT_RECOURSE` (`[tolerance.recourse]`'s
last resort) because neither `[failure]` class is shown for them. Each
is either reachable by input, and then owes its decision's lever, or
reachable only by a bug, and then owes a panic.

- **`Structure(_)`** (`interpolate_offset_grid`, `KnotVector::on_domain`
  and `NurbsSurface::new`): can the loop bisect sample parameters close
  enough to adjacent floats, on a domain whose width is comparable to
  its magnitude, that `on_domain`'s rounding collapses two averaged
  knots? Per-step rounding is about `width · 2⁻⁵³`, so it needs some ~50
  bisections at one place inside `OFFSET_FIT_WORK`. Evidence so far: a
  face on `[1e15, 1e15 + 1]` refuses earlier on the narrow span
  (`tests/offset_fit.rs` `a_knot_span_a_few_ulps_wide_refuses_on_its_span`),
  and a direct probe of `interpolate_columns` + `on_domain` over 20 000
  random parameter sets on the `[1e15, 1e15 + 8]` grid collapsed none.
  The `DomainInvalid` arm needs a domain whose width overflows.
- **`Fit(_)` past `TooFewPoints` and `ParamCountMismatch`**: `Lsq` (a
  collocation pivot that does not certify), `NonFinitePoint` (pass 2's
  solve overflowing into a non-finite row), `FitError::Structure`
  (averaged knots that rounding crowds). None reached in a sweep of far
  domains (lo 1e14 … 8e15, widths 1 … 64, degrees 2 and 3, 0 … 2
  interior knots, three d/tol pairs).
- **`MintLimb`**: the on-locus limb (`on_locus_cell`) is an `f64`
  residual on the model's coordinates, and the loop accepts on the
  certified hull alone (`fit_offset_at`, `achieved <= tolerance`), so a
  sample's rounding above a hull just under the tolerance is not
  excluded. The NaN arm (`nan_residual_at`) needs a fit solved from
  finite samples to overflow. Not reached by a quarter cylinder shifted
  1e3 … 1e9 at tolerances 1e-4 … 1e-10, nor scaled to 1e300 … 4e307
  (the meters refuse first).

## Repair shape

Per arm: a reproduction moves it to its decision's lever with a door
test; an argument that excludes it moves it to `unreachable!` with its
rows to `#[should_panic]`. For `MintLimb`'s on-locus arm, a loop that
also accepts on the on-locus limb (or a mint that compares it with the
rounding the sample carries) may make the question moot.

## Rides with this row

A topo `test_support_samples.rs` sample of an unproven `Fit` arm (e.g.
`FitError::NonFinitePoint`), so editor-core's
`refusal_concision_chains` `every_offset_fit_refusal_ends_exactly_once`
pins the `Fit/` last-resort ending as it pins `Fit/TooFewPoints`'s
lever (its roster's `Fit/` count goes to 2). Split out of PR 5476: a
topo change pulls the sweep eps rows into that PR's gate, where a known
main red at 1e-12 sits (`cpair/curve-pair-polish-meters-refuse-shallow-meetings`).
