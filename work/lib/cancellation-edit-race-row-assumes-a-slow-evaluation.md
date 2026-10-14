---
id: cancellation-edit-race-row-assumes-a-slow-evaluation
kind: issue
title: test_cancellation's mid-run edit row assumes evaluation outlasts a 20 ms sleep, and fails against a release wheel
status: open
opened: 2026-10-04
priority: P4
cost: E
---


Found by REACH (PR 3984's last fix pass) running the Python suite
locally against a `maturin build --release` wheel.

## Finding

`crates/pncad-py/tests/test_cancellation.py`
`TestCancelingARunUnderWay.test_editing_the_document_mid_run_is_refused_not_raced`
starts a thread that sleeps 0.02 s and then edits the document, and
asserts the edit was refused because `evaluate` still holds the
borrow. Against a release wheel the `WIDE` stack's evaluation can end
before the 20 ms are up, so the edit lands on an idle document and the
row fails ("a mutation landed on a document under evaluation instead of
being refused"). Measured: it failed in two full-suite runs out of two
and passed in five out of five when run alone. Hosted CI builds the
wheel without `--release` and passes.

## Fix

Synchronize instead of sleeping: have the edit wait on a signal that
the evaluation has taken the borrow (or on a run long enough by
construction at either profile), so the row tests the borrow rather
than the machine's speed.
