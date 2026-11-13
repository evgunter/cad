---
id: the-certificate-has-no-separator-across-a-plane-tangent-to-a-torus-band-so-the-composed-die-runs-out-of-pairs
kind: issue
title: The departure arm has no separator across a plane face tangent to a torus band along their circle, so the composed die's 21 pip rims split until the injectivity check runs out of its 65,536 pairs
status: open
opened: 2026-11-12
priority: P0
cost: M
---


## Finding

From CLEAR's lane clear-pairs
(`the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies`,
its Diagnosis). Once a sphere's pole reads as a collapsed side,
`diepips` and `diechamfer` hold; `diecomposed` (89 faces: the pipped
cube with its 21 pip rims replaced by torus bands) still refuses
`Budget(Pairs)` at depth 29. Two thirds of its split attempts
(44,558 of 68,891) are the departure arm on a die face (a plane) against one of
its pip's torus bands, about 1,730 splits per pair, ending at cells
`0.003` wide at the rim.

The band meets the plane tangentially along the rim circle: the
circle is the torus's `v = π/2` line, the top of its tube. No
separator the arm tries has a first-order sign on both faces there:
- the circle's own plane (`plane_through`) is the die face's
  plane, so it vanishes on the whole plane face;
- the plane's carrier read on the band, and the torus's carrier read
  on the plane, are both second order at a tangent edge, so the
  derivative across the zero side encloses zero at every depth;
- the graph separator reads the band (`Negative`) but not the plane.

The instrumented run (lane clear-pairs, `f64`) printed `plane =
(None, None)` and `graph = (None, Negative)` on 20,963 of these
pairs. The other third of the splits is the distance arm, 360–380
per pair between a die face and a band of another face's pip.

## What a fix needs

A separator that leaves a G1 edge at first order on both sides. For
a planar circle, the cylinder over it (axis the circle's normal,
radius its radius): the plane face lies outside it and the band,
curving into the pip, inside it, each with a nonzero radial speed at
the rim. It is the `Implicit::Cylinder` the carrier arm already
reads, built from the circle instead of a face. The plane face's
presentation is a `Zero::Curve`, so `Curved::psi` must read it too.
Pin: one pip with a torus rim on a plane (a smaller fixture of the
composed die) holds within the default budget.
