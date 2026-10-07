---
id: lily-walls-curved-clearance-crowds-the-band-under-k-lint
kind: issue
title: k-lint (dev-probe) flags 40 bool_circle_curved_clearance margins on the lily_walls demo, unseen while the probe sweep was red
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## What

Nightly's `k-lint (dev-probe)` row lints the CSVs `k_probe_sweep.sh`
writes. The sweep has been red since #3715 (its plain loop reds on a
probe suite before the dumps), so the lint step has not run on main
since then. On `reach/dev-probe-red`, which greens the sweep, the lint
reads (local run, every ε row, 2026-10-02):

```
k-lint: GATE FAILED — the margin distribution changed: 75 margin(s) crowd a decision
     40 bool_circle_curved_clearance   (all demo/lily_walls; 12 / 14 / 14 at ε 1e-6 / 1e-9 / 1e-12)
     27 chart_bound_outer_span         (filed: work/chart/chart-bound-outer-span-decides-a-poisoned-margin.md)
      8 bool_circle_torus_root_slack   (filed: work/germ/circle-torus-root-slack-crowds-the-zero-band-at-1e-12.md)
```

The 40 `bool_circle_curved_clearance` margins (decided at
`crates/topo/src/boolean/reduce.rs`, the circle × curved-face
clearance) draw 46 FLAG lines, since a margin can break two rules at
once: 8 in the ambiguity band (indeterminate), 26 definite below the
baseline floor (4e-5), 6 definite within 10² of the escalation
threshold, and 6 zero-classified within 10² of the coincidence
threshold. At ε 1e-6 the lily wall's margins run from
6.5e-8 to 3.0e-6. That branch changes no margin under this name, and
its geometry is bit-identical to main's: `UnitVec3::levered` normalizes
exactly as `UnitVec3::new` did. So the flags are main's.

## Owed

Per the K-REPORT runbook: measure which lily-wall pairs these are, and
decide whether the baseline wants re-deriving or the clearance is
deciding a margin it cannot carry. Do not change geometry to get under
the threshold.

## Built (branch cleave/lily-clearance)

**The flags are gone from main, and the kernel change that retired them
is already there: #3817.** No kernel, geometry, scene or threshold
change here, and no baseline to re-derive.

**Main today.** A full `scripts/k_probe_sweep.sh` at `origin/main`
`1f27ea0881` (2026-10-07), linted by `tools/k-lint` over all three ε
rows: **0 flags under `bool_conic_curved_clearance`** (#3805 renamed
`bool_circle_curved_clearance` to it). `demo/lily_walls` records 21
clearance rows per ε, bit-identical across ε: 14 definite positive
(7.4632e-3 ×4, 8.6100e-3 ×4, 3.1667e-1 ×6) and 7 definite negative
(−8.8565e-3 ×2, −2.6370e-2 ×2, −3.1673e-2 ×2, −7.9892e-2). The smallest
|m| is 186× the 4e-5 metre floor. The 2026-10-06 nightly (run
37460629022, head `d9bdfaf9`) reads the same: 0 flags under either name.

**What the 40 were.** This row was filed at 16:47 UTC on 2026-10-02, from
a sweep on `reach/dev-probe-red`, whose base had neither #3805 (hence
the old name) nor #3817, which merged at 19:54 UTC that day. #3817's
row
(`work/topo/circle-clearance-records-its-charge-on-an-arc-ending-on-the-carrier.md`)
measured them: lily wall 7, the lantern minus a ball of r 0.16 at
`(−2.80, 0, 0.90)`. Each one is a meridian fragment of one sphere, split
at the section, tested against the other operand's sphere:

- the lantern zone's meridians (centre `(−2.367, 0, 0.794)`, r 0.44)
  against the ball;
- the ball's meridians against the lantern zone.

Each fragment ends at a cut vertex on the sphere it is tested against.
So its true clearance is at most 0, and exactly 0 at that end. The 46
FLAG lines that row counted (12 + 18 + 16) are this row's 46.

**An independent closed form says the margins were the enclosure, not
the geometry.** A circle's residual against a sphere is a first
harmonic, `F(θ) = c₀ + A₁cos(θ − φ)`, with `A₁ = r·d/R`, where `d` is the
in-plane offset between the centres. That puts `|F″| ≤ A₁`, and the
sampled enclosure's chord-dip charge
(`geom_brep::conic_arc_residual_range`, `ARC_RESIDUAL_SAMPLES` = 256) is
`A₁·(Δt/256)²/8`. Here `d` = |(0.433, 0, −0.106)| = 0.4458, which gives
A₁ = 1.2259 for the zone meridian against the ball and 0.16210 for the
ball meridian against the zone.

| fragment | Δt (rad) | A₁(Δt/256)²/8 | recorded |
|---|--:|--:|--:|
| ball vs zone | 3.120 | 3.0098e-6 | −3.0099e-6 |
| ball vs zone | 2.685 | 2.2290e-6 | −2.2287e-6 |
| zone vs ball | 1.044 | 2.5485e-6 | −2.5497e-6 |
| zone vs ball | 0.726 | 1.2324e-6 | −1.2325e-6 |
| zone vs ball | 0.329 | 2.5309e-7 | −2.5352e-7 |
| ball vs zone | 0.457 | 6.4574e-8 | −6.4602e-8 |

Every recorded margin is `−charge`, within the three-decimal rounding
of the centres (≤ 0.2%). The 0.0218 rad fragment (closed form
1.4694e-10, recorded −1.4725e-10) is where the sample rounding term
starts to show. So the clearance was being asked a question it could
not answer `Positive`, and it recorded the enclosure's own width as a
micrometre feature. That is verdict (ii), a wrong quantity, and #3817
is its fix: `curved_face_arm` now decides the endpoint sides of an
uncovered conic arc first, and never asks the clearance of an arc with
an end on the carrier.

**The pin bites on main today.**
`crates/sweep/tests/tilted_sphere_pair_k_rows.rs` runs in the sweep's
default selection, rostered in `probe-suite-census.sh`, and passes.
With `end_on_carrier` forced `false` in `reduce.rs`, it fails at ε 1e-9:
`union r 1 at (1.4, 0, 0): a clearance recorded -1.6054874429298314e-6,
under the metre floor`.

**Other names the same lint flags on main** (not this row's):
- `chart_bound_outer_span`, `bool_circle_torus_root_slack`,
  `volume_backstop` (demo/table), `props_quad_last_round` and
  projectbox's `props_quad_converged` already have rows.
- **Filed**: `work/hone/sphere-region-root-slack-crowds-the-zero-band-at-1e-12.md`.
  One `bool_sphere_region_roots_slack` sample on lily_walls,
  1.5093e-14 at every ε, flagged by rule 2 at 1e-12.
- **Evidence added** to
  `work/quad/projectbox-cutaway-convergence-margins-crowd-the-band-under-k-lint.md`:
  two `demo/lily` `props_quad_converged` rows at 1e-9.
