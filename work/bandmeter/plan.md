# BANDMETER — the plan

the blend surgery's ring, cap and strip meters reading wider than they
need

Opened 2026-10-26 by BAND's priority-seam split (`work/README.md`,
Track size). Nothing dispatched.

## The slate

**28.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-ring-of-ellipse-edges-refuses-the-ring-meter` | M | a support ring with an ellipse edge refuses the ring meter |
| P3 | `a-requested-ruled-ring-edge-refuses-at-the-ring-meter` | M | a ruled band's requested edge in a planar support's ring refuses at the ring meter |
| P3 | `blend-support-ring-meters-read-two-ways` | M | the support ring meters read a ring wholly inside the strip as a crossing |
| P3 | `planar-strip-meter-encloses-the-strip-in-its-bounding-rectangle` | M | the planar strip meter reads the strip's bounding rectangle |
| P3 | `support-boundary-meter-bounds-other-carriers-by-the-whole-carrier` | M | the rim support-boundary meter bounds an ellipse, spiric or NURBS edge by a whole-carrier box |
| P3 | `cap-sliver-meter-reads-a-curved-edge-term-by-term` | M | the cap meter reads a circle or ellipse edge one enclosure face at a time |
| P3 | `an-oblique-caps-inside-reading-charges-a-ring-its-major-semi-axis` | M | an oblique cap's inside reading charges a ring its major semi-axis |
| P3 | `a-chord-ends-triangle-could-read-a-cycle-outside` | E | a chord end's exact triangle could certify a cycle wholly outside |
| P3 | `a-cap-slivers-rim-arcs-whole-circle-arms-have-no-cap-row` | M | the cap sliver's rim arcs' whole-circle arms have no cap row |
| P3 | `ruled-cap-meter-reads-a-co-requested-open-link-at-its-stored-place` | M | the ruled cap meter reads a co-requested open link at its stored place |
| P3 | `a-turns-overrun-meters-the-whole-cut-off-sliver` | M | a turn overrun meters the far band's whole cut-off sliver |
| P3 | `an-overruns-sliver-refuses-a-short-near-edge` | M | a turn overrun's sliver refuses a short near edge |

## Order

`ruled-cap-meter-reads-a-co-requested-open-link-at-its-stored-place`
is the one row that could be a missed refusal rather than an
over-refusal (found by reading, not measured), so it is measured
first. The rest pair up by meter:

- **Ring arm (a).** `a-ring-of-ellipse-edges-refuses-the-ring-meter`
  and `a-requested-ruled-ring-edge-refuses-at-the-ring-meter` both
  widen what arm (a) reads; `blend-support-ring-meters-read-two-ways`
  gives the arms the three-way reading the reach and the cap meter
  already have.
- **The cap sliver.** `cap-sliver-meter-reads-a-curved-edge-term-by-term`,
  `an-oblique-caps-inside-reading-charges-a-ring-its-major-semi-axis`,
  `a-chord-ends-triangle-could-read-a-cycle-outside` and
  `a-cap-slivers-rim-arcs-whole-circle-arms-have-no-cap-row` all read
  `CapSliver` in `end_face.rs`.
- **The overrun.** `a-turns-overrun-meters-the-whole-cut-off-sliver`
  and `an-overruns-sliver-refuses-a-short-near-edge` are both
  `turn_plan`'s sliver; BANDRULE's
  `a-feature-inside-a-turns-overrun-is-not-carved` waits on an exact
  reading of the overrun too.

`planar-strip-meter-encloses-the-strip-in-its-bounding-rectangle` and
`support-boundary-meter-bounds-other-carriers-by-the-whole-carrier`
are independent.

## Review posture

Per unit, by the review tiers in `memories/orchestration-model.md`.
Each dispatch names its tier in the log.
