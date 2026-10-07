# Verify (second pass): PR #4159 at `60209380`

**Verdict: NOT VERIFIED, on one point outside the kernel.** B1, B2 and B3 are each resolved as claimed. The central bar holds: in about 30k poses at three ε, no torus `Touch` cleared a pair that crosses past the band or cuts more than one disc. Every mutant is red.

The one blocking point is the PR's `.config/nextest.toml` (N1 below). Its last merge resolution dropped a `)`. As a result, main's slow row `a_ruling_lying_on_a_wall::a_ruling_across_an_ellipse_builds_every_op_undeclared` and the PR's 13.8 s `torus_touch_off_faces::near_misses_answer_by_the_closed_form_distance` both leave the `ci` slow set. The same line also conflicts with current main, so the PR has to be re-merged anyway, and the fix rides that merge. Nothing turns red because of it. It changes main's CI selection silently.

## Scope

- Frozen head `60209380fc626c3eeb1e1da4dc91b2321c2af1a4`. The branch had not moved when checked.
- Base for comparisons: `origin/main` `8e5dc42c5`. For the trial merge: `def02b80f`.
- I read the PR body through `get` only. I read no comments or reviews.
- Local runs used `CARGO_INCREMENTAL=0` and private target directories outside the checkout.
- Probes were scratch code in a detached worktree and were never committed. They are described below so they can be rebuilt.
- Since `b79864e7a2`, the kernel diff is:
  - the B3 line in `sphere_cylinder`;
  - `torus_sphere_touch`'s signature regrouped (Q7, behaviour unchanged).
  The test diff is:
  - the B1 condition (`torus_touch_off_faces.rs:264–296`);
  - the B2 filter (`section_cert_rows.rs:2415–2429`);
  - the new B3 row (`:2293–2354`).

## B1: a narrowed row

| check | result |
|---|---|
| The ×1e3 donut alone, `validate_geometric` at ε 1e-12, on `origin/main` (no PR code) | **Refuses identically.** `VolumeUncomputable { … Escalated { Indeterminate { Enclosure { lo: 0.0, hi: 1.036e-12 } … predicate: "props_rim_level" } } }` on face 4. At ε 1e-9 it validates. At 1e-12 the donut validates at ×1e-3, ×1, ×10, ×100 and ×300, and refuses only at ×1e3. The refusal is main's, so the claim holds. |
| Condition `ε < 64·u·2.5·l` | The threshold is 3.55e-11 at l = 1e3 and 3.55e-17 at l = 1e-3. With ε ∈ {1e-6, 1e-9, 1e-12} it selects only ×1e3 at 1e-12. The condition over-approximates: it would also select ×300 at 1e-12 (threshold 1.07e-11), where the donut validates. The row has no such leg. If one is added, `expect_err` fails loudly, so this hides nothing. |
| Nothing else dropped or weakened | **True.** The diff against `b79864e7a2` only inserts the branch: validate, assert refusal, stand down, `continue`. Every other leg runs `assert_every_op` as before. The stood-down leg is not vacuous: it asserts that every finding is a `VolumeUncomputable` from a `PropsError::Escalated`. |
| Loud stand-down | **True.** It uses `test_utils::vacuity::stood_down`, the repo's convention (`crates/test-utils/src/vacuity.rs:258`). Run with `--no-capture` at 1e-12 it prints `SKIPPED (donut ×1e3 at ε 1e-12): the donut refuses as an operand, so its touches are not built`. At 1e-9 it prints nothing. |
| The same touch on a donut that validates at 1e-12 | ×10 and ×100: brick, ball and rod build every op, both orders, against the closed form (validate ×3, closed mesh, volume to 1e-9). At ×300, the brick and rod build, but **the ball refuses** (finding M1). At 1e-9 and 1e-6, ×10, ×100 and ×300 all build. |

**M1 (minor, non-blocking; a refusal, not a wrong body). At 1e-12 the off-face ball touch refuses from ×200 up. The cause is face placement on the ball, not the section pass.** I bisected l at 1e-12 and instrumented the run:
- l = 120 and 150 build.
- From l = 200 up (also 250, 300, 400, 500, 700) the ball refuses `FallbackExtentUnsupported` (tangent), in both orders. The brick and rod build at every one of those scales.
- In every refusing case, `torus_sphere` reads the pinch `Zero` and `torus_sphere_touch` decides the touch, with margins of `extreme = l` and `elliptic = l/2` (in metres).
- `certify`'s `place(t.at)` then returns `[Some(In), None]`: there is no verdict for `at = (0, 0, 2.5l)` on the trimmed sphere face, and `at` is the ball's −z pole.

So the stand-down at ×1e3 also hides that the ball leg would refuse there even if the donut validated. That code is not on this PR's path. It is a reach limit of sphere-face placement near the pole at large scale, and it is conservative. It is worth an item on the placement owner's slate. I did not file it, because this brief allows `verify.md` only.

## B2

- **The filter is right.** It is `if rho + delta > 0.0` on the three spheres and `if rc + delta > 0.0` on the three walls. Only `radius + δ ≤ 0` is excluded. The plane draw is unchanged.
- **The kernel refuses that datum at its door.** `validate.rs:953–958`, tier 3: "a cylinder or sphere radius … that is not definitely positive", via `geom::Surface::representability_margins`.
- **200 runs at each ε with `CAD_FUZZ_SEED` unset** (one fresh seed per process; the binary was run directly, 4 in parallel):

  | ε | failures |
  |---|---|
  | 1e-6 | **0/200** |
  | 1e-9 | **0/200** |
  | 1e-12 | **0/200** |

  Each ε also had three sanity runs reporting `1 passed`.
- The first verifier's reproducer, `CAD_FUZZ_SEED=155` at 1e-6, passes on the head.

## B3

**`Touch::at` on the `sphere_cylinder` arm.** I re-measured with my own probe, not the PR's row:
- A ball of radius `rc − e` sits inside a wall, offset `e` ∈ {5, 10, 100, 1e3}ε from its axis, along a random axis.
- `rc` ∈ [0.5, 2]·scale.
- The wall's origin is at the ball, or 1e3, or 1e3·scale along the axis.
- 40 poses per cell, both orders.
- Distances are taken against the stored f64 surfaces in double-double arithmetic, so the measurement is not limited by f64 resolution.

Worst distance off either carrier:

| ε | ×1e-3 | ×1 | ×1e3 (origin ≤ 1e3) | ×1e3, origin 1e6 along |
|---|---|---|---|---|
| 1e-9 | ≤ 0.0002 ε | ≤ 0.0002 ε | ≤ 0.0005 ε | ≤ 0.12 ε (≤ 0.54 coordinate ulps) |
| 1e-6 | 0.0000 ε | 0.0000 ε | 0.0000 ε | 0.0001 ε |
| 1e-12 | (≤ 1 ulp) | ≤ 0.15 ε | ≤ 0.40 ε | 16.3 ε, but only 0.07 coordinate ulps: f64 cannot do better at 1e6 |

Every cell is within one coordinate ulp. The first verifier's 651ε (×1, 1e-12) is gone.

**With the old spelling** (`perp = cs − foot; e = perp.norm()`) the same probe reads:
- 1e-9: 447ε at ×1e3, origin 1e3; 2.1e9ε at origin 1e6.
- 1e-12: 4.1ε to 13ε at ×1, origin 0; up to 3e9ε at ×1, origin 1e3; 2e10ε at ×1e3.

**The new row `a_ball_touching_a_wall_near_its_axis_stands_on_both_carriers`:**
- With the old spelling it is **red** at 1e-9 (×1e3, axis 2, origin 1e3, e = 100ε: 1.155ε off the wall against a bound of ε + 8 ulps = 1.005ε) and at 1e-12 (×1e-3, origin 1e3, e = 10ε: 4479.7ε). It is green at 1e-6, as the PR says.
- **The tolerance (ε + 8 ulps of `far + 2·scale`) is tight enough.**
  - It adds 0.005ε at ×1e3 and 1e-9, 1.78ε at ×1e-3 and 1e-12 with origin 1e3, and less elsewhere.
  - The head stands at most 0.4ε off. The mutant exceeds the bound by 1.15× at its narrowest cell and by 2500× at 1e-12.
  - An `at` off its carriers by more than ε plus a few coordinate ulps cannot pass.
- The row allows only an ε of real error. That is the margin a `Touch` is read under anyway.

**The class sweep:**
- **The four filed sites** are real copies of the shape (`work/issues/perpendicular-part-over-a-small-norm-outside-section-cert.md`):
  - `join.rs:1579–1580` `parallel_radical_plane`: `w = delta − a1·(delta·a1)` into `UnitVec3::new`;
  - `join.rs:2049–2051` `cs_transverse_frame`: `off = center − foot`, `off/d`;
  - `offset_axial.rs:1162–1164` `solve_corner`: `radial_old`, `rho_old`;
  - `props/curved.rs:2744–2754` `side_on_meridian`: `radial/|radial|`.

  Each divides a perpendicular part by its own norm, decided only above the band.
- **Three "guarded" sites, spot-checked, hold:**
  - `sphere_region.rs:215–219` `tangent` re-projects after normalizing.
  - `offset_axial.rs:1576–1584` divides by `n0·n1`. Those are unit meridian normals whose axial part was decided `Zero` upstream, so the norms are ≈ 1, as its comment says.
  - `recl.rs:907` normalizes a sector representative whose norm is not small for a non-degenerate sector.
- The "Correction" paragraph owns the earlier false claim.

## Mutants A–P (ε 1e-9; all 57 PR rows, seed 1, `--no-fail-fast`; then the fuzz row alone, seeds 2 and 3)

| # | rows red, seed 1 | fuzz, seeds 2/3 | change from the first table |
|---|---|---|---|
| A `square_to` as `v − a·(a·v)` | `ball_touching_a_wall` (new), `top_parallel`, `fuzz` | red/red | the new B3 row also kills it |
| B plane elliptic `abs` | `off_outer_half`, `inner_equator`, `fuzz` | red/red | same |
| C sphere extreme dropped | `fuzz`, `not_touches` | red/red | same |
| D sphere elliptic `abs` | `inner_equator`, `fuzz`, `ball_every_class` | red/red | same |
| E both elliptic `abs` | `off_outer_half`, `inner_equator`, `fuzz`, `ball_every_class` | red/red | same |
| F face check dropped | `silent_pair`, both `…both_faces` | **green/green** | still survives the fuzz |
| G plane elliptic dropped | `off_outer_half`, `inner_equator`, `fuzz`, `not_touches` | red/red | same |
| H sphere elliptic dropped | `inner_equator`, `fuzz`, `ball_every_class` | red/red | same |
| I plane foot other side | `top_parallel`, `centre_of_every_loop`, `fuzz`, `torus…both_faces` | red/red | same |
| J farthest for nearest | `ball…builds`, `centre_of_every_loop`, `fuzz`, `every_scale`, `tilted_frame`, `ball_every_class` | red/red | same |
| K wall far ruling | `centre_of_every_loop`, `fuzz` | red/red | same |
| L any wall pinch a touch | `fuzz`, `not_touches` | red/red | same |
| M wall touch removed | `rod…builds`, `centre_of_every_loop`, `every_scale` | **green/green** | still survives the fuzz |
| N nest gap dropped | `pinches_and_undecided…` | **green/green** | still survives the fuzz |
| O girdle dropped | `pinches_and_undecided…` | **green/green** | still survives the fuzz |
| P sphere extreme, `o1` only | `fuzz`, `not_touches` | red/red | same |

The short names are the first verifier's.
- **F, M, N and O still survive the fuzz row.**
  - F: the fuzz row tests classification, not placement.
  - M is a refusal: it loses reach and can give no wrong answer.
  - N and O are on the sphere arms, which the torus fuzz does not draw.
- None of the four hides a wrong answer: a unit row kills each one.

## Central bar: an independent closed-form oracle, about 10k poses per ε

The probe is my own code, mounted in-crate. It shares no helper with the PR's rows. For each partner it takes the closed-form critical values of the partner's level function on the torus, computed in double-double from the stored f64 surfaces:
- **plane:** `σ·s·R + κ·r`, elliptic iff `σ = κ`;
- **sphere:** `|d_σ − r|` and `d_σ + r` at `C_σ ± r·u_σ`, elliptic iff the point's radial coordinate exceeds `R`;
- **parallel wall:** `|±(R ± r) − e|`, elliptic on `R + r`, plus 0 where its axis pierces the tube.

A `Touch` is **wrong** when any of these holds:
- the critical value nearest the level is not the global min or max;
- the partner crosses past it by more than `2ε + 8` coordinate ulps;
- the gap to the next critical value is not strictly larger than the crossing depth;
- the extreme is not elliptic;
- `at` is off either carrier by more than `1.01ε + 16 ulps`.

The draws:
- Random ring tori with any axis, `r/R` from 0.03 to 0.98, centres within 2·scale, at ×1e-3, ×1 and ×1e3.
- `v` biased to both equators and to `±π/2 + kε/r`, k ∈ {0, ±½, ±1, ±2, ±10, ±100, ±1e4}.
- δ ∈ {0, ±½, ±1, ±2, ±10, ±100}ε.
- Partners: a plane, three spheres (outside, inside the tube, about the torus) and three walls (beside, about, in the hole).
- Both orders. 3400 tori per scale.

| ε | poses | classifications | Touch | R-tan | other | **wrong** | worst `at` off a carrier at δ = 0 (×1e-3 / ×1 / ×1e3) |
|---|---|---|---|---|---|---|---|
| 1e-9 | 10200 | 142800 | 18338 | 86948 | 37514 | **0** | 4e-9 ε / 4e-6 ε / 0.003 ε |
| 1e-6 | 10200 | 142514 | 17692 | 85456 | 39366 | **0** | 4e-12 ε / 5e-9 ε / 5e-6 ε |
| 1e-12 | 10200 | 142800 | 17806 | 85138 | 39856 | **4 (M2)** | 4e-6 ε / 0.004 ε / 1.70 ε (≤ 1 coordinate ulp) |

- By arm at 1e-9: wall 7416, sphere near 5942, sphere far 1994, plane 2986.
- Touches where the level stood more than 2ε from every critical value: 0 at 1e-9 and 1e-6, and 26 at 1e-12. Those are all apart, so harmless.
- A first cut of the oracle flagged 50 near-top plane touches at ×1e3 and 1e-12. That was the oracle's own error: it charged 64 coordinate ulps (about 270ε) against the gap. Their exact gaps are 22–60ε against depths ≤ 1.14ε, so they are sound.
- I did not run my oracle against mutants. Its plane and sphere checks are the conditions the mutants break, and the PR's rows kill all sixteen.

**M2 (minor, non-blocking; no wrong body). At ×1e3 and 1e-12, the sphere arm's elliptic margin is decided on f64 rounding levered by `r/d`.**
- Two poses (four classifications) read `section_torus_sphere_near_tube` `Touch` for a ball **inside the tube**, posed 10ε of arc onto the hyperbolic side of the bottom parallel, with δ = −0.5ε.
- The kernel's elliptic margin `σx − R = r·(s − R)/d` reads **+14.3ε and +11.8ε, decided `Positive`**.
- The exact margins on the same stored surfaces are −14.4ε and −1.05ε.
- The cause:
  - `meridian`'s `s` (≈ 1780 or 2903) is 2 ulps off the exact value: 4.5e-13 against 2.3e-13 per ulp.
  - The sphere's centre is `d` ≈ 18 to 53 from the tube centre against `r` ≈ 1127 to 1389, so the lever `r/d` is up to 63.
  - The decision takes the f64 margin at face value.
- Why it is not a wrong answer:
  - The touch's soundness conditions hold exactly: a strict global minimum of `f`, a decided gap and depth ≤ 2ε. Along the parallel, the nearest point is always a minimum of `f` whatever its curvature.
  - The pair is apart (δ < 0).
  - The broken condition is the brief's extra rule, "elliptic", and it is broken only at a near-parabolic point within rounding.
- It is still a margin decided without its rounding. It belongs with the PR's filed lever items, or as a `Margin::levered` on `r/d`. I would not block on it.

## Merge with current main (`def02b80f`)

- **The trial merge conflicts** in `.config/nextest.toml`, on the `sweep::all` slow-set line. Main has since added `reach_split_gate_azimuth::every_cut_clear_of_a_partial_turn_sphere_face_splits`.
- **N1 (blocking, one character).** The head's line reads:

  `test(=a_ruling_lying_on_a_wall::a_ruling_across_an_ellipse_builds_every_op_undeclared | test(=torus_touch_off_faces::near_misses_answer_by_the_closed_form_distance) | …`

  The `)` after `…_undeclared` is gone, so `=…_undeclared | test(=…near_misses…` parses as one literal name.
  - `cargo nextest list -p sweep --profile ci` on the head lists both `a_ruling_lying_on_a_wall::a_ruling_across_an_ellipse_builds_every_op_undeclared` and `torus_touch_off_faces::near_misses_answer_by_the_closed_form_distance` (13.8 s) in the `ci` set.
  - On main the first is in the slow set, and the PR says the second is.
  - The PR body's "six fast torus rows stay in the `ci` set" is this defect read as intent: the file has five fast rows and three slow ones.
  - The fix: restore the `)` in the re-merge the conflict forces anyway.
- **On my scratch resolution** (main's line plus the three torus entries, correctly closed):
  - `cargo nextest list --workspace --profile ci` is clean. It lists the five fast torus rows and none of the three slow ones, nor either of main's two entries.
  - The 57 PR rows with `--run-ignored all` pass **57/57 at 1e-9, 1e-6 and 1e-12**.

## Suites on the head

| run | result |
|---|---|
| nextest `-p geom-core -p topo -p sweep`, default profile (slow set included), ε 1e-9 | **5667/5667 passed** |
| The 57 PR rows, `--run-ignored all`, ε 1e-6 | **57/57** |
| The 57 PR rows, `--run-ignored all`, ε 1e-12 | **57/57** (B1 stands down loudly) |
| The 57 PR rows, ε 1e-9 (mutant baseline) | **57/57** |

No red appeared, so there was nothing to check against main.

## Verdict

**NOT VERIFIED.** The blocking point:

1. **N1.** `.config/nextest.toml` on the head drops a `)`. That takes main's slow row `a_ruling_across_an_ellipse_builds_every_op_undeclared` and the PR's 13.8 s `near_misses_answer_by_the_closed_form_distance` out of the `ci` slow set. The same line conflicts with current main (`def02b80f`). Re-merge main, restore the `)`, and confirm with `cargo nextest list --profile ci`.

All else holds:
- B1: main refuses the ×1e3 donut identically; the condition selects only that leg; the stand-down is loud and asserted.
- B2: 0/600 fresh seeds failed.
- B3: `at` stands within one coordinate ulp of both carriers, the new row kills the old spelling, and the sweep checks out.
- All 16 mutants are red.
- The central bar: about 30k poses with 0 wrong bodies.

Non-blocking findings, for the orchestrator to file:
- **M1:** the off-face ball refuses at 1e-12 from ×200 up, on sphere-face placement at its pole.
- **M2:** the sphere arm's elliptic margin is decided on rounding levered by `r/d` at ×1e3 and 1e-12.
