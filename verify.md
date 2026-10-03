# Verify: PR #3973 (reach/ellipse-torus-roots) at f6f69915e4

The branch head equals the frozen head, f6f69915e44dbca2733e1bfbb0572100af648b10. The review read is the dual review on `analysis/reach-dual/3973-r1` and `-r2`, frozen at 9f8ab75a24. Everything ran locally in its own worktrees, with separate target dirs for head and for main (d4ac5bcef, the merge base). Each mutant is the smallest textual edit, run against the topo and geom-brep rows matching `ellipse_roots|circle_roots|circle_torus|conic_torus|torus_residual|carrier_ceiling|torus_harmonics` (46 rows) and then reverted. `git status` was clean after every mutant.

## Mutants

| # | mutant (edit) | rows red | claim |
|---|---|---|---|
| A | running error dropped: `(meter.residual)(root).magnitude()` → `.value.abs()` | `the_slack_meter_charges_every_term` | holds |
| B | `speed_hi` dropped: `speed_hi * reach / lever` → `reach / lever` | `the_slack_meter_charges_every_term` | holds |
| C | ceiling → floor: meter `f_per_metre_hi: h.f_per_metre_hi.min(h.f_per_metre_surface)` → `h.f_per_metre_lo` | `the_slack_meter_charges_every_term` | holds |
| D | sin/cos ulp uncharged (`conic_torus_residual`): `error: ulp * x.abs()` → `T::zero() * x.abs()` | `the_slack_meter_charges_every_term` | holds |
| E | meter 1000× lenient: `Margin::of(arc)` → `Margin::of(arc * 1e-3)` | `the_slack_meter_charges_every_term`, `a_root_the_band_cannot_place_is_not_certified` | holds |
| F | meter dropped: `Some(TORUS_ROOT_SLACK)` → `None` | `the_slack_meter_charges_every_term`, `a_root_the_band_cannot_place_is_not_certified` | holds |
| G | `M₄` read at degree 2: `harmonics().fold` → `harmonics().take(2).fold` | `a_fourth_harmonic_graze_is_not_read_clear` only (slack row green) | holds as the PR body states it (see note 1) |
| H | old carrier reach: `(w.norm() + gram.sqrt() + R) * pad` → `w.norm() + conic.speed_hi() + R` | `the_carrier_ceiling_charges_the_frame_defect` only (slack row green) | holds as the PR body states it (see note 1) |
| I | the door returns `Miss` where the meter refuses | `a_root_the_band_cannot_place_is_not_certified` ("case 0: Miss for a carrier crossing four times"), slack row, fuzz row | holds |
| J | fuzz: drop one certified root (`count - 1` in `torus_roots`) | `certified_torus_answers_hold_against_the_true_distance`, `torus_crossings_match_the_true_distance` | holds |
| K | fuzz: shift the first root by 5 bands of arc | fuzz row (it trips the distance assert first), `torus_crossings_match_the_true_distance` | holds |
| K′ | as K, with the fuzz row's distance assert removed, at 5 and at 1.5 bands | fuzz row, at the arc-place assert (`:1471`) | the placement check kills on its own |
| L | `conic_torus_residual` reordered: `c + u·ac + v·bs − tc` → `c − tc + u·ac + v·bs` | `the_running_torus_residual_is_the_plain_chain` | holds |

## ε runs

| ε | PR rows (topo/geom-brep, 49) | sweep `ellipse_torus` (2) |
|---|---|---|
| 1e-9 | green | green |
| 1e-6 | green | green (the gap filter leaves only 1e-4 m) |
| 1e-12 | green | green |

The full nextest suites of the changed crates (geom-core, geom-brep, topo, sweep) at ε 1e-9, slow set included: **6058/6058 passed**. No red to check against main.

## Claim checks

1. **Slack row: five grazes, each slack-term mutant red, plus three more.** Holds. A–F turn `the_slack_meter_charges_every_term` red. G and H turn red only the rows the PR body names for them, not the slack row; the brief's "also red" reads as if they did (note 1).
2. **`f_per_metre_hi` from the frame as stored.** Holds. The reach is the root of the Gram matrix's larger eigenvalue, padded 64u. Reverting to the old bound (H) turns `the_carrier_ceiling_charges_the_frame_defect` red.
3. **Pinned-graze row fails on `Miss`, and the meter decides its first pose.** Holds. I turns it red with exactly that message, and F/E turn it red through the bare-walk check.
4. **Fuzz row demands the exact count and root place.** Holds. `assert_eq!(count, truth.len())` kills J. The arc-place assert alone kills K′ at 1.5 bands. Roots past the f64 oracle's quarter-band resolution are counted and skipped, as the body says.
5. **`conic_torus_residual` bits equal `implicit_residual`'s, pinned by a row.** Holds (L).
6. **1e-12 crossing row; 1e-6 sweep gaps ≥ 100ε; green at all three ε.** Holds as stated. At ε 1e-12, though, the crossing row demands **no** certificate: a door that always answers `Uncertain` leaves it green at 1e-12, and red at 1e-9 and 1e-6. Every pose's `100·noise/slope` (≈ 4.6e-10 m) exceeds the band. So at 1e-12 the row checks safety only, not liveness (note 2).
7. **Circle × torus keeps main's reading, one verdict moves, item filed.** Holds. r1's differential (`r1_circle_diff.rs`, 3,600 poses, main d4ac5bcef against head) moves **1** torus verdict (ε 1e-12, ×1e3, fam 2 i=71: `Certified` → `Uncertain`). 133 `Escalated` margins differ in their low digits only, and circle × sphere is byte-identical. The door passes `f_per_metre_hi: h.f_per_metre_lo` and `residual_reach: None`. `work/hone/circle-torus-clear-margin-reads-the-floor.md` exists.
8. **"No body builds" withdrawn.** Holds. The body now says "No body is newly built" and "the claim 'a body builds' is withdrawn".

## Notes (non-blocking)

1. The brief's wording lists G and H as "also red" on the slack row. They are red on their own rows (`a_fourth_harmonic_graze_is_not_read_clear`, `the_carrier_ceiling_charges_the_frame_defect`), which is what the PR body's table says.
2. At ε 1e-12 the ellipse × torus door's liveness is pinned by no row. The crossing row accepts `Uncertain` on every pose there, and the fuzz and slack rows accept or require `Uncertain`.
3. r1's style point (*sure*) is unaddressed: the fuzz `pose` still swaps the semi-axes half the time (`ellipse_roots.rs:1282`). It builds `Curve3::Ellipse` literals with minor > major, which `Curve3::ellipse` rejects as `AxesSwapped`. No lane claim covers it.

## Verdict

**VERIFIED.** Every lane claim holds. Notes 1–3 are wording or coverage, not blocking.
