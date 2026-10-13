# SYM-17 Phase 2: review R2 (PR #4197 at `a89f7a54eb`)

**Verdict: APPROVE-WITH-FIXES.** The core is sound and well guarded. One MAJOR blocks the merge: the PR's own CI step `doc-tests` is red, a mechanical rename fixes it, and neither the PR body nor the brief mentions it.

## Findings
- **MAJOR-1: the guide doctests are red (8 of them). `sure`, demonstrated by execution** (hosted run 38323491246, job 115056960854, step `doc-tests`: "error[E0433]: cannot find type `OrthoFrame`").
  - `pncad::prelude` drops `OrthoFrame` (`crates/pncad/src/prelude.rs:153`), but the doctested guide still uses it: `docs/GUIDE.md:551,600,656,685,758,798,848` and `docs/guide/fail-loud.md`.
  - The PR body's "every job is green except `test`'s ε=1e-12 row" is false, and so is the brief's premise. Its local checks list no `cargo test --doc`.
  - Also stale: `docs/LIBRARY-DESIGN.md:310`, a NOTE.
- **MINOR-1: products of door-admitted motions are never decided again, so a sub-band defect accumulates past the band. `likely` on severity; demonstrated at geom-core, document path by inspection.**
  - `check_rigid` admits a 4e-10 axis scale (`rigid_motion.rs:679`). Its 64-fold `compose` has `|c0|²−1 = 5.1e-8`, which the same door refuses `NotRigid(col0_unit)` (probe `r2_products_of_admitted_maps_drift_past_the_band`).
  - `transform_rigid` would move a body by that product. On `main`, the composite was decided once at the body door. The path is `placement.rs` `chain_motion` → `Motion::compose` over literal steps or nested group frames.
  - "Products not decided again" is Ev's ruling. But the module doc's "at `f64`, rigid up to rounding" and "at `Interval`, ONE rigid motion per point" (`rigid_motion.rs:16-27`) hold only for mints. For door-admitted factors it is "rigid within the band, times the chain length", and nothing says so.
- **MINOR-2: no committed row guards the inverse's finiteness refusal. `sure`, demonstrated by execution.**
  - Mutant `inv` drops `finite(.., MotionPart::Inverse)` (`rigid_motion.rs:647`). It survives all 1883 rigid/motion/placement/refusal rows of geom-core + topo + editor-core.
  - The refusal is reachable from finite input: a 45° turn with `t = (MAX, MAX, 0)`. Only my probe `r2_inverse_overflow_is_reachable` reds.
  - These are covered only together with a sibling: the angle checks of `rotation_about_axis` and `rotation_then_translation` (one row for both), and the translation check of `rotation_then_translation` (caught via `translation`'s row).
- **MINOR-3: the K-REPORT addendum's argument is false. `sure`, by inspection of the PR's own re-baselines.**
  - `docs/K-REPORT.md:3044` says the `transform_rigid_*` population "only shrinks", so "no rule can fire".
  - The PR's own pins show it growing at literal sites: `asm2a_instantiate` is +10 net per instance, and `msolve11_mate_log` gains 20 door samples at a mate site that asked none. The conclusion may survive, but the reason given is wrong.
- **NOTE-1:** a literal offset is decided at the door twice per instance (once for the content key, once for the op). That is the 466→476. `likely` inefficiency, inspection.
- **NOTE-2:** a non-finite linear entry is now `Undecided` (a poison source) where `main` said `NotRigid`. The text still reads "…or hold a number that is not finite" (`rigid_motion.rs:354`). `sure`, inspection; acceptable.
- **NOTE-3:** "each mint decides only finiteness" is weaker at enclosures: `is_finite_length` waves an `Interval` through (`real.rs:859`), so finiteness there is deferred, not decided. This is sound (it refuses later). `unsure` whether the docs should say so.
- **NOTE-4 (Q4):** `work/frame/exact-frame-mints-cover-three-of-the-world-frames.md` still cites `OrthoFrame` and `ortho_frame.rs`. The new half-turns and the public `UnitVec3::exact_*` answer part of it, and the item is not updated. `likely`.
- **NOTE-5 (Q4):** `docs/MIRROR-DESIGN.md:82` still lists `transform_rigid`'s "typed `NotRigid` refusal", which this PR removed (`TransformError::NotRigid`). `sure`.

## Claims
1. **Soundness of the mints: holds, apart from MINOR-1.** `accepted by inspection`.
   - Rodrigues over a witness axis is a rotation for every unit axis and angle the boxes enclose. `compose` and `inverse` enclose products and transposes. Dual transports the tangent through the same arithmetic.
   - No public path builds the type. The fields are private. `from_linear`, `exact` and `from_aim` are private or crate-private, and so is `UnitVec3::by_construction`. There is no serde, `From` or `Default`. `RigidMotion::compose` is `Affine3`'s product bit for bit (row passes).
   - The door-opened mutant (`check_rigid` returns `Ok`) reds 15 rows across all three crates: shear, mirror and literal-frame rows.
2. **The door is unchanged in what it decides. `sure`, inspection.**
   - It diffs line for line against `main`'s `topo::check_rigid` (`transform.rs:310-366`): same seven margins, three probes, names and F10 flag. The only change is that `Err(_)` now splits into `Undecided`.
   - Folds:
     - `Frame::admission_fault` (`placement.rs:343`) folds `Undecided` into `FrameFault::NotRigid`. It is disclosed, and the "may scale or shear" wording is honest. Unscheduled, but fine.
     - `apply_recuts` (`boolean/ops.rs:5897`) maps any door error to `corrupt`, as `main` did through `transform_rigid`. Pre-existing fold; `unsure` at enclosure lanes.
     - Every other caller (`Motion::rigid`, placed union, STEP `PlacementMap`) keeps the variant.
   - The fold mutant (`Undecided`→`NotRigid`) reds 5 rows.
3. **Nothing `OrthoFrame` promised is lost**, apart from the prelude and guide name (MAJOR-1). All 11 public items carry over. The 7 ported tests keep their names and pass.
4. **Re-baselines are said and right.** `likely`.
   - `chaintol`'s rows and header: re-measured, they match (below).
   - asm2a/kstats: the arithmetic is +20 (two door reads) −10 (body door), and checks out.
   - `OWN_FRAMES` 2→22: matches two literal steps at the door.
   - The m10_6 golden: the explanation is plausible, not independently derived (`unsure`).
   - The refusal-row moves and the pncad tags are consistent by inspection.
   - Nothing is silent, except that K-REPORT's prose is wrong (MINOR-3).
5. **Measurements: reproduced.** Table below.
6. **Phase 1 holds.** On the merge-base: first refusal `col0_unit` at σ = 0.01, reach 1.6667e-10, then `col01_orth`, at one link and at four.
7. **Deviations.**
   - Revolve axis: scheduled on PATHS `the-sketch-plane-is-its-frame`.
   - Mate side: scheduled on INTENT's new item.
   - No second normalize: an improvement.
   - `FrameFault` fold: honest wording, unscheduled.
   - Verbose test migration: unscheduled, see S4.

## Style
- S1 (Q1): pncad-py keeps the retired name: `ortho_frame_error_tag` (`tags.rs:892`), `ortho_frame_err` (`py/place.rs:184`). `sure`.
- S2 (Q1): two rotation families now sit side by side. One normalizes, the other takes a witness: `Mat3::rotation_about` / `rotation_about_unit` (`mat.rs:188,197`), `identity_minus_rotation_about` / `identity_minus_rodrigues`, and `Affine3::rotation_about_axis` / `RigidMotion::rotation_about_axis`. `swept.rs:142` re-spells `rotate_point_about_axis` by hand. `likely`.
- S3 (Q1): near-parallel accessors.
  - `Frame::motion` (a `Map`) vs `Frame::rigid_motion` (decided), `placement.rs:252/268`.
  - `Placement::motion` (`Affine3`, drops the witness) vs `motion_kept`, `placement.rs:835/850`.
  - `likely`.
- S4 (Q7): `RigidMotion::check_rigid(&m, Band::linear(Tol::witness()).unwrap()).unwrap()` is spelled inline dozens of times, often on matrices built with `Mat3::rotation_about` that a mint could build (`review_m4_pr2_transform.rs:125-144`, `mesh/src/curved.rs:1556,2203`). There is no helper. `likely`.
- S5 (Q1): `RigidMotionError` has two spellings of "translation not finite": `NonFinite(MotionPart::Translation)` (a mint) and `NonFiniteTranslation{check}` (the door). `sure`.
- S6 (Q5): the module doc lists unbuilt `turned_between`, `translated`, `embed` and `pin` in present tense (`rigid_motion.rs:80-108`; carried from `main`). It files `turned_between` under "decided mints", while `ops.rs:5895` and the PR call it the by-construction route. `sure`.
- S7 (Q6): the chaintol header's reach (6.49e-9 / 6.73e-10, `chaintol.rs:60-63`) is a measurement with no guard, no register and no reason at the claim site. The bisection row was not committed. `likely`.
- S8 (Q3): `a_widened_rotation_moves_a_body.rs` asserts only `is_ok()`. That is monotone the wrong way: a degraded (wider) witness passes just as easily, and no row checks that the moved cube encloses the rotated corners. `likely`.
- S9 (Q2): `Frame`'s exactness doc, "down to normalizing the axis on this side because that node does" (`placement.rs:~89`), predates the switch to `rotation_about_unit`. `unsure` whether it is still true in the intended reading.
- S10 (Q4): the half-fix check. The rename sweep listed `*.rs` patterns only, so the doctested Markdown (MAJOR-1) and open work items (NOTE-4) were outside what its pattern could match. `sure`.

**Counts:** MAJOR 1, MINOR 3, NOTE 5, style 10. **Ratings (1–5):** correctness 4, design 4, tests 3.
**Questions exercised:** Q1, Q2, Q3 (seven mutants planted), Q4, Q5, Q6, Q7, Q8 (`rigid_motion.rs` read end to end).

## Gate verification
- The hosted run has two red steps in `test`:
  - ε = 1e-12, `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`. Reproduced red on the merge-base `818e51b3ee` and on the head (same panic at `coincidence_door.rs:44`; filed four times under `work/flush/`). Not this PR's.
  - `doc-tests`: this PR's (MAJOR-1).
- Every other job is green.

## Run locally (debug unless noted; contended box)
- `nextest --profile ci -p geom-core -p topo -p editor-core` on the head: 6755 passed, 0 failed.
- Mutants via `R2_MUT`:
  - `open`: 15 red;
  - `fold`: 5 red;
  - `tr`: 2 red;
  - `cmp`, `ang`, `anc`: 1 red each;
  - `inv`: 0 committed rows red (only my probe).
- Reach, by bisection in log σ (40 steps), release, row `r2_reach_probe`, one leaf over the whole box. Mine beside the PR's:

| links | merge-base, mine | head, mine | PR before → after |
|---|---|---|---|
| 1 | 1.6667e-10 (`col01_orth`) | 6.5105e-9 (`pcurve_envelope`, [0, 1.0000000000027e-9]) | 1.666e-10 → 6.49e-9 (refuses 6.54e-9) |
| 4 | 1.6667e-10 (`col01_orth`) | 6.7641e-10 (`pcurve_envelope`) | 1.666e-10 → 6.73e-10 (refuses 6.78e-10) |

- At σ = 0.01 on the head: `carrier_endpoint_start` `[0, 8.9988e-5]` on `EdgeKey(1v1)`, 8 refused at one link and 35 at four (matches the PR).
- Symbolic, one link: `symbolic_zero` 2008 → 1978, `frozen` 2804 → 2626, `numeric` 1353 on both, CERTIFIES. That is exact to the PR. My times (0.25 / 0.28 s) were taken under load, so I don't compare them.

**Probe commit:** `e692e0b34` on `sym/17-review-r2`:
- the `#[ignore]`d `chaintol::r2_reach_probe`;
- `rigid_motion.rs`'s `R2_MUT` mutant switch, off by default;
- the `r2_probes` module.

Not for merge.
