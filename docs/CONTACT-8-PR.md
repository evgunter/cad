# CONTACT-8: the merge prunes dangling seam edges; a boolean never ships an unglued planar group

Carries `work/contact/area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step`.
Builds the design Ev ratified on PR 3350, as `docs/DESIGN.md`'s
"Maximal-faces precondition and the merge stage" states it.

## What changes

**S1: prune by topology** (`merge_group`, `crates/topo/src/merge_faces.rs`).
After the absorption joins a planar group's faces, the intra-face pass
repeats one step until no doubled edge is left:

- the first doubled edge (edge-arena order) that has a **free end**, a
  vertex with no other edge, is deleted together with that end by
  `kev(half running toward the free end)`, and the end is recorded in
  `MergedGroup::killed_vertices`;
- an edge with BOTH ends free is a ring holding nothing but itself
  (the last edge of a doubled cycle, such as a plugged hole's rim):
  `kev` takes one end, leaving the ring a lone vertex, and
  `mekr_chord(EmptyRing)` then `kev` delete that vertex with its ring —
  both ends recorded, the bridging edge minted and killed within the
  step. If the arena says that ring is the survivor's outline, the
  shape refuses `UnsupportedConfiguration` before any of the three
  calls;
- only when no doubled edge has a free end is the first one handed to
  `kemr`, which mints a ring (or refuses `UnsupportedConfiguration` if
  its halves are in two loops, as before).

The free-end test is `strut_tip`, which reads the vertex orbit and
announces a broken one. A shared chain of `k` edges loses its `k − 1`
interior junctions one free end at a time, and pruning runs again after
every ring, so a chain that only dangles once a ring is cut off is
pruned too. The argument is stated once, at the site (the door's docs
and `killed_vertices` point there): a dangling edge inside a face
encloses no area, nor does a lone-vertex ring, so deleting them and
their free ends leaves the merged face's region exactly as it was. `kev` on a free end re-bases no
fan, so no carrier is left certified against a dead vertex.

Deleted: the `straight_seam` pre-decision, its "exactly two shared
edges" guard, `Body::redundant_subdivision_vertex` with its two
`decide` sites (`merge_seam_collinear`, `merge_seam_opposed`), the band
and the escalation arm. The function had one caller.

**S2: a planar group the merge cannot glue refuses the call**
(`group_contract`). Every planar group, structural or declared, runs
under `GroupRegime::RefusesTheCall`; only a curved group records a skip.
All four boolean output stages (`boolean::ops`' seamed stage,
`fallback`'s graft and `finish_fallback`, and `boolean::rest`) map the door's `Err` to
`BooleanError::Merge`, so the step refuses with the merge's own typed
reason. `MergeRung` and the door's declared-face set existed only to
pick the regime and are gone; `planes_declared_equal` answers `bool`.

**No reachable planar shape refuses after S1.** I looked: the corpus
this unit runs (the reproducer, the L-corner, the four-way junction,
the three-edge chain, the bigon, the seam around a hole, the plugged
hole three ways) all merge, and the review's 22 unions (8 at an
oblique pose) all merged. The regime is pinned by the unit rows on
the regime itself (`planar_runs_refuse_and_curved_runs_record`, and the
declared nested membrane refusing `SameFace` at the door). The refusals
left on the planar surgery path are typed by the surgery itself
(`UnsupportedConfiguration`, `MergedFaceRoleAmbiguous`, an operator's
own `Op` refusal), all on the staged clone; the post-run tier-2 gate
(`ResultNotClosed`) is reached by no shape I know of. No refusal
text was added, so no shared ending was due.

**S3: records.** Nothing changed in `remap_contacts`: a vertex the
pruning deletes is dead, has no descendant, and every lane (`vert`,
`vert_strict`) drops a record citing it. Records cannot reach it
through a boolean by construction: a pruned vertex is a junction of
the seam the zip laid in the merged plane, so a vertex-on-face rest
there was fused (and dropped) by the zip, and a vertex-vertex record
there names the other operand's coincident vertex, which the zip fuses
into it (consumed). `a_union_f` ships no records at all, and its row
says so. What the merge could still get wrong is the descendant chase,
so `a_record_citing_a_pruned_free_end_drops` runs a REAL pruning merge
(a prism top split by a bent seam), absorbs its outcome into
`Descendants` as the boolean does, adds the fusion row a zip would
write into the deleted vertex, and requires a v-v, a-on-b and b-on-a
record citing either key to drop, with a live-vertex control. A
`Descendants` that mapped the killed vertex to a survivor turns it red. The pruning never
fuses vertices and never removes a boundary vertex: `kev` is only
called on a vertex whose orbit has one member, and that one edge is a
doubled edge of the survivor, so the vertex bounds no region. No case
contradicting the clause turned up.

## S4: the history check

`git log --all -S'redundant_subdivision_vertex'` finds it born in
`5d6851712` (PR #1131, 2026-08-28, "F7 pole half") and documented in
`66d3f9537` (its delta-review fix pass). PR #1131's body and its two
comments give one reason for collinearity: an earlier valence-only
trigger "was falsified by `merge_skip`'s brick flush caps, whose L-shaped
seam has exactly that shape" — that is, the L-corner row that pinned
the skip as expected behaviour. The corpus table in that PR lists which
fixtures the trigger fired on; it cites no wrong result for a bent seam,
and the licence prose ("the union of the two collinear pieces is the
same locus") describes a deletion in which both seam edges die anyway.
The withdrawn gate exemption (`d416dbcb1`, `fcd05d2f5`) is a different
mechanism, at the F7 gate. **No reason other than the test pin turned
up**, so the licence goes, and with its only caller gone the function is
deleted.

## Rows

| row | base (`d3a962cdb`) | head | reason |
|---|---|---|---|
| `merge_skip::declared_l_corner_caps_merge_and_stay_tier3_green` (was `skipped_declared_merge_is_tier3_green_and_visible`) | both cap groups skipped, `GroupNotClosed`, 12 faces | both merged: 2 `merge_groups`, `merge_skipped` empty, 10 faces, volume 1.625, tiers 2/3/3′ green | S1: the corner is the free end of the dangling second seam edge |
| `f7d_delta_probes` D1 (`d1_collinear_and_bent_seams_both_repair`) | collinear repairs; bent refuses `ResultNotClosed{ScaffoldingEmptyLoop}` | both repair, `(f, v, e)` − `(1, 1, 2)`, tier 2 and 3 green | S1: the angle decides nothing |
| D2 (`d2_no_band_decides_the_repair`) | 0.1× zero repairs; 5× `Escalated{merge_seam_collinear}`; 10⁶× `ResultNotClosed` | all three repair | S1: the band and its escalation arm are gone |
| D3 (`d3_a_four_way_junction_repairs`) | `ResultNotClosed{ScaffoldingEmptyLoop}` | repairs, `(3, 1, 4)` | S1: three `kef`s leave the fourth spoke dangling from the centre |
| D4 (`d4_a_zero_width_bigon_is_absorbed_with_its_tip`) | `ResultNotClosed{ScaffoldingEmptyLoop}` | repairs, `(1, 1, 2)` | S1: once the zero-area bigon is absorbed its tip is on no boundary |
| `verbs_f7_collinear_seam` | printed `Ok(1)` / `Err(ResultNotClosed)`, asserted nothing | asserts both bodies repair identically | the differential it printed is now a claim |
| `verbs_f7_r2_probes` mid-vertex, two-mid chain | gate `NonMaximalFaces`; merge printed `Err(ResultNotClosed)` (chain: not run) | gate unchanged and now asserted; merge repairs, `v − 1` and `v − 2` | S1; the two-mid chain is the three-edge seam chain row |
| `verbs_f7_r2_probes` control, `review_f7_pole_r1_probes` P1–P4 | gate refusals | unchanged | prose only: they pin the F7 gate, not a trigger |
| `sweep` `f7_pole_split_cap_repairs_to_one_face` | passes | passes, unchanged code | the straight seam is the one-edge case of the pruning |
| `sweep` `f7d_delta_probes`, `verbs_f7_r2_probes` | — | prose only | |
| `merge_faces` unit: `planar_runs_refuse_and_curved_runs_record` (was `the_planar_fixtures_take_the_two_regimes`) | declared planar cube → `RecordsASkip` | declared planar cube → `RefusesTheCall`; curved cube → `RecordsASkip`, records `PeriodClosure` | S2, pinned on the regime itself |
| unit: `every_contradicted_fact_escapes_the_recording_regime` | 5 facts on declared planar fixtures | 4 facts on curved fixtures (a curved survivor refuses `PeriodClosure` before `kev`/`kemr`) | S2 moves the recording regime to curved runs only |
| unit: `every_contradicted_fact_refuses_the_refusing_regime` | 5 facts | all 6: `kev` now reached on the cube's pruned seam tree, `kemr` on the new split ringed top | S1 makes the `kev` tear reachable |
| unit: `a_declared_planar_group_the_merge_cannot_glue_refuses_the_call` (was `the_door_records_same_face_as_a_skip`) | nested membrane group records `SameFace` | refuses the call with `SameFace`, body untouched | S2 at the door |
| unit: `an_ok_carries_a_recorded_skip_beside_the_placeholder_census` | declared nested cube's `SameFace` skip | curved cube's `PeriodClosure` skip | S2: only curved groups record |
| unit: `a_seam_with_no_free_end_mints_a_ring` | — | new | `kemr` still mints the ring a seam with no free end separates |
| `boolean::ops` unit: `a_record_citing_a_pruned_free_end_drops` | — | new | S3 |
| `contact8_dangling_seam` (6 rows) | see below | new | the reproducer end to end, the hole, S2 |

The new suite at base and head:

| row | base | head |
|---|---|---|
| `an_area_overlap_cap_contact_publishes_one_cap_top_and_bottom` | both cap groups skipped (`GroupNotClosed`), 0 `Merged` rows | 2 `Merged`, one cap each height, 10 faces, volume 1.75, tiers green |
| `the_merged_union_takes_a_third_brick_declared` | refuses `UndeclaredCoincidence` on the same-operand pair `(A, A)` | runs, no skips, 8 faces, volume 2 |
| `the_merged_union_refuses_an_undeclared_third_brick_across_operands` | the same same-operand `(A, A)` pair, which no declaration can cover | `UndeclaredCoincidence` on `(A, B)`: the two bottom caps, a pair `flush_declarations` covers |
| `the_bent_seams_corner_is_deleted_and_no_record_survives` | the corner (1, 1) survives in the shipped body | corner gone; `contacts` empty |
| `a_bent_seam_around_a_hole_merges_to_one_ringed_cap` | both cap groups skipped (`GroupNotClosed`, two empty loops each) | one cap per height, one ring each, volume 9 |
| `an_exactly_plugged_hole_merges_to_whole_caps` | ships, both cap groups skipped (`GroupNotClosed`, four empty loops each) | 6 faces, volume 12, tiers green; next union volume 16, green |
| `a_half_plugged_hole_merges_its_bottom_cap` | ships, the bottom cap group skipped (`GroupNotClosed`) | 11 faces (bottom cap whole, pocket on top), volume 11; next union 15, green |
| `an_oversized_plug_merges_to_whole_caps` | ships, both cap groups skipped (`GroupNotClosed`) | 6 faces, volume 12; next union 16, green |

The plug rows at the first head (`448fea9d1`) all refused
`Merge(ResultNotClosed{ScaffoldingEmptyLoop})`: the pruning `kev`'d the
rim's last edge with both ends free and left a vertex-only ring.

No golden, render or committed baseline moved.

## Class sweep

Pattern: every production call of the merge door, and every consumer
of `SkippedMerge` / `merge_skipped` / `.skipped`
(`grep -rn "merge_coplanar_faces\|merge_skipped\|\.skipped" crates demos tools --include=*.rs`).

| hit | disposition |
|---|---|
| `boolean/ops.rs` seamed stage (`merge_coplanar_faces_declared` → `BooleanError::Merge`) | fixed by S2 at the door |
| `boolean/ops.rs` `fallback`'s assembly/void graft (declared pairs) | fixed by S2 at the door |
| `boolean/ops.rs` `finish_fallback` (one operand kept, no declarations) | structural only, refusing regime already |
| `boolean/rest.rs` rest door | fixed by S2 at the door |
| `boolean/ops.rs` `describe_minted_edges` reads `skipped` faces | not this unit: now only curved records reach it, which is what it re-describes |
| `splitting/reassembly.rs` | a test helper's explicit merge (structural, refusing regime already) |
| split products (`splitting/finish.rs`, `splitting/mod.rs`) ship same-key coplanar pairs | not this unit: ratified (M2, F7) — merging is never silent there and the caller opts in; the opt-in merge is a structural planar run, which refuses rather than skips |
| sweeps (revolve's pole-split cap) ship two half-discs on one key | not this unit: same opt-in; the pole cap now repairs through the pruning (`f7_pole_split_cap_repairs_to_one_face`) |
| `editor-core` never reads `merge_skipped` | not this unit; evidence added to `work/contact/editor-core-never-reads-merge-skipped.md` (only curved records remain) |
| `mesh/src/curved.rs`, `pncad-py` `ChecksReport.skipped`, `seqgen` tallies | other senses of "skipped", not merge groups |

What the pattern cannot match: a door that ships two coplanar
neighbours without ever calling the merge. Second pass, shaped at that
gap: every producer the boolean's F7 gate (`NonMaximalFaces`) is
documented to catch — `grep -rn "NonMaximalFaces\|coplanar" crates/*/src --include=*.rs -l`
and the "Coplanar artifacts" sections — gives the split and the
revolve cap above and nothing else that ships as a boolean output.

## Also touched

- `docs/DESIGN.md` is unchanged: PR 3350 already states the design.
- `work/zip/a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made.md`:
  a dated note that the collinearity licence it quotes is gone and the
  clause now forbids boundary-vertex removal (its vertices are boundary
  vertices, untouched by the pruning).
- `demos/tour/src/lily.rs`: the comment citing the deleted function.
- `crates/editor-core/tests/m4_pr5_declare.rs`: the recipe-door L-corner
  row is renamed `declared_l_corner_caps_merge_at_the_recipe_door_tier3_green`,
  asserts the merge (10 faces), and its F6 pure-seam-vertex pin states
  its premise on the merged caps (the wall-crossing vertices, not the
  deleted corner).
- `docs/KERNEL-VERBS.md`: the pole-half paragraph's licence sentence.
- `scripts/gates/loop-boundary-discards.sh`: one audited register
  entry for the pruning's bridge-target arm (an empty outline refuses
  `LoopNotCycle`, never passes over).
- `work/topo/D262.md`: a dated note that `redundant_subdivision_vertex`
  and its seven arms are gone.

## Not changed: the re-scan

Each pass of the intra-face loop re-scans every edge for doubled edges
and every doubled edge's orbit, so a group whose merge deletes `k`
edges costs O(E·k). The groups here are small and the scan is the one
the absorption loop already made; a worklist would be a second
bookkeeping of the same set, so it is left.

## Local results (fix pass, on the merge of origin/main)

- `topo` + `sweep`, nextest: eps unset 3300/3300, `1e-6` 3300/3300,
  `1e-12` 3300/3300.
- `editor-core` concision rows, perf12, docm6, m4_pr5_declare: 35/35.
  `test-utils`: 79/79.
- `cargo clippy --workspace --all-targets -D warnings`: clean.
- rustdoc (`-D warnings`, private items, `topo`): clean.
- every `scripts/gates/*.sh`, self-test and pass: green (the pruning's
  bridge arm is registered in `loop-boundary-discards.sh`, audited).
  `python3 scripts/work.py lint`: ok.
- First pass only: `demos/tour` clippy clean and nextest 89/89,
  `demos/wild` clippy clean; the fix pass touches no demo.

Hosted CI has not run; the branch is pushed without a PR.
