---
id: runline-is-curated-in-the-facade-and-unbound-in-python
kind: issue
title: RunLine is curated in the pncad façade and neither bound in Python nor listed, so main's binding census is red
status: closed
pr: 4945
opened: 2026-10-13
priority: P0
cost: E
closed: 2026-10-13
---


Found by the FLUX lanes on PRs 4932 and 4938, whose `python suite`
job went red on the merge ref with a failure their diffs do not touch.
It reproduces on `origin/main` alone (`a469c499d`):

```
cd crates/pncad-py/tests
python3 -m unittest test_binding_census.TestBindingCensus.test_every_curated_name_is_bound_or_listed
AssertionError: Lists differ: ['RunLine'] != []
```

PR 4922 (`d720a29b8`, "Carry RunLine through the façade") added
`RunLine` to the façade's curated lists (`crates/pncad/src/select.rs`,
`crates/pncad/src/prelude.rs`). Nothing in `crates/pncad-py` binds it,
and `crates/pncad-py/tests/test_binding_census.py` neither lists it in
`BOUND_AS` nor in `NOT_BOUND`, so the census fails on every PR's merge
ref.

Fix shape: bind `RunLine` beside the `RoleSeg::EdgeCrossing` spelling
Python already carries, or list it in `NOT_BOUND` with its family and
a `gap:` pointer, per the census's own message.
