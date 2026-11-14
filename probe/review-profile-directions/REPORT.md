# Review — PR 6963 `boxfinish/profile-directions` @ 8835b1bb (FULL)

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. Every verb-recorded direction I could reach is the true unit at f64, every registered radial identity I traced holds over the box where the door's decisions let it through, and the gate is green on this head at the STEP level. The fixes are a wrong recourse, a loosened differential that dropped a field it never meant to drop, three unpinned "records X" claims (surviving mutants), and the word "unit" promising more than the table check holds.

## MAJOR
None.

## MINOR (ranked)
1. **The recorded-direction rows cannot tell a verb that records from one that forgot.** DEMONSTRATED, sure. Three mutants survive the WHOLE profile suite (all 731 rows, f64 and Sym):
   - M13: drop `.radial([away(u), away(end_ang.unit)])` in `path.rs` `tangent_arc_geom`;
   - M14: `bulge_leg` records nothing on `Some(sign)`;
   - M6: record a director on the point-target leg (`emit_straight_leg_at` calls `direct_leaving`).

   The cause: `bulge_and_tangent_arcs_record_their_centre_to_vertex_radials` checks `ValidatedSegment.directions` against `(v − c)/r` to 1e-12, and validation's own mint passes that check. The row named for the claim can only catch a *wrong* recording, never a missing one. M14 is probably caught by editor-core's `m10_bulge_interval` axis count (not run). M6 pins nothing: "point-target legs record no director" is a disclosed decision with no row. Wrong recordings are caught (M1, M2, M3, M4, M5, M7, M8, M9, M11, M12 all red; M10, onto's swap dropped, is red in `arc_onto::*` through the registration, not in `recorded_directions`).
2. **`path_differential::assert_validate_identically` was loosened wider than the change needs.** BY-INSPECTION, sure. `without_directions` hand-lists role, vertices, (start, end, kind), tangent_joints and cusp_joints. `ValidatedLoop` also holds `decided_joints` (carriers, reverses, margin), which the old `format!("{:?}", loops())` compared bit for bit and nothing now compares. The narrowest true loosening blanks only `directions`. It is also a new hand-kept field list (the Q1 trap), so any future `ValidatedLoop` field goes uncompared silently.
3. **`DirectionBelowSceneResolution`'s recourse is wrong for a line.** BY-INSPECTION, sure. `validate.rs` Display uses `ARC_SCENE_RESOLUTION_RECOURSE` ("…or author the arc with a smaller radius (a flatter arc stores its centre farther out)") for every `which`, including `RecordedDirection::Director`. That is exactly the case `refusal_concision_chains`' new census row builds (`which: Director`). The census row pins the text without reading its truth.
4. **An accepted table direction is unit only to ε/ℓ, while the README and the field doc say "unit".** DEMONSTRATED, sure (`review_direction_slack.rs`, run at ε = 1e-9, 1e-6 and 1e-12). `segment_direction` reads ‖o + ℓd − p‖ = ℓ‖d − u‖, so it accepts ‖d − u‖ ≤ ε/ℓ in angle AND in length, and stores `d` verbatim. Measured on a square:

   | Side ℓ | Angle error | Length error | Stored |
   |---|---|---|---|
   | ≈30ε | 1.9° | \|d\| − 1 = 3.4e-2 | (0.99944, 0.03355) |
   | 100ε | 0.57° | 1.0e-2 | |
   | 1 m | ε rad | ε | |

   The ratio is the same at all three ε (sides below about 10ε refuse elsewhere).

   A door that takes such a `d` as a plane normal builds a wall that meets both of its own vertices within the band. But a point at distance D from that wall then reads its distance scaled by |d| (D·ε/ℓ off), which is far outside the band for D ≫ ℓ. In shipped builds only `RawLoop::new_directed` (test-support) and `into_loop` of a constructed loop (verb directions, exact) reach the check, so no production direction is off today. That is why this is MINOR, not MAJOR.

   The format question: either the README's "unit" is the verbs' promise and a table's accepted direction should be stored re-normalised or re-minted, or "unit" needs the qualifier "to ε/ℓ". I lean to the first, because a table direction then carries information only in its kind.
5. **The disclosed "not recorded" list is unscheduled.** BY-INSPECTION, likely. The list is: `line_to` hops off an axis, point-target legs, `Center` and side arcs, fillet arcs. `the-sweep-doors-read-the-profiles-recorded-directions.md` schedules the doors only; no file under `work/` schedules recording these, and the following unit's closing test names the tour's certified stop. The tour rocker is fillets and side arcs throughout, so it will still carry quotient widths into the doors. A line×line fillet's radials are its carriers' unit normals, which are exact, so the construction holds them. A filleted rectangle and a slot are the same shape. The plate (rectangle plus circles) and axis-aligned slabs (F43 axis) are covered. This is a Q6 deviation that owes a file.

## NOTE
- `value_channel` (`validate.rs` ~1342) re-mints `(end − start)/|·|` or `(p − c)/rim` for ANY direction that is not a point, including a VERB-recorded one with width (split circle at a boxed phase, a tangent arc over boxed data). The f64 witness stores the verb's own value, so the channel's direction agrees with it only to rounding, not "as validation at f64 makes it". The fn doc still says every non-point value is `ValueNotAPoint`. BY-INSPECTION, likely. No consumer compares the two (grep: only `loft.rs` reads `is_ok()`).
- Audit row F43 overstates two things. "A lane that answers differently reads the same population" is false when `literal_sign` ties: the arc then records nothing, and validation's mint adds 4 axis readings. "No reader rests on the pick" is also not quite true: the symbolic tier's registered radial identities rest on `literal_sign`'s Positive or Negative. That is sound because `choose` reads at least band over the box, but it is a reader. BY-INSPECTION, likely. In practice a tied bulge escalates `segment_straightness` before the mint.
- `m10_bulge_interval` D-tab row comment says "its two straight sides' chords … of the six". The D-tab (`m10_10_r2_probes_interval.rs:322`) has THREE `LineTo` sides: 3 × 2 = 6 readings, so `[3, 3]` is right and the comment is wrong. Re-derived: D-tab `[3, 3]` = 3 axis-aligned hops (one theorem zero plus one numeric sign each); boss `[1, 1]` = 1 hop. `m10_8`'s +28/+28 is consistent with 7 validations × 4 axis-aligned slab hops (not re-run from the merge base).
- `lift.rs` `rotated` (dev-side v1 lift) rebuilds through `from_chain`, dropping directions, under a doc that says "every stored bit carried verbatim". Not reachable from a verb-recorded loop that matters today.
- Slow-set move: on PR 6953's run on the same day, `the_cut_cylinders_ellipse_face_takes_nothing_away` took 0.720 s. On this PR's final run its sibling `the_cut_cylinder_reads_its_truth` took 0.498 s, against 0.686 s there. That is consistent with runner noise. But the body's reason ("this diff does not touch crates/sweep, so the time is the runner's") is a non sequitur: the row validates profiles through the code this diff changes.

## Per claim
1. **Held.** BY-INSPECTION plus mutants.
   - circle: `c ± |r|`, with r decided positive.
   - split circle: the vertices are built from the same (c, s) pair.
   - bulge/via: re-derived (a − c)/r = sgn(x)(−2xû − (1−x²)n̂)/(1+x²); the vertices are `from_chord`'s own chord ends. `literal_sign` is box-sound (`choose` = least band; a tie records nothing).
   - tangent: re-derived via the bisector reflection P = −R: t_b = R(u), radial_b = σJ⁻t_b. σ = sgn ρ is decided.
   - onto: swapped, and M10 is red through the registration.
   - `.decided()` has one caller (a fillet, which never has radials), so the drop is defensive.
   - The planted-lie row covers the quarter circle only. On the other paths it is the registration in shipped rows that catches a lie (M10), and M9 is caught only by the circle row.
2. **Held at f64.** DEMONSTRATED by the probe: every validated direction checked against its chord/rim at 1e-9 with |d| − 1 < 1e-12, across all profile rows and the first 254 `sweep::all` rows; zero hits. Directed legs and rectangles decide length/side positive first, so the director always runs start to end. Box rows: the circle's (macroscopic box) and the rectangle's and directed leg's (ε/64) are exact. Not run: `from_chord_radials` at Interval near x → 1 and x → 0 (by inspection it is better conditioned than the quotient; there is no division by x).
3. **Broken in its word "unit"** (MINOR 4, measured).
4. **Mostly held.** (a) BY-INSPECTION; I did not build the two-variables drive case. (b) Population claim: see NOTE. (c) No reader of directions exists yet, so no −0/+0 reaches a digest (the author's digest diff is not re-run). (d) No consumer compares lanes' directions.
5. **Held for verbs, lifts, reversal and canonicalisation.**
   - The sites: `map_scalar`, `reversed`, `lift`, `value_channel`, `canonicalize_loop` (no rotation), `into_loop`/`lift_computed`, `Core::directors` and its reset in `set_leaving`, and the seam retrim of v0 (a straight first side only).
   - An arc that reads as a line drops its radials and mints a chord director.
   - Exceptions: `value_channel` (NOTE) and `lift.rs rotated` (NOTE).
   - No mis-indexed direction found.
6. **Held,** except the `path_differential` loosening (MINOR 2). Decisions are taken once. The re-pins are consistent by arithmetic (NOTE). `sym11_exact_channel_rows` was not checked.
7. **Partly held.** The rows go red for wrong values (11 of 14 mutants red). They cannot see missing recordings (MINOR 1). The hop row's containment test is monotone the wrong way: any wider enclosure passes. The ε/64 box is evidence only for recorded literals; directions with width are shown only at ε/64, and the macroscopic case waits on the `line_span` file, which is scheduled.
8. **Held as to the format's content; the gaps are unscheduled** (MINOR 5). For the door to owe nothing, the format would also need the placed start's rigidity, which is outside the profile.
9. **Held.**
   - The surface: `ValidatedSegment` has pub fields and is `Copy`, but no door accepts a caller-built one into a `ValidatedProfile`, so a caller can mint a nonsense segment but not a validated profile that holds one.
   - Neither refusal is reachable from Python or a shipped Rust path, because tables cannot write directions outside test-support.
   - The `DirectionBelowSceneResolution` recourse is wrong (MINOR 3).
10. **Held.** Run 43641064452, head 8835b1bb (verified on the PR).
    - kernel default eps: 12099 passed.
    - kernel eps 1e-6 and 1e-12 (profile + 5 pin files): 732 each.
    - app default: 4757 passed; app eps rows: 121 each (the m10/sym pin files are named in EPS_EXTRA).
    - The slow-set step ran 0.
    - Inherited reds: I did NOT spot-check the 14.
11. **My sweep, by type:**
    - every producer of a `ProfileLoop` (`from_chain`/`from_directed_chain`/`RawLoop`): found `lift.rs rotated` and `test_support` dropping directions;
    - every reader that differences a `ValidatedSegment`'s points: `value_channel`'s own re-mint, which is a new instance of the pattern the unit closes.

## What the end-to-end exercise showed
I instrumented `canonicalize_loop` to check each stored direction against its chord/rim on every validation. It found no wrong direction in any constructed profile. The change solves the right problem for circles, rectangles, directed legs, bulge/via/tangent/onto arcs and axis-aligned hops.

In practice most authored profiles in the corpus are pen-drawn `LineTo` hops (the D-tab, the slabs) plus fillets. Off-axis hops and every fillet, Center or side arc still reach the doors as a quotient with width. So the following unit can finish the cut plate and the width slab, but not the rocker, without more recording.

Ergonomics: two parallel enums and an `Option` whose `None` has three meanings (Style).

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 was partly exercised: I read seg.rs's new code and lib.rs's ProfileLoop region whole; path.rs (~9k lines) was NOT read end to end.
- **The representation (Q2/Q7), sure.** `directions` beside `kind` is a second enum kept in step by hand. The type admits `SegmentKind::Line` + `SegmentDirections::Arc`, and FOUR sites handle that impossible pair:
  - `check_directions` `_ => Kind`;
  - `directions_match_kind`;
  - `validated_directions`' fall-through;
  - `value_channel`'s `(Arc{..}, Line) => return Err(site)`, which reports a structural impossibility as `ValueNotAPoint`.

  On `ValidatedSegment` the directions belong in the kind's arms (`Line { director }`, `Arc { arc, turn, radials }`). I think the representation is wrong there. On `ProfileLoop` the `Option` beside `Segment` is the same shape.
- **Q5, sure.** `None` on a `ProfileLoop` means three things: a verb that held no exact direction, a table that wrote none, or a `from_chain` caller that dropped them (`lift.rs rotated`, `test_support`). No reader can tell which, and the rows cannot either (MINOR 1).
- **Q1, likely.** Five places mint or re-mint a direction: the verbs, `check_directions` (accepts and stores verbatim), `validated_directions`, the axis rule, and `value_channel`'s re-mint. The last is a second spelling of the chord-unit mint (`(end − start)/start.distance(end)` against `ChordFrame::of`); they agree today only because both call `distance`.
- **Q1/Q7, likely.** `Core::directors` is a parallel `Vec` beside `verts`, grown by `resize` and reset in `set_leaving`, so two arrays are kept in step by hand. The director belongs in the vertex's `Option<BuiltArc>` slot, as an arc's radials live in `BuiltArc`.
- **Q2, likely.** "Radials only with registered facts" is held by convention. `BuiltArc::new(.., Facts::Decided).radial(..)` would record directions that `ByConstruction` validation trusts, with nothing registered or checked. `decided()` clears them, but `radial()` does not refuse on Decided facts.
- **Q3, sure.** The hop row's "never narrower" is containment at two box ends, which is monotone the wrong way. The tangent/bulge row cannot fail on a missing recording.
- **Q4, likely.** `value_channel`'s doc ("every stored scalar … read as its one f64 … ValueNotAPoint") and `lift.rs rotated`'s "every stored bit carried verbatim" are now false.
- **Q6, likely.** The "not recorded" list and "the door will still owe a rigidity registration" are disclosed in the PR body but have no `work/` file.
- **Q2, unsure.** F43's ledger prose argues "no reader rests on the pick" at length, while the registered identities do rest on it.
- **Q1, sure.** `without_directions` is a hand-kept field list (MINOR 2).

## Rubric
- **Idiom and structure: 3.** Clean, local code, but two parallel enums where one would do, and a parallel `directors` array.
- **Test quality: 3.** Eleven of fourteen mutants go red; the "records" claims cannot fail on omission, and the differential lost `decided_joints`.
- **Doc and comment honesty: 3.** The construction proofs are correct and checkable; "unit", the `value_channel` doc, the F43 population claim, the D-tab comment and the line-director recourse are wrong or overstated.

## Spec deviations
- **Reported by the author:** point-target legs, hops, Center/side/fillet arcs are not recorded; `path_differential` re-baselined; one row moved to the slow set; `sym11` left un-re-pinned.
- **SILENT:**
  - `value_channel` re-derives recorded directions with width (the unit says the lift never re-derives);
  - the loosening drops `decided_joints`;
  - a table's accepted direction is stored non-unit;
  - `lift.rs rotated` drops directions.

## What I executed / read
- **Executed:**
  - `crates/profile/tests/review_direction_slack.rs` at three ε;
  - the validate-time direction probe (`probe/review-profile-directions/dirprobe-instrumentation.patch`) over all profile rows and 254 `sweep::all` rows (the run was stopped there for time);
  - 14 mutants (`mutants.py`, logs `mutants.log` and `mutants2.log`);
  - CI step-level log reads for both test shards.
- **Branch:** `boxfinish/review-profile-directions` @ 960254bd.
- **Read only:** the driver certificate argument (4a); `sym11` and the 14 inherited reds; the probe-switch-off branch's M1 report; Python reachability; editor-core pins (not run locally).
