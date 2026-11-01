---
id: main-red-three-curated-member-names-are-unbound-in-python
kind: issue
title: Main red: the binding census fails on three curated names PR 6208 added (MemberCountFault, MemberEnv, Reached)
status: closed
opened: 2026-11-01
closed: 2026-11-01
priority: P0
cost: E
---

## What

`crates/pncad-py/tests/test_binding_census.py`
`TestBindingCensus.test_every_curated_name_is_bound_or_listed` fails on
main: "3 curated façade name(s) are neither bound in Python nor listed:
['MemberCountFault', 'MemberEnv', 'Reached']".

The three names enter the curated façade at
`crates/pncad/src/document.rs:124`
(`pub use editor_core::{Index, IndexFault, MemberAt, MemberCountFault, MemberEnv, Reached};`),
written by `8f650cbe` and merged to main with PR 6208
(`spaces/pattern-value-rows`, merge `387df94f`). Neither `pncad.pyi`
binds them, nor do `BOUND_AS` or `NOT_BOUND` in the census list them.

## Evidence

- Reproduced on bare `origin/main` at `45d0f75f` in a clean worktree: the
  wheel was built with `maturin build -m crates/pncad-py/Cargo.toml`, and
  the one test was run. It fails as above.
- Seen red in PR 6163's CI (run 41288351275, job 124794392032, `python
  suite`) on the merge with main.
- Main's push CI on `387df94f` (run 41287176354) skipped every test job,
  so main's own runs do not show it.

## The fix

Bind each name, or add it to `BOUND_AS` or `NOT_BOUND` with its family,
as the test's message says.

## Fixed by

CLEAR, on branch `clear/binding-census-red`: `MemberCountFault` is in
`BOUND_AS` as `EvaluationError.kind` (it crosses as the `index_count` /
`index_count_out_of_range` tags), and `Reached` and `MemberEnv` are in
`NOT_BOUND` as `behind-a-door` (`evaluate` makes both reads inside).
