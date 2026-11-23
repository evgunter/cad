---
id: python-writes-repetition-through-doc-each-and-binder-presets
kind: issue
title: Python writes repetition through Doc.each, and its pattern presets write the binder
status: open
opened: 2026-11-23
---

PR 7506 (the kernel binder build) gives Python only `DocEdit.set_yields`.
Every Python test that patterns writes its yields by hand: for example
`crates/pncad-py/tests/test_pattern.py` `row_of` and
`crates/pncad-py/tests/bench_scene.py`. The Node pattern writers
(`crates/pncad-py/src/py/doc.rs` `Node.linear_pattern`, `grid`,
`circular_pattern`, `bolt_circle`) author one placement and no binder.

The design (`work/formwords/a-family-read-at-a-body-seat-maps-implicitly.md`,
Python bullet) asks for the following:

- `doc.each(N, lambda k: …)`, `doc.each(holes, lambda k: …)` and
  `with doc.each(N) as k:`.
- Preset writers that write the binder.

The next FORMWORDS PR builds these, stacked on 7506: branch
`formwords/binders-py`, held until 7506 lands.
