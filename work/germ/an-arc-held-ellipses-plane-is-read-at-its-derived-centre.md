---
id: an-arc-held-ellipses-plane-is-read-at-its-derived-centre
kind: issue
title: An arc-held ellipse's plane, and its height's offset, are read at its derived centre, which rounds at u·far off the plane
status: closed
opened: 2026-11-23
priority: P2
cost: M
refs: [7510, 7523]
closed: 2026-11-23
---


Filed by GERM's `germ/harmonic-belly` lane, the sibling that
`the-plane-arms-harmonic-amplitude-rounds-at-a-far-centred-conics-reach`
names. It is not fixed there because no fix stays inside GERM's ground
without reading what Ev's ruling on PR 7523 keeps opaque.

## Finding

An ellipse held on its arc (`Ellipse::anchored`,
`crates/geom/src/curves/conic.rs`) answers `Conic::frame` at its derived
centre, `anchor − u·a·cos θ₀ − v·b·sin θ₀`. That centre rounds at
`u·far` when the anchor is `far` from it, and it rounds along every
direction, the plane's normal included. Three readers take that frame
for the conic's plane, or its origin for the centre the height is formed
about:

- `OnSurface::plane` (`crates/geom-brep/src/curve_pair/on_surface.rs`)
  hands `Curve3::frame` out as the edge's plane. The co-surface arm's
  `under` and `cross` read the curve across that plane, so its origin can
  stand off the true plane by up to `u·far` along its normal.
- `LoopArc::plane` (`crates/topo/src/ring_path.rs`) builds the section
  arc's plane from `c.conic().frame()`, and `plane_roots` / `misses_in_plane`
  read pieces across it. The same offset applies.
- `curve_pair::pose::belly` and `height` (`crates/geom-brep/src/curve_pair/pose.rs`)
  form the height's offset `d₀ = n·(c − o)` about `k.frame().origin()`.
  For a conic held at its centre that point is stored, and the belly is
  now read to its own digits from it (`geom_core::Real::harmonic_belly`).
  For an arc-held conic it is the derived centre, so `d₀` carries that
  centre's `u·far` rounding, and so does the belly.

## Measured

This was measured on `germ/harmonic-belly`, with the belly re-posed. The
plane-arm probe (`curve_face::pose_rows::a_graze_of_a_far_centred_conic_is_read_wrong_nowhere`)
was re-run with the ellipse built by `Ellipse::anchored` at its vertex,
and the oracle's offset was formed about the stored anchor
(`n·(A − o) − a·(u·n)`, exact). At ε 1e-12, seed 0x1, it reads **21 / 134 / 410
wrong of 2000 at far 1e4 / 1e5 / 1e6, of them 0 / 0 / 220 against the
exact sign past the escalation**. The same probe with the ellipse held at
its centre reads 0 everywhere.

## Why not fixed here

The stored anchor lies in the plane exactly. A plane placed at it, or a
height formed about it (`h(θ) = n·(A − o) + a(u·n)(cos θ − cos θ₀) +
b(v·n)(sin θ − sin θ₀)`), would cancel nothing. The anchor is reachable
from outside `geom` only through `Curve3::data()`'s datum walk
(`CurveDatum::Anchor`, `Phase`). Ev's ruling on PR 7523 (recorded in
`work/conic/the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre.md`)
makes the stored point opaque: "no reader sees `S`, the phase or the
placement". So that route would read around the decided interface.
ONEROUND's PR 7510 gates `Conic::frame` and `Curve3::frame` as
centre-frame doors and lists both plane readers among the sites still
reading them.

## Settles when

The conic answers its plane at a point that lies in it to that point's
own digits, and its first-degree height about its stored point. CONIC's
polynomial (PR 7523, "arc-held … posed about its anchor") is that door,
and these three readers move onto it. The settling measurement:
`curve_face::pose_rows::a_graze_of_a_far_centred_conic_is_read_wrong_nowhere`
re-run as above (an arc-held ellipse, the oracle about its anchor),
reading 0 wrong at far 1e4, 1e5 and 1e6.

## Closed (2026-11-23, PR 7553): a duplicate of ONEROUND's unit

- **The belly half is fixed by PR 7553.** `Conic::belly_along` reads an arc-held ellipse from its anchor and phase. The plane probe on arc-held ellipses (`curve_face::pose_rows::a_graze_of_a_far_centred_conic_is_read_wrong_nowhere`) now reads 0 wrong at far 1e4 / 1e5 / 1e6, where it read 21 / 134 / 410 above.
- **The plane half is folded into `work/oneround/an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre.md`.** That covers `OnSurface::plane` and `LoopArc::plane`, which still read the derived centre. It is a dispatched P1 unit whose scope already covers `curve_pair/` and the class, so it is noted there with a dated section rather than kept as a second row.
