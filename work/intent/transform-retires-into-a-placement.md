---
id: transform-retires-into-a-placement
kind: issue
title: D10 stage 3 PR D: constructions read no frame and nothing moves a body; Datum, Transform and PlacedFrom retire, each use becoming a placement
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [a-mate-relates-two-poses]
refs: [intent-stage3-is-built]
---

INTENT stage 3, PR D. Spec: `docs/INTENT-STAGE3-SPEC.md` §5. Built on FORK-S3P (row 95, PR 4324) and FORK-S3M (row 97, PR 4326).

Frames enter only at placement:

- A profile is 2-D and reads no plane.
- `Tube` and `HollowTube` are built about their own axis.
- `Extrude` loses `side`: the side is the placement's mates' to say.
- `Node::Datum` retires.

Every construction is built in coordinates of its own. One combined with another body is placed against that body (the combine's first operand's root) by the restate ladder: a `Plane` mate to a parallel face at a `Standoff`, else `InFrame` of a constructed frame, else the migration names it and refuses to regenerate (spec Q8). Its values are computed from the two datums. A first operand, or a body never combined, keeps whatever world placement it had. Every value charted on a re-coordinated body is recomputed from the pre-D solve, so no solved world pose moves. No construction is placed against the world by this migration, so a cutter never joins the product. The last `Datum::FaceFrame` read as a frame goes with `Datum`.

Nothing moves a body. `Node::Transform` and `PortKind::PlacedFrom` retire, and each use becomes a `Place`. A copy's names pass through as `Transform`'s did. `Step::Literal` retires, and A6's admission runs on evaluated pose values.

An absolute pose read by a non-root operation (a `Split` tool) goes down the same ladder. Until D, every construction reading no body is of the document's one kind (spec §3's interim). D lands "a body's kind is its root". `Pattern` and `PlacedUnion` still place by rule until G. The `diefillet` pips' union is written against today's `Node::Union` and moves with FORK-DM4's units. The façade's "sketch on a face" writes profile, construction, placement and combine as one gesture. The tour's `chain`, `diefillet` and `teapot`, the viewer's transform gesture and Python's datum builders and `.transform` follow.

Closes the placement half of `a-boxed-rotation-refuses-not-rigid-at-every-placer`. Waits on C (`a-mate-relates-two-poses`), because a construction placed against a face needs a mate reading its plane and values for the freedoms it leaves.

## Handoff

(2026-10-19, INTENT wrapping up at low weekly usage. Kept current; the last bullet of each list is the newest.)

**Branch** `intent/s3-d-transform-retires-into-a-placement`. The head is the commit that carries this section or a later one. The draft PR body is the last subsection here ("Draft PR body"); move it to the PR when it opens.

**Done and on the branch**
- The kernel: `Datum`, `Transform`, `PlacedFrom`, `Step::Literal` and C's `WorldPin` retired; profiles are 2-D; extrude has a side; tube has no frame; a loft takes its first section plus stations (row 161 Q2). Every construction is the root of its own space and enters the world by `Node::Place`, through the helpers in `crates/editor-core/src/mate/pin.rs`.
- The fan-out: editor-core tests, the corpus, pncad, pncad-py, the viewer and the tour, all re-spelled.
- **origin/main merged in** (merge `ea25544a9a`): F, H, #5312, #5320, the loft stations and about 40 PRs in all.
- Box-lane solve fixes (566f1b6fdc) recovered 16 m10 rows. f64 is unchanged by them (the re-bless lane proves this against 991bd0c655).
- The C-5 Sym snap change, which the orchestrator ruled on.
- The superseded items, D′'s item (`a-placement-pins-by-written-values-without-a-mate`, with the §1 pick ruling from PR 5315), and the follow-up items filed.

**In flight at handoff (the last commit is a WIP sweep of three lanes' uncommitted edits)**
- **Kernel lane, box-lane solve, tour certified lane:** `src/mate/{pin,solve,value}.rs`, `src/eval/{mod,wire}.rs`, `src/eval/wire/pose.rs`, `demos/tour/src/chaintol.rs`. Status: the chain is recovered to its pre-D published fractions [1, .3702, .1851, .1110], all three chaintol tests pass in release, and the plate study is back to its published 211/512 after the fix in its step [6]. Still to do: re-run the plate study and the m10 rows on that final fix, then the m10 Interval rows (bulge ×3, derived_frame ×2, tilted, m10_5_r1, m10_6_r1, m10_8, m10_9 ×3, sym_drive_memo): restate the pin and premise moves, and fix any real loss. Log: `/home/user/kernel-tour-status.txt` (mechanisms A–D: a placed target read in its own coordinates; unnormalised chart axes; the settled map composed as a RigidMotion instead of Gram–Schmidt; the target read in the copy's own coordinates).
- **Re-bless lane:** goldens and stored documents re-baselined with each one's own bless mechanism. The log of every old→new value is `/home/user/rebless-log.md`; copy it into the PR body. **Refused, needs a call:** `name_words_corpus`, where the name-word budget grew (full max 196→366, full total 411230→444393).
- **Triage lane:** behaviour rows. Log: `/home/user/triage-log.md`. Done: the persist rows (re-frozen per DESIGN Band 4: a pre-D document refuses typed rather than migrating), kstats/asm2a counts, asm4's row deleted as unrepresentable, part_depth_bound, fix_pattern_mate_crossing. **Real gap, not applied:** inline at a pose refuses `ConstructionReadsCopy` for a copy in a construction's own space, which a world move doesn't touch. Proposed fix: restrict both checks in `refactor.rs` inline to copies in the World space. Repro: `/tmp/claude-0/-home-user-cad/25203a37-0c23-510d-944e-7fd83f81c887/scratchpad/diag/asm4_union_probe.rs`. Left: wire_rv_unknown (which clears with die_tool.pncad's re-bless), m6_composed_node, sym11, sym_9, revalue declared_tangency, coincidence die_composed_tour (14 of 42 proven, the row says 21), intent_s2_c_world, m10_di_dual.

**Rulings on the two open calls (orchestrator, 2026-10-19)**
- **`name_words_corpus`:** re-bless it at the measured values, without hiding it. The PR body names the corpus document that sets the new max (196→366) and why its names grew: placed-copy wrapping under D's `stand`. An item owned by D′ (`name-words-budget-grew-under-placed-copy-wrapping`) records that retiring `RoleSeg::Placed`/`InPart` (PR 5315) should bring the budget back, and that D′ re-tightens it.
- **Inline at a pose, for a copy in a construction's own space:** apply the fix in D if it stays inside inline/refactor and carries its own row. If it spreads, file it as a P1 item with the failing case and leave today's behaviour typed. Either way the row that shows it is never skipped or quarantined.

**Triage lane, finished (2026-10-19; log `/home/user/triage-log.md`)**
- All 22 rows classified and on the branch. Inline at a pose now carries a copy seated on a construction (`refactor.rs::inline`, `in_loose_space`), with its own row, `asm4_split_inline::inline_at_a_pose_carries_a_copy_seated_on_a_construction`. That leaves `InlineError::CopyPinnedToConstruction` with no reachable producer. It is kept, because it is in pncad-py's tags and the binding census; retire it in a follow-up.
- Left for the kernel lane:
  - `revalue_declared_tangency`: a real bug. `place_at` writes an antiparallel z as a spin by π, so the chart sits at the antipode by rounding. Proposed fix: spell it as a `Flip` on the copy's side.
  - `m10_di_dual_corpus`: die_composed_tour has 10 undecided touches at Interval.
  - `sym_9_retry_interval` ×2 and `sym11_exact_channel_rows`: receipts drift with the solve edits; re-take them once those settle. sym11's frozen column fell (two_hole_plate 729→161), which its text calls a finding.
- `intent_s2_c_world` stays red until the viewer's `gallery_ring.pncad` is re-blessed (then take `FILES[3]` with `print_product_rows`).

**Re-bless lane, finished (2026-10-19; log `/home/user/rebless-log.md`, copy it into the PR body)**
- **f64-neutrality proven.** 566f1b6fdc only removes scaffolding; the three box-lane fixes landed in 991bd0c655 behind env switches. With the fixes off, on, and at 566f1b6fdc, the left digests of `m10_p_fence` (f64 and ids masked) and of msolve14 a3 at ε 1e-9, 1e-6 and 1e-12 are identical. The guard digests moved only by D's re-spelling.
- **Re-blessed by each file's own mechanism, each re-pin explained in its file:**
  - `die_tool.pncad`, `wire_rv_bytes.json`, `mint_node_ids.txt`, `slot_tables.txt`;
  - `golden.cad` and `m10_6_certifying_keys.txt` (`dropped=1` held);
  - `plate_param.pncad`, `gallery_ring.pncad` (from `demo-tour gallery`);
  - `edit_placement_corpus_bits` (51→118 rows), lib_g16, perf2, seat4/7/8, asm2b row5, m4_pr3, m4_pr4, the m4_pr8 manifest, switch_program_vocabulary, msolve14 a3, `m10_p_fence` (its header says the line-by-line dump against main was not taken), m10_sym_profile (box-lane values pinned after the kernel lane's edits; re-check once that lane lands);
  - `name_words_corpus`, per the ruling: max 196→366, set by `nested_islands_106_depth2`, where each `Placed{of}` hop is spoken three times on an edge piece between two crossings. Item: `name-words-budget-grew-under-placed-copy-wrapping`.
  - pncad's twin-fixture block re-synced, so pncad is 76/76.
- **Refused, needs work:**
  - `perf12_census_goldens` is a behaviour change. heat_sink's census findings went 125→157. kitchen_sink went from 29 to 52 findings and from 16 rows to 8; its zero contact margins became nonzero, and two `CensusUndecidable` findings are new (Solid1 against Solid4). Look at whether kitchen_sink's placements now leave a sliver contact where the datum frames were exact (`a-turn-by-the-nearest-quarter-turn-rounds`). The blessed copies for reference are in the scratchpad as `perf12_blessed_*.txt`.
  - `msolve7_member_residue::a4`: `die_tool.pncad` now carries a genuine mate (from the `place_against` that replaced its Transform), so the row's premise "none carries a mate" needs restating.
- **Viewer `--test all`: 760 pass, 10 fail.**
  - **Probe/pick (7 rows):** `assembly_display::the_probe_gesture…`, `assembly_walk::the_exit_demo_walk`, `review_gui4_r1` ×3, `review_gui4_r2::a_rotating_probe_is_picked…`, `story_assembly::the_windmill_story`. They report "one probed part 0 != 1" or that the ray misses. This is likely the probe reading a placed copy in the wrong space, the same family as the `read_off` fix.
  - **`creation_ops::the_ring_stream_reproduces_the_gallery_document_bit_for_bit`:** the viewer's ops no longer author the tour's ring.
  - **Two goldens outside the lane's list:** `display_budget::the_budget_commits_the_delta…` and `review_pick_r2::the_certified_determinant…`.
- **Not run:** pncad-py's `test_placed_union.py::TestTheDieTool`, which compares against the regenerated `die_tool.pncad`.
- **Hazard:** a worktree built in the shared target dir can poison `/home/user/cad`'s build, because the artifact hashes are the same. If a mixed build is suspected, touch the `.rs` files. Disk was at about 2.4 GB free after the lane's cleanup.

**Kernel lane, finished (2026-10-19; log `/home/user/kernel-tour-status.txt`)**
- The tour's certified lane is back at its pre-D published numbers:
  - plate study 211/301;
  - chain at Interval [0, 8.9988e-5] for 1–4 links;
  - the 1-link chain on Sym certifies;
  - certifiable fractions [1, .3702, .1851, .1110], and all 3 chaintol tests pass in release.
- The box-lane solve composes the settled map as motions, so `admitted`'s Gram–Schmidt is no longer taken on frame lanes:
  - a placed target is read in its own coordinates (`pin.rs` `target_own_plane`, `wire/pose.rs` `own_direction`);
  - the chart's axes are kept unnormalised;
  - translation is set from the chart's components, where `on_axes` holds;
  - `onto`'s rigidity is decided only where the map is placed.
  All of it applies only where `spins_by_frames` holds. msolve14 a3 still passes: the f64 solve is main's, bit for bit.
- `place_at` now puts a `Flip` on the copy's side where its face looks down the world's z, instead of writing a spin by π (the revalue antipode fix). The tour's `chain.rs` joint signs follow from it. revalue `declared_tangency` now shows 4 `pcurve_chart_azimuth_frame` flips against 2 owed; re-take it.
- m10 rows all pass now, restated as pure pin and count moves: bulge ×3, derived_frame, tilted, m10_8, m10_9, sym_drive_memo, sym_9 ×2. Toggling the lane's changes off showed those moves came from the merged tree, not the lane.
  - `m10_6_r1`'s "bound straddled" premise is gone: the web encloses to the point [2, 2]. Its restated assertion is weaker than the original; review it.
- `sym11` is re-taken, not restated, because its text calls a move a finding. Its frozen column fell sharply (plate 729→161, bracket 2044→464, pad 3522→726), which needs a look.
- `m10_di_dual_corpus`: die_composed_tour has 16 undecided touches at Interval. The mechanism was not found.
- **Design-level, needs a ruling:** `drive_and_report(1 link, f=1)` certifies 0 leaves, so `demo-tour certified` fails. At the nominal, link 1 is at rest and the witness places it by no map, deciding no transforms. Every leaf off θ = 0 moves it, so the decision lists differ and the drive refuses `flip_crossing`. Under C-5's rest snap plus "the identity is placed by no map", a toleranced placement whose nominal is at rest cannot certify in the drive; before D, the Transform node always ran. Sent to the orchestrator.

**Ruling on the at-rest drive (orchestrator, 2026-10-19): (a).** C-5 is narrowed: snap only when the bundle reads no variable. A placement whose bundle reads a toleranced or free variable always runs its map. Tried and reverted (3fc2842244, reverted by the next commit). The attempt read `bundle_reads_a_variable` (every free variable reached from the bundle's numbers and mate reads, through definitions and the operations behind outputs, excluding the copied shapes) before the snap in `eval/wire.rs` `fold_with`. Measured:
- chaintol 3/3 still pass;
- **the plate study fell from 211/512 to 0/512**: 94.9% of leaves refuse `flip_crossing`, 4.6% hit the budget;
- `m10_p_fence` f64, Interval and ids-masked moved (expected, since a varying placement now runs its map at the nominal);
- m10_bulge ×3 (carrier_* counts rose at the nominal), `m10_9_no_registrant` (two_hole_plate 1381 decisions) and sym_9's kept-atom row regressed;
- whether the 1-link chain drive certifies is unknown, because `demo-tour certified` stops at the plate study.

So running the map at the nominal does not by itself make the lanes' decision lists agree: the plate's placements still flip. The next lane should find which decision differs between the nominal and a leaf once both run the map (`holds_rest`/`with_rest` are skipped on that path, and the transform's re-certification predicates may still decide differently). Then re-apply the narrowing with that fixed, re-run the plate study, chaintol and `certified`, and re-bless the f64 guards that move.

  **Found (2026-10-19, with the narrowing re-applied locally):** in the two-hole plate drive (`m10_5_clearance_interval::a_driver_certified_leaf_carries_the_certificate`), the flip is at the hole placement, node 44. Every transform predicate counts 0 at the nominal and n at the leaf: `transform_rigid_*` 0→1, `carrier_endpoint_*` 0→36, `carrier_on_surface_*` 0→324, `dihedral_*` 0→252. The cause is `mate::solve::marked` (`src/mate/solve.rs:470`): with the snap skipped, the f64 witness's map is still exactly the stored identity, and `marked` turns it into `Motion::Identity`, which places the copy by no map and decides nothing. A box lane's map is never a stored identity, so the leaf runs the transform. The narrowing therefore has to skip `marked` too. A bundle that reads a variable is placed by its map as `Motion::Map` even at the bit-exact identity.

  **Tried and reverted (not committed).** The narrowing plus that `Motion::Map` bypass of `marked`:
  - fixes node 44: the plate study is back to 211/512 certified (94.9%), and the two m10 plate rows pass;
  - chaintol 3/3 pass;
  - **but the 1-link chain drive still certifies 0 leaves** (`demo-tour certified` panics at `chaintol.rs:568`, "0 certified");
  - 23 editor-core rows go red, among them m10_10 (all_four_discharge, walked_rows_envelope), m10_di_dual and r2_m10_di_probes (Dual against Interval), `revalue_cut_cylinder`, m10_bulge ×3, m10_9 ×3, sym_9 ×2, sym11 and m10_sym_profile ×2, beside the expected f64 guard moves (`m10_p_fence` ×3, msolve14 a3).

  So the chain's 0 has another cause besides `marked`. Next step: with both changes applied, print the chain drive's refusal detail (as `m10_5_clearance`'s row prints it: node, predicate, count a→b) for the 1-link leaf at θ = 0, and find which node and predicate still differ.

  **Second differing decision (found 2026-10-19, both changes applied locally):** the 1-link chain leaf refuses `flip_crossing` on `datum_unit_norm`, counted 5 at the nominal and 4 at the leaf, at nodes 50 and 63 (the joint placements). So one unit-normalisation decision that f64 makes is skipped on the box lane. Likely source: the frame-lane-only path the kernel lane added. `pose::own_direction` (`src/eval/wire/pose.rs`) reads the raw carrier direction and snaps it with `mate::solve::on_an_axis`, which decides nothing, while f64 builds the same direction through `UnitVec3::new` and decides `datum_unit_norm`. The fix is to make the two lanes decide the same thing: decide the unit norm on the frame-lane path too (and keep the raw direction), or skip it on f64 as well.

  **Fix found and tested locally (2026-10-19); not committed.** It has three parts, all in mate/eval:
  1. the narrowing (`bundle_reads_a_variable` before the snap in `eval/wire.rs` `fold_with`);
  2. a varying bundle is placed by `Motion::Map` even at the stored identity, bypassing `marked`;
  3. in `mate/value.rs` `Chart::of`, the taken-back direction is normalised on every lane (`let back = unit(...)?`), so a frame lane decides `datum_unit_norm` as f64 does, while keeping its own direction.

  With all three, **`demo-tour certified` exits 0**: plate study 211/512, the 1-link chain drive certified 1, 0 refused, the 4-link drive certified too. **Triage done; reverted, because the bar is not met.** Re-apply the three parts from this description.

  With the fix applied, 23 rows fail in the m10 / sym / revalue / msolve14 set (392 run; log `/home/user/c5-ec.txt`). Sorted:
  - **Expected moves; re-bless each with the reason "a varying placement now runs its map at the nominal, so its transform decisions count there" (17):**
    - m10_bulge ×3 (`carrier_*` and `dihedral_*` counts rose at the nominal, e.g. segment_boss carrier_endpoint 24→30);
    - m10_9 ×3 (two_hole_plate's decisions to 1381; the registry and pad receipts);
    - sym_9 ×2 (two_hole_plate [1009, 662]→[1381, 980]; an upward move, which its text says is re-baselined);
    - m10_sym_profile ×2;
    - m10_10 ×2 ([232, 56]; the walked rows' envelope Some([32, 0]));
    - the `m10_6_ci` certifying-keys golden;
    - msolve14 a3, and `m10_p_fence` ×3 (f64, Interval, ids masked).
  - **Premise restored (1):** `m10_6_r1` "bound straddled". The web straddles 2.0 by ulps again ([1.9999999999999973, 2.0000000000000013]), so restore the row's original assertion that the kernel lane weakened.
  - **Real and unexplained, which blocks the commit (3):**
    - `m10_di_dual_corpus::dual_interval_evaluates_with_the_interval_value_channel` and `r2_m10_di_probes::deep_value_channel_identity_interval_vs_dual_interval`: on `die`, the `Dual<Interval>` value channel diverges from Interval. The likely place is the `Dual` `SolveScalar` impl (`eval/mod.rs`), which delegates `spins_by_frames` but whose `marked`/`is_stored_identity`/`carries_no_variation` answers may differ from Interval's on the new path.
    - `revalue_corpus::revalue_cut_cylinder`: the comparison move changed from in-step to apart.
  - **A finding to read (1):** sym11. two_hole_plate's receipt is [1381, 980, 1429], stored [1003, 654, 729]. More is proven, and more is frozen too.
  - **Pre-existing (1):** `m10_di_dual_corpus::assemble_census_verdicts`, die_composed_tour's 16 undecided touches at Interval, unchanged by the fix.

**C-5 lane, finished (2026-10-19). The narrowing is in. The editor-core suite is 2540/2548, and its 8 reds are the 8 that were red on the base (`c3bbb464c9`). The tour certifies.**
- **The fix** (`eval/wire.rs` `fold_with`, `mate/value.rs` `Chart::of`), the three parts above:
  - (1) `bundle_reads_a_variable` gates the at-rest snap;
  - (2) a varying bundle is placed by `Motion::Map` even at the stored identity, bypassing `marked`;
  - (3) `Chart::of` decides the taken-back direction unit on every lane.
- **What "reads a variable" means, refined from 3fc2842244's walk; the orchestrator should confirm.** The walk as first written made 34 split, inline and kstats rows red (the triage had run only the m10/sym set). Two refinements:
  - (a) **A variable is a declared one or a toleranced one.** A written number is VR6's anonymous untoleranced `FreeVar`, and no box or seed moves it. This follows `refactor::reads_a_number` and D's inline narrowing ("one that reads a named or toleranced variable keeps refusing"). Counter-text: `mate/anchor.rs`'s header ("a typed number with no tolerance is a document variable too … only the constant `0` reads nothing"), which `Anchors` itself keeps.
  - (b) **A mate's selection of the copy's own entities is not entered where `mate::anchor::Anchors::fixed` calls every name fixed.** Without this, a stand on a body with a declared height runs its map; `intent_s3_c_mates::a_copy_stands_where_its_body_stands_at_every_value` is the row.
  - **Known gap in (b):** `Anchors::fixed` answers whether the face's plane moves, not whether the pose read off it does. A root extrude's start cap is "fixed", yet its plane pose's origin moves with the profile's numbers. The m10 plate fixture's holes (`r.against`, start caps at ±half_spacing) therefore still snap. On this head that measured harmless: the m10_5 plate drive rows and the tour's plate study pass.
  - Skipping the copied shapes outright was tried and rejected, because it is wrong for a face the copy's parameters move.
- **The Dual value channel** (`eval/mod.rs`): `Dual<T>` did not delegate `exactly_zero`, so `mate::solve::on_an_axis` read an axis on Interval but never on `Dual<Interval>`. It now delegates `exactly_zero`, `holds_rest` and `with_rest` to the value channel, keeping each tangent. Dual64 is unchanged. `m10_di_dual_corpus::dual_interval_…` and `r2_m10_di_probes::deep_value_channel_…` pass.
- **Rows restated, each with an in-file paragraph:**
  - `revalue_cut_cylinder`, InStep → Apart, floor 1302. With the narrowing off, the apart move parts at decision 3667/3673 (`datum_unit_norm`): `p0` snaps the cutter while `p1` runs it.
  - m10_bulge: the boss and the parameter D-tab run their maps, and the literal D-tab still snaps. The shared table now carries an overlay, `D_TAB_RUNNING_ITS_MAP`.
  - m10_9 and sym_9: +186 symbolic and +159 numeric per varying placement; the plate holds.
  - `decide_3`: the link +36/+18, with its trades unchanged.
  - seat8: cut_cylinder and kitchen_sink moved in body `Debug` only; with the narrowing off they read their old pins.
  - `m10_6_ci` keys: the neck's.
  - `m10_6_r1`: its straddle is restored.
  - `m10_p_fence`: stale at the base, measured there. The narrowing moves no f64 outcome or point (scratch dump, off against on); it moves the Interval number. The Probe row is set to the f64 number but not run (below).
  - C-5's own rows in `intent_s3_c_mates`, ×3 (a seeded or boxed value reading a declared `s`/`t` now runs its map in every lane).
  - msolve14 a3, m10_10 and m10_sym_profile are back to their pre-narrowing pins, measured at all three ε.
- **sym11, left red as a finding.** `PAST_THE_CEILING` holds pre-D receipts. On this head, at 1e-9, as [symbolic, numeric, frozen]:

  | document | stored | measured |
  |---|---|---|
  | plate | [1003, 654, 729] | [1009, 662, 161] |
  | annulus | [539, 399, 904] | [728, 561, 777] |
  | link | [323, 259, 542] | [323, 257, 236] |
  | bracket | [1392, 1008, 2044] | [1770, 1334, 2119] |
  | pad | [1260, 1159, 3522] | [1260, 1157, 726] |

  The decision columns are explained: D's charts add +3/+3 per placement, and C-5 adds +186/+159 per varying placement (annulus, bracket ×2). **The `frozen` column is not explained.** It collapsed under D (plate 729→161, link 542→236, pad 3522→726) and rose under C-5 (annulus 144→777, bracket 458→2119). Dispute is 0, and nothing fired.
- **The 8 reds left, all red on the base:** `edit_placement_corpus_bits`, `perf2_name_keying_differential`, `seat7_sweep_lowering`, `perf12_census_goldens`, `intent_s2_c_world::the_migrated_world…`, `intent_s3_c_mates::the_place_gesture_pins…`, the `m10_di_dual` census (16 undecided), and sym11.
- **Also found:** `cargo test -p editor-core --features probe` does not compile on D. `tests/m4_pr8_k_probe.rs` names `ExtrudeSide` and `NodeValue::placement`, and so does `tests/parts_entered_before_the_map.rs`. These are D's fan-out, not yet fixed, so the fence's Probe row is unverified.
- **Checks:** clippy `-p editor-core --all-targets -D warnings` is clean, after mechanical fixes in five test files. Tour (release): chaintol 3/3 and `certified` exit 0. The plate study is 211/301, the 1-link chain drive certified 1, and the 4-link drive certified.

**Window of 2026-10-19 (about 3% of usage left)**
- **The tour die corpus:** `cargo run --release -- die-corpus ../../crates/editor-core/tests/corpus/tour/die_composed_tour.pncad` in `demos/tour` gives a file byte-identical to D's committed one (1,589,134 bytes) on D's head. Nightly 65's "not byte-regenerable since #5267" was raised against main's copy, which D replaces. Nothing that reads it needed re-pinning.
- **msolve7 a4:** restated as `a4_every_checked_in_document_loads_and_its_bundles_reach_the_attribute`, since under D every placement is a bundle and the corpus now reaches the attribute. It asserts that every document loads and that the corpus carries bundles. 3/3 pass.
- **revalue `declared_tangency`:** re-taken. The owed `pcurve_chart_azimuth_frame` flips went from 2 to 4 (the placed tangent copy's charts on the same exact zero), and `pcurve_loop_branch` joins `after`. Noted on the owed inlane item. `revalue_corpus` is 12/12.
- **The pncad-py suite** (`crates/pncad-py/run-python-tests.sh`): 925 run, 2 failures, 1 error. Not fixed for lack of usage; next steps:
  - `test_placed_union.TestTheDieTool.test_the_re_authoring_is_the_corpus_document_byte_for_byte`: Python's re-authoring differs from the re-blessed `die_tool.pncad`, with different content hashes, ~49.9k vs ~49.7k characters. Likely the kernel lane's `place_at` change (a `Flip` on the copy's side, not a spin by π) or the re-bless regenerated the Rust side while pncad-py's die-tool authoring spells it the older way. Diff the two saves, align `crates/pncad-py/tests/test_placed_union.py`'s authoring (or the façade gesture it calls) with the Rust generator, then re-run.
  - `test_north_star.TestTeapot.test_the_spout_is_placed_and_the_handle_is_a_tube`: "a loft has exactly one cap per end" reads 0 != 1. The spout loft's caps are no longer found, likely because a cap's name now carries the placement's `Placed` layer, or because of the loft-station reshape (row 161 Q2). Read how the test finds caps.
  - `test_assembly_author.TestAssemblyRefusals.test_a_pose_read_off_a_failed_node_names_its_own_cause`: F's door (merged from main) now refuses the bundle the test builds as overconstrained ("each of constraints 1, 2 fixes an equation the rest of its bundle already fixes"). Restate the bundle so it is not overconstrained and keeps its subject: a pose read off a failed node names its own cause.

**Known reds at the full-suite run on `ea25544a9a`:** 68 editor-core failures, listed with their owners above. pncad's 2 are re-bless. The viewer `--test all` suite needs a re-run (the 17 gallery_ring goldens are re-bless). The viewer's 2 lib reds are the GPU tests (no adapter). `every_suite_file_is_aggregated` fails only because of the local `zz_*` probe roots.

**Uncommitted:** nothing. The local `zz_*` probe roots and their `Cargo.toml` entries were removed at handoff, so `every_suite_file_is_aggregated` passes again.

**Lane of 2026-10-19/20** (head after its last commit; nothing uncommitted)
- **origin/main merged again** (merge `0a1d95321e`, 205 first-parent commits: #5068's Ruling A — `ProfileLift` retires, the `f64` lane is pass 1, every lane elaborates guided — OpInput, the union decision log's k-ary `Meet` rows, PCERT B1/B2a, the joint's turn #5354/#5377, the D-rod fixtures). Main's frame-plane mint (`FramePlacement`, `mint_frame_placement`, `profile_plane_f64`, `frame_plane_lane`) and its datum builders stay retired: a profile is on its own `xy`, `prepare_profile`/`wire_profile` take no plane. Main's new rows that author frames, datum planes, `ExtrudeSide` or a block at a height are re-spelled as constructions placed against the first operand (`emit_union_seam_rank`, `union_fold_face_face_touch`, `emit_boolean_vertex_keys`, `split_pinch_keeps`, `edit_strip_mouth_cups`, `intent_part_census_seam`, `a_keel_ring…`, `f64_lane_validates_once`, `docm6`, and others). Counts both sides moved were re-measured; `m10_8` and `asm2a` hold additively. The empty `wire_frame_placement_carry.rs` and its `all.rs` entry are deleted.
- **Probe compile** (`--features probe`): `m4_pr8_k_probe`'s `FramePlacement` compare retires with its subject; `parts_entered_before_the_map`'s lone block is drawn about its centre. Both pass.
- **`InlineError::CopyPinnedToConstruction` retired** (refactor.rs, the pncad-py tag, census rows and stub text): a placement mated to a construction lives in that construction's loose space, which inline skips, so the check could not fire.
- **pncad-py green (932 run)**: die_tool's Python authoring writes the revolve axis as `Formula.zero("length")`, as the corpus writes it (the only semantic difference between the two saves was `0 m` vs the constant zero); the teapot's placed caps are read under `Placed`; the failed-node pose row's cause is a world read slid by `1 m / gap` with `gap = 0` (F's door refuses the old over-constrained bundle); main's new `test_interference` rows and `test_checks`' two junction rows (red on main: #5354 restated their Rust twins, not these) re-spelled; the shared-tag roster's `band` takes main's 17.
- **Viewer green but one (769/770)**: the 7 probe/pick rows were the pick index drawing world placements only (D's WIP filter), so a free-moved loose copy was never drawn; it now draws each loose copy's space too. `door_at_rest` pins the composed die's 3 unproven rows (main's two `Meet` rows). The corpus walks (`review_pick_r2`, `display_budget`) silently skipped a bump that sets a variable (`cut_cylinder`, `part_select`); they take it now, and both goldens are re-pinned with what moved in their files.
- **editor-core fixed**: two corpus port defects — the kitchen sink's lone block extruded up once `SetExtrudeSide` retired (its copies now hang below the xy, as before; this was most of perf12's kitchen_sink move), and `corpus::place_pattern_to` placed a driven heat sink's further fins at the identity, a foot low (it now places them where the document places its fins; this was all of heat_sink's 125 → 157). The place gesture's row is restated: the Flip on a face looking down means the chart never reads at its pole, so it charts rather than pins. Re-pinned, each with its reason in its file: perf12 (all ε), intent_s2_c_world, edit_placement_corpus_bits, perf2, lib_g16, name_words' digest, seat7, seat8.
- **Mechanisms found**: perf12's remaining kitchen_sink moves (1e-33 margins, one curved pair out of reach) and m10_di_dual's 16 `curve_pair_pose_root_gap` escalations at Interval are both `a-turn-by-the-nearest-quarter-turn-rounds` (evidence appended to that item): quarter-turn placement maps carry `6.1e-17` residues at f64 and `~1e-14`-wide enclosures at Interval, and the census's root gap turns that into `[0, ~1e-6]`. Not the snap: no such placement is at rest.
- **Reds left, editor-core** (final run of this lane: 2576 of 2591 pass, 15 red):
  - Snap-fenced, left as measured (each moves with C-5's narrowing composed with main's new predicates, or is in the fence by name): `m10_9` ×3 (annulus symbolic 632, pad shut (1546, 1324, 3024), bracket shut (1566, 1456); pins restored to D's last values), `m10_bulge` ×2 (main's pcurve rows ×5/3 where the boss and parameter D-tab run their maps: cut_side 26 → 42, placement 52 → 84, …), `m10_p_fence` ×3, `m10_sym_profile` (the slab moves additively, Plain/Assertion 524 calls 2724 forms), `sym11` (plate [945, 701, 161], annulus [632, 605, 844], link [295, 269, 263], bracket [1630, 1401, 2409], pad [1570, 1336, 3035]), `sym_9` ×2 (e.g. plate [1009, 662] → [945, 701]; drive leaf [1419, 1452, 34]), `m10_6_ci`'s certifying keys, `revalue_boss_union` (`bool_pair_gap` re-values away).
  - Other: `m10_di_dual_corpus::assemble_census_verdicts_match_f64_at_interval` (the quarter-turn class above; fixing it is that item's).
- **Red left, viewer**: `creation_ops::the_ring_stream…`. The viewer's gesture places a new body by `place_at` at the identity and the tour stands it by `stand`; switching the gesture to `stand` exposes that `stand` shares a revolve's written axis numbers, which the edit door refuses. Filed `a-stand-on-a-revolve-shares-its-written-axis-numbers` (P1, design): it needs a ruling.
- **Checks**: `cargo clippy --workspace --all-targets` (and `--all-features`) `-D warnings` clean; demos/tour and demos/wild clippy clean; benches and tools/* check; rustfmt clean; `work.py lint` ok.

**Lane of 2026-10-20 (FORK-REST and the identity bundle; rulings 1 and 2 of the orchestrator's brief)**
- The previous lane's work is d1f57659dd..851c56a3a7: FORK-REST (only the identity bundle as written places by no map; `rest_verdict`, `solve_copy_at_identity`, the `SolveScalar` rest hooks, `marked`, `bundle_reads_a_variable` and `Composed`'s at-rest parts deleted), the mateless bundle (`Value.mate` optional, `Coord::SpinAbout`, `DocEdit::AddValues`, `MateFault::Unpinned`), `stand` writing the identity bundle, the façades (`Placed::write`/`identity`, pncad-py `MateValue(None, ..)`), the viewer's gesture by `stand`, `RemoveConstraints`, golden.cad/die_tool.pncad/slot_tables re-blessed.
- **Its restart WIP commit ff37f08174 carried 434139ef1c's tree** and undid all four commits; 6287cf64fd reverts it. The untracked probe `zz_dlane_probe.rs` (per-node body digests over the corpus) is deleted.
- First full editor-core run on the restored head: 2562/2591, 29 red. Mechanisms measured with a lane probe (not committed):
  - every identity-bundle copy in the corpus is bit-identical to its source (`body_digest`);
  - the identity bundle's one `AddValues` mints a different number of ids from the plane mate and three values it replaces, so every node minted after a stand moved its id: exactly heat_sink, declared_tangency, kitchen_sink, cut_cylinder (second half) and part_select. That is all of seat4 (heat_sink), seat7, seat8, lib_g16, perf2, edit_placement, name_words and perf12 (perf12 is byte-identical with every `digest:` masked, all three ε). Re-blessed in cf7bec44b4.
  - seat4's empty boolean: a plane-mated copy at the exact identity now runs its map (FORK-REST).
- `demo-tour certified` (release) exits 0: plate study 211/512 (1557 forms frozen), 1-link drive 1 certified, 4-link drive certified; chaintol 3/3.
- m10_bulge: one D-tab table for both spellings (the overlay retired: the literal runs its map too); the boss's pcurve rows are main's (×5/3). m10_10: the plate's holes run their maps (carrier_matches_mapped_source 196→232, envelope 16→32). In flight.
- FORK-REST moved only the plate among the sym/m10_9 documents (sym11 receipts equal the 434139ef1c measurements on annulus, link, bracket, pad); the plate's holes, which the narrowing's gap (b) kept snapping, run their maps.

- Re-taken under the rulings, each with its reason in its file, attributed against `434139ef1c` measured in a worktree (and main `1fcd4bd807` for sym11's `frozen`): m10_bulge (one D-tab table), m10_10, m10_9, sym_9 (both rows), sym11 (its `frozen` column explained), m10_p_fence, m10_sym_profile, msolve14 a3, m10_6 keys, revalue_boss_union, review_pick_r2; `gallery_ring.pncad` regenerated, `plate_param.pncad` re-blessed. decide_3 and kstats pass.
- The viewer is 770/770 locally after its fixes (the session's `DocEdit` match lacked `RemoveConstraints`; its rows read the place gesture's one `AddValues`), pncad 76/76, pncad-py 932 OK (`UNPINNED_RECOURSE` bound; `remove_mate` answers a whole bundle's other values; the census accounts the new names), `--features probe` 331/331 on its rows.
- Items: closed `a-placed-copy-rests-at-the-identity-only-at-the-nominal`, `a-symbolic-runs-copy-decided-at-rest-keeps-its-map`, `a-stand-on-a-revolve-shares-its-written-axis-numbers`; D′'s item says what D builds; evidence on `a-turn-by-the-nearest-quarter-turn-rounds` (m10_di_dual's 16 escalations still occur; die_composed's fillet refuses at Interval at ε = 1e-12, which reds m10_p_fence's Interval row on the nightly's 1e-12 row) and on the boss_union inlane item; filed `lib/the-census-resolves-a-word-two-types-share-to-the-first`.
- **Final measurements on f10a342a9d (local; CI is the record and has not run this head):** editor-core `--test all` 2590/2591, the one red `m10_di_dual_corpus::assemble_census_verdicts_match_f64_at_interval` (the quarter-turn item's); viewer 770/770; pncad 76/76; pncad-py 932 Python OK and 133/133 Rust; `--features probe` rows 331/331; `demo-tour certified` exits 0 and chaintol 3/3; `cargo clippy --workspace --all-targets --all-features -D warnings`, demos/tour and demos/wild clippy, `cargo fmt --check` and `work.py lint` clean. Known red on the nightly's ε = 1e-12 row: `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval` (die_composed's fillet refuses at Interval there, as it did on `434139ef1c`; the quarter-turn item).
- Not done: main has moved 472 commits past the merge base (`1fcd4bd807`); merging it is the next lane's. Then open the PR from the draft body below.

**Next, for a fresh lane**
1. `git fetch origin && git checkout intent/s3-d-transform-retires-into-a-placement && git pull`, then `export CARGO_TARGET_DIR=<one target dir> CARGO_INCREMENTAL=0`, then `cargo nextest run -p editor-core --test all --no-fail-fast` and `cargo nextest run -p viewer --features app,test-support --test all --no-fail-fast`. Settle each red per the three lane logs. The logs live in `/home/user` and do not survive the container, so this section names what each said.
2. Merge origin/main again if it has moved, then run `cargo clippy --workspace --all-targets -- -D warnings`, demos/tour's clippy and certified lane, and `python3 scripts/work.py lint`.
3. Open the PR from the draft below (the interim `stand`, pointing to D′; C-5's change; the re-bless log). Run the review pair at D's planned tier, with the row as the last commit, then merge (the orchestrator released the hold).

**Lane of 2026-10-20 (main merged; exact turns), in progress**
- origin/main merged (`557e581a64`, 562 commits): conflicts were the m10/sym count rows (main's PCERT B2b paragraph kept beside D's; D's values held until re-measured), `refusal_concision_chains` (main's per-arm rows without the retired face-frame and frame-direction kinds), `pncad.pyi` and `test_checks.py` (main's). Gate fixes: `geompred::prepare` takes the caller's `tol`; `test_support`'s placement helpers take `tol` (the fixture wraps them at the witness).
- **A turn by a rational fraction of a turn is exact** (`3ea5a27157`): `geom_core::Angle`/`Turns` read sine and cosine off the fraction; `VarEnv::turns` records each angle variable whose definition is a constant fraction of a turn, at `Doc::bind_definition`; every rotation fed by an angle reads it (pose and world-read spins, loft stations, a revolve's heading, circular pattern steps, mated and unmated value spins). Sym folds sin/cos/tan of a rational multiple of π where rational. Row: `intent_s3_d_placements::a_block_placed_upright_by_a_written_quarter_turn_stands_on_the_worlds_planes`.
- **Writers** (`a9a6851826`, `3a30473b61`): `moved_frame`/`FrameMove` and `corner_frame` (editor-core, façade); the corpus pips (die_pips, die_composed) and the tour die stand on lines stated from the cube's corner; the sink's revolve and the vessel stand upright by a bundle with no mate spun `turn/4`; `test_support::plane_on` reads a face with the stated normal, `axis_on` the meet of two, `plane_on_frame` keeps the frame read.
- Both reds measured green: `m10_di_dual_corpus` 8/8, and every corpus document builds at `Interval` at ε = 1e-12. Next: the full suites, re-bless what moved, the eps rows, viewer, pncad, pncad-py, probe, tour certified, clippy/fmt/lint.

### Draft PR body

INTENT stage 3, PR D — `work/intent/transform-retires-into-a-placement.md` (spec `docs/INTENT-STAGE3-SPEC.md` §5, fork-log rows 159 and 161). Also closes `a-copy-standing-where-its-body-stands-has-no-d10-spelling` (C's world pin) and `datum-distance-reads-a-datum-node-not-a-pose`.

The loft station's D10 `[variables.kinds]` sentence was approved by Ev on #5295 (fork-log row 161).

#### What D builds

Frames enter only at placement, and nothing moves a body (D10; FORK-S3P, FORK-S3M).

**Constructions read no frame.** A profile is 2-D: `ProfileProgram.plane`, `Node::Datum` and every `Datum::*` are gone. An extrude goes up its own +z (`Extrude.side`, `ExtrudeSide` and `DocEdit::SetExtrudeSide` retire). A tube's spine is the circle about its own z (`Tube`/`HollowTube.frame` retire). Every construction is the root of its own space (`Space::Loose { root }`), so a node reading two constructions refuses `SpaceMismatch`; a later combine operand is placed against the first operand's root.

**A loft's sections carry stations** (row 161, Q2; D10 sentence approved on #5295): `Loft { first, then: [LoftSection { profile, station: [StationMove::Slide { along, by } | Spin { about, by }] }], v_degree }`, the moves in the first section's own axes, applied in order (`SlotId::StationSlide/StationSpin { section, step }`). `wire_loft` composes each station onto the first section's plane. `SetMembers` on a loft must keep its section count (`EditError::LoftSectionCount`), since a new section has no station. Content key: tag 15 retires, `Loft` is 41, and the move kinds and axes are keyed.

**Transform retires.** `Node::Transform`, `Placement`, `Step`, `PortKind::PlacedFrom`, the member walk's `Placing::Transform`, `VerbatimEdge::Whole` and the transform slots (`Translation`, `RotationAxis`, `RotationAngle`, `Normal`, `U`, `V`, `Spin`, `PlacementStep`) go. A rigid motion of a body is a `Place`.

**C's world pin retires** (row 159). `ConstraintKind::WorldPin`, `DocEdit::PinWhereItStands` and `InlineError::WorldPinOnAMovedWorld` go. A copy standing where its body stands is `mate::stand`, which writes the identity bundle (below).

**Plane-less roots are placed on their own axis** (row 161, Q1), by `place_at` and `place_against` (no longer by `stand`, which writes the identity bundle for every copy). For a full revolve or a full-ring tube they write an `Axis` mate with constant-zero `Slide(Z)` and `Spin`. Its `on` is the revolve's `axis` output where the copy is the revolve itself, else a circular edge of the copy minted by the revolve (the edit door lets `on` read only the placement's own shapes), or the ring's torus axis. The target is the world's x axis, spun about z by the heading and slid by `axis_through`, each move reading what its slot reads (a constant `0` slot writes none). `revolve_about` and the tour's diefillet, ring and teapot write `axis_through` as the constant zero, not `0 mm`.

**The bundle with no mate, and FORK-REST** (the orchestrator's rulings 1 and 2, 2026-10-20; D10 `[placement.mates]`, `[placement.values]`). D builds D′'s data model for a placement with no mate:
- `Value.mate` is optional. `Coord` has `Slide(TargetAxis)` along and `SpinAbout(TargetAxis)` about the target space's x, y and z (`SlideAxis` is renamed `TargetAxis`).
- The door rule for a bundle with no mate (`mate::solve::unmated`, `unpinned`): every freedom written (new `MateFault::Unpinned`), the spins a chain of at most three with no two in a row about one axis, and a repeat refused as overconstrained. `NoChartWhy::{NoMatedDirection, BesideAMate}` name a value that mixes the two kinds.
- Evaluation in the written order (`mate::value::unmated_motion`): the spin chain about the copy's origin, then the slides, composed as motions.
- `DocEdit::AddValues` writes such a bundle whole; `DocEdit::RemoveConstraints` removes one. Persist, the wire bytes and the content key (v16) carry them. The façade writes it through `Placed::write`/`Placed::identity`, pncad carries `Placed`, and Python has `MateValue(None, ..)`, `Coord.spin_about` and `Doc.add_values`/`DocEdit.add_values`.
- **`mate::pin::stand` writes the identity bundle**: no mate, and the constant `0` on the three slides and the three spins. It reads nothing of the copy, so a revolve, a ring and an instance stand as an extrude does, at every value of their variables. Its Axis-mate fallback and its variable-free-face search retire with it. The viewer's "place where authored" gesture writes `stand`, so the ops-authored ring is the gallery document bit for bit (`creation_ops::the_ring_stream_reproduces_the_gallery_document_bit_for_bit`), and `a-stand-on-a-revolve-shares-its-written-axis-numbers` closes.
- **FORK-REST: whether a copy is placed by no map is read off its recipe, never decided from values.** Only the identity bundle as written is placed by no map, in every lane. Every other placement runs its map in every lane and at every value, even where the map comes out as the exact identity. Deleted: `mate::solve::rest_verdict` and the `solve_copy_at_identity` decision, the `SolveScalar` rest hooks (`is_stored_identity`, `carries_no_variation`, `identity_varying_as`, `with_rest`, `holds_rest`), `marked` on the fold's map, `eval/wire.rs`'s `bundle_reads_a_variable` and the snap in `fold_with`, and `Composed`'s at-rest parts. `SolvedPoses::at_identity` is the recipe query "this placement's bundle is the identity bundle". `Motion::Identity` comes only from a stored frame's bits and the identity bundle. This replaces C-5 and its narrowing (below), which it supersedes.

**The placement helpers**, `mate::pin`:
- `stand`;
- `place_against(doc, target, body, at)`: a copy placed against a body's own geometry, either by a plane mate between own-xy faces slid along the normal, or by a frame read off the target for a tilted face, or by the copy's own axis for a revolve or ring;
- `place_at`: preference order is a face that reads no variable, then the copy's own axis, then any planar face, then the pin;
- `frame_at`, `pin_at`.

Each has an `_in` form over an evaluation the caller holds. C's interim report machinery (`Anchor`, `Placed::anchor`, `pinned_at`) retires. `mate::anchor::Anchors` stays for `place_at`, `pin_at`, `frame_at` and `place_against` (D′ re-spells them); `stand` reads no anchor.

**The values defect (ruled (A)).** A world mate's values are charted from the world's own frame, not from the moved read. So a datum's chain splits: the out-of-plane part goes on the world read, and the in-plane slides and the spin go on the values. Pinned by `intent_s3_d_placements::a_datums_in_plane_offset_rides_on_the_values`: a world read slid (3, 4) with zero values lands at the origin, and Slide X = 3, Slide Y = 4 lands at (3, 4).

**Inline at a moved landing.** A part placement whose world bundle holds numbers alone is re-pinned by `pin_at` at the copy's moved pose (new `InlineError::RepinRefused`). One that reads a named or toleranced variable keeps refusing (`ValueOnAMovedWorld`). This narrows what C's world pin allowed; the narrowing is filed as an item.

**Sketch on a face** is one gesture: `editor_core::sketch_on_face` / `Recording::sketch_on_face`, re-exported by the façade. It writes the profile, the extrude, its copy placed by `Flip(start cap)` → `Plane { face }` with Slide X = Slide Y = 0 and Spin = `spin`, and the union with the face's body. DM1 is restated to say so.

**Selections.** `GeomPred::DatumDistance` → `PlaneDistance { face, cmp, value }`, measured against a face of the selected body read as its plane. `SelectRefusal::{NotADatum, DatumHasNoValue}` → `{NotAPlane, FaceUnresolved}`. The datum face-frame refusals (`NodeErrorKind::FaceFrameNotPlanar`, `FaceFrameReadback`, `FrameDirection`) have no producer and retire.

**Names.** Unchanged from B: a copy's names are qualified `Placed { of }`, once per copy by construction. Spec §5's "pass through, as `Transform`'s did" predates B's qualification and is re-worded. This follows from B and is not a new decision.

**A turn by a rational fraction of a turn is exact** (D10 `[variables.literals]`, D8 `[recipe.forms]`; this lane closes `a-turn-by-the-nearest-quarter-turn-rounds` and SYM's `sin-and-cos-of-rational-turns-fold-exactly`). D's spins were built by Rodrigues from `sin_cos` of the f64 nearest a quarter turn, so a frame a `Datum` used to give exactly left the axes by `6.1e-17`; that was `m10_di_dual_corpus`'s 16 escalations at `Interval` and the 1e-12 fillet refusals.
- `geom_core::Angle`/`Turns`: an angle carries the fraction of a turn it was written as, and its sine and cosine are read from it (exact `0`, `±1/2`, `±1`; the octant's otherwise); `I − R` reads `(sin, 1 − cos)` exactly for whole quarter turns. The rotation constructors take an `Angle` (`Mat3::rotation_about_unit_by`, `Affine3::rotation_about_unit_axis_by`, `RigidMotion::rotation_by`, `rotation_about_axis_by`); the radian forms are the same body.
- The form crosses at one door: `Expr::constant_turns` reads a definition's form, and `Doc::bind_definition` (every environment ends there) records it in `VarEnv::turns`; `expr::eval_angle` and `eval::slots::angle` hand it to every rotation fed by an angle: pose and world-read spins, loft station spins, a revolve's heading, circular pattern steps, mated (`settle`'s `turn_to`, `Chart::turned`, `head_at`) and unmated placement spins. Decided from the recipe's form, never from a value.
- Sym folds sin, cos and tan of `q·π` where the value is rational (`trig_at_rational_pi`, beside `unary_at_zero`).
- Writers that mean an axis-aligned frame state it: `editor_core::moved_frame`/`FrameMove` and `corner_frame` (façade too); the corpus pips (`die_pips`, `die_composed`) and the tour die stand on lines stated from the cube's corner (`demo-tour die-corpus` regenerated `die_composed_tour.pncad`); the kitchen sink's revolve and the vessel stand upright by a bundle with no mate spun `turn/4`; `test_support::plane_on` reads a face with the stated normal, `axis_on` the meet of two, and `plane_on_frame` keeps the frame read for a normal no face carries. `test_support`'s placement helpers take the caller's `tol` (the witness gate).
- Row: `intent_s3_d_placements::a_block_placed_upright_by_a_written_quarter_turn_stands_on_the_worlds_planes` (`f64`, `Interval`, `Dual64`, `Sym`; a bundle with no mate, a mated spin, a world read spun by `turn/4`).

#### Fixed in passing (each with a row)

- `place_against` slid a down-facing target the wrong way (`a_copy_slid_up_its_target_stands_up_it`).
- `payload_reach` read 0 for a boolean, split or instance copy, so every slide value on such a copy was undecidable. This is a pre-existing bug: C never wrote values on such a copy. No open item covered it. Row: `a_boolean_copy_stands_over_its_bodys_reach`.
- `Evaluation::loose_roots` counts loose COPIES only (new `Evaluation::loose_copies`); a construction root is no loose copy (`a_construction_is_no_loose_copy`). The façade's `step_for_node` writes a construction in its own coordinates and refuses only a node in a loose copy's space.

- `inline` at a moved landing re-pinned a nested instance as though it were a `Place` (`refactor.rs`, `unreachable!`). It now pins the instance through its own copies (`p2_split::i1`, `i2`; `p2_gauge_offsets::an_inline_at_a_turned_pose…`).
- (Superseded by the identity bundle, which reads no face.) `stand` on an instance chose the part's end cap. `Anchors::fixed` called every `InPart` name unfixed, so the end cap, which the part's height moves, was taken. It now reads an instance's face by the name's shape: a construction's start cap under the part's copies is fixed. The row is `an_instance_stands_on_its_parts_base`. The part's recipe is not reachable from the host's evaluation, so the remaining gap is the item `a-stand-on-an-instance-reads-its-part-by-the-names-shape`.
- `frame_at` and `place_against` refused a placed TARGET with `AlreadyPlaced`. They now read a placed copy where its placement moved it, with `at` given in the copy's own coordinates and carried by its motion. A placed copy as the body being placed still refuses. Row: `a_copy_stands_against_a_copy_already_placed`.
- A solved placement's map on the Interval lane, read off geometry a widened parameter moves, refused "whether the map is rigid … undecided". The enclosure's columns are too wide for `check_rigid`'s seven margins. The solve composes the map of rigid pieces only (orthonormal frames, turns about witnessed axes, slides), so where the door cannot call it, it is re-witnessed by Gram–Schmidt over its first two columns (`mate::solve::admitted`, funnel `solve_map_rewitness`). A lane the door calls, f64 included, keeps its bits. This had hit most of the m10 drive documents.
- The flip report had no words for the mate solve's predicates (`mate_*`, `solve_copy_at_identity`) or for the rigidity margins a placement's map is admitted through (`transform_rigid_*`). They are now worded in `mate::DECISION_WORDS`, which `decision::words` consults.

- **The box lanes' solved map stays tight** (24 m10 Interval rows; 16 were certification losses). Before D, a toleranced parameter moved a datum frame; in D it reaches the body through a placement's motion. The solve widened that map well past what the box justifies. Three fixes, each behind a `SolveScalar` hook that defaults to the old path, so f64, Dual and Probe keep their bits:
  - A value's spin is set from the two charts' frames (`Chart::turned`) where `spins_by_frames` holds, instead of turning the folded map by `v − atan2(…)`, which counted the map's width twice.
  - A direction two of whose coordinates are exactly zero is read as that axis (`mate::solve::tightened`).
  - Fork C-5 (the C lane's code-level rest snap, 99774f9395, no ratified clause behind it) was changed, then narrowed, on this branch, and is now gone: FORK-REST (above) removes the snap. The lanes now decide one list because no lane decides whether a copy rests. Swept (the class "a lane deciding a path another lane skips"): every static per-scalar branch hook in `editor-core/src` (`T::spins_by_frames()`, 7 sites in `mate/value.rs` and `eval/wire.rs`) and every method or const hook (`exactly_zero`, whose one reader is `mate::solve::on_an_axis`: `tightened`, `Chart::of`'s own direction, `Chart::on_axes`). Disposition: each chooses only how an enclosure is spelled or how tight it is; every predicate one lane decides there the others decide too (`Chart::of` decides `datum_unit_norm` on every lane; `admitted` asks `check_rigid`'s seven margins of every placed map, the frame lanes of `Composed`'s constant turn); `pose::own_direction`, a frame lane's alone, reads `topo::readback`, which is generic over `Real` and decides nothing. What the pattern cannot see is a lane-dependent branch inside `geom-core`'s own predicates; the drives' `flip_crossing` comparison of witness and leaf decision lists is the runtime check, and the tour's plate study and chain drives pass it.

  Items filed for what is left: `sym-solve-atan2-has-no-branch-rule-about-pi` (P2), `a-slide-value-doubles-its-translations-width` (P3), `a-face-planes-normal-read-off-geometry-is-wide-at-a-box` (P3).
- A frame's turns near gimbal lock (`mate::pin::turns`): a quarter turn rounded just below π/2 took the non-gimbal branch at `cos β ≈ 3e-16`, read its x and z turns off rounding, and placed a circular pattern's copy 15.6° off. The branch now switches within `√ε` of the lock.
- A re-point strands a name its derivation reaches through. A copy's `Placed { of }` name is in reach through the placement's shapes, not its mates (`edit::stranded_by_repoint`, `taken_out_of_reach`; `dm4_names_by_variable::a_repoint…`).

#### Design pages

- `REFERENCES.md` DM1 (ratified in chat 2026-09-04) is restated from "a derived frame is a datum carrying a face name" to "a sketch on a face is placed against the face's plane". This follows from FORK-S3P's retirement of `Datum` (Ev-ratified); DM1a/b/c are re-worded to their surviving content.
- DM3 (source: 0877790361, the DOCM vocabulary ratified in chat 2026-09-04) loses the comparison to the retired node and keeps the member rule as decided.
  - Before: "Names pass through unchanged, as `Transform`'s do (`role.rs`): the member keeps its `Member { (i, …), of }` names…"
  - After: "Names pass through unchanged (`role.rs`): the member keeps its `Member { (i, …), of }` names…"
- `ASSEMBLY.md` (split's instance stands by `stand`; the member walk has no Transform), `README.md` (the sketch plane, loft sections), `profile/README.md`, `verbs/README.md`, `names/README.md` and `DESIGN.md`'s status lines and companion row are re-worded for the code change.
- Spec §4 points at D for the interim's end, and spec §5's Names bullet is re-worded.

#### Migration: the corpus

TODO table: document → how restated.

#### What moved

- The revolve axis constant zero (`axis_through: [0, 0]`, which reads nothing, in place of `0 mm`), in 11 files:
  - `tests/fixture/mod.rs::revolve_about`;
  - the editor-core suites `dm4_names_by_variable`, `switch_slots`, `intent_s3_a_poses`, `wire_operand_door`;
  - the viewer suites `combine_ops`, `gesture_table`, `creation_ops`;
  - the tour's `ring`, `diefillet`, `teapot` and `tests/teapot_document`.
- Content key: node tag 15 retires (a loft over sections on datum frames), `Loft` is tag 41, and a station's move kinds and axes are keyed. The VERSION bump and the other retired tags are listed below.
- `EditError::LoftSectionCount` narrows `SetMembers` on a loft (item `a-loft-set-members-cannot-add-a-section`, P2).

#### Behaviour changes for callers

TODO from lanes: Python API before → after; viewer ops before → after; tour stops' numbers.

From rulings 1 and 2:
- **Python:** `MateValue(None, coord, by)` is a value naming no mate; `Coord.spin_about("x" | "y" | "z")` beside `Coord.slide(axis)`; `DocEdit.add_values` and `Doc.add_values` write a bundle with no mate whole; `UNPINNED_RECOURSE`, and the `mate_unpinned` fault word; `Doc.place(shapes)` with neither constraints nor a pose writes the identity bundle; `Doc.remove_mate` on a value of a bundle with no mate removes the bundle and answers the other values' addresses.
- **Façade:** `Placed.mate` is optional, `Placed::write` writes either kind of bundle, `Placed::identity`, `PlacedBy::Identity`; `DocEdit::AddValues`, `DocEdit::RemoveConstraints`, `DocEdit::unmated`, `MateFault::Unpinned`, `NoChartWhy::{NoMatedDirection, BesideAMate}`, `Coord::SpinAbout`, `TargetAxis` (was `SlideAxis`).
- **Viewer:** "place where authored" and every new body's placement write `stand`'s identity bundle as one edit (one undo step, where the mate and three values were four).
- **Tour:** the certified lane reads its published numbers: plate study 211/512, the chain's 1- and 4-link drives certify, chaintol 3/3.

#### Hit lists (the class swept)

TODO.

#### Tests deleted (retired subjects)

TODO from lanes.

#### Goldens re-blessed

TODO: what moved and why (the re-bless lane's log). The lane of 2026-10-19/20 re-pinned perf12 (all ε), intent_s2_c_world, edit_placement_corpus_bits, perf2, lib_g16, name_words' digest, seat7, seat8, display_budget and review_pick_r2, each with its reason in its file.

The lane of 2026-10-20 (rulings 1 and 2), each with its reason in its file, attributed against D's head before the rulings (`434139ef1c`, measured in a worktree) and, for sym11's `frozen`, main before D (`1fcd4bd807`):
- **Ids only (the identity bundle mints a different number of ids from the plane mate and three values it replaces; every standing copy is bit-identical to its source):** seat4 (heat_sink), seat7, seat8, lib_g16, perf2 (and the persisted text of every document with a stand), edit_placement_corpus_bits, name_words' digest, perf12 at all ε (byte-identical with every `digest:` masked), msolve14 a3 at all ε (green at `434139ef1c`), m10_p_fence's `f64` and Probe numbers (the id-free number is `434139ef1c`'s, every document's one by one).
- **FORK-REST (a mated placement runs its map where the snap placed it by no map):** seat4's empty boolean; m10_bulge (the literal D-tab runs its map, so both spellings share one table and the overlay retires); m10_10 ×2 and the plate's ledger in m10_sym_profile (the plate's holes; rows that read their rims doubled); m10_9 and sym_9's kept-atom row on the plate alone (945 -> 1253 symbolic); sym11's plate row; m10_p_fence's `Interval` number.
- **Moved before the rulings, on the lane of 2026-10-19/20 (main's merge at `0a1d95321e` and two corpus port fixes), left red there unattributed, and re-taken here at the values `434139ef1c` reads:** m10_9's other four documents and the pad's rule-F row, sym_9's other four and its drive leaf, sym11's other four, the slab's ledger, m10_6's certifying keys, m10_p_fence's id-free number, the boss's pcurve rows in m10_bulge.
- **The identity bundle in a stored document:** `crates/viewer/tests/gallery_ring.pncad` (regenerated from `demo-tour gallery`; the old ring's `Axis` mate, which the snap placed by no map, would now run its quarter-turn map) and `crates/pncad/tests/plate_param.pncad` (`PNCAD_BLESS`).
- **review_pick_r2's tally:** one landing moved, `tube_arc` after its first edit, whose copy stood by a mate read off its own geometry and now stands by the identity bundle (measured landing by landing against `434139ef1c`).
- **Restated:** `revalue_boss_union` owes no laundering (FORK-REST removes its `bool_pair_gap` swap; evidence on its inlane item); sym11's `frozen` column explained in its file (Degree freezes on products of a datum frame's forms before D; low-degree forms in a construction's own coordinates in D; Degree freezes back on the plate where its holes' maps run).

The exact-turn lane (2026-10-20), each with its reason in its file:
- **main's merge at `557e581a64`, measured identical with the turn form off:** m10_10 (fidelity's branches gone), m10_9 (door-shut receipts; the pad's rule-F row), m10_bulge ×3 (`pcurve_fidelity_branch` gone), sym11, sym_9 ×2, and m10_sym_profile's counts (PCERT B2b; `select_le_zero`'s tangents in `frozen`).
- **exact turns:** m10_sym_profile's two `Plain` digests; m10_5_r1's receipt (621 splits, 224 outside); m10_6_r1's straddle row restated (the web no longer straddles 2, so the bound sits ε/4 above it, inside the band); `die_tool.pncad` (`PNCAD_BLESS`: its ball's spin is the constant `0`, where it was a typed `6.1e-17 rad`); and, in the five documents that carry a written turn or a re-spelled writer (`kitchen_sink`, `die_pips`, `die_tool`, `die_composed`, the tour die): edit_placement_corpus_bits, lib_g16, perf2, name_words' digest, intent_s2_c_world, seat7 and seat8 (`kitchen_sink`; seat8's empty side, its tool read off the end cap), perf12 at all three ε (only `kitchen_sink`: its contacts read `0.0` where they read `1e-33`, and its curved pair is back in reach, 28 → 29 findings), and m10_p_fence's four numbers (the `Interval` one now one number at every ε).

#### Items filed

- `a-value-charted-on-the-target-reads-frame-carries-the-datum-chain` (P2): the (B) lever.
- `an-inline-at-a-moved-landing-reading-a-variable-refuses` (P2): the narrowing above.
- `a-loft-set-members-cannot-add-a-section` (P2): a members edit that adds a section has no station.
- `a-stand-on-a-revolve-shares-its-written-axis-numbers` (P1, design): closed by ruling 2 (`stand` writes the identity bundle, which reads no axis number).
- Closed by FORK-REST: `a-placed-copy-rests-at-the-identity-only-at-the-nominal`, `a-symbolic-runs-copy-decided-at-rest-keeps-its-map`.
- Evidence added: `a-turn-by-the-nearest-quarter-turn-rounds` (m10_di_dual's 16 escalations still occur; die_composed's fillet refuses on the Interval lane at ε = 1e-12, so m10_p_fence's Interval fence reads another number on the nightly's 1e-12 row), `under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move` (the swap is gone; the parting stays).
- TODO: from lanes.

- The exact-turn lane: filed `work/carve/a-partial-revolve-by-a-fraction-of-a-turn-ends-off-its-axes.md` (P3) and `work/msolve/an-axis-mated-spin-at-interval-widens-the-copys-translation-by-the-measured-spins-dust.md` (P3); closed `a-turn-by-the-nearest-quarter-turn-rounds` and SYM's `sin-and-cos-of-rational-turns-fold-exactly`; recorded the helpers' `atan2` residue on D′'s item.

#### The anchor census (for the record)

Runtime census of plane-less and loft constructions in the test trees before D (full list kept in the lane's scratch): full revolves in 195 tests (76 modules), partial revolves 82, full tubes 75, arc tubes 66, lofts 92. Corpus: 4 full revolves plus vessel's 2; tour: 4.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01ERgAp2pYACAfH3eGXR4Ppw

