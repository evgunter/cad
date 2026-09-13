---
id: torus-coincident-pair-cannot-reach-the-covered-rung
kind: issue
title: A coincident torus pair cannot reach the declared-cover rung: the sampled enclosure's chord-dip charge outruns every band
status: open
opened: 2026-09-13
---


## What

CURVED-TORUS PR-2 gave the circle rung a torus arm, so
`reduce.rs`'s `bool_circle_curved_clearance` now DECIDES on torus
pairs instead of taking the frontier on a `None`. On a COINCIDENT
pair it decides definitely-**Negative**, and the declared-cover rung
behind it — which is reached only on a `Zero` — stays out of reach.

**The measurement** (`crates/geom-brep/tests/curved_torus_arc_residual.rs`,
`a_coincident_torus_pair_encloses_pm_charge_and_reads_negative`).
MATE-7a's fixture is two identical tori, `R = 5`, `r = 0.06`. The
edges are seam meridians of the torus they ride, so the residual is
identically zero along them and the sampled enclosure is `±charge`
with `charge = f2·(τ/K)²/8 = 1.8e-5 m` at `f2 = 0.2415 m/rad²` and
`K = ARC_RESIDUAL_SAMPLES = 256`. Its one-sidedness margin is
`−1.8e-5`, which is a definite Negative at all three eps cells of the
run matrix (`DEFAULT_EPS = 1e-9`, `1e-6`, `1e-12`). The refusal is
therefore raised at the rung's `Zero | Negative` arm
(`boolean/reduce.rs`, measured by instrumented backtrace on
`sweep/tests/mate7a_torus_rest.rs`'s
`the_admitted_torus_lane_stops_at_the_curved_pierce_frontier`), one
line below the `None` door it used to take.

**Why raising `K` does not fix it.** The charge falls as `K⁻²` and
the band does not move with it, so there is always a `K` at which a
coincident pair reads Zero — but it is set by the band, not by the
geometry: at `ε = 1e-12` it needs `K > 4e3` on this fixture and more
on a larger torus. Chasing it is the wrong shape. What a coincident
pair wants is a verdict that is not a sampled enclosure at all: a
carrier-identity rung (`carrier_eq`'s torus arm, which MATE-7a
landed) reached BEFORE the numeric clearance, or the declared cover
consulted on a Negative as well as on a Zero.

**Not PR-2's fence.** PR-2's fence is `implicit.rs` and the one
`reduce.rs` fold; changing which rung the declared cover is consulted
from is a reduction-order change and belongs with the gate-policy
unit.

## Home

CURVED — `torus-operand-gate-admission` is the unit that carries the
stem glue's door sequence and is where this lands; filed from
CURVED-TORUS PR-2.
