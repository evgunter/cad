---
id: runline-is-a-curated-facade-name-the-binding-census-does-not-list
kind: issue
title: RunLine reached the pncad prelude without a binding-census entry, so the python suite is red on main
status: open
priority: P1
opened: 2026-10-13
---


## What

`RunLine` became a curated façade name in `d720a29b81` ("Carry RunLine
through the façade"), merged with PR 4922 (`emit/vertex-run`) at
`eef3255a1b`. It is exported by `crates/pncad/src/select.rs:96` and
`crates/pncad/src/prelude.rs:739`. It is neither bound in Python nor
listed in `crates/pncad-py/tests/test_binding_census.py`. So
`test_binding_census.TestBindingCensus.test_every_curated_name_is_bound_or_listed`
fails on every PR's python suite:
`AssertionError: Lists differ: ['RunLine'] != []`. This was seen on FLUX's
PR 4932, run 38341212261, which does not touch bindings.

## The shape of a fix

Its siblings `RolePath` and `RoleSeg` sit in `NOT_BOUND` as `SHAPE`
(`test_binding_census.py:2625`). So the likely one-line fix is
`"RunLine": SHAPE,` beside them. Binding it in Python is the
alternative. Which family it belongs to is EMIT's call.
