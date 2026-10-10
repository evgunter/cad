---
id: the-census-crossing-lane-misplaces-a-shared-points-crossing-on-a-near-collinear-pair
kind: issue
title: The census's crossing lane reads a near-collinear pair's crossing through a closed form whose f64 error is ulp·|d|/θ: pairs meeting only at their shared point escalate pm_census_ee_span at ε = 1e-9 (982 escalations) and read a definite EdgeEdgeCross at ε = 1e-12 (845)
status: open
opened: 2026-10-08
priority: P0
cost: M
refs: [4335]
---

## What

Found by JOIN's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

`crates/topo/src/census.rs` `ee_cross_spans` reads the crossing parameters
as `d.cross(eb.dir).dot(n) / |n|²`, with `d = eb.p0 − ea.p0` (metres long)
and `n = ea.dir × eb.dir`. For two edges at angle θ the rounding in the
numerator is about `ulp·|d|·θ`, so the parameters err by about
`ulp·|d|/θ`. The pairs this lane reaches in near-tangent results share a
point key and are nearly collinear (sin θ 1e-8 to 3.4e-7). Exactly, their
lines cross at the shared point: the exact `sa`, `la − sa`, `sb`, `lb − sb`
are about 1e-52. In f64 they read 1e-9 to 1e-8 at ε = 1e-9.

- **ε = 1e-9:** 982 of the census's 988 `pm_census_ee_span` escalations
  over the probe's 28 800 runs (d = ±1e-7: 178, ±1e-8: 804; and 559 more
  at ±3e-7 and ±3e-8). The body is right; only tier 3′ fails.
- **ε = 1e-12:** the error now exceeds K·ε. 845 `UndeclaredContact
  EdgeEdgeCross` findings, all at a shared point by the exact parameters,
  every tilt from 1e-5 to 1e-11. The f64 crossing lies 1.0e-11 to 1.1e-5
  from it. These are definite false findings, so once the door gates at
  tier 3′ they refuse a right body.
- At ε = 1e-6 the error stays under ε and nothing escalates.

**Witness.** `notch307 nt e0 a0 d1e-7 pc I` (∩ at the oracle volume):
`EdgeKey(18v1)` (`(3.789, 1.894, 1)` → `v = (2, 1, 1)`)
and `EdgeKey(20v3)` (`(0.211, 0.106, 1)` → `v`). They are two vertices
on one point, collinear end to end, sin θ 1.0e-7. The f64 span reads
1.105e-9, and the exact span is 7.5e-54. At ε = 1e-12,
`notch307 nt e0 a0 d1e-6 cp I` reads `EdgeEdgeCross` `17v5`×`38v1` with
the witness 1.2e-11 off `v`.

**What it hides.** In 305 of the 982 escalations at ε = 1e-9, the two
edges leave the point the same way and stay within K·ε for 0.06 to 1.0
from it, at least 1% of the shorter edge. An exact reading would pass those pairs silently. That
sliver is JOIN's
`a-near-tangent-split-leaves-a-face-corner-that-runs-within-the-band`
(one vertex), and
`two-copies-of-a-pierce-carry-edges-that-run-within-the-band` (two
vertices on one point).

Repro: `NT_DUMP=1 cargo run -p sweep --release --example near_tangent_census_probe | python3 scripts/oracles/near_tangent_census_classify.py`, with `NT_ONLY`/`NT_POSE`/`NT_D` to pick the pose and `CAD_TOLERANCE_EPS` the row. The classifier reproduces each census margin bit for bit in f64 and recomputes it at 60 digits on the same coordinates.

## The shape to give

Take the crossing at the point the two edges share, rather than through
the closed form: a pair meeting at a shared point key has no interior
crossing there. Or read the parameters from a nearby origin, so `|d|`
is small. Either way, decide what the lane then owes the same-heading
pairs above: today the arithmetic is all that keeps them from passing.
