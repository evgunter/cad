---
id: blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex
kind: issue
title: A shaft ending inside a full-turn bore refuses in the REST zip - ChordEndpointRevisited (a seam chord's end on the bore's self-mated seam) or ChordBetweenIsolatedPierces
status: parked
opened: 2026-10-02
priority: P0
cost: M
blocked_on: [intent-stage4-is-built]
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

## Measured again, every blind span (2026-10-02, after the fix pass)

`full_turn_bore_mate.rs`'s collar and `shaft(deg, y0, h)`, both
operand orders (`collar ∪ shaft` and `shaft ∪ collar` refuse alike):

| span | `y` | 0° (on the bore's seam) | 60°, 90° |
|---|---|---|---|
| blind from below | `[0.5, 1.5]` | `ChordEndpointRevisited` | `ChordEndpointRevisited` |
| blind from above | `[1.5, 2.5]` | `ChordBetweenIsolatedPierces` | `ChordEndpointRevisited` |
| wholly inside | `[1.2, 1.8]` | `ChordBetweenIsolatedPierces` | `ChordEndpointRevisited` |

`ChordBetweenIsolatedPierces` is the seam chord between two pierce-ring
vertices `mint_chord` has no site for: at 0° the shaft's floating cap
rim meets the bore's seam ruling only at the shaft's own rim vertex, so
both of the chord's ends are lone vertices of the bore face. Both
sub-frontiers are the same shape — a cap rim floating inside a
full-turn face — and close together.

## Measured on main (2026-10-06)

Measured on `origin/main` 3f1e3b0d at the six `poses()`, both operand
orders, and the 1e-9, 1e-6 and 1e-12 rows.

- **Blind from below and blind from above, at 0°, 60° and 90°:** the
  union builds in the chord join at `2π + π/4` (one shell, tier 3, the
  census), and the declared-REST zip is not entered. Pinned by
  `full_turn_bore_mate.rs`'s
  `a_blind_shaft_unions_on_and_off_the_bores_seam`. Twelve of the
  eighteen cells in the table above have moved.
- **Wholly inside (`y ∈ [1.2, 1.8]`), at every azimuth:** the chord
  join refuses first, with `Join(SectionInvariant { what: "ring
  re-homing on a chart: the run's azimuth window spans a full period,
  so a ring vertex has no single branch on it" })`. That is
  `chord_join::chart_ring_side`, from `ChordJoiner::rehome_rings` after
  the mef across the bore. The zip then refuses `RestZipUnsupported {
  what: ChordEndpointRevisited }`, from `mint_chord`'s
  two-halves-at-one-end arm, on the first span `realize_seam` takes on
  the collar:
  - cell `InFace(bore)`; host the bore, a `Cylinder` of radius 0.5;
  - twin the shaft's rim arc (`Twin::Circle`);
  - `u` is the collar vertex where the shaft's lower rim crosses the
    bore's seam ruling, at `(0.5, 1.2, 0)`; it has edges, and the bore's
    one loop visits it twice;
  - `v` is a pierce-ring vertex of the bore at one of the shaft's rim
    vertices; it has no edges, and the bore's loops visit it 0 times.

  At 0° the table above says `ChordBetweenIsolatedPierces`; that cell
  now refuses `ChordEndpointRevisited` too, on this same span shape.
- **Undeclared:** every span refuses `CurvedPierceUnsupported` in the
  reduction's curved crossing (`reduce.rs`, the deferred touch), before
  the join.
- **The corner datum.** The chord join chooses the corner of a vertex
  the loop visits twice at insertion: the germ lies in one sector of
  `sectors::build_sectors`, each with its own orbit half-edge, and
  `insert::mint_run` hangs the strut there. The join's chord then
  starts from that strut's half, `HalfGerm.he`. The zip's
  `read_segments` keeps each segment's end vertices and loci but not
  `HalfGerm.he`, and `undo_struts` removes the struts. `mint_chord`
  then lists the face's halves at `u` (`halves_at`); `u` has two, and
  nothing it reads tells them apart.

## Parked on the D10 hold (2026-10-06)

This row is on declared-contact ground, so it waits on `d10-one-way-to-say-intent-is-unbuilt` (`work/join/log.md`, the 2026-10-03 hold). D10 stage 4 retires the declared-REST zip: `work/intent/the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms.md`. When the hold lifts, close this row if its code is gone, or move it to the join if its scene still refuses there.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: ChordEndpointRevisited is in rest.rs mint_chord, deleted at stage 4; the rest is arm 2 of the three-arms item. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)
