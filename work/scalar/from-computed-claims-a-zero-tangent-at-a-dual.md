---
id: from-computed-claims-a-zero-tangent-at-a-dual
kind: issue
title: Real::from_computed returns tangent zero at a dual, so a re-entered value that moves with the inputs yields a plausible wrong sensitivity
status: parked
opened: 2026-11-19
priority: P1
cost: H
blocked_on: [7155]
refs: [dl5-has-no-arm-for-a-selection-whose-result-is-the-payload, census-rows-whose-bracket-read-re-enters-the-scalar-are-filed-as-payload-or-selection]
---


Filed by the SCALAR orchestrator, 2026-11-20. The build of DL5.2
(`crates/geom-core/README.md`, "Reading a bracket where nothing
certifies"), parked on Ev's answer to PR 7155.

## What

`Dual::from_computed` is `Self::constant(T::from_computed(x))`:
tangent zero, the claim that the value does not move with the inputs.
`Sym::from_computed` is an opaque unknown. `Real::from_computed`'s own
doc says a scalar that defaulted it to `from_f64` "would re-enter
every computed value as a constant", and at a dual that is what it
does.

A survey of the 157 calls outside tests (read, not executed; counts
and lists in `work/scalar/log.md`, 2026-11-20): 57 a dual cannot
reach, about 60 only decided on or reported, 9 constants, about 10
flat to first order by a comment's argument, 10 wrong, 10
undetermined. The wrong one on the path production runs is ATREST's
`link-lowest-on-freezes-its-candidates-so-a-dual-sensitivity-through-a-tie-drops-a-term`.

## What closing it takes

- `from_computed` at a dual returns an unknown tangent: the degraded
  tangent `copysign` gives at a kink (`KinkJacobian`), NaN over `f64`
  and the whole line over an enclosure.
- A stationary door: zero at a dual, opaque at `Sym`, every use on a
  gated list with its kind (the seed of a step at `T`, a smooth
  extremum's position, an exactly free choice) and pinned against
  finite differences.
- The 9 constants move to `from_f64`; the flat sites take the
  stationary door or a step at `T`; the wrong sites are repaired or
  left reading unavailable with a row each.

## Check before building (nobody has)

- DL2 memo keys hash tangent bits, so the unknown tangent's NaN must
  be canonical (`ContentBits for f64`).
- No dual-path code may branch on a tangent's finiteness (E9).
- Whether the whole line or NaI is what E7's consumer reads as "do not
  prune" at `Dual<Interval>`.
- How many stackup sensitivities turn unavailable when it lands: the
  "decided only" bulk was classified by reading. The dual corpus and
  `blend_dual_sensitivity` show it.
- Re-entry that avoids the door: `Interval::point`, the `launders`
  rows of `from-f64-sites.tsv`, and a `value_channel` map followed by
  evaluation at `T` (`SplineWall::outward`, which also goes round
  `project`'s `CertifiedBounds` door).
