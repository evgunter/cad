---
id: a-miss-in-clip-to-answers-its-input-rather-than-poison
kind: issue
title: SpanLocate::clip_to answers its input on a miss, quietly absorbing a state the fit cannot reach, where fail-loud would poison
status: open
opened: 2026-10-19
priority: P3
---


Found by reviewer B's narrow delta-read of C6 S5c (PR 5332), D4 (unsure). Not taken in that PR; every not-taken finding gets its own item (the INLANE orchestrator's rule).

## Finding

`impl SpanLocate for Interval`'s `clip_to` (`crates/geom-core/src/interval.rs`, beside `enclosure_hull`) returns `self` when `[max(lo, self.lo), min(hi, self.hi)]` is empty.

In the fit, the only caller (`crates/geom/src/curves/fit.rs`, `enclosure_piece`), a miss is unreachable:
- `locate_held` and `span_range` admit only spans whose knots' brackets meet `t`;
- `check_ordered` keeps every parameter inside the domain.

Reviewer B executed a mutant that poisons on a miss, and every fit row and probe stayed green. So the branch absorbs an impossible state. Returning `self` is sound, since it encloses every member, but silent. Under the project's fail-loud rule, poison would say "impossible" out loud.

## Owed

A decision:
- **(a) Poison on a miss.** A future caller that clips to a range no member is in then refuses rather than proceeding loose. This needs a row: a direct `clip_to` miss is poison.
- **(b) Keep `self`.** The trait docs then state it as the deliberate choice, since the evaluation lane loses only tightness, unlike certification's refusing clamp.
