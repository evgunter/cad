---
id: a-widened-rotation-angle-is-unmeasured-on-the-certified-lane
kind: issue
title: the chain's certified picture: a widened rotation angle through Node::Transform has never been measured on Interval or Sym<Interval>, and the derived-frame walls stand between the chain and an enclosure per link
status: open
opened: 2026-09-22
priority: P1
cost: D
---


## What

**Specifically requested by Ev, 2026-09-22 (P1):** the certified
version of the chain demo (`SYM-14`) — an enclosure per link, growing
down the chain, beside the advisory cloud. What stands between the
chain and that picture, as surveyed at the unit's cut:

- **A widened rotation angle has never been measured anywhere in the
  tree.** The only widened `Node::Transform` is a widened TRANSLATION
  with `rotation_angle: ang(0.0)`
  (`crates/editor-core/tests/m10_derived_frame_interval.rs`, the
  `Transform` row). An interval angle puts interval `sin`/`cos` into
  the rotation matrix — unmeasured on `Interval` and on `Sym<Interval>`.
- **The derived-frame walls are live**: a frame whose axes carry a
  widened parameter froze on degree (`derived-frame-placement-freezes-on-the-symbolic-lane`,
  answered per family by rules E, F and G — the tilt-`u` Newell wall
  closed with DECIDE-3 on `props/sign-hull`, `tiltUV` still moves not
  one count), a `FaceFrame` on a revolved cap refuses
  (`a-face-frame-on-a-revolved-cap-refuses-on-pcurve-loop-continuity`),
  and at half-width `5e-2` clause 1 refuses
  (`work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`).
- **Cost**: two stacked derived frames were 219 s per replay with rule
  E off and 1.1 s on; four links over four box axes is a large drive
  against the plate's 512-leaf cap.
- **Not a wall, noted**: the assembly placement lane
  (`placement::Frame`, `DocEdit::SetPlacement`) stores numbers and
  cannot carry a parameter at all; the chain is expressed through
  `Node::Transform` or a `Datum::Frame`, which can.

## What answers it

SYM-14's Phase 2 table: one leaf at `Interval` and `Sym<Interval>` for
chains of 1–4 links at the nominal box (certifies / refuses / the first
refusing predicate / cost), the drive at a small budget, the widest
box that certifies whole. This row CLOSES if the certified lane reaches
the four-link tip's assertion at any box (the enclosure drawn on the
sheet); otherwise it carries the walls' names, each filed as its own
row at P1 with this request, and stays open.

## Home

SYM — the tier's reach on a document Ev asked for; the walls it names
are the tier's (SYM/DECIDE) or PROPS' (the frame family) by their
subject.
