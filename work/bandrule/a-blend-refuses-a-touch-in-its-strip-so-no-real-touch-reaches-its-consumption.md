---
id: a-blend-refuses-a-touch-in-its-strip-so-no-real-touch-reaches-its-consumption
kind: issue
title: A blend refuses a touch inside its strip, so no real touch reaches the carry's consumption of a support face or a carved span
status: open
opened: 2026-10-12
priority: P3
cost: M
---

## What

A fillet or chamfer whose strip holds another solid's touch refuses
`BlendError::FaceClearanceUncertified` (`fillet3_face_clearance`,
margin about −1.2e-8) before any carry runs. Measured on PR 4809's
branch, each with `R = 0.1` and the other solid's touch inside the
strip:

- two unit cubes kissing along x = y = 1, chamfering one kissing edge:
  refused at the other cube's top face, which shares the
  kissing cube's top plane;
- a cube balanced on its corner at (0.5, 0.05, 1) on a unit cube's top,
  chamfering the top front edge: refused at a face of the tilted cube;
- `a ∪ c` with `c`'s corner resting at (0.05, 0, 0) on `a`'s rim,
  filleting or chamfering `a`'s two edges at the origin: refused at
  a face of `c`.

So the carry's consumption arms for a blend (`Blended::lineage`'s
removed band edges, carved spans and support faces) are reachable only
from bookkeeping records that touch nothing
(`crates/sweep/tests/blend_carries_records.rs`). After the blend the
touch would be gone and the result valid: the record is consumed under
D1.

## Fix

Decide whether the reach screen should exempt another solid's cell that
merely touches the strip (a touch the operand records), rather than
refusing it as a clearance it cannot certify. If it should, admit it,
and add a row per blend consumption arm with a real touch (one is
sketched in each bullet above).
