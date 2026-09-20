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
with `charge = f2·(Δθ/K)²/8` at `f2 = 0.2415 m/rad²` and
`K = ARC_RESIDUAL_SAMPLES = 256`: **1.83e-5 m** over a full meridian,
and **4.56e-6 m** over MATE-7a's own refusing edge, which is a half
meridian (the charge is quadratic in the span). Its one-sidedness
margin is `−charge`, never a `Zero`.

The refusal is raised at the rung's `Zero | Negative` arm
(`boolean/reduce.rs`, measured by instrumented backtrace on
`sweep/tests/mate7a_torus_rest.rs`'s
`the_admitted_torus_lane_stops_at_the_curved_pierce_frontier`), one
line below the `None` door it used to take — **and which typed
refusal comes out is eps-dependent**: at `DEFAULT_EPS = 1e-9` and at
`1e-12` the margin clears the escalation threshold and the rung
refuses `CurvedPierceUnsupported`; at `1e-6` it sits inside the
ambiguity band and `bool_circle_curved_clearance` escalates
(`Indeterminate { margin: -4.56e-6, band: { zero: 1e-6, escalate:
1e-5 } }`). A refusal whose VARIANT moves with the run's eps is a
second thing wrong with the coincident case, not a separate one: both
come from a margin manufactured by the enclosure's own charge rather
than by the geometry, which is identically zero.

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
