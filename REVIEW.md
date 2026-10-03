# JOIN-3 delta review: PR #3895, fix pass 3191f2ee → 2cc1455c

**Verdict: APPROVE-WITH-FIXES.** Nothing builds wrong. There is no SOUND→refusal against current main, and every non-operand is `Containment(VolumeUncertified)`. One MINOR: the re-pinned unit row that claims to test both closing branches cannot see a missing reversal.

**Trees.** All builds are release, with my own target dirs.
- **main** is `origin/main` b624b887.
- **head** is the live PR head 317a2626 (2cc1455c + a filing commit + a main merge), merged locally with b624b887. Its code delta over 2cc1455c is main's alone.
- The batteries come from the reviewers' files as folded into the PR. They were copied verbatim onto main. R2's pose lists were moved to new values (`review-delta/r2-new-poses.patch`). Runner and grader: `review-delta/`, plus R1's `review-r1/grade.py` from its branch.

## Claims
1. **Unified reversal: holds (executed).**
   - `crates/geom/tests/j3_delta_reversal.rs` (3 rows, green) covers line, circle, ellipse and spiric:
     - `reversed().reversed()` is the identity, bit for bit;
     - `b(−t1) = c(t1)` and `b(−t0) = c(t0)` on three windows, one of them nearly 2π;
     - the tangent flips;
     - a reversed spiric passes `Curve3::spiric`'s door.
   - The four `chord_join` sites read `Curve3::reversed`: `oriented_arc`, the tangent ruling, `running_from` and `along_edge_spec`. On circle, ellipse and line the substitution is literal, so nothing changes.
   - One intended change: `running_from` on a spiric used to refuse and now reverses it. No lane mints a spiric segment.
   - Copies outside `chord_join` are under Style.
2. **Shared run choice: holds (by inspection).**
   - `join` and `segment_curve` both call `first_chord` (and `second_chord` when the first is skipped) on the same pre-surgery body.
   - The two skip tests agree: `skip_adjacent_chord`'s `Segment` arm and `segment_skip` both ask `edge == Some(he.edge)`, with the same `seg_a`/`seg_b`.
   - Each lane matches:
     - prev-adjacent: the second plan, run `[d1.prev]` or `[d1.next]`;
     - `mekr`: the target's cycle, from h2 when l2 is the face's outer, else from h1;
     - outer: the `clean_dir` order is passed to both.
   - On the 4-cycle `[h1, x, h2, y]` with `y` as the segment, old `segment_curve` read run `[y]` while `join` minted against `[x]`. The plans now agree on `[x]`, a silent fix.
   - **Caveat (ring lane).** The curve is computed in the given order. `resolve` may then flip it, so `join`'s first chord can co-bound a different run than the one `segment_curve` read. This is harmless only because `chord_spec` on a `Surface::Plane` face never reads `run` (`chord_join.rs:1811-1837`). Deviation (a) says so.
3. **`Decided` arm: holds. No curved pose reaches it.**
   - I instrumented the head (stderr probes only).
   - Across the whole sweep suite (1990 pass), the arm fires 135 times, all in `join1_mechanisms`, `mate2_cyl_rest` and 7 `curved_mergedoor` rows.
   - Across about 70k battery poses (all R1-copy, both R1 batteries, R2 pocket, tilted, groove and snowman) it fires 0 times.
   - Its desync arm fires 0 times.
   - A curved ring face meeting the across-segment condition fires 0 times. That probe ran the sites check before the planar refusal, so on a curved face the arm is unreachable: such a face refuses `ring-lane face has no planar carrier`, which is main's behaviour.
   - **M2** (the arm disabled) turns exactly the 4 rows the PR names red. The other 4 rows that reach the arm stay green, so for them the order is moot.
4. **No wrong body, no SOUND→refusal, non-operands at infinity only: holds.** Main → head on new seeds and poses; columns read main/head.

| battery (new range) | poses | refusal→SOUND | refusal→other | SOUND→refusal | BAD | non-op |
|---|---|---|---|---|---|---|
| R1 `j3r1_mixed_pockets`, seed 7171 ×100 | 5400 | 334 (280 `JoinDesync`, 54 `SeamOrientation`) | 0 | 0 | 0/0 | 0/0 |
| R1 `j3r1_mixed_pockets`, seed 31415 ×100 | 5400 | 234 | 0 | 0 | 0/0 | 0/0 |
| R1 `j3r1_tilted_through`, seed 7171 | 552 | 0 (50 → non-op) | 2 `JoinDesync`→`VolumeUnmeasured` | 0 | 0/0 | 491/541 |
| R2 pocket: φ∈{.35,1.2,2.8,5.1}, 3 new offsets | 5400 | 1005 | 153 →`VolumeUnmeasured`; 144 →`SectionInvariant` (ring re-homing, filed in item 7) | 0 | 0/0 | 0/0 |
| R2 tilted: θ∈{.08,.22,.38}, φ∈{.9,2.6,5.0} | 3240 | 0 (2293 → non-op) | 72 →`Pcurves{Escalated TrimContainment}` (D−0.4999999); 13 →`VolumeUnmeasured` | 0 | 0/0 | 849/3142 |
| R2 `j3r2_rand`, seed 5150 ×400 | 2400 | identical | — | 0 | 0/0 | 0/0 |

   - **Non-operand payloads.** Every head non-operand is `Containment(VolumeUncertified)`: 541 in R1 tilted, and 3142 in R2 tilted after re-running it with the full payload printed. R1's main column shows the same kind. No inclusion–exclusion or oracle mismatch line appears.
5. **New and re-pinned rows can go red: partly falsified.**
   - **M1** makes `running_from` never reverse (`at == halves.1` also returns the spec as computed). It turns these red:
     - `axis_lap::a_blind_d_pocket_builds_from_either_face`;
     - two of the three `reach_wall_chord_rows`, with `Certification{ResidualExceeded EndpointStart}`.
   - Under M1, `merge_faces::winding_arm_tests::a_run_on_arcs_is_decided_by_its_bulge` **stays green** (MINOR 1).
   - `reach_wall`'s M1 failure is at its older ∪ `RimUnmeasured` arm, so no mutation of mine showed the new `Want::Sound` arm going red.
   - M2: see claim 3.

## Findings
- **MINOR 1: the re-pinned winding unit row cannot see a missing reversal.**
  - Shown by M1, executed above.
  - The PR body (item 4) says the row calls `run_closing` "on both branches: the curve as computed, and run back".
  - The row picks `halves` from the edge's `plus` claim, so both of its calls reach `running_from`'s `at == halves.0` arm. `Curve3::reversed` is never exercised there.
  - Only the sweep rows guard the reversal branch.
  - `crates/topo/src/merge_faces.rs:5358-5417`.
- **NOTE 1: new refusal payload on the tilted near-sliver.** At θ=0.08, D−0.4999999 now refuses `Pcurves{Certify Escalated TrimContainment}`. It refused `JoinDesync` before. That is a typed escalation on a sagitta of about 1e-7, refusal→refusal; R2 saw `SectionLoopUndecided` at θ ≥ 0.15.
- **NOTE 2: `reach_wall`'s strengthened `Want::Sound`.** It adds tier 3′ and the operand check. Neither of my mutations shows it going red on its own (unsure).
- **NOTE 3: the earlier `Decided` arm built some curved ring poses.** On 3191f2ee the arm came before the planar refusal, so curved ring faces were let through. I found no pose of that shape in any battery, so I could not measure what moved. Main never had the arm, so this is no regression against main.

## Style
Questions exercised: Q1 (prose sweep, plus a data sweep for `axis: -`/`dir: -`/`u_ref: -`), Q2, Q3 (mutations), Q4, Q5 (the module doc), Q7. **Q8 was not done:** `chord_join.rs` is about 3400 lines; I read the changed regions and `chord_spec`'s dispatch.
- **sure: two spellings of the boolean adjacency skip.** `segment_skip` (`chord_join.rs:3233`) restates `skip_adjacent_chord`'s `Chords::Segment` arm (`:2384-2389`). `join` reads one; `segment_curve` and `segment_chord_sites` read the other. The "one run choice" holds only while they agree, which is a fresh instance of the class this fix closes.
- **sure: an undisclosed reversal copy, unfiled.** `sweep/src/blend/surgery.rs:2950-2982` (`scaled`) flips the axis with `(−t1, −t0)` by hand. The PR's sweep names only `rest.rs` and `boxes.rs`, so the "one home" claim misses a third copy.
- **likely: a second reversal spelling inside `running_from`.** Its scaffold-line arm (`chord_join.rs:2333-2336`) builds the reversal as `line_between(ends.1, ends.0)` and discards the `reversed()` carrier it just computed.
- **likely: a silent fallback in `oriented_arc`.** `reversed().unwrap_or_else(|| carrier.clone())` (`chord_join.rs:1415-1418`) returns the *unreversed* carrier if the conic is not reversible. The comment asserts the invariant ("a section conic is a circle or an ellipse"); the code does not enforce it. That is not fail-loud.
- **sure (minor): dead error arm in the tangent ruling.** `line.reversed().ok_or(…"carried a non-line")` (`chord_join.rs:1914`) re-checks what the let-else at `:1890` has already proved.
- **likely: stale module doc.** `boolean/join.rs:34-39` says the ring lane "computes [the curve] first and orders the halves by it". The across-segment `Decided` arm orders a ring-face match before the curve; the doc omits it.
- **unsure: one premise, two checks.** The ring lane's "the run is unread" premise rests on `chord_spec`'s `Surface::Plane` test. `choose_roles` gates on `face_outward_normal(..).is_some()`. Nothing ties the two together.
- **unsure: an untested desync arm.** `choose_roles`' arm "two role orders … mint different chords" is unreachable on any loop longer than 4 by the plan algebra, and was never hit (0 of about 70k poses). It is a guard with no row.

REVIEW COMPLETE
