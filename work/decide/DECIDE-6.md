---
id: DECIDE-6
kind: unit
title: the decision read's cost: decline before enclosing, with every decision unchanged
status: review
opened: 2026-09-25
priority: P2
cost: D
branch: decide/6-read-cost
pr: 3229
refs: [decision-read-triples-the-plate-pin-suites-wall-time]
---

## What

`decision-read-triples-the-plate-pin-suites-wall-time`. The decision
read encloses both halves of every `Select`/`min`/`max` decision to
depth 8 before it can decline, and most calls decline, so DECIDE-3 took
the plate's dev pin suite from 143 s to 535 s.

This unit makes the read decline before paying for the enclosure, with
every decision unchanged. It measures first (calls, declines by cause,
enclosure time), then takes the item's three cheap answers in order,
only as far as the measurement justifies:
1. an enclosability pre-pass;
2. a memo per form digest;
3. a depth-1 attempt first.

Spec: `docs/DECIDE-6-SPEC.md`. Opus implementer. Review tier: single
FULL review (`work/decide/log.md`, 2026-09-25).
