---
id: the-plane-arms-harmonic-amplitude-rounds-at-a-far-centred-conics-reach
kind: issue
title: The plane arm reads wrong signs, uncharged, where the conic is stored far from its arc: its harmonic amplitude k.along(n) rounds at u·far and cancels against the now-exact offset
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Filed by GERM's `germ/plane-pose-far-frame` lane (PR 7411) from its
FULL review (finding M1). This loss was there before PR 7411 and was not
introduced by it: the PR made the conic's offset `d₀` exact, and this is
the other term of the same gap.

## Finding

`curve_face::pose::plane_conic` (`crates/geom-brep/src/curve_face/pose.rs`)
reads a conic's height above a plane as `d₀ + R cos(θ − φ)` through
`curve_pair::pose::height` (`crates/geom-brep/src/curve_pair/pose.rs`,
`height`), and decides the belly `r − |d₀|` with
`r = √(a² + b²)`, `(a, b) = k.along(n)`. Since PR 7411, `d₀ = n·(c − o)` is
read to its own digits (`Real::difference_dot`). The amplitude is not.
For a conic whose semi-axes are of the order `far` (a section ellipse of
a shallow cut, centred far from the vertex the plane grazes),
`k.along(n)` multiplies `far`-sized semi-axis vectors by `n` and rounds
at `u·far`. The belly `r − |d₀|` then cancels two `far`-sized terms down
to a gap of the order of the band, so the gap is read at `u·far`, with
no charge (`[simple.conditioned]` forbids one).

## Probe (the reviewer's, restated)

- An ellipse with `a = far` and `b = far/3`, centred `far` from its
  vertex, grazed near the vertex by a plane whose stored origin is near
  the arc.
- ε 1e-12, gap set within ±2× the escalation and read exactly by an
  `Expansion` oracle on the stored terms. 2000 poses per far.

Reads definite against the exact sign:

| far | wrong of 2000 | of them beyond the escalation |
|---|---|---|
| 1e5 | 163 | 30 |
| 1e6 | 359 | 274 |

Example: exact gap −3.86e-11, read `Isolated`.

A sibling on the same ground: an ellipse held on its arc (`Ellipse::anchored`)
answers `frame()` at its derived centre, `anchor − u·a·cos θ₀ − v·b·sin θ₀`,
which rounds at `u·far` when the anchor is far from the centre. A reader
that takes that frame as a plane (`OnSurface::plane`,
`ring_path::LoopArc::plane`) places the plane's origin off the plane by
that much, along the normal. Placing the plane at the stored anchor, which
lies in it exactly, would avoid this, but no public door hands the anchored
placement out (`Ellipse::stored` is `pub(super)`).

## Settles when

The belly `r − |d₀|` is posed so its digits are the gap's own. That
means the gap is formed about the conic's point nearest the plane rather
than about its centre, or `r − |d₀|` is formed from the stored terms
cancellation-free. Measured by the probe above as a row at ε 1e-12,
reading 0 wrong at 1e5 and 1e6, with no charge.
