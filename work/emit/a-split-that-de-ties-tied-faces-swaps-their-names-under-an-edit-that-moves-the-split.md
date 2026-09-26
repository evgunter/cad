---
id: a-split-that-de-ties-tied-faces-swaps-their-names-under-an-edit-that-moves-the-split
kind: issue
title: A split that de-ties two tied faces names each piece against the split's walls, so an edit that moves the split to the other face swaps the names
status: open
opened: 2026-09-26
priority: P2
---


## What

Two tied faces are one parent in a union. When a member splits one of them,
each face gets a `Fragment(SideOf)` against the splitting member's walls, and
N2's narrowing makes each vector a unique name. The vectors say where a face
lies relative to the walls, not which of the two tied faces it came from. An
edit that moves the split onto the other tied face therefore hands the same
names to the other face.

This is pre-existing: main spells the unsplit face differently, but it swaps
the same way.

## Evidence

The fixture is `tied_prongs` in `crates/editor-core/tests/emit_union_dividing.rs`
(a 4×4×4 block with two slot ceilings that are one tied face of the fork),
united with a bar at x 2.9..3.1, y 0.8..1.7, z 2.5..3.5, behind a transform.
The bar crosses the lower ceiling and splits it. A `SetParam` on the
transform's y translation (+1.5) moves the bar across the upper ceiling
instead. Both member orders behave the same. Face centroids, by published
name:

| name (after the tied parent) | before the edit | after the edit |
|---|---|---|
| `SideOf[Positive, Negative]` | lower ceiling, x > 3.1 (y 1.25) | upper ceiling, x > 3.1 (y 2.75) |
| `SideOf[Negative, Positive]` | lower ceiling, x < 2.9 (y 1.25) | upper ceiling, x < 2.9 (y 2.75) |
| `SideOf[Mixed, Mixed]` (PR 3241) / the bare tied name (main at 2c15147b63) | upper ceiling (y 2.75) | lower ceiling (y 1.25) |

The pieces are sided in `emit_union::name_by_parents` (emit_union.rs:1350)
against the partners `emit_union::dividing` returns (emit_union.rs:1677). On
main, the fold's pair step sides them in `emit_topo::name_fragment_group`
(emit_topo.rs:846).

## Why it matters

Two names that denoted the lower ceiling's pieces now denote the upper
ceiling's, and the whole-face name moves the other way. Nothing reports it,
and a reference follows the name to the other face.

## Fix direction

The tie was the honest answer: nothing covariant tells the two ceilings
apart. A split that de-ties them has to either keep its pieces tied, as N2's
tie propagation would, or discriminate them against something that follows
the tie's candidate rather than the split's walls. Which one is a design
call on N2's tie rule, so it may need Ev.
