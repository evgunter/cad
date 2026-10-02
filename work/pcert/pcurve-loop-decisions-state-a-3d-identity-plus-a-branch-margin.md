---
id: pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin
kind: issue
title: the pcurve loop decisions (continuity, closure, pole joint) compare angles over a parameter box, so they widen there the way check 4 did
status: open
opened: 2026-10-02
priority: P0
cost: H
design: true
refs: [loop-walk-branch-is-an-opaque-floor-atom, 3781]
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
