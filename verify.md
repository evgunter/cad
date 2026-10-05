# Verify: PR #4042 at 10629e13a4d

Verifier lane for REACH, brief `briefs/verify-4042.md`. Head verified: `10629e13a4d9c2f5cea8a25d871a32de1bb04a6d`, which is the frozen head; the branch had not moved at the end of the run. It merges `origin/main` at `515e79ebf`. The review read is `analysis/reach-review/4042` (`6cccaf0a`, review of `395cdbef14`), and its probes were used unchanged. The fix pass read is `f7551951`, `ac6009f9`, `10629e13` and the main merge `fb6d1e1f`.

The ε switch is `CAD_TOLERANCE_EPS`, read at runtime. Runs use nextest with the dev profile and my own target dirs. The probes ran as release `#[cfg(test)]` modules in a separate tree.

**Verdict: VERIFIED.** No claim is false on its central bar. Two non-blocking notes follow: an overstated rounding figure (R1) and one assert left unshared (S1).

## Mutants (apply → full `-p topo -p sweep`, 4,412 rows → revert)

| mutant | rows red at 1e-9 | 1e-6 | 1e-12 |
|---|---|---|---|
| none (clean head) | — | `pocket_ring_steep_ellipse` (main's, below) | — |
| `drop_a2_circle_mutant.sh` (circle arm stops charging A₂) | **`circle_wall_rows::a_near_square_circle_inside_the_band_by_its_second_harmonic_is_not_on_the_wall` only** | the same row only (+ main's pocket row) | the same row only |
| `drop_a2_mutant.sh` (all carriers) | the new row + `ellipse_rows::the_first_harmonic_arm_places_its_roots_along_the_arc` | the same 2 (+ pocket) | the same 2 |
| `tilt_mutant.py` (switch back to tilt) | `circle_wall_rows::the_second_harmonic_band_reaches_past_the_tilt_band`, `ellipse_rows::a_section_whose_projection_is_a_circle_is_a_first_harmonic`, + `ellipse_rows::{a_graze_is_read_by_its_depth, a_clear_carrier_is_a_miss, a_wall_s_own_tilted_section_is_on_it, crossings_match_the_true_distance}` | the same 6 (+ pocket) | the same 6 |

Under the circle mutant, the new row's failure says `OnSurface` at 1.400000004814217e-9 / 1.4000009800341218e-6 / 1.3999912340523224e-12 m off the wall (r = 1 m). That matches the PR body's figures. Every mutant was reverted, and the tree was clean afterwards. The tilt mutant's 1e-12 suite was re-run after a container restart, with the mutant still applied.

## ε results (clean head)

| run | 1e-9 | 1e-6 | 1e-12 |
|---|---|---|---|
| nextest `-p topo -p sweep` | 4412/4412 | 4411/4412: `sweep pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound_or_refuse_typed` | 4412/4412 |
| that red row on `origin/main` 515e79eb | — | **red there too**, with a byte-identical message ("11 runs miss…", same poses and values) apart from the thread id. Main's, not this PR's | — |
| review `door_fuzz` (`review_door_fuzz`), 1,200 poses | **WRONG 0**, SLACK 16, AMBIG 0 | **WRONG 0**, SLACK 0, AMBIG 0 | **WRONG 0**, SLACK 64, AMBIG 0 |

The new and changed rows (`conic_quadric::*`, `ellipse_torus::*`, `tang_circle_cylinder`) are inside those suites and green at all three ε.

The fuzz used the probe's default oracle density of 1e5 samples per turn at 1e-9 and 1e-6. At 1e-12 it used 2e4 (`PROBE_S=20000`, the review's stated density), because 1e5 there would not finish in the session; an aborted 1e5 run covered poses 0–338 with 0 WRONG.

Every SLACK pose is a ladder pose: the along-arc error is NOTE-2's known class, pre-existing and filed. The least A₂ among them is 1.0ε (1e-9; it is the review's pose 537) and 1.01ε (1e-12). None is an arm pose.

## Claim checks

- **MINOR-1: holds.** The row exists and asserts that A₂ is in the zero band, that the oracle reads the depth past the zero band and the top within it, and that the door answers not `OnSurface`, at r = 1 m and 100 m. It is the only red row under the circle mutant at all three ε (table above). `tilt_mutant.py` reddens both earlier new rows, and `drop_a2_mutant.sh` reddens the new row plus the ellipse arm row, at all three ε.
- **Rounding: the soundness conclusion holds; the quoted maximum is overstated (R1).** Method: a probe module dumps the kernel's own `f64` `conic_{cylinder,sphere}_harmonics` together with the bit-exact inputs. Python then recomputes the same formulas in `Fraction` (√ at 80 digits). The reading error is `|δc₀|+|δA₁|+|δA₂|`, with `A₁`, `A₂` re-formed as the door forms them, divided by `rounding_charge(terms)/2r`. The poses are circles and ellipses against walls and spheres, r and ρ from 1 mm to 1 km, centres up to 1 km out, with half of them near the surface (offset 1e-12 to 1e-2·r, tilt 1e-12 to 1e-2).
  - 1,200 conics: max **0.1915**; p99 0.134. The two poses over 0.19 are circle × sphere, which never reaches the arm (it returns early through the factored harmonic).
  - 20,000 conics: max **0.316** (a generic, far wall × circle pose), wall ellipse 0.245, sphere ellipse 0.196. The near-surface, arm-reachable classes reach at most **0.155**. Using the exact `axis × u_ref` instead of the rounded `v_ref` gives 0.317.
  - **No pose exceeds the charge** (ratio > 1: 0 of 21,200), and the per-coefficient sum `Σ|δ|` stays ≤ 0.344 of it. So the one charge covers the arm's readings. But "at most 0.19 … about 5× room" is a property of the PR's sample, not a bound: the measured room is about 3×.
- **Style: holds, with one remnant (S1).**
  - `ellipse()`, `oracle()` and `assert_matches_oracle` live once in `conic_oracle`. `ellipse_rows` and `ellipse_torus` keep thin wrappers that delegate.
  - `crossings` refines every sampled `|f|` extremum and bisects both sides. I read it adversarially and found no defect, beyond a pair hidden inside the first or last sample cell (no extremum test at `k = 0`), which no row relies on.
  - `ellipse_torus_roots` has one `ClassificationInvariant`: the kind, `Conic::of` and the surface are matched in one `let … else`.
  - `grep -i "square arm"` finds nothing in `tang_circle_cylinder.rs` or anywhere under `crates/`.
  - S1: `circle_wall_rows.rs:102` still carries its own `assert_matches_oracle` body, Pose-typed and a near copy of the shared one. Only its sampled oracle moved to `conic_oracle::crossings`. The review's finding named the oracles and the `ellipse_torus` copy, which are fixed, so this is a remnant rather than a reopened finding.
- **No regression: holds.**
  - `door_fuzz` gave 0 wrong at all three ε (table above).
  - `door_log.py` on head and on `origin/main` 515e79eb over `-p topo -p sweep` at 1e-9 logged **6,479 calls on each tree**. The logs are **byte-identical** after sorting, once two predicate names are mapped: `bool_circle_cylinder_disc` → `bool_conic_quadric_disc` and `bool_circle_sphere_extreme` → `bool_conic_quadric_first_extreme`, both per the PR's rename table. Answers: 6,124 Certified, 255 OnSurface, 88 Uncertain, 10 Miss, 2 Escalated.
  - Instrumented suites: head 4412/4412, main 4410/4410.
- **Suites: holds.** Green at 1e-9 and 1e-12. At 1e-6 the only red row is `pocket_ring_steep_ellipse`, which is red identically on main.

## Not checked

I did not run the tour, CI on GitHub, or the band-edge sweep `review_arm_edge`.
