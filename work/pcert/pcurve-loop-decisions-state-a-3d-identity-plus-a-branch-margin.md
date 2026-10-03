---
id: pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin
kind: issue
title: the chart's angular comparisons (loop continuity, closure, pole joint, trim containment) compare angles over a parameter box, so they widen there the way check 4 did
status: open
opened: 2026-10-02
priority: P0
cost: H
design: true
refs: [loop-walk-branch-is-an-opaque-floor-atom, 3781]
needs_ev: true
---

The follow-on the incidence-and-fidelity unit names, filed when that
unit landed its check 4.

`topo::pcurves`' loop checks decide on differences of chart angles:
`pcurve_loop_continuity` (`entry.u − prev.u` at a joint),
`pcurve_loop_closure` and `pcurve_loop_pole_joint`. Over a parameter
box no arithmetic sees through those differences. On M10-7's plate at
`1e2·ε` (shipped set) the counts are:

| decision | theorem | numeric |
|---|---|---|
| `pcurve_loop_continuity` | 12 | 12 |
| `pcurve_loop_closure` | 0 | 4 |
| `pcurve_loop_pole_joint` | 0 | 12 |

The demo tour's `chaintol` is bounded here. When `transform_rigid`
re-certifies the rows of a placed link, the walk refuses at
`pcurve_loop_continuity` (enclosure about `±3.6e-5` at 2 links). The
certifiable fractions measure `[6.5e-7, 2.2e-7, 1.1e-7, 6.8e-8]`
against the published `[1.0, 0.37, 0.19, 0.11]`, with or without check
3 in the box certificate.

Both designers on PR 3759 proposed the same restatement. A joint is a
3-D coincidence, which is already a theorem or registered, plus a branch
margin `|Δu| < π`, a real margin with π of room: `S(u₁, v) = S(u₂, v)`
forces `u₁ − u₂ ∈ τℤ`. Closure is the loop's winding count, also
integer-valued with margin π. The pole joint is unchecked. The walk's
opaque branch (`loop-walk-branch-is-an-opaque-floor-atom`) is a likely
first step, since the continuity margin reads the shifted row too.

## Scope widened: trim containment (measured on PR 3812's head)

Measured on PR 3812's head `47f6f723`, in a scratch build with
env-gated drops (nothing committed). These are the demo tour's three
`chaintol` rows at ε = 1e-9:

- **The loop's angle margins dropped** (`pcurve_loop_continuity` in
  `pin_branch` and at rest, `pcurve_loop_closure`/`_height` in the
  chord walk and `loop_closes`): still red, with the fractions
  unchanged at `[6.51e-7, 2.22e-7, 1.12e-7, 6.75e-8]`. The next wall is
  check 5, `pcurve_trim_containment` on `HalfEdgeKey(10v1)` under
  `transform_rigid`, with an enclosure of ±3.55e-5 at 2 links. That is
  the same angle-difference class: the stored azimuth's chart box
  against the `ChartWindow` edge.
- **The loop's margins and trim containment dropped:** all three rows
  pass, the published fractions `[1.0, 0.3702, 0.1851, 0.111]`
  reproduce, and the wall is `dihedral_wedge`, as `chaintol`'s header
  says.
- **Check 4 is not a wall.** Its envelope on the placed rows (whose
  incidence would need `Rᵀ·R = I`) does not bound `chaintol` at the
  published fractions.

So the unit is the chart's angular comparisons, not only the loop's:
loop continuity, closure, the pole joint, and trim containment's
window test on a periodic azimuth channel. Each states a branch margin
over a 3-D identity or a window identity.

The trim-window part touches C4's "trim containment against the
caller's `ChartWindow`" (`crates/geom-brep/README.md`), so it is the
designer pair's question, weighed after 3759 and 3812 land.
