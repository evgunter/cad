# MEASUREDOOR — the plan

Opened 2026-10-27 by the re-cut of FLUX and FLUXTAIL. Together they
held 95.5 budget points against 30, and both shared
`props/curved.rs` and `props/spiric.rs`, so their orchestrators
could not run side by side. The rows were re-cut by logical thread
instead of by priority, so that the five successors can run in
parallel: each mostly owns its own files, and where two share a file
they work on different functions of it
(`docs/doc-ledger/flux-leaves-the-tracker.md`). Nothing dispatched.

## The slate

**13 budget points** of dispatchable work against a ceiling of 30, in 7 rows (2026-10-27).

| pri | item | cost | title |
|---|---|---|---|
| P1 | `main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket` | M | Main red (slow set): two reach_volume_backstop rows fail since PR 5734's exact cylinder-sliver bracket |
| P3 | `a-finished-body-is-validated-again-by-an-explicit-tier-3-assert` | M | about 380 test functions finish a body and also assert validate_geometric, so tier 3 runs twice where they read one body; the asserts right after finished cannot fail |
| P3 | `a-pinched-seam-region-has-no-witness` | M | check 6 leaves a pinched seam region unread, and no body reaches that branch |
| P3 | `an-op-output-validated-at-its-last-act-drops-its-verdict` | M | topo::shell and the STEP reader validate the body they hand out, then hand it out as a plain Body, so every reader pays the gate again |
| P3 | `shell-classification-volume-refusals-end-as-a-kernel-defect` | E | topo: classify_shells ends a zero, straddling or undecided shell volume as a kernel defect, over a caller's body |
| P4 | `the-cover-census-may-subsume-the-seam-regions-pole-read` | E | The cover census catches every seam-region flip the pole read does on the flip corpus; retiring the pole read is Ev's, since it rewrites C12 (7) |
| P4 | `the-door-reads-each-seam-members-loops-three-times` | E | The measurement door parses each sphere seam member's loops three times: its own side, the region's, and its flux arm's |

## Order

`main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket` is a main red and goes first. Decide whether PR 5734's narrower readings are right, then re-meter `last_round_open` and re-pose the midpoint fixture, or fix the bracket. CLEAVE has the backstop open on PR 5757 and PR 5780, so check those before building. The two verdict rows (`an-op-output-validated-at-its-last-act-drops-its-verdict`, `a-finished-body-is-validated-again-by-an-explicit-tier-3-assert`) are one design: a validated body type and the asserts it makes dead. WIRE's `a-body-value-consumed-by-k-nodes-is-gated-at-rest-k-times` asks the same question from the editor's side. The seam-region rows batch by file. `the-cover-census-may-subsume-the-seam-regions-pole-read` is Ev's to rule, since it rewrites C12 (7).

## Review posture

FLUX's, inherited: the review tiers of `memories/orchestration-model.md`
(orchestrator's read, single style or full review, or a dual under
`docs/DUAL-REVIEW-PROTOCOL.md`, logged in `docs/DUAL-REVIEW-LOG.md`),
named with its reason at dispatch. Expect a dual on a row that changes
what the kernel will certify over a family: a wrong closed form is the
error that reads as an improvement.

What PROPS and FLUX learned, stated so that briefs carry it:

- **A spec that asserts a premise hands the unit a defect.** One
  premise about `du_of_rims` failed three times running in PROPS, the
  last time on main. A spec here requires a decide.
- **A re-baselined golden is not a re-pointed row.** Check what a row
  witnesses before moving its constants: its subject can move under a
  change while its numbers stay movable.
- **A narrower bound is the expected outcome of a width fix, and
  therefore not evidence that the fix is correct.** Containment is.
  Test against exact arithmetic, with cases chosen to break it.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
