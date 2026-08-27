# Review R1 — PR #1106 (GUI-2: viewport selection), frozen head 609c22d4

**Verdict: APPROVE-WITH-FIXES** (conditional on hosted green at the merge head;
609c22d4 itself is green — run 33119990541 verified at step level).

Probes: `crates/viewer/tests/review_gui2_r1.rs` (7 gating rows + 1 ignored
evidence probe, all green, 143-row viewer suite green around them),
`review-probes/gui2-r1-mutants.md` (4 mutants). Branch `gui/gui-2-review-r1`.

## MAJOR

- **M-1. An index that cannot be built is rebuilt on every repainted frame.**
  `crates/viewer/src/app.rs::sync_scene` (lines ~241–280): the early-return now
  requires `index.is_some_and(current_for)`, so when `PickIndex::build` (or
  `index.scene()`) errors, `index` stays `None` and every subsequent repaint
  re-enters the rebuild — `NodePick::build_all` re-tessellating every healthy
  root per frame until the failing arm is reached. Any failed/poisoned ROOT is
  enough (`build_all` refuses on `NodeResult::Failed`), and that is a routine
  editing state (GUI-3's `broken_document`, any bad dimension). GUI-3's code
  retried once per landed generation (`scene_generation` alone gated). During
  this state the viewport also silently keeps the PREVIOUS generation's picture
  with only a debug status line — same as GUI-3, but now paid for per frame.
  Sure on the control flow (code reading; app-gated, so not demonstrable
  headlessly). Fix is small: remember the generation whose build failed.

## MINOR

- **m-1. The seal's prose overclaims; the leak is one type-inference away.**
  PR §9 / `crates/pncad/tests/all.rs` census: "a consumer … cannot spell their
  type, so it cannot store one in its own state." Falsified by a compiling
  probe (`evidence_probe_an_arena_key_is_storable_and_comparable_via_inference`,
  ignored, green): `struct Stash<T>(T); Stash(resolved.entity)` stores the bare
  arena key in layer-3 state and `==` compares it across runs, `pncad` only —
  `Resolved` is `Copy` and `entity` is a public `PartialEq` field. The seal
  blocks *naming*, i.e. accidental misuse; it does not block extraction,
  storage, or comparison by a determined consumer. The shipped viewer stores
  none (verified), so G1 holds for the code that exists; the census sentence
  and PR wording should say "unnameable, deliberate contortion required," not
  "cannot be stored."
- **m-2. The two status-line fixes are prose, not rows.** Both live in
  `app`-gated, never-executed code with no replay coverage: `perform_batch`'s
  hover-only-batch rule and the fold-landed-only-when-moved rule
  (`app.rs:309–330, 570–578`). The charter asked "verify behind replay rows" —
  there are none, and none can exist while the policy sits in the frame loop.
  `lib.rs`'s claim "everything between [event conversion and painting] … is all
  exercised by tests/" is thereby false: status policy, hover dedup, and the
  id-compare are between the two ends and untested. Factorable per G1's own
  discipline.
- **m-3. One name under several ids: false disagreement + half-lit highlight.**
  `pick.rs` `by_name` doc says a name CAN be drawn more than once and "the
  highlight should light every occurrence rather than the first"; `highlight()`
  lights the FIRST only, and `app.rs`'s ray/id comparison compares raw ids, so
  on hardware a cursor over the second occurrence prints "picking paths
  disagree" when both paths named the same face. Compare names (via `name_of`),
  not ids. Likely (fixture not built; the code asserts reachability itself).
- **m-4. `build_all`'s no-empty-vector contract has a false arm.**
  `editor-core/src/resolve/pick.rs` doc: "A node denoting no body answers
  NotABody rather than an empty vector" — but `sources_of` answers
  `Some(vec![])` for `BooleanValue::Empty`, so a fully-annihilated boolean root
  returns `Ok([])`. Harmless downstream today (draws nothing), wrong contract.
- **m-5. The landed-pair cost accounting is understated.** PR §5 says "one
  `Doc` clone per landed evaluation"; `request_eval` now clones the doc twice
  per request (`requested_doc` + the submit) plus once per landing plus once at
  construction. Real cost is fine; the stated number isn't the whole of it.

## NOTE

- n-1. Charter correction (dispatch is a hypothesis): the green run's seeds
  were `editor-core,pncad,viewer` — `pncad-py` was NOT a seed (no pncad-py file
  changed); the toolkit rows drew RUN regardless.
- n-2. The shipped transform-agreement row is blind to SCALE errors in
  `cursor_projection` (mutant 2 stayed green: its cursor is derived from the
  same projection, so the bounded residual is sub-pixel before scaling). The
  promoted R1 row pins shift AND scale exactly, so the headlessly-checkable
  half of #1097 §4's "sign/scale" caveat is now covered.
- n-3. `resolve`'s own docs say "Prefer `resolve_with_prior` whenever a
  last-good run exists"; the session discards the prior landed pair at `land()`
  and calls single-run `resolve`, so the unresolved UI shows the poorer
  diagnosis — while the entire prior-diagnosis vocabulary (`Tombstone`,
  `Diagnosis`, `TieWitness`, `MeshPatchKey`, `RecipeEditRef`,
  `resolve_with_prior`) was carried through the new door unconsumed.
- n-4. #1106's `HitTestError` Display gap is disclosed but unscheduled — "worth
  an issue" is not an issue number; per the Q6 discipline the fix pass should
  file it (same for the display-path `product` question if taken as a followup).

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (Q8: read `pick.rs` whole,
`session.rs` whole in two passes; `gpu.rs`/`app.rs` via full diff + key regions).

- `pick.rs:210–215` vs `pick.rs:476–482` — the `by_name` field doc and
  `highlight`'s doc state opposite policies for multi-id names (every
  occurrence vs first). One of them is the intent; the code implements the
  other. **sure** (also m-3).
- `app.rs:642` vs `camera.rs:601` — the px↔NDC conversion (with its y-flip) is
  spelled independently in the id-query path and in `ray_through`, plus its
  inverse four times in tests; no one home, and a y-flip drift between the two
  spellings is exactly what the hardware comparison would misdiagnose.
  **likely**.
- `app.rs` ray/id comparison compares ids where the property is about NAMES —
  stricter than the claim "both paths answer the same query," and the source of
  m-3's false positive. **likely**.
- Status policy in the frame loop (m-2) — an invariant held in unexercisable
  code where a pure function would do; not how I'd have done it (Q7). **sure**.
- `standing()` runs the full resolve ladder every frame for a face selection
  (`properties_ui` → `standing()` per repaint); fine at gallery scale, an
  unmeasured per-frame cost on large documents. **unsure**.
- "0 = nothing" is spelled three ways: `IdMap::NOTHING`, a bare `0` in
  `scene.rs::build_parts` (`unwrap_or(0)`), and `0u` in WGSL. The scene one
  could name the constant. **likely**.
- `sync_scene`'s doc "Gated on the generation so a frame that changed nothing
  re-tessellates nothing" is false in the error state (M-1). **sure**.
- Q1 prose+constants sweeps over the touched area: no undisclosed copies found
  beyond the conversion duplication above.

## CODE QUALITY REPORT

Counts: 1 MAJOR, 5 MINOR, 4 NOTE, 8 style. Deviations: 5 reported in the PR
(display-path, Display gap, GPU-untested, cursor_projection relocation,
landed-pair cost) vs 4 found unreported (M-1 retry loop, m-3 doc/code
contradiction, m-4 false contract arm, m-5 cost understatement).

- Idiom/structure: **4/5** — the value-first decomposition (PickIndex, IdMap,
  Standing, one `op_for` path) is genuinely good; docked a point for policy
  logic stranded in the app-gated frame loop and the M-1 control-flow slip.
- Test quality: **4/5** — 26 new rows, all four of my pick.rs mutants caught by
  the joint suite (each by some row); docked for the transform row's scale
  blindness and the plate-only weakness of the inverse row's key asserts
  (mutant 3 note in the probe record).
- Doc/comment honesty: **3/5** — unusually honest about GPU debt and the soft
  edge, but the census "cannot be stored" claim is falsified by a compile, and
  two doc/code contradictions (m-3, m-4) plus a stale gating sentence (M-1)
  shipped in one unit.

## End-to-end exercise

`e2e_a_gallery_ring_is_picked_edited_killed_and_revived`: opened the committed
gallery ring (re-stamped to the run's ε), aimed the cursor by projecting a
front-facing triangle centroid of the drawn tessellation, drove
Hover+Click through `pick_stream`→`op_for`→`perform`, verified hover/selection
agree, tree row and slot rows reached through the ONE `Selection::node()`
inversion, `resolve` green on the landed pair; deleted the owner via
`SessionOp::DeleteNode`, saw the typed unresolved standing, empty slot rows,
stale index; undid, saw the SAME un-re-picked value live again and a fresh
index answer the same name at the same cursor. Ergonomics: the consumer API is
pleasant — the whole walk needed no viewer internals; the one wart is that a
consumer must know to rebuild the index itself (`current_for` says stale, and
nothing library-side owns the rebuild loop — which is where M-1 bit the app
too). NOT exercised: the GPU id pass and shaders (never run anywhere), egui
widget→event mapping, `perform_batch`/status policy, the rfd dialog, the
threaded seam (inline evaluator used; threaded rows are GUI-3's), highlight
uniform consumption. Local runs unique-signal only; verdict rides hosted green.

## Claims disposition

1. HOLDS for the shipped code (no arena key stored in layer 3); the "cannot
   store/compare" prose overclaims — the seal is a naming barrier, not a
   capability barrier (m-1, evidence probe).
2. HOLDS — the family rewrite is coherent, the cut is picking+resolution
   exactly (nothing outside those families left NOT_CARRIED), the direct
   editor-core edge rightly rejected (it would hand layer 3 nameable
   `EntityRef`/`EntityKey`); GUI-1's endorsement banked this exact door, so no
   reversal of substance; slight speculative width (`resolve_with_prior`
   vocabulary unconsumed, n-3).
3. HOLDS — both arguments verified against `sources_of` (indices non-dense:
   Split keeps layout indices; probing would be by-hand pairing) and
   `entity_name` (key never leaves); `crates/bvh` untouched; m-4 is a doc arm.
4. HOLDS with a mapped edge — mutants 1/3/4 turn the agreement rows red;
   mutant 2 (scale) is a shipped blind spot, now pinned by a promoted R1 row.
5. HOLDS — shipped corner rows verified, plus my random-camera/pixel inversion
   sweep, refusal rows, centre→axis; different body (ring) and aspects.
6. HOLDS — round-trip, cross-body (theirs) and cross-node (mine)
   collision-freedom, NOTHING=0, re-index stability; hardware caveats in
   #1097 §4 are honest and specific (verified against the issue text).
7. HOLDS — (Generation, δ) key, discard-whole, no repair door on the type;
   witness-row reason judged sound (gating would check a pairing the keys
   don't carry); survival rows + redo green in both suites; `landed_pair`
   pairing is correct because `land()` drops non-current generations before
   reading `requested_doc` (verified incl. mid-gesture scratch pairing);
   cost accounting understated (m-5).
8. Judgment: consult `product` once per landed evaluation and surface its
   verdict (badge or status) — the tree badges cover per-node failures but not
   gather-level naming refusals, and no other viewer path computes them; once
   per landing is cheap and is not a per-frame second opinion. Not blocking.
9. PARTIAL — the mapping halves are behind rows (primary-click-moves-no-camera
   etc.), but the two fixes themselves are in app-gated code with no replay
   rows (m-2); the reasoning in the comments is correct as far as reading goes.
10. VERIFIED — run 1 red exactly on the interval-square gate at camera.rs:615
    (fix 51276182 is the ratified powi(2) spelling); run 2 red exactly on
    `every_document_layer_root_export_is_carried_or_listed`; run 3 GREEN with
    seeds `editor-core,pncad,viewer` (not pncad-py — n-1), verdict step printed
    the three RUN lines, doc-gate ran un-skipped, clippy-app step compiled the
    eframe/wgpu graph — all read at step level, not job names.
11. HOLDS — my own sweep (struct fields + `data_mut|memory(` grep) found the
    same inventory and no additional shadow; the three new fields' dispositions
    are accurate (the atomic is a channel; serial disambiguates NOTHING); the
    stated blind spots are the real ones, plus mine: closures over `Arc`
    (id_answer is the one instance, and it is the disclosed channel).
12. Judgment: issue-worthy, not fix-now — the #1103 shape fits (payload owner
    should grow Display; composing wording in viewer would be worse); but per
    Q6 it owes an issue NUMBER at the fix pass (n-4).

## Isolation / glimpse disclosure

Did not fetch, check out, or read `gui/gui-2-review-r2` or its artifacts; no
PR comments read beyond the body. Two incidental exposures to disclose: (1) the
BINDING reading itself — `docs/GUI-LOG.md`'s GUI-3 unit entry ends "(block
GUI-B1 slot 4, arm opus)", naming GUI-2's drawn arm; I did not open
`docs/MODEL-AB-LOG.md` or any other A/B material, and nothing in this review
keyed on it. (2) `with-build-slot.sh`'s waiting line printed another lane's
held-slot command (`cargo build -p viewer --features app`) — a command name
only, no artifact content. No model identifier appears in anything pushed.

## Cost

Tokens: ~310k of the session budget at report time (15.00M → 14.69M).
Wall-clock: ~1h25m from charter receipt to report delivery. Local runs:
3 suite runs + 4 mutant runs + clippy/fmt, all under `with-build-slot.sh`
with a worktree-local target; no detached waiters left running.
