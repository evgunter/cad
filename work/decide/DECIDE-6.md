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

`decision-read-triples-the-plate-pin-suites-wall-time`. The row put
the plate's dev pin suite going from 143 s to 535 s at DECIDE-3 on the
decision read's depth-8 enclosure, and named three cheap answers to
take before it: an enclosability pre-pass, a memo per form digest, and
a depth-1 attempt first.

This unit measured first (calls, declines by cause, the read's and the
enclosure's time) and found the premise false. On every document of
the plate's pin suite the read is under half a percent of a replay,
and running it ten times at every call does not move the suite. So it
took none of the three answers. What it lands is the read's
instrument, its evidence rows, and the finding: on the row itself, and
in `rule-g-is-the-link-and-pads-leaf-cost`, which holds the measured
separation putting DECIDE-3's cost on rule G.

Spec: `docs/DECIDE-6-SPEC.md`. Opus implementer. Review tier: single
FULL review (`work/decide/log.md`, 2026-09-25).
