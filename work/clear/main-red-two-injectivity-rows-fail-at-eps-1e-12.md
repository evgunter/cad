---
id: main-red-two-injectivity-rows-fail-at-eps-1e-12
kind: issue
title: two self-intersection rows fail the nightly at CAD_TOLERANCE_EPS=1e-12: the tee pins an eps-dependent Interval union refusal, and the long helical sweep's f64 build escalates its iso-line rim mint
status: closed
opened: 2026-10-31
priority: P1
cost: M
closed: 2026-10-31
---


## Finding

The nightly's `eps=1e-12` row is red on main. The cause was found by the
FRUSTUM review (#6114) and reproduced on base `2d49bebbc7`.

- **The tee**, `a_cylinder_cylinder_tee_refuses_typed_at_a_vertex_its_junction_shares_alone`
  (`crates/sweep/tests/self_intersection_proves_injectivity.rs:1231`).
  The row pins the Interval union's known failure exactly:
  `Escalated { bool_ruled_region_span, invalid margin }`. At ε 1e-12 the
  union refuses earlier and differently:
  `CurvedPierceUnsupported { operand A, face 3v1, edge 11v1, band 1e-12/1e-11 }`.
  - So the pinned refusal depends on ε. It is still a typed refusal.
  - Whether the row's f64 half passes at 1e-12 is unchecked, because the
    panic comes first.
- **The helix**, `a_long_helical_sweep_certifies_within_the_default_budget`
  (`:749`). The f64 sweep fails to build at all. The
  `pcurve_iso_line_rim` mint (`crates/topo/src/pcurves.rs:1480`)
  escalates on half-edge 18v1 with margin `[0, 1.86e-12]`, inside the
  band `zero 1e-12 / escalate 1e-11`.
  - The helical rim's rounding residual (~2e-12) is larger than ε, so the
    iso-line test lands in the sliver.
  - This is ε-sensitivity in the sweep build, not in clearance.

## Class

A row that pins one exact refusal arm, run under the ε matrix, pins that
arm at every ε. That holds only if the arm is ε-independent, and nothing
checks it before the nightly does.

## Cause, as found

- **The helix's failing build was the `Interval` one, not `f64`.** The
  margin is an enclosure (`MarginDiag::Enclosure { lo: 0, hi: 1.86e-12 }`),
  which `f64`'s `Decide` never reports; `helical::<f64>()` built. The
  path sweep's lane frames were carried along the path at `Interval`
  (~3e-14 wide), while the walls were the lifted `f64` skin. The rim
  carrier and the wall row it lies on came from two computations, so
  the probe's distance was an interval-dependency bound: `u = t / span`
  over an uncertain span, through a row `(1 − u)·P0 + u·P1`. Since
  #6138 the carry is one `f64` computation composed onto each lane's
  start frame, and the row builds and passes at every ε with no change
  here.
- **The tee** pinned one ε's refusal arm. It now pins that the union
  refuses typed.
