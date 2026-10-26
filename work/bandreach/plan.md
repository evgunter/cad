# BANDREACH — the plan

predicate 2 reading wider than it needs: the blend's reach meter and
its support screen

Opened 2026-10-26 by BAND's priority-seam split (`work/README.md`,
Track size). Nothing dispatched.

## The slate

**18.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `blend-reach-meters-a-nurbs-face-on-its-box-alone` | M | the reach has no surface distance for a NURBS face, so a loft wall whose box meets a band refuses |
| P3 | `blend-reach-takes-an-open-arc-link-over-the-whole-turn` | M | an open arc link's reach is taken over the whole turn |
| P3 | `blend-reach-refuses-a-bore-clear-of-a-ruled-cut-offs-sliver` | M | the reach refuses D-rod bores that stay clear of the sliver the cut-off removes |
| P3 | `the-cross-chain-excuse-misses-an-oblique-cell-past-a-windows-end` | M | the cross-chain excuse misses an oblique cell past a link's window end |
| P3 | `blend-reach-never-meters-the-bands-of-one-chain-against-each-other` | M | the reach never meters two bands of one chain, or of two chains that meet, against each other |
| P3 | `the-strip-screen-reads-a-supports-vertices-not-its-edges` | M | `reach::screened` reads a support's vertices alone |
| P3 | `the-support-screen-refuses-a-feature-across-the-edges-line` | M | the support screen refuses a feature across a requested edge's line, where the band never reaches |
| P3 | `support-screen-skips-adjacent-boundary-features-wherever-they-approach` | E | the support screen skips adjacent boundary features wherever they approach |

## Order

Three rows say the meter may excuse what it never reads, and they
come first, each measure-first:
`blend-reach-never-meters-the-bands-of-one-chain-against-each-other`
(no door reaches it while every chain is straight),
`the-strip-screen-reads-a-supports-vertices-not-its-edges` (a curved
edge can dip into the strip between samples), and
`support-screen-skips-adjacent-boundary-features-wherever-they-approach`
(a curved neighbour that comes back). The two screen rows read one
premise and may share a unit with
`the-support-screen-refuses-a-feature-across-the-edges-line`. The
reach's over-refusals (NURBS box, whole turn, ruled cut-off bore,
oblique cell) are independent. The NURBS row wants the boolean's
certified NURBS-box machinery.

## Review posture

Per unit, by the review tiers in `memories/orchestration-model.md`.
Each dispatch names its tier in the log.
