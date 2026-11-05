---
id: meet-at-interval-refuses-on-its-basin-meter-and-reads-an-ends-distance-at-rounding
kind: issue
title: curve_pair::meet at Interval refuses on its basin meter where the region rows decided, reads an end's distance at rounding, and costs 3-6x the rows
status: dispatched
opened: 2026-11-04
priority: P1
cost: M
refs: [6516, region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet, curve-pair-polish-at-interval-reads-the-iterates-width, inline-bracket-midpoints-are-spelled-in-two-forms]
branch: onecopy/polish-meters
---

Found by `onecopy/meet-pose` (PR 6491, now a draft), which routed the four region `landings` (`topo/src/boolean/{region,sphere_region,torus_region,ruled_region}.rs`) through `geom_brep::curve_pair::meet`. On `Interval` and at a fine band, `meet` decides less than the hand rows it replaced and costs more. **The landings route (PR 6491) waits on this row.**

## (i) The basin meter refuses at Interval

`topo/tests/props_sphere_cap_door.rs` `a_pole_a_hair_either_side_of_a_tilted_rim_is_read_at_both_scalars`, at ε 1e-12 (band 1e-12/1e-11), δ 1e-2, at `Interval`:
- 124 of 128 (leg, arc) meetings refuse `MeetFault::Unsettled(Unsettled::Meter)`, from the polish's basin meter (`curve_pair/pose.rs` `Polish::closest`) on interval enclosures;
- the other 4 escalate on `curve_carrier_gap` (enclosures `[0, 2.6e-12]`).

The pole then reads `ChartSingularityUndecided` where the rows read `ChartSingularityNotAVertex { lies: Interior }`. At f64 the same scene decides.

## (ii) An end's distance read at rounding

`sweep/tests/one_region_rows.rs` `points_a_band_off_a_lunes_pole_vertex_read_at_the_ends_precision`, band 1e-12/1e-11, point `(0, 1, 1e-11)` on the 90° lune:
- The point lies on the first arc's own great circle, `1e-11` past its end (the pole).
- Every leg from it meets that carrier at its start.
- `meet`'s span-end chord (`curve_pair_span_end_chord`, `curve_pair/span.rs` `Span::place`) reads `9.9998e-12` against the escalation `1e-11`. That difference is coordinate rounding (~1e-16 on unit coordinates), so every leg escalates.
- The row it replaced read the root's angle to the end as the raw root less the end's parameter, at the end's relative precision, and decided `Out`. That is what the pin is for.

## (iii) Cost

CI `test (kernel)` on PR 6491 head `650d63fa96`: 21 fast-set rows went over the 1 s bar, nearly all certified-scalar region rows, for example:
- `one_region_rows::a_torus_face_refuses_a_ring_off_it_or_inside_another_at_the_certified_scalar` 6.46 s;
- `verbs_tubewall_r2_probes::certified::r2_interval_rows_over_varied_radii` 4.20 s;
- `one_region_rows::torus_windows_a_hair_apart_in_tube_angle_validate_at_the_certified_scalar` 3.51 s;
- `snowman::the_snowman_builds_at_the_interval_scalar` 2.68 s;
- `verbs_tubewall::certified::the_hollow_elbow_certifies_at_interval` 2.11 s;
- `pis_arc_capped_poses::the_cut_cylinder_reads_its_truth` 2.00 s;
- `cell_region_bodies::a_ball_is_sound_and_its_sphere_decides_its_cells` 2.00 s;
- `pis_arc_capped_poses::the_cut_cylinders_ellipse_face_takes_nothing_away` 1.96 s;
- `a_ring_on_a_cone_face::the_rings_volume_brackets_its_closed_form_at_the_certified_scalar` 1.82 s;
- `cert_m2r1_passes::m2r1_passes_interval` 1.79 s;
- `one_region_rows::a_sphere_face_refuses_a_ring_inside_another_at_the_certified_scalar` 1.78 s;
- `one_region_rows::two_windows_at_opposite_azimuths_validate` 1.74 s;
- `fillet_h5_hostless_rim_interval::the_hostless_rim_carves_at_the_certified_scalar_on_both_material_sides` 1.65 s;
- `tilted_sphere_pair::a_tilted_sphere_pair_builds_at_the_interval_scalar` 1.53 s;
- `one_segment_loop::one_segment_circles_revolve_at_interval` 1.42 s;
- `verbs_tubewall_r1_probes::certified::enclosures_contain_and_stay_tight` 1.41 s;
- `a_vertex_on_a_grazing_plane::an_unresolved_section_reading_keeps_the_offsets_verdict` 1.24 s;
- `one_region_rows::windows_on_an_azimuth_grid_validate_at_the_certified_scalar` 1.21 s;
- `one_region_rows::a_torus_window_read_in_band_escalates_with_its_margin_at_the_certified_scalar` 1.17 s;
- `sf2b_interval_probe::interval_offset_charts_together_sphere_zone` 1.14 s;
- `contact11_torus_chart_l::a_notched_half_donut_answers_its_volume` 1.03 s.

CI `test (app)` flagged `editor-core` `m10_5_r1_probes_interval::a_block_with_a_rounded_bump_certifies_strictly_positive` at 2.44 s.

Locally, on a box shared with another lane: `sweep` `a_cone_apex_meets_a_vertex::two_cones_through_one_apex_build_across_the_pose_families` took 126 s with the landings through `meet`, against 47 s with the pose change alone.

## Asked

Make `meet` at least as decisive as the region rows at `Interval`, where the band allows a decision:
- a basin meter that does not refuse on enclosure width alone;
- an end's distance read at the end's precision, not as a chord of unit-sized coordinates.

Bring its per-pair cost near the rows' cost. Then re-run PR 6491's landings route against these rows.


## Also owed (2026-11-04, from the orchestrator's read of PR 6492)

- `pose::conic_conic`'s parallel screen reads `constant(row::PARALLEL, c0, swing + noise)` and drops a gap read through `matches!(.., Ok(true))`. When the swing is level, the same margin is read again on `row::OFFSET`. So a gap height records two escalations for one margin, and the PARALLEL one is unanswered when a later route decides. Read the margin once and act on it twice. This belongs to the class in `a-heuristics-choice-has-no-door-of-its-own` (PR 6483).

## Decided: what the polish certifies at Interval (2026-11-04)

This is a design fork weighed by three designers (A Fable; B and C Opus; draw on `analysis/design-fork/onecopy-polish-meters`). All three agreed after one round; reports are in the orchestrator's scratch, `oc-fork5`. No ratified text changes: the meters are specified only in `pose.rs`'s module docs (PR 5121, agent-written), and C5 does not name them. This also answers the design question on CPAIR's `curve-pair-polish-at-interval-reads-the-iterates-width` and `curve-pair-polish-meters-refuse-shallow-meetings` (route 2, "κ over the polish's actual travel").

**Diagnosis (sure).** Both refusing meters compare a *location along the carriers* with the band. A location's enclosure at T is the data's spread over sin φ, while a distance between carriers is 1-Lipschitz in the data. The ~13,000 refusals are that conditioning. The meters guard only completeness: an Off at the polished pair speaks for the region only if that pair is the region's closest pair.

**The final state.**
1. **Search is structure.** Gauss–Newton runs in f64 on the brackets' midpoints. The pair enters T through `from_computed`, which is the "f64 structure + T payload" pattern of `projection_policy`. `bounds_census` records the Dual disposition as a `Selection` (#874's class: a Dual meeting loses its tangent's dependence on the data). No consumer reads that dependence today, because the only Dual bodies are gate fixtures that never reach `meet`. This supersedes the branch's per-step `place` recentring, which still ran each step at T.
2. **The window's Off is one lower bound in metres of gap, read against the band.** At the pair, `w = q − p` and `n̂ = unit(a′×b′)`.
   - The *slack* `|w − n̂(n̂·w)|` is the part of the gap a move along the carriers could still remove.
   - The bound is `lower(|w·n̂|) − bend`, where `bend` is the carriers' curvature over the move the slack implies (about `(κ₁+κ₂)(slack/sin φ)²`). The implementer proves the exact constant in `pose.rs`'s docs.
   - If the bound reads Positive against the band, there is no candidate.
   - Otherwise the candidate goes to `Span::contact`, which reads On with no convergence premise. An Off from `contact` under an undecided bound is a straddle that escalates and never prunes (C5).
   - There is no K premise: the slack is never read against the band on its own row.
3. **Travel folds into the basin.** The basin margin is `sin φ − 2(κ₁+κ₂)·max(region, travel + Δ)`, with Δ the slack-implied move. It is read exactly. A location enters only as a turn, levered (`Lever`). `curve_pair_pose_travel` and `curve_pair_pose_polish` retire, and the registry, K roster and audit follow.
4. **One rule for every meter.**
   - A generator's own inequality (the basin) is read exactly, through one named `geom_core` door that takes no `Band`, and is logged: a `Meter` refusal is an outcome a narrower box can answer.
   - Anything in metres of gap (the window bound, `contact`) is read against the band and logged.
   - Nothing is unlogged.
   - The four existing `decide(.., Band::least())` sites (`curve_pair::compare`, walk `step_taken`, `curve_foot_polish`, the basin) read that door.
5. **What makes the next generator land right.** A generator states its excess in metres and decides nothing about its window: its candidate carries `excess: T`, which is the slack plus the bend, and zero for an end or a witness. The reader (`Found::read`) does the one composition. The exact door has no band parameter, and `metered(row, margin, want, band)` is deleted. A meter against the band becomes untypeable.
6. **Riders.**
   - `Unsettled::Meter { row }` names its check (`[loud-failure]`).
   - `an_unconverged_polish_refuses` gets a fixture that still has slack after STEPS, and pins that its window escalates.
   - The walk's polish (`walk.rs`) gets the same Off-by-lower-bound treatment, because today its `contact` Off at a polished piece pair prunes on an unproven pair.

**Measure on landing.** Print the meters once at Interval on tubewall r2 and the point-interval circles at tilt 1e-3, before and after. Revalue `m10_sym_profile` and the K roster.

## Measured on `onecopy/meet-interval` (2026-11-04)

Commits `3e18c0ba64`, `d52d7e161e` take what needs no ruling:
- `polish::pair` reads each step from a point (an enclosure's middle) and solves it in the cross-product form `s = (w × b′)·n/|n|²`, `n = a′ × b′`. The normal-equations form read the rounding of the gap over `sin² φ`.
- `Span` evaluates its end points once (`Span::end_point`), and an endless span has no ends (`Span::ends`): no end candidates, no witnesses anchored at its seam.
- `conic_conic` reads its height screen once and acts on it twice.

Against PR 6491's head with these applied: (i) `a_pole_a_hair_either_side_of_a_tilted_rim_is_read_at_both_scalars` passes; (ii) still escalates; the cost (iii) falls but stays at about 4x main's on six sampled rows (15.5 s against main's 3.5 s, 18.4 s before).

**Question 1: what the polish's meters certify at an enclosing scalar.** In `verbs_tubewall_r2_probes::certified::r2_interval_rows_over_varied_radii`, 13,000 of 24,000 `meet` calls refuse `Unsettled::Meter`, each sending its path to the next one: 11,344 on `curve_pair_pose_travel` (margin `[9.3e-9, 1.25e-7]` at band `(1e-9, 1e-8)`) and 1,640 on `curve_pair_pose_polish` (step `[0, 1.2e-9]`). A step read from one point still encloses the carriers' own spread divided by the sine between the tangents. So the convergence meter compares where along the curves the closest pair sits with the zero band, when the reading needs how far apart the curves are. Several answers are viable:
- (a) run the search and its meters on the enclosures' midpoints (the f64-structure lift of `geom::projection_policy`), and leave the deciding to `Span::contact` at `T`;
- (b) meter what the reading needs: the last step's metres times the sine, or the gap's change;
- (c) an interval Newton (Krawczyk) existence test for the closest pair at an enclosing scalar.
The same shows in `meet_rows::circles_on_one_sphere_meet_at_the_certified_scalar` at tilt 1e-3 (step `[0, 2e-12]` at band `(1e-12, 1e-11)`, from point-interval circles). The committed row pins only tilt 1e-2.

**Question 2: the precision pin sits on the band's closed edge.** In `points_a_band_off_a_lunes_pole_vertex_read_at_the_ends_precision`, every leg from `p = (0, 1, 1e-11)` is a great circle through `-p`, which lies `1e-11` from the south pole inside the 90° meridian. That edge's span ends at `t1 = π_f64`, whose point is `(7.5e-33, -1, -1.2246e-16)`. So the meeting at `-p` lies `1e-11 − 1.22e-16 = 9.99988e-12` from the span's end, exactly for the data. The hand row read `π_f64 − fl(π − 1e-11) = 1.0000000827e-11`: the root's rounding to the `π`-grid cancelled the end's. That is coincidence, not precision. With endless legs reading no seam, the first refusal is now `curve_carrier_gap` at `9.99988e-12`, the north pole against a leg that passes `1e-11` from it. A point at exactly the escalation width of a vertex is decided by `1e-16` either way. Either the pin moves off the edge, or `meet` reads an end's distance in parameter space, which would be a second reading of a fact the chord reads.

Question 2 was ruled a test, not a design question (ONECOPY orchestrator, 2026-11-04). The pin is re-posed on `onecopy/meet-interval` as `points_two_escalations_off_a_lunes_pole_vertex_read_on_their_side`, with its points `2e-11` off the vertex. Question 1 is with the designers.

## The review's fix pass (PR 6516, 2026-11-04)

**One finding, two homes.** The polish fix is also CPAIR's
`curve-pair-polish-at-interval-reads-the-iterates-width`, which PR 6516
closes. Its pin
(`band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval`,
pinned red at 1e-12) would have gone red on the nightly after merging,
and `band_planar_cut_off_interval`'s `tier_3_prime` admission would have
stayed with nothing using it. Both are lifted. The pin cited its row,
and `scripts/gates/pin-cites-an-open-row.sh` reds a pin whose row is
closed, so closing the CPAIR row is what makes the gate demand the
un-pin. What was missed is the step before that: this row was filed
without the CPAIR row in its `refs`, so fixing it did not close that
row. A gate already links pins to rows, so no row is filed for the
link.

**The precision pin, again.** The `2e-11` re-pose no longer guards the
landing's precision branch (`sphere_region.rs`, `to_end`): with
`to_end` always the half-width less the angle from the middle (M-a),
the old pin goes red and the `2e-11` one stays green. No row can guard
it at an honest offset. Rows at `1e-11 + 1e-16` and `1e-11 + 3e-16` are
green at head and green under M-a, and any offset small enough to read
the two forms apart lies inside the end's own rounding (`1.22e-16`), so
a row there pins that rounding, which Question 2 ruled coincidence. The
branch's comment records this; the `2e-11` pin is kept (orchestrator's
ruling, option (a)).

**Still owed:**
- `Span::new` evaluates both end points eagerly. That includes
  `topo/src/boolean/reduce.rs`'s split, which builds a `Span` only to
  call `branch`. Calling `Curve3::branch_near` there instead would
  spell `Span::branch`'s composition a second time. Making the span
  lazy is a larger change and buys little: `Span::new` already reads
  the carrier's speed range eagerly, which costs more than two
  evaluations.
- The transverse route's parallel straddle (n3 of the review).
- `geom_brep::krawczyk::midpoint` was `lo + (hi − lo)·0.5`, bitwise
  `geom::bracket_mid`'s form, and now reads it. `pair_section/
  spline_implicit/sweep/critical.rs`'s `mid` is `½(lo + hi)`, which
  differs in the last bit, so it stays: it belongs to
  `inline-bracket-midpoints-are-spelled-in-two-forms`.


## State after PR 6516 (2026-11-04)

PR 6516 fixes regression (i) at its root. Gauss–Newton now steps from a point, ends are read once, and an endless span has no ends. Question 2's ruling also lands: the lune pin is re-posed, and its precision branch is recorded as unguardable at an honest offset.

What is still owed on this row is the polish-meters decision above, "Decided: what the polish certifies at Interval". Its implementation is the next unit. PR 6491's landings wait on it.

## Amended at dispatch (2026-11-04)

The implementer read the four sites before writing code. Item 4 of the decision was wrong for two of them, so it is amended, ruled by the orchestrator:

- **The exact proof door** (logged, with no `Band` parameter) takes only the proof inequalities: `curve_pair::compare`'s callers and the basin.
- **Not every site is a proof.** `curve_foot_polish` is a heuristic choice, which ledger F32 ruled out of the log, and a test pins that. It stays where it is. F33 is the same case.
- **Walk's `step_taken` is also a choice.** A dropped step can lose a foot but never invents one. It stays on its current door until the choice-door unit (`a-heuristics-choice-has-no-door-of-its-own`).
- **The rule** is: a proof's inequality is read exactly and logged; metres of gap are read against the band and logged; a heuristic's choice is unlogged under a ledger row until the choice door exists.

Item 2's bend is made precise:
- `bend = K·m₋²/2`, with `K = κ₁+κ₂` and `m₋ = (sin φ − √(sin²φ − 2K(slack + e)))/K`.
- `sin φ` is the basin's lower bound over the region.
- The escalation enters the move, so to first order the move is `(slack + e)/sin φ`, not `slack/sin φ`.
- Where the discriminant is negative, the bound is undecided: it escalates and never prunes.

**Second amendment: the composition (items 2 and 5).** As written, the decision could not prune a window whose carriers meet *outside* the spans, which is the common two-circles-on-a-sphere case. There `L` is not Positive. `contact` reads Off past an end, and under the decision that Off escalates. Main prunes it today through the convergence premise, which the decision retired. Ruled:

- The polished candidate carries `reach = m₋`, the bend derivation's bound on how far along either carrier any pair within `e` lies from `p`. Ends, witnesses and exact roots carry 0.
- `Found::read` prunes an Off only when `L` decides Positive, or when `p`'s past-end side margin less `reach` decides Positive against the band. Both are logged. Anything else escalates.
- `reach = 0` reproduces today's reading.
- A location enters only to prune, never to accept.
- The implementer measures escalations against main at all three ε before the PR. If any test flips, or the past-end-by-less-than-`e/sin φ` class is not rare, the unit stops.

## Third amendment (2026-11-05): the run argument, after the cost gate tripped

The census of `reach` (A) moved no test outcome. But a polished pair past a span's end by less than its reach is not rare: about 6 distinct poses in three sweep tests, recurring at every ε, which main decides and the head escalated. They are near-tangent and steep poses, where `sin φ` is small and the reach is long.

**Ruled: (C).** An Off past a span's end prunes when:
1. `p` is decided within `e`, with the sign of `d(p)` known;
2. the basin certifies that `d` is strictly monotone on the closed stretch from `p` to that end;
3. the end candidate there reads `|d(end)| > e` Positive, on the same sign side.

Then `|d|` grows from `p` through the end and into the span for as far as the basin reaches, so nothing in the span within the basin meets. This is a decided reading of the pose, not a premise about the polish. Otherwise the reach test serves, then escalation. Straddles apply the test at each end.

Census gate: sweep re-run at three ε; decided → escalated against main is 0, or each one is explained as outside the basin.
