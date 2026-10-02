---
id: nurbs-interval-ders-at-a-wide-parameter-grows-with-translation
kind: issue
title: geom: NurbsSurface::ders at an Interval parameter wider than a point assembles the rational quotient in the absolute frame, so its derivative enclosure grows with the net's translation (0.26 wide at the origin, 54 at 100 m, 5.4e4 at 1e5 m)
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## Found (§5 sweep of `ssi/rational-chart-speed`, 2026-10-02)

**Measured, no consumer known to reach it.** `NurbsSurface::ders`
(`crates/geom/src/surfaces/nurbs.rs`) hulls `SurfaceWindow::ders_in_span`
over the spans an `Interval` parameter overlaps, and `ders_in_span`
sums the homogeneous derivatives `A = Σ N·w·P` and `w = Σ N·w` in the
absolute frame before the quotient rule `S_u = (A_u − w_u·S)/w`. At a
thin parameter the two terms each carry the net's translation times
`w_u`, and their difference cancels it to a rounding width, which is
D4 ¶2's allowance. At a parameter wider than a point the two are
decorrelated enclosures and the translation term stays in the width.

On a cubic × linear wall weighted 1.8 / 0.7 (the fixture
`rational_wall` in `crates/geom-brep/src/ssi/enclose.rs`'s tests), the
`x` component of `S_u` at `v = 0.5` over `u ∈ [0.4, 0.41]` reads:

| translation | enclosure | width |
|---|---|---|
| 0 | [0.650, 0.909] | 0.26 |
| 100 m | [−25.3, 28.5] | 54 |
| 1e5 m | [−2.6e4, 2.8e4] | 5.4e4 |

At `u ∈ [0, 1]` it reads `[−∞, ∞]` at every translation. The thin rows at
`u = 0.4` are 6.5e-15, 1.2e-12 and 1.2e-9 wide: rounding widths.

The SSI chart enclosure had the same shape and was fixed by reading
each derivative coefficient as a difference of control points
(`NurbsBoxes::cell_quotient_numerator`). A grep for `.ders(` /
`ders_in_span(` at an `Interval` hull in `crates/*/src` found no
caller passing a non-thin parameter, but generic `T` call sites
instantiated at `Interval` by a caller cannot be told apart by grep.

## What is open

Decide whether the evaluation lane owes a translation-invariant
enclosure at a wide parameter (a cell-local origin inside
`ders_in_span`, as `NurbsBoxes` and `mesh::chords` do), or document
that it is a thin-parameter door and refuse or flag a wide one.
