---
id: meter-error-normal-floor-carries-its-sup-lever-untagged
kind: issue
title: MeterError::NormalFloor carries its sup speed lever as a bare f64 beside a typed PatchRegularity
status: open
opened: 2026-10-24
priority: P3
cost: E
---


## Finding

`crates/geom-brep/src/offset_meters.rs`, `MeterError::NormalFloor`'s
`speed_lever: f64` is the patch's faster chart speed, a SUP rate
(metres per parameter unit), read off `PatchRegularity::speed_lever()`,
which answers a `SupSpeed<f64>`. The receipt strips it with `.get()`
where it is built (`offset_normal_floor`, `speed_lever:
reg.speed_lever().get()`). It is a public error payload, so a reader of
the receipt gets no direction on the number.

The obstacle is real, not scope: `MeterError` derives `PartialEq`, and
`SupSpeed` has no `PartialEq` and will not get one (no `==` on a tagged
rate, `geom_core::predicate`'s rate-pair docs). Typing the field means
hand-writing `PartialEq` for a public error, which is a rate comparison
outside the classify seam, or dropping the derive. `Display` is not the
blocker: its text survives a `.get()`.

Recorded on `certified-arms-are-an-untagged-inf-rate-beside-a-typed-pair`
(CHART) as a member of that class. CHART's arms-typing PR typed the
chart members and left this one, which is in ENCL's ground, so it is
filed here.
