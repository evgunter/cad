---
id: witness-bifurcation-arm-has-no-inner-word
kind: issue
title: the witness-bifurcation arm projects no inner word, and cannot until the M6 solver constructs one
status: open
opened: 2026-09-08
refs: [LIB-ARMS]
---


Found by LIB-ARMS, which projects every other `NodeErrorKind` arm's
inner refusal as `EvaluationError.inner_kind`.

`NodeErrorKind::WitnessBifurcation(WitnessBifurcation)`
(`crates/editor-core/src/eval/mod.rs`, the arm's own doc) carries a
record whose `kind: BifurcationKind` is exactly the discriminant this
attribute projects everywhere else — three arms, `fold_proximity`,
`ambiguous_basin`, `residual_failure`
(`crates/editor-core/src/witness.rs:122`). It arrives in Python as
`kind == "witness_bifurcation"` with `inner_kind` `None`
(`crates/pncad-py/src/tags.rs`, `node_inner_kind_tag`).

Two facts hold it there, and both are stated at the site:

- **The arm is never constructed.** Its own doc says so: "NEVER
  constructed before the M6 solver: the arm exists so the solver
  lands as logic, not a schema change." Nothing can raise one, so no
  word could be exercised from either language.
- **The façade curates the record interior.** `BifurcationKind` and
  `WitnessBifurcation` are in `NOT_CARRIED`
  (`crates/pncad/tests/all.rs`, the witness/verdict/diff
  instrumentation family), so `pncad-py` — which depends on `pncad`
  and `quantity` alone — cannot name the type its tag function would
  take. LIB-ARMS carried `NamingError`, `ProgramRefusal`, `SeedError`
  and `ParamBoxError` out of that list for exactly the payload rule
  that would apply here; it did not carry this one, because a payload
  nothing can hold is not a payload a consumer is missing.

What closes it: the day the M6 solver constructs the arm, the façade
carries `BifurcationKind` and `node_inner_kind_tag`'s
`WitnessBifurcation` row stops returning `None` — a three-line change,
plus the census row and a Python row that reaches one of the three
words. Until then the `None` is a decision and not an omission, and
this file is what says the decision has an expiry.
