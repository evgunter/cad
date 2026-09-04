---
id: interior-iso-curve-de-boor-extractor
kind: issue
title: Interior iso-curves have no certified route - the de Boor collapse extractor P-2 reached
status: open
opened: 2026-08-29
github: 1195
refs: [498]
---

## From GitHub issue 1195

Opened 2026-08-29; 0 comments.

## What

An INTERIOR iso-curve of a described NURBS chart has no certified route. PCURVE
P-2 (#498) is the construction that first reached one, which is the condition the
existing refusal banks the work against — quoting it in full below.

## The two refusals, and why there are two

**What the kernel does today (post-revert, measured `run6`):** the mint refuses at
DERIVATION, in `nurbs_iso_derive`'s wall–wall seam arm
(`crates/topo/src/pcurves.rs`), for half-edge `16v1` of the P-2 fixture:

```
Certify { half_edge: HalfEdgeKey(16v1), error: IsoUnsupported { what:
  "the carrier's start point lies on neither chart boundary — not a boundary iso
   of this face's chart" } }
```

**What lies behind it.** P-2 briefly widened that arm to mint the measured interior
column as an exact `Pcurve::IsoLine`. The derivation then succeeded — `Ok(IsoLine
p0=(1.0, 0.0) pl=(0.0, 1.0))` — and CERTIFICATION refused instead, at
`side_of` in `crates/geom-brep/src/pcurve_cache.rs`:

```
IsoUnsupported { what: "an INTERIOR iso (the fixed channel sits on neither chart
  boundary): boundary rows are control-net copies, an interior iso needs the de Boor
  collapse extractor — which arrives with the construction that first mints one" }
```

That widening was REVERTED, because minting it is exactly what
`geom-brep/tests/imported_chart_arc_rim.rs::an_interior_column_still_refuses`
requires the kernel to refuse. So the derivation-side refusal is the honest current
state, and the certification-side one is the capability actually missing. Both are
the same gap seen from two sides; the second names it.

## Why `General` is not a fallback here

U2's `General` arm certifies at the FITTED grade, whose C2 certificate is a
statement about an operand PAIR (hull sup + uniqueness tube). `topo`'s
`mate_surface` reads that pair from an `EdgeDescription::Intersection`, which names
two surfaces. `16v1` carries a `Chart` description, which names ONE — so there is no
tube to state and `certify_general` refuses `FittedMateMissing`.

The contrast inside one fixture is the clearest statement of the gap: on the SAME
widened chart, at two interior columns of the same wall,

- `9v1` is described as an `Intersection` (plane × NURBS) → mints `Pcurve::General`,
  certified, envelope `3.86e-14 m` at every ε in {1e-6, 1e-9, 1e-12};
- `16v1` is described as a `Chart` (the neighbouring wall's own image) → no exact
  class (interior column) and no operand pair (single-surface description).

The de Boor collapse extractor unblocks `16v1` specifically because it needs no
mate: it makes the EXACT iso class available at an interior column, where today the
hull bound rests on a boundary row being a control-net copy.

## Consequence today

A body carrying an interior-column seam BUILDS and its `Intersection` seam mints and
certifies, but `mint_pcurves` cannot complete the face, so the body does not validate
at rest — `validate_pcurves` requires a face's cache set to be COMPLETE. That is the
half of #498's acceptance criterion 1 that P-2 does not claim.

## Scope note

This is new kernel capability (a de Boor collapse extractor for interior iso-curves),
not a widening of existing arms, and it was deliberately left out of P-2's scope.

## Home

Named PCURVE exit-walk residue; that program is closed, and `pcurve_cache` is fenced out of both S-BOOL and S-CERT as Track Q ground, so it lands unowned under `work/issues/`.
