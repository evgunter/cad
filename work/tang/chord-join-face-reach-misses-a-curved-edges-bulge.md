---
id: chord-join-face-reach-misses-a-curved-edges-bulge
kind: issue
title: chord_join's face_reach encloses a face's boundary vertices, not its curved edges' bulge
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Found by PR 4118's full review (MINOR-2).

## What

`chord_join::face_reach` (`crates/topo/src/chord_join.rs:558`) hands
the section table the ball about the base vertex whose radius
`face_extent` measures (`crates/topo/src/splitting/rules.rs:481`).
`face_extent` reads only the face's boundary VERTICES. A curved
boundary edge can bulge past every vertex. For example, an obliquely
trimmed cylinder wall whose ellipse rim carries one seam vertex reaches
about `2r` along the axis beyond it. The ball then under-states the
consumed region, and an under-stated lever reads a tilt smaller than it
is: the wrong-answer direction (`crates/geom-brep/src/extent.rs`
module docs).

## The shape of a fix

Read the face's certified box instead, the ball the boolean's carrier
doors and germ frame already use (`boolean::rest::face_ball`, through
`census::face_reach`), or add each curved boundary edge's carrier ball.
`face_extent` has other callers (its lever arms in the split lane), so
whether they move with it is part of the item.
