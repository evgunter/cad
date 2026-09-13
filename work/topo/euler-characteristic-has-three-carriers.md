---
id: euler-characteristic-has-three-carriers
kind: issue
title: The Euler characteristic has three carriers: readback::EulerCounts, validate's ComponentCounts and review_m1_pr4's ShellComponent
status: open
opened: 2026-09-12
refs: [no-public-census-or-genus-query, 2131]
---

## Finding

After PR 2131 the whole-body Euler–Poincaré census has one public door,
`readback::euler_counts` → `EulerCounts { v, e, f, r, s }` with
`EulerCounts::genus` and its typed parity refusal `EulerParityError`.
The same characteristic `v − e + f − r` is carried by two more types in
the crate, each with its own field names and its own parity spelling:

- `validate.rs`'s `ComponentCounts { vertices, edges, faces, rings }`,
  the validator's per-component pass, whose
  `ComponentCounts::satisfies_euler_poincare` decides
  `chi % 2 == 0 && chi <= 2` — parity AND a non-negative-genus bound in
  one predicate, where the door reports a negative genus as a value.
- `review_m1_pr4.rs`'s `ShellComponent { v, e, f, r }` with an inherent
  `ShellComponent::genus`, the review module's own component walk
  (`shell_components`), test-local.

The door's `EulerCounts::genus` doc sends a caller who wants the
per-component statement to "the validator's component pass", so the
door and the validator are already spoken of as one family while they
share no type and no arithmetic.

## Why it matters

Three spellings of one identity is the class the door was minted to
close, one level down: a parity guard that drifts between them (the
validator's `chi <= 2` arm is documented as believed-unreachable; the
door has no such arm) is a drift in what a green row means. A single
`EulerCounts`-shaped carrier — the validator's component pass producing
`EulerCounts` with `s = 1` per component, the review module's walk
reading the same — would leave one parity check and one genus
arithmetic in the crate.

## Not asserted

Whether `ComponentCounts` should become `EulerCounts` (the validator
then owes a `genus() ≥ 0` check of its own on top of the door's parity)
or `EulerCounts` should grow a per-component constructor; and whether
the review module's twin folds at all, given its header asks not to be
simplified to match the implementation. Shape (C) of the ruling on
`no-public-census-or-genus-query` — per-component genus as public API —
is the door this residue would be the first consumer of.
