---
id: mate-lane-escalations-reach-no-nodes-log
kind: issue
title: The mate lane's escalations reach no node's log: the whole-document solve, and coset's own mint
status: open
opened: 2026-09-20
---


## What

Two escalations the mate lane produces reach a consumer without ever
passing through a node's escalation log. Found by the PROPS
escalation-channel unit (PR 2928), which closed the op-minted family in
`geom-brep` and measured that closing it does NOT close these.

1. **The whole-document solve runs before any node's bracket opens.**
   `editor_core::eval` calls `mate::solve_document` at document level;
   the funnel escalations inside `mate::solve` (`solve.rs`, the
   `mate_clocking_redundant` and sibling sites) therefore land on
   whatever frame the caller happened to leave open, not on a node's,
   and reach `drive::classify_replay` only as
   `NodeErrorKind::Mate(MateFault::Indeterminate)` — which is not one
   of the two arms that read a source, so it takes the `_ => Bisect`
   fall-through and no terminal-sliver test is ever applied to it.
   Pinned today by
   `asm_r2a_mate_solve::row7e_a_mate_solve_escalation_is_on_no_nodes_log_but_visible_in_an_outer_frame`.
2. **`mate::coset` mints an `Indeterminate` of its own** after
   `UnitVec3::new` answers — the `UnitVec3Error::NonFiniteLength` arm
   builds `Indeterminate { margin: Invalid, band, predicate:
   Some("mate_axes_parallel") }` by hand, so the escalation its caller
   receives is on no frame's log even when a bracket IS open. Exactly
   the shape PR 2928 retired in `geom-brep`, one crate over.

## Shape

(2) is a small, local fix with the doors PR 2928 added: the mint
becomes a `k_stats` gate door call, so the funnel records what it
produces. `geom-core`'s `gate_measured` is the nearest fit (a value
that is not a measurement at all); a `UnitVec3` door that escalates
through the funnel rather than through a hand-built payload is the
better one, and is this item's design question.

(1) is the structural half: either a document-level frame the solve
records into, spliced onto the nodes it constrains, or the solve
recorded per mate node. Until it lands, `classify_replay`'s error-enum
arms stay load-bearing for this path (PR 2928's comment in `drive.rs`
says so).

## Refs

Split out of `work/props/escalation-channel-misses-op-minted-indeterminates.md`
family 3, which PR 2928 closed family 1 of.
