---
id: a-sphere-against-a-spline-wall-a-kilometre-across-settles-no-foot
kind: issue
title: A sphere against a spline wall a kilometre across reads no whole section: a certified foot point does not converge at 5e-13 m
status: open
opened: 2026-11-12
priority: P2
cost: M
---



Filed by the `sectarms/join-reads-spline-implicit` lane.

## What

`pair_section::spline_implicit::carry_rows`'s lofted wall scaled by
`s = 1000` against the sphere about `s·(0, 1.1, 0)` of radius `0.4·s`
classifies, and its whole reading refuses before any carry:

`Unaccounted { cause: "ssi: the certified foot point at t = 0.375 would
not converge (last distance 4.687428401686771e-13 m), so the on-locus
residual against the NURBS operand cannot be stated" }`

The same pair at `s = 0.01, 1, 100` reads and carries, and the plane
`z = s` against the wall reads at `s = 1000`. A foot that stands
`5e-13` m off at a kilometre is at the format's resolution there, so a
convergence test read in absolute metres would refuse it. Not checked
which test it is.

The carry row skips the sphere at `s = 1000` and cites this file.
