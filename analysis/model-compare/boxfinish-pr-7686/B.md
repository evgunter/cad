# Review: signed radius (PR 7686), frozen head `ddbd1deced`

**Verdict: NOT-MERGEABLE-AS-IS.** Two facts, both checked: the head has no hosted CI run and conflicts with main (M1), and an existing profile row that is green on the merge base is red on the head at ε = 1e-12 (M2). The unit's substance held under everything else I ran; with M1/M2 and the fixes below it would be APPROVE-WITH-FIXES.

Probes: branch `boxfinish/review-signed-radius-own` at `82ebac38f` (`crates/sweep/tests/review_signed_radius_own_probes.rs`, `demos/tour/tests/review_signed_radius_own.rs`). Base = `f2815a232a` in a second worktree and target dir, same probe file on both.

## MAJOR

**M1. The gate did not run, and the head does not merge. DEMONSTRATED, sure.**
- Hosted, looked at start and end: the only check on `ddbd1deced` is `merge ref`, **failure** (run 45341655092): "PR #7686 conflicts with its base 'main' (mergeable_state: dirty) … CI created NO run for this push". Zero `ci.yml` runs on this head.
- `git merge-tree origin/main HEAD` conflicts in 7 files: `profile/src/seg.rs`, `geom-core/src/lib.rs`, `m10_9_pins_interval.rs`, `seat7_sweep_lowering.rs`, `sym11_exact_channel_rows.rs`, `sym_9_retry_interval.rs`, `a_sweep_keeps_the_frame_it_is_handed.rs`. Four are pin files this unit re-cut, so the pins get cut again.
- What a queued lint job would have said, run by me on the head: `cargo fmt --check` green; `scripts/run-gates.sh` green; `python3 scripts/work.py lint` green; clippy as CI spells it (`--workspace --exclude viewer --all-targets --all-features -D warnings --force-warn clippy::disallowed_methods`) green; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit` green.

**M2. `profile` `a_far_arcs_lens_reads_its_own_width::a_far_arcs_lens_past_ten_eps_wide_validates` is green on base and red on head at `CAD_TOLERANCE_EPS=1e-12`. DEMONSTRATED, sure.**
- Base: profile 768/768 at default, 1e-6, 1e-12. Head: 769/769, 769/769, **768/769**.
- Payload: `lens 0, 1.07e-11 wide: Err(ArcBelowSceneResolution { check: OnCarrier, margin: 6.10e-11, headroom: -994370.5 })`. Raising site: `seg::check_carrier`, which this unit re-posed from `rim(a) − radius` to `off_carrier(a)`.
- The row stands down at every other ε, so the author's profile differential (ε 1e-9 only) could not see it. The body's "profile: none change colour" and "`f64` is unchanged but for…" are both false on this row.
- Same class as m1 below. I did not establish whether the new refusal or the old acceptance is the right answer for a radius-1e6 lens; that is a design question the row forces.

## MINOR

**m1. `f64` decisions moved in a class the body does not list (claim 15 broken). DEMONSTRATED, sure.** 5040 generated documents (12 verbs × 7 poses × 60), head against base: 93 differ; 22 are frame bits only, 71 change stage.
- Every stage change is an arc of radius ≳ 4e6 m (bulge ~1e-8 on a 4 m chord; a via point ~1e-7 m off its chord). The head's path door refuses at `PathArcStartOnCarrier` (escalated, margins 1.6e-9 … 6.8e-9) or `ArcMissBelowSceneResolution` where the base stored the loop; four go the other way.
- No document built on one tree and not the other: the base refused each later, at validation or the extrude. For a profile-only consumer, ~44 that validated on base now refuse at the path door.
- A circle of radius 8.6e7 m moves `ArcMissBelowSceneResolution → Escalated`, far from the body's 1e160 overflow case.
- Unpinned by any row at the default ε; M2 is its ε = 1e-12 instance.

**m2. Two kept registrations depart from the weighed design with no scheduled row. BY-INSPECTION, sure.** The design retires four; `Arc2::register_radials` and `register_placed_carrier_ends` stay, disclosed in the body only. The unit's own row still says they retire, and no `work/` row records why they are kept or what would retire them. (Their soundness: claim 5, held.)

**m3. `sugar.rs:1398` and `:1568`: "the enclosing-class gate decided it non-negative" is not what the gate decides. BY-INSPECTION, likely.** `FilletEnclosingCarrier` (`sugar.rs:671`) refuses only `Negative`; `Zero` (ρ in band, negative values included) passes. With the `abs` dropped, `rho − |h|` and the circle-pair margins move by up to 2|ρ| there, so a Zero/Negative decision can move at `f64`. Not in the body's list, no row. ρ does not reach a stored `Arc2::radius`.

**m4. The body's receipts table disagrees with the tree. BY-INSPECTION, sure.**
- "`revalue` plate floor at ε 1e-12: 1802 → 1783": `revalue_corpus.rs:576` is 1802 on base and head. What moved is `boss_union` `at_1e9` 1462 → 1406, absent from the body.
- The D-tab lost **four** theorems, not two: `arc_span` [6,4] → [4,6] beside `circle_side` [2,2] → [0,4] (`m10_bulge_interval.rs:291, 304`).
- The `circle_side` loss has no row: the cited bulge-beside-a-line row covers `arc_span`/`line_span` only.

**m5. Coverage of the removed `abs` sites. BY-INSPECTION, likely.** `each_arc_verbs_stored_arc_is_exact_over_a_box` keys its radial assertions on recorded radials; fillet, sweep and sweep cw record none. `ARC_VERBS` has no `ArcLen` leg, `Radius` arrival, family bulge leg or carrier-form fillet. Three of the six removed `abs` have no row their mutant reddens.

**m6. `u_ref` "a unit through the tier" is asserted only at the ε/64 box** (`a_sweep_keeps_the_frame_it_is_handed.rs:1208`, skipped unless the stage is `finished`), where an in-band enclosure passes it too. The unit row's closing test (exact `u_ref` over a box, per verb) is not pinned at a width that distinguishes. BY-INSPECTION, likely.

**m7. `m10_9_the_rim_registrant_discharges_the_plates_endpoint_identity` is now misnamed** (no rim is registered) and pins only `open.symbolic > shut.symbolic`; the body's "32 decisions" is not asserted. `Door::LoadBearing` is `#[expect(dead_code)]`: no document on the head exercises a load-bearing door. BY-INSPECTION, sure / likely.

**m8. Stale text in facts this change moved. BY-INSPECTION, sure.**
- `validate.rs:2659` ("`Seg::radius` is `signed_radius.abs()`"); `path/verbs.rs:396, 407, 415` and `path/family.rs:219` (radius "as `|r|`", "the rim at each end is the radius"); `path.rs:4604`.
- `sym11_exact_channel_rows.rs:38` ("the link replays whole" over a pin that now refuses); `m10_9_pins_interval.rs:128`.
- `swept.rs:1607` calls `arc_from` a second spelling of the profile's mint, which divides by the rim, not the radius.
- `demos/tour/.config/nextest.toml:22` still lists the deleted row.
- E12's same-OBJECT paragraph still says "the swept arc's rim and span identities discharge because the registrant builds the very nodes" (unsure: history or rot).

## NOTE

- **n1.** A table arc of negative radius with a vertex exactly on the mirror circle: base `InconsistentArc { OnCarrier }`, head `Escalated` (invalid margin). Test-only door; unlisted.
- **n2.** `Arc2::carrier_end`'s "one function … at every value, the radius's sign included" is `0/0` at radius 0; the docs do not except it.
- **n3.** `sym11` pad's frozen ceiling rose 1963 → 2863 with no comment at the pin, on a table whose doc says a new freeze reds. `m10_9` link bracket `certifies_at 3.029e2` / `refuses_at 4.395e2` leaves [303, 439]·ε unwatched.
- **n4.** `sugar.rs:1320` (`copysign(rho)`) and `:284` (`half_tan.abs()` into stored trim points of a `Registered` arc) are pre-existing; the second is held by `work/profile/the-fillets-registered-tangency-says-it-holds-where-the-doors-decisions-hold.md`.
- **n5.** The body's grep names `sweep/src/revolve.rs`, which does not exist; `revolve/` was not swept (`revolve/axis.rs:369` reads a `copysign` into a bound; sound in value).

## Claims

1. **Held.** No non-positive stored radius in 5040 `f64` documents (near-collinear vias, on-line tangents, bulges to 1e-13, poses to 1e9). By algebra each verb's sign is the gate's or the decided side's own expression.
2. **Mostly held, by reading.** One raw `radius − rim` left at `seg.rs:2497` (circle-pair contact depth, a measured foot, right in value, unlisted). Downstream readers in geom/topo/sweep are sign-tolerant magnitudes.
3. **Held for the mirror wall** (exact mirror escalates, off it `InconsistentArc`, zero radius `SweepRange`). **Broken on "the only class"**: m1, M2, n1.
4. **Held.** A bulge boxed across zero never leaves the door: `PathJunctionTurn` escalates first, mid-path and as first leg; the fixture door stores radius `[2, inf]` and validation escalates. The tie arm is reached by no row, and the design's "a tie refuses" would be simpler.
5. **Held, by algebra, no counterexample.** `c + radius·radial = vertex` and the turned pair are identities for circle, split circle, chord lowering (any `s` with `s² = 1`), tangent (`σ` cancels) and via; no division, so a zero or negative radius falsifies none. The placed pair is an identity for any radial because `arc_span` multiplies the sweep by the turn's literal. A ring freeze is a family the tier misses, which E12's provenance clause admits; see m2 for the row owed.
6. **Held.** A table radial stretched 0.1·ε or 0.4·ε: `f64` builds squared; `Interval` and `Sym<Interval>` refuse typed at `EndpointStart` (poisoned frame), no panic, no false registration. The author's row covers `f64` only.
7. **Held, by reading.** No consumer sees `w/‖w‖` against `(a − c)/radius` as other than an in-band residual.
8. **Held, with a gap.** The normalize is right at enclosure scalars; `lift`'s doc has no sentence for `f64` (re-rounds) or `Dual`.
9. **Held.** The tour row passes on the head. My probe: the whole-box assertion margin is `[-5.99995e-5, 2.599995e-4]`, and five members (corners and an interior point) measure webs whose margins lie inside it, the corners at its ends, so hole positions and radii in the cut body are right. The row itself asserts nothing about the body, and is in the tour's slow set (nightly only). "This unit does it": shown through the door-arm mutant; the `radius.abs()` mutant not re-run.
10. **Held at the base.** 80 cells (12 verbs × 6 widths, plus 8 wide): no verb stops earlier at `Sym<Interval>`; the body's table reproduces, including bulge 0.5 ± 0.02. At `f64` it is not true of profile-only documents (m1, M2).
11. **Partly broken**: m4, n3. No unsound replacement found: an in-band numeric Zero is a liveness loss over a box, not a wrong side. Totals conserved on every pin checked.
12. **Held for `m10_10` and `m10_5`** (scale moved only). **`m10_9`**: m7.
13. Door-arm mutant on the tour row, **run: red**, both hole extrudes at `carrier_endpoint_start` `[0, 4.57e-4]`, as predicted. (d) topo mutants: **not run** (cost); by reading, the disc row asserts "no `In`" and gets easier as the reader degrades, so nothing on the head sees a curved straddle on the wrong side.
14. **Held, by reading, likely.** The three clauses describe the moved mechanism; E12's literal-sign sentence predates this unit (`[ev]` commit `708114233`). Not executed.
15. **Broken**: m1, M2.
16. **Partly taken.** Sweep whole suite, default ε: head 3901/3906, base 3897/3902, the same five reds at the same lines (`bisector_nodes::the_near_node_poses…`, `germ_tilted_rod::other_spins…`, `section_end_poses::a_section_end_at_the_band_edge…`, `self_intersection_proves_injectivity::a_notched_rod…`, `verbs_germarms_interval::the_ring_lane…`). The body says four: its count is stale. Two of the `work/green` rows naming them read `closed`, so which open row holds each is unverified. Profile at three ε: M2. **Not run**: topo, editor-core at any ε, sweep at 1e-6 / 1e-12, the whole tour (one box; a 4-job build was OOM-killed).
17. Dispositions right in kind (six spot-checked); the two flagged sites: m3, n4. Stale text: m8.
18. Every changed signature has a sentence except `lift` (claim 8). Style S1.
19. After this PR a recorded radial can still be: a table's written one (unit only to the `segment_direction` band; the frame door squares or poisons it); absent (tie arm, fillets, swept-angle and family legs, minted at validate from the measured rim); a lifted one (normalized at the target). The frame door does not depend on `UnitVec2`: it rests on `measured_frame`.

## End-to-end

The change solves the stated problem: the cut plate evaluates over its study box and its web encloses its members. What is missing in practice: the reader change is not neutral at `f64` for large radii (the gap sees a stored arc's own unrepresentable sagitta that `rim − radius` absorbed), and that is neither disclosed nor decided. Liveness over a box moved only for the circle; every verb beside a line still stops in validation (filed), and a lone circle slab does not finish at any box ≥ ε (filed).

## Style

Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q8 (`geom-core/src/arc.rs` whole; `seg.rs` and `swept.rs` in the changed regions only). Q7 as taste.

- **S1 (Q1/Q7, likely).** "The literal ±1" is a bare `T` in `from_chord`, `from_chord_radials`, `lower_arc`, `tangent_arc`, and an `Option<T>` in `bulge_leg` / `BuiltArc::lowered`, while `via_arc` takes `ArcSweep`. Three spellings of one concept, and every proof rests on `σ² = 1`, which only a literal has (`arc.rs:63`, `lib.rs:327`, `path.rs:4057`).
- **S2 (Q1/Q5, likely).** `Arc2::carrier_end` is now `landing` in a second spelling; its test was rewritten to assert they agree off the carrier too. Four of `register_endpoints`' eight facts restate the other four (`arc.rs:~258, ~345`).
- **S3 (Q1, sure).** The fix mints the shape it closes: `seg::mint_radial` divides by the rim, `check_directions` and the sweep's test `arc_from` by the radius, and a comment reconciles them (`seg.rs:923`, `swept.rs:1607`).
- **S4 (Q2, likely).** `register_placed_carrier_ends`' justification is about four times its body, and argues that a registration the design retired is harmless (`swept.rs:795`).
- **S5 (Q3, likely).** The disc row's only "some aim reads a side" guard switches off above 1e-3; the loft row has no tightness bound (`an_aimed_ray_over_boxed_door_bodies.rs`, `a_sweep_keeps_the_frame_it_is_handed.rs`).
- **S6 (Q3, sure).** `m10_sym_profile_interval.rs:438`: every `blocked` flag is now `false`; the column is dead.
- **S7 (Q6, likely).** `PLATE_MAX_TERMS` 76 → 222 rests on a measured 111 nothing derives; a threefold growth of the plate's largest form is a cost signal with no register.
- **S8 (Q1, sure).** The tour row hand-copies `4096` / `128` under "the drive's default budget" where the row it replaced read `DriveConfig::default().symbolic` (`tolerance.rs:~180`).
- **S9 (Q7, likely).** `sketch_vector`'s arc arm is dead by its own doc (`swept.rs:718`).
- **S10 (Q7, unsure).** `Arc2::radius` is "positive in value" by doc alone where DESIGN has `Size` for that; the invariant of claim 1 is a convention.
- **S11 (Q4, sure).** The class behind m8: `|r|` / "rim … is the radius" survive in six doc comments in `profile`. I would also look in `crates/geom-core/src/sym/{trig,root}.rs` (`abs(signed_radius)` in the present tense) and `docs/K-REPORT.md`.

## Rubric

- **Idiom and structure: 3.** The readers and verbs are clean; the sign-as-`T` and the doubled carrier end are not.
- **Test quality: 3.** The mutants named are real and the one I ran is red; the rows miss three removed sites, the wide-box `u_ref`, and the `f64` change entirely.
- **Doc and comment honesty: 3.** Code docs mostly follow the change; the PR body is wrong in four checkable places and six comments still describe the magnitude.

## Spec deviations

- **Reported by the author:** two registrations kept; the tie arm stores a measured radius; `mint_radial` keeps the rim; three rows restated; the door-arm mutant not re-run; topo and sweep slow sets not re-run.
- **SILENT:** the `f64` movement at large radii (m1) and the red row at 1e-12 (M2); the `sugar` gate comment and its decision sliver (m3); the D-tab's `arc_span` loss and the `boss_union` floor (m4); no hosted run on the head.

## Executed / only read

- **Executed:** the five lint-job commands; the probe file on head and base (stage ladder, 5040 `f64` documents, tied bulge ×2, negative-radius table arc, stretched radial at three scalars, the moved documents in full); sweep whole suite at default ε on both; profile at three ε on both; the tour row and my member probe on the head; the door-arm mutant on the tour row.
- **Only read:** claims 2, 7, 11, 12, 14, 17, 18 and the reading half of 13; E12; the pin files.
