# PROBE — the plan

topo's own guards, probes and generators that cannot go red

Opened 2026-09-20 by TOPO's priority-seam cut (`work/README.md`, Track
size). Nothing dispatched yet.

## The slate

Carrying **13.5 budget points** of dispatchable work against a ceiling of
30 — about one sitting, which is what the cut was for.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `S93` | E | #713's prose-held-invariant sweep minted two new prose-held caller obligations at mev's fan site and kev's fan merge |
| P3 | `census-snapshot-arm1-discard-has-no-justification` | E | census.rs snapshot's arm-1 LoopBoundary discard is the one of three with no written justification |
| P3 | `censussubject-eq-answers-false-for-a-new-variant-against-itself` | E | CensusSubject's hand-written PartialEq has a catch-all arm, so a new variant compares unequal to itself and breaks the Eq it also implements |
| P3 | `orphaned-proptest-corpus-for-seqgen` | E | the tracked seqgen proptest-regressions corpus is orphaned by the row's file move and never replayed |
| P3 | `review-d18-drives-no-mekr-though-it-reaches-link-half-edges` | E | review_d18's hammer drives no mekr, though mekr reaches link_half_edges at twelve splices |
| P3 | `seqgen-proptest-row-logs-no-seed` | E | seqgen's proptest fuzz row draws from entropy and logs no seed, so a green run records nothing |
| P4 | `D20` | D | D5's +46% on the seqgen lane is real and unattributed after #722 — closes on an attribution off hosted CI |
| P4 | `D360` | E | Sweep topo refusal enums by variant name, expecting let-else and matches! shapes (a standing sweep rule) |
| P4 | `bits-witness-slice-workaround-outlived-its-defect` | E | bits_witness takes a slice only to dodge a gate defect that PR 2030 fixed, and its comment cites a work/issues path that no longer exists |
| P4 | `fused-two-shell-body-doc-predates-movefac` | E | fused_two_shell_body says its shape is not publicly constructible, but movefac builds it |
| P4 | `probe-message-carve` | E | review_d18_probes carves an unreachable! message with a line window and the first ')', not a balanced read |
| P4 | `review-d18-probes-header-miscounts-its-own-rows` | E | review_d18_probes.rs's header says four rows and the file carries five |

## Order

By cost, not by subject: eleven of the twelve are class `E` and several are a single commit. Take them as drive-bys where you are already in the file (`work/README.md`, "The tracker is not comprehensive") and dispatch the remainder as one batched unit rather than one PR each. `D20` is the only `D` row and is an attribution question, not a fix.

## Review posture

OPEN, for this program's first dispatch. TOPO ran the full v6 dual on
kernel units; Ev took S-TCOST off the protocol entirely on 2026-09-12
and protocol v7 (`docs/MODEL-AB-LOG.md`) runs the dual on triaged-in
units only. Nobody has re-asked the question for this ground, so the
first orchestrator answers it here rather than inheriting an answer.
