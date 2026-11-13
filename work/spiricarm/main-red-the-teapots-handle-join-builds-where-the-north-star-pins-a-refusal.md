---
id: main-red-the-teapots-handle-join-builds-where-the-north-star-pins-a-refusal
kind: issue
title: main red: the teapot's handle union now builds, where the north-star row pins its refusal at the result's validation
status: open
opened: 2026-11-11
priority: P1
cost: E
---


Found by ONEHOME stage 2 (PR 6820): CI's python suite on the PR's merge
with main.

## What

`crates/pncad-py/tests/test_north_star.py:3286`
(`TestTeapot.test_both_joins_refuse_through_the_document`) asserts
`ev.succeeded(handle_join)` is false: the handle (a torus) unioned with
the vessel is pinned to refuse at the result's validation, its volume
not computed, because "the vessel's sphere face [is] trimmed by a conic
loop no sphere-chart quadrature reads yet" (the comment above the
assertion).

On main, after the rotational lane's torus stage (PR 6808,
`spiricarm/rotational-lane-torus`, the likely cause, not bisected), the
union builds: the assertion
fails with `True is not false`. Measured on plain main (`origin/main`
at the time of filing, debug cdylib, the one test alone), and on PR
6820's branch merged with it; PR 6820's branch before that merge passes.

Main's push CI does not run the python suite, so the red shows only on
the next PR that seeds `pncad-py`.

## Done when

The row reads what the kernel now does: if the handle union is right to
build (the sphere face's quadrature now reads the conic loop), the
north-star row pins the built union (its volume and area against the
handle and vessel's) and its comment says which lane reads it; if not,
the regression is found and fixed.
