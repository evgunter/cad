# JOIN-3 review, lane R1 (PR #3895, frozen head 3191f2ee)

**Verdict: APPROVE-WITH-FIXES.** No wrong body anywhere. Both mutations redden the blind D row. The one
regression is real on the frozen head but disappears once main is merged in, so it is MINOR.

Lane isolation: I did not fetch or read `join/3-segment-curve-review-r2`, any other review's output,
or PR #3895's comments. I read the PR body through `pulls/3895` only. No glimpse to disclose.

## What I ran (release builds, own target dirs)

Five trees:

- **head**: 3191f2ee.
- **base**: 0b6e39ca, the merge base, JOIN-1's head; the PR's own "main".
- **main**: b4a1667a. It carries JOIN-1, plus 317 later commits.
- **merged**: head with main merged in locally, never pushed. The code merged clean; the only conflict is
  `work/tang/pierce-ring-has-no-join-arm.md`.
- **m1, m2**: head with my two mutations.

My batteries are in `crates/sweep/tests/join3_review_r1.rs` (ignored). The off-line grader is in
`review-r1/`. The batteries are shaped differently from the PR's: JOIN-1's R2 battery is all-planar,
so it never mints a conic or runs a closing conic.

- **`j3r1_mixed_pockets`**: random bulge profiles of 2–5 sides, with lines, convex arcs and concave arcs,
  against the block `[−1,1]²×[0,1]`. Poses: blind top, blind bottom, through, buried void, flush
  top/bottom/both, and the flush poses again with every flush contact declared. ∪∖∩ in both orders.
  Seeds 4242 and 99001, 150 profiles each, 16 500 poses.
- **`j3r1_tilted_through`**: the same profiles tilted 0.05–0.45 rad about a random horizontal axis, through
  the block. Ellipse arcs plus lines sit on both caps. 1 680 poses.
- **`j3r1_d_family`**: the D rod at flat ∈ {±0.4999, ±0.49, ±0.45, ±0.3, ±0.1, 0, 0.499}. That runs from
  the near-tangent sliver to the near-full disc. Every pose above, plus tilts 0.2 and 0.6. 864 poses.

Each pose prints t2, t3′, the certificate, `v` against the closed form, and the operand gate
(`assert_legal_operand`'s union with a far brick). The closed form reads the tool's own measured
volume. Arcs of bulge 1 can leave the block, so `review-r1/grade.py` re-grades those poses against the
area clipped to the block, using a 20 000-segment polygonization. Every pose it re-graded matches to
1e-9. No pose whose profile leaves the block was passed by the unclipped oracle.

R1 batteries run: `join1_r1_battery` (42 336 poses), seam, declared, reflex, tube and capsule.
`r2_random_zprism_pairs` ran on NEW seeds 31337, 777 and 2026, 400 cases × 3 ops each. The sweep suite
on head: 1944 passed, 0 failed.

## Claims

1. **Holds** (by inspection, plus my sweep). I grepped every non-test caller of `section_case`,
   `wall_section`, `select_arc`, `face_azimuth_window`, `bool_planar_chord_spec`, `chord_spec`,
   `along_edge_spec`, `conic_segment_term`, `planar_run_winding_decided`, `mef_chord` and `mekr_chord` in
   `topo/src`. That is the producers, where the PR's sweep took the consumers. On a boolean match, every
   hit is either upstream of `segment_curve` (the window reads at `boolean/join.rs:518,549`) or not a
   matched chord: `solid_contain` (containment), `rest.rs` (JOIN-2) and the split lane (filed). The
   `mef`, `mekr` and second `mef` read `Chords::Segment`, and `resolve` and `ring_run_ccw` read the same
   `SegmentCurve`.
   One undisclosed side effect: in the Cyl×Plane arm, A's wall window (`join.rs:549`) is now read before
   A's surgery. On base it was read after `sa.join_split` had divided that face. The Plane×Cyl arm always
   read it before, so the two arms now agree. No battery transition traced to it. NOTE.
2. **Holds, executed; nothing falsified.** I looked for a wrong sign or a premature decision in 19 044
   poses: lines mixed with convex and concave arcs, ellipse arcs, and the D at flat −0.4999, whose
   sliver's mean width is about 7e-5. **Zero BAD in every tree.** Every wrong order the mutants forced
   fails loud: `JoinDesync`, or a minted edge refused at certification. None builds wrong. I found no
   decision that should have escalated.
3. **Holds for correctness, falsified in letter for "a legal operand".** Head has no BAD in R1, R2 or my
   batteries. Every new build passes t2, t3′, the certificate and the closed form. But 354 new builds,
   main→merged (144 tilted D and 210 random tilted), are not legal operands:
   `Containment(VolumeUncertified)`. That is the filed
   `work/contact/at-infinity-probe-measures-in-closed-form-only`. Main fails the gate the same way on
   1 441 of its own 1 465 tilted builds. MINOR.
4. **Holds against main; falsified against the PR's own base.** **MINOR.**
   - **Head vs base: SOUND→refusal on 85 poses**, all of them declared-flush ∩ (`c0 flushtop DECL I BT`
     and the like). The refusal is `ResultVolumeImplausible { "vol(A ∩ B) ≤ vol(B)", got:
     0.0659642504798124, bound: 0.06596425047981229 }`, an excess of 1 ulp. The body is the tool itself,
     so its bits moved and base's exact-band backstop (`ops.rs:1668`) trips. 11 poses go the other way,
     and base already trips it on 191 poses.
   - **Head vs main** shows 265 such refusals, because main's REACH rework of the backstop (`a1b00394`)
     trips on none.
   - **Merged vs main: 0 SOUND→refusal and 0 BAD** across R1, seam, R2 and all of my batteries.

   The PR's batteries (all-planar R2; R1 with no conic flush) could not see it, and the PR table
   reports none. A body-bits change that trips an exact backstop is a hazard, not something to hand-wave.
   The fix is to state it in the PR body and merge main before merging the PR.
   **The re-pinned rows:** groove, snowman, `reach_wall_chord_rows`, `verbs_germarms*` and `axis_lap`
   each move refusal→build or refusal→refusal. The new refusals are the notched-wall
   `VolumeUnmeasured`, `SectionNotPolar` and `CurvedBooleanUnsupported`; none of the moved rows built
   before.
   **The PR's seam row is incomplete.** Base→head, ball: besides the 999 `NoChartedRun`→`SectionNotPolar`
   there are 876 `SectionArcSide{NoCertifiedRun}`→`SectionNotPolar` and 45
   `SectionInvariant{"a run edge has no closed-form chart image"}`→`SectionNotPolar`. Main→merged gives
   759, 459, 240 (`SectionArcWindow`→`SectionArcSide`) and 45. These are not obviously "further on": the
   curve is now computed for both solids before either joins, so the B side's polar gate fires first.
   It is the same frontier reached in a different order. MINOR, a report gap.
   Tilted `t96 … I TB` built on main (not a legal operand) and refuses `VolumeUnmeasured` on head and on
   merged. That is 1 pose. NOTE.
5. **Holds, executed.**
   - **M1** flips `run_closing`'s direction (`chord_join.rs:1864`, `!=` → `==`). The blind D row goes red
     with `JoinDesync{"minted-edge description failed certification"}`. In `j3r1_d_family`, 72 of 288
     upright builds and 72 of 144 tilted builds refuse.
   - **M2** drops the closing conic from the bulge sum (`loop_winding.rs:431`). The row goes red with
     `JoinDesync{"ring-run winding is degenerate (zero enclosed area)"}`, and 144 of 288 upright and all 144 tilted D builds refuse.

## Findings

- **MINOR (executed): a SOUND→refusal regression on the frozen head**, against its own base (claim 4).
  It is masked once main is merged. The PR body should say so, and the PR merges only with main in.
- **MINOR (executed): the PR's seam row under-reports** 921 base→head transitions (claim 4).
- **MINOR (executed): new builds that are not legal operands** (claim 3). Add these consumers to
  `at-infinity-probe-measures-in-closed-form-only`, as was done for the notched wall.
- **MINOR (inspection): the across-segment `Decided` arm** (`boolean/join.rs:1588-1610`) skips both
  `clean_dir`'s loose-pair separation check, which every other same-loop order passes, and the
  ring lane's "no planar carrier" refusal. A curved ring face now proceeds on this arm. "The order is
  moot" is argued only in prose. I found no pose that breaks it.
- **NOTE**: the brief calls `join1_r2_rand` "ignored". `r2_random_zprism_pairs` is not `#[ignore]`d. It
  runs in CI at its default seed 12345 and N 150.
- **NOTE**: the merge with main will conflict in `work/tang/pierce-ring-has-no-join-arm.md`.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6 and Q7. For Q8 I read `loop_winding.rs` end to end;
`boolean/join.rs` (3052 lines) and `chord_join.rs` (3926 lines) only in part.

- **Q1, sure** (`chord_join.rs:1892-1917`): `SegmentCurve::running_from` is a fifth hand spelling of
  "θ ↦ −θ about the flipped axis". The others: `chord_join.rs:1305-1330` (`select_arc`'s cw arc),
  `:2627` (`along_edge_spec`), `boolean/rest.rs:1283` and `boolean/boxes.rs:3235`. A structural fix
  minted a fresh copy. Sweep every `axis: -axis` / `dir: -dir` site.
- **Q1/Q2, likely** (`chord_join.rs:2705-2730`): `segment_curve` re-derives by hand which run `join`'s
  first chord co-bounds: the prev-adjacent belly, the cross-loop `mekr` target and the outer test. Its
  doc says it is "the one `Self::join`'s first minted chord co-bounds", and nothing ties the two
  spellings together. If `join`'s run choice changes, the curve silently comes from another window.
- **Q7, likely** (`boolean/join.rs:491-493, 619-620`): the halves `(ea, ra)` are handed separately to
  `choose_roles`, to `curve_order` and to `resolve`, kept in step by convention. `RoleLane` holds the face
  and normal but not the pair it decides.
- **Q3/Q7, likely** (`chord_join.rs:1859-1866`): the `h1 == halves.1` case of `run_closing` is dead in
  production. `ring_run_ccw` is only ever asked about `(ea, ra)`, the order the curve was computed in
  (`join.rs:1667`). No row reaches the forward arm through the join.
- **Q4, sure** (`sweep/tests/join1_mechanisms.rs:11`): it still cites `SegmentEdge::Is`, which is gone.
  The doc rotted; the code is right.
- **Q5, sure** (`boolean/join.rs:34-37`): "computed once per solid, before its role order is chosen" is
  false on the mekr and outer lanes, where the order is decided first (deviation (a)).
- **Q6/Q4, unsure** (`chord_join.rs:2734-2737` against the PR's own CLEAVE issue): the PR's split-lane
  issue warns that computing the curve before the adjacency skip mints an aux surface for a join whose
  chords are both skipped, an orphan that tier 1 refuses. The boolean lane now does exactly that:
  `split_curve` and `bool_planar_curve` mint before the skip. I found no boolean pose that reaches it,
  but nothing says why it cannot happen.
- **Q4, likely** (`boolean/join.rs:1605-1617`): the `Decided` across-segment arm runs before the planar
  check. That reorders a premise ring-lane readers relied on: a ring-lane face is planar.
  Same instance as the MINOR above.
- **Q2, unsure** (`boolean/join.rs:1588-1593`): the six-line prose argument that "the order is moot"
  is the only guard on a branch that bypasses two checks. The comment is doing the code's work.

REVIEW COMPLETE
