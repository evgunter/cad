---
id: mate-solve-carries-the-cluster-maintenance-half
kind: issue
title: mate/solve.rs holds the D-3 cluster-record maintenance beside the solve, a second concern the module doc gives one bullet
status: open
opened: 2026-09-19
---


(MSOLVE-7 fix pass, 2026-09-19.) `crates/editor-core/src/mate/solve.rs`
is two concerns in one file. The solve proper — reading edges, the
partitions, the per-pair coset fold along the spanning tree, the poses
— is what the module doc is about; the D-3 cluster-record MAINTENANCE
the edit door runs after any edit that moves the mate graph —
`ClusterMaintenance`, `Maintain`, `maintain`, `registry_after`,
`unsolved_because`, `undecided`, `reconcile` — is about 28% of the
file (the `// ---- D-3` region to the end) and gets one bullet
(`reconcile`) in a module doc whose first sentence names the other
five entry points. The two share `SolvedPoses` and `gauge_of` and
nothing else; the maintenance reads the solve's ANSWER and re-keys the
placement registry from it.

Candidate: `mate/maintain.rs`, holding the maintenance half with its
own module doc (what it re-keys, when the edit door runs it, why a
solve with no verdict refuses the edit), `solve.rs` keeping the solve
and the module doc it already has. Not taken in MSOLVE-7: the fence
was the solve's cost and seats, and a file split is a review of its
own.
