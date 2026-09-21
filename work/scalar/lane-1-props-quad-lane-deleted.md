---
id: lane-1-props-quad-lane-deleted
kind: unit
title: LANE-1: PropsQuadLane deleted — the quadrature door is a parameter, the certified name keeps its quadrature, a _structural twin carries the None
status: review
opened: 2026-09-21
branch: scalar/lane-1
pr: 3010
---

## What

The first of the three kernel lane traits goes (`H5` §RATIFIED ruling
3): `PropsQuadLane`'s two methods were `Bounds::lo` under another name
and a runtime-shaped answer to the question the `CertifiedBounds` bound
already asks. The quadrature door becomes a `QuadLane<T>` value in
LANE-0's hook shape (one constructor at `Decide + CertifiedBounds`),
taken as `Option<_>` by the props and tier-3′ bodies; ruling 3's door
rename applied uniformly — `mass_properties`, `classify_shells`,
`validate_pseudomanifold*`, `contact_marks*` keep their quadrature at
`Decide + CertifiedBounds`, `_structural` twins carry the `None`, the
existing `_certified` twins fold into the plain names, every `Dual`
caller moves to `_structural` by name; `datum_lo` → `Bounds::lo` at
its one site with the allowlist entry re-worded; the identity test
replaced by a `compile_fail` doctest and a `_structural`-at-`Dual`
row. Spec: `docs/LANE-1-SPEC.md` (deleted at merge). Block SCALAR-B5
slot 1. Ground: ATREST (`validate.rs`), SHELL, REACH, CHART, the
unowned `props.rs`, LIB/BIND (prelude, binding census), WIRE (verbs,
editor-core), PROPS (allowlist prose, DL3's sentence), GUARD (two gate
entries, naming-only), PCERT (prose), TCOST/TINT; announced.
