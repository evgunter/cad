---
id: angular-arms-are-an-untagged-lever-beside-a-typed-rate
kind: issue
title: The angular chart arms are untagged metres-per-radian levers beside a typed rate pair
status: open
opened: 2026-09-15
priority: P1
cost: E
---



## Finding

RATE-PAIR typed the parameter↔metres crossing (`SupSpeed`/`InfSpeed`,
`Margin::metered`/`metered_sup`) and its fix pass typed the first
channel of a chart where that channel IS a rate — `ChartArm::Rate` on
plane and spline charts, in `crates/topo/src/pcurves.rs`. What it did
NOT type is the other variant, and there is no type to reach for:

- `pcurves::chart_u_arm`'s angular kinds — cylinder `r`, sphere
  `|r·cos v|`, torus `|R + r·cos v|`, cone `|v·sin α|` — carried as
  `ChartArm::Angular(T)`, a bare scalar;
- `pcurves::azimuth_lever`, which produces the same quantity;
- `geom_brep::pcurve_cache::chart_arms_at`'s cone arm, minted a
  `SupSpeed` and read back through `.get()` into `Margin::levered` at
  `pcurve_azimuth_period` — the one place the sup tag exists only to
  be removed;
- `chart_windings`' arm, and `chart_bound`'s span check when the arm
  it is handed is `Angular`.

Metres per RADIAN is not metres per parameter unit, so neither half of
the rate pair names it and typing it as one would be a false claim —
that is the carve-out this row records, stated as what it actually is
rather than as "the levered door's". The question the kernel has not
answered is whether an angular lever wants its own tag (an
`AngularArm<T>`, or a bound-direction pair over one), or whether the
`ChartArm` enum's own discrimination is the whole of what the reader
needs.

The shape to avoid: an arm minted as a `SupSpeed` and immediately
`.get()`-stripped, which is what `chart_arms_at`'s cone arm does today.
Either the arm is a tagged rate all the way to the door, or it is
never tagged.

## Home

Filed by SCALAR's RATE-PAIR lane in its fix pass, as the successor of
`loop-continuity-meters-u-through-levered-and-v-through-metered`
(closed in the same PR), which named this as the class's next member.
TRIM owns `crates/topo/src/pcurves.rs`.
