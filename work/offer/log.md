# OFFER — the log

## 2026-09-22 — opened

Cut out of CHROME, which was carrying 88 budget points against a
30-point ceiling, along its priority seam per `work/README.md` "Track
size" — see `work/forms/log.md`'s opening entry for the shape of the
cut, which opened FORMS in the same commit.

Seven rows arrived by `git mv` with their ids, bodies and history
unchanged. `viewer-expresses-no-gpu-adapter-preference` came here
rather than to VGEOM, which owns `gpu.rs` beside this program, because
it fails VGEOM's charter test (*a row belongs here only if a wrong
number, or no number, reaches the screen*): what the viewer does not
express is a preference, not a value. Band 10100-10199
is claimed for this program in the same commit (`docs/MODEL-AB-LOG.md`).
Nothing dispatched.

Signed (CHROME orchestrator).

- 2026-09-30 — Seam note from AUTH-9 (`author/declared-union`). `session.rs`: `commit_run` split into `stage_run` and `record_run`, and `add_boolean` judges its staged boolean before recording it (the undeclared-contact refusal and its offer). No offer-program path changed. (AUTH-9 implementer)

- 2026-09-30 — Correction from AUTH-9 (`author/declared-union`, PR #3543) to its note above, which said no offer-program path changed. That was wrong. The boolean door's undeclared-contact refusal is a refuse-then-offer on this program's ground: `Refusal::Contact` and its `RefusedBoolean`/`DeclareOffer` live in `session/refuse.rs` beside `offer_wording`, `frame::declare_offer` sits beside `creation_offer`, and `drafts.declare_offer` sits beside `new_param_offer`. The two homes, the staleness split and the doubled recourse are recorded on `a-refusal-offers-no-action-in-the-viewer`. (AUTH-9 implementer)

- 2026-09-30 — Seam note from AUTH-10 (`author/held-face-mark`, PR 3556). A pick a form or tool HOLDS has a mark of its own: the selection's colour (`Theme::held`), told from the live selection by shape (stripes on a face, a hollow line on an edge). `gpu.rs`: `ViewportCallback` takes one `Composed` (minted only by `pane::viewport::frame_marks`) in place of `highlight` and `edges`; `Uniforms` gains `held_ids` and a `held` mark lane read by both shader sites; `Uniforms::edge` is renamed `screen` (its `z` is a held face's stripe width); `LaneStyle` gains `hollow`; the edge shader's `across` varying takes default interpolation, since GLSL ES has no `noperspective`, and `every_entry_point_translates_to_glsl_es_300` translates every entry point through wgpu's naga; `EdgePass.held` is renamed `uploaded`, as is the scene-buffer closure in `ensure_geometry`. `session/refuse.rs`: `FaceFrameFault::NotDrawn` and `face_frame_seat_drawn` (re-exported from `session.rs`). (AUTH-10 implementer)

- 2026-09-30 — Seam note from AUTH-14 (`author/edge-name-fault`). The index's loud edge-name arms now reach the author. `pickindex.rs`: `PickIndex::edge_names_in(node, body)` walks a body's drawn edges and answers `EdgeNames { named, refused }`, where `EdgeNamesRefused { first: EdgeNameFault, refused, drawn }` renders through the fault's own `Display`; every refusal from that walk is loud (the ids come from the index's own window), and a body the index does not draw answers nothing and refuses nothing. `EdgeNameFault` gains `Eq`. Test doors: `PickIndex::unname_edge` (cfg(test)) plants the naming layer's refusal; `PickCache::index_mut` (cfg(test, app)). `blend.rs`: `load_all_edges` refuses the whole load with `BlendEvent::EdgesUnnamed { target, refused }` when any drawn edge refuses (was `NoEdgesOnTarget` when all did, a partial set when some did). `marks.rs`: `HeldEdges::segments` is `HeldEdges::mark`, answering the segments and the refusal; `EdgeOverlay::held_refused` carries it. `pane/viewport.rs` writes it per frame into `ViewerBehavior::held_edges_refused`; `app.rs` zeroes/assigns `ViewerApp::held_edges_refused` like `profiles_undrawn` and draws `frame::held_edges_badge` (Advisory, the pick-index seam's subject; `impl SeamSubject for EdgeNamesRefused`). `frame.rs`: `tool_notice` answers `EdgesUnnamed` `Retold::Again`. `gpu.rs`: one test literal gains the field. `test_support.rs`: `plate_indexed(tol)` (the fixture `marks.rs`'s tests held privately). README: the badge population is eleven. Tests: `frame_policy.rs` (badge count 11, the new retold row), `error_display.rs` (`edge_names_refused_forwards_its_first_refusal`), `blend_authoring.rs` (the held-set row reads `mark`). (AUTH-14 implementer)

- 2026-09-30 — Seam note from AUTH-14 (`author/edge-name-fault`), review fixes; supersedes the shapes in the note above. `pickindex.rs`: `EdgeNamesRefused` is `{ node, body, first: UnnamedEntity, named, refused }`, rendered as "the index names N of the M edges it draws on body B of node K; the first it cannot: <EdgeNameFault::Unnamed>"; `edge_names_in` reads the window through `PartWindows::named_in`, so only the unnamed arm can refuse; `EdgeNameFault` does not gain `Eq` after all. `blend.rs`: `BlendEvent::EdgesUnnamed { refused }` (no separate `target`). `frame.rs`: `SeamSubject for EdgeNamesRefused` is `Subject::Document` (the line's `tool_notice` subject too), not the pick-index seam. `test_support.rs`: `unnamed_edge(node, body)`. Filed `work/chrome/the-per-frame-badge-reads-are-three-hand-copied-fields`. (AUTH-14 implementer)

- 2026-09-30 — Seam note from AUTH-15 (`author/accept-part-version`, PR #3591). A new `SessionOp::AcceptPartVersion { id }` commits `pncad::workspace::update_to_store`'s edits (one `DocEdit::UpdateReference` per site whose pin moves) as one action. The store is read through a new `DocSession::read_store`, which `add_instance` and `part_catalogue` now share; `parts::catalogue` takes the scanned `&Workspace` and is infallible. `frame::version_offer(kind, files)` sits beside `declare_offer` and reads a `session::VersionOffer` off an instance's own `PartFault::Unresolved { fault: PinMismatch }`. `tree::TreeRow` gains `version_offer: Option<VersionOffer>` (every `TreeRow` literal needs the field). `DocSession::tree_rows` withholds every offer while `busy()`. `pane::features::feature_row_ui` draws `Refusal::version_question` and a `VersionOffer::LABEL` button under the row (`RowClicks::accept`). `DeclareOffer::ACCEPT_LABEL` and `DECLINE_LABEL` replace `pane::create`'s "Declare"/"Decline" literals. Every exhaustive `SessionOp` table gains the arm (`tools.rs`, `frame::acts`, the three in `session/op.rs`, and `tests/gesture_table.rs`, whose `OP_COUNT` is 47). `test_support::{PART_FILE, part_refused}` are new fixtures. Rows: `tests/instance_authoring.rs` (accept, one undo, the store's refusals, the committed-document read), `frame`'s `PartFault` census, the label-versus-recourse row in `session::refuse`, and a `pane::features` unit test.

- 2026-10-26 — seam from ATREST (stage 5 C, `the-at-rest-census-is-a-check`, branch `atrest/the-at-rest-census-is-a-check`): viewer `session.rs`: the at-rest badge reads the checks report; no offer path changed.

## 2026-11-22 — first sitting: the track claimed and re-cut

(OFFER orchestrator) Track set `active`. It stood at 34/30, and
re-pricing its five legacy `cost: D` rows would have raised it, so it
was cut along its charter before anything was dispatched:

- **To DOORS** (`work/doors/`), by `git mv`, ids and bodies kept: the
  three free-move rows, `viewer-free-move-misses-pattern-placed-mates`,
  `viewer-free-move-and-place-where-shown-over-a-whole-group` and
  `viewer-free-move-decides-rigidity-by-its-own-predicate`. They are
  one story, the free-move probe, and the second is P3 of the
  edit-placement spec whose residue DOORS already holds from PLACING.
  `crates/viewer/src/display.rs` moved from OFFER's paths to DOORS'.
- **To VSEAM** (`work/vseam/`), id and body kept:
  `document-seam-no-in-session-change-detection`. Its head is a memo
  served across a store change, which is VSEAM's charter ("an answer
  cached across a generation").
- **Re-priced**: `a-refusal-offers-no-action-in-the-viewer` D → H
  (a general offer type over three existing instances, plus the
  recourse separated from its explanation), design;
  `cascade-delete-shows-the-strand-count` D → M (its body is the
  spec); `viewer-free-move-misses-pattern-placed-mates` D → E (the fix
  is one comparison through `member_of`, plus its row);
  `document-seam-no-in-session-change-detection` D → M, design;
  `viewer-expresses-no-gpu-adapter-preference` D → M, design.
- **design: true** added on `certify-affordance-on-the-bounds-panel`,
  `a-node-name-the-viewer-shows-is-not-a-link-to-it` and
  `a-check-offers-no-way-to-add-the-assertion-it-asks-for`, which each
  carry a question with several viable answers.

The review posture the plan left open is answered there: the tiers
and the model comparison apply.

**Dispatch, letters and tiers** (logged before any draw):

- `cascade-delete-shows-the-strand-count` — letter M, tier SINGLE,
  FULL review: the pre-click number is a derived end-state count on a
  destructive button, and a wrong number there is a confident wrong
  answer.
- `the-viewer-has-no-guide-to-its-controls` — letter S, tier SINGLE,
  STYLE review: a new view drawn from the binding table; what is at
  risk is the structure (one source for bindings and guide), not a
  computed value.
- Designers, concurrently: the offer (`a-refusal-…` with
  `a-check-…`), a node named to a person (`a-node-name-…` with VNEWS'
  `the-viewer-spells-a-node-as-its-kind-before-its-name`), and the
  certify control.

## 2026-11-22 — the certify fork goes to Ev

(OFFER orchestrator) `certify-affordance-on-the-bounds-panel` has reconciled. The fork took two rounds, then a further designer after the seed swapped twice, then a final round, and every designer now agrees. It goes to Ev as an `[ev]` PR on `offer/certify-design` because it reads Ev's 2026-09-13 pacing ruling ("replaces the probe's reading") in a way Ev must confirm. The row's `## Design` section holds the agreed final state.

The fork turned up these findings, which the building unit owes:
- The row's signature is stale. `certified_range` now takes `stop: &Stop`, `SlotIsNotALiteral` is now `SlotIsDefined`, and there are eleven `RangeRefusal` arms.
- `docm9_range_vs_probe.rs` is not feature-gated; `tests/all.rs` includes it.
- The probe and the proof disagree on Count fields and on documents that already fail (`WitnessDoesNotBuild`). The row says the two "already agree on what they will not answer", which is true only of formula-driven slots.
- The probe's `Verdict` at `invalid` knows which node newly fails, but `Bound::Edge` drops it. That is the cheap source for "node stops building".
- Filed: `work/doctail/range-guard-refusals-lack-the-kernel-defect-ending` and `work/offer/the-bounds-probe-runs-inline-on-the-frame`. The resolver gap is already `work/scalar/the-box-driver-carries-no-part-resolver`, at P2. Since the certify unit needs that gap closed, its band is owed P1 when this unit is dispatched; the unit will re-band it or split it.
## 2026-11-22 — the names fork goes to Ev

(OFFER orchestrator) The fork on how a node is named to a person (`a-node-name-the-viewer-shows-is-not-a-link-to-it` with VNEWS' `the-viewer-spells-a-node-as-its-kind-before-its-name`) reconciled in one round. It goes to Ev as an `[ev]` PR on `offer/names-design`, because it changes DESIGN.md Band 1 "Node labels". The designers agree on the final state, and the PR carries their split on one taste sub-choice: how a label that two live nodes share reads on screen.

Points from the designers' orchestrator sections:
- The feature tree has no programmatic scroll today, so revealing the selected row is new work.
- The main feasibility risk is hit-testing mentions inside one laid-out text block. Neither designer checked it, and the building unit checks it first.
- `world::placements_of` and `copies_where` move into the kernel as views of `Doc::read_sites`.
- Re-baselining the plain-text spelling moves `crates/pncad-py/tests/test_document.py`, `test_monte_carlo.py` and `pncad.pyi`.
- `work/chrome/tree-rows-lead-with-raw-tags.md` closes with the vnews row; its placement-constraint half stays open on its own.
- `work/chrome/a-checks-cell-for-a-vertex-selects-its-node.md` is independent of this fork.
- The two relation rows (`a-made-body-…` and `a-placement-and-its-part-…`) are one unit: Properties shows operand slots, plus read-by.
## 2026-11-22 — the offer fork goes to Ev

(OFFER orchestrator) The fork `a-refusal-offers-no-action-in-the-viewer` with `a-check-offers-no-way-to-add-the-assertion-it-asks-for` has reconciled. It went one round, then a further designer weighed in after the designers swapped on two points, then a final round in which all designers agreed. It goes to Ev as an `[ev]` PR on `offer/offer-design`, because it adds a sentence to DESIGN `[one-way.places]`. The agreed final state and a correction of the row's stale evidence are in the refusal row's `## Design` and `## Correction` sections.

Rows the building unit owes, filed with it:
- a member-budget door;
- an expected-component-count door, together with the question of whether that count belongs in the document;
- a tolerance door, and the kernel row that types the tolerance recourse, which is prose in fourteen kernel files;
- a test listing the arms with no control.

Already filed: `work/clear/a-clearance-claim-and-a-zero-gap-value-say-one-thing-two-ways`.

Found and owed to the unit: `checks_window` offers "Add to the product" while a run is outstanding, which the one staleness rule fixes. `CERTIFY_STOPPED` names Re-evaluate and offers no button for it.

Unknown: whether `saying` can mint the one index-reaching assertion for a family row. Where it cannot, the finding names the construction alone and does not refuse at `Error`. That is a behaviour change, and the PR states it.

CHROME is told on its log when the unit dispatches.
**Dispatched 2026-11-22.** Implementers: `cascade-delete-shows-the-strand-count` on `offer/cascade-delete-shows-the-strand-count`, `the-viewer-has-no-guide-to-its-controls` on `offer/the-viewer-has-no-guide-to-its-controls` (draws on `analysis/model-compare/offer-2026-11-22`). Designers, two per fork, concurrently and blind to each other, with problem statements only: `offer-an-actionable-recourse`, `offer-a-node-named-to-a-person`, `offer-the-certify-control` (draws on `analysis/design-fork/<fork>`). Coupling notes left on VNEWS' and CHROME's logs.

**2026-11-22: three `[ev]` PRs are open.** Each one waits for Ev:
- names: #7339, on `offer/names-design`;
- certify: #7340, on `offer/certify-design`;
- offer: #7341, on `offer/offer-design`.

The design-fork rows are 297, 298 and 299. Their second commit (Ev's decision, and the label→model mapping from `analysis/design-fork/offer-*`) lands on each PR once Ev answers. The designers' full reports and round replies are in `~/.local/share/cad-work/offer-briefs/reports/`, which is this container only. The rows hold what moved in each round.
- 2026-11-22 — **Ev answered #7341:** "this all sounds good". The offer design is decided, with every lean taken. Fork-log row 299 is completed with the mapping: A Opus 5.5, B Fable 5.2, and further designer C Fable 5.2. `design` is cleared on both rows. (OFFER orchestrator)

## 2026-11-22 — the offer build, cut

(OFFER orchestrator) Ev decided the offer design on #7341, which is now merged. The build is cut into three units:
1. `a-check-offers-no-way-to-add-the-assertion-it-asks-for` is re-priced M → H and carries the kernel half: a finding carries the assertion that quiets it, with the interference face pair and the law test.
2. `the-viewer-has-one-assertion-tool` (new, M) is the general assertion door.
3. `a-refusal-offers-no-action-in-the-viewer` (H) is the offers function. It is parked on 1 and 2 and stacks on them.

Filed, as the design requires:
- `the-viewer-has-no-member-budget-door` (P2);
- `the-checks-expected-component-count-has-no-home` (P2);
- `the-tolerance-recourse-is-prose-and-has-no-viewer-door` (P2).

`program.md`'s keep_out clause "never a diff from here" was agent-written in CHROME's cut, with no ruling from Ev (`git log -S`). It now says that OFFER carries the decided kernel half and announces itself on ATREST's and COINCIDE's logs.

**Dispatch, letters and tiers** (logged before any draw):
- `a-check-offers-no-way-to-add-the-assertion-it-asks-for`: letter H, tier DUAL. It is the architectural pattern every later finding follows, and the minting rule is a soundness claim (an offered assertion quiets its row).
- `the-viewer-has-one-assertion-tool`: letter M, tier SINGLE, FULL review. It writes claims into the document, and a wrong claim shape fails silently: it would not quiet.

The local disk has 8 GB free and two lane targets are live, so these lanes run as their own cloud sessions.
- 2026-11-22 — **Dispatched as cloud sessions**, because this container's disk is the limit:
  - `a-check-offers-…` on `offer/a-check-carries-the-assertion-that-quiets-it`, session `session_013QoYWW3B3DSwxBw5VaTjDN`;
  - `the-viewer-has-one-assertion-tool` on `offer/the-viewer-has-one-assertion-tool`, session `session_013QoYWYBtUXfoqieYNpg1n1`.

  Each reports to the orchestrator's session by `send_message`. The draws are on `analysis/model-compare/offer-2026-11-22`. (OFFER orchestrator)
- 2026-11-22 — **Ev answered #7339:** names are decided, as recorded in the row's `## Decided` section. Fork-log row 300 (renumbered from 297, which another fork took on main) is completed: A is Opus 5.5 and B is Fable 5.2. The truncation row I filed on CHROME was reverted before it reached main, at Ev's word. (OFFER orchestrator)

## 2026-11-22 — the names build, cut

(OFFER orchestrator) Ev decided names on #7339, which is now merged. The build is cut into three units:
1. `the-viewer-spells-a-node-as-its-kind-before-its-name`, claimed from VNEWS. It is the kernel half (the typed sentence, mentions, the spelling) and leaves the viewer compiling on the new spelling. Once it lands, every surface already reads `"label" (tag)`.
2. `a-node-name-the-viewer-shows-is-not-a-link-to-it`: the sentence widget and links, plus the tree scrolling to the selection. It is parked on 1 and stacks on it.
3. `a-placement-and-its-part-do-not-link-to-each-other`: reads and read-by in Properties. `a-made-body-…` rides with it. It runs alongside 1.

**Dispatch, letters and tiers** (logged before any draw):
- 1 is letter H, tier DUAL. It is an architectural change to how every sentence the kernel hands out is built (56 `Say` impls, the Python stubs), and it would be hard to change later.
- 3 is letter M, tier SINGLE, FULL review. A wrong relation shown as a link is a confident wrong answer.
- 2026-11-22 — **Dispatched.** All four run as cloud sessions and report to the orchestrator by `send_message`.
  - **Names lanes:**
    - `the-viewer-spells-a-node-as-its-kind-before-its-name`, on `offer/a-node-is-spoken-as-a-mention`, session `session_013QoYWX8dyxZkisPkoWqKE2`;
    - `a-placement-and-its-part-do-not-link-to-each-other`, on `offer/a-node-shows-what-it-reads-and-what-reads-it`, session `session_013QoYWWdDR39ioDz6evAqWY`.
  - **Reviews:**
    - cascade PR #7346, FULL, session `session_013QoYWYJT7Yej2UM7GWwepM`;
    - guide PR #7336, STYLE, session `session_013QoYWVuZ9dUBajTwT442oB`.

  Draws are on `analysis/model-compare/offer-2026-11-22`. Each fix pass runs as its implementer's own agent, continued. (OFFER orchestrator)
- 2026-11-22 — **Ev answered #7340:** the sampled `range?` "can just be deleted". Ev never meant it to sample. `certified_range` is to be made efficient and to back a real `range?`. The design is in the row's `## Decided` section. Fork-log row 298 is completed (A Opus 5.5, B Fable 5.2, C Fable 5.2). Every designer had taken the probe as given from the row and the 2026-09-13 summary, and Ev rejected that framing. Worth noting for the experiment: the designers questioned the brief's premise on cost, but never on whether the probe should exist. (OFFER orchestrator)

## 2026-11-22 — the certify build, cut

(OFFER orchestrator) Ev decided certify on #7340, now merged. The build is cut into two units:
1. `the-certified-range-refines-away-from-the-current-value` (new, P1, H) is the kernel search.
2. `certify-affordance-on-the-bounds-panel` makes `range?` the certified search and deletes the sampling probe. It is parked on 1.

`the-bounds-probe-runs-inline-on-the-frame` is closed as superseded.

**Dispatch, letters and tiers** (logged before any draw):
- 1 is letter H, tier DUAL. It is a certified-interval soundness claim: the proven interval must be proven. It also sets a second refinement order beside E6's.
- 2026-11-22 — **Dispatched** `the-certified-range-refines-away-from-the-current-value` on `offer/the-certified-range-refines-toward-the-current-value`, as cloud session `session_013QoYWUSe878mMdYJzyx7Hd`. (OFFER orchestrator)
- 2026-11-22 — **A class two reviews found independently: a drift guard whose oracle copies the rule it guards.** In #7336, `names` in `tests/controls_guide.rs` re-spells `Held::met_by`. In #7346, the test helper `stranded` re-spells the viewer's `Holder`, which itself re-spells editor-core's `pub(crate) NameCarrier`. Such a test agrees with itself when the rule changes, and it cannot see mutations that move both copies in the same direction. Both fix passes now call the one home. **Standing note for this program's briefs:** a test that pins a rule calls the rule's home, or derives its expectation from a different fact. It never carries a second copy of the rule. (OFFER orchestrator, at adjudication)
- 2026-11-22 — **Merged #7336** (`the-viewer-has-no-guide-to-its-controls`). The review found 0 MAJOR. The fix pass took every MINOR. Its one deviation, the wheel's modifiers decided outside the table, is filed. Model-compare row: `docs/model-compare-log/rows/PR-7336.md`. (OFFER orchestrator)
- 2026-11-22 — **Merged #7346** (`cascade-delete-shows-the-strand-count`). The review returned APPROVE-WITH-FIXES with 0 MAJOR, and the fix pass took every finding. Model-compare row: `docs/model-compare-log/rows/PR-7346.md`. (OFFER orchestrator)
- 2026-11-22 — **Friction: runner queue.** Run 44988698571, the merge of main into #7336 at eae4ed99, had its gate jobs queued for a runner for about 23 minutes (05:23Z to 05:46Z) before any job started. That wait counts against neither CI bar; it is a finding in its own right. At least six OFFER PRs and lanes were pushing at the time. (OFFER orchestrator)
- 2026-11-22 — **#7356 review: NOT-MERGEABLE-AS-IS.** The MAJOR finding is that read-by drops readers at space seats, so a body does not list the placements standing in its space. Also: a 30× per-frame cost, an undisclosed reach of the Pick change into a seat decision, and a third kernel spelling of "placements reading a variable" beside `take_over::copies_reading`. The fix pass takes all of it. The same reviewer re-checks afterwards. **Class, logged here:** a "one home" fix that misses an older kernel spelling, because its sweep was scoped to the crate it was moving out of. (OFFER orchestrator)
- 2026-11-22 — **Main went red because of #7346.** The binding census was missing a `NameCarrier` row, and #7346's gate skipped the python suite. The fix is #7377, and the coverage gap is filed as GREEN's `the-python-suite-skipped-a-diff-that-changed-pncads-reexports`. This program owes the fix, and is making it. (OFFER orchestrator)
- 2026-11-22 — **Merged #7348** (`the-viewer-has-one-assertion-tool`). The review found no MAJOR, and the fix pass took every finding sent to it. The orchestrator merged main, which carries #7346's viewer changes, in before the row commit, so the gate ran on the combined tree. Model-compare row: `docs/model-compare-log/rows/PR-7348.md`. (OFFER orchestrator)
- 2026-11-22 — **#7362, the kernel half, is in dual review** on frozen head `2dcb9ce9`. The two reviewers run concurrently, R1 `session_013QoYWUAT3W3nMTd116A5LG` and R2 `session_013QoYWVZFmApiUBqAD4cssb`, from identical briefs, and the R1 slot draw is on the analysis branch. The fix pass will run off the union of both reviews' findings. #7348 is merged. (OFFER orchestrator)
- 2026-11-22 07:30Z — **Friction: runner starvation.** In #7377's run 44993815739, `test (app)` was still queued about 50 minutes after the change filter finished (06:41Z). OFFER alone had about eight branches pushing: #7356, #7362, #7369, #7375 and #7377, plus the merges. Per the orchestration model, runners are a budget. I am holding new dispatches until the PRs in flight drain; nothing was combined, because each of those PRs waits on its own review. (OFFER orchestrator)
- 2026-11-22 — **#7362 dual review: both reviewers returned NOT-MERGEABLE-AS-IS, with findings demonstrated.**
  - **Bilateral:** a partly resting family is offered a claim that is violated for its other members.
  - **R2 only, both MAJOR:**
    - two families on one index are quietable with no value carried, which opens a hole in the Error gate;
    - interference between members of one family offers a pair that no assertion satisfies.
  - **Root cause named by both:** the mint (`authored`) and the matcher (`reference`/`member_name`) are inverses kept by hand. The fix pass makes them one rule, adds completeness to the law test, and adopts every red probe as a row.
  - **One file:** ATREST gets a row for indexed reads in the quieting rule, the DESIGN "Families" gap.
  - **Records:** the correspondence pre-note and the reviewer figures are in the orchestrator's ledger, and land in the PR-7362 row at merge. (OFFER orchestrator)
- 2026-11-22 09:35Z — **Liveness: a fix-pass message did not reach the lane.** The orchestrator sent the #7362 fix-pass list at 08:24Z. An hour later the kernel-half lane's transcript did not contain it. The lane had spent that turn polling CI after merging main, which the list told it to defer. The list was resent at priority `next`. **Rule for this program:** after any `send_message` to a lane that is mid-poll, confirm on the next check-in that the lane's transcript contains the message. Do not take `delivered` as proof that the lane saw it. (OFFER orchestrator)
- 2026-11-22 — (DOCTAIL orchestrator) `editor_core::NameCarrier` (7be2080) arrived with no binding-census line, so `pncad-py`'s `test_binding_census::test_every_curated_name_is_bound_or_listed` reds every PR that runs the Python suite; main's push CI skips that job. DOCTAIL's PR 7151 ports the line. A follow-up that adds a façade name should run the Python suite or add its census line.
- 2026-11-22 — **#7369 (certify kernel search) is in dual review** on frozen head `4550e584`. The two reviewers run concurrently, R1 `session_013QoYWYajuFJPVgjqYbWhWG` and R2 `session_013QoYWYYdQ9GTU1mzZaUKF9`, with identical briefs. Soundness of the proven interval is claim 1. **For the viewer unit:** `DriveConfig::default()`'s max depth of 24 is too shallow for the plate, which needs about 30 halvings, so pass a deeper one. Filed: SCALAR's `a-replay-that-nearly-certifies-costs-fifty-times-one-that-stops-early` (P3). Closed: the pydoor row and DOCTAIL's range-guard row. The kernel-half (#7362) fix pass is under way; its fix list reached the lane late. (OFFER orchestrator)
- 2026-11-22 10:35Z — **Fable outage (model-compare rule 12).** The Fable limit is exhausted until Nov 28, 15:00 UTC. Stopped: the #7362 fix pass and #7369's R2 reviewer; #7356's re-review stops next. Ev was asked on #7362, with a recommendation to proceed on Opus. Nothing is dispatched in Fable's place until Ev answers. Opus lanes continue: the names sentence (#7375) and #7369's R1. (OFFER orchestrator)

- 2026-11-22 10:55Z — #7369 R1 (Opus) reported at frozen head 4550e584: APPROVE-WITH-FIXES. One MAJOR, demonstrated: once the answer is final the search keeps halving ground that cannot change it, and two_sided at the default depth spent its whole 5000-replay cap (599 s) after the answer last moved at replay 128. The certify row's handoff "a depth of 64 costs nothing extra" is falsified by it; the viewer unit must not be dispatched on that sentence. Also minors m1–m3, notes n1–n3 and style S1–S8. Filed to the ledger. Under the dual protocol, adjudication waits for R2, which is blocked on the Fable outage (rule 12, Ev asked on #7362). R1 was asked to push its probes to `offer/review-7369-probes`.
- 2026-11-22 12:28Z — **#7375 (names sentence) is green** at a4ab03e8 and ready for its dual review. The review needs a Fable reviewer, so it is held under rule 12 with the other three, and Ev's question on #7362 now names it. Whichever of #7375 and the relations PR #7356 lands second ports the other's call sites to Say/Sentence. (OFFER orchestrator)
- 2026-11-22 21:02Z — **Ev: "you can proceed on opus"** (#7362, 20:58Z). Five Opus agents were dispatched, and per rule 12 none of their four units records a model-comparison row (the analysis branch says so):
  - **#7362 fix pass:** a fresh lane, session_013QoYWUsboL9eSwNu3C7uW6, starts from the pushed head 52816df9. The stopped lane's unpushed work is lost; it had reached "kernel changes compile".
  - **#7369 R2:** session_013QoYWVGnKgUHg5VQdjC5vs, on R1's brief verbatim.
  - **#7356 re-review:** session_013QoYWV9sbfKxwJhVYc1tSG, a fresh reviewer at d496705c, given the first review quoted in full.
  - **#7375 dual review:** R1 session_013QoYWVsxumQTEB9XcsqaqD and R2 session_013QoYWU47o2kBoGV7ZA6Kdz, on identical briefs, at a4ab03e8.
  
  `[ev]` is off #7362's title. (OFFER orchestrator)
- 2026-11-22 21:34Z — **#7369 adjudicated on the union of R1 and R2.** Both reviewers say APPROVE-WITH-FIXES and found the same MAJOR independently: the proof is sound (0 violations; every soundness mutant red), but the search keeps working after its answer is final, and the handover's "a depth of 64 costs nothing extra" is false. Also found by both: the `Leaves { max_leaves: steps }` field and Python's known-bad default pair. R1 alone: the "builds EXACTLY" wording on canonical_order, inward rounding, and Python not resuming after a cancel. R2 alone: a Floor frontier can say "more time will not help" while more time could help; the cross-side order and tie-break are unpinned (two surviving mutants). The fix pass was cut to a fresh Opus lane, session_013QoYWV4UQ5Q5xCusSRdyv8, because the implementer was Fable. **The certify viewer unit waits for this fix pass and its corrected depth/cap advice.** No row (rule 12). (OFFER orchestrator)
- 2026-11-22 21:58Z — **#7356 re-review (Opus): NOT-MERGEABLE-AS-IS.** The first round is fixed, and all 7 of its mutants are now killed. But the fix pass's pick step in `placements_of_var` makes a clearance seated on a placement land silently on one picked piece. This is a regression against base, and an 8th mutant (no pick step) survives. Two smaller MINORs: the placement answers disagree on a family member; seat labels spell a mint ordinal. The second fix pass went to the original lane (Opus), session_013QoYWWdDR39ioDz6evAqWY, with the rule to keep "who reads" and "what copies" apart and to move no door's seat answer from base except `is_seen`. It does not port to #7375 until #7375 lands. The re-reviewer, session_013QoYWV9sbfKxwJhVYc1tSG, is kept for the delta. No row (rule 12). (OFFER orchestrator)
- 2026-11-22 22:12Z — **#7375 adjudicated on the union of R1 and R2. Both say NOT-MERGEABLE-AS-IS.** Found by both: the `name_words_corpus` slow pin is red at the head and was never run by PR CI, so nightly would go red; and mentions are still flattened silently. The flattening runs through about 69 `Display`-via-TAG impls, which the cfg-out method cannot see. It covers about 20 sites, among them the cross-document PartFault and a viewer tool notice, and no row can see it. R1 alone found a carried route stamping the top document on a mid-assembly hop (needs a data change); R2's document probe missed it. R2 alone found a weak Python needle. The fix pass went to the names lane (Opus), session_013QoYWX8dyxZkisPkoWqKE2. Required: fix the flattening road, not only its sites; a law row that can go red; one full slow-set run over the touched crates. **The cost of reviewing a unit twice shows here:** each reviewer caught a MAJOR the other missed. No row (rule 12). (OFFER orchestrator)
- 2026-11-23 01:22Z — **#7356 second fix pass reported** at 7062b7ae, `gate ok` green. MAJOR-A is fixed by keeping three questions apart: who reads (read-by keeps the pick), what copies (no pick step), and what follows a redefinition. Every door is back to its base answer, `is_seen` included; its hazard went as evidence on DOORS' family-copy row. MINOR-A moves base for one case (a named family member's copy counts as the family's body), and the PR body discloses it. Seat labels now read "constraint N". Filed: `properties-do-not-say-a-node-is-read-by-the-product-or-a-definition`. The delta went to the re-reviewer for round 3. (OFFER orchestrator)
- 2026-11-23 01:56Z — **#7362 fix pass reported** at 2ff3d4a5. The mint and the matcher are now one rule (`Saying::reads_cell`), and COMPLETENESS is in the law. Every item is addressed, and two rows were filed on ATREST (indexed reads in the quieting rule, P2; a family copy's overlap carries no pair, P3). CI: all green except `test (kernel)`, red on the TESSLIE P1 main red at 1e-12 (the sweep prism corner). The orchestrator reproduced that on main 6e9dd995, logged it on TESSLIE, and said so on the PR. A fresh Opus verifier, session_013QoYWYYhsMvnYUgNkSApvZ, checks the one rule, that COMPLETENESS can go red, items 1–3 against new variants, and the deviations. No row (rule 12). (OFFER orchestrator)
- 2026-11-23 02:31Z — **#7362 verifier: NOT-MERGEABLE, one MAJOR.** The coincidence side is now one rule, and COMPLETENESS goes red under 4 mutants. Items 1–3 hold against three new variants. But the interference side still has a separate mint (`could_quiet`) and matcher (`face_of`): an interference over a family copy carries `None` and passes the Error gate, although a family Gap quiets it. That is the reviewers' MAJOR-2 class, moved to interference. A second fix pass went to the lane: one rule on the interference side too, COMPLETENESS in its law, and a row for a split whose tool reaches an index. (OFFER orchestrator)
- 2026-11-23 02:36Z — **#7356 round 3: APPROVE** at 7062b7ae. Every applicable mutant is killed, and the MINOR-A base move is right. **The order changes: #7356 lands before #7375.** It is approved, #7375 is mid-fix, and the #7375 lane is told to port. One last small push was asked: the reviewer's product-door probe as a row, `source_root` reuse, a loud seat-label fallback, and a filed P2 row for the three answers to "the body a placement copies". Merge on green. No row (rule 12). (OFFER orchestrator)
- 2026-11-23 04:38Z — **#7375 fix pass reported** at a9e35b6, `gate ok` green, including ε 1e-6 and 1e-12.
  - **The flattening rule:** a thread-local `saying` depth covers every road into a sentence, and `Sentence`'s `Display` refuses inside it. Every sentence-bearing `Display` is one `display_said`.
  - **The fixes:** ~20 sites plus 3 the guard found are converted. `Route` hops carry their document. The golden is re-pinned (only `Union` moved, 1098 times).
  - **Filed:** `some-spoken-causes-and-reports-are-still-text` (P2).
  - **The slow-set run:** 12 red rows. 9 are GREEN's open P0. 3 are geometric rows outside it (`split_pinch_keeps`, `review_pick_r2`, `m10_sym_drive_memo`), and the verifier checks those against main.
  - **Verification:** a fresh Opus verifier, session_013QoYWU2NNCDAqrANhvMAVn, checks the guard's coverage, release behaviour, mutants, cross-document stamps, the re-pin's "only Union moved", and those 3 rows.
  - **Order:** #7356 still lands first, and #7375 then ports it. (OFFER orchestrator)
- 2026-11-23 05:50Z — **#7375 verifier: MERGEABLE-WITH-NITS.**
  - **Held:** the guard catches in-body flattening (41 mutant reds, all guard panics). Cross-document stamps are right to four documents. The re-pin is honest (only `Union` moved: the corpus's one label is on a union). The 3 extra slow reds are red on main identically.
  - **Nits:** F1, the new law row cannot go red alone, since the corpus is said by TAG and has no mentions; F2, the residue row misses two String causes; F3, `spell` is public and unguarded.
  - **Merge:** a9e35b6 now conflicts with main in `drive.rs`, so the lane is merging main and folding in F1, F2, F3 and a wording nit.
  - **The order is final: #7375 lands first.** The #7356 lane then merges main and ports its naming sites and `Related` onto the mention type. (OFFER orchestrator)
- 2026-11-23 06:30Z — **#7356 merged** (a6853f06) at c28488d. Its substantive jobs (lint, test (app), viewer wasm) were green; only the `gate ok` aggregator was still queued behind the backlog. **Ev (06:2xZ): with CI queuing this bad, a PR that touches only docs, comments or work items merges on a local `work.py lint`; and the orchestrator may skip CI on any PR after whatever local tests make it comfortable.** The two relations units are closed. The order flipped one last time: #7375's lane folds the #7356 port into the main merge it is already doing, and is told it may verify locally. No model-comparison row (rule 12). (OFFER orchestrator)
