---
id: ring-1-interval-type-ungated
kind: unit
title: RING-1: geom_core::interval compiles unconditionally; the feature gates only the instantiation
status: open
opened: 2026-09-21
branch: scalar/ring-1
---

## What

The first cut of `H5`'s ruling 1 (PR 2701, `## RATIFIED`): the
`Interval` scalar's module, arithmetic and impls compile in every build;
`interval-transcendentals` becomes a normal dependency of `geom-core`;
the `interval` feature keeps gating only the kernel's instantiation at
`Interval` (the lane impls in other crates and the gated test files)
until RING-3 drops it. Measured free (107 s vs 106 s clean workspace
build, 5 files, +2/−9 lines). Spec: `docs/RING-1-SPEC.md` (deleted at
merge). Block SCALAR-B4 slot 2 (OPUS). Ground: PROPS (`geom-core`),
CIW, GUARD; announced.
