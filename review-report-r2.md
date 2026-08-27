# GUI-2 (PR #1106) — reviewer R2 report

Frozen head `609c22d4`. Review branch `gui/gui-2-review-r2`. Consumer suite
`crates/viewer/tests/review_gui2_r2.rs` (28 rows: 27 green, 1 `#[ignore]`d because it is
RED against this head — it IS MAJOR-1, written as the gate it should become). Mutation
logs and run records under the review scratch; every mutation is reproduced in-line below.

## Verdict: APPROVE-WITH-FIXES

(four-term: APPROVE / APPROVE-WITH-FIXES / REQUEST-CHANGES / REJECT.) Conditional on hosted
green. Four MAJORs, three of them a few lines each. The unit is strong where it matters
most — the selection VALUE is clean, the survival semantics work on every document I threw
at them, `landed_pair` is genuinely correct, and 26 of my 28 independently-derived rows
were green on first run. What did not survive is one highlight lookup, the seal's central
argument, and the row standing in for the GPU pass's geometry.

## MAJOR

**MAJOR-1 — the highlight marks the wrong patch when one name is drawn more than once.**
`pick::highlight` (pick.rs:482–497) reads only `FaceSelection::name` and lights
`ids_of(name).first()`, discarding the `node` and `body` the selection carries for exactly
this question. The shape is reachable: two `Transform` roots over one extrude is legal (the
root set is the DAG's sink set and the shared extrude is an ancestor of both sinks, not a
root — doc.rs:110–118), and `Transform` is a pass-through contributing no role segment, so
both copies carry the extrude's names. Measured on that document: `6 names drawn more than
once out of 12 ids`; selecting a face on the RIGHT placement lights `PatchId { node:
RecipeNodeId(2), body: 0, patch: 0 }` — the left one. Deliverable 4 ("the selected patch
rendered visually distinct") is not met there, and the ray/id comparison at app.rs:629–636
reads `highlight.hovered` rather than the ray's own patch, so the same shape makes #1097
§4's sweep report a permanent false disagreement over half the model. `pick.rs` also
contradicts itself: lines 210–213 say the `Vec` exists because "the highlight should light
every occurrence rather than the first". Row:
`the_highlight_marks_the_selected_bodys_patch_not_another_with_the_same_name`.

**MAJOR-2 — the façade seal's stated argument is false as compiled code, and it is the
whole argument the census rewrite rests on.** §9 and `pncad/tests/all.rs`:2624–2632 say a
consumer "can read and `Debug`-print those fields but cannot spell their type, so it cannot
store one in its own state — which is what the seal is for." The WRAPPERS are nameable and
public: `Resolved` is `Copy`, `Tombstone`/`MeshPatchKey` are `Clone + PartialEq`, so an
ordinary layer-3 struct field `Option<pncad::select::Resolved>` stores an `EntityRef` with
no unnameable type anywhere, and the derived `PartialEq`/`Ord` compare arena keys minted by
two DIFFERENT evaluations — the body-lineage-scoped comparison G1's rule exists to forbid.
Measured, compiled and run through `pncad` only: `stored a Resolved in a consumer field;
cross-evaluation kept == second is true; EntityRef { body: 0, key: Face(FaceKey(1v1)) }`.
**The shipped selection value is clean** — `FaceSelection` is `StableName` + node + body and
I verified it structurally — so this is not a G1 violation in the code; it is a widening
whose defence does not hold, stated three times (PR §9, select.rs:81–91, all.rs:2624). Row:
`arena_keys_can_be_stored_and_compared_through_the_widened_door`.

**MAJOR-3 — the headless row standing for the id pass's transform cannot see the failure
class #1097 books as hardware-only.** `the_id_passs_transform_samples_the_pixel_the_ray_was
_cast_through` (select_pick.rs:368–423) asserts only that the hit point lands INSIDE the
target — and the hit point is the cursor's own point, i.e. the transform's fixed point,
where every scale and every sign is invisible. Planted mutations, whole `viewer` suite:
`column[0] = (… ) * -sx` → **136 passed, 0 failed**; `column[1] = (…) * (sy * 100.0)` →
**136 passed, 0 failed**. That is precisely #1097 §4 failure 1 ("the 1×1 trick's sign or
scale is wrong"), which the issue schedules as a hardware question; it is not one. (The
sanity check: dropping the `- cx * w` translation DOES redden it, and flipping `ndc_y` in
`ray_through` reddens two rows — so the row is not inert, it is aimed at the one point
where the property is trivially true.) My row
`the_sampled_pixel_is_one_pixel_wide_and_correctly_oriented` — which asserts that half a
pixel out lands on the target edge and a whole pixel out lands outside, with the y sign
pinned — reddens on both mutations. Claim 4 is falsified for this row; the other agreement
row (`the_ray_paths_answer_is_the_id_maps_inverse`) DOES go red (`by_name` off-by-one), so
claim 4 half-holds.

**MAJOR-4 — leaving the viewport prints a permanent false "picking paths disagree", and
#1097 §4 tells the operator to misdiagnose it.** app.rs:638 builds an `IdQuery` only when
`cursor_px.is_some()`. When the pointer leaves the pane, no new query is issued, `id_serial`
is not reset, `id_answer` still holds the last id under the cursor, and `session.hover()`
has been cleared to `None` — so the guard at app.rs:629 still matches and `from_gpu != 0 ==
from_ray` fires on every subsequent frame. #1097 §4 says "Off the model both answer nothing:
id `0`, ray `0`, so no message", and lists `R32Uint` clear semantics as the diagnosis for a
message over empty space. The unit's one-gesture hardware check will therefore report a
failure it does not have. Found by reading; GPU-gated, so not reachable headlessly.

## MINOR

- **M1 — the error path now re-tessellates every root on every frame.** `sync_scene`
  (app.rs:243–276) added `index.current_for(...)` as a conjunct of the early-return guard.
  When `PickIndex::build` or `index.scene()` refuses, `self.index` stays `None`, so the
  guard is false forever and the whole build is re-attempted each frame, leaving a stale
  picture under a current `scene_generation`. Before this unit the `scene_generation ==
  landed` guard stopped after one attempt.
- **M2 — the pairing fix was not swept to its sibling.** `DocSession::tree_rows`
  (session.rs:741) still calls `tree::rows(self.doc(), self.evaluation())` — the SHOWN
  document against the landed evaluation, exactly the "run that never happened" pair §5
  argues against; and `open()` (session.rs:1036) clears selection and hover but not
  `landed`/`landed_doc`/`landed_generation`, so a second document renders against the
  previous one's evaluation until the run lands. Measured: `tree_rows while a run is
  outstanding: 2 rows before the edit, 1 after; landed_pair still names 2 nodes`. Q4's
  sibling-sweep obligation: an invariant discovered by a fix protects only the code that
  already knew.
- **M3 — the cost claim is understated.** §5 says "one `Doc` clone per landed evaluation".
  `request_eval` clones twice (session.rs:1082, 1085) and `land` a third time
  (session.rs:797): **two added clones per evaluation cycle** — superseded requests pay too —
  plus two retained `Doc` copies (`requested_doc`, `landed_doc`) where the body claims one.
  `DocSession::new`'s `requested_doc: doc.clone()` (session.rs:592) is dead: `request_eval()`
  on the very next line overwrites it. `Arc<Doc>` would make the claim true.
- **M4 — the carried set includes the raw-assembly lane, unused, while §10's disposition
  denies it exists.** `MeshPick`, `MeshPickError` and `PickTarget` (all fields `pub`) are
  carried together, so `pncad::select` now offers `MeshPick::build(&mesh)` +
  `PickTarget { node, body, pick }` + `pick_face` to every façade consumer — the confident-
  wrong-answer lane #1098 names. §10 keeps the witness row ignored because "the viewer never
  assembles a raw `PickTarget`", which is true of `PickIndex` and not of the door this PR
  opens. `MeshPick`/`MeshPickError` have no consumer here (12 of the 22 newly-carried names
  don't), and the census guard checks carried-or-listed, never carried-and-used.
- **M5 — `SceneError::MispairedIds` is a new fail-loud arm with no row anywhere.** Its own
  doc calls it "the silent wrong answer the whole id mapping exists to make impossible" and
  nothing goes red if the check is relaxed. Added
  `a_part_whose_ids_do_not_pair_with_its_patches_is_refused` (short AND long, plus the empty
  case that must stay legal).
- **M6 — gpu.rs claims a movement gate that does not exist.** `read_id_at`'s doc says the
  query "only runs when the cursor moves inside the viewport"; app.rs:638 issues one on every
  frame the cursor is inside, moved or not. The blocking readback is therefore per-frame, and
  #1097 §4's named remedy ("gate the query on the cursor having MOVED") reads as already done.
- **M7 — the `Display` gap is a class reported as an instance.** `PickError::Display`
  debug-renders BOTH payloads and only `HitTestError` is reported; `CameraError`
  (camera.rs:120) is dismissed in the same comment as "this crate's own and pre-existing" with
  no finding. `CameraOpError`, `SceneError`, `SceneDocError`, `ReplayError`, `StartupError`
  and this unit's own new `IdMapError`/`PickIndexError` also lack `Display` — and
  `PickIndexError` reaches the user's status line as `format!("pick index: {error:?}")`
  (app.rs:266). Sweep the class, don't fix the instance.
- **M8 — `read_id_at` leaks a mapped buffer on its own error paths.** `get_mapped_range()
  .ok()?` and `view.get(..4)?` return without `unmap()`, so the next call's `map_async` on an
  already-mapped buffer is a validation error rather than another `None`. Unexecuted.
- **M9 — a row's name promises a case it does not exercise.**
  `select_pick::a_selection_with_no_evaluation_behind_it_is_not_reported_as_live` uses a
  plate session that HAS an evaluation and tests a `Node` selection; the
  `Standing::Face { resolution: None }` arm the name names is untested. My
  `a_selection_with_no_run_behind_it_is_not_live` covers it (a held seam, no run landed).

## NOTE

- **N1 — read "hosted green" at its width.** `scripts/ci-filter.py` SAMPLES one lane and one
  ε row per run (ratified 2026-08-22). Run 33119990541 drew interval + ε=default; `build +
  archive (default)`, plain `clippy` and the default-feature test legs were all SKIPPED, and
  the 1e-12 draw #1102 names never ran on this head. Correct by design; stated so nobody
  reads the green as matrix-wide.
- **N2 — GQ6-RESURVEY §3's role assignment is inverted and nothing records it.** §3 puts the
  GPU id buffer on "hover/click exactness" and the CPU ray on "snapping"; this unit makes the
  ray authoritative and the id pass advisory, argues it well in the PR body, and changes no
  doc — the PR touches zero files under `docs/`.
- **N3 — the `powi(2)` comment postdates its code by one commit** (`git blame`: 1787864902 vs
  1787866912, commit 51276182). It is honest ("at f64 the two are the same number" — true),
  but the gate's own header says powi(2) is NOT universally tighter (a square below 2^-960
  pads once more), so the forward-looking half is slightly stronger than the ratified rule.
- **N4 — a drag both moves the camera and picks.** input.rs's module docs say "An event is at
  most one of the two — a drag moves the camera and picks nothing." True per event; false for
  the stream the app builds, since app.rs pushes a `Hover` on every frame `hover_pos()` is
  `Some`, including every frame of an orbit drag. So orbiting runs a full ray cast and a
  blocking GPU readback per frame.

## Adjudication input

- **Claim 8 (the display-path deviation) — FIX, once per landed evaluation.** The stated
  reason ("a verdict computed to be thrown away is a second opinion about what is on screen")
  frames it wrongly: it is not a second opinion about the picture — the triangles are
  identical, as §4 correctly argues — it is the ONLY opinion about whether the document's
  product is well-formed. "The tree already badges the failures that matter here" does not
  hold: `RowStatus` is per-node Failed/Poisoned read off the evaluation, and a naming-collision
  refusal in the GATHER is not a node failure, so nothing badges it. §4 concedes the outcome
  ("a document whose product refuses for a NAMING reason now draws instead of showing that
  refusal") — a wrong-but-rendered body with no channel saying so. One `product` call in
  `sync_scene` beside the index build is one graft and no second tessellation.
- **Claim 12 (`HitTestError` Display) — issue, not fix-now, but the issue must exist and must
  be the class.** A third edit to `resolve/` in a unit whose spec fences that directory is the
  wrong trade, and #1103 is the right precedent — but #1103 IS an issue, and "worth an issue"
  is not a schedule (Protocol v5 / style Q6). File it covering M7's class, not the instance.

## Style

Confidence per finding. Nothing here gates.

**Q1 — almost-but-not-quite parallel roles.** (a) `NodePick::build_all`
(editor-core/src/resolve/pick.rs:304–350) copies `build`'s entire 20-line node-standing match
verbatim; two spellings of one ladder, and the intermediate `indices: Vec<u32>` it then
consumes is unnecessary (both borrows are shared). **sure.** (b) `IdMap::NOTHING` is named in
five doc comments and spelled as a bare literal at every code site: `scene.rs:259`
`unwrap_or(0)`, `gpu.rs:676` `unwrap_or(0)`, `gpu.rs:364` `LoadOp::Clear(Color::TRANSPARENT)`,
WGSL `0u` twice — a class; the WGSL two are unavoidable, the Rust three are not. **sure.**
(c) The pixel→NDC conversion `2*px/w − 1`, `1 − 2*py/h` is written at camera.rs:600–601,
app.rs:640–643 and twice more in `select_pick.rs`, with no home. The disclosed-copy grep
(`verbatim|re-derived|mirror of`) over the touched area finds none of these — a clean prose
sweep is evidence about the prose. **likely.**

**Q2 — a comment doing the code's work.** (a) pick.rs:210–213 and pick.rs:488–497 are two doc
comments in ONE file stating opposite highlight rules; the field's stated rationale is
unimplemented (MAJOR-1). **sure.** (b) `PickIndex::scene`'s `self.id_slice.get(next..next +
patches).unwrap_or_default()` (pick.rs:340) hedges the invariant the comment three lines above
asserts, and the fallback is indistinguishable from "unpickable part" rather than loud.
**likely.** (c) pick.rs:29 — "there is no door through which it could" — is true of `PickIndex`
and false of the `pncad::select` door the same PR opens (M4). **sure.**

**Q3 — can the test fail.** Covered above: MAJOR-3's two green mutations, M5's untested arm,
M9's mis-named row. Also, `the_ray_paths_answer_is_the_id_maps_inverse` never asserts
`key.patch`, so shifting the patch index in the key list leaves it green (measured: only
`distinct_patches_never_share_an_id_across_bodies` reddens); my
`the_ray_path_and_the_id_map_invert_each_other_patch_included` closes that. **sure.**

**Q4 — invalidated premises.** M2 (the sibling `tree::rows` pair), N2 (GQ6-RESURVEY §3),
M4 (#1098's witness disposition vs the widening). On the good side: input.rs's "the primary
button is bound to nothing, on purpose and only until GUI-2" paragraph was correctly rewritten
rather than left to rot, and #1097 §2 was correctly amended to name TWO pipelines. **sure.**

**Q5 — promised vs contained.** M6 (gpu.rs's movement gate). And §11's sweep hit list, run
again with a different pattern — field TYPES rather than names, across the whole crate —
turns up one the list misses: `DocSession::requested_doc: Doc<ProfileProgram>`
(session.rs:553), a third retained document copy added by this unit and argued nowhere
(`landed_doc` is argued in §5; `requested_doc` is not in §6 or §11). It is squarely inside
§11's own stated class. **sure.**

**Q6 — deviations scheduled.** The GPU half is scheduled (#1097 §4, extended properly and
with the three real failure modes named — that part is exemplary). The `HitTestError` gap is
disclosed and unscheduled. **sure.**

**Q7 — is this how I would have done it.** (a) `ViewerBehavior` now carries 12 borrowed
fields and `viewport_ui` runs ~150 lines doing six jobs (event assembly, unit conversion,
camera fold, pick path, highlight, id query, paint); it wants splitting. **likely.** (b)
`id_answer: Arc<AtomicU64>` packing `serial << 32 | id` is a hand-rolled sequence lock where
a `Mutex<Option<(u32, u32)>>` costs nothing at one write per frame and needs no reserved-zero
convention. **unsure** — the `Arc` argument for avoiding the render-state borrow is real. (c)
`Standing` is recomputed twice per frame for a face selection (`properties_ui` computes it,
then `slot_rows()` computes it again), so `resolve` runs twice a frame; "recomputed, never
cached" is a good rule that wants one call site per frame. **likely.**

**Q8 — whole file.** Read `crates/viewer/src/app.rs` (1099 lines) end to end. It has grown a
titled section per unit and the accumulation now shows: three panes' worth of widget code, the
gesture translation, two dialogs, the layout, the matrix conversion and the whole cursor path
in one module whose header says "toolkit adaptation, and nothing else". Nothing in it is
wrong; it is the shape that stops being read. **likely.**

## Code quality report

Counts: **MAJOR 4, MINOR 9, NOTE 4.** Deviations **reported** by the PR: 9 (the display-path
gather verdict; `HitTestError`'s `Display`; the soft edge on `Resolved::entity`/`MeshPatchKey`;
the #1098 witness disposition; the two status-line fixes; the two `editor-core` extensions;
the pncad widening; the unexecuted GPU half; `cursor_projection` moved out of the feature
gate). Deviations found **silent**: 4 (MAJOR-1's contradicted field doc, M1's per-frame
rebuild, M6's non-existent movement gate, Q5's `requested_doc`).

1. **Idiom / structure — 4/5.** `PickIndex` is one value with one constructor, no repair door
   and `op_for` really is the shipped path the tests drive; against that, `ViewerBehavior`
   reached 12 borrowed fields and `viewport_ui` ~150 lines inside a single unit.
2. **Test quality — 3/5.** 26 new rows, all green, mostly well aimed — but the one row
   standing for the GPU pass's geometry stays green under `* -sx` and `* (sy * 100.0)`,
   `MispairedIds` has no row at all, and one row's name promises a case it does not reach.
3. **Doc / comment honesty — 3/5.** The disclosure discipline is unusually complete and
   #1097 §4's three failure modes are exactly right; against that, the seal's central argument
   is false as compiled code, gpu.rs claims a movement gate that is not there, and two doc
   comments in one file state opposite highlight rules.

## End-to-end exercise — scope, and what was NOT exercised

Driven as a consumer through shipped doors only, on the committed gallery ring
(`gallery_ring.v14.pncad`, ε re-stamped from the serializer): `SessionOp::Open` → 3 tree rows,
1 drawn part, 4 ids → camera framed on `scene.bounds()` → cursor derived by projecting a drawn
triangle's centroid (the pane centre looks THROUGH the ring's hole — an honest miss, and worth
knowing) → `pick_stream` → `op_for` → hover then click → `Selection::Face(node 2, body 0,
Band(loop 0, segment 1))` → tree row and 1 slot row off the one value → `DeleteNode` → `live()`
false with `Failed(ResolutionFailure { error: NodeGone { name: …, edit: NodeDeleted { node:
RecipeNodeId(2) } }, offers: [] })` and empty slot rows → `Undo` → live again, same panel,
nothing re-picked → the pre-edit index correctly not `current_for` the new generation.
Separately: undo across the selection's BIRTH and redo, on a pattern; a consumed instance; a
held evaluation seam proving the landed pair never mixes generations.

**Ergonomics.** The API is genuinely good to consume: `op_for` collapses the whole cursor path
into one call a test drives identically to the app, and `landed_pair` is the right shape — I
could not construct the mismatched pair even trying. Two frictions: (1) there is no way to ask
"which ids belong to this (node, body)" — `ids_of` is name-keyed only, which is what MAJOR-1
falls out of; (2) building a `PickIndex` requires four arguments a consumer must keep in step
by hand (doc, eval, generation, δ) when three of them come from the same `DocSession` — a
`DocSession::pick_index(δ)` would remove the one pairing left to the caller.

**Not exercised, exactly.** (a) The GPU pass itself — `IdPass`, `read_id_at`, both WGSL entry
points, the `R32Uint` clear, the readback: `viewer::gpu` is `app`-gated and private, so nothing
in `tests/` can reach it; MAJOR-4 and M6 and M8 are read, not run. (b) The egui widget mapping —
`viewport_ui`'s `hover_pos`/`clicked_by` translation, `standing_ui`, the tint shader — no
toolkit in a headless run. (c) Hardware: sign/scale of the 1×1 trick beyond what MAJOR-3's new
row now pins, culling, frame rate under the per-frame blocking readback. (d) `crates/bvh` —
confirmed untouched by this PR (diff over `66460de4..609c22d4` touches 15 files, none under
`crates/bvh`), so GUI-1's ray query rides its own gates.

## Claims disposition

1. **G1 across the new door — HOLDS for the value, argument FAILS for the door.** `Selection::
   Face` is name+node+body, verified; the seal is a type trick that does not seal (MAJOR-2).
2. **The `pncad::select` door — coherent, and the cut is one name family too wide.** The family
   rewrite reads correctly and rejecting a direct `editor-core` edge is right (it would hand
   layer 3 `EntityRef`/`EntityKey`/`Entry` outright); `MeshPick`/`MeshPickError` carry the raw
   lane for no consumer (M4). 88→66 verified against the census's own failure output.
3. **The two `editor-core` extensions — both arguments hold.** `build_all`'s "the indices are
   not a dense range" is true (`sources_of`: a split's empty half occupies 1 and not 0);
   `patch_names`' "the key never leaves" is true (patch index in, name out). `crates/bvh`
   untouched, confirmed. Style residue only (Q1a).
4. **Ray-authoritative / id-compare — HALF.** `the_ray_paths_answer_is_the_id_maps_inverse` can
   go red; `the_id_passs_transform_samples_the_pixel…` cannot, for the class it exists for
   (MAJOR-3). And the comparison the app actually runs is against the highlight, not the ray
   (MAJOR-1).
5. **Un-projection — HOLDS, re-derived.** Two boxes, four aspect ratios including a tall pane
   and a square one, corners plus face centres, residual `< 1e-12 · distance`; plus the
   direction the unit does not take (cursor → ray → point → project → same cursor, random
   cursors, varying seed); plus centre→view-axis with left/right and top/bottom symmetry; plus
   all eight non-finite inputs and four area-less panes, where the unit checks two.
6. **IdMap — HOLDS.** Round trip both directions on keys of my own, NOTHING reserved in both
   directions, past-the-end and `u32::MAX` are `None`, the empty map is legal, single-field
   neighbours never collide, repeats refused, three-instance pattern gives patch 0 three ids,
   re-indexing one generation is identical. The hardware caveats are honest and complete.
7. **Cache + survival — HOLDS; the cost sentence does not.** `PickIndex` keyed on (Generation,
   DisplayTolerance), stale not current, no repair door; three survival rows plus redo all
   reproduce on my own fixtures; `landed_pair` verified against a held seam — the pair never
   mixes generations, and the "run that never happened" is genuinely unreachable through it.
   The #1098 witness's kept-ignored reason is sound as far as `PickIndex` goes and is undercut
   by M4. Cost is two clones, not one (M3).
8. **Display-path deviation — FIX.** See adjudication.
9. **Status-line fixes — HOLD, behind rows.** `perform_batch`'s `acted` flag and the
   `folded.applied.is_empty()` land condition are both right; my replay row
   `the_two_mappings_partition_the_event_stream` confirms no event is read by both mappings.
   Undercut in the app by MAJOR-4, which puts a permanent message back in the same line.
10. **CI — VERIFIED at step level.** 33119200338 red on `interval-square powi(2) allowlist` at
    `crates/viewer/src/camera.rs:615`; 33119337802 red on
    `every_document_layer_root_export_is_carried_or_listed` naming all 22 names; 33119990541
    conclusion `success` with `viewer toolkit rows - the filter's verdict` (step 10) and
    `clippy (viewer app feature - eframe + wgpu)` (step 11) both `success`, not skipped, and
    the filter emitting `tier=closure pkgs=editor-core,pncad,pncad-py,viewer`. Read N1 beside it.
11. **The sweep — one miss.** The three `ViewerApp` dispositions are right, hover's one home is
    right, and the two stated blind spots hold (`grep` for `data_mut|memory(|memory_mut|.data(|
    persist|Id::new` over `crates/viewer/src/` finds nothing; `ViewportRenderer::geometry` is
    the only GPU-side copy). My re-run by field TYPE finds `requested_doc` (Q5).
12. **`HitTestError` Display — issue, and it must be the class.** See adjudication + M7.

## Isolation and glimpse disclosure

`docs/MODEL-AB-LOG.md` not opened. `gui/gui-2-review-r1` never fetched, checked out or read.
One incidental glimpse to disclose: while searching for the four-term verdict vocabulary I ran
`git log --all --diff-filter=A --name-only -- "review-report-*.md"`, whose output included the
commit SUBJECT `445243d6 review(gui-2 r1): full report`. I read no content from it; the
vocabulary came from `24b93306` (GUI-3 R2, closed history). No model identifier appears in
anything pushed on this branch, and there is no `Co-Authored-By` trailer.

## Cost

Wall clock ≈ 60 min. Tokens ≈ 300k. Local runs were unique signal only — six planted-corruption
mutations, my own consumer suite, the gallery e2e, and the merge-base differential on
`landed_pair`; no CI-covered battery was re-run for its own sake. Verdict conditional on hosted
green.
