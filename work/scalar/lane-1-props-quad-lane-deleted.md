---
id: lane-1-props-quad-lane-deleted
kind: unit
title: LANE-1: PropsQuadLane deleted — the quadrature door is a parameter, the certified name keeps its quadrature, a _structural twin carries the None
status: closed
opened: 2026-09-21
closed: 2026-09-21
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
row. The fold is a behaviour change on one body class: the plain
`validate_pseudomanifold*` / `contact_marks*` names at `f64` (the
`pncad` prelude, `pncad-py`'s `Body.validate_pseudomanifold`,
`step-import`'s gate, the tour) now run check 2's plane × NURBS lane,
so on a body carrying an M7-8 edge whose certificate has gone stale
the verdict is one `EdgeCertification` per lane edge where the
lane-keeping body answered `VolumeUncomputable`; every other body is
unchanged, and no public door reaches such a body today (filed on
EXCH). Spec: `docs/LANE-1-SPEC.md` (deleted at merge). Block SCALAR-B5
slot 1. Ground: ATREST (`validate.rs`), SHELL, REACH, CHART, the
unowned `props.rs`, LIB/BIND (prelude, binding census), WIRE (verbs,
editor-core), PROPS (allowlist prose, DL3's sentence), GUARD (two gate
entries, naming-only), PCERT (prose), TCOST/TINT; announced.

## Closed (2026-09-21) — PR 3010

`PropsQuadLane` deleted with its five impls, the trait read in
`reporting_hook`, `lane_certificate`, the prelude re-export, the
binding-census entry and the source-text identity test (its
reader-census ledger entry with it). `topo::QuadLane<T>` — one private
fn-pointer field, one constructor `QuadLane::certified()` at `Decide +
CertifiedBounds` — taken as `Option<_>` by `mass_properties_with`,
`classify_shells_via`, `tier3_local_checks`,
`pseudomanifold_certificate_via`, `contact_marks_declared_via`; ruling
3's door rename applied uniformly: `mass_properties`, `classify_shells`,
`validate_pseudomanifold`(`_certificate`), `contact_marks`(`_declared`)
at `Decide + CertifiedBounds` supplying `Some` by name, the
`_structural` twins (`T: Decide` for the two measurement doors, `Decide
+ Bounds + AtRestPolicy` for the tier-3′ family) supplying `None`, the
four `_certified` names folded into the plain ones, every `Dual` caller
moved by name; `datum_lo` → `Bounds::lo` at its one site, guarded by a
bracket-read census; `AtRestPolicy: Decide + PcurveFittedLane +
ChartRegionLane`. **The fold's content, ruled to stand**: the plain
`f64` names are the certified door, so on an M7-8 body (a plane × NURBS
face) their verdict is the edge-by-edge `EdgeCertification` findings
where the base's lane-keeping door said `VolumeUncomputable`; no public
door can build such a body (filed on EXCH), the in-crate row
`m3_the_plain_names_report_the_corrupt_m7_8_wall_edge_by_edge_and_nothing_else`
pins it, and the door's doc names `pncad-py`'s caller. Both corpus
dumps byte-identical base vs head at `f64`, `Dual64`, `Interval`; the
`compile_fail` doctest fails on the bound; the wiring pinned by
`fn_addr_eq`; DL3's sentence restored to the mechanism shape
(naming-only, `fbcf8ecb6b` cited). Reviews: dual, both APPROVE WITH
FIXES, the one MAJOR bilateral; eleven items, the Python row and the
`value.rs` docstring declined with evidence, five rows filed (ATREST
×2, GUARD, LIB, EXCH) plus the tour's `CertifiedBounds` bundle and the
prelude asymmetry noted for their owners.
