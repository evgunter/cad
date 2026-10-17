---
id: subtracting-a-slit-tool-resting-its-floor-on-a-face-escalates-sector-straight
kind: issue
title: ∖ of a slit-bearing tool whose floor rests on the other operand's face refuses Escalated at sector_straight, margin 2.5
status: open
opened: 2026-10-14
priority: P2
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
