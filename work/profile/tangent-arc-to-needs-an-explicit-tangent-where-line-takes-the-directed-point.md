---
id: tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point
kind: issue
title: tangent_arc_to needs an explicit .tangent() after a leg, while line(len) takes the leg's directed point
status: open
opened: 2026-11-02
priority: P1
cost: M
design: true
---

Reported by Ev (in chat, 2026-11-02): "`tangent_arc_to` appears to
require an explicit `tangent` in advance, whereas `line` assumes
tangency. these should be made consistent though i'm not sure how best
to do that yet." Filed by the VSEAM orchestrator.

## What the vocabulary says

`docs/PATHS-DESIGN.md` §3, the transition table:
- `line(len)` — "Directed → Point; directed point → directed point".
  Off a directed point (a leg's arrival), the leg "departs along the
  point's own intrinsic tangent".
- `tangent_arc_to(p)` — "Directed → Point" only. After a leg, the
  author writes `.tangent()` first to bind the director; the corpus
  spells it `.tangent().tangent_arc_to(p)` (§2, §2a item 6, and the
  worked examples).

So two continuations that both mean "carry on smoothly from here" take
different incoming states. One reads the arrival's tangent implicitly,
and the other needs it bound explicitly.

## What deciding it needs (`design`)

Pick which reading is right for the family and apply it uniformly:
- both read a directed point implicitly;
- both need the director bound;
- or the difference is principled and wants stating.

Weigh it against the §2 departure rules, the junction checks
(`JunctionTangent`), `continue_to` and the arc analogs of `line(len)`
(`ArcLen`/`Sweep`, "tangent-departing, endpoint DERIVED"), and against
`[one-way]`. It goes to the designer pair before any lane builds it.
PATHS-DESIGN is ratified (#124), so the answer is an `[ev]` PR. P1: two
spellings of one continuation that should behave uniformly.
