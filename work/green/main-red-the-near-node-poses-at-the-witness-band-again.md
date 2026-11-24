---
id: main-red-the-near-node-poses-at-the-witness-band-again
kind: issue
title: main red: sweep bisector_nodes::the_near_node_poses_at_the_witness_band is red again (its 2026-11-14 row is closed)
status: closed
opened: 2026-11-23
priority: P1
cost: M
refs: [main-red-a-near-node-cap-at-1e-8-refuses-on-quadrature]
closed: 2026-11-23
pr: 7678
---


## The finding

(Filed by the WALKS orchestrator, 2026-11-23.) Hosted CI on PR 7477 (`walks/refusal-split`, head 3befc445, test (kernel), job 137448075289) panics at `crates/sweep/tests/bisector_nodes.rs:636` in `bisector_nodes::the_near_node_poses_at_the_witness_band`. PR 7475's FULL reviewer reproduced the same red on that PR's merge base (origin/main f1ca5d1b) with the same symptom, so it is main's, not either PR's. The row that last fixed this test, `main-red-a-near-node-cap-at-1e-8-refuses-on-quadrature`, closed 2026-11-14; the test file was last touched by GERM on 2026-11-20 (`77fe49a98b`, `f6c4c19a46` on `germ/sector-side-order2`), the first place to bisect.

## What a fix is

Bisect main's merges since 2026-11-14 to the merge that re-reddened it; the program whose change did is owed the fix.

## Closed (2026-11-23, GREEN U19, PR 7678)

**Bisected:** PR 7354 (`sectcone/conic-pairs-on-the-value`, merge
`4a94f7109b`). The row is green on its first parent `5ea1ac0a70` and red
on `4a94f7109b` (probe runs 45312428995 and 45314894647). It is not
GERM's 2026-11-20 edit to the file.

**Class: a band-edge reading moved, not a regression.** Probe run
45317834459 printed every δ = 1e-8 cell on `5ea1ac0a70`, `4a94f7109b` and
main `a433ff83d6`. Four single-cap cells changed: A's cap direct (B, A) ∪,
and B's cap direct (A, B) ∪ ∩ ∖. Before, each was a containment
escalation or a build. Now each refuses on the rim arc's own
transversality to the partner's wall, `CertifyError::Escalated {
TransversalityArm, dihedral_arm_wedge }`, at 9.99999991e-9 and
9.99999997e-9 against the escalation edge 1e-8. The dihedral is δ, so
this is the edge the test already says a single cap "lands either side
of by its rounding", now read by another door. Every other cell, and
every δ = 1e-6 and 1e-4 cell, is unchanged. Two builds were lost (B's cap
direct (A, B) ∩ and ∖), and SECTCONE's log says so.

**Re-pin** (`crates/sweep/tests/bisector_nodes.rs`): `RIM_IN_BAND` names
the four cells, and `rim_transversality_in_band` holds each to that
refusal with its margin in (9.9999999e-9, 1e-8]. The re-based and the
minted carrier are both accepted. **Mutant:** the pinned file on
`5ea1ac0a70` is red, because A's cap direct (B, A) ∪ is a containment
escalation there (probe run 45330776592).
