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
