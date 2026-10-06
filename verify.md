# Verification of PR #4128, frozen head 723fbdeb0a

**Verdict: VERIFIED.** No blocking point. One NOTE (band-edge rounding at ε 1e-12 × 1e3 m), and the one mutant the lane already discloses as surviving.

- Branch `reach/pierce-tangent-off-face` had not moved: `origin` head = `723fbdeb0a8a7497d77a3fa511ddcd96e4ba068a`.
- Review frozen head `4ec0d4b25c`. The lane's commits after it are `cc1312dc` (carrier_touch), `22676135` (splitting) and `723fbdeb` (census port); the rest are merges of main.
- The PR body was read through `pull_request_read get` only. No comments or reviews were read.
- Everything ran in a private worktree and target dir. The probes were mounted untracked and nothing was pushed to the PR branch.

## Review findings

| finding | claim | checked | result |
|---|---|---|---|
| MAJOR-1 (ellipse stored minor first) | `speed_bound` reads `Conic::speed_hi`/`curvature_hi`; annulus `[min, max]` of magnitudes | diff read; `curvature_hi = speed_hi/speed_lo²` = a/b² in either order; mutants L1–L3; reviewer probe `a_swapped_ellipse_keeps_its_crossings` (60 poses) passes on head; my swapped-storage probe (351 poses × 3 ε, below) 0 misses; edge-clearance probe 1800 swapped-ellipse queries × 3 ε, 0 wrong | **fixed** |
| MAJOR-1 sweep | containment.rs ×2, join.rs ×1 fixed; listed readers symmetric; 3 refusal-side filed | own grep of every non-test `Curve3::Ellipse` destructuring (`crates/*/src`). Beyond the lane's list, `scalar_lift`, `quad_lane`, `transform`, `pcurve.rs`, `emit_topo`, `chord_join` frame and the step-export writer read major/minor as **parameter-frame coordinates** (along `u_ref` / `axis×u_ref`), which is order-agnostic. `solid_contain.rs:1313` refuses a swapped section (its `minor == radius` seat fails), confirming it is refusal-side. The rest are test fixtures or torus radii | **complete**; I found no unlisted order-assuming reader |
| MINOR-1 (necessity paragraph) | argues from `Dual64: CertifiedEnclosure`, admits the box grant | `real.rs:1254-1275` and the gate comment read. `inside_out_operand.rs:97-103` instantiates the boolean at `Dual64`, and `CertifiedEnclosure` has impls only for `f64`, `Interval`, `Sym<T>` and `Probe`. The paragraph names `edge_clear_of_ball`'s box miss as a terminal `Bounds` grant in the #571 direction. `bounds-allowlist.sh` passes | **fixed** |
| MINOR-2 (stale module doc) | names `carrier_touch::off_face`, states both radii | `pierce_tangent_off_face.rs:1-26` read | **fixed** |
| NOTE-1 (conic × torus not observable) | filed | `work/reach/a-conic-touching-a-torus-off-its-face-passes-the-pierce-and-refuses-at-the-section-pass.md` exists; the reviewer's `circle_torus_rows`/`ellipse_torus_rows` pass on head at 1e-9 and 1e-6 | **filed** |
| NOTE-2 (K telemetry) | filed | `work/reach/the-off-face-touch-decisions-have-uncalibrated-margins.md` exists | **filed** |
| NOTE-3 (×1e3 lens at 1e-12) | not this PR's | reproduced. At 1e-12 the ×1e3 balls are not finished operands (`VolumeUncomputable`, `props_band_opposite`), so the reviewer's `scaled_item_poses_build`/`scaled_near_rim_poses` red at 1e-12 and my e2e skips that scale there | **explained** |
| Q1 `distance` vs `carrier_eq::distance_to` | kept; doc cites the other | doc read | explained |
| Q1/Q7 budgets, floor | derivations at the sites | read. `CLUSTER_BUDGET = 8` matches the conic × torus degree. I did not re-measure "70–150 pieces a meeting" | explained |
| Q3 coverage | rows for the torus inner side, two touches over the axis, midpoint on the axis, ellipse span, elliptic edge | rows exist and pass at three ε | **fixed** |
| Q4 ON-endpoint arms | comment argues no disagreement | `reduce.rs:2304-2319` read. The arithmetic holds: the ON end is within ℓ+\|d(m)\| of the foot and the boundary is beyond ℓ+\|d(m)\|+escalate, so it is > escalate from the end | explained |
| Q6 straddle / spline-spiric residues | filed | both items exist | filed |

## Mutants

Each mutant was applied by hand as the smallest edit matching its description, the named rows were run, and the file was restored byte for byte. The lane's L1–L7 are at ε 1e-9; R1–R3 are the reviewer's.

| # | mutant | row(s) | result | claim |
|---|---|---|---|---|
| L1 | head's ellipse speed `(major, major/minor²)` | `carrier_touch_rows::an_ellipse_keeps_its_crossings_in_either_storage` | **red** | killed ✓ |
| L2 | head's annulus `(minor, major)` | `carrier_touch_rows::an_elliptic_edge_is_not_cleared_of_a_ball_on_it_in_either_storage` | **red** | killed ✓ |
| L3 | ellipse speed = field `minor` | `an_ellipse_keeps_its_crossings_in_either_storage` | **red** | killed ✓ |
| L4 | `off_face` always false | `pierce_tangent_off_face::*` (10) | **8 red**; the two refusal rows green | ✓ (8 of 10, the inner-equator and over-the-axis rows among them) |
| L5 | `ConicArc::hit` Hessian from `b` | `containment::tests::an_ellipse_stored_minor_first_tighter_than_the_band_straddles_it` | **red** | killed ✓ |
| L6 | `ConicArc::of` speed lever from `a` | all 9 `containment::tests` | green (survives) | disclosed as surviving ✓ |
| L7 | `certify_section_area` perimeter by `sa` | all 47 `splitting::` rows | green (survives) | disclosed as "no row" ✓ |
| R1 | drop the face check (`ball_off_face` true once in reach) | `pierce_tangent_off_face::*` | red: `a_touch_just_inside_the_lens_face_refuses`, `a_touch_outside_the_lens_face_within_reach_of_its_rim_refuses`; 8 build rows green | ✓ |
| R2 | drop only the edge clearance | same | red: `a_touch_outside_the_lens_face_within_reach_of_its_rim_refuses` only | ✓ |
| R3 | Lipschitz bound only | same | red: `a_touch_just_outside_the_lens_face_builds` only | ✓ |

My own probes, checked against the same mutants to show they can go red:

| mutant | `verify_clusters_keep_every_meeting_in_a_ball` | `verify_edge_clearance_is_a_lower_bound` | `verify_random_lens_brick_poses` (e2e) |
|---|---|---|---|
| L1 | **red** | green | — |
| L2 | green | **red** | — |
| L3 | **red** | green | — |
| R3 | green (sound, only looser) | — | — |
| R1 | — | — | **red** (504 on-face touches built) |
| L4 | — | — | green, no wrong; off-face and near-rim build 1068/2016 and 0/2016 against 2016/2004 on head, the PR's new reach |

Reviewer probes on head: `carrier_touch_localization.rs` 5/5 pass at 1e-9, 1e-6 and 1e-12 (`clusters_hold_every_band_meeting` 1620 poses; `a_swapped_ellipse_keeps_its_crossings` 60 poses). `e2e_pierce_tangent.rs` 9/9 pass at 1e-9 and 1e-6; at 1e-12, 7/9, the two reds being NOTE-3's ×1e3 operand.

One caveat on `a_swapped_ellipse_keeps_its_crossings`: it `continue`s on a `None` (budget refusal), so it could pass vacuously. My probe counts refusals instead: 0 of 351 swapped-ellipse poses refused at each ε.

## ε results

PR rows: `carrier_touch_rows` (3), `pierce_tangent_off_face` (10), the two new `containment::tests`, the `boxes` inventory and `offer_rows` rows.

| ε | PR rows | reviewer probes | my clusters probe (1404 poses) | my edge probe (7200 queries) | my e2e (lens × edge brick) |
|---|---|---|---|---|---|
| 1e-9 | all pass | all pass | 0 misses, 0 refused | 0 wrong | 1008 poses, 6048 runs, 0 wrong |
| 1e-6 | all pass | all pass | 0 misses, 0 refused | 0 wrong | 1008 poses, 6048 runs, 0 wrong |
| 1e-12 | all pass | 2 red (NOTE-3, main's) | 4 band-edge misses (NOTE below), 0 refused | 0 wrong | 672 poses (×1e3 skipped, NOTE-3), 4032 runs, 0 wrong |

**nextest, ε 1e-9, `-p geom-brep -p geom-core -p topo -p sweep`** (head, reviewer and verifier probes mounted): 6533 rows, **6532 pass**, 1 red. The red, `sweep::all every_suite_file_is_aggregated`, is my own probe mounts in `crates/sweep/tests/all.rs`: with them removed the row passes. **0 reds attributable to the PR.** `bounds_census` passes on head and is red on `origin/main` 78bee3ac (`every_sole_bracket_bound_door_is_in_the_roster`, reproduced here), so the port in 723fbdeb is what makes it green. At ε 1e-6 and 1e-12 only the rows above were run, not the full crates.

### My widened oracle (step 6)

- **`verify_clusters_keep_every_meeting_in_a_ball`** (`clusters` against an independent closed form):
  - Poses: sphere / cylinder / ring torus (inner side included) × line / circle / ellipse ordered / ellipse **swapped**, × scale 1e-3, 1, 1e3. Each curve is tangent at a random carrier point, offset Δ ∈ {0, ±0.3·zero, ±escalate, ±2·escalate, ±30·escalate, ±1e-7·scale, +1e-4·scale, −1e-2·scale (a true crossing)}. That is 1404 poses per ε, about 1000 of which truly meet the carrier or come within escalate of it.
  - Oracle: a 40k-sample signed distance, with bisection at every sign change and golden-section refinement at every local minimum of |d|.
  - Checks: every oracle meeting (|d| ≤ escalate, or a root) lies in a returned cluster, AND its true foot lies in that cluster's ball `ℓ + |d(m)| + escalate` about the middle's foot (the triangle-inequality chain `ball_off_face` relies on).
- **`verify_edge_clearance_is_a_lower_bound`:**
  - Random line, circle, ellipse-ordered and ellipse-swapped edges built through `mev` with an intersection description, × three scales.
  - Random balls at 1e-4..1 × scale.
  - A "clear" verdict must mean the true distance exceeds the radius: 0 wrong of about 3200 clears per ε.
- **`verify_random_lens_brick_poses`** (public boolean, all six op/order runs):
  - The PR's lens × edge-brick family at random touch points and directions.
  - Families: off-face (touch, ±in-band, and true sphere crossings outside the lens); near-rim off (within 0.1 of the rim); on-face touch; on-face ±2·escalate; on-face crossing 1e-3 deep.
  - Oracle: closed-form volumes for the disjoint truths, inclusion–exclusion between the ops that built, and `point_in_solid` at points whose side is known by construction (lens centre, brick interior, and a point in both operands when they cross).
  - The membership query itself escalates in a few k = 1e-3, ε 1e-6 poses; I count those as unknown, not wrong.

  | ε | family | runs | built | refused | wrong | refusals |
  |---|---|---|---|---|---|---|
  | 1e-9 | off-face | 2016 | 2016 | 0 | 0 | — |
  | 1e-9 | near-rim off | 2016 | 2004 | 12 (0.6%) | 0 | Escalated |
  | 1e-9 | on-face touch | 504 | 0 | 504 | 0 | CurvedPierceUnsupported |
  | 1e-9 | on-face +2e | 504 | 504 | 0 | 0 | — |
  | 1e-9 | on-face −2e | 504 | 0 | 504 | 0 | Escalated 444, Join 60 |
  | 1e-9 | on-face crossing | 504 | 24 | 480 | 0 | Join |
  | 1e-6 | off-face | 2016 | 1938 | 78 (3.9%) | 0 | Escalated 72, Pieces 6 |
  | 1e-6 | near-rim off | 2016 | 1320 | 696 (34.5%) | 0 | CurvedPierceUnsupported 438, Escalated 258 |
  | 1e-6 | on-face touch | 504 | 0 | 504 | 0 | CurvedPierceUnsupported 456, Escalated 48 |
  | 1e-6 | on-face +2e | 504 | 450 | 54 | 0 | Escalated |
  | 1e-6 | on-face −2e | 504 | 0 | 504 | 0 | Escalated 498, Join 6 |
  | 1e-6 | on-face crossing | 504 | 6 | 498 | 0 | CurvedPierceUnsupported 114, Escalated 54, Join 330 |
  | 1e-12 | off-face | 1344 | 1344 | 0 | 0 | — |
  | 1e-12 | near-rim off | 1344 | 1344 | 0 | 0 | — |
  | 1e-12 | on-face touch | 336 | 0 | 336 | 0 | CurvedPierceUnsupported |
  | 1e-12 | on-face +2e | 336 | 336 | 0 | 0 | — |
  | 1e-12 | on-face −2e | 336 | 0 | 336 | 0 | Escalated 294, Join 42 |
  | 1e-12 | on-face crossing | 336 | 18 | 318 | 0 | Join |

  **No on-face touch ever builds** (0 of 1344 runs across the three ε). The on-face +2e builds do not come through `OffFace`: with `off_face` forced false (L4) they still build 504/504, through the certified no-root path. A touch 2·escalate outside the face is outside the band.

**NOTE (not blocking): band-edge misses at ε 1e-12 and scale 1e3.**
- Four of 1404 poses at 1e-12 had an oracle "meeting" outside every cluster. In two of them the span returned no cluster at all.
- All four have Δ = +escalate exactly (the band's edge), at scale 1e3. Their minimum |d| is 0.978–1.035·escalate, no crossing is involved, and the one sign change in those spans lies in a kept cluster.
- A dedicated sweep (`verify_band_edge_misses`: Δ = ±{0.5, 0.8, 0.9, 0.95, 0.99}·escalate, 720 poses per cell, three scales, three ε) gives 0 misses everywhere, except at ε 1e-12 and scale 1e3 for 0.95 (3/720) and 0.99 (26/720). The smallest missed |d| there is 0.955·escalate.
- At 1e3 m coordinates, escalate = 1e-11 is about 90 ulps of the coordinates. 4.5% of it (~4.5e-13 m) is within the rounding of the distance both the kernel and my oracle compute from the stored geometry.
- The curve at those poses stays more than 9·zero off the carrier, so no crossing is lost and no wrong body follows. But strictly "within the band" is not decided there for f64 at that scale.
- The reviewer's 1620-pose probe scaled Δ with the pose and never sat at the band edge. Not this PR's to fix alone: a generic f64-decide resolution question at ε 1e-12 × 1e3 m.

## Claim checks (Last fix pass)

| claim | result |
|---|---|
| `ball_off_face` walks the boundary through `face_boundary_linked` | true (`carrier_touch.rs:298-306`) |
| `Conic::curvature_hi` added beside `speed_hi`/`speed_lo`, not a fourth copy | true (`implicit.rs:709-715`) |
| containment.rs: `hit` Hessian from the smaller magnitude; `of` lever `max(|a|,|b|)` | true (diff `22676135`) |
| join.rs perimeter metered by `ConicFrame::reach` | true; `reach = max(|major|,|minor|)` (`loop_winding.rs:177-195`) |
| the order sweep is complete | true by my own grep (above) |
| refusal-side readers `chord_join`, `solid_contain`, `pcurve_cache` | `solid_contain` refusal-side confirmed; `chord_join` reads a frame (order-agnostic); `pcurve_cache:7200-7275` not re-read |
| every kernel producer mints through `Curve3::ellipse`; STEP is the one source | not independently re-proven; consistent with the filed item |
| census port `723fbdeb` fixes main's `bounds_census` red | true: red on `origin/main` 78bee3ac (reproduced), green on head |
| suites 5473/5473 (1e-9), 5471/5473 (1e-6), 5473/5473 (1e-12) before the last merge | not re-run at that head; this head's 1e-9 run is above |
| `cargo build`, clippy, fmt, gates, `work.py lint` clean | `bounds-allowlist.sh`, `loop-boundary-discards.sh` and `work.py lint` pass here; clippy and fmt not re-run |
| "70–150 pieces per meeting" | not re-measured |

## Diff read adversarially (commits after 4ec0d4b25c)

No false or overstated claim found in the lane's three commits.
- `speed_bound` routes Circle through `Conic` too: `(|r|, 1/|r|)`, the same as before.
- `Conic::of` returns `None` for other kinds, which keeps the door.
- The annulus is a true lower bound in-plane for any storage.
- The `real.rs` paragraph now matches the code, the box grant included.
- The Q4 comment's arithmetic holds.

Two observations:
- The PR's `a_swapped_ellipse` reviewer probe, adopted as a row, exercises clusters only. `off_face` with a swapped ellipse is reached end to end only through STEP (filed).
- L6 survives by design; its row pins the population, not the bound.
