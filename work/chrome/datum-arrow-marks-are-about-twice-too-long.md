---
id: datum-arrow-marks-are-about-twice-too-long
kind: issue
title: Datum arrow marks are about twice as long as they need to be (Ev-requested, high priority)
status: open
opened: 2026-09-17
---

## Ev's note (verbatim)

> can the arrow marks generally be a bit smaller? they're like twice the length they need to be

## Priority

**High priority — requested directly by Ev** (in chat, 2026-09-17, from
Ev's own list of UI nits). This row goes ahead of the rest of the
program's order; see the plan's *Ev's requests* section.

## Where it lives

`datums.rs`: `FRAME_ARM_PX = 108.0`, which is deliberately longer than
one grid cell (`TARGET_PITCH_PX = 80`) so that the arrowhead sits past
the first grid crossing. Halving it puts the head inside the first
cell. So the fix has to decide whether that crowding matters once the
grid is thinner or fainter (see VGEOM's
`datum-grid-lines-are-too-prominent-and-cover-profile-lines`), or
whether the head needs another way to stand clear of the ruling.

Related marks that "arrow marks generally" may also cover:
- a plane's normal tick, `NORMAL_TICK_PX = 46`
- the profile preview's heading arrows, which are sized off
  `sketch::TIP_MARK_PX = 20` since #2829

Pairs with `frame-arrows-differ-in-length`. Both change the same two
constants, so one unit should take both.
