---
id: enforce-checks-has-no-caller-outside-tests
kind: issue
title: enforce_checks, the registry's one refusing path, has no caller outside tests
status: open
opened: 2026-10-08
---


`enforce_checks` (`crates/editor-core/src/checks.rs:1516`) is documented as
the registry's ONLY refusing path (`checks.rs:23`, `checks.rs:185`), and the
Python binding says the same (`crates/pncad-py/src/py/checks.rs:8`). No
code outside tests calls it: the hits are its re-exports
(`crates/editor-core/src/lib.rs:114`, `crates/pncad/src/document.rs:546`)
and test rows (`crates/editor-core/tests/dsc_checks.rs`,
`crates/editor-core/tests/node_labels.rs`, `crates/pncad-py/tests/`).
So no product path refuses through the registry today; every clause that
says a check "refuses" at `Error` describes what a caller could ask for,
not what any caller does.

Found by FORK-S5C's designer A (branch `design/intent-s5-s5c-A`,
`design/intent-s5-s5c-A.md`, "Off-question finding"). The fork's answer
decides whether the at-rest census refuses anywhere; whichever way it
goes, the docs that say the registry refuses should say who calls it, or
a caller (the tour, CI, export) should exist.
