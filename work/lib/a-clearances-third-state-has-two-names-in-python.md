---
id: a-clearances-third-state-has-two-names-in-python
kind: issue
title: A clearance's third state reads status Refused in Python while a value assertion's reads Unevaluated
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by the DOORS style review of PR 6292 (S-m7).

Python's `Value.assertion.status` reads `"Unevaluated"` for a value
assertion the build could not decide, and `"Refused"` for a clearance
in its third state (`crates/pncad-py/src/py/value.rs:1022`,
`crates/pncad-py/src/py/doc.rs:3417`). Both are the verdict's third
state (ERROR-DESIGN E3's amendment), so one concept answers to two
words, and a script that checks `status == "Unevaluated"` silently
passes every clearance.

## Repair shape

One word for the third state, with the class carried beside it
(`refusal` for a clearance, the reason for a value assertion). Which
word is a binding-surface choice for this program: the stubs
(`pncad.pyi`) and `TestTheClearanceRequirement` move with it.
