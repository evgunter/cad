---
id: one-kind-mirror-per-geometry-enum
kind: unit
title: One fieldless kind mirror per geometry enum, in geom: CurveKind and SurfaceKind beside Curve3 and Surface, every copy deleted
status: closed
opened: 2026-10-02
priority: P1
cost: M
refs: [curve-kind-placement-disagrees-with-the-ratified-seat-clause, 3763, 3664]
branch: tquery/one-kind-mirror
pr: 3777
closed: 2026-10-02
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

## How authoring went (PR 3777)

**Hand-written, the fallback.** The first cut derived the kinds with
`strum::EnumDiscriminants`. Review found the derive's docs false:
strum_macros 0.28 always copies a payload variant's `doc` attributes
onto its discriminant, so `SurfaceKind::Cone` carried the cone's field
prose and `SurfaceKind::Nurbs` described an `Arc` payload it does not
have. Docs are one of the things the derive had to carry, so the fix
pass took the fallback:
- `CurveKind` / `SurfaceKind` are hand-written in `geom`, one line of
  doc per variant.
- `Curve3::kind` / `Surface::kind` are wildcard-free matches, so a new
  variant is a compile error until it has a kind.
- `ALL` is still derived (`strum::VariantArray` on the kind enum), so
  no roster or census is kept by hand.

## Closed (2026-10-02, PR 3777)

One hand-written `CurveKind` / `SurfaceKind` in `geom` (exhaustive `kind()`, `ALL` from `strum::VariantArray`); `topo::query`'s, `geom_brep`'s and tess-meter's mirrors and the test-only derived ones deleted; one `name()` / `adjective()` register pair, every free-string namer of the same word set routed through it.
