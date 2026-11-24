---
id: torus-parallel-touches-wait-on-the-line-torus-door
kind: unit
title: A torus touch along a whole parallel is read by its near band once the line x torus door decides its silent answers as a clearance
status: parked
opened: 2026-11-23
priority: P1
cost: M
blocked_on: [the-line-torus-door-misses-silently-within-the-band, a-torus-touch-is-read-by-its-near-band]
---


The parallel half of `a-torus-touch-is-read-by-its-near-band`. PR 7611's
dual review showed both reviewers, independently, that a parallel touch
read off the exact pose is unsound today. A coaxial reading admits an
axis offset or tilt inside the band and takes the whole parallel as the
locus, with `at` at the torus's `u_ref`. Where the partner dips into the
tube on the far side of `at`, the edges parting the crossing from `at`
are the partner's straight rulings, standing OUTSIDE the solid within
the band, where line × torus is silent. Measured on PR 7611's head
`e5cbc7cd4` (ε = 1e-9), each building every op where main refuses R-tan:

- a sleeve sector over [60°, 300°], with its inner wall at R + r and its
  axis moved +0.2ε or +0.5ε (Intersect comes back empty though the
  solids overlap);
- a D-pin in the hole, offset and shrunk by 0.2ε to 0.6ε, crossing the
  inner equator by up to 0.4ε along about 140° of arc.

So a parallel touch needs S′ outside the solid as well as inside. That
is the line × torus door deciding its silent answers as a clearance past
Kε. Once the door is fixed, admit the coaxial wall, sphere, torus and
cone parallels and the axis-normal plane's tangent circle, and build the
two poses above as refusals and the off-face poses (a hub flush with a
ring's top, a sleeve above its widest circle) as builds.

## Ev's ruling (2026-11-24, PR 7587)

Approved once sound ("if it's sound i'm happy with it"). This unit is the build once `the-line-torus-door-misses-silently-within-the-band` lands; no design question remains.
