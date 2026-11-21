---
id: volume-bracket-ends-are-t-fields-built-from-f64-bounds
kind: issue
title: MassProperties' volume bracket ends and quadrature midpoints are T fields built from f64 bounds
status: open
opened: 2026-11-20
---

Found building `a-bound-to-decide-against-never-becomes-a-t`, whose scope names these fields as `f64` fields (DL5.2, `crates/geom-core/README.md`).

## What

- `topo::props::MassProperties::enclosure` (`crates/topo/src/props.rs`) builds `VolumeEnclosure { volume_lo: volume - pad, volume_hi: volume + pad, .. }` with `pad = T::from_computed(self.volume_pad)`.
- `deciding_walk` (same file) builds a `VolumeEnclosure` from the interval re-derivation's ends, `T::from_computed(volume.lo())`, `volume.hi()`, `area.hi()`.
- The quadrature sum's midpoints enter `T` at the face flux accumulation (`c.flux + T::from_computed(mid)`, and `flux`/`area` from `fc`/`ac`).

## Why it was not moved in that unit

The unit says the fields become `f64`. Two things stand in the way, read in the code:

- `volume` is a measured `T` (a stackup's dual measures it, and its tangent is real), so `volume ± pad` as an `f64` needs a bracket read of `MassProperties::volume`, which `MassProperties<T: Real>` does not have (`Bounds` is the gated right).
- The ends are operands, not only decided: `SignReading::ends`, the `+V` check's lever (`volume_lo` and `surface_area` meet in an `over_lever` quotient), and the width reads (`volume_hi - volume_lo`). An allowance on a margin (`Margin::less`) cannot be divided by a surface area.

## The question it needs

Either the enclosure keeps `volume: T` and its pad as `f64` and every reader decides through `Margin::of(volume).less(±pad)` (the quotient read then needs its own door), or the bracket read on the volume is granted where the enclosure is built. The second is a `Bounds` scope question, so it is the scope rule's (`certification-interiors-are-closed-to-a-dual-by-a-marker-bound`).
