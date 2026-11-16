---
id: a-search-that-answers-by-mutation-and-ok-unit-hides-its-question-mark
kind: issue
title: escalation-question-marks.sh reads a return of Ok(()) as no answer, so a search that answers by mutating its input and returning Ok(()) hides its band-reading ? from the gate
status: open
opened: 2026-11-12
priority: P3
cost: E
refs: [shared-vertex-crossings-that-tie-or-interleave-are-unprobed]
---


Found by the full review of GAUGE's PR 6832 (2026-11-12).

## What

`scripts/gates/escalation-question-marks.sh` counts a band-reading `?`
inside a loop only where the outermost loop has an early answering
exit. It reads a `return` of `Ok(())` as no answer (the header near
line 28, and the awk near line 128). A search that answers by mutating
its input and then returning `Ok(())` therefore has its `?` go
uncounted, though it is a first-refusal search like any other.

Witness: `insert::choose_starts` (`crates/topo/src/boolean/insert.rs`,
added by PR 6832). Its loop over the choices of a shared vertex's
starts swaps each choice in, reads `blocked(..)?`, and returns `Ok(())`
on the first clear one. That `?` is the right shape: the choices are
read in order of preference, so an earlier choice the band cannot read
is a premise of the later ones. PR 6832 records that in
`escalation-question-marks-callee.counts`' comment. The gate itself
does not count the line.

The blind spot is the gate's, not this site's. Other searches of the
same shape are invisible to it.

## Owed

Read a `return Ok(())` that follows a write to a binding the loop
mutated, or to a `&mut` parameter, as an answering exit. Register the
sites that turns up, `choose_starts` among them.
