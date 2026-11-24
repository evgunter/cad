# Design fork: what an undecided at-rest decision asks of the symbolic tier, and what the ask costs when the answer is no (BOXFINISH)

Protocol: ddd201f411f (`docs/DESIGN-FORK-PROTOCOL.md`, the last commit on `origin/main` touching it at dispatch). Item: `work/boxfinish/an-undecided-at-rest-decision-climbs-the-ladder-on-a-margin-that-is-not-zero.md` (P1, H, design), on `boxfinish/orchestrator` at dispatch. Whether it goes to Ev depends on whether the answer changes E12 or another ratified decision.

## Draws (2026-11-24, 09:14 UTC, at dispatch)

| Byte | Value | Meaning |
|---|---|---|
| arm | 170 | 86 or above: TWO designers (one Fable, one Opus) |
| labels | 57 | odd: Fable is A, Opus is B |

Mapping: **A = Fable 5.2, B = Opus 5.5**. Versions as configured at dispatch; confirmed from each session's served model when the reports are in.

The statement of the problem (identical for both) is `probe/briefs/designer-the-climb.md` on `boxfinish/briefs` at `416eb1b3c5d`; it names no candidate solution. Exposures to a candidate, as hypotheses: the item and M11 report that leaving the tier unasked on four named margins changes no verdict (a scratch substitution, not a proposal).

## Sessions (dispatched 09:16 UTC, concurrently, one statement of the problem; checkout `boxfinish/orchestrator` at `cbc9453b153`)

| Label | Model (configured) | Session |
|---|---|---|
| A | Fable 5.2 (`claude-fable-5-2`) | session_013QoYWTzB9rBLberBxrJ5dT |
| B | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXFZDL2yMboFxQ7x7a |

## First reports (recorded as delivered, before any reconciliation; both in by 09:25 UTC, raw on `boxfinish/briefs` `probe/briefs/fork-the-climb/` at `e14ae2f4cb5`)

| | A | B |
|---|---|---|
| Answer | Give "no" a proof of its own, from the value side: where the box reading shows no definite sign at an exact-witness symbolic scalar, read the margin's expression once more at one named off-centre point of the leaf's box in outward-rounded intervals. A definite sign there proves the margin is no identity; the tier is asked nothing and the decision stays undecided. Otherwise the ladder climbs as today. A graph that reaches a value with no expression declines. | Before the tier builds any form, enclose the margin's graph with each parameter at one fixed off-centre point of the leaf's box (and each value with no expression at one point of its own). An enclosure that excludes 0 proves no rung can answer; the decision stays undecided with no climb. Otherwise the ladder runs as today. |
| One line of argument | The tier cannot produce a "no": a residual form that is not the zero polynomial proves nothing, so the cost is unbounded by construction; the missing part is a refuter, not a cheaper ladder. | The ladder cannot say "no" at all; a zero form is zero at every point of the box, so one certified nonzero point refutes it. |
| Confidence | likely (the argument sure; that the point reading refutes these four margins unmeasured) | likely |
| Rejected the framing | In part: the ask has no "no" to cost. | In part: the same. |
| Proposed changing ratified text | Yes: one sentence added to E12's "The two channels are checked where an answer is returned". | Yes: one sentence added to E12's "identity test, on demand" bullet. |

**Agreed at first report on the recommended final state** (a point refutation ahead of the ladder; no verdict changes; nothing negative kept per drive; the structural question untouched; measure first). They differ in two details an implementer's measurement settles: what a value with no expression does at the point (A: the reading declines; B: any one point of its own value is admissible, pending a check of how the form treats it), and which E12 bullet carries the sentence. No reconciliation round is needed.

Served models at first report: not yet read from the sessions.

## To Ev (09:34 UTC)

`[ev]` PR 7698 (`boxfinish/ev-refute-before-climb`): the agreed recommendation, with one sentence added to E12 as the diff; both `For Ev` sections verbatim, labelled A and B. Row 7698 in `docs/DESIGN-FORK-LOG.md` (the recommendation half) is on that branch. The mapping above is filled into the row once Ev decides.
