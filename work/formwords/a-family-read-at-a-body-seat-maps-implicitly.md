---
id: a-family-read-at-a-body-seat-maps-implicitly
kind: issue
title: An operation that reads a family maps over its members implicitly; Ev: mapping over a list should be written, not inferred
status: open
opened: 2026-11-22
priority: P1
cost: H
design: true
refs: [a-subtract-tool-seat-takes-a-whole-family-and-cuts-once-per-member, a-split-of-a-family-has-no-halves-a-read-reaches]
---


Ev on `[ev]` 7386 (2026-11-22): "i am confused about how the issue was
possible, though. we shouldn't be implicitly mapping over lists! i
would've thought there'd be a difference in how those were written,
like `{k from 1 to N: union(A, {f(k)})}` vs `union(A, {k from 1 to N: f(k)})`
(i don't remember the actual syntax)".

Today D10 `[repetition]` has no binder. "A definition or operation whose
reads reach `k` is evaluated once per value of it": membership is
derived from reads. So `subtract(plate, holes)` silently means
`{k: subtract(plate, holes[k])}`, and the subtract-by-a-pattern trap
(`a-subtract-tool-seat-takes-a-whole-family-and-cuts-once-per-member`,
closed by making the tool a `Bodies` seat) is one instance of the
class. It came in at FORK-PAT (`docs/DESIGN-FORK-LOG.md` row 99,
2026-10-08, commit 132d3e4716). Ev had asked whether "a map
higher-order function" would suffice; the designers moved away from a
stored `map` ("restates its domain per step, kept in step by hand") to
derived membership.

P1: architecture that entrenches. Every family-reading door (the split
of a family, `a-split-of-a-family-has-no-halves-a-read-reaches`; the
seats; Python) builds on the implicit map. Designers weigh it, and the
decision goes to Ev: it changes a ratified decision.
