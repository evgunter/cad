# WIREREST — the plan

WIRE's second P3 and P4 tail: a split's plane and seam-vertex residue, a union's fold-step and coincidence-row residue, and naming tests that do not hold at every K.

Opened 2026-10-20 by WIRE's cut. Nothing dispatched.

## The slate

**29 budget points** against 30 (re-weighted tiers, 2026-10-28).

| pri | item | cost |
|---|---|---|
| P3 | `a-halfs-plane-has-two-definitions-and-n3-linking-two-homes` | M |
| P3 | `a-plane-piece-the-nesting-rule-cannot-place-reads-as-bordering-the-exterior` | M |
| P3 | `a-split-merge-of-one-operand-faces-pieces-is-a-merged-of-one` | E +design |
| P3 | `a-split-merging-a-two-piece-wall-reads-its-seam-vertex-as-a-pinch-corner` | M |
| P3 | `a-union-folding-a-bridging-prism-last-refuses-its-fold-step` | H |
| P3 | `a-zip-listed-chord-between-merged-faces-along-a-rim-has-no-end-to-end-witness` | M |
| P3 | `three-names-tests-fail-at-ambiguity-k-1-5` | M |
| P3 | `a-unions-member-edge-group-size-reads-the-fold-steps-spelling` | M |
| P3 | `identical-twin-coincidence-rows-arise-on-coincident-lines` | E |
| P4 | `the-judgement-naming-fault-seam-refuses-other-tests-unions` | E |

Parked, and not counted: `an-order-along-rank-spells-its-groups-size`
(P3 M +design), on WIRE's `a-unions-line-of-several-edges-is-oriented-by-its-least-edge`,
whose build of FORK-ORIENT (nothing is ranked) retires `OrderAlong`.

## Order

- **The split rows come first, as one unit.** These are the first four rows of the slate: the half's plane, the nesting limit, the merge of one, and the two-piece wall. All four were found around PR 5486, and all read `topo::SplitNaming`.
- The D9 rows and the verb-door row moved to WIREREFUSE on 2026-10-28 (`work/wirerefuse/plan.md`).

## Review posture

Inherits WIRE's (WIRE's plan, recoverable by `docs/doc-ledger/wire-leaves-the-tracker.md`): protocol v7, single reviews.

**Class note from WIRE's 2026-10-20 sitting:** a row-4 (`unreachable!`) argument must hold for every K > 1 and every ε that `Tolerance` accepts. It must not rest on two independent tolerance decisions agreeing. Review probes with `CAD_AMBIGUITY_K=1.5` and `CAD_TOLERANCE_EPS=1e-6`.

## WIRE's remainder, moved in 2026-11-03

WIRE closed with eleven P3 rows and nothing above them
(`docs/doc-ledger/wire-leaves-the-tracker.md`); they moved here by
`git mv`, ids and bodies unchanged, with WIRE's `paths`. None counts
towards the load, which only weighs P0, P1 and unbanded rows.

| item | cost |
|---|---|
| `a-boolean-records-a-face-pair-decision-only-where-the-two-faces-meet` | E +design |
| `a-boolean-zero-decided-and-consumed-in-place-reaches-no-row` | M |
| `a-split-records-an-on-vertex-only-where-it-leaves-two-runs` | E +design |
| `a-split-lineage-lists-a-joined-edge-over-chords-and-an-operand-rim-as-minted` | M |
| `a-splits-joined-seam-and-set-piece-keys-have-no-tie-witness` | E |
| `a-joined-set-over-two-tied-operand-edges-loses-its-operand` | M |
| `a-flush-pairs-rim-point-is-spelled-two-ways-by-fold-order` | M |
| `a-unions-tied-fold-rows-at-a-member-corner-bypass-the-corner-rule` | M |
| `union-pairwise-verdicts-that-disagree-through-lineage-refuse-as-a-kernel-defect` | M |
| `an-assembles-contact-carry-raises-emission-for-a-bridge-the-graft-keeps-total` | M |
| `a-crossings-sense-residue-left-in-the-emitters-and-the-remap` | E |

The two `+design` rows are one question: whether a decision a boolean
or split takes and uses in place is a published row. Weigh them
together before building either.
