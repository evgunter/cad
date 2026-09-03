---
id: description-staleness-ladder-three-spellings
kind: issue
title: The description staleness/adjacency-coherence ladder has three hand-kept spellings, already drifted - it wants one home
status: open
opened: 2026-08-31
github: 1391
refs: [1378, 1382, 1390, G9]
---

## From GitHub issue 1391

opened 2026-08-31, 0 comments.

(S-BOOL orchestrator) Filed from BOOL-1's dual review ([#1378](https://github.com/evgunter/cad/pull/1378)); both reviewers converged on the class independently. At #1378's frozen head `3f14f3c4`, three sites hand-spell "is this edge description coherent with its adjacent faces / stale":

- `crates/topo/src/splitting/finish.rs:477–489` (the #1378 smooth arm's keep-vs-restate ladder),
- `crates/topo/src/boolean/ops.rs:966–996` (`describe_minted_edges`' smooth arm),
- `crates/topo/src/validate.rs:2199–2218` (tier 3's `DescriptionNotAdjacent` reader).

Measured drift, not hypothetical: the split arm has no `Chart(c) if c.seam` clause where both siblings demand the chart be BOTH adjacent surfaces; the boolean keeps an `Intersection` naming the correct adjacent pair on a smooth edge where the split restates it ("wrong whatever it names") — one of those is wrong per D2; the boolean leaves `Scaffold` alone where the split restates; and the boolean **rebuilds** line carriers (`line_between`) where the split **restates** (#1382 carries that instance). Nothing states that the sites differ or why.

The ask: one home for the ladder (likely beside `EdgeDescription`), with the per-site policy differences either erased or stated as parameters — the S4 one-vocabulary-N-copies shape. Candidate additional copy sites a reviewer named for whoever takes it: `sweep/src/revolve/upgrade.rs:177`, `geom-brep/src/certify.rs:1643` (the other non-empty smooth arms), and #1390's extrude cap-rim arm.

S-BOOL ground (`boolean/`, `splitting/`; the validate.rs reader is Q's G9-adjacent territory) — scheduled behind the BOOL-Q track lanes or as its own unit; #1378's fix pass may close the seam-clause gap locally without waiting for this.

## Home

S-BOOL: two of the three spellings are in `crates/topo/src/splitting/*` and `crates/topo/src/boolean/*`, S-BOOL's territory, and the issue names S-BOOL ground with the `validate.rs` reader as Track Q's `G9`-adjacent seam.
