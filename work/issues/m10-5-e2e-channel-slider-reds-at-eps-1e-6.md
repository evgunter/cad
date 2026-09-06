---
id: m10-5-e2e-channel-slider-reds-at-eps-1e-6
kind: issue
title: m10_5_r1 e2e_channel_slider_over_an_epsilon_box reds on the interval lane at eps = 1e-6
status: closed
opened: 2026-09-03
closed: 2026-09-03
pr: 1670
---


## What

`e2e_channel_slider_over_an_epsilon_box`
(`crates/editor-core/tests/m10_5_r1_probes_interval.rs:867`) asserts, on
every `Violated` verdict of its sweep,

```rust
let d = recomputed_distance(&report);
assert!(d < c && d >= 0.5 - 1e-9, "{d}");
```

At `CAD_TOLERANCE_EPS=1e-6` on the interval lane the recomputed distance
comes back **0.4999999687499974**, which clears `d < c` but misses the
lower bound `0.499999999` by 3.1e-8 — about thirty times the 1e-9 slack
the row allows, and about a thirtieth of the 1e-6 tolerance the run is
configured with. The panic message is the bare distance.

## Where it was seen

Hosted run **33739184479**, jobs 100600670357 (`test (interval, eps =
1e-6, 1/2)`) and 100600670350 (2/2), on the `refs/pull/1669/merge`
commit — i.e. `main` merged with a branch whose diff touches no
`editor-core` file at all. Reproduced locally on `main`'s content with
`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p editor-core --features
editor-core/interval -E 'test(e2e_channel_slider_over_an_epsilon_box)'`:
same row, same distance.

`main`'s own recent runs did not draw this point — every one of them was
a tracker or docs push whose test shards were skipped — so the row has
not been gated at (interval, 1e-6) since it landed.

## Why it is not an eps-band annotation

The row's own bound is written as an absolute 1e-9 against a nominal
0.5 m clearance. That constant is independent of the tolerance the run
is configured with, so the row asserts the same tightness at 1e-12 and
at 1e-6 while the quantity it measures moves with eps. Either the bound
belongs on the run's tolerance, or the sweep should not be reporting
`Violated` at this eps — which of the two is the M10-5 clearance unit's
call, not a reviewer's. Do not widen the constant without deciding which.

## Not this unit's

Found by TCOST-B2 (PR 1669) as an inherited red: that branch's diff is
`crates/{step-export,step-import,stl,geom,geom-core,bvh,geom-brep,verbs}/tests/`
only, and `git diff origin/main...HEAD -- crates/editor-core` is empty.

## Closed (2026-09-03, PR 1670 — fixed before this item was read)

Fixed on main at `3de2e3d9` by the M10-6 lane's k-probe hotfix, which
hit the same red from the other side: that branch PINNED `eps=1e-6` to
verify a k-lint row and found this row and its sibling in
`m10_5_clearance_interval::a_bound_over_the_parameter_band_is_violated`.

The diagnosis this item asks for, filled in: the floor is an absolute
`1e-9` against a box that is ε-RELATIVE (ε/32 wide, so 3.125e-8 at
1e-6), and a carrier window is a superset of its face by about the width
of the parameter box it was enclosed over — so a witness station can sit
that far outside the face and the recomputed distance undershoots the
nominal gap by the same order. The window superset is
`clearance.rs`'s stated looseness, not a defect in the engine; the
defect was the row's absolute floor.

Both floors are now `4.0 * half()` — the box's own width — with the
mechanism and the measurement at the site. Verified at 1e-6, 1e-9 and
1e-12: 287 m10 tests green at 1e-6.
