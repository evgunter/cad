---
id: adjacent-kinds-cannot-tell-a-crease-from-a-co-surface-seam
kind: issue
title: The selection vocabulary cannot say 'the crease between two spheres': AdjacentKinds(Sphere, Sphere) also names every seam meridian, and only rim_of's CoSurface refusal separates them
status: open
opened: 2026-10-02
refs: [edge-side-surfaces-have-no-door, seed-finder-home-reads-only-the-y-station, 3787]
priority: P3
cost: M
---


## What

The snowman scene (`demos/tour/src/snowman.rs`, `waist`) fillets the
union of two coaxial balls of revolution at its waist. Said by
description, the waist is "the edges between two sphere faces", and
`topo::query::edge_adjacent_matches(body, e, {Sphere}, {Sphere})`
(the kernel half of `GeomPred::AdjacentKinds`) names SIX edges on that
union: the waist's two arcs and four seam meridians, two per ball,
because a full revolve leaves each ball as two half-bands on ONE sphere
key and a meridian's two sides are therefore both `Sphere`. Handing all
six to `fillet_edges` would ask it to roll a ball along a co-surface
seam, which has no wedge.

Nothing in the selection vocabulary says "two DIFFERENT surfaces" or
"a concave edge": the kind atoms read tags, and `Convex`/`Reflex` is
reserved and unbuilt (`docs/SELECT-DESIGN.md`, GS-Q2). The scene
separates them through `query::rim_of`, which refuses a co-surface seed
`RimError::CoSurface` — so a refusal is doing a filter's job, and every
caller re-derives that exclusion (`demos/tour/src/bodies.rs`'s
`bud_rim` hand-rolls the same two-sided test on surface keys before it
calls `rim_of`, and `demos/tour/tests/common/rim_select.rs` carries it
as `Seeds::TwoSided` for the tour's fillet suites).

GS-Q2 recorded that its demand evidence was "a comment rather than a
call site, since the demo's kind-pairs covered the real case". This is
a call site where the kind pair does not cover it: a crease between two
surfaces of the SAME kind. The die's pip rims were `(Plane, Sphere)`,
which a meridian cannot match; the snowman's waist is `(Sphere,
Sphere)`, which every meridian of a revolved ball matches.

## Shape of a fix (the owner's call)

An EXACT atom needs no margin: "the edge's two sides rest on different
surface keys" is a stored-data read of the same class as
`edge_adjacent_matches`, and `rim_of` already computes it
(`edge_sides`). Exposing it as a query predicate (and a `GeomPred` atom
at the document door, on WIRE's ground) would let the snowman's
selection be one filter. The decided `Convex`/`Reflex` atom is the
fuller answer and stays GS-Q2's open design.

## Related

- **The design record**: `docs/SELECT-DESIGN.md` GS-Q2, which reserved
  `Convex`/`Reflex` on the grounds that kind-pairs covered the demand.
- `edge-side-surfaces-have-no-door`: the read-back door for an edge's
  two side surfaces. That door would let a caller spell the
  distinct-surfaces test by hand; this row asks for it as a SELECTION
  atom, so a description says it without a hand-rolled walk.
- `strut/seed-finder-home-reads-only-the-y-station`: the sibling
  seed-finder friction — rims picked by a hand-rolled scan because no
  selector names them.

## Why the boolean's seam record is not the answer

`BooleanBody.naming.seam_edges` is public and, on the snowman union, is
exactly the waist (the review measured it equal in five
configurations). It is not a selection:

- It is a naming-EMISSION record — "the seam edges surviving the zips",
  recorded for the naming layer — so it names the waist only because
  this body's one crease happens to be the op's seam. A crease that was
  authored rather than cut (a revolve's own concave corner), or one
  that survives a later op, is not on it.
- It is unavailable at the document door, where `select_where` speaks
  stable names, so a document author cannot reach for it.
- A user selecting a rim on a body describes GEOMETRY ("where the two
  balls meet"), and the vocabulary should be able to say that.

So the scene stays on the description, with `rim_of`'s refusal doing
the filter's work, and this row records the gap.
