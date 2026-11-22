---
id: a-memo-hit-pays-for-the-route-the-lane-used-to-skip
kind: issue
title: A warm memo hit runs the curved-face route's doors serially before the memo is consulted, where the kind route skipped them with the lane
status: open
opened: 2026-11-17
priority: P3
cost: M
---

## What

`mesh::tessellate_with` reads every face's route (`tessellate::Routes::of`,
`crates/mesh/src/tessellate.rs`) before the memo is consulted, serially,
and the route asks the curved-face doors (`curved::iso_rectangle_shape`,
`crates/mesh/src/curved.rs:580`) of every analytic curved face, and the
split-side fact (`walk::splits_a_straight_side`) of every rectangle. On
main before PR 7041 the doors ran inside the curved lane, which a memo hit
skips, so a warm hit paid for neither.

Measured on PR 7041's fix-pass head against main `184e484ed2`, this
container (4 vCPU), the mesh test binary (`mesh` at opt-level 2), 60 warm
hits per body at δ 0.05, three runs alternating the trees, median per run:

| body | main (µs) | head (µs) |
|---|---|---|
| washer | 151–161 | 230–267 |
| rounded prism | 186–194 | 282–376 |
| cone | 89–92 | 123–152 |
| donut | 317–533 | 419–510 |
| ball | 147–185 | 184–243 |

The second review measured the same shape (washer 86–138 → 185–188 µs,
rounded prism 128–157 → 288–346).

## The two cures in sight

- **Route in the parallel stage.** The route is per face and reads nothing
  another face writes; `Routes::of` could run in the same parallel map as
  the lanes, with the chord pass reading the table after it. The chord
  pass reads the route (`chords::tighten`, `chords::edge_target`), so the
  route would run in its own parallel pass before the chord pass.
- **Key the memo on the route's inputs.** The doors read the face's loop
  and surface, which the memo key already folds; a key hit could carry
  the route it was taken under, so a hit skips the doors as the kind
  route skipped them with the lane.
