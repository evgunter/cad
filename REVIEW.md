# Review: JOIN-2 fix pass 1 (PR 3880, delta 17c254c99d..4c18c2cbe)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 5.
The delta's own commits are 064c66cf6, 3722698ce, 3107047c, e75501d86 and 4c18c2cbe (first-parent, merges excluded). I read the dual review through `git show` and the PR body through the API. I read no other delta-review branch and no PR comments.

Instruments:
- **Battery** `crates/sweep/tests/join2_d_probes.rs` (ignored rows, my own shape, 1680 lines). Rounded or sharp plates stacked flush (REST). It covers:
  - combs whose prong corners are pierce rings in one face, with bases on, at and past a fillet;
  - a plus sign whose hub corners are rings reached from two sides;
  - fillets of radius ra/rb tangent at one point, concentric and equal arcs, concave notches, and two rounded plates.
  - Every pose runs U, A∖B, B∖A and A∩B in both orders through `differential::outcome`, against a closed-form oracle (shoelace plus segments).
- Main 82b9ceb2b and head were built release in separate target dirs, and the sorted lines were diffed.
- **Mutants** M1/M1b/M2/M2b/M3/M3b. Each ran nextest `-p topo -p sweep` (4215 rows) plus the battery and the reviewers' tie and ring probes, in debug.
- **Instrumented build**: `eprintln` at `germ_loci`'s `paired` arm and at its tie arm, over the whole sweep suite and the batteries.
- **Head suite**: topo + sweep 4214/4215. The one red row, `every_suite_file_is_aggregated`, was caused by my battery file landing mid-run.

## Claims

1. **Span order: holds.** Executed.
   - Argument: a vertex becomes joined only when a span with exactly one unjoined end is realized. So every such span is taken before any span with both ends joined (the only kind that can divide a face with `mef`), and no ring is left on the old face.
   - Battery (comb, plus; r = 0.5, 1; both orders): 0 BAD, 0 SOUND→refusal. 56 unions go refusal→SOUND (38 were `ChordBetweenIsolatedPierces`, 18 `UnpairedLooseEnds`). No line at head refuses `ChordBetweenIsolatedPierces`.
   - Mutants:
     - **M1** (`unjoined(i) >= 1`, the old ring-first order) turns two rows red: `unions_through_ring_vertices_and_like_far_ends_build_sound` and `a_channel_whose_arm_ends_join_two_ring_vertices_builds_sound`.
     - **M1b** (spans with both ends joined first) turns four rows red, adding `mate2_r1_probes::probe_misaligned_azimuth_split_unions` and `mate2_r2_probes::r2_full_period_peg_unions`. 31 battery lines go SOUND→refusal (`NotSameFace`, `SectionInvariant`), with 0 BAD.
2. **Like-far-ends tie: holds; a wrong pick fails safe.** Executed and read.
   - Midpoint as witness, for a line or circle: the germ is tangent to the plane at the site, so a circle that also passes through its midpoint in the plane lies in the plane (tangency counts twice). Leaving and re-entering the trim needs a crossing of the partner's edges. The doc says the sweep splits at those; no row exercises the case.
   - **M2b** (picks the edge *outside* the trim) turns both tie rows red, and 42 lines go SOUND→refusal (`UnpairedLooseEnds`) with 0 BAD. So a wrong choice refuses at the join; it does not ship a body.
   - The plane band is absolute (`Margin::of(height)`, `sectors.rs:1049`), like `bool_arc_chain_on_circle`.
   - A curved partner face, or a trim the walk cannot read, gives `None`, which leaves the pre-fix `OnEdge` both, a refusal.
   - Instrumented: the tie ran 84 times in the batteries and 22 times in the suite. Every time, exactly one edge was inside. The both / neither / `None` arms are reached by no row (NOTE-1).
3. **`coincide`: sound, but vacuous.** Executed.
   - Two distinct circles tangent at one point meet nowhere else, and neither do a line and a circle tangent at a point. So when the far ends are paired, a line or circle edge always coincides with its partner.
   - Instrumented: the `paired` arm ran 2604 times in the sweep suite and 2252 times in the batteries, and `coincide` returned `true` every time. See MINOR-1.
4. **No regression: holds.**
   - Battery, main → head: SOUND 606→662, EMPTY 208→208, ERR 866→810, BAD 0→0. SOUND→refusal: 0. New BAD: 0.
   - R1's grid (31 104 lines; on main, `boolean_join_refusal` was replaced by a stub string): SOUND→refusal 0, refusal→SOUND 872.
   - The **804 BAD lines (`t3p=false`, volume right) are the identical set on main and head** (diff of the BAD lines is empty). The grid's `bad == 0` assertion is red on both trees.
   - 32 grid lines move from refusal to refusal (`ChordEndpointRevisited`).
5. **Rows can go red: falsified for `coincide`, holds for the order and the tie.**
   - M1, M1b, M2 (tie reverted to `OnEdge` both: 2 rows red) and M2b are covered above.
   - **M3** (`coincide` forced true, the pre-fix behaviour) and **M3b** (forced false) each run 4215/4215 green, and the battery and probe lines are byte-identical to head.
6. **Style refactors are behaviour-free: holds, with two caveats on loudness.**
   - `edge_vertices` reads the same two starts.
   - `sole_common_face` counts the same intersection. Order is irrelevant there, and `faces_at` dedups.
   - `on_faces` / `vv_sides` select the same fields as the inline matches they replace.
   - `Twin::of` gives the same `start` whenever the twin's ends match. It now fails `JoinDesync`, and does so before `TwinCarrierUnsupported`.
   - `faces_at` differs from the old `incident_faces` in three ways: it expands the null site, it skips null-scaffold edges, and it *silently skips* a half-edge whose face does not resolve where `incident_faces` desynced. The first two are inert once `read_segments` has undone the struts. The third is a loss of loudness (S6). Battery and suite line counts are identical either way.

## Findings

**MINOR-1 — `coincide` decides nothing any row can see** (`sectors.rs:948`, `981-1015`). Executed: M3 and M3b, plus instrumentation.
- No row, battery line or probe line moves whether `coincide` is forced true or false, and it never returned `false` in about 4.9k evaluations.
- By the tangency argument in claim 3, its answer is fixed for the carriers it reads. Every other carrier gets "no".
- That "no" path is not covered either: an identical NURBS or ellipse pair whose far ends are paired falls through to `Touch`. If the two far ends record differently (one also `vf`), one edge is read tangent and `tangent_face` would host a chord along an existing edge. I could not reach that shape, so whether it refuses or not is *unsure*.
- r1 MINOR-2 asked for an identity whose failure is visible; the fix pass added a branch with no row, and the PR body names none.

**MINOR-2 — `runs_in` folds corruption into "undecided"** (`sectors.rs:1045`, `1056`). By reading.
- `let Ok(..) = face_plane(..) else { return Ok(None) }` and `point_in_face(..).unwrap_or(None)` turn `CorruptFace`, a stale loop and `EdgeCarrierUnsupported` alike into `None`.
- `None` then means `OnEdge` both, and the join refuses. So there is no wrong body, but a kernel invariant break reads as an ordinary refusal. CLAUDE.md asks for fail-loud.

**NOTE-1** — The tie's both / neither / `None` arms are reached by no row (instrumented). The claim that they "stay `OnEdge` both, only a refusal" rests on reading the code, plus M2's red rows for the reverted arm.

**NOTE-2** — `ChordBetweenIsolatedPierces` now has two raise sites, `rest.rs:933` (new) and `rest.rs:1266` (`mint_chord`, still reachable from `mirror_edges`). Neither is reached by the suite, my battery or the grid at head. `rest-zip-frontier-refusals-reached-by-no-row` lists the frontier but not the new site.

**NOTE-3** — The grid's 804 BAD lines are main's and are filed. Confirmed identical; not this PR's.

**NOTE-4** — The tie rows pin the chosen edge only through SOUND outcomes. M2b shows a wrong pick refuses rather than ships, so the rows guard a gain, not soundness.

**NOTE-5** — I did not exercise `topo`'s own suites under instrumentation, only under the mutants' nextest runs.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q6, Q7. Q8 only partly: I read `rest.rs`'s module doc and §4, and `sectors.rs` 900-1215, not either file end to end. Q5 was read against `rest.rs`'s header.

- **S1 (Q1/Q7)**, *sure*. `realize_seam`'s closure `joined` (`rest.rs:922`, "has any edge") shadows the module fn `joined` (`rest.rs:1076`, "an edge joins u to v") in the same file.
- **S2 (Q1)**, *sure*. `edge_mid` (`sectors.rs:972`), `coincide` (`986-996`) and `runs_in` (`1048`) each spell `get_edge → get_curve_geom → certified`. The message "a germ edge has no certified curve" is written twice. The fix pass that unified `edge_vertices` minted this copy.
- **S3 (Q1)**, *likely*. The line miss `(p - origin).cross(dir).norm()` (`sectors.rs:1000`) has no shared home, while `circle_miss` is documented as "the one spelling" for circles. `coincide` is also a third point-on-carrier test, beside `contain.rs` `point_on_circle` and `reduce.rs` `bool_arc_chain_on_circle`.
- **S4 (Q4)**, *sure*. `solid_contain::face_plane`'s doc says "Its one external consumer feeds the normal to `point_in_face`" (`solid_contain.rs:578`), but `runs_in` is now a second. *Likely*, class: `runs_in` picks one of three face-plane spellings (`reduce.rs:512`, `solid_contain.rs:582`, `chord_join.rs:3680` `face_plane_normal`).
- **S5 (Q3)**, *sure*. `coincide` cannot go red (MINOR-1). It is a new branch with no row, in a pass whose brief was to make the reviewers' points visible.
- **S6 (Q2/Q7)**, *likely*. Unifying `faces_at` with `incident_faces` relaxed fail-loud in the REST lane (`sectors.rs:1204`, `if let Some(f)`). Look also at `tangent_face` and every other `faces_at` caller.
- **S7 (Q6/process)**, *likely*. `two-tangent-edges-parting-…` carries `design: true` ("trim or curvature", its own words). It was decided and closed inside a fix pass, and the flag is still set on the closed row. `work/README.md:90` says a `design: true` row is weighed by designers before a lane builds it, and I found no designer record.
- **S8 (Q2)**, *unsure*. `runs_in`'s doc ("the sweep splits an edge at every crossing of the partner, so the midpoint speaks for the whole edge") asserts an upstream invariant that nothing at this site checks, and no row has an edge re-entering the trim.
- **S9 (Q3)**, *likely*. `contact8_dangling_seam.rs:304` pins the join's refusal as `JoinDesync`, an invariant-break variant, as the expected shape. A real desync passes it.
- **S10 (Q7)**, *unsure*. The span order is still a priority rescan by counts, rebuilt each pick (O(n²); r1 S10 not taken). A worklist grown from joined vertices would make "outward" structural rather than emergent.
- **S11 (Q1)**, *likely*. One frontier is raised at two sites (`rest.rs:933`, `1266`) with one display text, so a refusal does not say which site fired.

REVIEW COMPLETE
