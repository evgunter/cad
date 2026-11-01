---
id: subtracting-a-slit-tool-resting-its-floor-on-a-face-escalates-sector-straight
kind: issue
title: ∖ of a slit-bearing tool whose floor rests on the other operand's face refuses Escalated at sector_straight, margin 2.5
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier]
---

Found by the sweep of topo's
`reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier`
(branch `cleave/revert-repairs-coincident-edges`). Measured at rest,
default ε, one pose.

## What

The tool is the split's one-end slit body (`slit_body` in
`crates/topo/tests/a_doubled_line_does_not_revert.rs`): the box
`[0,4]×[−1,3]×[0,2]` less the trench, the piece above `z = 1`, its
section face carrying the keel from (1.5, 1, 1) to (4, 1, 1) as two
coincident edges. `subtract(a, tool)` with `a` the slab
`[−1,5]×[−2,4]×[0,1]` (the tool's floor resting on `a`'s top), or the
same slab ending at `x = 3`, refuses:

```
Escalated { decision: Corner(Straight { full_circle: false }),
  diag: Indeterminate { margin: MarginDiag(Value(2.5, Some(Rejection {
  sign: Positive, passes: Negative }), ..)), .. predicate: Some("sector_straight") } }
```

(margin 1.5 for the slab ending at `x = 3`). The answer is the slab
itself, 36: the tool only rests on it.

A margin of 2.5 is no near-zero reading; the rejection says the
reading's passes disagree on its sign. The likely subject is the slit's
0° tip at (1.5, 1, 1), where the sector pass reads a corner whose two
edges coincide (`crates/topo/src/sector_shape.rs`, rung 3). Not traced:
read the raising site before building on that.

## A second witness (PR 5357's review, 2026-10-19)

The review of PR 5357 found that `union(flipped cusp result, line)`
still escalates `Corner(Straight)` on a definite `Positive`. That is the
review's witness in its own words: the cusp result is a `tool ∖ line`
cell of sweep's `a_difference_leaving_a_cusp_along_the_ruling_builds`
(not traced further here). It is the same mis-typing that PR 5357
fixed in the line reader (`census::line_wedges` now types `sector_straight`'s
definite rejection of a cusp as `WedgeRefusal::Unread(_, TouchUnread::Degenerate)`, not as an
escalation). Here the `Corner(Straight)` decision reaches the
Boolean's sector reading (`sector_shape` rung 3) and is reported as
`Escalated`, so its lever offers a move no tolerance answers. Type the
definite rejection there as a wedge end too (D1: a consumer with no
wedge-0/2π answer refuses typed).

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P3: a definite rejection reported as `Escalated` with a lever no tolerance answers: library usability: the refusal names the wrong cause, recourse or payload, P3 by `work/README.md` Priority.
- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
