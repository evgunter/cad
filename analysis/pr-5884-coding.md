# PR 5884 dual review — blind coding inputs

The A/B labels were assigned by a recorded byte; the mapping is held by the orchestrator, not in this file.

Frozen head: b0d6040ca5360c5f45ffda3338b887c20769866b (merge-base 2d0d7307). Arm: holdout (a concurrent pair), letter M. Both reviewers were dispatched at 23:59Z with one brief. Neither was interrupted, and both report that lane isolation held.

## Review A

FULL review (holdout arm), PR #5884 @ b0d6040c vs merge-base 2d0d7307. Probes on the reviewer's probe branch (<sha> probes, <sha> an env-guarded trace of `apart` pushes). Lane isolation kept: I read no other review branch, no PR comments, no scratch. Disclosure: one shared-target-dir run mixed merge-base and head artifacts and briefly made the merge-base look green on perf12. I discarded it and re-ran both after `cargo clean` of every workspace package. The numbers below come from clean runs only.

Dispatch correction (NOTE): the file is `crates/topo/src/census.rs`, not `boolean/census.rs`.

**MAJOR: none.**

**MINOR**
1. [not executed] `census.rs` `curved_flush_at_rest`, the `edges` closure: `face_edges(..).unwrap_or_default()` turns a face whose loops don't walk into an empty edge set. That gives `!shared`, so no row and no finding: a silent drop. Two other paths are silent too, where the planar half is loud:
   - a Door 1 result in band, or an `Err`, just `continue`s;
   - the planar half gives an in-band SameOriented pair a `FLUSH_UNDECIDED` (via `aligned_unread`), and the curved half gives it nothing.
   The census's arm 1 probably stays loud for these, but the flush door does not.
2. [executed, 3 ε] Sub-zero lenses read `Ok(true)`, i.e. interference.
   - Probe: arcs of radius 2–100 escalation widths, crossing 0.1–3 widths apart.
   - At r=100w, lenses of 0.056·zero, 0.10·zero and 0.22·zero read `Ok(true)`, from both `face_interiors_meet` and `arc_interiors_meet` alone, at 1e-6, 1e-9 and 1e-12.
   - At r=20w the reading is not monotone: a lens of 0.28·zero reads `true`, and one of 1.13·zero reads unread.
   - The wrong answer goes the loud way (a false interference, never a false clear), but it contradicts the PR body's measured claim ("lenses 1 to 10ε read unread, deeper than 10ε read true"). That claim holds only at the test's r=1000w.
3. [executed] The planar flush half still never reads an arc-only shared run.
   - Fixture: rod in a tube. The rod's top disc and the tube's top annulus lie on one plane, face the same way and share the circle r=1. Result: `flush_rows=[]`, `flush_findings=[]` at every ε.
   - The cause: the planar half needs a straight shared side (`parallel_overlap`).
   - This is pre-existing, but it is the item's own shape (aligned arc-bounded faces that only abut, never read flush), it is silent, and the closing text implies it is covered. It should be filed or scoped out explicitly.

**NOTE**
4. [executed, 3 ε] Claim 1, near-tangent: never falsified. `Ok(false)` came out only for lenses ≤0.0078·zero, across r = 2, 5, 20 and 100 widths. Radii below 2w are refused at construction (`mev` ParamSpan). The zero band is a fair yardstick here: a merged lens is a sliver at most a crossing width long and under zero thick, which is the `[coincidence]` meaning of zero.
5. [executed, 3 ε] Claim 1, overlap shapes. I built 13 overlapping pairs, and every one reads interference or uncleared, never clear: a wedge straddling the full band's seam; a wedge straddling the half-band joint; reflex wedges and reflex lunes (more than half way round); a shared ruling that stops part-way; a hemisphere against a lune; a cut ball against a hemisphere; tubes, i.e. faces with holes; pins and half pins overlapping by 1000ε. I checked every flush row in these fixtures, and each pairs one copy's start rectangle with the other's end rectangle, which share only the axis. Correct.
6. Rustdoc soundness, checked step by step: it holds, including the clopen argument and "unless C is the whole carrier". The whole carrier needs a face with no edges, which is `Unbuilt`. A full sphere or band carrying only a seam reads its seam pieces with a `Cone::Full` cone, so it reads `true`. One coupling is unenforced: a shared run's ends are cut only by the vertex `contact` read, while the run comes from `meet`. If the two readers disagree at a run end, one piece straddles the run and the off-run part, and its midpoint classifies only part of it. Not executed.
7. [executed, 3 ε] Claim 2, the axial shortcut, is sound. A face's interior lies strictly inside its boundary's span, because height along a ruling has its extremes only on the boundary. `boundary_axial` is an outer enclosure, and unclaimable edges give `None`, which goes to the full reading. Pins overlapping by 0.5ε give 4 flush rows (a Zero overlap, a sliver under zero); by 3ε, `FLUSH_UNDECIDED`; by 1000ε, interference.
8. Claim 3, arm 1 [read, not executed]: arm 1 asks a per-pair question. An `apart` pair is one carrier with disjoint interiors, and every other face of the two solids is its own pair, so the skip loses nothing. `apart.contains` is a linear scan of a `Vec` inside the O(n²) loop: perf only.
9. [executed] Claim 4, perf12 goldens. Head at 1e-6, 1e-9 and 1e-12 gives three identical outputs. Against head's committed goldens they differ only at `kitchen_sink` line 48/64 (28 vs 29). The merge-base, clean-built, fails at line 48 too, so that red is inherited. The head trace shows `apart` firing only on `cut_cylinder`'s Face2/3 × Face6/7, the 4 wall pairs. So the 14→10 findings and 0→2 rows are as described. origin/main's tip also fails at line 48. `asm_r2b` at (1,8,9) passes at all three ε.
10. Claim 5, condition 3: I walked replay `reads_copies` and `carrier_verdict` (`carrier_pair_decided`, kind-generic), `checks/at_rest.rs:222` (relation only), `coincide.rs` `stated_by_a_mate` (`planes_equated`), the step-import pass-through, and pncad-py `tags.rs`. A grep for `CensusAtRest` and `Relation::SameOriented` found no reader the PR missed. As for planes, the replay re-proves only the carrier relation, never that the interiors are apart.

## Style
Questions exercised: Q1–Q6. Q7 folded into the items below. Q8 skipped: `census.rs` is about 20k lines, so I read the diff and its callees.
- Q1 (sure): `census.rs:10571` `flush_curved_kind` is a third copy of the cylinder-or-sphere kind list. The copy at `:5994` admits it "mirrors" Door 1's arms; this one says nothing. `glue.rs:226` is another to check.
- Q1 (sure): the flush row literal appears twice (`census.rs:10516`, `:10645`), and the shared/escalated edge-pair loop is written once for planes (`parallel_overlap`) and once for curved faces (`interiors_meeting`). The unread mapping in the two halves has already drifted (item 1).
- Q1 (likely): `face_edges` → `BTreeSet` is built in both `arc_interiors_meet` and `curved_flush_at_rest`.
- Q2 (likely): the soundness rustdoc is longer than the code, and its premise rests on two readers agreeing (`meet` and `Span::contact`), which nothing checks (item 6).
- Q3 (likely): `arcs_crossing_twice_near_a_tangency_are_never_read_apart` runs only at r=1000w, so it cannot see the small-radius regime where its own description stops being true (item 2).
- Q4 (unsure): `CensusTrace::aligned` now counts certified-apart pairs as examined and not accepted. `pins_stacked…`'s `accepted.len()==examined.len()` premise survives only because the spun copies share interior.
- Q5 (sure): the PR body's lens measurement is stated as general, but it holds only at one radius (item 2).
- Q6 (sure): the arc-only planar flush gap is undisclosed and unscheduled (item 3).

Verdict: **APPROVE-WITH-FIXES**: 0 MAJOR, 3 MINOR, 7 NOTE (plus the dispatch correction).

## Review B

ATREST review (holdout, full) of PR #5884 @ b0d6040c: APPROVE-WITH-FIXES — 0 MAJOR, 1 MINOR, 7 NOTE.
Probes on the reviewer's probe branch (<sha>) (census.rs `arc_interiors::probe_*`, a sweep probe test file). Merge-base 2d0d7307. Lane isolation kept: I did not read any other review branch or PR comments, and glimpsed none. I fetched only main and the PR branch.

### Correctness
1. MINOR (executed, 3 ε; behaviour predates this PR, but this PR's doc and test overclaim). Two arcs crossing twice with a lens 0.05–0.9ε deep (inside the zero band; r = 10–1e5 widths) read `face_interiors_meet == Ok(true)`, i.e. shared interior. The new piece reading itself gives `Err(Undecided)` here (stretch escalation). The `Ok(true)` comes from main's unchanged `shared_interior_witness`. Effect: an in-zero overlap is read as SameSideInterference, a false interference, not a false flush. The new rustdoc says "either answer certified". The new row `arcs_crossing_twice_near_a_tangency…` asserts `!= Ok(true)` only at k=1; at k=2..20 (lens 0.01–1ε) the reading is Ok(true) and the row still passes. The PR body's measurement summary does not mention it. Fix: pin or own it in the row and the doc, and file an issue for the witness reading inside the zero band.
2. NOTE (executed). Condition 1 holds. No lens deeper than the zero band reads Ok(false): r = 0.6–1e5 widths × depth 0.3–50ε at 1e-6, 1e-9 and 1e-12; radii below about 3 widths cannot be built. The closed forms read a touch on the harmonic's extreme in metres; the `ringed` merge (anything within K·ε) can only hide a lens deeper than ε at radii below about K·ε/8. The zero band is the right yardstick: every point of a lens under ε thick lies within ε of the other face's edge.
3. NOTE (checked by hand). The clopen argument holds. Three wording gaps in the rustdoc: (a) the face's own vertices are cut points but not meetings; continuity covers them. (b) Pieces skipped by `pm_census_arc_piece` appear only in a code comment, not in the argument. (c) "unless C is the whole carrier" should read "the carrier less finitely many points" (a full-sphere face with pole vertices). In practice seams read `Cone::Full`, and a face with no edges is Unbuilt. Probe "ball and its mirror" reads 1 interference.
4. NOTE (executed). Overlaps never go silent; each reads exactly 1 interference at 3 ε: self-mirrored half cylinder, lune, ball, hemisphere and cut ball (reversed half-edges); reflex wedges a half turn apart; reflex lunes a quarter turn apart; a half-height half cylinder (a shared run stopping part-way); a 0.01-rad wedge inside a half; cut balls flipped; caps spun; an annulus cap against a disc (a face with a hole). Every flush row printed is a genuine abutting pair. Planar arcs: identical halves, reflex against half, sub-runs, straddling a vertex: all Ok(true) both ways; abutting pairs Ok(false). Not executed: a disconnected face, and a full circle with no vertex.
5. NOTE (executed). The axial shortcut is sound. The height has no critical point on a cylinder, nor on one tilted within Door 1's Zero. `boundary_axial` is a superset hull; a NURBS edge is Unclaimable, so None, so not `apart`. Full and half pins at overlaps of −3…1e4 ε, 3 ε: interference exactly at k ≥ 11 (past K·ε); loud in the gap; flush/apart at ≤ 0.9ε.
6. NOTE (by reading). Arm-1 `apart` skips only that one face pair; every other face pair is still read. No path hides interference through another face.
7. NOTE (executed). The re-pins are what they say. perf12 blessed on head and merge-base at 1e-6, 1e-9 and 1e-12 and diffed: only `cut_cylinder` moves (14→10 findings, 0→2 rows). `kitchen_sink` reads 28 on both trees (inherited). asm_r2b, sweep curved_copies and the topo arc_interiors/flush/same_side rows are green on head at 3 ε. One more inherited red: sweep `review_4781_r2_probes::r2_every_same_side_touch_pose_answers_as_a_touch` is red at 1e-12 on both trees.
8. NOTE (by reading, unsure). Condition 3, walked: replay `reads_copies`/`carrier_verdict`; `coincide::stated_by_a_mate` via `planes_equated`; `checks/at_rest::constant_pairing`; product.rs. Missing from the PR's list: step-import `gate3_findings` (lib.rs:990) turns any `flush.findings` into a TierInvalid import failure. The curved half adds FLUSH_UNDECIDED/UNREAD findings that flush_at_rest never emitted for cylinders or spheres. Not proven whether such a pair can occur while the census is otherwise clean. Also unstated in the PR body: coaxial pins stacked end to end now emit flush rows, so `unproven-coincidence` reports for every stacked wall pair.
9. NOTE. DESIGN `[coincidence]` defines flush as "boundaries meet"; the code requires a shared run, so a meeting only at the poles gets no row. The PR body's "matches DESIGN" holds for "one carrier" only. Also, the curved half drops a Door 1 in-band pair silently; the planar half reports FLUSH_UNDECIDED with `aligned_unread`. Not verified whether arm 1 is loud for such a pair.

### Style (Q1–Q5, Q7; Q6 found nothing unscheduled; Q8 partial)
- Q5 (sure): the brief's `crates/topo/src/boolean/census.rs` does not exist; the file is `crates/topo/src/census.rs`.
- Q1 (sure): a third copy of the cylinder|sphere kind list: census.rs:5994 and :10571 `flush_curved_kind`.
- Q1 (sure): `curved_flush_at_rest` re-spells the planar flush loop (group gate, Door 1, shared/escalated loop, verbatim `crate::Coincidence {..}` literal).
- Q1 (sure): `apart` is a Vec scanned in both orders, beside `decided`, a symmetric BTreeSet. O(n) per arm-1 pair.
- Q1 (likely): `face_edges(..).map(e.edge).collect::<BTreeSet<_>>` is written three times.
- Q7/perf (likely): `curved_flush_at_rest` re-evaluates `edges(fb)` on each pass of the `ea` loop and `edges(fa)` per candidate, and re-runs `face_interiors_meet` on pairs the census already read.
- Q3 (sure): see MINOR 1. The near-tangency row cannot go red on an in-band lens read as interference.
- Q2/Q5 (likely): FLUSH_UNDECIDED's advice ("move onto the surface") is wrong when Door 1 was Definite and only an edge-meeting reading escalated.
- Q4 (likely): `face_interiors_meet` and `shared_interior_witness` claim "either answer certified", but the witness answers first and is the source of MINOR 1.
- Q2 (likely): the soundness paragraph is longer than the code; the skip rule and own-vertex cuts are justified only in a code comment.
- Q7 (unsure): `along` is an insertion sort under a non-transitive tolerant comparison; harmless, but odd.
- Q8 (unsure): census.rs is 20.3k lines; changed regions and neighbours read only.
