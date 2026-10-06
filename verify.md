# Verification of PR #4122, frozen head `28bb10d2`

Verifier lane for the REACH orchestrator, 2026-10-06.

- **Head.** `git rev-parse origin/reach/operand-gate-separating-direction` = `28bb10d2017a5a597932c5c3063dc5f7d83983a4`, the frozen head. The branch has not moved past it.
- **Inputs.** I read the claims from the PR body's "Last fix pass" section, through `pull_request_read get`. I read no PR comments or reviews. The findings are from `analysis/reach-review/4122` (`review.md`, `probes/`).
- **Builds.** I worked in my own worktrees at the frozen head, with `CARGO_INCREMENTAL=0` and target directories outside the checkout.
- **Mutants.** Every mutant is one source edit behind an environment switch, all compiled into one build. With every switch off, that build gives verdicts identical to the frozen head on all 24,282 oracle runs: I diffed every run's verdict at all three ε.

**Verdict: VERIFIED.** No touching or crossing pair was cleared by `separating::apart` to a wrong body, in any row, probe or random pose. Every claim in the "Last fix pass" section that I could check holds. One overstatement is noted, and it does not block. Before merging, two non-blocking conditions stand (see the end): no hosted CI has run on `28bb10d2`, and the branch now conflicts with `origin/main` in `ops.rs`.

## Mutants

"Red" means the row fails. Each mutant was applied as the smallest edit matching its description, and reverted by switching it off.

| mutant | edit | row | ε | result | as claimed? |
|---|---|---|---|---|---|
| **M3** (reviewer's) | every reach in `apart_along` shrunk by 0.1% of its width at each end | `operand_gate_support_plates::plates_at_the_exact_support…` | 1e-9 | **red** | yes |
| M3 | 〃 | 〃 | 1e-12 | **red** | yes |
| M3 | 〃 | 〃 | 1e-6 | green | yes (the pad; see below) |
| M3 | 〃 | the four `operand_gate_pose` rows | all three | green | yes (the reviewer's MINOR-1) |
| M3 | 〃 | reviewer probe (b) `review4122_plates_at_the_exact_support` | 1e-9, 1e-12 | red, with *new* non-NOTE-2 cases: 270° torus, s = 1e-3, dir 3, δ ∈ {0, −1e-9, −1e-12} | yes |
| M3 | 〃 | my random-pose oracle | 1e-9 / 1e-6 / 1e-12 | **red**: 77 / 50 / 56 wrong | — |
| M3 at 0.05% | weaker shrink | probe (b) | 1e-9 | only the 6 baseline NOTE-2 cases, nothing new | yes ("leaves the probe green too") |
| M3 at 0.05% | 〃 | support-plate row | 1e-9 | green | — |
| M3 at 0.2%, 0.5%, 1%, 5% | stronger shrink | support-plate row | 1e-6 | **red** at every strength (first: 270° and full torus at s = 1, δ ∈ {0, −1e-6}) | the row is live at 1e-6 |
| **M4** | the Approx arm asks `Item::Face(sphere face)`, *and* a sphere face's reach is narrowed to its boundary hull | `ops::tests::the_approx_arm_asks_whether_the_ball_reaches_the_face` | 1e-9 | **red**: "inside the ball, off the cap: face FaceKey(1v1)" | yes |
| M4, face half only | `Item::Face(sphere face)`, reach unchanged | 〃 | 1e-9 | green | yes (today's equivalence) |
| M4, narrowing half only | `WholeBall` reach replaced by `boundary_reach` | 〃 | 1e-9 | green | yes: the arm does not read the sphere face's reach |
| NOTE-5 old oracle | the row's half-width as `rho*(1 - ni*ni).sqrt()` | `boxes::tests::a_tilted_circles_box_is_its_own_extent` | 1e-9 | **red** on the near-ẑ normal, axis 2: [1.0999999984, 1.1000000016] against [1.1, 1.1] | yes |
| circle box as a cube (first pass) | `circle_box`'s in-plane frame replaced by x̂, (0,1,1)/√2 | 〃, and the reviewer's `circle_box` probe | 1e-9 | **red**, axis 0: [−0.4, 1.0] against [−0.3745, 0.9745]; the probe is red too | yes |
| **M1** (first pass, reviewer's) | `apart` tries x̂, ŷ, ẑ only | cone-clear, ball and Approx rows | 1e-9 | **red** | yes |
| M1 | 〃 | `a_bar_through_the_cone_wall_refuses_at_every_pose` (the twin) | 1e-9 | green | yes |
| **M2** (reviewer's) | `apart` always true | the twin | 1e-9 | **red** | yes |
| M2 | 〃 | my oracle | all three | **red**: 746 / 589 / 510 wrong | — |
| **M5** (mine) | `apart` always false | NOTE-2 cases in probe (b) | 1e-9 | the same 6 cases as the head | NOTE-2 is not `apart`'s doing |
| M5 | 〃 | cone-clear, ball and Approx rows | 1e-9 | red (main's refusals come back) | — |

**M3 at ε 1e-6: is it the pad or a blind row?** It is the pad.

- `sweep_pad` is K·ε + 2ε = 12ε, with K = 10. A clear needs gap − 2·pad ≥ K·ε, so a gap of about 34ε: 3.4e-5 at ε 1e-6 and 3.4e-8 at ε 1e-9.
- The killing fixture (s = 1e-3) gains about 1.5e-6 of gap under the 0.1% shrink. That is below the threshold at ε 1e-6 and above it at ε 1e-9 and 1e-12. This matches the observed red/green split.
- At ε 1e-6 the row is red from 0.2% upward, with 0 poses skipped, so it is not blind there.

**A caveat on the M3 kill.**
- The row's red under the 0.1% shrink rests on a single fixture: one direction at one scale, at two of the three ε.
- At s = 1 and s = 1e3 the 0.1% shrink is 1e-3 to 1 absolute, far above any pad, and still does not turn the row red. Later doors refuse or record the contact there. The kill is real but thin.
- My oracle supplements it: it kills M3 at all three ε.

## ε results (frozen head)

| run | ε 1e-9 | ε 1e-6 | ε 1e-12 |
|---|---|---|---|
| `operand_gate_pose` (4 rows), `operand_gate_support_plates`, `n3r1_prune` (2), `germ_interior_oval::a_plane_clear_of_the_bumps_net_never_reaches_the_certificate` | 8/8 | 8/8 | 8/8 |
| support-plate row counts | runs 1080, built 230, skipped 0 | runs 1080, built 140, skipped 0 | runs 735, built 235, skipped 23 (as claimed) |
| `boxes::tests::a_tilted_circles_box_is_its_own_extent`, `boxes::tests::every_door_that_reads_a_box_is_inventoried`, `ops::tests::the_approx_arm_asks_whether_the_ball_reaches_the_face` | 3/3 | 3/3 | 3/3 |
| reviewer `circle_box` probe (adapted to `circle_box(&Circle, pad)`) | pass (worst looseness 1.1e-13·ρ) | pass | pass |
| reviewer probe (a), `review4122_touching_pairs_never_clear` | pass | pass | pass |
| reviewer probe (c), `review4122_built_results_hold_analytic_membership` | pass | pass | pass |
| reviewer probe (b), `review4122_plates_at_the_exact_support` | red, 6 cases, **all NOTE-2** | red, 6 cases, all NOTE-2 | red, 1 case, NOTE-2 |
| nextest `-p topo -p sweep -p geom-core`, default profile (slow set included) | **5437/5437** | — | — |

- **Probe (b) is as the review said it would be.** Every failing case is the 270° torus touched at δ = 0 where the support falls on a cut. That is the case the PR row excludes, the one the review predicted. M5 shows the same cases with `apart` disabled.
- **No red needed a check against main.** `geom-core`'s `bounds_census` passes on this head. The head's base (`88e09fb9`) predates main's blend `surgery.rs` red.

## Independent oracle: random poses

This is my own test, not the PR's or the reviewer's. It is in my scratch space, not pushed.

**Families.** The PR's own:
- the annular cone frustum, full and 270°;
- the torus, full and 270°;
- the ball, R = 0.5·s.

Each is set against a bar of random dimensions.

**Placing the bar.**
- The bar's **vertex, edge or face** sits at a closed-form signed gap δ from the solid's support along a direction d.
- For half the frustum cases, d is the cone wall's normal, so the contact lies in the wall's interior rather than on a rim.
- The bar's corner is at support + δ·d, and every one of its edge directions has d·e ≥ 0. So the gap is exactly δ.
- δ ∈ {1e-3·s, 1e-6·s, 0, −ε, −1e-3·s} and s ∈ {1e-3, 1, 1e3}.
- A random rigid pose is applied to the pair, and every op runs in both orders (∪, ∩, A∖B, B∖A).

**What counts as wrong.**
- δ < −ε and ∪ comes back an `Assembly` or ∩ comes back empty.
- δ > 0 and ∩ builds a body.
- Any built result misclassifies a sample under `point_in_solid`, checked against analytic membership. Samples are dense near the contact and wide around it; any sample within 1e-7·s of a boundary is skipped.

**What counts as a flag.** δ ∈ {0, −ε} built as disjoint is flagged, not counted wrong.

| ε | posed cases | runs | built | refusal rate δ>0 / touch / cross | samples | **wrong** | touch built as disjoint |
|---|---|---|---|---|---|---|---|
| 1e-9 | 1500 | 9000 | 1781 | 0.459 / 0.925 / 0.990 | 49,757 | **0** | 259 |
| 1e-6 | 1500 | 9000 | 1136 | 0.655 / 0.955 / 0.993 | 31,737 | **0** | 166 |
| 1e-12 | 1047 (453 refused by the move or the at-rest gate) | 6282 | 1428 | 0.348 / 0.926 / 0.986 | 39,903 | **0** | 180 |

**The touch flags.**
- Every flag is a **torus** at **δ = 0** exactly; none is at −ε.
- They are point or line contacts: a bar vertex on the face, or a support on a 270° cut.
- ∪ is an `Assembly` with the contact recorded: 64 with 1 contact and 2 with 2, on the cut cases. ∩ is empty.
- That is the right regularized answer for a point touch. These flags are the NOTE-2 family and its full-torus analogue.

**Was any touching pair cleared by `apart`?** I diffed every run's verdict, head against M5 (`apart` never clears), over all 24,282 runs.
- Every δ > 0 difference is a pair the head builds and M5 refuses: 474 / 275 / 396 at ε 1e-9 / 1e-6 / 1e-12. This is the completeness the PR adds. At 1e-6, 19 more differ only in which refusal they give.
- Exactly **one** δ ≤ 0 case changes from refused to built: case 1155 at ε 1e-9, a 270° torus at s = 1e-3 with a bar vertex touching at δ = 0, not on a cut.
- The head's answers there are ∪ `Assembly` with 1 contact recorded, ∩ empty, and A∖B and B∖A each the operand unchanged. That is correct for a point touch, and membership sampling agrees.
- What `apart` clears there is the bar's other edges against the torus face. Those really are apart. M5 refuses on them with `CurvedPierceUnsupported`.

**The oracle can go red.** It reports 50–77 wrong under M3 and 510–746 under M2.

**The Approx arm asks about the whole ball.** Checked three ways:
- by inspection: `ball_may_reach` takes only the centre, the radius and the ball's box;
- the narrowing-only half of M4 leaves the arm's row green;
- under the same narrowing, `an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose`, the ball row and the support-plate row are all unchanged (green).

## Claim checks (the "Last fix pass" section)

| claim | check | result |
|---|---|---|
| MINOR-1: probe (b) and the membership oracle are promoted as `operand_gate_support_plates`; δ ∈ {1e-6·s, 0, −ε}; 6 directions; 3 scales; 23 skips at 1e-12, 0 elsewhere; red under M3 at 1e-9 and 1e-12 (270° torus, s = 1e-3, dir 3, δ = 0 and −ε); green at 1e-6 | ran it; read the source | **true**. **Overstated:** "every op runs in both orders (∪, ∩, A∖B, B∖A)". ∩ runs `a∩b` only (`operand_gate_support_plates.rs:260`). My oracle runs `b∩a` as well and finds 0 wrong. |
| NOTE-2 excluded from the disjoint check only; still sampled; reds without the exclusion at s = 1e3, dir 1 and dir 4, as on main | source; probe (b) shows those cases; M5 shows they are not `apart`'s | true |
| NOTE-2 filed; "the contact is recorded" | file exists; my oracle's cut-touch ∪ results all carry contacts | true (I did not separately confirm the contact kind is `VfContact`) |
| NOTE-4: `Item::Ball`; `ball_may_reach`; the sphere face's reach no longer read | source; M4 and both its halves | true |
| NOTE-3: doc fixed | `reduce.rs:271–276` | true |
| NOTE-5: oracle by `hypot`, plus the near-ẑ normal | source; old-oracle mutant | true |
| MINOR-2 / S8: lazy `OperandAxes` shared per stage; one cell for both sweep directions; ± deduplicated by `axis_key`; none computed for all-planar ops whose boxes never overlap; `separating.rs` takes no `Bounds` bound | read `separating.rs`, `reduce.rs`, `ops.rs` | true. Reusing the first read's axes after the sweep splits the bodies is sound, since any direction is a valid certificate. |
| S1: the circle spelled once | `ops.rs` call site | true |
| S5: `FACE_FREE_RECORDS` and its filter gone; the hone item updated | diff | true |
| S6: renamed `…_never_reaches_the_certificate` | diff | true |
| filed (b) NOTE-1 evidence and (c) the S3 row item | `work/reach/` | true; `work.py lint` ok |
| support-plate row about 35 s debug; joins the `ci` slow set | 25 s debug here; listed in `.config/nextest.toml` | true |
| verification at `28bb10d2`: the box-door inventory row green; the rest green at three ε | my runs above | true |

**A test-hygiene note, not blocking.** `the_approx_arm_asks_whether_the_ball_reaches_the_face` (`ops.rs:5787`) shares one `OperandAxes` cell across both bricks. The second brick reads the first's axes. That is harmless here: both bricks are axis-aligned, and the "inside" verdict does not depend on the axes.

## Review findings: disposition

| finding | disposition |
|---|---|
| MINOR-1 | fixed, by the support-plate row (M3 kill verified, thin) |
| MINOR-2 | fixed (lazy shared axes, ± dedup) |
| NOTE-1 | filed as evidence on the seam-anchor item |
| NOTE-2 | excluded and filed; confirmed not `apart`'s (M5) |
| NOTE-3, NOTE-4, NOTE-5 | fixed |
| NOTE-6 (CI red) | see the conditions below |
| S1, S5, S6 | fixed |
| S3 | filed |
| S7 | was a confirmation; nothing to do |
| S2 (second boundary walk in `separating::anchor`, "unsure") | **not mentioned** in the fix pass. It is a style question the reviewer marked unsure, so it does not block. |
| Q8 (`ops.rs` not read end to end) | reviewer scope; not a finding |

No finding is left unaddressed without a reason, apart from S2, which is an optional style question.

## Conditions before merging (non-blocking for this verification)

1. **No hosted CI has run on `28bb10d2`.** The latest CI run is 37432923247, on `2bbba716`. The commit status for the head is `pending`, with 0 statuses. Hosted CI is the verification of record, and it has to run on whatever head merges.
2. **The branch conflicts with `origin/main`.** Main is now 239 commits ahead, at `33e5000f`.
   - The conflicts are in `crates/topo/src/boolean/ops.rs`, from #4099's boundary-iterator rewrite and the merged PR 4044 (trimmed-sphere escape).
   - **`face_boundary_meets`.** The PR's `apart(edge, circle)` sits inside a loop main rewrote.
   - **`sphere_faces_apart`.** Main changed the return type to `Option<(FaceKey, SectionRefusal)>`; the PR adds the `axes` parameter.
   - Both conflicts look mechanical. But a resolution that drops the narrow-phase call only costs completeness, while one that mis-wires the circle item could cost soundness.
   - After the merge, re-run `operand_gate_pose`, `operand_gate_support_plates` and `the_approx_arm_asks_whether_the_ball_reaches_the_face` at three ε.
   - Main's known reds then come along with the merge. Among them is `bounds_census::every_sole_bracket_bound_door_is_in_the_roster` (blend `surgery.rs`), which is not this PR's.
