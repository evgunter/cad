---
id: reporting-margin-gate-misses-comparisons-of-two-read-readings
kind: issue
title: The reporting-margin gate's header says equality is counted through mint sites, which a comparison of two classifier readings bypasses; sweep production once and correct the header
status: open
opened: 2026-10-18
priority: P3
cost: E
---

`scripts/gates/reporting-margin-door.sh:18` says "EQUALITY IS THE OTHER
ROUTE, and it is counted too", on the premise that a comparand must be
minted with `MarginDiag::value` or `MarginDiag::enclosure`. That premise is
false: a production site can compare two readings the classifier minted
(`matches!(&read, [Err(a), Err(b)] if a == b)`, RESTREAD's PR 5261 first
head in `crates/topo/src/census.rs` `pair_edge_face`) and decide on the
reporting number with no mint of its own. Two `Indeterminate`s compare equal
whatever their values (Ev's PR 3402 ruling, quoted in the gate). Review
caught the one instance; the gate passed it.

Owed: a one-time grep of production code (outside `#[cfg(test)]`) for
`==`, `!=`, `matches!(… if a == b)` and `assert_eq!` whose operands are
`Indeterminate`, `MarginDiag`, `Decided`, or an error enum that derives
`PartialEq` over a reading (`ValidationError::CensusEscalated`,
`CarrierRefusal`, `MeetFault`), with a disposition per hit and any live
decision on a reporting number fixed through `decide`. Then reword the
header at `:18` to say what the gate counts (mint sites) and name this
route as a known gap. No widening of the gate: a syntactic matcher cannot
tell those operands' types, and its false positives would land on every PR
in the lint job's gates step. If the sweep finds more than one live site,
the type-level route (drop `PartialEq` from the readings, give tests a
pinning comparator) is the durable fix and is filed as its own row.

What would close it: the sweep's hit list with dispositions in the PR body,
and the header at `reporting-margin-door.sh:18` no longer claims equality is
counted.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): One instance ever (PR 5261's first head), and review caught it. A syntactic gate widening to `==` on reading-typed operands cannot see types, so its false positives would land in the lint gates step on every PR. The grain is a one-time production sweep plus a correction to the false header premise at `scripts/gates/reporting-margin-door.sh:18`. A latent decision on a reporting number is P3 (latent unsoundness), not P2. `reform/reporting-margin-gate-misses-comparisons-of-two-read-readings.md`.
