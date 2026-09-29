---
id: DECIDE-6
kind: unit
title: the decision read's cost: decline before enclosing, with every decision unchanged
status: closed
opened: 2026-09-25
priority: P2
cost: D
branch: decide/6-read-cost
pr: 3229
refs: [decision-read-triples-the-plate-pin-suites-wall-time]
closed: 2026-09-25
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

## Closed (2026-09-25)

Merged as #3229 into `props/sign-hull` (landing head `25f64b906`, run
36148113154). The unit closed at its measurement.

**What the measurement found.**
- The decision read alone is under 0.4% of every replay (link 1.09 ms
  of 38.1 s; bracket 50 ms of 13.9 s; pad 4.2 ms of 293 s, dev).
- Shutting the read, or running it ten times over, moves no suite beyond
  noise.
- None of the item's three answers was taken: each saves at most what
  its own walk costs.

**What ships.** The profiling instrument (`ReadProfile`, under
`sym-profile-testing`), with the decline's cause noted at the
enclosure's own refusal arms. It comes with evidence rows, a gating row
checking that the profile decides nothing, and a scalar-door row for
`order`'s difference.

**Where the cost went.** The 143 s → 535 s the item recorded went to
rule G, measured by the review's separation: rule G shut takes the pad's
dev leaf 280.6 → 122.4 s, while the exact quotient and A0's `min`/`max`
folds are flat. It is filed as `rule-g-is-the-link-and-pads-leaf-cost`.

**Review.** A single FULL review on `cadc16eaf`: APPROVE-WITH-FIXES
0/5/6. Fix pass 1–11 plus style at `25f64b906`: the read timed alone,
the mirror walk replaced by hooks at the enclosure's refusal arms, and
the attribution measured.
