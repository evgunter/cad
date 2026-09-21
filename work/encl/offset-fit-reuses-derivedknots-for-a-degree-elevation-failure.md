---
id: offset-fit-reuses-derivedknots-for-a-degree-elevation-failure
kind: issue
title: Composite::build reports a failed degree elevation as PatchBoundError::DerivedKnots, so the message describes a derivative knot vector that was never built
status: open
opened: 2026-09-21
---


(FIX implementer, filed from the second-hop recourse unit, which gave
`PatchBoundError`'s arms their repairs and so had to read what each one
means at every site that mints it.)

## What

`Composite::build` (`crates/geom-brep/src/offset_fit.rs`) raises a
base whose `u` or `v` direction is below degree 2, because the
composite needs a derived knot vector and a degree-1 direction has
none. Both elevation calls report failure as

```rust
.map_err(|_| OffsetFitError::PatchBound(PatchBoundError::DerivedKnots))?;
```

`DerivedKnots` renders `patch_bound`'s own note — *"NURBS direction
whose derivative knot vector fails to materialise"* — so a failed
**degree elevation** prints a sentence about a derivative knot vector
that was never asked for. The two are different failures of different
operations: `derived_knots` is called downstream at line ~1971 and has
its own route to the same variant.

The recourse clause this unit added to the note is written to be true
of both readings (it says to report the description rather than to
repair one, which is right for an elevation failure too), so the
message is not actively wrong today — but the variant is still naming
the wrong operation, and a reader chasing "derivative knot vector" in
an elevation failure is chasing nothing.

## Repair shape

Either a `PatchBoundError` arm for a refused elevation, or —
better, since the failure is `geom`'s `KnotAlgebraError` and not a
patch-bound structural refusal at all — an `OffsetFitError` arm that
carries it. `OffsetFitError` already carries three other typed
carriers whole.

## Fence

`crates/geom-brep/src/offset_fit.rs` is **ENCL's**.
