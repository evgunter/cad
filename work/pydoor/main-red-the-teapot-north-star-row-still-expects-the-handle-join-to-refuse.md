---
id: main-red-the-teapot-north-star-row-still-expects-the-handle-join-to-refuse
kind: issue
title: main red: python test_north_star TestTeapot.test_both_joins_refuse_through_the_document expects the handle join to refuse; since f1f3a9a8dc it builds
status: open
opened: 2026-11-11
priority: P1
---


Filed by the onedoor fix-3 lane (PR 6505), whose PR CI met it.

## What

`crates/pncad-py/tests/test_north_star.py`,
`TestTeapot::test_both_joins_refuse_through_the_document`, asserts
`assertFalse(ev.succeeded(handle_join))` (handle ∪ vessel refuses
`boolean`/`result_invalid`, the sphere face's volume unread). Main's
`f1f3a9a8dc` ("teapot wall 2 retired") gave the vessel's chorded sphere
face and the handle's chorded torus face a rotational lane, so the
union now builds and validates: `demos/tour/src/teapot.rs` retired
wall 2 and `work/splice/curved-demo-joins-build.md` records it. The
python row was not moved with it.

## Evidence

At main `865e27e41b` (no PR 6505 code), a debug wheel
(`maturin build -m crates/pncad-py/Cargo.toml`) and
`python -m unittest discover` over `crates/pncad-py/tests`: 955 tests,
one failure, this row, `True is not false` at its `assertFalse`.

## What closing it takes

Move the row to the Rust scene's new state: the handle join succeeds,
held to what the scene can say of it (it validates; its volume against
an independent reading once one exists, per the splice row), and the
docstring's "two unions, each refused" restated.
