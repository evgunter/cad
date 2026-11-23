---
id: a-full-ring-step-copies-its-count-formula
kind: issue
title: pncad::pattern::RingStep::Full lowers by copying the count formula into the step: one fact written twice, so a count edit leaves the step stale
status: open
opened: 2026-11-02
priority: P2
cost: M
---


## Finding

`pncad::pattern::RingStep::Full(count)` (`crates/pncad/src/pattern.rs`)
lowers a full ring by writing the count's formula a second time into
the step (`turn / count`). The ring's count and its step then hold one
fact twice. Editing the index's count afterwards leaves the step at the
old fraction, and the members overlap or leave a gap.

FORMS' design fork row 247 put an optional "full" step form to Ev: a
step that lowers to `turn/N` reading the index's own count with no
second reader (`[recipe.forms]`). If Ev wants that form, this row is
where it lands. If not, the façade's `Full` should say plainly that it
writes the count twice, or go.

Signed (FORMS orchestrator).

- 2026-11-03 — moved from LIB to PYDOOR by `git mv` when LIB closed, id and body unchanged (LIB had three P1 rows left behind 124 closed ones; its library-door rows went to PYDOOR, opened for them, and its tag, stub, census and projection rows to BIND; Ev, in chat).
