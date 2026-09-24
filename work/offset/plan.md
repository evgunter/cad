# OFFSET — the plan

the offset lane: the carriers it cannot hold and the folds it calls bounds

Opened 2026-09-20 by SHELL's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**14.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `offset-lane-has-no-conic-carrier` | H | the offset lane cannot carry a hyperbolic edge: a partial-revolve cone's wall meets its moved meridian cap in a conic the kernel has no carrier for (C5 R1) |
| P1 | `doors-still-read-the-whole-body-for-tier1` | D | the simultaneous doors' tier-1 reads are still whole-body: the closing closure check and the asserting setters |
| P1 | `offset-meters-cell-normal-midpoint-direction-is-an-f64-fold` | H | offset_meters: cell_normal's assembly-B direction norm is an f64 fold used as an upper-bound divisor |
| P1 | `topo-transform-rewritten-is-a-hashset-the-crate-doc-argues-against` | E | transform.rs's rewritten set is the one HashSet in topo, and the crate doc spends a paragraph concluding a SecondaryMap would be cheaper and D9-consistent |
| P4 | `offset-axial-predicates-missing-from-the-dimension-audit` | E | docs/predicate-dimension-audit.md lists a third of offset_axial.rs's decide names |

## Order

`offset-lane-has-no-conic-carrier` first: a partial-revolve cone's wall
has no offset carrier at all, which is a shape the kernel builds and
then cannot offset. `offset-meters-cell-normal-midpoint-direction-is-an-f64-fold`
is the soundness row beside it — an `f64` fold standing in for a bound
— and the two are the whole substance of this track.

The three remaining rows are structural and cheap; take them as
drive-bys where you are already in the file.

## Review posture

OPEN, for this program's first dispatch. SHELL inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
