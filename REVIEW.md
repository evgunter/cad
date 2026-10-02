# JOIN-3 review (lane r2): PR #3895 at frozen head `3191f2ee`

**Verdict: APPROVE-WITH-FIXES.** Nothing builds wrong. Across about 75,000 poses, head against main:
- no BAD body;
- no SOUND→refusal;
- no new non-operand that main does not already produce for the same kind of body.

The ring lane closes with the minted curve, and a mutation of my own reddens the blind D row.
Two fixes are needed: one population of new poses now stops at an unfiled internal-invariant
payload (MINOR 1), and a citation names a removed symbol (MINOR 2).

**Isolation.** I did not fetch or read `join/3-segment-curve-review-r1`, any other review's output,
or the PR's comments. I read the PR body through the REST API.
- The PR's live head is now `0fb72965`; I reviewed `3191f2ee` only.
- "Main" is `origin/main` at `b4a1667a`, which carries JOIN-1. It is NOT the PR's base `0b6e39ca`:
  main has 58 more first-parent commits, so some refusal→refusal rows differ from the PR's table (NOTE 3).

**Batteries.** They are in `crates/sweep/tests/join3_r2_probes.rs` and `join3_r2_r1copy.rs` on this
branch, all `#[ignore]`d.
- `join3_r2_r1copy.rs` is `join1_r1_probes` verbatim, except its `outcome`: it adds the certificate
  to SOUND, and adds a legal-operand column (a disjoint union with a far brick, which is what
  `assert_legal_operand` checks).
- Release build, `Tol::witness()`.

## Claims

**1. Every consumer reads the one curve: holds.** Checked by inspection and by execution.
- `ChordJoiner::join` takes `Chords`. On `Segment`, all three chords go through
  `spec_of` → `running_from` (`chord_join.rs:2779-2792`): the first `mef`, the `mekr` and the second
  `mef`. Only `Chords::Split` reaches `chord_spec`.
- `ring_run_ccw` reads `curve.run_closing(h1)` (`boolean/join.rs:1713`).

My sweep keyed on data, not names. In `topo/src` outside tests it covered every
`mef(`/`mekr(` that takes a spec, every `axis: -axis` reversal, and every `EdgeCurveSpec`
construction:
- `rest.rs`'s twin spec (~1265-1343) is the REST zip's, which is JOIN-2's ground.
- `select_arc` and `along_edge_spec` are producers that only `segment_curve` calls.
- `boxes.rs:3235` is the torus lane.

No consumer on a boolean match rebuilds the chord after `segment_curve`. The `mekr` target and its
`from` (`chord_join.rs:2858-2870`) agree with `segment_curve`'s run (`2719-2732`). The second
chord's `from = h2` reverses the curve.

**2. `RunClosing`'s sign: holds, executed.**
- `j3r2_pocket_battery` uses 15 profiles that mix lines and arcs:
  - D profiles with major, half and minor arcs, down to a sliver of sagitta `1e-7`;
  - a stadium and a tombstone, both with bulge-0.6 arcs so that no joint is tangent;
  - a lens, and a rectangle with a concave arc notch.
- Each profile is posed in 6 turns, 4 offsets and 5 entries (top, bottom, through, and flush with
  either cap), under the 6 ops.
- `j3r2_tilted_battery` puts the same profiles on sketch planes tilted 0.15 and 0.3 rad, so the
  block's caps cut ELLIPSE arcs.

Over 12,960 poses there is no BAD body:
- every body that builds holds tiers 2 and 3′ and the certificate;
- every one matches its closed form and inclusion–exclusion;
- on the tilted through poses, the oblique closed form `S/cos θ` holds on all 1071.

The near-degenerate tilted slivers (D−0.4999999) escalate, typed `SectionLoopUndecided` on 18
poses, rather than decide. The `forward = true` arm is never reached (S4).

**3. No body builds wrong: holds. The legal-operand clause holds only with a scope.**
Transitions, main → head. The BAD and NONOP columns read main/head.

| battery | poses | refusal→SOUND | refusal→refusal | SOUND→refusal | BAD | NONOP |
|---|---|---|---|---|---|---|
| R1 `j3r2_r1_battery` (all shapes) | 42336 | 603 rod | 603 `SectionArcWindow`→`VolumeUnmeasured` | 0 | 0/0 | 0/0 |
| R1 seam (rcyl, ball) | 12150 | 531 | 531 →`VolumeUnmeasured`; 1920 ball →`SectionNotPolar` | 0 | 1/1 (same pose) | 0/0 |
| R2 rand, NEW seeds 31337/4242/777 ×400 ×6 ops | 7200 | 0 | 0 | 0 | 0/0 | 0/0 |
| pocket (new) | 10800 | 2292 | 312 →`VolumeUnmeasured`; **156 →`SectionInvariant` (MINOR 1)** | 0 | 0/0 | 0/0 |
| tilted (new) | 2160 | 1559 | 18 →`SectionLoopUndecided`; 7 →`VolumeUnmeasured` | 0 | 0/0 | 576/2135 |
| groove: a rod across a block (new) | 108 | 54 | 54 →`VolumeUnmeasured` | 0 | 0/0 | 0/0 |
| spun snowman: 4 radii × 8 spins (new) | 192 | 192 | 0 | 0 | 0/0 | 0/0 |
| `reach_wall` c=0.9 deep (new) | 18 | 9 SOUND on head (main not run) | — | — | 0 | 0 |

R2's battery is identical on new seeds. That is expected: it is all plane×plane, so every curve is
straight.

The tilted non-operands are `Containment(VolumeUncertified)`, plus 15 `ShellWitnessExhausted`.
- They are not this PR's: on main, the 576 tilted poses that already build (stadium, lens,
  tombstone, notch) are non-operands with the same payload.
- On main, a tilted ROUND pocket builds non-operand too (`j3r2_tilted_rod_operand`).
- The tilted cutters themselves are legal operands, 89 of 90 (`j3r2_tilted_operands`).
- This is the open `work/contact/at-infinity-probe-measures-in-closed-form-only`, now reached by
  1559 more poses (NOTE 1).

**4. The re-pinned rows moved for the right reason: holds, executed.** These rows all pass on head:
`axis_lap`, `snowman`, `reach_wall_chord_rows`, `verbs_germarms*`, `tang_circle_cylinder`,
`review_fillet_h7_r1_probes`, `join1_mechanisms`, `mate2*`, `curved_mergedoor*`.

Each new refusal sits further on than the old one:
- `VolumeUnmeasured` is the backstop, after a completed join;
- `SectionNotPolar` is the planar side's polar gate;
- `CurvedBooleanUnsupported{A, Cylinder}` is the wall pair the old ring door masked.

Main→head, I found no SOUND→refusal in any battery. The two widenings of re-pinned rows are both
clean:
- **`reach_wall`, c = 0.9.** `j3r2_reach_wall_c09` adds tiers 2 and 3′ and the operand column.
  All 9 builds are SOUND and legal.
- **Spun snowman.** `j3r2_snowman_battery` runs 192 poses in both orders. All are SOUND and legal.

**5. The blind D rows can go red: holds, executed.** My mutation, M2, inverts `run_closing`'s
flag: `h1 != self.halves.0` becomes `==` (`chord_join.rs:1864`), so the closing conic is wound
backwards.
- `axis_lap::a_blind_d_pocket_builds_from_either_face` goes red with
  `JoinDesync { "minted-edge description failed certification" }`.
- `Straight` reddens the row with a different payload, so the row catches a mis-sensed curve as
  well as a missing one.
- Only that sweep row catches M2. The unit row builds `RunClosing` by hand (S3).

## Findings

**MINOR 1: 156 new pocket poses stop at an internal-invariant payload no work item owns.**
Executed. The pose is the block minus a D prism whose wall crosses the block's side face, e.g. D0.3
turned 4.0 rad, offset (0.8, 0.1), top entry.
- Main refuses `JoinDesync "ring-run winding is degenerate"` (132 poses) or `NoChartedRun` (24).
- Head refuses `Join(SectionInvariant { "ring re-homing reads the divided face's plane; this face's
  carrier is not a plane (arm not wired)" })` (`chord_join.rs:3082-3088`).

This is refusal→refusal, so it is not a regression. It is still the carried issue's own class: an
ordinary pocket that refuses in join-internal words, now on a curved face's ring re-homing.
`work/walks/three-answers…` mentions `face_plane_normal` only as a duplicate, not as a frontier.
File it, with these poses.

**MINOR 2: a citation names a removed symbol.** By inspection.
- `crates/sweep/tests/join1_mechanisms.rs:11` names the structural-skip mechanism
  "(`SegmentEdge::Is`)", which this PR removes.
- The closed `work/hone/planar-side-join-…md:19-23` says `between_edge_is_section`, "asked on a
  boolean lane, refuses `SectionInvariant`". That arm is gone too: the function now takes
  `&SectionCtx`, so the question cannot be asked.

**NOTE 1.** The tilted non-operands belong to the open `at-infinity-probe…` issue; adding them
there as evidence is cheap.

**NOTE 2.** `reach_wall_chord_rows::check` reads neither tier 2 nor tier 3′ nor the operand test on
the poses this PR newly builds. My probe passes them.

**NOTE 3.** The PR's table is against base `0b6e39ca`. Against main, the ball seam battery also
moves 876 `SectionArcSide` and 45 `SectionInvariant` poses to `SectionNotPolar`, all
refusal→refusal.

## Style

**Exercised:** Q1 (the prose grep plus a data grep for `axis: -axis`), Q2, Q3 (mutation M2), Q4 (a
grep for removed symbols across `crates`, `docs` and `work`), Q5, Q6, Q7.

**Q8, partially:** I read `loop_winding.rs`'s module docs and every hunk in full. I did not read
`chord_join.rs` (3926 lines) end to end, only the regions this PR touches.

- **S1 (Q1, sure).** `SegmentCurve::running_from` (`chord_join.rs:1872-1952`) mints another spelling
  of the reversal "θ ↦ −θ about the flipped axis". The other spellings:
  - `along_edge_spec` (`2626-2634`);
  - `select_arc`'s clockwise arm (`1303-1330`);
  - `rest.rs:1281`'s twin spec;
  - a variant at `boxes.rs:3233`.

  The PR closes "two answers to one question" for the curve and adds a fifth copy of the reversal.
  There is no "reversed" helper on `Curve3` or on `EdgeCurveSpec`.
- **S2 (Q1/Q2, likely).** `segment_curve`'s run choice (`chord_join.rs:2719-2732`) restates by hand
  which run `join`'s first chord co-bounds: the prev-adjacent skip and the `mekr` target. Its doc
  asserts the agreement, and nothing enforces it. `if l2 == outer { next2 } else { h1 }` is written
  twice, at `2730` and `2858`.
- **S3 (Q1/Q3, likely).** "The curve run back" has two encodings:
  - the winding's flag, `((carrier, params), forward)`, from `run_closing`;
  - the mint's flipped carrier with negated params, from `running_from`.

  Only one sweep row ties the two together (M2). The unit row
  `a_run_on_arcs_is_decided_by_its_bulge` (`merge_faces.rs:5363-5391`) never calls `run_closing`.
- **S4 (Q3, sure).** `run_closing`'s `true` arm is dead in production.
  `RoleLane::curve_order` returns `(ea, ra)` on the ring lane, and `resolve` winds `(ea, ra)`
  (`boolean/join.rs:1636-1690`), so `h1 == halves.0` always holds. "Right sign in every role order"
  is never exercised; the other order is inferred from antisymmetry.
- **S5 (Q1, likely).** "Halves either side of the segment's edge" is spelled three ways:
  - `choose_roles`' `follows_across_segment` (`boolean/join.rs:1594-1610`), added so the new
    closing does not wind a zero-area sliver;
  - `segment_curve`'s `prev1` test (`chord_join.rs:2723`);
  - the joiner's `prev_adjacent` + `skip_adjacent_chord` (`2807-2820`).
- **S6 (Q2, unsure).** The `RoleLane::resolve` doc (`boolean/join.rs:1640-1655`) carries the PR 5.5 /
  A×Z counter-island history unchanged into its new home. It is longer than the code it defends.
- **S7 (Q7, unsure).** In `segment_curve` (`chord_join.rs:2720-2733`), `let (u1, u2) = (u1, …)`
  rebinds `u1` to itself, and a closure named `between` returns an edge key.
- **S8 (Q5, likely).** The `ArcWindowCase::NoChartedRun` doc (`chord_join.rs:104-118`) now says a
  ring lands there "only when that run is scaffolding alone". The plausible claim is unpinned: the
  off-axis `axis_lap` laps still land there, and no row states what their run held.

REVIEW COMPLETE
