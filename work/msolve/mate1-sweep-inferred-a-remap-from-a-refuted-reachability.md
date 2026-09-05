---
id: mate1-sweep-inferred-a-remap-from-a-refuted-reachability
kind: issue
title: issue 1405's premise is wrong: the MATE-1 sweep inferred a remap requirement from a reachability AQ8's addendum had already refuted
status: open
opened: 2026-09-04
---


Routed by the FIX orchestrator out of
`work/fix/split-crossings-skip-pattern-mate-ends.md` (issue 1405),
which closed as *premise refuted, alignment kept* on PR 1749. Filed
here rather than appended to that item because the correction is owed
**whether or not** that PR lands, and FIX's directory is deleted when
the program closes — a correction scoped to outlive a program cannot
live in its slate. Filed here rather than into `work/mate/` so S-MATE
re-homes it by header edit rather than meeting it as a merge conflict.

## What 1405 claimed

That `refactor.rs`'s `is_mate_edge_end` recognizing only plain
`InstantiatePart` mate ends means a split whose seam severs a
pattern-headed mate would not carry that mate as an interface
crossing — and that the fix needs the `Instance(i)` remap through
split's node maps, not just a second match arm.

## Three findings against it, all executed on PR 1749

1. **The severing cut does not exist.** A pattern-placed head IS a
   member, so `mate::clusters` welds the mate at the pattern's input
   instance and a cut through it refuses `TornCluster` before any
   crossing collector runs. PR 1749's review closed this by exhaustion:
   **318 cut sets over two documents**, with the edge notion re-derived
   from public `reading_edges` rather than from the collector's own
   gate — no accepted cut ever left an A12 edge straddling.

2. **There was no remap to add.** `remap_seg` already preserved
   `Instance(i)`'s copy index correctly, and correctly:
   `NodeMap`'s domain is node ids only, built from cut-membership of
   `doc.order()`, while `Pattern`'s `count`/`kind` are slots that move
   verbatim. Pinned by a killed mutant (`i: *i` → `i: *i + 1`).

3. **A second match arm would have been HARMFUL, not merely
   insufficient.** A gate matching the head's *spelling*
   (`InstantiatePart | Pattern`) mints an interface crossing on a
   NESTED pattern head — which welds no cluster, so its mate's ends
   genuinely do straddle an accepted cut — for a mate that never
   solved, which AQ8's (b)-SKIP forbids. Killed by a second mutant.

## Why this is owed to S-MATE rather than just closed

The MATE-1 sweep report is S-MATE's and other units may be reading it.
The root cause is not a wrong observation but a wrong **inference**:
the sweep saw a gate that did not know the member vocabulary and
inferred a reachability, where AQ8's addendum had already established
the weld that makes it unreachable. Both facts were in the tree; the
sweep connected them the wrong way round.

That inference shape is the part worth carrying — a gate that looks
too narrow is not evidence that the narrow case is reachable, and the
cheap check is whether some earlier refusal gets there first.

## Related

`work/issues/aq8-skip-half-is-cited-as-ratified-and-is-not.md` — the
(b)-SKIP half this turns on has no ratified home, which is part of why
the sweep could reason past it.

## Home

`work/msolve/` — S-MATE's successor, opened 2026-09-04 for exactly this
residue.
