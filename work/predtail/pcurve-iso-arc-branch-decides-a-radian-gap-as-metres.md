---
id: pcurve-iso-arc-branch-decides-a-radian-gap-as-metres
kind: issue
title: pcurve_iso_arc_branch decides its Period::decide marks on a radian gap through Margin::of
status: open
opened: 2026-10-25
priority: P3
cost: E
---


Found sweeping `Period::decide`'s callers for
`period-decide-callers-lever-a-parameter-gap` (branch
`pred/turn-rule-follow-ups`).

`geom-brep/src/edge_nurbs.rs` (`arc_rim_candidates`' branch read,
row `pcurve_iso_arc_branch`) decides the turns between an iso row's arc
midpoint `middle` and its corner's azimuth `read` through
`Period::decide(.., middle - read, geom_core::Margin::of, band)`. The gap
is an angle on a circle carrier (radians), and `Margin::of` admits only a
value that is a length already; `[tolerance.margins]` (iii) has it cross
through the rate pair. The carrier is a `Curve3::Circle`, so its radius is
the exact rate: `|g| Margin::metered(g, InfSpeed::new(radius))`.

Unlike the levered sites that unit moved, this one is not bit-identical:
a radius other than one moves the band about each mark, so the change
moves verdicts on carriers far from unit size, and wants its own rows (a
small and a large radius, a gap near a mark at each). The read returns
`None` on a miss (`.ok()?`), so a wrong verdict today is a candidate
dropped or a branch taken where the marks were within `ε` radians, not
`ε` metres.
