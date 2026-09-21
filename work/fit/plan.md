# FIT — the plan

the camera, the display budget and the pick index: what the viewer decides to show and how much of it

Opened 2026-09-20 by VGEOM's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**21 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `ctrl-wheel-reaches-no-zoom` | E | A ctrl+wheel reaches no zoom: the toolkit routes it into zoom_factor_delta and the viewport reads only smooth_scroll_delta |
| P0 | `zoom-to-fit-frames-no-committed-profile` | D | Zoom to fit frames bodies only, so a document holding only profiles has nothing to frame |
| P1 | `a-flat-rung-pair-is-read-as-flat-below-it` | H | The display fit reads a body flat across one rung step as flat at every finer delta |
| P1 | `id-readback-failure-reads-as-nothing-under-the-cursor` | D | A failed id readback is reported as an empty cursor, and the chrome blames the picture for it |
| P1 | `pickindex-merges-parts-on-a-rounded-t-it-never-converts` | H | pickindex merges parts on a rounded t it never converts, and slacks it by a tuned 1e-6 |
| P1 | `the-budgets-predicted-count-is-not-always-an-over-count` | H | The display budget is not a cap, and the 1/delta law's two-sided error puts the drawn picture over it |

## Order

`ctrl-wheel-reaches-no-zoom` and `zoom-to-fit-frames-no-committed-profile`
first, and both are small. They are what a person hits in the first
minute of using the viewer, and neither needs a design answer.

Then `the-budgets-predicted-count-is-not-always-an-over-count`, which
is the substantive row: a budget that is not a cap is not a budget,
and `a-flat-rung-pair-is-read-as-flat-below-it` is the same law's other
side. `pickindex-merges-parts-on-a-rounded-t-it-never-converts` is
independent and the only row here on the pick path.

## Review posture

OPEN, for this program's first dispatch. VGEOM inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
