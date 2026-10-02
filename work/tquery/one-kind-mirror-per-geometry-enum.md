---
id: one-kind-mirror-per-geometry-enum
kind: unit
title: One fieldless kind mirror per geometry enum, in geom: CurveKind and SurfaceKind beside Curve3 and Surface, every copy deleted
status: dispatched
opened: 2026-10-02
priority: P1
cost: M
refs: [curve-kind-placement-disagrees-with-the-ratified-seat-clause, 3763, 3664]
branch: tquery/one-kind-mirror
---


## What

Ev's ruling on `curve-kind-placement-disagrees-with-the-ratified-seat-clause`
(PR 3763), as `crates/verbs/README.md` §1 S1 now states it:
- `CurveKind` and `SurfaceKind` are the workspace's one fieldless
  mirror of `Curve3` and of `Surface`, each in `geom` beside the enum
  it mirrors (`Curve3::kind`, `Surface::kind`).
- Every crate above reuses them.
- `topo::query` owns `CurveKindSet` / `SurfaceKindSet`, their bit
  numbering, and the EXACT predicates.

Copies this deletes:
- `topo::query::CurveKind`
- `geom_brep::CurveKind` (PR 3664)
- the test-only `geom::Curve3Variant` / `SurfaceVariant`
- `topo::query`'s hand-kept `ALL_SURFACE_KINDS`, with the two bit
  tables, which become derived from the roster

`geom_brep::SurfaceKind` moves down to `geom`. `route` keeps its
exhaustive match.

## Authoring (orchestrator's call, on Ev's "simplest/safest")

**Derived:** `strum::EnumDiscriminants` (+ `EnumIter` for the roster),
always on in `geom`, named `CurveKind` / `SurfaceKind`.
- A derived mirror cannot fall out of step with its enum, and no
  roster or census is kept by hand.
- `strum` is already in the lockfile under `test-support`, so this
  adds a production edge, not a crate.
- `name()` and the per-variant docs stay hand-written where the
  derive allows.

**Fall back to hand-written** in `geom` (exhaustive `of`, `ALL` with
a census guard) only if the derive cannot carry what readers need:
`Copy`, `Eq`, `Ord`, `Hash`, docs, and `Debug` names the payloads
already render. Say which way it went, and why, in the PR.

Every path outside `geom` / `geom-brep` keeps working through
re-exports (`topo::CurveKind`, the `pncad` prelude). The Python
binding keeps its own enums (its exhaustive match over the canonical
type, #1388 V2).
