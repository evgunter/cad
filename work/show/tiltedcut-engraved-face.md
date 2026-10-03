---
id: tiltedcut-engraved-face
kind: unit
title: the tilted cut's faces carry engraved blind pockets: lettering cut into the round cap, or the elliptical section face if it admits it
status: closed
opened: 2026-10-02
priority: P3
cost: M
pr: 3819
closed: 2026-10-02
---

## What

TANG PR #3748 (ATREST-9's arc-aware `point_in_solid`): a letter-like
tool — lines, a tangent arc, a bulge arc — subtracted into a cylinder's
round end cap now cuts a blind pocket at the closed-form volume, valid
at tier 3, where it refused `SectionLoopMixed` before. Evidence:
`crates/editor-core/tests/pierce_ring_engraving.rs`
(`a_blind_pocket_in_a_cylinder_cap_cuts_to_the_closed_form`; the tool
sliding across the rim is still pinned as a refusal there).

Fold it into `tiltedcut` rather than adding a cell: that cell is a
cylinder cut by a tilted plane, two halves with exact `Ellipse`
section edges. Engrave short lettering (two or three glyphs) into a
face of it:

- **first try the ELLIPTICAL section face** — the more striking part
  (an engraved oval nameplate). It is a planar face bounded by an
  ellipse rather than a circle, which the landed arm may or may not
  cover; one probe settles it.
- if the ellipse refuses, pin that refusal as a live wall probe (the
  `walls::wall` pattern — it panics when it retires), file it on TANG
  with the payload, and engrave the round end cap instead.

## Oracle

Each pocket's volume is its glyph's planar area × depth, exactly (the
glyphs are lines and arcs, so their areas are closed-form); the
scene's existing certified-quadrature bracket for the halves keeps
holding with the pockets subtracted.
