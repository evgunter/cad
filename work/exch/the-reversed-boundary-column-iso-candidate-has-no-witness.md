---
id: the-reversed-boundary-column-iso-candidate-has-no-witness
kind: issue
title: The reversed boundary-column iso candidate has no witness since dm1's l-bracket walls promote to cylinders
status: open
opened: 2026-11-20
priority: P3
cost: E
---

Found while landing whole-patch promotion (`exch/promote-torus-sphere-cone`).

`build.rs`'s iso adoption takes an edge whose carrier runs AGAINST a
wall's boundary iso through the reversed net (`Running::Mirror`, the
`net.reversed_v()` / `reversed_u()` arm). Its executed witness was
`r1_dm1_probe::the_l_bracket_alone_adopts_its_reversed_slit`: dm1's slit
`#389` runs reversed against its wall `#382`'s column. Both l-bracket walls
are exact rational cylinders, which stage-1 recognition now promotes
whole, so the slit adopts on a cylinder and the reversed arm is no longer
reached there. No other row reaches it on purpose.

Owed: a fixture whose spline wall no kind promotes (a bowed or elliptic
wall) with an edge stated against the wall's boundary column, asserting
the `Running::Mirror` adoption and the chart image it gives.
