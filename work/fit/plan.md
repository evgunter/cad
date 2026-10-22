# FIT — the plan

the camera, the display budget and the pick index: what the viewer decides to show and how much of it

Opened 2026-09-20 by VGEOM's priority-seam cut
(`work/README.md`, Track size).

## The slate

**25.5 budget points** of dispatchable work against a ceiling of 30
(re-priced 2026-10-22: legacy `D` rows read and priced, and two rows
parked on the D10 hold).

| pri | item | cost | design | title |
|---|---|---|---|---|
| P0 | `ctrl-wheel-reaches-no-zoom` | M | yes | A ctrl+wheel (and a pinch) reaches no zoom |
| P0 | `zoom-to-fit-frames-no-committed-profile` | M | yes | Zoom to fit frames the mesh only, not committed profiles or previews |
| P1 | `the-budgets-predicted-count-is-not-always-an-over-count` | H | | The display budget is not a cap |
| P1 | `a-flat-rung-pair-is-read-as-flat-below-it` | H | | Flat across one rung step is read as flat below it |
| P1 | `pickindex-merges-parts-on-a-rounded-t-it-never-converts` | M | yes | §3 only: the occlusion probe's tuned `1e-6` |
| P1 | `id-readback-failure-reads-as-nothing-under-the-cursor` | M | | A failed id readback reads as empty space |
| P3 | `a-degenerate-frame-refusal-prints-an-aspect-nobody-can-act-on` | E | | The refusal's sentence; rides with zoom-to-fit |
| P3 | `a-flat-rung-row-uses-an-absolute-epsilon-on-a-scaled-value` | E | | A test's absolute epsilon |
| P3 | `mispaired-ids-exempts-the-empty-window` | E | | `MispairedIds` skips the zero case |
| P3 | `name-of-answers-none-for-both-nothing-and-an-unassigned-id` | M | | `name_of`'s two `None`s |

Parked on the D10 hold (stage 3 D′,
`work/intent/a-placement-pins-by-written-values-without-a-mate.md`,
`[ev]` PR 5494, which retires loose spaces and decides what the viewer
shows): `one-failed-root-refuses-the-whole-pick-index` (P2) and
`a-moved-copys-pick-span-crosses-its-frame-unconverted` (P1, split out
of the pickindex row's §1). Each says why in its own body.

## Order

The two P0s first, concurrently: they are what a person hits in the
first minute. Each carries a product question (the zoom event's
vocabulary and rate; the default framing of an extent-free scene), so
each is weighed by a designer pair before a lane builds it. The
zoom-to-fit lane frames the union of what the viewport DRAWS (mesh,
committed profiles, previews), read off the drawing, never re-deriving
where anything is placed: that keeps it clear of D′. The aspect
sentence rides with it.

Then the budget pair: `the-budgets-predicted-count-is-not-always-an-over-count`
decides whether the budget is a cap, and `a-flat-rung-pair-is-read-as-flat-below-it`
is the same law's other side, so they are one weighing. The id-readback
and `name_of` rows both reshape `idpass::IdAnswer`, so they run as one
lane or in sequence. The pickindex row's §3 is independent.

Neighbours on the same files, worth reading before a lane on
`pick_for`: `work/vgeom/pick-ambiguity-lists-groups-in-id-order.md`,
`work/issues/pick-index-indexes-its-candidates-by-a-number-the-kernel-chose.md`,
and, downstream of `name_of`, `work/vgeom/marks-focus-drops-an-unnamed-patch-from-attribution.md`.

## Review posture

OPEN, for this program's first dispatch. VGEOM inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
