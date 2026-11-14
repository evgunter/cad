---
id: point-in-solid-refuses-a-torus-face-bounded-by-a-spline-chord
kind: issue
title: point-in-solid refuses a torus face bounded by a spline chord (PartialTorusFace), so a body whose torus face a chord trims stops at the piece sort
status: open
opened: 2026-11-11
priority: P2
cost: M
---


Filed by SPIRICARM's rotational-lane stage 3 (`spiricarm/rotational-lane-torus`),
the torus twin of `point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord`.

## What

Once a chorded torus face measures through the rotational lane, a
boolean whose result keeps one passes the result gate and can stop at
the result's piece sort, where point-in-solid reads no torus face a
spline chord trims:

```
Pieces(Probe { shell: ShellKey(1v1), source: PartialTorusFace { face: FaceKey(6v3) } })
```

`crates/sweep/tests/germ_tilted_rod.rs`
`other_spins_and_steeper_tilts_refuse_at_their_germs`: the fifth op of
every pose (the half donut against the tilted rod) stops there, pinned
as `"pieces on a torus face"` at the compiled ε.

## Done when

point-in-solid reads a ray against a chorded torus face (its trim test
on the face's projected images), and the row moves to a body held to
an independent volume.
