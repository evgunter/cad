# WIREREST — the plan

WIRE's second P3 and P4 tail: a split's plane and seam-vertex residue, a union's fold-step and coincidence-row residue, and naming tests that do not hold at every K.

Opened 2026-10-20 by WIRE's cut. Nothing dispatched.

## The slate

**26 budget points** against 30 (re-weighted tiers, 2026-10-28).

| pri | item | cost |
|---|---|---|
| P3 | `a-halfs-plane-has-two-definitions-and-n3-linking-two-homes` | M |
| P3 | `a-plane-piece-the-nesting-rule-cannot-place-reads-as-bordering-the-exterior` | M |
| P3 | `a-split-merge-of-one-operand-faces-pieces-is-a-merged-of-one` | E +design |
| P3 | `a-split-merging-a-two-piece-wall-reads-its-seam-vertex-as-a-pinch-corner` | M |
| P3 | `a-union-folding-a-bridging-prism-last-refuses-its-fold-step` | H |
| P3 | `a-zip-listed-chord-between-merged-faces-along-a-rim-has-no-end-to-end-witness` | M |
| P3 | `three-names-tests-fail-at-ambiguity-k-1-5` | M |
| P3 | `identical-twin-coincidence-rows-arise-on-coincident-lines` | E |
| P4 | `the-judgement-naming-fault-seam-refuses-other-tests-unions` | E |

Parked, and not counted: `an-order-along-rank-spells-its-groups-size`
(P3 M +design), on WIRE's `a-unions-line-of-several-edges-is-oriented-by-its-least-edge`,
whose build of FORK-ORIENT (nothing is ranked) retires `OrderAlong`.

## Order

- **The split rows come first, as one unit.** These are the first four rows of the slate: the half's plane, the nesting limit, the merge of one, and the two-piece wall. All four were found around PR 5486, and all read `topo::SplitNaming`.
- The D9 rows and the verb-door row moved to WIREREFUSE on 2026-10-28 (`work/wirerefuse/plan.md`).

## Review posture

Inherits WIRE's (`work/wire/plan.md`): protocol v7, single reviews.

**Class note from WIRE's 2026-10-20 sitting:** a row-4 (`unreachable!`) argument must hold for every K > 1 and every ε that `Tolerance` accepts. It must not rest on two independent tolerance decisions agreeing. Review probes with `CAD_AMBIGUITY_K=1.5` and `CAD_TOLERANCE_EPS=1e-6`.
