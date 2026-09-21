# ATREST — the plan

what tier 3 accepts and refuses: the at-rest validator's holes in both directions

Opened 2026-09-20 by TOPO's priority-seam cut (`work/README.md`, Track
size). Nothing dispatched yet.

## The slate

Carrying **23.5 budget points** of dispatchable work against a ceiling of
30 — about one sitting, which is what the cut was for.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned` | D | A multi-solid body whose one solid is inside-out (negative signed volume) passes tier 3 when the body's total volume is positive — validate pins only the total, never per solid |
| P0 | `check-9-nesting-is-line-bounded-only` | H | check 9's nesting half is silent on every ARC-BEARING outer loop: an annular rim between two circles (every shelled vessel of revolution) still accepts a ring outside its outer loop |
| P0 | `tier-3-does-not-check-shell-roles-per-solid` | D | tier 3 accepts a solid whose shells classify to two Outer boundaries — shell-to-solid grouping is unchecked |
| P0 | `validity-refuses-an-interior-chart-singularity` | H | Validity refuses a revolution-chart face whose loop is rims only (a pole or apex interior to the face) |
| P1 | `validate-tier3-curved-boundary-containment` | H | validate tier 3 — face-boundary containment on curved surfaces (the last unmarked deferral in the not-yet-checked list) |
| P3 | `declared-opposite-orientation-refusal-is-unreached-by-any-row` | E | three planar-door consumers are reached by no row with a reversed planar face — merge_faces's declared-pair rung, join's ring_run_ccw, rest's face_carrier — so dropping the sense at the door survives the suites there |
| P3 | `quadric-datums-unchecked-at-rest` | D | Check 1 names no analytic surface whose stored frame or datum fails to describe a locus - they escalate elsewhere, by accident |

## Order

The two admit-holes that any shelled vessel of revolution hits (`check-9-nesting-is-line-bounded-only`, `an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned`) before the refuse-hole (`validity-refuses-an-interior-chart-singularity`), because a checker that ADMITS something invalid is trusted by every program downstream while one that refuses something valid is merely in the way. `validate-tier3-curved-boundary-containment` is the last unmarked deferral in the not-yet-checked list and closes the set.

## Review posture

OPEN, for this program's first dispatch. TOPO ran the full v6 dual on
kernel units; Ev took S-TCOST off the protocol entirely on 2026-09-12
and protocol v7 (`docs/MODEL-AB-LOG.md`) runs the dual on triaged-in
units only. Nobody has re-asked the question for this ground, so the
first orchestrator answers it here rather than inheriting an answer.
