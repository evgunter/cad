---
id: ssi-box3-width-drops-a-refused-axis
kind: issue
title: ssi/enclose: Box3::width folds its three axes with f64::max, so a refused axis drops out of the sweep's floor test
status: open
opened: 2026-10-01
---


## The finding

`crates/geom-brep/src/ssi/enclose.rs`'s `Box3::width` is

```rust
self.x.width().max(self.y.width()).max(self.z.width())
```

`Certification::width` answers `NaN` for a refused side, and the
inherent `f64::max` returns the other operand when one is NaN. A box
with one refused axis therefore reports the width of the other two,
and a box with all three refused reports NaN only if the last two
are. Its consumer is `ssi/exhaust.rs`'s sweep: `cell.width() <= floor`
decides whether a cell is at the floor. On `SweepDuty::Account` a
dropped axis can read a cell as floor-sized and refuse
`ExhaustivenessInconclusive` with a `cell_width` that describes two
axes. On `SweepDuty::Seed` it pushes `cell.seed()`, whose centre is
NaN on the refused axis.

## Reachability

Low today. The sweep's cells are driver-minted boxes (`from_bounds`
and `split`), which do not refuse unless the domain itself carries a
NaN end. That NaN-window case is the one the chart-speed design
(`ssi-chart-speed-usability-boundary.md`, part 5) closes at
`NurbsBoxes::cells`. The fold is still the NaN-dropping shape the
lever-arm row (`ssi-lever-arm-min-fold-hides-poison.md`) and the
rate-fold unit fix elsewhere. It is a width rather than a rate, so
neither of them covers it.

## The fix shape

Fold with the NaN-propagating max the lever-arm unit lands, or ask
`is_certified()` on each side and answer NaN. A NaN width then fails
`<= floor` and the cell is refined until the budget answers. Whether
that is the right refusal, or whether a refused cell should refuse by
name, is the fixer's call.

Found by the `ssi/chart-rate` lane's §5 sweep for NaN-dropping folds
on SSI ground.
