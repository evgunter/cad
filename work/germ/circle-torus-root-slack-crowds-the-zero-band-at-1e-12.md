---
id: circle-torus-root-slack-crowds-the-zero-band-at-1e-12
kind: issue
title: bool_circle_torus_root_slack's zero-classified margins crowd the coincidence band at eps 1e-12 (8 k-lint rule-2 flags)
status: open
opened: 2026-09-29
priority: P2
cost: M
---

## What

`bool_circle_torus_root_slack`
(`crates/topo/src/boolean/circle_torus.rs`) classifies the root slack
of the circle x torus root lane. On `demo/lily_walls` eight of its
samples are classified `zero` with a margin above `band_zero / 10^2`,
so k-lint's rule 2 flags them — **only at eps 1e-12**, which is the
eps-coupled shape rule 2 exists to report.

From nightly run 36561506133 (head `aae5716bf`), job `k-lint
(dev-probe)`, the 1e-12 row (`band_zero = 1e-12`, so the rule's
threshold is 1e-14):

| shape | \|m\| (m) | count |
| --- | --- | --: |
| `demo/lily_walls` | 3.14912307142162e-13 | 2 |
| `demo/lily_walls` | 8.272757261396428e-14 | 2 |
| `demo/lily_walls` | 6.680730726416532e-14 | 2 |
| `demo/lily_walls` | 1.9291935961799184e-14 | 2 |

At 1e-9 and 1e-6 the same margins sit far below `band_zero / 10^2` and
rule 2 is silent, which is why the failing run's rule-2 count is 8 at
1e-12 and 0 at the other two rows.

## The margin is eps-independent BY CONSTRUCTION, not by observation

Four distinct values each recorded twice at all three rows is the
observation; the source is the argument, and it is the input a rule-4
decision turns on. `circle_torus_roots` builds

```rust
let slack = radius * (charge / slope + rounding / sin_spread);
```

from `radius`, the tilt `charge`, the residual's along-carrier `slope`
and `sin_spread` — all geometry — plus `rounding`, which is
`NOISE_ULPS * f64::EPSILON * 0.5` scaled by the contour/offset/radius
combination. `f64::EPSILON` is machine epsilon, not the tolerance.
**Nothing on this path reads `Tol` or the ambient eps**: every `Tol`
and `eps` occurrence in `crates/topo/src/boolean/circle_torus.rs` is
inside `mod tests`. The band the margin is classified against moves
with eps; the margin does not.

That matters for the ruling below in one direction: rule 4 is
calibrated for a statistic whose whole operating range SCALES with eps
(`props_quad_converged` records `1024*eps - width`), and this one does
not, so rule 4 as the M7 addendum derives it is not the shape that
fits here.

## This is a population change, not a distribution one

The predicate did not exist at the last green `k-lint (dev-probe)`
execution. `bool_circle_torus_root_slack` was minted at `987e97789`
(2026-09-28T17:02:22Z, "germ: circle x torus root lane, first cut")
and refined at `cb328fcbf`; PR #3375 merged it to main at `38588fede`,
2026-09-29T02:07:58Z. The last green execution is run 36449836735 /
job 109022798206 at 2026-09-28T16:16:28Z — before both. The same PR
rewrote `demos/tour/src/lily.rs`, which is the shape these eight
samples come from.

So no margin moved: a new family entered the census carrying a lower
tail that lands inside rule 2's proximity window at the tightest eps
row.

## What has to be decided

Whether a root slack at 10^-14 m that the lane classifies `zero` is a
real fragility statement at 1e-12 — rule 2 doing its job on a lane
whose slack should be metered or refused differently — or whether this
family needs a rule of its own. It is not `EPS_COUPLED_PREDICATES`
material as that list is derived today: membership there is explicit
and never inferred, and the section above says why rule 4's calibration
does not fit an eps-independent margin. Nothing happens to this family
until someone rules on it, and until then the `k-lint (dev-probe)`
nightly row is red on these eight.

## Home

GERM — the predicate, its lane and the lily scene rows are germ's.
Filed by the PROPS k-lint baseline unit, which measured the flags and
does not own the lane.

## The sphere sibling (2026-10-02, branch `reach/circle-sphere-slack`)

The circle × sphere instance
(`circle-sphere-root-slack-refuses-near-tangent-pairs-at-1e-12`) was
fixed by charging its root slack with the NEAR extreme's own error,
not the harmonics' whole term bound: the extremes evaluated factored,
`(D∓ − r)(D∓ + r)/2r`, with a first-order running rounding bound
(`Rounded` in `crates/geom-brep/src/implicit.rs`). That moved the
sphere's slack about six times lower on the near-tangent snowman
(1.40e-12 → 2.30e-13 at δ 1e-5). It does
not transfer as is to this lane, whose residual is a quartic with no
closed-form extremes, but the running bound can shadow `contour`,
`offset` and the radius combination this meter charges with
`NOISE_ULPS` half-ulps today.
