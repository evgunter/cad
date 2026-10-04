# Review r1: PR 4038 at frozen head `e2e114be`

**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 3 · NOTE 3.
No wrong body ships. Over 74 000 lines of new and r1 poses, no line goes from SOUND to a refusal and none goes to a definite BAD. Every refusal→BAD reaches BAD only through a filed limitation: an escalated census, or the point-in-solid probe on a curved operand. Every two-lobe ∩ builds by the walk, and the mutants are real. The fix asked for is a row for two pinches in one op: that path is reached and works, but nothing pins it.

**Method.** Release builds of the head and of **`8793177b`**, the head's own merge base, which differs from it by exactly the PR's 18 files. Comparing against `81dde823` would mix in 77 main commits (N1). The probes are `crates/sweep/examples/r1b_pinch_probes.rs` (new) and `r1_pierce_probes.rs` (PR 4026 r1's battery, ported to `AtRestBody`). Each line is `differential::outcome`: tier 2, tier 3′, the certificate, a legal operand and the volume to 1e-7. I added `twov` (faces running through two distinct vertices at one point) and tier 3 (`validate_geometric`).

Two oracles, both kernel-free:
- planar: convex pieces clipped by the cube, volume by the divergence theorem;
- cylinder: z-slices clipped to the cylinder's strip, tanh-sinh quadrature between the breakpoints.

The mutants and a split logger are env switches in `review-r1b/mutants.patch`. Every output is in `review-r1b/`, with `cmp.py` to diff them.

## Claims

1. **No wrong body ships: HOLDS (executed).**
   - New planar corners: Ltop/Lbot turned −45°, 30°, 100° and 200°; `Lmid` (v at mid-height of the reflex edge); a 327° notch; a U's inner corner. 240 directions plus near-tangent tilts at ±1e-3 and ±1e-6, every op, both orders: 28 512 lines. Base→head: 1 110 refusal→SOUND, 51 refusal→`PinchUncrossed`, **0 →BAD, 0 SOUND→refusal**. The 2 BAD lines (Uin fib0 ∪/∖, t3′) are identical on base.
   - The same corners at ±1e-7 and ±1e-8: 761 refusal→SOUND and 5 refusal→BAD. All 5 are `CensusEscalated` (m2). Tier 3 passes and the volume is exact on all 5.
   - Chains of fusions: a staircase whose two reflex corners lie on one cube-face plane, so one op pinches twice (`u2`, 720 lines). 103 refusal→SOUND, 0 BAD, 0 SOUND→refusal. 137 ops cross two pinches; 43 cross one and refuse `PinchUncrossed` at the second.
   - Curved pierced face: a cylinder wall (axis y, r=5), three corners, six wall angles, 24 turns, 684 lines. 24 ∩ lines go refusal→BAD, and only on the legal-operand check (m3).
   - `twov = 0` on all 58 429 head lines: no face meets two vertices at one point.
   - "The first pair the orbit offers": the `lastsite` variant crosses on the last qualifying pair instead. On r1's 15 552 lines it is byte-identical to the head, volumes included.
   - `anysurf` (two faces of different surfaces may cross) goes red in 4 rows, so the surface guard is pinned.
   - No definite refusal→BAD turned up, so none needed checking without the census.
2. **The pre-pass is sound and complete: HOLDS, with a test gap (m1).**
   - No missed pinch. On the head, no line in any battery refuses `Euler(SelfLoopEdge)` where the pre-pass ran.
   - No wrong crossing: `twov = 0` everywhere, and every built body is SOUND or reaches BAD only through a filed limitation.
   - The restart loop (`zip.rs:271`) is reached only by my `u2` battery. The B-side `vmap` branch (`zip.rs:263`) is reached by nothing I ran: the logs show every split on the A side.
   - The `ZipCorrespondence` refusal "did not separate its pair" (`zip.rs:276`) is typed, but I could not reach it (*unsure*). I found no collision that comes through a chain: every collision was a pair fused directly before (`split-log-summary.txt`).
   - `rest.rs`'s `zip_seam` callers: the extracted `section_cycle`, `align` and `start_of` are verbatim moves (inspection), and `rc_wide_battery` is line-identical (claim 5).
3. **Removing the ∩ rule is right: HOLDS (executed + geometry).**
   - Every two-lobe pose builds ∩ SOUND in both orders. That is r1's 340 per order on Ltop, Lbot, Lmirror, notch307 and shallow200, and every two-lobe pose of mine on the 8 turned L's, notch327 and Uin. The exception is 4 Uin poses that escalate in every op.
   - Why the walk is enough: the strut order is the angular order of the runs about the pierced face's normal, which is geometry of the operands and does not depend on the op. The op chooses only which side is kept, downstream.
   - The ∩ rule existed to dodge the zip's second fusion, which the pre-pass now removes. Restoring it (`interstart`) turns 2 rows red.
   - I found no pose where ∩ needs the old rule.
4. **`PinchUncrossed` is a correct refusal: HOLDS that it is typed and never wrong. A cheap body: *unsure*.**
   - The `outercross` variant (an outer loop may cross) turns all 51 of my `PinchUncrossed` lines into `ResultInvalid`: 44 `RingMeetsOuter` and 7 `LoopRoleInverted`. That reproduces the filed row's measurement on new corners.
   - I built no cheaper body. The row's own alternative, a bow-tie outer loop, is unmeasured. Its claim that the right body needs a shell divided at a vertex is inspection only (S6).
5. **Nothing else moved: HOLDS (executed, base `8793177b` vs head).**
   - `rc_wide_battery`, as 84 one-pair shards: 36 952 lines byte-identical. Shards 4, 42, 48, 52, 54, 60, 71 and 75 panic in `orbit_step_at` on both trees.
   - JOIN-1 `join1_r1_reflex_battery`: 971 lines identical, then the same panic on both trees.
   - `pierce_runs_battery` to the panic: 594 lines, 17 refusal→SOUND, the other 577 identical.
   - r1's battery: 516 refusal→SOUND, exactly the PR's count, and 0 other moves.
6. **The mutants are real: HOLDS (executed, `mutants.txt`).** The clean head is 11/11 green. The PR's five I re-ran reproduce exactly:

   | mutant | red rows |
   |---|---|
   | `nocross` | 5 |
   | `notwo` | 3 |
   | `noone` | 1 |
   | `interstart` | 2 |
   | `outercross` | 1 |

   Mine: `anysurf` turns 4 red. `single` (one crossing per op), `noB` and `lastsite` survive every row; see m1.

## Findings

**m1 (MINOR, test-gap, executed). No row holds two pinches in one op, and the B-side branch has no witness.**
- The `single` mutant (return after the first crossing) survives all 11 rows. On `u2` it turns 137 SOUND bodies into refusals (`single-u2.txt.gz`).
- The PR's batteries never reach a second pass: on r1's 15 552 lines, every split is on pass 1.
- `zip.rs:263-270` (a new vertex on the B side gains every A key's correspondence) is reached by no battery. The `noB` mutant is green everywhere.
- Fix: pin a staircase pose (`u2 S_tt`) as a row. Give the B-side branch a row, or record why it is unreachable.

**m2 (MINOR, executed). Near-tangent poses on new corners move refusal→BAD, all escalated.**
- 5 lines at d=±1e-7: Ltop_r-45 ∩/∖, Lbot_r-45 ∩/∖, Ltop_r30 ∖ (`nt-t3p.txt.gz`).
- Every one is `CensusEscalated`. Margins are 1.06–3.95e-9, with predicate `pm_census_ee_gap` as well as the `ee_span` the PR's evidence lists.
- They belong as evidence on the P0 `near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`. Base ships 139 such lines in this set.

**m3 (MINOR, executed). On a curved pierced face, 24 ∩ lines move refusal→not-a-legal-operand.**
- Each was `JoinDesync` ("derived ring role order") on base. On the head the volume is exact to 1e-9, and t2, t3′, t3, the certificate and `twov` all pass. The union with a far brick refuses `Containment(VolumeUncertified)`.
- That is the open row `at-infinity-probe-measures-in-closed-form-only`. Base already ships 294 such ∩ lines.
- The PR's claim of 0 refusal→BAD is planar-only. These poses belong on that row.
- 61 cylinder ∖ lines move from `Euler(SelfLoopEdge)` to `ResultInvalid(RingOnCurvedFace)`, the filed `an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane`.

**N1 (NOTE, executed): correction to the PR body and to the brief's baseline.**
- At the frozen head, main's `orbit_step_at` panic also aborts `join1_r1_reflex_battery` at line 971 and 8 of 84 `rc_wide` shards.
- So the PR's "line-identical" table (1 152 and 40 320 lines) cannot have been taken on this head; it predates the merge.
- The CLEAVE row `a-reflex-corner-on-a-cube-edge-panics-…` names only `pierce_runs_battery`.

**N2 (NOTE).** One declared pose's refusal moves through `boolean_op_recut`. The move is disclosed and is refusal→refusal; it is hold-adjacent, not a violation.

**N3 (NOTE).** The rows `the-intersection-ring-facing-is-measured-not-derived` and `…notch-or-shallow…every-chord-arc` stay open, although their claims are now false. The orchestrator owes them closure.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 on `zip.rs` whole)

- **S1 (Q1, likely).** The zips' fusion order is written twice: `zip.rs:237` (`once(0).chain((1..n).rev())`) and `zip.rs:450-458` (pair 0, then `for j in (1..n).rev()`). Pairs and alignment were factored out, but the order was not, so the two are kept in step by hand. A drift would change which corners move. Union-find still finds every cycle, so it is not a soundness loss (`lastsite` is insensitive).
- **S2 (Q1, likely).** There are two homes for "a face crosses a pinch":
  - `split_across`: `mev`, then `kemr` or `kef`, giving "two holes meeting at the point";
  - `finish.rs` `pinch_site` / `Joint::Hole` (`finish.rs:710`, `:755`): `mef` + `kev`, then `kfmrh`, giving "two holes meeting at the vertex".

  The second crosses an outer loop by dividing the face; the first rules outer loops out. Whether the two must agree is *unsure*.
- **S3 (Q2, unsure).** `TwoFaces` treats "one surface" as `SurfaceKey` identity (`zip.rs:348`). It never compares `Face::sense`, and `kef` checks neither. That is safe only if no body holds one surface with both senses, and nothing states or enforces that.
- **S4 (Q5, likely).** The module doc of `zip.rs` (lines 1–28) describes only the zip. The new pre-pass, the module's largest item, with its own surgery vocabulary, is absent from it.
- **S5 (Q3, sure).** `a_pinch_no_kept_face_can_cross_refuses_typed` pins one pose and one op. m1's gap means no row goes red if crossing stops after the first pinch.
- **S6 (Q6/Q4, unsure).** The filed row says the right body is "two vertices, no face meeting both". In its cube-edge family, each cube face passes `v` twice, round the notch, so a two-vertex body may need a face to meet both vertices. If so, that shape is not realizable as written.
- **S7 (Q7, unsure).** After each crossing, `cross_pinches` re-runs the whole simulation, re-aligning every seam (`continue 'pass`, bounded by pairs + 1). That is quadratic and hard to reason about. A worklist would read better to me.
- **S8 (dispatch premise, sure).** The brief's baseline `81dde823` is 77 main commits behind the head's base. Every count here is against `8793177b`. The r1 battery reproduces the PR's 516 exactly against it.

No glimpse of the other lane: I fetched only `join/pierce-two-out-runs-*` and my own branch.

REVIEW COMPLETE
