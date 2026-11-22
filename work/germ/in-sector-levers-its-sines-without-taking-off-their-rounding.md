---
id: in-sector-levers-its-sines-without-taking-off-their-rounding
kind: issue
title: sectors::in_sector levers its sines and cosine without taking off their rounding, as arc_side and apart do
status: open
opened: 2026-10-08
priority: P3
cost: E
---


Filed by PR 4358's review (NOTE-2), on HONE's slate beside
`in-sector-is-a-second-spelling-of-within`: `sectors.rs` is HONE
ground and this is the same reader.

## What

`crates/topo/src/boolean/sectors.rs` `in_sector` decides
`bool_cone_within` (the sine `(u × d)·n` past each bound) and
`bool_cone_facing` (the cosine `d·m` to the sector's middle) by
levering the raw floating-point value at the least joint deviation of
the points it reads (`least_lever`). The value is a product of unit
vectors and rounds by a few ulp. The lever magnifies that rounding
like any real deviation, so a value that is only rounding, levered at
a long reach, can read as decided.

`arc_side` takes eight ulp off its determinant before levering it
(`certain = (det − r)⁺ + (det + r)⁻`), and PR 4358's `apart` does the
same for its sines (`bool_cone_apart`). D4 counts the rounding of a
product in its margin, so `in_sector` is the one reader of the three
that does not.

## The shape to give

Take the same eight ulp off each of `in_sector`'s three values before
levering them. Measure which rows move: `cone_fuzz`'s exact oracle and
the bound family's short-reach rows read through `in_sector`.

Moved from HONEROOT to GERM on 2026-11-20, by `git mv`, when HONEROOT merged into GERM: HONEROOT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one GERM carries.
