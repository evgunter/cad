---
id: main-red-the-teapot-north-star-test-expects-the-old-spline-face-refusal
kind: issue
title: main red: python test_north_star TestTeapot.test_both_joins_refuse_through_the_document expects the spline-face refusal the boundary pass replaced with PointSplitCarrierUnsupported
status: closed
opened: 2026-10-26
priority: P1
cost: E
closed: 2026-10-26
---


Filed by CLEAVE, from PR 5544's CI (run 40343654725, job `python suite (wheel + guide + north-star)`).

## What

`crates/pncad-py/tests/test_north_star.py:3320`, `TestTeapot.test_both_joins_refuse_through_the_document`, asserts that the teapot's union refuses with "cannot yet work out where one solid's spline (NURBS) face meets". Main now refuses that union with `PointSplitCarrierUnsupported`: "a spline (NURBS) edge of the second operand has to be split where the solids meet, and the Boolean cannot find a point's parameter on a spline". That is the behaviour SPLICE's own row `a-meeting-on-a-spline-boundary-edge-has-no-split-parameter` records since the boundary pass (`666083d12f`, `76d5bbcfd5`).

The python suite runs only when a PR touches `pncad-py`, so the PR that moved the refusal did not run it. The first PR to touch `pncad-py` afterwards (CLEAVE's 5544, which changes nothing on `reduce`'s path) sees it red. That PR's `test` job is green; this is its only failure.

## Owed

Re-pin the test to the refusal main gives, or, if SPLICE's open P1 fixes the split first, to whatever that fix gives. One line in the test. Either way, the test should name the refusal kind rather than a sentence of its text.

## Closed (INTENT, 2026-10-26)

Re-pinned by the refusal's arm, as owed: `refusal.inner_kind == "point_split_carrier_unsupported"` replaces the sentence of text, so SPLICE's fix for `a-meeting-on-a-spline-boundary-edge-has-no-split-parameter` moves this row loudly, by kind, rather than by wording. It was blocking INTENT's D′3 (#5636) and value-chart (#5742) PRs. `TestTeapot` passes locally (5 rows).
