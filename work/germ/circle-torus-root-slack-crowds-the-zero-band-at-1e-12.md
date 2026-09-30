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

| shape | |m| (m) | count |
| --- | --- | --: |
| `demo/lily_walls` | 3.14912307142162e-13 | 2 |
| `demo/lily_walls` | 8.272757261396428e-14 | 2 |
| `demo/lily_walls` | 6.680730726416532e-14 | 2 |
| `demo/lily_walls` | 1.9291935961799184e-14 | 2 |

The margins themselves are eps-INDEPENDENT — four distinct values,
each recorded twice — so they are a fixed geometric quantity that only
crowds the coincidence band once eps is tight enough. At 1e-9 and 1e-6
the same margins sit far below `band_zero / 10^2` and rule 2 is
silent, which is why the failing run's rule-2 count is 8 at 1e-12 and
0 at the other two rows.

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
real fragility statement at 1e-12 (rule 2 doing its job on a lane that
should be metered or refused differently), or whether this family
belongs under a calibrated rule of its own the way
`props_quad_converged` does (`EPS_COUPLED_PREDICATES` and rule 4,
`docs/K-REPORT.md`'s M7 addendum). Membership of that allow-list is
explicit and never inferred, so nothing happens to this family until
someone rules on it. Until then the `k-lint (dev-probe)` nightly row
is red on these eight.

## Home

GERM — the predicate, its lane and the lily scene rows are germ's.
Filed by the PROPS k-lint baseline unit, which measured the flags and
does not own the lane.
