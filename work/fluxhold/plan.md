# FLUXHOLD — the plan

FLUX's rows on the D10 hold.

Opened 2026-10-10 by FLUX's priority-seam cut (`work/README.md`,
Track size), `blocked` with nobody on it. Under the hold
(`work/flux/log.md`, 2026-10-03), no new unit may meaningfully use
`Measure`/`Assertion`, placement or declared pairs until D10 is
built on that ground.

## The slate

**1 budget point** of dispatchable work against a ceiling of 30.

| pri | item | cost | status | title |
|---|---|---|---|---|
| P2 | `the-box-driver-carries-no-part-resolver` | M +design | parked on [intent-stage3-is-built] | The box driver carries no part resolver, so no drive, and so no stackup, runs over an assembly |
| P4 | `assertion-verdict-derives-partialeq-alone` | E | open | The viewer's hand-written Eq for Asserted rests on an Eq reason that UnevaluatedReason no longer is |

## Order

A row whose trigger fires moves by `git mv` to the live program whose
charter fits, and this track closes when it is empty:

- the box driver to SCALAR (`crates/editor-core/src/drive.rs` is in
  its paths, and its charter names the box driver's readings), or to
  INTENT's own unit if it absorbs the row;
- the assertion row to the viewer program that holds
  `crates/viewer/src/tree.rs` (CHROME or POLISH claim
  `crates/viewer/src/*`).

The rest of the slate has gone: the `Measure` row closed at the cut
(superseded by stage 2 D, PR 4355), the stackup row went to STACK, the
declare-lever row closed with stage 4 F, and the authored-directions
row moved to FRAME on 2026-10-26 beside its sibling there.

## Review posture

FLUX's, inherited: the review tiers of `memories/orchestration-model.md`.

## Exit

The slate is closed or re-homed. No `## Exit criteria`; no exit walk.
