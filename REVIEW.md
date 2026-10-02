# JOIN-3 review (lane r2): PR #3895 at frozen head `3191f2ee`

**Verdict: APPROVE-WITH-FIXES.** Nothing builds wrong. Over about 75,000 poses, head against main, there is no BAD body and no SOUND→refusal. There is also no new non-operand of a kind main does not already produce. The ring lane closes with the minted curve, and a mutation of my own reddens the blind D row. Two fixes:
- one population of new poses stops at an unfiled internal-invariant payload (MINOR 1);
- a citation names a removed symbol (MINOR 2).

**Isolation.** I did not fetch or read `join/3-segment-curve-review-r1`, any other review's output, or the PR's comments. I read the PR body through the REST API.
- The PR's live head is now `0fb72965`. I reviewed `3191f2ee` only.
- "Main" is `origin/main` at `b4a1667a`, which carries JOIN-1. It is not the PR's base `0b6e39ca`: main has 58 more first-parent commits, so some refusal→refusal rows differ from the PR's table (NOTE 3).

**Batteries.** They are in `crates/sweep/tests/join3_r2_probes.rs` and `join3_r2_r1copy.rs` on this branch, all `#[ignore]`d. The r1 copy is `join1_r1_probes` verbatim except for `outcome`, which adds the certificate to SOUND and adds a legal-operand column: a disjoint union with a far brick, which is `assert_legal_operand`'s test. Release build, `Tol::witness()`.

## Claims

**1. Every consumer reads the one curve: holds.** Checked by inspection and by execution.
- On `Chords::Segment`, all three chords go through `spec_of` → `running_from` (`chord_join.rs:2779-2792`): the first `mef`, the `mekr` and the second `mef`.
- Only `Chords::Split` reaches `chord_spec`.
- `ring_run_ccw` reads `curve.run_closing(h1)` (`boolean/join.rs:1713`).

My own sweep keyed on data, not names: every `mef(`/`mekr(` that takes a spec, every `axis: -axis` reversal, and every `EdgeCurveSpec` construction in `topo/src` outside tests.
- `rest.rs`'s twin spec is the REST zip's, which is JOIN-2's.
- `select_arc` and `along_edge_spec` only produce `segment_curve`'s value.
- `boxes.rs:3235` is the torus lane.

No consumer on a boolean match rebuilds the chord. The `mekr` target and `from` (`chord_join.rs:2858-2870`) agree with `segment_curve`'s run (`2719-2732`).

**2. `RunClosing`'s sign: holds, executed.** `j3r2_pocket_battery` runs 15 profiles that mix lines and arcs:
- D profiles with major, half and minor arcs, down to a sliver of sagitta `1e-7`;
- a stadium and a tombstone, with bulge-0.6 arcs so no joint is tangent;
- a lens, and a rectangle with a concave arc notch.

Each profile is taken through 6 turns × 4 offsets × 5 entries: top, bottom, through, and flush with either cap. `j3r2_tilted_battery` tilts the sketch planes 0.15 or 0.3 rad, so that the block's caps cut ELLIPSE arcs.

Over 12,960 poses there is no BAD body. Every built body holds tiers 2 and 3′ and the certificate, and passes its closed form or inclusion–exclusion. The oblique form `S/cos θ` holds on all 1071 tilted through poses. The near-degenerate tilted slivers (D−0.4999999) escalate, typed `SectionLoopUndecided` (18 poses). One limit: the `forward = true` arm is never reached (S4).

**3. No body builds wrong: holds. Legal-operand: holds, with a scope.** Main → head; BAD and NONOP read main/head.

| battery | poses | refusal→SOUND | refusal→refusal | SOUND→refusal | BAD | NONOP |
|---|---|---|---|---|---|---|
| R1 `j3r2_r1_battery` (all shapes) | 42336 | 603 rod | 603 `SectionArcWindow`→`VolumeUnmeasured` | 0 | 0/0 | 0/0 |
| R1 seam (rcyl, ball) | 12150 | 531 | 531 →`VolumeUnmeasured`; 1920 ball →`SectionNotPolar` | 0 | 1/1 (same pose) | 0/0 |
| R2 rand, NEW seeds 31337/4242/777 ×400 ×6 ops | 7200 | 0 | 0 | 0 | 0/0 | 0/0 |
| pocket (new) | 10800 | 2292 | 312 →`VolumeUnmeasured`; **156 →`SectionInvariant` (MINOR 1)** | 0 | 0/0 | 0/0 |
| tilted (new) | 2160 | 1559 | 18 →`SectionLoopUndecided`; 7 →`VolumeUnmeasured` | 0 | 0/0 | 576/2135 |
| groove: rod across a block (new) | 108 | 54 | 54 →`VolumeUnmeasured` | 0 | 0/0 | 0/0 |
| spun snowman: 4 radii × 8 spins (new) | 192 | 192 | 0 | 0 | 0/0 | 0/0 |
| `reach_wall` c=0.9 deep (new; head only) | 18 | 9 SOUND | — | — | 0 | 0 |

**Reading the table.**
- R2 rand is identical because it is all plane×plane: every curve is straight.
- The tilted non-operands are `Containment(VolumeUncertified)`, plus 15 `ShellWitnessExhausted`. They predate this PR:
  - on main, the 576 tilted poses that already build are non-operands with the same payload;
  - so is main's tilted ROUND pocket (`j3r2_tilted_rod_operand`);
  - the tilted cutters themselves are legal operands, 89 of 90 (`j3r2_tilted_operands`).
- That class is the open `work/contact/at-infinity-probe-measures-in-closed-form-only`, now reached by 1559 more poses (NOTE 1).

**4. Re-pinned rows moved for the right reason: holds, executed.** These rows pass on head: `axis_lap`, `snowman`, `reach_wall_chord_rows`, `verbs_germarms*`, `tang_circle_cylinder`, `review_fillet_h7_r1_probes`, `join1_mechanisms`, `mate2*`, `curved_mergedoor*`.

Each new refusal is further on than the old one:
- `VolumeUnmeasured`: the backstop, after a completed join;
- `SectionNotPolar`: the planar side's polar gate;
- `CurvedBooleanUnsupported{A, Cylinder}`: the wall pair the old ring door masked.

No battery has a SOUND→refusal. I widened two re-pinned rows, and both are clean:
- `j3r2_reach_wall_c09`: tiers 2 and 3′ plus the operand column on c=0.9; 9/9 SOUND and legal.
- `j3r2_snowman_battery`: 192 poses in both orders; all SOUND and legal.

**5. The blind D rows can go red: holds, executed.** My mutation M2 inverts `run_closing`'s flag, `h1 != self.halves.0` → `==` (`chord_join.rs:1864`), so the closing conic is wound backwards. `axis_lap::a_blind_d_pocket_builds_from_either_face` goes red with `JoinDesync { "minted-edge description failed certification" }`.
- That payload is not `Straight`'s, so the row catches a mis-sensed curve as well as a missing one.
- Only that sweep row catches M2 (S3).

## Findings

**MINOR 1: 156 new pocket poses stop at an internal-invariant payload no work item owns.** Executed. Example: the block minus a D prism whose wall crosses the block's side face, e.g. D0.3 turned 4.0 rad, offset (0.8, 0.1), top entry.
- On main these poses refuse `JoinDesync "ring-run winding is degenerate"` (132) or `NoChartedRun` (24).
- On head they refuse `Join(SectionInvariant { "ring re-homing reads the divided face's plane; this face's carrier is not a plane (arm not wired)" })` (`chord_join.rs:3082-3088`).

Refusal→refusal, so not a regression. It is still the carried issue's class: an ordinary pocket that refuses in join-internal words, now at a curved face's ring re-homing. `work/walks/three-answers…` names `face_plane_normal` only as a copy. File it with these poses.

**MINOR 2: a citation names a removed symbol.** By inspection.
- `crates/sweep/tests/join1_mechanisms.rs:11` names the structural-skip mechanism "(`SegmentEdge::Is`)", which this PR removes.
- The closed `work/hone/planar-side-join-…md:19-23` says `between_edge_is_section` "asked on a boolean lane refuses `SectionInvariant`". That arm is gone: the function now takes `&SectionCtx`, so the question cannot be asked.

**NOTE 1.** The tilted non-operands belong to the open `at-infinity-probe…` issue; add these poses there as evidence.

**NOTE 2.** `reach_wall_chord_rows::check` reads neither tier 2 nor tier 3′ nor the operand test on the poses this PR newly builds. My probe passes them.

**NOTE 3.** The PR's table is against base `0b6e39ca`. Against main, the ball seam battery also moves 876 `SectionArcSide` and 45 `SectionInvariant` poses to `SectionNotPolar`, all refusal→refusal.

## Style

**Exercised:** Q1 (the prose grep and a data grep for `axis: -axis`), Q2, Q3 (mutation M2), Q4 (a grep for removed symbols across `crates`, `docs` and `work`), Q5, Q6, Q7.

**Q8, partly:** I read `loop_winding.rs`'s module docs and every hunk. I did not read all 3926 lines of `chord_join.rs`, only the touched regions.

- **S1 (Q1, sure).** `SegmentCurve::running_from` (`chord_join.rs:1872-1952`) is a new spelling of the reversal "θ ↦ −θ about the flipped axis". The earlier spellings:
  - `along_edge_spec` (`2626-2634`);
  - `select_arc`'s clockwise arm (`1303-1330`);
  - `rest.rs:1281`;
  - a variant at `boxes.rs:3233`.

  Closing "two answers to one question" minted a fifth copy of the reversal. There is no reversal helper on `Curve3` or `EdgeCurveSpec`.
- **S2 (Q1/Q2, likely).** `segment_curve`'s run choice (`chord_join.rs:2719-2732`) restates by hand which run `join`'s first chord co-bounds. Its doc asserts the agreement, and nothing enforces it. `if l2 == outer { next2 } else { h1 }` is written twice, at `2730` and `2858`.
- **S3 (Q1/Q3, likely).** "The curve run back" is encoded twice:
  - the winding uses a flag, `((carrier, params), forward)` (`run_closing`);
  - the mint uses a flipped carrier with negated params (`running_from`).

  Only the blind D sweep row ties the two together. The unit row `a_run_on_arcs_is_decided_by_its_bulge` (`merge_faces.rs:5363-5391`) builds `RunClosing` by hand and never calls `run_closing`.
- **S4 (Q3, sure).** `run_closing`'s `true` arm is dead in production. On the ring lane `curve_order` returns `(ea, ra)` and `resolve` winds `(ea, ra)` (`boolean/join.rs:1636-1690`), so `h1 == halves.0` always. The other role order is inferred from antisymmetry, never computed.
- **S5 (Q1, likely).** "Halves either side of the segment's edge" is spelled three ways:
  - `choose_roles`' `follows_across_segment` (`boolean/join.rs:1594-1610`), added so the new closing never winds a zero-area sliver;
  - `segment_curve`'s `prev1` test (`chord_join.rs:2723`);
  - the joiner's `prev_adjacent` + `skip_adjacent_chord` (`2807-2820`).
- **S6 (Q2, unsure).** The `RoleLane::resolve` doc (`boolean/join.rs:1640-1655`) carries the PR 5.5 / A×Z counter-island history unchanged into its new home. It is longer than the code it defends.
- **S7 (Q7, unsure).** In `segment_curve` (`chord_join.rs:2720-2733`), `let (u1, u2) = (u1, …)` rebinds `u1` to itself, and a closure named `between` returns an edge key.
- **S8 (Q5, likely).** The `ArcWindowCase::NoChartedRun` doc (`chord_join.rs:104-118`) now says a ring lands there "only when that run is scaffolding alone". The off-axis `axis_lap` laps still land there, and no row states what their run held, so the claim is plausible but not checked.

REVIEW COMPLETE
