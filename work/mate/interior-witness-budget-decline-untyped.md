---
id: interior-witness-budget-decline-untyped
kind: issue
title: interior_witness budget exhaustion: type the decline (it currently re-raises the carried refusal, indistinguishable from a thin overlap)
status: open
opened: 2026-09-01
github: 1478
refs: [1472, 1435]
---

## From GitHub issue 1478

opened 2026-09-01, 0 comments.

Filed at MATE-8's adjudication (PR #1472, issue 1435's unit) as the scheduled home for a deviation both review arms found disclosed-but-unscheduled.

**The gap.** `chart_region::interior_witness`'s stage-2 budget guard (`WITNESS_BUDGET`, 128 segments / 4096 cells) is honest in the all-or-nothing sense — it never silently narrows the search and calls the result a decision — but its exhaustion is spelled `bool` and re-raised as the carried `TouchingBoundary` → `CensusUnsupported { Face }`, the same error a genuinely thin overlap gives. No caller can tell "the schedule gave up" from "the overlap is undecidably thin". Both MATE-8 review arms demonstrated the silent half (a fat, decidable overlap over the segment cap declines with zero probes issued).

**Why it wasn't typed in MATE-8.** The blocker is NOT the `ChartRegionError` Display contract (`ChartRegionError` and its `Display` are inside that unit's fence, and `RayExhausted` is the in-fence precedent for a typed schedule exhaustion). The real out-of-fence blockers are the two deliberately exhaustive matches in `crates/topo/src/census.rs` (~1191–1201 and ~2297–2307): a new `ChartRegionError` arm is a compile error there by design.

**Scope for the unit that picks this up:**
1. A typed exhaustion arm (e.g. `WitnessBudgetExhausted { segments, cells }`) threading the two census.rs matches, so the decline says so.
2. The cell-budget reachability fact: exhaustion is reachable *inside* the 128-segment cap (~50 stacked horizontal runs × a tilted crosser exceeds 4096 cells within ~125 segments), so the constant's "a few dozen segments in practice" sizing claim needs either a guard or an honest restatement.
3. Optional, taste: stage 2's ascending-x probe order spends budget on cells far from any overlap; ordering by cell extent would be more exhaustion-robust.
4. An integration-level exercise of the guard if one is reachable through the public doors (the MATE-8 reviews found `ScaffoldAtRest` trips first on the natural constructions — this may only be reachable in principle; say so honestly if so).

## Home

`work/mate/` — MATE-8's own scheduled residue, and the threading site `crates/topo/src/census.rs` is in S-MATE's territory glob.
