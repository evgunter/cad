---
id: the-interval-charts-least-turn-is-wider-than-eps-at-its-band-edge
kind: issue
title: The Interval mate chart's least turn is about 12ε wide at its band's conditioning edge
status: open
opened: 2026-11-15
priority: P2
cost: M
---

Found by the decided least-turn mint's band-edge rows (fork 270,
`onehome/least-turn-mint`). Pinned by
`crates/editor-core/src/mate/value.rs`'s
`the_interval_charts_least_turn_encloses_its_image_at_its_band_edge`.

**Measured** (ε 1e-9, K 10; an oblique target `d = (0.3, −0.2, 0.9)/|·|`;
the copy direction `−d` tilted by `τ` about an oblique axis across `d`;
the chart's lever is the bundle's arm, 0.5, `Lever::unsourced`):

| scalar | refusal ends at τ | widest `S·n` component, or `\|S·w − w\|`, at the edge | rate |
|---|---|---|---|
| `f64` | 1.97373e-7 | 5.3e-10 (`\|S·w − w\|`) | within `2u/chord` |
| `Interval` | 1.97373e-7 | 1.2e-8 (width), about 12ε | about `22u/chord` |

At 10× and 1000× the edge the Interval width falls as `1/chord`
(9.9e-10, 9.9e-12). The enclosure contains `d` at every tilt; it is
sound and wider than ε.

**Why it matters.** The conditioning edge
(`RigidMotion::least_turn`'s `reach·chord − 2·arm`,
`crates/geom-core/src/linalg/rigid_motion.rs`) is set by the rounding
reach, which holds the `f64` turn within ε there. At `Interval` the
quaternion near the pole compounds its operations' outward roundings, so
the box admitted at the edge is about 12ε wide. Placements that chart
just past the edge carry axes that wide into the copy's pose.

**The consequence, reproduced** (PR 6988's FULL review). A copy point
at arm 0.5 placed by a chart just past the edge is about 5.6ε wide. A
later exact-zero coincidence verdict on it ("is the copy's point on the
target plane?") then reads Undecided from 1× to 2× the edge, where the
same verdict at `f64` decides Zero. So a placement the chart admitted
is refused later, under another decision's story. That is
`[tolerance.recourse]` territory: the refusal names a coincidence
question when the cause is the chart's width at its edge.

**What closes it.** Either the Interval enclosure at the edge within ε
(a narrower spelling near the pole), or the conditioning edge read from
the scalar's own growth rate, so the Interval edge sits where the width
reaches ε. The pinning row's last assertion flips when either lands.
