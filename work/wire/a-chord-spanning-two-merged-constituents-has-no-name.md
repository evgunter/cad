---
id: a-chord-spanning-two-merged-constituents-has-no-name
kind: issue
title: A seam chord that crosses the merged-away boundary between two same-side constituents of a merged face has no naming rule, and N2 states no chord-descent rule at all
status: open
opened: 2026-10-17
priority: P1
cost: M
design: true
refs: [a-merged-face-with-several-same-side-constituents-has-no-chord-rule]
---


## Finding

PR 5263 builds the merged-chord rule for a chord one constituent holds
whole (the constituent whose operand face holds the chord). Its review
(2026-10-17) built the case the rule does not cover: plate `[0,3]²×[0,1]`
∪ the split boss (r 0.3 about (1.5,1.5), z 0.5–1.5, pair union), minus
`Union{block x(1.4,1.6) y(−1,4) z0 0.8 dz 1.2; block x(0.9,1.4) y(1.4,4)
z0 1.3 dz 0.7}`. The second block's floor cuts the boss's merged wall in
one edge from (1.217,1.4,1.3) to (1.4,1.783,1.3), crossing the abut line
the merge removed, so the chord borders both constituents. After the fix
pass it refuses typed `MergedChordConstituents`; on the first build of PR
5263 it picked one piece by the chord's middle and the pick flipped with
the block's y extent. A second witness: a slab wall on the boss's split
plane (block x(1.4,2.0) y(1.5,4)) puts a chord along the abut line, where
both constituents answer `OnEdge`.

Two questions, for designers before anything is built:
- what such a chord's name is (both constituents? the operand edge's
  line, unqualified? a refusal?), so that it is stable as the cut moves;
- whether N2 should state the chord-descent rule at all. Today the rule
  PR 5263 builds is stated only in
  `a-merged-face-with-several-same-side-constituents-has-no-chord-rule`;
  `crates/editor-core/src/names/README.md` N2 has no clause for it, so a
  clause would be new ratified naming text, which is Ev's.

Signed (WIRE orchestrator).
