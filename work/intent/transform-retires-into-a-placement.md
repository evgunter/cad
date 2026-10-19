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

**Known reds at the full-suite run on `ea25544a9a`:** 68 editor-core failures, listed with their owners above. pncad's 2 are re-bless. The viewer `--test all` suite needs a re-run (the 17 gallery_ring goldens are re-bless). The viewer's 2 lib reds are the GPU tests (no adapter). `every_suite_file_is_aggregated` fails only because of the local `zz_*` probe roots.

**Uncommitted, deliberately:** the `[[test]] zz_*` entries in `crates/editor-core/Cargo.toml` and the `tests/zz_*.rs` probe roots. Remove both before the PR (`git checkout crates/editor-core/Cargo.toml && rm crates/editor-core/tests/zz_*.rs`).

**Next, for a fresh lane**
1. `git fetch origin && git checkout intent/s3-d-transform-retires-into-a-placement && git pull`, then `export CARGO_TARGET_DIR=<one target dir> CARGO_INCREMENTAL=0`, then `cargo nextest run -p editor-core --test all --no-fail-fast` and `cargo nextest run -p viewer --features app,test-support --test all --no-fail-fast`. Settle each red per the three lane logs. The logs live in `/home/user` and do not survive the container, so this section names what each said.
2. Merge origin/main again if it has moved, then run `cargo clippy --workspace --all-targets -- -D warnings`, demos/tour's clippy and certified lane, and `python3 scripts/work.py lint`.
3. Open the PR from the draft below (the interim `stand`, pointing to D′; C-5's change; the re-bless log). Run the review pair at D's planned tier, with the row as the last commit, then merge (the orchestrator released the hold).

### Draft PR body

INTENT stage 3, PR D — `work/intent/transform-retires-into-a-placement.md` (spec `docs/INTENT-STAGE3-SPEC.md` §5, fork-log rows 159 and 161). Also closes `a-copy-standing-where-its-body-stands-has-no-d10-spelling` (C's world pin) and `datum-distance-reads-a-datum-node-not-a-pose`.

The loft station's D10 `[variables.kinds]` sentence was approved by Ev on #5295 (fork-log row 161).

#### What D builds

Frames enter only at placement, and nothing moves a body (D10; FORK-S3P, FORK-S3M).

**Constructions read no frame.** A profile is 2-D: `ProfileProgram.plane`, `Node::Datum` and every `Datum::*` are gone. An extrude goes up its own +z (`Extrude.side`, `ExtrudeSide` and `DocEdit::SetExtrudeSide` retire). A tube's spine is the circle about its own z (`Tube`/`HollowTube.frame` retire). Every construction is the root of its own space (`Space::Loose { root }`), so a node reading two constructions refuses `SpaceMismatch`; a later combine operand is placed against the first operand's root.

**A loft's sections carry stations** (row 161, Q2; D10 sentence approved on #5295): `Loft { first, then: [LoftSection { profile, station: [StationMove::Slide { along, by } | Spin { about, by }] }], v_degree }`, the moves in the first section's own axes, applied in order (`SlotId::StationSlide/StationSpin { section, step }`). `wire_loft` composes each station onto the first section's plane. `SetMembers` on a loft must keep its section count (`EditError::LoftSectionCount`), since a new section has no station. Content key: tag 15 retires, `Loft` is 41, and the move kinds and axes are keyed.

**Transform retires.** `Node::Transform`, `Placement`, `Step`, `PortKind::PlacedFrom`, the member walk's `Placing::Transform`, `VerbatimEdge::Whole` and the transform slots (`Translation`, `RotationAxis`, `RotationAngle`, `Normal`, `U`, `V`, `Spin`, `PlacementStep`) go. A rigid motion of a body is a `Place`.

**C's world pin retires** (row 159). `ConstraintKind::WorldPin`, `DocEdit::PinWhereItStands` and `InlineError::WorldPinOnAMovedWorld` go. A copy standing where its body stands is `mate::stand`: a `Plane` mate from a face on the copy's own xy that reads no variable (an extrude's or a loft's start cap; `Flip` on the copy's side where it looks down its z) to the world's xy, with the constant `0` on the three freedoms left.

**Plane-less roots stand on their own axis** (row 161, Q1). For a full revolve or a full-ring tube, `stand` writes an `Axis` mate with constant-zero `Slide(Z)` and `Spin`. Its `on` is the revolve's `axis` output where the copy is the revolve itself, else a circular edge of the copy minted by the revolve (the edit door lets `on` read only the placement's own shapes), or the ring's torus axis. The target is the world's x axis, spun about z by the heading and slid by `axis_through`, each move reading what its slot reads (a constant `0` slot writes none). `revolve_about` and the tour's diefillet, ring and teapot write `axis_through` as the constant zero, not `0 mm`.

**The placement helpers**, `mate::pin`:
- `stand`;
- `place_against(doc, target, body, at)`: a copy placed against a body's own geometry, either by a plane mate between own-xy faces slid along the normal, or by a frame read off the target for a tilted face, or by the copy's own axis for a revolve or ring;
- `place_at`: preference order is a face that reads no variable, then the copy's own axis, then any planar face, then the pin;
- `frame_at`, `pin_at`.

Each has an `_in` form over an evaluation the caller holds. C's interim report machinery (`Anchor`, `Placed::anchor`, `pinned_at`) retires. `mate::anchor::Anchors` stays: it picks the read that reaches no variable, for `stand`, `frame_at` and `place_at`.

**The values defect (ruled (A)).** A world mate's values are charted from the world's own frame, not from the moved read. So a datum's chain splits: the out-of-plane part goes on the world read, and the in-plane slides and the spin go on the values. Pinned by `intent_s3_d_placements::a_datums_in_plane_offset_rides_on_the_values`: a world read slid (3, 4) with zero values lands at the origin, and Slide X = 3, Slide Y = 4 lands at (3, 4).

**Inline at a moved landing.** A part placement whose world bundle holds numbers alone is re-pinned by `pin_at` at the copy's moved pose (new `InlineError::RepinRefused`). One that reads a named or toleranced variable keeps refusing (`ValueOnAMovedWorld`). This narrows what C's world pin allowed; the narrowing is filed as an item.

**Sketch on a face** is one gesture: `editor_core::sketch_on_face` / `Recording::sketch_on_face`, re-exported by the façade. It writes the profile, the extrude, its copy placed by `Flip(start cap)` → `Plane { face }` with Slide X = Slide Y = 0 and Spin = `spin`, and the union with the face's body. DM1 is restated to say so.

**Selections.** `GeomPred::DatumDistance` → `PlaneDistance { face, cmp, value }`, measured against a face of the selected body read as its plane. `SelectRefusal::{NotADatum, DatumHasNoValue}` → `{NotAPlane, FaceUnresolved}`. The datum face-frame refusals (`NodeErrorKind::FaceFrameNotPlanar`, `FaceFrameReadback`, `FrameDirection`) have no producer and retire.

**Names.** Unchanged from B: a copy's names are qualified `Placed { of }`, once per copy by construction. Spec §5's "pass through, as `Transform`'s did" predates B's qualification and is re-worded. This follows from B and is not a new decision.

#### Fixed in passing (each with a row)

- `place_against` slid a down-facing target the wrong way (`a_copy_slid_up_its_target_stands_up_it`).
- `payload_reach` read 0 for a boolean, split or instance copy, so every slide value on such a copy was undecidable. This is a pre-existing bug: C never wrote values on such a copy. No open item covered it. Row: `a_boolean_copy_stands_over_its_bodys_reach`.
- `Evaluation::loose_roots` counts loose COPIES only (new `Evaluation::loose_copies`); a construction root is no loose copy (`a_construction_is_no_loose_copy`). The façade's `step_for_node` writes a construction in its own coordinates and refuses only a node in a loose copy's space.

- `inline` at a moved landing re-pinned a nested instance as though it were a `Place` (`refactor.rs`, `unreachable!`). It now pins the instance through its own copies (`p2_split::i1`, `i2`; `p2_gauge_offsets::an_inline_at_a_turned_pose…`).
- `stand` on an instance chose the part's end cap. `Anchors::fixed` called every `InPart` name unfixed, so the end cap, which the part's height moves, was taken. It now reads an instance's face by the name's shape: a construction's start cap under the part's copies is fixed. The row is `an_instance_stands_on_its_parts_base`. The part's recipe is not reachable from the host's evaluation, so the remaining gap is the item `a-stand-on-an-instance-reads-its-part-by-the-names-shape`.
- `frame_at` and `place_against` refused a placed TARGET with `AlreadyPlaced`. They now read a placed copy where its placement moved it, with `at` given in the copy's own coordinates and carried by its motion. A placed copy as the body being placed still refuses. Row: `a_copy_stands_against_a_copy_already_placed`.
- A solved placement's map on the Interval lane, read off geometry a widened parameter moves, refused "whether the map is rigid … undecided". The enclosure's columns are too wide for `check_rigid`'s seven margins. The solve composes the map of rigid pieces only (orthonormal frames, turns about witnessed axes, slides), so where the door cannot call it, it is re-witnessed by Gram–Schmidt over its first two columns (`mate::solve::admitted`, funnel `solve_map_rewitness`). A lane the door calls, f64 included, keeps its bits. This had hit most of the m10 drive documents.
- The flip report had no words for the mate solve's predicates (`mate_*`, `solve_copy_at_identity`) or for the rigidity margins a placement's map is admitted through (`transform_rigid_*`). They are now worded in `mate::DECISION_WORDS`, which `decision::words` consults.

- **The box lanes' solved map stays tight** (24 m10 Interval rows; 16 were certification losses). Before D, a toleranced parameter moved a datum frame; in D it reaches the body through a placement's motion. The solve widened that map well past what the box justifies. Three fixes, each behind a `SolveScalar` hook that defaults to the old path, so f64, Dual and Probe keep their bits:
  - A value's spin is set from the two charts' frames (`Chart::turned`) where `spins_by_frames` holds, instead of turning the folded map by `v − atan2(…)`, which counted the map's width twice.
  - A direction two of whose coordinates are exactly zero is read as that axis (`mate::solve::tightened`).
  - **Fork C-5, changed (the orchestrator's ruling; C-5 is the C lane's code-level rule, 99774f9395, with no ratified clause behind it):** a symbolic map at rest now snaps where its own lane's does (`carries_no_variation` asks the lane scalar), where C-5 said "a symbolic map … never settles". Sym's verdict has to agree with its lane's: otherwise every drive over an identity-placed copy compares decision lists that differ and refuses `flip_crossing`.

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

#### Hit lists (the class swept)

TODO.

#### Tests deleted (retired subjects)

TODO from lanes.

#### Goldens re-blessed

TODO: what moved and why.

#### Items filed

- `a-value-charted-on-the-target-reads-frame-carries-the-datum-chain` (P2): the (B) lever.
- `an-inline-at-a-moved-landing-reading-a-variable-refuses` (P2): the narrowing above.
- `a-loft-set-members-cannot-add-a-section` (P2): a members edit that adds a section has no station.
- TODO: from lanes.

#### The anchor census (for the record)

Runtime census of plane-less and loft constructions in the test trees before D (full list kept in the lane's scratch): full revolves in 195 tests (76 modules), partial revolves 82, full tubes 75, arc tubes 66, lofts 92. Corpus: 4 full revolves plus vessel's 2; tour: 4.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01ERgAp2pYACAfH3eGXR4Ppw

