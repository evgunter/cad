---
id: ruling-coverage-and-ssi-depend-on-each-other
kind: issue
title: pair_section::ruling imports ssi::enclose::Box3 for its coverage while ssi calls pair_section::ruling::coverage: a two-way dependency between the two modules
status: open
opened: 2026-11-02
priority: P4
cost: E
---


Filed from the PR 6309 review (cylinder × cylinder ruling sweep, 2026-11-02).

## What

There is a two-way dependency between `ruling` and `ssi`:
- `pair_section::ruling::coverage`, the row-5 coverage oracle, takes
  the found branches' tube boxes as `crate::ssi::enclose::Box3`;
- `ssi::march_cylinder_cylinder` calls `pair_section::ruling::coverage`.

Both modules sit in `geom-brep`, so this compiles, but the layering is
crossed: `pair_section` already calls into `ssi` through `SsiLane`, and
now `ssi` calls back into `pair_section`.

## What a fix has to supply

One of:
- the coverage call moved to `pair_section::march_over_the_ball`, which
  holds both the section and the branches (their tube boxes through
  `ssi::certify::tube_boxes`);
- or the coverage moved under `ssi`, beside the march it checks.
