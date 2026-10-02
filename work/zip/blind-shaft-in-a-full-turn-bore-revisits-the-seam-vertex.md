---
id: blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex
kind: issue
title: A shaft ending inside a full-turn bore refuses RestZipUnsupported(ChordEndpointRevisited): the seam chord's end sits on the bore's self-mated seam
status: open
opened: 2026-10-02
priority: P0
cost: M
---


Found by the REACH lane that closed
`work/reach/full-turn-bore-rest-mate-does-not-union.md`, measured on
`reach/fullturn-bore-mate`.

## Fixture

`crates/sweep/tests/full_turn_bore_mate.rs`'s collar (the rectangle
`ρ ∈ [0.5, 1.5]`, `y ∈ [1, 2]` revolved a full turn about `y`: ONE
bore face with a self-mated seam ruling at azimuth 0) and its
`shaft(deg, y0, h)` with `y0 = 0.5`, `h = 1.0` — the shaft enters
through the lower rim and its top cap floats inside the bore at
`y = 1.5`. Bore × peg walls declared `Rest` (`wall_decls`). The same
file's through and flush spans union at every azimuth and pose.

## Measured

`union_with` refuses
`RestZipUnsupported { what: ChordEndpointRevisited }` at azimuths 0
and 90 (identity pose), from `rest.rs`'s `mint_chord`. The seam has a
run at `y = 1.5` (the shaft's top rim), which crosses the collar's
seam ruling: the crossing layer splits that ruling there, so the seam
segment's end on A is a vertex ON the self-mated seam edge — a vertex
the bore face's single loop visits twice, once per side of the seam.
`halves_at(body, face, u)` returns both half-edges and the
`([hu], [hv])` match refuses.

## What would close it

Pick the occurrence by the wedge the chord leaves into: at each visit
the face's interior wedge runs from the outgoing half-edge's tangent
to the incoming one's reverse about the face's outward normal, and
exactly one visit's wedge holds the chord's direction (decided, not
compared). `sectors::build_sectors` already walks those wedges.
Related: `rest-zip-seam-chord-on-cylinder-wall` (the arc-split
collar's floating peg ships a straight chord where the seam is an
arc) — the same span shape, one door further on.
