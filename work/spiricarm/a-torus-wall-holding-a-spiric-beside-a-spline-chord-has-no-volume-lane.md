---
id: a-torus-wall-holding-a-spiric-beside-a-spline-chord-has-no-volume-lane
kind: issue
title: A torus wall holding a spiric beside a spline chord has no volume lane: the spiric lane refuses the chord's lifted image
status: open
opened: 2026-11-02
priority: P2
cost: M
---


## Finding (GREEN lane `green/main-red-sweep`, 2026-11-02)

`crates/sweep/tests/germ_tilted_rod.rs`
`other_spins_and_steeper_tilts_refuse_at_their_germs` crosses the half
donut with a tilted rod. At β 0.6 and 0.7, ∪ and the rod-first
difference (and ∩ at β 0.7, spin π/2) refuse at the result gate's
volume:

```
ResultInvalid { errors: [VolumeUncomputable { source: Face { source:
  QuadratureUnsupported { what: "a torus wall edge beside a spiric whose
  lifted image is neither a spiric wall image nor a harmonic rim or
  meridian" } } }] }
```

This is the same at the default ε, 1e-6 and 1e-12 on main `a285cdbf`.
The refusal is raised in `topo::props::quad_lane` (`:1086`). A torus wall
any of whose loops carries a spiric goes to the spiric lane, which reads
only spirics, rims and meridians. Here the wall also carries an edge
with some other image. The rod's cap plane cuts the torus in a spiric,
and its wall cuts it in the section's fitted chords. So the other edge
is most likely a chord's projected image, though this lane did not
read the face to confirm it.

Since PR 6013 these runs pass the spline join and reach this door
(GREEN's eighteen-sweep row). The row pins each cell by cause, and this
row owns the `"spiric wall volume"` cells.

Reproduce:
`cargo nextest run -p sweep --profile ci --ignore-default-filter -E 'test(=germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs)'`
(about 3 min at the default ε at opt-level 2; about 30 min at 1e-12).

## What closing it takes

The spiric lane reads a chord's projected image beside a spiric, or it
routes such a wall to a lane that does: the spline-rim arm the torus-wall
row asks for, or the projected-boundary lane. Then re-pin the germ table
at whatever the runs do next.
