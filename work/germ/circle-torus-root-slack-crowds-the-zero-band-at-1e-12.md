---
id: circle-torus-root-slack-crowds-the-zero-band-at-1e-12
kind: issue
title: bool_circle_torus_root_slack's zero-classified margins crowd the coincidence band at eps 1e-12 (8 k-lint rule-2 flags)
status: closed
opened: 2026-09-29
priority: P2
cost: M
branch: germ/klint-zero-arm-tally
closed: 2026-10-10
refs: [sphere-region-root-slack-crowds-the-zero-band-at-1e-12]
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

## Measured on main (2026-10-10, `af7a95b09`)

**The flags are still there, bit for bit.** PR 4357's `RootSlack`
meter (`bool_circle_torus_sub_root_slack`) is on the certified
subdivision only; `parallel_axes_roots` (`circle_torus.rs`), the lily's
pose, still decides `bool_circle_torus_root_slack` as written above.
The nightly cannot say so: last night's `k-lint (dev-probe)` (run
38027799934, head `b9bbd645`) died in the sweep's plain pre-pass
(`editor-core::assemble_one_local_battery`, `EmptyProduct`) before any
dump, so the lint step did not run. Measured locally instead: the
dumps `scripts/k_probe_sweep.sh` merges (editor-core `m4_pr8_k_probe`
+ the tour's `k-probe`) at ε 1e-6, 1e-9 and 1e-12, linted by
`tools/k-lint` exactly as the nightly step does.

Rule 2's zero arm, every flag, all programs (it is silent at 1e-6 and 1e-9):

| ε | predicate | shape | \|m\| | count | owner |
| --- | --- | --- | --- | --: | --- |
| 1e-12 | `bool_circle_torus_root_slack` | `demo/lily_walls` | 3.149e-13, 8.273e-14, 6.681e-14, 1.929e-14 | 2 each | GERM |
| 1e-12 | `bool_circle_torus_meridian` | `demo/lily_walls` | 1.0416e-14 | 1 | GERM |
| 1e-12 | `bool_sphere_region_roots_slack` | `demo/lily_walls` | 1.5093e-14 | 1 | HONE (`refs`) |

Every GERM row the sweep records is the same at all three ε: the
circle × torus parallel-axes rows, the meridian row, and
`bool_conic_quadric_first_*`. The nearest unflagged zero tail is
`bool_conic_quadric_first_root_slack` at 6.51e-15, 0.65 of the 1e-14
threshold. **The subdivision meters record no samples at all**:
`bool_circle_torus_sub_root_slack`, the ellipse × torus rows and the
section-certificate cone and torus rows. The corpus and the tour never
reach them, so k-lint says nothing about PR 4357's or PR 4522's meters
either way.

**Each of the eight is pure rounding.** I instrumented the slack line
in a scratch build (not committed). On every sample `tilt = 0`, so
`charge = 0` and `radius·charge/slope = 0`. The whole margin is
`radius·rounding/sin_spread`, where `rounding` is
`HARMONIC_NOISE_ULPS` (16) half-ulps of the `acos` argument's terms:

| slack | radius | contour | `sin_spread` |
| --- | --- | --- | --- |
| 3.149e-13 | 4.94 | 1.048 | 0.0294 |
| 8.273e-14 | 4.94 | 1.152 | 0.1127 |
| 6.681e-14 | 1.048 | 4.94 | 0.1388 |
| 1.929e-14 | 1.152 | 4.94 | 0.4833 |

(offset 3.9 on all.) So the statistic is a machine-ε rounding bound,
amplified by `1/sin spread` at a near-tangent crossing. Its operating
point is ~1e-14 to 1e-13 on a scene of this size. Rule 2's window at
1e-12 is `[1e-14, 1e-12]`.

## Not (a): no metering fix clears it

The sphere sibling's fix shadows the inputs with a first-order running
bound (`Rounded`). Here that would replace 16 half-ulps with the
expression's actual few, a ~4-5x cut. That clears the two 1.93e-14
samples. The 3.15e-13 pair is 31x over and stays flagged: its
`1/sin_spread` is a real conditioning cost of placing a root by
`acos` near ±1, not looseness. Refusing past `band/10²` instead would
turn the lily's walls `Uncertain` at 1e-12, which is geometry chasing
the lint. Tightening the bound is still worth doing on its own merits,
but it is not the answer to this row.

## So (b): the question for the ruling

Rule 2's zero arm assumes a zero-classified margin is a geometric
coincidence, so that one near `band_zero` is a feature the band nearly
resolves. A rounding meter (a root slack, a noise meter) is zero by
construction, and its value is the arithmetic's error at that point.
At ε = 1e-12 every such meter on a metre-scale scene sits in the
window, so rule 2 is reporting machine ε, not ε.
`PROXIMITY_FACTOR`'s own docs already name "a fixed rounding-scale
coincidence" at 1e-12 as its binding ceiling. Options:

1. **A rounding-meter roster** beside `EPS_COUPLED_PREDICATES`
   (explicit membership, matching that list's policy). Its rows skip
   rule 2's zero arm, or are judged against a comparand of their own,
   e.g. the band itself, where a slack past `band_zero` already
   refuses in the kernel. Members today: `bool_circle_torus_root_slack`,
   `bool_sphere_region_roots_slack`, the `*_root_slack` and `*_noise`
   rows. Cost: a list to maintain, and the policy that a row's
   membership is argued from its source, as this item does.
2. **A cross-row rule instead of a roster**: k-lint reads all three ε
   files at once, so it can see that a zero-classified margin has the
   same value at every row and is therefore not ε-coupled. Such a
   margin is judged on an absolute scale, not `band_zero/10²`. No list,
   but it infers membership, which the current lint deliberately never
   does. It also drops `bool_circle_torus_meridian` (1.04e-14, the same
   at every ε: a rounding-scale coincidence on a true meridian), which
   a roster of slack meters would not.
3. **Status quo**: rule 2 is right, and a root placed within a third
   of the band at 1e-12 is a fragility to fix in the kernel. Per the
   section above, the only kernel fixes are a tighter bound (2 of 8)
   or refusal (geometry chasing the lint). So this keeps the dev-probe
   row red on the lily at 1e-12 for as long as the lily is in the
   sweep.

Lane recommendation: (1). It keeps the lint's "membership is explicit"
rule and covers the HONE sibling with the same entry. (2) is the
general answer if a third family turns up.

Until it is ruled, `k-lint (dev-probe)` stays red on these 10 (8 GERM
+ 1 GERM meridian + 1 HONE) once its sweep pre-pass is green again.

## The question for Ev (designers weighed it, 2026-10-10)

Two designers, from one problem statement, agree on the diagnosis and on its home:
- **The root-slack rows are bounds, not coincidences.** `slack` is a non-negative upper bound on the kernel's own placement error, and the lane accepts the root iff the bound is at most ε. A three-way sign decision (Zero, in-band, Positive) answers a two-way question, so rule 2's zero arm, rule 1 (an in-band slack is logged as an escalation) and rule 3 (a Positive slack read as a feature) all misread these samples.
- **Recommended:** a kernel-side bound or certification door in `geom_core::k_stats` that records `within` or `exceeded` with the sample's kind. k-lint derives the kind from the record, keeps these rows out of rules 1–4, and reports usage, meaning the closest approach to ε per file, as a fact about the scene and ε. There is no hand-kept roster. The five root-slack sites migrate, HONE's sphere-region slack among them.
- **Separately:** the one remaining flag, `bool_circle_torus_meridian` at 1.04e-14, is a genuine coincidence whose honest rounding at a 5 m scene crosses ε/10² at 1e-12. Both designers recommend a zero-side ceiling on rule 2, cut from the coincidence zeros, the twin of M7-F1's definite-side cap, rather than re-cutting `PROXIMITY_FACTOR`.

Their full texts, A and B, are on the `[ev]` PR.

## Ev's ruling (PR 4567, 2026-10-10)

> "ah, didn't realize it failed--yeah just tallying it is fine"
>
> "i don't think i see the benefit to this / i'm not sure why it's semantically correct" (on the kernel bound door)

- k-lint's rule 2 zero-arm flags are **tallied and printed** (e.g. "rounding uses 31% of ε here (lily, 1e-12)"); they do not fail the `k-lint (dev-probe)` gate.
- No kernel bound or certification door. The root-slack decisions keep going through `decide`.
- The unit that builds this is the item's next step.

## Closed (branch `germ/klint-zero-arm-tally`, 2026-10-10)

Built as Ev ruled on PR 4567. Rule 2's zero arm is no longer a
`Reason`: `tools/k-lint`'s `zero_arm_usage` reads a zero-classified
margin above `band_zero / 10²`, `Scan::zero_arm` tallies it per
predicate with the largest `|m| / band_zero` and its row, and the CLI
prints that per file and counts it in the TOTAL line. It fails nothing.
Rule 2's definite arm and rules 1, 3 and 4 are unchanged, and no kernel
decision moved. On the real dumps at ε 1e-6, 1e-9 and 1e-12 the ten
former flags print as tallied notes, and a run over the dumps' zero
rows alone exits 2 before and 0 after (the PR body carries the
output). The full dumps still exit 2 on findings this change does not
touch: `chart_bound_outer_span`'s rule-1 poison
(`work/chart/chart-bound-outer-span-decides-a-poisoned-margin.md`),
`bool_pair_gap`'s ε-coupled margins
(`work/instr/bool-pair-gap-is-eps-coupled-and-flags-74-rows-per-eps-under-k-lint.md`,
filed by this unit), `props_quad_*` on the cutaway and the lily
(`work/quad/`) and `demo/table`'s `volume_backstop`
(`work/tally/table-union-backstop-margin-sits-under-the-k-lint-floor.md`).

The nightly `k-lint (dev-probe)` row still does not reach the lint:
the sweep's pre-pass is red on
`work/tint/probe-sweep-prepass-reds-on-assemble-one-local-battery.md`
(P1, TINT's), which this change does not touch.
