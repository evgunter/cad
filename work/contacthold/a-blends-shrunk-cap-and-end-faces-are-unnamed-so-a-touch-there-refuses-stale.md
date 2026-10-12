---
id: a-blends-shrunk-cap-and-end-faces-are-unnamed-so-a-touch-there-refuses-stale
kind: issue
title: A blend's shrunk cap and end faces are unnamed, so a touch on the part it cut off refuses stale
status: open
opened: 2026-10-12
priority: P3
cost: M
refs: [the-shells-retired-seams-slits-and-apexes-map-to-every-outer-rim-face]
---

## What

`Blended::lineage` (`crates/sweep/src/blend/build.rs`) states a fate
for the cells `BlendNaming` names: the band's edges and corners
removed, a carved source edge divided into its remnants, a support face
divided into itself and its trimlines and feet. At a cut-off the blend
also shrinks the cap and end faces it trims, and `BlendNaming` names
none of them, so their fate is `Kept` under their key. A record whose
touch lies on the part of such a face the blend cut away lands on no
pair, and with nothing stated removed the carry refuses it
`CarryRefusal::Stale` where D1 consumes it.

The refusal is loud, never a drop, so this is a missing statement, not
a wrong answer.

## Fix

Have `BlendNaming` name each face the blend trims (the cut-off's cap and
end faces), and state each in `Blended::lineage` as divided into itself
and the trimline, its remainder removed. Row: a rest on the cut-off part
of a cap face is consumed, and the result passes at rest.
