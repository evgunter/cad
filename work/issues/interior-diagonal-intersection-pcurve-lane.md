---
id: interior-diagonal-intersection-pcurve-lane
kind: issue
title: nurbs_iso_derive — interior/diagonal Intersection carriers (the trimmed-NURBS/cut-loft pcurve lane)
status: open
opened: 2026-08-15
github: 498
refs: [427]
---

## From GitHub issue 498

opened 2026-08-15, 0 comments.

## The class

`nurbs_iso_derive`'s `Intersection` arm (M8-4) covers exactly one
residency: a carrier that lies on a chart **boundary column**
(`u = u₀` or `u = u₁` of the payload's own knot domain), which is the
wall–wall seam class the certification lane's SEAM arm covers. Anything
else refuses typed and permanently:

```
IsoUnsupported { what: "an Intersection carrier on neither boundary column of this
chart — an INTERIOR or DIAGONAL intersection locus, which has no boundary-row
closed form (the trimmed-NURBS/cut-loft pcurve lane is that unit's)" }
```

## What is excluded

An `Intersection` whose locus crosses the chart's interior — the
trimmed-NURBS / cut-loft lane. Two sub-classes, one blocker each:

1. **Interior isos.** A locus that is an iso of the chart but not a
   boundary one. The certification side already names the blocker at
   `pcurve_cache.rs`'s `side_of`: "boundary rows are control-net copies,
   an interior iso needs the de Boor collapse extractor — which arrives
   with the construction that first mints one".
2. **Diagonal / general loci.** A genuine SSI curve across the patch.
   No `Pcurve` variant holds a general curve in UV today; that
   representation question is #427's (pcurve unification, an M9 design
   item) and this issue must not pre-empt it.

## Why it is a named residue rather than a gap

No construction mints such an edge today: STEP adoption's
declare-and-check rung is the only producer of `Intersection`-on-NURBS
edges, and the shapes it produces are wall seams. The class becomes
reachable with the cut-loft / trimmed-NURBS unit (a plane cutting a
NURBS wall), and the pin that fires the day the arm over-reaches is
`sweep/tests/m8_4_intersection_iso.rs::an_interior_column_intersection_refuses_typed`.

## Acceptance when it is taken up

- The extractor (or the general-UV representation, per #427's ruling)
  with its own certification class — the current seam class compares the
  carrier against the chart's own boundary ROW and cannot serve here.
- The interior negative control above FLIPS to a positive row.
- The refusal text above loses the excluded sub-class it no longer
  refuses.

(M8 orchestrator program)

## Home

`work/issues/`: `topo::pcurves::nurbs_iso_derive` and `pcurve_cache` are ground the closed PCURVE program vacated; S-CERT's territory stops at `geom-brep/src/props/*` and does not reach the pcurve mint.
