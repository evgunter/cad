---
id: a-loft-section-corner-over-a-box-wider-than-eps-refuses-in-the-section-gate
kind: issue
title: A loft whose end section has a corner moving over a box wider than ε refuses in the section gate on chord_side, before any rim exists
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Found by designer C on the edge-span design fork (fork record on
`analysis/design-fork/edge-span`, ONEHOME). A loft whose end section has
one corner moving over a box wider than ε (half-width `w`) refuses at
`Interval` in the section gate, before any rim exists:

```text
Skin(SectionProfile { gate: ChordSide, predicate: "chord_side" })
```

with margin `±1.005·w`, on the two segments that share the moving
corner. The refusal is raised where each section validates at `T`
(`crates/sweep/src/skin.rs:1411`, `validate_sections`, and its guided
twin at `crates/sweep/src/skin.rs:1456`): the chord-side reading of the
corner's two segments is as wide as the corner's own box, so it cannot
leave the band once `w` exceeds it.

The rows that do run a moving corner at `Interval`
(`crates/sweep/tests/common/enclosure.rs:16`, `box_about`) take a box
`ε/256` either side of it, so none reaches this case. What the gate
should read for a section whose vertex is wider than the band — a
corner that is a corner at every value in its box, rather than one sign
of one chord — is the open question; until it is settled a box wider
than ε on a section corner is refused, typed, at the door.

The fixture the designer ran is not recorded on this row; the first
step is a pinned row that reproduces the refusal at the default ε.
