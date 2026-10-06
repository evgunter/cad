# Verify: PR #4159, the last fix pass

**Verdict: NOT VERIFIED.** The kernel change holds the central bar: no wrong torus `Touch` in about 36k poses across three ε, and MINOR-1 is fixed in every torus copy. Three points block the merge, all small to fix:

1. **B1. A new row is red at ε = 1e-12.** `torus_touch_off_faces::the_off_face_touches_build_at_every_scale` fails on every run. The nightly runs every row at every ε, so merging turns the nightly red.
2. **B2. A new row is flaky at ε = 1e-6.** `every_torus_touch_holds_against_a_sampling_oracle` fails on about 1 in 60 default-seed runs. Its fixture draws walls with a negative radius.
3. **B3. The MINOR-1 sweep claim is false.** `sphere_cylinder` holds a fourth copy of the cancelling perpendicular part, and the PR edited that arm. `Touch::at` stands up to 651ε off its carriers there. The code is the same on main, and I found no wrong body from it.

## Scope

- Frozen head `b79864e7a2de3bc3c20223c51cbed98a051c231b`. The branch has not moved past it (checked 2026-10-06).
- It merges cleanly with `origin/main` `fa1f2a631`. The review's NOTE-1 conflict is gone.
- I read the PR body through `get` only, plus the check runs of the head. I read no comments or reviews.
- Local runs used `CARGO_INCREMENTAL=0` and private target directories.

## Claim checks (the "Last fix pass" section)

| claim | result |
|---|---|
| MINOR-1 fixed: `square_to` takes `a × (v × a)`; `meridian` is the one home of `in_pi` for both arms, the scrape witness (`lone(in_pi(…))`) and the sphere arm's `w_perp`; `axis_pose` uses `square_to` | **True.** `section_cert.rs:378–389`, `:747`, `:772`, `:801`, `:823`, `:842`, `:852`, `:877`. Measurements below. |
| MINOR-1 sweep: "`section_cert.rs`: all three sites fixed" | **False (B3).** `sphere_cylinder` (`:1179–1180`, used at `:1209` for `Touch::at` and `:1220` for the witness) computes `perp = cs − (o + d·(w·d))`, which is `w − d·(d·w)`, and divides it by `e`. That is the same shape spelled another way, which a grep for `v − a·(a·v)` cannot match. The other sites outside this file I did not re-check. |
| Fuzz row promoted, ~550 touches per run | True: 2138 touches at `CAD_FUZZ_EFFORT=4`. But it is **flaky at 1e-6 (B2)**. |
| Scale rows: brick, ball and rod at ×1e-3 and ×1e3, every op, both orders | Green at 1e-9 and 1e-6. **Red at 1e-12 (B1).** |
| NOTE-2: both `!tan.zero` guards removed, `Pinch::touch` holds the check | True. An undecided pinch now reaches `torus_sphere_touch` or the elliptic sign, and `Pinch::touch` still returns R-tan (`:418`). Equivalent, as claimed. |
| NOTE-3: docs say "one loop bounding a connected region … not necessarily small" | True (`:103–106`, `:291–292`, `:413–415`). |
| Style: σ from `tan.index`, the literal gone | True (`:791`, `:915`). Index 0 is NEAR in both arms' first `signs` call, and no other `signs` result feeds σ. The review's Q7 second half (`torus_sphere_touch`'s 8 positional args under `allow(too_many_arguments)`) is neither addressed nor explained. Taste, non-blocking. |
| Q3: `assert_body` volume bound relative, `1e-9·want` | True (`extent_scan_off_face_tangency.rs:174`). |
| Q2: note at the wall touch | True (`:972–974`). |
| Mutants killed (4) | True. All red, see the mutant table. |
| Hosted CI on `b79864e7`: green | True. `gate ok`, `test`, lint and the rest succeeded; the interval oracle and cache prime were skipped. The scale row is in the slow set, and the PR's ε step runs the `ci` profile, so CI never ran it at 1e-12. B1 is what the nightly will see. |
| Local 1e-6 / 1e-12 `ci` profile green | Plausible as stated: the `ci` profile excludes the slow set. It does not cover B1. |

## The review's findings

| finding | status |
|---|---|
| MINOR-1 (plane `at` off its carriers near the top parallel) | **Fixed** in the torus arms. See the measurements. The class sweep is incomplete (B3). |
| NOTE-1 merge conflict | Resolved. The head merges cleanly with current main. |
| NOTE-2 equivalent guards | Removed. |
| NOTE-3 "small loop" docs | Fixed. |
| NOTE-4 scope | No change needed. Still true: over-face and seam poses stop at `CurvedPierceUnsupported` in the e2e probe. |
| Q1 `in_pi` twice | Fixed (`meridian`). |
| Q1 `NEAR` literal; Q7 σ by string | Fixed (`index`). |
| Q7 8 positional args | Not addressed and not explained. Non-blocking. |
| Q2, Q3, Q4/Q5 | Addressed. |

## Mutants (ε = 1e-9)

Each mutant was applied by hand to `section_cert.rs`. I ran all 56 PR rows (`section_cert_rows::`, `torus_touch_off_faces::`, `extent_scan_off_face_tangency::`) with `CAD_FUZZ_SEED=1`, then the fuzz row alone with seeds 2 and 3, then reverted. The full row names are below the table.

| # | mutant | source | rows red (seed 1) | fuzz row, seeds 2/3 |
|---|---|---|---|---|
| A | `square_to` as `v − a·(a·v)` | fix pass | `top_parallel`, `fuzz` | red/red |
| B | plane elliptic margin as `abs` | fix pass | `off_outer_half`, `fuzz`, `inner_equator` | red/red |
| C | sphere extreme dropped | fix pass, review | `fuzz`, `not_touches` | red/red |
| D | sphere elliptic as `abs` | fix pass | `fuzz`, `ball_every_class`, `inner_equator` | red/red |
| E | both elliptic margins as `abs` (a touch on the hyperbolic side) | PR rows, review | `off_outer_half`, `fuzz`, `ball_every_class`, `inner_equator` | red/red |
| F | drop the face check (`certify` clears a touch without placing it) | PR rows, review | `silent_pair`, `extent_scan…both_faces`, `torus…both_faces` | green/green |
| G | plane elliptic check dropped | PR rows | `off_outer_half`, `fuzz`, `not_touches`, `inner_equator` | red/red |
| H | sphere elliptic check dropped | PR rows | `fuzz`, `ball_every_class`, `inner_equator` | red/red |
| I | plane foot on the other side | PR rows | `top_parallel`, `centre_of_every_loop`, `fuzz`, `torus…both_faces` | red/red |
| J | sphere farthest point for the nearest | PR rows | `centre_of_every_loop`, `fuzz`, `ball_every_class`, `ball…builds`, `every_scale`, `tilted_frame` | red/red |
| K | wall touch on the far ruling (`e + rc`) | PR rows | `centre_of_every_loop`, `fuzz` | red/red |
| L | any wall pinch read as a touch (all four names) | PR rows, review | `fuzz`, `not_touches` | red/red |
| M | wall touch removed (R-tan) | review | `centre_of_every_loop`, `rod…builds`, `every_scale` | green/green |
| N | nest gap (`section_sphere_pair_apart`) dropped | PR rows, review | `pinches_and_undecided_tangencies_are_not_touches` | green/green |
| O | girdle check dropped | PR rows, review | `pinches_and_undecided_tangencies_are_not_touches` | green/green |
| P | sphere extreme against the same circle only (`o1`) | review | `fuzz`, `not_touches` | red/red |

Short names in the table:
- `top_parallel` = `a_plane_touch_near_the_top_parallel_stands_on_both_carriers`
- `fuzz` = `every_torus_touch_holds_against_a_sampling_oracle`
- `off_outer_half` = `a_plane_tangent_to_the_tube_off_its_outer_half_refuses_as_a_tangency`
- `inner_equator` = `torus_touch_off_faces::a_touch_on_the_inner_equator_refuses_off_the_faces`
- `not_touches` = `torus_tangencies_off_an_elliptic_extreme_are_not_touches`
- `ball_every_class` = `torus_and_ball_every_class`
- `silent_pair` = `a_touch_clears_only_out_of_a_face_on_a_silent_pair`
- `centre_of_every_loop` = `a_torus_touch_is_the_centre_of_every_loop_its_margin_admits`
- `*…both_faces` = `the_same_tangencies_on_both_faces_refuse` (extent scan) and `the_same_touches_on_both_faces_refuse` (torus)
- `ball…builds`, `rod…builds`, `every_scale`, `tilted_frame` are the `torus_touch_off_faces` rows of those names.

Every mutant goes red. The review's `!tan.zero`-drop mutants no longer apply: the guards are gone.

My own oracle (below) also goes red on A, C, E, I, J, K, L and P. Its counts of wrong touches were A 340+6, C 38, E 566, I 766+156, J 340, K 970, L 1992 and P 38. On E and J it also tripped the "no Touch on the hyperbolic side" assertion. So its zero on the head means something.

## Rows at three ε (56 PR rows), and nextest

| ε | result |
|---|---|
| 1e-9 | 56/56 |
| 1e-6 | 56/56 with the default seed. With other seeds the fuzz row is flaky (B2). |
| 1e-12 | 55/56. **`the_off_face_touches_build_at_every_scale` is red (B1).** |

- **nextest `-p topo -p sweep`, full suite (slow set included), ε 1e-9: 4723/4723 passed.**
- **B1 in detail.** At ×1e3 the donut operand itself fails `finished()` before any boolean runs:
  ```
  VolumeUncomputable … Indeterminate { Enclosure { lo: 0.0, hi: 1.036e-12 } … predicate: "props_rim_level" }
  ```
  ×1e-3 passes. The failing path is sweep's revolve plus `validate`/props. The diff changes no file on that path: `section_cert.rs` is reached only from `boolean/ops.rs`. So the failure is main's kernel behaviour, but the row is new, so the nightly red would be this PR's. The reviewer's e2e probe fails the same way on its ×1e3 tori at 1e-12. Fix: run the ×1e3 leg only where the fixture validates, or pin a band with `Band::linear_at`.
- **B2 in detail.** At 1e-6 the PR's fuzz row went red on seed 55433 at `CAD_FUZZ_EFFORT=4`, and on 1 of 60 default-effort seeds (seed 155). At 1e-9 it was 0 of 60. Every failure is `section_torus_offset_wall_near_outer` on a `Wall(…, rc)` with `rc < 0`: `rc = range(0.05, 0.99)·(R − r)` plus δ = −100ε at ×1e-3. A radius ≤ 0 breaks the surface convention (`geom/src/convention.rs`, `radius > 0`), so this is a fixture defect, not a kernel one. The seed is random per run unless CI pins one. Fix: skip draws with `rc + δ ≤ 0` (and `ρ + δ ≤ 0`).
- **Main's listed reds.** None of the rows I ran is among them. The 1e-9 full run includes `bounds_census` and the split-seam rows, and they pass here.

## Widening: an independent oracle, at least 1,000 poses per ε

The probe `verify_torus_touch_oracle.rs` is mounted in-crate. It uses no kernel margins. For each partner it takes the **closed-form critical points of the level function on the torus**:
- plane: `h₀ + σ·s·R + κ·r`;
- sphere: `|d_σ ∓ r|`;
- wall parallel to the axis: `|±(R ± r) − e|`, plus the two zeros where its axis pierces the tube.

It computes the part square to the axis in the torus's own `(e₁, e₂)` basis, which is not the kernel's cross-product form. A `Touch` is right only when, on one side, all of these hold:
- the extreme critical value is within 2Kε of the level;
- the next critical value lies strictly beyond it, so the region cut off is one disc;
- the extreme is at an elliptic point;
- `at` is on both carriers within Kε;
- `at` joins the extreme along a path staying below the next critical value.

The draws:
- Random ring tori with any axis, `r/R` from 0.03 to 0.98, centres up to 2·scale from the origin, at ×1e-3, ×1 and ×1e3.
- Tangent points biased to the equators and to v = ±π/2 ± {0, ½, 1, 2, 10, 100, 1e4}·ε/r.
- δ ∈ {0, ±½, ±1, ±2, ±10, ±100}·ε.
- Partners: a plane; three spheres (outside, inside the tube, about the torus); three walls (beside, about, in the hole).
- Both operand orders.

| ε | poses | classifications | Touch | R-tan (refusal) | other | **wrong** | worst `at` off a carrier, δ = 0 (×1e-3 / ×1 / ×1e3) |
|---|---|---|---|---|---|---|---|
| 1e-9 | 12000 | 24000 | 3346 (14%) | 15374 (64%) | 5280 | **0** | 3e-9 ε / 2e-6 ε / 0.003 ε |
| 1e-6 | 11964 | 23928 | 3310 (14%) | 15146 (63%) | 5472 | **0** | 3e-12 ε / 2e-9 ε / 3e-6 ε |
| 1e-12 | 12000 | 24000 | 3300 (14%) | 15018 (63%) | 5682 | **0** | 3e-6 ε / 0.002 ε / 2.05 ε |

- The touches split by arm as follows at 1e-9: wall 1952, sphere near 740, sphere far 282, plane 372. The oracle's undetermined count was 0.
- With δ ≠ 0, the worst distance is the decided margin itself: 1.000–1.001 ε.
- The 2.05 ε at ×1e3 and 1e-12 is at f64 resolution. Coordinates reach 6.9e3 there, and one ulp of that is 1.5e-12, or 1.5 ε.

**The reviewer's probes on the head:**
- `probe_torus_touch_soundness`: green at 1e-9 (2916 touches) and 1e-12 (2890). At 1e-6 it reports 462 "bad". I checked all 462:
  - 460 are the oracle's own limit. At ×1e-3 its "another minimum more than 1e-4·scale away" threshold (1e-7) is below δ = ε = 1e-6. The "other" minimum lies exactly |δ| from `at` and is the same minimum.
  - 2 are a wall with a negative radius (the B2 fixture defect).
  - None is a kernel error. My oracle reads 0 wrong at 1e-6.
- `probes/torus_touch_e2e.rs`:
  - 1e-9: 8/8 green.
  - 1e-6: 0 wrong bodies. 19 "bad", every one a typed `point_in_solid` `Err(Escalated)` at ×1e-3 (a refusal, not a wrong answer).
  - 1e-12: 0 wrong bodies. 4 rows panic on the ×1e3 operand's validation (B1's cause).

## MINOR-1, re-measured

`Touch::at`'s worst distance from either carrier, plane tangent at v = π/2 − t, t ∈ {1e-4 … 1e-8} rad:

| probe | ε | ×1e-3 | ×1 | ×1e3 |
|---|---|---|---|---|
| review's `probe_plane_touch_at_off_its_carriers_near_the_top_parallel` (3 fixed axes; ×1 and ×1e3 only) | 1e-9 | — | 0.0 ε | 0.0 ε (was 8.9 ε to 1.2e4 ε) |
| same | 1e-6 | — | 0.0 ε | 0.0 ε |
| same | 1e-12 | — | ≤ 0.8 ε | ≤ 0.8 ε |
| mine (40 random tori per scale) | 1e-9 | 0.000 ε | 0.000 ε | 0.001 ε |
| mine | 1e-6 | (no decided tilt) | 0.000 ε | 0.000 ε |
| mine | 1e-12 | 0.000 ε | 0.002 ε | 1.36 ε (coordinate ulp ≈ 1.5 ε) |

The class fix reached every torus copy: both `in_pi` uses (`:801` touch, `:823` scrape witness), the sphere arm (`:852`, `:877`) and `axis_pose` (`:747`).

**B3, the copy it did not reach.** `sphere_cylinder` takes `foot + perp/e·rc` with `perp = cs − (o + d·(w·d))`. I tested a ball inside a wall, nearly coaxial, offset e ∈ {5, 10, 100, 1e3}ε, touching the wall, with the wall's origin 0 or 1e3·scale along the axis. 60 poses per cell, both orders. Worst `at` off a carrier, against the actual f64 surfaces:

| ε | ×1, L = 0 | ×1, L = 1e3 | ×1e3, L = 0 |
|---|---|---|---|
| 1e-9 | 0.000 ε | 1.27 ε (e = 5ε) | 915 ε (e = 10ε), 6.7 ε (e = 100ε) |
| 1e-12 | 651 ε (e = 10ε), 4.9 ε (e = 100ε) | 1.5e9 ε (e = 10ε) | 6.3e7 ε (e = 1e3ε) |

- This breaks `Touch::at`'s contract the way MINOR-1 did.
- The error is mostly along the ruling. It leaves `at` near both carriers only to second order, so in the poses I built placement reads off-carrier and refuses: conservative, like MINOR-1.
- I found no wrong body. I did not build an e2e pose that clears through it: any face edge within that displacement of the touch also cuts the ball.
- The code is on main unchanged (`git show origin/main:…section_cert.rs`, `sphere_cylinder`). The PR added the girdle gate to this arm and claims the class swept, so the claim is false.
- Fix: one line, `let (perp, e) = square_to(cs - o, d);`, with the foot taken separately. Or file it.

## Top parallel: the hyperbolic and elliptic split

Planes and outside balls tangent at v = ±π/2 ∓ kε/r for k ∈ {1, 10, 100} (elliptic), or ±π/2 ± kε/r (hyperbolic). 40 tori per scale (the first is the PR's donut), both orders, 160 classifications per cell.

- **Hyperbolic side: 0 Touch in every cell, at every scale and every ε (2880 classifications per ε).**
- Elliptic side, 1ε from the parallel: 0 Touch everywhere (all refuse).
- Elliptic side, 100ε: 152–160/160 Touch.
- Elliptic side, 10ε: partly decided, 0–156/160 depending on scale and ε. For example, at 1e-9 the plane reads 24/134/128 and the ball 32/80/98 for ×1e-3/×1/×1e3.

None of the elliptic touches was wrong under the oracle.

## Verdict

**NOT VERIFIED.** The blocking points:
1. **B1.** `torus_touch_off_faces::the_off_face_touches_build_at_every_scale` is red at ε 1e-12. Its ×1e3 donut fails operand validation (`props_rim_level`). Hosted CI does not run the slow set at 1e-12, but the nightly runs every row at every ε.
2. **B2.** `every_torus_touch_holds_against_a_sampling_oracle` is flaky at ε 1e-6 (about 1/60 default seeds; reproduce with `CAD_FUZZ_SEED=155`). Its fixture draws walls with `rc + δ < 0`.
3. **B3.** The MINOR-1 sweep claim is false. `sphere_cylinder` (`section_cert.rs:1179–1209`) is a fourth copy, with `Touch::at` up to 651ε off its carriers at ×1 and 1e-12. It predates the PR, is conservative where measured, and I found no wrong body. Fix it in this PR (the arm is already edited here) or file it.

The central bar holds: no torus `Touch` cleared a pair that crosses or touches on a face or within an edge band, in about 36k poses at three ε against an independent oracle, the PR's rows, and the reviewer's e2e (0 wrong bodies). Non-blocking: the review's Q7 note on `torus_sphere_touch`'s argument list is unanswered.
