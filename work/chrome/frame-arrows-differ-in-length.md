---
id: frame-arrows-differ-in-length
kind: issue
title: A frame's two arrows differ in length and size, which reads as unbalanced (Ev-requested, high priority)
status: closed
opened: 2026-09-17
branch: chrome/frame-arrows
pr: 2856
closed: 2026-09-19
---

## Ev's note (verbatim)

> why is it that on a plane the two arrow marks are different lengths and sizes? i guess it makes sense to show the handedness in some way, but this way looks weird and unbalanced. certainly the length (from the origin) should not differ; maybe the size could, but maybe one could be doubled or something instead

## Priority

**High priority — requested directly by Ev** (in chat, 2026-09-17, from
Ev's own list of UI nits). This row goes ahead of the rest of the
program's order; see the plan's *Ev's requests* section.

## Where it lives

`datums.rs`: `FRAME_ARM_PX` (the +x arrow, 108 px) and
`FRAME_Y_ARM_FRACTION` (0.62, so +y is about 67 px). The barbs are a
fraction of each arrow's own length (`FRAME_BARB_FRACTION`), so the +y
head is smaller too. The comment on `FRAME_Y_ARM_FRACTION` gives the
reason: a grid is symmetric under a quarter turn, so the arrows have to
say which one is x.

## What a fix has to decide

- Both arrows the same length from the origin. Ev: "certainly the
  length (from the origin) should not differ".
- How handedness shows instead. Ev suggests a differently-sized head,
  or doubling one of the heads, for example a double arrowhead on +x.

The comment is a code comment, not a ratified README clause (checked:
`crates/viewer/README.md` and `docs/DESIGN.md` say nothing about frame
arrows), so the fix can rewrite it in place.

`datums.rs` is CHROME's by the 2026-09-15 carve-out, which is why this
row is filed here.

## Closed

Landed by PR 2856 (`chrome/frame-arrows`): both arms run `FRAME_ARM_PX` (44 px, down from 108) from the origin; +x carries a doubled head, +y a single one (`FRAME_HEADS`); the arm is held inside the first grid cell by a const assert against the floor derived from `PITCH_STEPS` (`LADDER_STEP`) and a runtime sweep of `grid_pitch`. `sketch::TIP_MARK_PX` (profile heading arrows, 20 px) was left as is and raised with Ev in chat.
