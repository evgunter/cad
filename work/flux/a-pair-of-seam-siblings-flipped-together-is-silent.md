---
id: a-pair-of-seam-siblings-flipped-together-is-silent
kind: issue
title: Two sphere seam siblings with both poles on their loops, flipped together, pass tier 3 and measure a wrong volume
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [a-sphere-face-whose-boundary-encodes-no-side-is-measured-under-its-bit-alone]
---

## Measured

Found by the dual review of PR 4573 (R2, by execution; the probe is
`review_4573_flips` on `review/4573-018BFdMe`,
`crates/sweep/tests/review_4573_e2e.rs`). Over ball-pair and slab–ball
booleans, every sphere face was flipped alone and every pair of faces
on one surface was flipped together: 6 of 568 flips passed tier 3 AND
measured, with a wrong volume — one reported `0.33` against a true
`7.19`. All six flip a pair of seam siblings. The class is on main
too: the sphere arm's PR did not open it, and its claim that check 4
covers it was false.

## Why nothing sees it

A one-loop sphere face with both chart poles on its loop (a lune, a
hemisphere split at its poles) encodes no side
(`crates/geom-brep/src/props/curved.rs`, `pole_free_side`), so check 6
exempts it (`crates/topo/src/validate.rs`, `CurvedSenseInverted`'s
"What is exempt"). Two such faces meet along their seam meridians
tangentially, and check 4 reads material pairing across `Smooth`
edges: a flip of ONE sibling is a lamina there. A flip of BOTH keeps
the seam consistent; their other edges are section arcs meeting planar
or other sphere faces transversally, where check 4 reads nothing, and
the radial flux term reads the flipped bit, measuring each face's
complement.

## Possible owners

- A pair-of-siblings reading on one surface: the union of the siblings
  sharing a seam is one region whose boundary is the section arcs
  alone, with the seam interior to it — a loop that holds no pole on
  it and so encodes its side by `sphere_loop_side`'s pole reading.
- A body-level check (the +V invariant per solid catches a reversed
  solid, not a pair of reversed faces inside a positive one).

## Witness to write

`review_4573_flips`' silent cases, as a committed row that is red until
the owner lands.
