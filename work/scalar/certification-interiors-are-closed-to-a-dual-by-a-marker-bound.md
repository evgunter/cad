---
id: certification-interiors-are-closed-to-a-dual-by-a-marker-bound
kind: unit
title: Certification interiors are closed to a dual by type: the Certifies marker bound, lanes declared over it, and the scope rule reworded
status: closed
opened: 2026-11-20
priority: P1
cost: H
parent: from-computed-claims-a-zero-tangent-at-a-dual
closed: 2026-11-21
pr: 7190
---

The rule is `crates/geom-core/README.md`, "Reading a bracket where nothing certifies" (DL5.2, DL5.3), ratified by Ev on PR 7155. The site counts below are two designers' readings (`work/scalar/log.md`, 2026-11-20), mostly not traced to consumers: re-measure before building on one.

## Scope

- A sole marker trait `Certifies` in `geom-core`: sealed, no methods, `Decide` as supertrait, **no bracket right**; implemented by `f64`, `Probe`, `Interval` and `Sym`, not by `Dual`, with a compile-fail pin.
- Every lane type declared over it: `NurbsLane`, `FittedLane`, `SplineFoot`, `SectionLane`, `OffsetFitLane` (`T: Real` today) and `QuadLane` (`T: Decide`). The compiler then asks the bound of every function that names a lane; follow it inward.
- `AtRestPolicy`'s lanes become an associated type with no values at a dual, in place of the `Option` its `Dual` impl answers `None` to (`topo/src/props.rs`).
- A gate under `scripts/gates/`: files listed as certification code, where a generic item without `Certifies` fails. Open the list with `pcurve_cache.rs`, `pcurve_cache/projected.rs`, `pcurve_cache/spline_incidence.rs`, `geom-brep/src/certify.rs`, `ssi/certify.rs`, and whatever the lane bound did not reach.
- The `Bounds` scope rule in `real.rs` reworded, and `bounds-allowlist.sh` with it. Ev, PR 7155: "weakest bound that works ... should be updated to have more of a focus on semantic correctness", never "broaden it to anything that doesn't error", and "the exclusion is free and only the bounds reading needs the extra check". Text to start from: *A bound states which scalars the code means something at. Write that bound, and never a wider one because it compiles. Excluding a scalar is free. What needs an entry here is a bracket read in code that also decides: the entry shows the read is necessary and that it decides nothing.* The gate fires on a bracket-reading trait beside a deciding one, never on `Certifies`. Ledger entries that argue "the next tighter bound breaks its caller" (SEAT-4 is one) are re-stated as necessity of the read.

## Not this unit

Any `from_computed` or `from_f64` site. `carrier_span` and the other "fenced by its callers" functions take the bound here; their re-entries are the sibling units'.

## Measure first

Which construction callers instantiate a certification-interior item at a dual today (the dual corpus and `blend_dual_sensitivity` compile at `Dual<f64>`): each one is a function to split, not a reason to leave the bound off. Report the list before the sweep.

## Decides

Whether an interior that reads a bracket through a bound it inherits keeps its allowlisted compound beside `Certifies`. The name `Certifies` must not match the bracket gate's `…Bounds`/`…Enclosure` pattern.

## Review

Letter H, tier dual: a trait every certifying function will name, a change to what a gate checks, and a reworded rule; broad and hard to change later.
