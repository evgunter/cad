---
id: euler-characteristic-has-three-carriers
kind: issue
title: The Euler characteristic has three carriers: readback::EulerCounts, validate's ComponentCounts and review_m1_pr4's ShellComponent
status: closed
branch: walks/euler-theorem
pr: 7430
opened: 2026-09-12
refs: [no-public-census-or-genus-query, 2131]
priority: P1
cost: M
closed: 2026-11-23
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

## Staleness audit (2026-11-22, WALKS, origin/main 7709b0ea)

Still live as written. Additional hand chi: `ComponentEulerViolation`'s Display (validate.rs), review_m1_pr1.rs, and sweep/topo test files (`work/green/census-tuple-rows-assert-chi-not-genus.md`). Three postures on a negative genus: a value (`EulerCounts::genus`), a report (validator), a panic (review_m1_pr4). The component relations under the counts differ: `ShellComponent` glues through shared vertices, `ComponentCounts` through mated edges, so the two "per-component" genera are different quantities on vertex-touching components — coupled with `shell-glue-relation-has-three-implementations`. `ComponentCounts` feeds a public error with usize fields (pncad-py tags it). Weighed with that row as fork F5.
## The question to Ev (2026-11-22, fork F5, with `shell-glue-relation-has-three-implementations`)

Two designers, two reconciliation rounds. Agreed: pass 11's per-component Euler–Poincaré check is a theorem of checks 1–7 and 10 (it fires today only as an echo of pass 10), so it goes, with `ComponentCounts`, `satisfies_euler_poincare` and the public `ComponentEulerViolation`; pass 11 counts components for tier 2 and keeps its OWN walk as the shipped re-derivation (D2), while movefac, seqgen and shell.rs share one routine (`walks/glue-home`). The `[topology.tiers]` restatement is in the `[ev]` PR's diff. Split: what the census door's `genus()` should be — name both numbers (`EulerCounts` gains `c`; `h()` = s − χ/2, `genus()` = c − χ/2), or rename the current value `h()` and add nothing.

## Decided (Ev, 2026-11-22, PR 7424)

"sounds good! just renaming to `h()` and adding no code sounds very simple and good presuming that it is, y'know, correct." So: the agreed part as proposed (pass 11's Euler arm, `ComponentCounts`, `satisfies_euler_poincare` and the public `ComponentEulerViolation` go; pass 11 is gated on 1–7 and 10 and counts components; the validator keeps its own walk; `[topology.tiers]` states the identity as a theorem of the checklist), and the census door's `genus()` is renamed `h()` with no component count added; its doc says h is the operator coordinate s − χ/2 and equals the genus at tier 2. "Presuming it is correct" binds the unit: the theorem's test pin (per-component counts with s = 1 asserting `h() ≥ 0` on every seqgen step and over the movefac fixtures) must run red-first against a planted violation before the arm is deleted.

The pass-11 gate is passes 1–7, not "1–7 and 10" as the PR 7424 text put it (review of PR 7430): the pass-10 gate only stopped the Euler arm's echo, the arm is gone, and gating the component count on pass 10 would drop tier 2's `ShellDisconnected` for a shell left in pieces beside a cross-shell edge.

## Closed

2026-11-23: PR 7430 merged (the theorem pinned red-first, the arm and `ComponentCounts` deleted, `genus()` renamed `h()`, pass 11 gated on 1–7). The shell-glue half closed with PR 7438.
