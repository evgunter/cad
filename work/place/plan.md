# PLACE — placement on a gauge (plan)

Opened 2026-10-02 at EDIT's exit. Recoverable history:
`docs/doc-ledger/edit-leaves-the-tracker.md`.

## Charter

Finish the placement slate in `docs/EDIT-PLACEMENT-SPEC.md`:
- P1 merged (#3497, DR-26);
- P2a merged (#3625);
- P2-core merged (#3676, DR-34);
- P2-split is this program's first unit.

The contract is ASSEMBLY.md A4, A9 and A11 (2)–(5). Ev's rulings on `[ev]` #3437, #3441 and #3505 are the record behind it.

## The slate, in order

1. **P2-split** (`placement-split-and-inline-at-a-gauge-are-refused-until-p2-split`, P1 H):
   - **What it builds:** the gauge hoist, a cut holding a gauge, an inline at an offset over any other part, and a mate-placed instance inlined when its part is one group at the empty chain. These are the interim refusals P2-core left: `CutHoldsGauge`, `NeedsAGauge`, `MatePlaced` and `MateFaceFrameCrosses`.
   - **What it does first:** spec the `## P2-split` section of `docs/EDIT-PLACEMENT-SPEC.md` from the row and from the P2 section's rulings as built.
   - **Open question on the row:** whether a re-spelled `FromFace` side may skip the frame rule's root-at-the-empty-chain condition.
   - The parent row `placement-is-spelled-three-ways-node-registry-and-rule` closes when P2-split merges.
2. **`split-and-inline-refusals-short-of-the-shape-guard`** (P3 E): the split and inline arms that still state no recourse. They are filed by exact id in `refusal_concision_refactor.rs`'s `FILED_NO_RECOURSE`. It rides with P2-split if that unit rewrites those arms.
3. **`document-order-is-read-off-node-id-comparison-since-ids-are-digests`** (P1 M): the sweep of the class whose one known site (`admit_mate`) P2-core fixed.

P3 is the viewer owner's: the group-wide free-move probe and "place where shown". File it on that slate when P2-split merges, or earlier if the owner asks.

## Review posture

P2-split is an L unit, so under `docs/DUAL-REVIEW-PROTOCOL.md` at `7cb05367e` (Ev's ruling on readout 2) it gets a **concurrent Opus pair**, with blinded coding, a fix pass and the DR row as the PR's last commit. EDIT's P1 and P2-core went the same way (DR-26, DR-34).

The two other rows take the orchestrator's read, or a single review if the sweep finds sites.

## Exit shape

Every row merged or re-homed with a reason, and the spec's P2-split section recording what was built. No exit criteria are set, so no walk is owed.
