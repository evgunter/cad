---
id: loop-boundary-discards-selftest-fails-on-a-clean-tree
kind: issue
title: scripts: loop-boundary-discards.sh --selftest fails on a clean origin/main (UNREG/MISCOUNT in section_loops loop_edges)
status: closed
opened: 2026-10-14
closed: 2026-10-26
priority: P2
cost: E
---

(Filed by the ENCL orchestrator. An ENCL lane hit this while verifying PR 5101, which does not touch either file.)

## What

On a clean origin/main, `bash scripts/gates/loop-boundary-discards.sh --selftest` fails with "SELFTEST FAILED: the gate FAILED on a clean fixture". The gate itself, run without `--selftest`, passes. The selftest prints:

```
UNREG|crates/topo/src/splitting/section_loops.rs|4|loop_edges|let LoopBoundary::Cycle { first } = body .get_loop(lk) .ok_or_else(corrupt)? .boundary else { return
MISCOUNT|crates/topo/src/splitting/section_loops.rs|loop_edges|LoopBoundary::Empty { .. } => return|pinned 1|matched 0
```

So the register entry for `loop_edges` (`crates/topo/src/splitting/section_loops.rs`) pins a `LoopBoundary::Empty { .. } => return` arm, but on the selftest's clean fixture the site reads as a `let … else { return` discard instead.

The most recent change to the gate is TANG's commit 8f0b0ffd25 ("TANG: review fixes …", a 3-line edit to the gate script), about a day before this filing. `loop_edges` itself last changed 9 days ago (29a45b6204). That points at the gate edit, or at the selftest's fixture copy, rather than at topo. Not bisected.

## Ask

Make the selftest pass on a clean tree. Either re-register `loop_edges`'s discard as the shape it now has, or fix whatever makes the selftest fixture read it differently from the live gate. Then say which it was.

## Closed (2026-10-26, tracker sweep)

**Duplicate, and fixed.** The same failure as
`loop-boundary-selftest-cannot-plant-an-arm-fragment` (opened
2026-10-06, which carries the diagnosis: the planter picked a let-else
form for an ARM fragment). That row is closed by this sweep's fix to
`gate_plant_clean` in `scripts/gates/loop-boundary-discards.sh`; see its
Closed section. The cause was not TANG's `8f0b0ffd25`: the failure was
already reproduced on `212ccfa1` on 2026-10-06.
