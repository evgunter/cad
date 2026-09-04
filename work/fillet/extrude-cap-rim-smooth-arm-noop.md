---
id: extrude-cap-rim-smooth-arm-noop
kind: issue
title: extrude's cap-rim smooth arm is a literal no-op resting on a falsified premise sentence (prefer-intrinsic does not cover it)
status: open
opened: 2026-08-31
github: 1390
refs: [1152, 1378, 1382]
---

## From GitHub issue 1390

Opened 2026-08-31; 0 comments.

(S-BOOL orchestrator) Filed from BOOL-1's dual review ([#1378](https://github.com/evgunter/cad/pull/1378), issue 1152). A reviewer's workspace-wide sweep of `DihedralClass::` consumers found `crates/sweep/src/extrude.rs:1189` (at #1378's frozen head `3f14f3c4`): `Ok(DihedralClass::Smooth) => Ok(())` on extrude's cap-rim describe — a literal no-op smooth arm of the same genus issue 1152 fixed in `topo::split`.

Its justification comment reads *"tier 3's prefer-intrinsic enforcement exempts definitely-smooth edges, so a Smooth rim stays valid"* — and #1378 falsifies exactly that reasoning: the tier-3 rule that fired on the split defect is `DescriptionNotAdjacent`, not prefer-intrinsic, and smoothness does not exempt an edge from it. So either (a) the arm is genuinely unreachable with a stale citation (reachability was argued "believed unreachable" and not falsified by the review — nobody constructed an input), in which case the sentence should state the true argument, or (b) it is the next instance of the class and owes the #1152 treatment. `sweep/` ground (SMELL Track T fence): filed rather than fixed by S-BOOL.

Related: the staleness-ladder consolidation issue filed alongside this one; #1382 (the boolean rebuild instance).

## Home

The instance is `crates/sweep/src/extrude.rs`, which S-BOOL explicitly declines as outside its ground (SMELL Track T fence) and which no open program's territory glob covers, so it lands in `work/issues/`.
