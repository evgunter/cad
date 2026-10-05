---
id: steep-ellipse-gating-row-misses-its-volume-oracle-at-eps-1e-6
kind: issue
title: steep_ellipse_poses_build_sound_or_refuse_typed is red on main at ε 1e-6: 11 sound poses miss the volume oracle by ~2e-7 relative
status: open
opened: 2026-10-04
---

Found by REACH running the full battery at three ε on PR 3977's merge
with main (`223c5749`, main at `e4a0a0d1`), and reproduced on main
itself: the row's output is identical on both trees.

## Finding

`crates/sweep/tests/pocket_ring_steep_ellipse.rs`
`steep_ellipse_poses_build_sound_or_refuse_typed` passes at ε 1e-9 and
1e-12 and fails at ε 1e-6 with 11 misses. Every miss is a pose the row
expects `Sound`, whose body builds and passes tier 2, tier 3′ and the
certificate, and whose volume stands off the oracle by about 2e-7 m³:

```
11 runs miss:
theta=60 x=(-0.9499999999999997, 0.05) mirror=false U AB (Sound): body t2=true t3p=true cert=true v=Ok(3.2679290807236097) want=3.267929297735445
theta=60 x=(-0.9499999999999997, 0.05) mirror=false U BA (Sound): body t2=true t3p=true cert=true v=Ok(3.267929453371701) want=3.267929297735445
theta=60 x=(-0.9499999999999997, 0.05) mirror=false S AB (Sound): body t2=true t3p=true cert=true v=Ok(0.063504573363085) want=0.06350479107385605
theta=60 x=(-0.9499999999999997, 0.05) mirror=false S BA (Sound): body t2=true t3p=true cert=true v=Ok(3.1879294533717) want=3.1879292977354448
theta=60 x=(-0.9499999999999997, 0.05) mirror=false I AB (Sound): body t2=true t3p=true cert=true v=Ok(0.0164949958548511) want=0.016495208926143937
theta=70 x=(-0.05, 1.1695217600652346) mirror=true U AB (Sound): refused Escalated { decision: PierceCurvature, diag: Indeterminate { margin: MarginDiag(Value(3.471483053123673e-6, None)), terminal_sliver: false, band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 }, predicate: Some("bool_pierce_sector_side_curved") } }
theta=70 x=(-0.05, 1.1695217600652346) mirror=true U BA (Sound): refused Escalated { decision: PierceCurvature, diag: Indeterminate { margin: MarginDiag(Value(3.471483053123673e-6, None)), terminal_sliver: false, band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 }, predicate: Some("bool_pierce_sector_side_curved") } }
theta=70 x=(-0.05, 1.1695217600652346) mirror=true S AB (Sound): refused Escalated { decision: PierceCurvature, diag: Indeterminate { margin: MarginDiag(Value(3.471483053123673e-6, None)), terminal_sliver: false, band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 }, predicate: Some("bool_pierce_sector_side_curved") } }
theta=70 x=(-0.05, 1.1695217600652346) mirror=true S BA (Sound): refused Escalated { decision: PierceCurvature, diag: Indeterminate { margin: MarginDiag(Value(3.471483053123673e-6, None)), terminal_sliver: false, band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 }, predicate: Some("bool_pierce_sector_side_curved") } }
theta=70 x=(-0.05, 1.1695217600652346) mirror=true I AB (Sound): refused Escalated { decision: PierceCurvature, diag: Indeterminate { margin: MarginDiag(Value(3.471483053123673e-6, None)), terminal_sliver: false, band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 }, predicate: Some("bool_pierce_sector_side_curved") } }
theta=70 x=(-0.05, 1.1695217600652346) mirror=true I BA (Sound): refused Escalated { decision: PierceCurvature, diag: Indeterminate { margin: MarginDiag(Value(3.471483053123673e-6, None)), terminal_sliver: false, band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 }, predicate: Some("bool_pierce_sector_side_curved") } }
stack backtrace:
```

The row reads volume against its oracle with an absolute tolerance of
`1e-7` m³ (`pocket_ring_steep_ellipse.rs:632`, `miss(..., 1e-7)`, applied
at `:509` as `(v - volume).abs() < vol_tol`), whatever the run's ε. At
ε 1e-6 the built bodies sit ~2e-7 m³ (about 7e-8 relative on a 3.27 m³
body) from the closed form: a deviation of the order of the run's ε,
which the fixed tolerance does not scale with. Likely the tolerance and
not the bodies; whether each body is right to the band is for JOIN to
confirm.

## Fix

Scale the volume tolerance with the run's ε (or with the band over the
body's surface area), so a correct body at ε 1e-6 is read as sound, and
check that 1e-9 and 1e-12 still bound the misses this row exists to see.


## Reproduced independently (2026-10-04)

PR 3985's three-ε battery (`reach/arc-from-pairing`) found the same
11 misses on main `e4a0a0d18` alone, with the same values (θ 60,
`x = (−0.95, 0.05)`, A ∪ B 3.2679290807 against 3.2679292977). Its
lane filed a second item for it, folded into this one at that PR's
merge.
