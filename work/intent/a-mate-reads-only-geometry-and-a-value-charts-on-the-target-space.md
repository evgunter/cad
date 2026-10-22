---
id: a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space
kind: unit
title: D′2: WorldRead retires into values; a value names no mate and charts on the target space's axes; a point mate's spin chain; the placement helpers become one pure constructor; inline is one rule; the flatten edit
status: parked
opened: 2026-10-22
priority: P0
cost: H
parent: a-placement-pins-by-written-values-without-a-mate
blocked_on: [a-placement-names-its-target-space]
---



## What

INTENT stage 3, D′2. Spec: `docs/INTENT-STAGE3-SPEC.md` §5′.2,
which governs where this body and it differ. It builds PR 5494's
`[placement.mates]` and `[placement.values]` (rows 190 and 191),
the helpers ruling (`work/intent/log.md`, 2026-10-20) and ASSEMBLY
A4's inline:

- No space has roles: `Target::World` and `WorldRead` retire, and
  each world mate's generator writes values on the world's axes.
- `Value { coord, by }` names no mate; `Coord` is
  `Slide(X|Y|Z) | Spin(X|Y|Z)`. The chart is the target space's
  axes, swung onto the one mated direction where a mate fixes one
  (`chart_body` retires).
- A lone point mate's rotation is a spin chain about the target
  space's axes through the mated point; spins are counted by axis.
- `mate::pin` keeps one pure constructor from the caller's mates and
  moves; `place_at`, `pin_at`, `frame_at`, `place_against`,
  `Anchors` and their heuristics retire; the place-part gesture
  writes the identity bundle.
- Inline mints a `new_space()`, copies the part verbatim, and places
  it under the instance's bundle; `refactor::flatten` is the edit
  that flattens an identity placement. The viewer's mate tool
  removes the values a new mate fixes.

Waits on D′1, because the chart reads the named target.

## Done when

The spec's §5′.2 rows, and tests 20, 21 and 24. Geometry moves only
where a mated target body stands off its space's identity or a
helper's rounded turn is replaced by the turn its caller meant, each
reported by document and node.

**Absorbs:**
`a-value-charted-on-the-target-reads-frame-carries-the-datum-chain`,
`sym-solve-atan2-has-no-branch-rule-about-pi`,
`a-slide-value-doubles-its-translations-width`,
`an-inline-at-a-moved-landing-reading-a-variable-refuses`,
`inline-refuses-a-placed-part-off-the-identity`,
`the-viewer-cannot-mate-two-freshly-inserted-parts`.
