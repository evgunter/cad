---
id: interior-witness-budget-decline-untyped
kind: issue
title: interior_witness budget exhaustion: type the decline (it currently re-raises the carried refusal, indistinguishable from a thin overlap)
status: review
opened: 2026-09-01
github: 1478
refs: [1472, 1435]
branch: fix/census-typed-declines
pr: 1750
---

## From GitHub issue 1478

Opened 2026-09-01; 0 comments.

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

## Closed

`ChartRegionError::WitnessBudgetExhausted { segments, cells }` exists,
on `RayExhausted`'s shape and with its `Display` contract (the named
quantity, then the recourse). `interior_witness` and
`decomposition_witness` return a typed `WitnessOutcome` —
`Certified` / `Declined` / `BudgetExhausted` — instead of a `bool`, and
`declared_pair_overlap` spells the third as the new refusal rather
than leaving the carried `TouchingBoundary` standing. Both caps
answer it: the segment cap with `cells: 0` (nothing was looked at),
the cell cap with the probes spent. The two deliberately exhaustive
matches in `census.rs` carry the new arm; so does
`tests/census_g2_carrier.rs`'s variant-name match, which is a third
exhaustive site the item did not name.

Scope item 2 — the sizing claim. Re-derived rather than inherited:
a simple comb of 28 teeth (56 stacked horizontal runs, 114 segments)
against one thin tilted crosser (4 segments) is 118 segments, under
the 128-segment cap, and its arrangement overruns 4096 cells at
probe 4097. The row `r2p7_cell_budget_is_reachable_inside_the_segment_cap`
builds that pair and reads both numbers off it, so `WITNESS_BUDGET`'s
doc now cites an executed row instead of an arithmetic sketch. The
filer's own figures (~50 runs, ~125 segments) are in the same place
and are true; they are simply not the tightest ones.

Scope item 3 (stage 2's ascending-x probe order) is deliberately NOT
taken — taste, and out of this unit.

Scope item 4 — reachability through the public doors. Not achieved,
and said so rather than manufactured: the MATE-8 reviews' finding
stands, `ScaffoldAtRest` trips first on the natural constructions, and
nothing in this unit made the guard reachable through `assemble`. What
is pinned is unit-level and mutation-verified: reverting either cap to
an untyped decline reds exactly `r2p5` and `r2p7` and nothing else.

**Swept for** the shape *a schedule or budget exhaustion spelled as a
bool, an untyped early return, or a re-raise of a carried refusal* —
every `BUDGET`/`_CAP`/`MAX_*`/`SCHEDULE` constant in `crates/*/src`,
every comparison of a counter against a screaming-case constant, and
every literal-cap guard. `WITNESS_BUDGET` was the only untyped one:
`drive.rs`'s `BudgetKind`, `clearance.rs`'s `CellBudget`,
`parts::MAX_DEPTH`, `step-import`'s `CONVERSION_DEPTH_LIMIT` and the
`RayExhausted` family all already answer a named refusal.

**What the sweep could NOT match**: a guard whose cap is an inline
literal compared against something other than a bare local (the
literal-cap pattern is anchored on `if <ident>[.len()] <op> <digits>`),
a budget carried as a struct field or function parameter rather than a
constant, and — the real blind spot — an exhaustion that is honest at
its own site but flattened by a CONSUMER. This unit's own change is an
instance of that: `census.rs` maps `WitnessBudgetExhausted` onto
`CensusUnsupported` exactly as it maps `TouchingBoundary`, so the
distinction is legible to a caller of the public chart-region door and
still invisible to a caller of the census. Widening the census's own
refusal vocabulary is a separate question and is not taken here.
