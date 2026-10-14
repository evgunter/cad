---
id: profile-fillet-radius-off-at-eps-1e-6
kind: issue
title: profile: the fuzzed offset-carrier fillet recovers its radius 2.6e-7 off at CAD_TOLERANCE_EPS=1e-6 (seed 0x063fda568e08fb0f, iter 380)
status: closed
opened: 2026-09-04
closed: 2026-10-07
refs: [1877]
priority: P0
cost: H
---


(S-CERT orchestrator) Filed from CERT-M3's gate (PR #1877, run 33925337156,
job `test (eps = 1e-6, 2/2)`), where it is a red INHERITED from main: the
PR touches nothing under `crates/profile`, and the failure reproduces
byte-for-byte on main's own tree.

## The failure

`crates/profile/tests/review_s2.rs:673` —
`review_s2::fuzz_offset_carrier_construction_tangency_and_bulge`, oracle
(a) of `check_corner`:

```
recovered radius 0.07525678837520786 vs 0.07525705177877821 — iter 380:
reproduce with CAD_FUZZ_SEED=0x063fda568e08fb0f CAD_FUZZ_EFFORT=1
```

The fillet circle re-derived from the emitted `(t1, t2, bulge)` has a
radius 2.6e-7 off the requested one, against the oracle's 1e-9. Iteration
380 of the seed's sweep, at effort 1.

## Reproduction (2026-09-04)

Replayed with the seed, one test, separate build directories per tree:

| tree | `CAD_TOLERANCE_EPS=1e-6` | default eps |
|---|---|---|
| main `b7f347254` | **FAIL**, identical message | pass |
| PR #1877 head `19a775c0e` | **FAIL**, identical message | pass |

```
CAD_FUZZ_SEED=0x063fda568e08fb0f CAD_TOLERANCE_EPS=1e-6 \
  cargo nextest run -p profile --test all \
  -E 'test(fuzz_offset_carrier_construction_tangency_and_bulge)'
```

So the defect is (i) tolerance-dependent — the construction at the
`1e-6` band emits a `(t1, t2, bulge)` whose implied radius misses by
~3.5e-6 relative — and (ii) seed-dependent, which is why the eps=1e-6 row
is green on most runs of main: the fuzz draws a fresh seed per run and
this draw is one of the ones that finds it. Not measured here: which of
the emitted three carries the error (the tangent points against their
carriers, oracle (b), or the bulge), and whether the 1e-9 oracle is the
right bar at a 1e-6 band or the construction is genuinely off. Both are
the owning program's to measure at the site.

## Debt

The row is red on main independent of #1877; the S-CERT program annotated
the PR and did not absorb the fix (crates/profile is outside its fence).

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to BLEND (the sweep crate and the profile fillet door are BLEND's charter; crates/sweep/src/loft.rs passes to BLEND at this exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## 2026-10-02 — re-checked under 5b (store-constructed-carriers): does not dissolve

The fillet arc now stores the radius as authored, so the stored radius is
the requested `0.07525705177877821` exactly. The seed still reds at 1e-6,
and the stored arc shows why: at iteration 380 the fillet is a
near-half-turn (sweep `π + 8.2e-7`), its rim at `t1` is `3.1e-8` off the
radius and its rim at `t2` is `-5.6e-7` off, and turning `t1` through the
sweep lands `(5.6e-7, -2.0e-7)` from `t2`. The construction's tangent point
`t2` sits off its own fillet circle by about ε/2; the oracle's recovered
radius (chord and stored sweep) reports that inconsistency. Measured on
`claude/clever-bardeen-4itqb3` with
`CAD_FUZZ_SEED=0x063fda568e08fb0f CAD_TOLERANCE_EPS=1e-6`.

## Outcome

Both halves were true: the construction was off, and the oracle asked for
more than any construction can give.

- **The corner has no exact fillet.** At iteration 380 the two offset
  circles (radii ρ₁ = 0.33105, ρ₂ = 0.01824, centres d = 0.31281 apart)
  miss internal tangency by 5.27e-7, inside the 1e-6 band, so
  `fillet_offset_circles_internal` decides them tangent. No centre is at
  both offset radii, so the two rims must carry at least that gap between
  them, and the old 1e-9 bar on the recovered radius could not hold at
  this ε.
- **The decided centre carried more than the gap.** It was the
  radical-line foot `(d² + ρ₁² − ρ₂²)/2d` along the link, which sits
  gap·ρ₂/d off one offset circle and gap·ρ₁/d off the other. Here that is
  3.07e-8 and 5.58e-7, which sum to 1.117x the gap. On near-equal carriers
  with a small fillet the factor (ρ₁ + ρ₂)/d is unbounded. The fuzz's
  natural draws reached 4.1x at ε = 1e-4.
- **The fix** (`sugar.rs`, `ArcCarrier::offset_circles`) puts the decided
  centre midway between the two offset circles' nearest points on the
  link. Each rim is then off by gap/2, and the two sum to the gap. Where
  the offset circles are separated, as here, no centre can undercut that.
  Where they overlap by the gap they cross at two exact centres, about
  √(gap·ρ) off the link and ill-conditioned in the gap, and the decided
  centre's gap is a bound rather than the floor. The line×circle
  branch's foot puts the whole gap on the circle's rim and is unchanged.
- **The oracle** (`review_s2.rs`, `check_corner`) now allows a corner
  whose re-derived offset gap is below kε (decided tangent, since
  [ε, kε) escalates) half that gap, plus its second-order term and
  rounding, on its chord-read checks. It adds (g): read off the stored
  arc, each arc×arc rim is at most half the gap and the two sum to at
  most the gap, plus 32 ulps of the corner's scene.
- **Rows:** `the_decided_tangent_fuzz_corner_builds_at_both_scalars_and_meets_the_oracle`
  covers the seed's corner, and
  `near_half_turn_and_extreme_sweep_fillets_meet_the_oracle_at_both_scalars`
  covers internal lenses on three carrier pairs, the external lens and
  the line-into-circle lens at margins ±0.1/0.5/0.9ε, 0 and past the band,
  plus line×line turns from 0.05 to π − 0.01. Both run at f64 and at
  Interval, and both were red first at every ε on (g). The fix pass
  added lenses on carriers of 30 and 100 to the second, and
  `drawn_decided_tangencies_meet_the_oracle_at_both_scalars`, which draws the
  decided class directly from a pinned seed, at random radii, rotation,
  reflection, scale and offset.
- The class outside the fillet is filed as
  `work/issues/decided-tangent-point-is-the-radical-foot.md`.
