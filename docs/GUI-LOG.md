# GUI log — the v1 GUI program

Narrative record; the plan is `docs/GUI-PLAN.md` (RATIFIED
2026-08-27), the architecture `docs/GUI-DESIGN.md` (G1–G3, GQ1–GQ7)
with `docs/GQ6-RESURVEY.md` as the toolkit/viewport/picking factual
record. Convention as in the other programs: seam entries at
pipeline seams, unit entries at merges, the tail is the live state.

## Opening state (2026-08-27)

Opened on Evan's go ("the program is ready to start whenever you
want"), the same day the plan was ratified and merged (#1087). The
program is the plan's six units GUI-0 … GUI-5 (GUI-5 optional,
GUI-6 banked post-v1); every design decision the units lean on is
ratified elsewhere and cited from the plan — nothing here
re-litigates.

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `gui/`** — unit branches
  `gui/<unit>-<slug>`. The orchestrator works on the session branch
  `claude/v1-gui-orchestration-s72sh0` (harness-designated; the
  prefix convention's `gui/orchestrator` name is not available to
  this session, so away-channel filtering for this program should
  include both prefixes if armed).
- **A/B ordinal band: GUI = 400–499**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in the same commit that
  opens this program, per that entry's rule.
- **Implementer blocks: named `GUI-B1`, `GUI-B2`, …** — the `B`
  avoids colliding with the unit names GUI-0 … GUI-6, which the
  other programs' `<PROGRAM>-<n>` block convention would do here.
  Draws recorded in `docs/MODEL-AB-LOG.md` beside the other
  programs' blocks.
- **This session runs in a remote container**, not the mngr
  worktree environment the operational memories assume: no
  persistent `~/.local/share/cad-work`, no statusline usage
  monitors, GitHub through MCP rather than `gh`. Implementer lanes
  are isolation-worktree subagents; the memories' substance
  (build-slot mutex, per-lane `CARGO_TARGET_DIR`, push early and
  often, CONFLICTING = silent CI outage) binds unchanged.

**Dispatch slate at opening:** GUI-0 (scaffold spike) and GUI-1
(`Bvh::ray` + hit-test service) are independent of each other and
both dispatchable immediately; specs `docs/GUI-0-SPEC.md` and
`docs/GUI-1-SPEC.md` accompany this entry. GUI-2 consumes GUI-1's
service and GUI-0's viewport; GUI-3 consumes GUI-0's chrome; GUI-4
consumes GUI-2+GUI-3; GUI-5 is stretch.

**Open question carried from the plan:** OQ-b (docking crate,
`egui_tiles` vs `egui_dock`) — decided inside GUI-0, rationale in
that unit's PR.

## Dispatch record (2026-08-27, after #1088 merged the opening)

Both units dispatched concurrently as isolation-worktree subagent
lanes, arms read back from the block GUI-B1 draw record in
`docs/MODEL-AB-LOG.md` at dispatch (the VERBS-4 deviation remedy)
and echoed verbatim:

- **GUI-0** (docs/GUI-0-SPEC.md), branch `gui/gui-0-scaffold` —
  arm per draw record: **slot 1 = OPUS**.
- **GUI-1** (docs/GUI-1-SPEC.md), branch `gui/gui-1-ray` — arm per
  draw record: **slot 2 = FABLE**.

Briefs point at the spec + `docs/prompts/implementer-discipline.md`
by path and carry both halves of the foreground rule, the
build-slot mutex, lane-private output paths, the no-trailer
blinding rule, and the CONFLICTING-CI rules. Implementers open
their PRs; reviews dispatch at PR-open per protocol v6
(cross-model dual, banded ordinals from 400, parity byte drawn per
dual at dispatch).

**State-sync shape (2026-08-27, adopting the amended protocol in
`memories/orchestration-model.md` the day it landed, #1095):**
unit rows and log entries ride the unit's own PR as its LAST
commit, pushed only after both reviewers' reports are delivered
(the row names the arm; a docs commit on a branch reviewers have
checked out breaks blinding through their own `git log`), and the
merge does not wait on a fresh CI run when that commit is
docs-only atop an already-green head. Dispatch-time records that
must exist before reviews complete — the v6 parity byte, frozen
head, ordinal claim — go on THIS orchestrator branch, pushed
immediately: the GUI band (400–499) has a single claimant, so the
band makes the claim raceless and the record reaches main with the
next merge that carries this branch. Design conversations,
protocol amendments, and spec ratifications keep their own PRs.

**Ruling — viewer CI posture (Evan, 2026-08-27, in-conversation,
on his own proposal):** the GUI is treated as a third-party
consumer of the API. Concretely, landing in the GUI-0 fix pass:

- The toolkit-touching steps (`clippy -p viewer --features app`,
  the doc gate's `--all-features` pass over viewer) run only when
  the change filter's **seeds** (crates whose files changed)
  intersect {`viewer`, `pncad`, `bvh`} — seed-keyed, not
  closure-keyed, because `pncad` is in every kernel change's
  closure but seeds only when its own files change.
- The skip is a **recorded axis in the filter output** (the
  klint_row lesson — never a green job name over a silent skip),
  and a viewer row joins the nightly lane to re-take the gated
  coverage against toolkit-dependency drift.
- Viewer's default-feature build and headless tests stay in the
  ordinary dependent closure: `pncad` mostly re-exports, so a
  breaking change to a re-exported type does not seed the façade —
  the cheap in-closure rows are what put that breakage (and
  behavior drift caught by the volume/winding tripwires) on the
  offending kernel PR instead of an innocent later one.
- This settles the GUI-0 implementer's adjudication item 1 (the
  doc-gate `--all-features` question) in the gated direction.
- Extracting `viewer` from the workspace as its own root (own
  lockfile, the `benches/` shape) is deferred to v1's close —
  GUI-2…4 are the maximum-churn window for viewer↔editor-core
  co-evolution.

Next actions: liveness check-ins on both lanes; at each PR-open,
freeze head, claim ordinal (recorded here, pushed), dispatch the
v6 dual; the ruling above lands with the GUI-0 fix pass.

**Incident (2026-08-27 ~17:25Z): container restart killed three
in-flight review lanes** — GUI-0 R2 and both GUI-1 reviewers
(GUI-0 R1 had already delivered). All three isolation worktrees
survived with their branches (GUI-0 R2 with local commits past the
frozen head); all three resumed by message with the post-restart
cautions (cwd reset, suspect in-flight results, push-early, the
isolation rule re-stated now that `gui/gui-0-review-r1` exists on
origin). Rows at merge annotate the wall-clock gap per the
recording discipline; reports must disclose the interruption for
the v6 fair-pair adjudication.
Next actions: dispatch GUI-0 and GUI-1 per the block GUI-B1 draw;
reviews per protocol v6 (cross-model duals, banded ordinals from
400) at each PR.

## GUI-0 merged (2026-08-27, PR #1094, sample #29 / ordinal 400)

The `viewer` crate exists: eframe + `egui_tiles` chrome (OQ-b
closed in-unit), thin wgpu viewport drawing an evaluated document
at display-δ, typed renderer-free camera operations, 45 headless
rows. Both reviews APPROVE-WITH-FIXES, all substantive findings
bilateral, v6 tally contribution 0; full record in the
MODEL-AB-LOG row. The program-level outcomes:

- **The seam-friction reading is provisional by design**: no §5
  fallback condition met, but the spike edits nothing — GUI-3
  re-takes the measurement where it counts.
- **Evan's viewer-CI ruling is live**: seed-keyed toolkit gate
  with a published skip axis, nightly viewer row, doc-gate split.
  The SKIP direction's first hosted exercise is the next
  kernel-only PR — whoever sees that run confirms the verdict
  step drew SKIP (not silence).
- **#1097** banks first light on real hardware (+ culling flip,
  winding check).
- GUI-3 is now unblocked (chrome exists); GUI-2 additionally
  wants GUI-1's service (in fix pass at this writing).

## GUI-1 merged (2026-08-27, PR #1093, sample #30 / ordinal 401)

The hit-test service exists: `Bvh::ray` with the conservative-
superset contract now magnitude-universal (the review pair's
bilateral overflow refutation fixed in code — both reviewer
witnesses gate green), and `ray → StableName` through
`editor-core::resolve::pick` with typed errors end-to-end plus the
`NodePick` atomic provenance door (#1098 documents the raw-target
class it closes). A 9,000-ray reviewer differential against an
independent oracle found zero disagreements. Census NOT_CARRIED
stands with both reviewers' endorsement; the curated
`pncad::select` picking door is banked as a future decision.
Program consequence: **GUI-2 is unblocked** (viewport selection —
consumes this service and GUI-0's viewport; block GUI-B1 slot 4).

**Sequencing call (2026-08-27, orchestrator, L-numbered: L-GUI-1):
GUI-2 holds until GUI-3 lands.** Both units edit the same files in
`crates/viewer` (app chrome, input bindings, gpu module); the
crate-disjoint lane rule exists because same-crate concurrency
buys cross-lane merges instead of work. GUI-3 was dispatched first
(it re-takes the §5 seam measurement, which can also inform GUI-2's
selection-state shape); GUI-2's spec is written when GUI-3's PR
freezes, and its dispatch takes block GUI-B1 slot 4 (arm opus per
the draw). Cost: one idle dispatch slot for a few hours; the plan's
1-before-2 ordering already made GUI-2 non-blocking for GUI-3.
## GUI-3 merged (2026-08-27, PR #1101, sample #31 / ordinal 402)

The document panels exist: tree with typed badges, property panel
with the refusal affordance, the evaluation seam (cancel semantics
hardened at the fix pass — canceled runs never land; both seam
implementations coalesce and are covered by threaded rows), the
sibling-minting undo tree, typed open/save, and the demo-document
gallery (scene list corrected: ring, not scalar). **The §5
seam-friction re-take is GO on egui, authoritative** — the
iced-fallback question is closed for v1 barring new evidence.
Riders: the viewer toolkit CI rows moved to the every-lane fmt job
(the lane-sample gap both reviewers confirmed, now structurally
closed); #1103 banks the expression unparser; the merge inherited
main's #1102 red at the 1e-12 draw (cited, not this unit's).
Program consequence: **GUI-2 dispatches now** (spec staged; block
GUI-B1 slot 4, arm opus); GUI-4 wants GUI-2 + GUI-3.

**Process defect, mine (2026-08-27, disclosed by GUI-2's R1): a
GUI-LOG unit entry leaked the NEXT unit's arm to its blinded
reviewers.** The GUI-3 entry above says "(block GUI-B1 slot 4, arm
opus)" — GUI-2's arm — and GUI-LOG unit entries are in reviewer
binding reading. R1 disclosed the exposure (did not open A/B
material; nothing keyed on it); R2 has the same reading. Recorded
on the GUI-2 row for the blinded adjudication to weigh. THE RULE
THIS BUYS: **a log entry visible to reviewers never names an arm
for any unit whose reviews have not concluded** — arms live in
MODEL-AB-LOG (reviewer-fenced) until then; this file references
slots only. The offending line is left as-is (history is not
rewritten; the leak already happened) — future entries comply.
## GUI-2 merged (2026-08-28, PR #1106, sample #32 / ordinal 403)

Viewport selection exists and the G3 interaction set is complete
minus assemblies: click-to-select with the one selection value
shared between viewport and tree, hover, highlight, survival under
vanishing refs, and the GPU id pass — which EXECUTED for the first
time during the fix pass's screenshot capture (lavapipe; geometry
correct, no false disagreement; #1097 hardware modes stay open).
Three program-level outcomes recorded:

- **#1098's raw-target lane is closed structurally**: the census
  trim made `MeshPick` unnameable at the façade, so `PickTarget`
  has no reachable constructor — the confident-wrong-name class
  is now unreachable from layer 3, not merely documented.
- **The ID-buffer/ray roles inverted vs GQ6-RESURVEY §3** (ray
  authoritative, id-buffer comparative), argued in the PR and now
  stated at the code seam; recorded here so the design record
  reflects practice.
- **#1111** banks the Display-gap class (eight error types + the
  editor-core pair).
- The pair's blinding was contaminated by the orchestrator's own
  GUI-3 log entry (rule recorded above); flagged on the row.
- #1102's main-red resolved independently (#1108, the census
  owner's ε-fix) — the GUI program's citations of it are closed.

Program consequence: **GUI-4 dispatches now** (block
GUI-B2 slot 1) — the last required v1 unit.

**The blinding-leak rule fired a SECOND time (2026-08-28, disclosed
by GUI-4's R2): the GUI-2 unit entry above named GUI-4's arm — 
written by this orchestrator AFTER recording the rule.** The arm is
now redacted from that entry (docs state the present; the leak
itself is history both reviewers' briefs required disclosing —
R2 disclosed, R1's exposure is presumed identical). The GUI-4 pair
is flagged on its row like GUI-2's. Compliance correction adopted:
the at-merge unit-entry TEMPLATE now ends "(block <id> slot <n>)"
with no arm token, and the pre-push self-check for orchestrator
log commits is `grep -i 'fable\|opus' docs/GUI-LOG.md` scoped to
entries about units with unconcluded reviews.

**Ruling — block-draw disclosure (Evan, 2026-08-28,
in-conversation, closing the #1112 thread):** verifiable
precommitment of block draws is NOT required; hashes are permitted
but optional. GUI adoption: block draws are recorded at draw time
on the orchestrator branch (pushed = durable; arms read back from
it at dispatch per the VERBS-4 remedy) and the block record merges
to main only once the block's last reviews conclude. Already-
exposed blocks (GUI-B2's remaining slots included) carry
contamination flags on their remaining duals' rows.
## GUI-4 merged (2026-08-28, PR #1113, sample #33 / ordinal 404) — THE REQUIRED v1 PATH IS COMPLETE

Assembly interaction and the mate tool exist, and the plan's
acceptance is EVIDENCED ON THE REAL GALLERY: the render lane runs
the tour's gallery exporter and an exit-nonzero probe on every
real document (10/10 open and resolve; the flat-pack's patterned
posts hide and probe through consuming-edge ancestry; constrained
instances refuse typed; no accepted-but-inert operation anywhere),
alongside the ten-stage fixture walk whose in-file argument states
why no single real document can carry the whole sequence. The
admission verdict outlives commit (tree note + the once-per-landing
A5 at-rest badge). Residue banked: #1117 (save-a-copy identity),
the `SetPlacement` vocabulary gap (R1 n8 — issue at close), GUI-5
(optional, Evan's call), GUI-6 (banked post-v1), #1097's hardware
first light. The unit's dual carried three disclosed blinding
contamination sources (row has the accounting; the block-draw
format ruling landed mid-review). v1 status: **GUI-0…GUI-4 all
merged, samples #29–#33, ordinals 400–404; v6 tally 0 confirmed +
1 candidate pending blinded coding.** Next: the exit walk with
Evan.

## Exit walk RATIFIED (2026-08-28 05:13Z — Evan merged #1121)

The v1 GUI's required path is complete and signed off:
GUI-0…GUI-4 merged (samples #29–#33, ordinals 400–404), acceptance
evidenced on the real gallery hosted, the egui GO authoritative,
residue banked (#1097, #1111, #1117, #1120, GUI-6). The walk's
second question — **GUI-5 (the threaded web lane): dispatch,
defer, or drop** — received no answer with the merge and stays
OPEN as Evan's call; block GUI-B2 slot 2 stands ready, and the
block record's remaining-slot handling follows the branch-side
ruling either way. Program state: idle on that one decision; no
lanes running; the PR subscription and fallback watches are
retired.

## Program close (2026-08-28, Evan's GUI-5 ruling: DEFER)

GUI-5 (the threaded web lane) is DEFERRED post-v1, banked beside
GUI-6 — the plan's posture ("skipping it costs v1 nothing")
carries it; the wasm guard keeps the compile-level option green in
CI. Block GUI-B2: slot 1 consumed (GUI-4); slots 2–4 banked with
the block's branch-side draw record — a future dispatch of GUI-5
or GUI-6 takes the next slot with its arm read back from that
record. **The v1 GUI program is CLOSED**: plan delivered and
ratified (#1121), five units merged (samples #29–#33, ordinals
400–404), residue owned (#1097 hardware first light, #1111,
#1117, #1120, GUI-5 + GUI-6 banked). The tail of this log is the
program's final state; future GUI work opens its own log or
reopens this one at a new seam.

**Post-close maintenance (2026-08-28, PR #1125 — the first-light
hardening bundle):** Evan's real-hardware run (WSLg) produced six
findings in one evening, all fixed here: explicit window sizing +
WSL-detected X11 backend preference (WSLg's Wayland RAIL resize
confirmed broken, X11 confirmed working); chrome panes scrollable
(overflow was unreachable); a chooser-backend probe failing LOUD
at first sight when no zenity/portal exists (rfd's silent-None
lane closed as far as it can be); Alt+primary orbit for trackpads;
delete buttons naming the feature they delete; a viewer README
with the bindings and the CONFIRMED WSL troubleshooting chains
(including the memorable one: dpkg claiming fonts-dejavu-core
installed while the files were gone from disk — hex-box tofu via
a Type 1 fallback Pango cannot shape). One gate-policy edit rides
along, flagged for Evan's retroactive glance: `no-ambient-env.sh`
allowlists viewer frame.rs as the single door for the GUI shell's
platform probes (environment-as-subject, argued in the gate
header). No A/B ceremony per the post-close maintenance precedent
(#1108); orchestrator-reviewed. Next: the add-parameter affordance
PR (queued behind this merge).

**Post-close maintenance (2026-08-28, PR #1129 — the add-parameter
affordance, Evan-requested):** the panel creates document
parameters (name + dimension + value, one `SetDocParam`, one undo
step) and the expression box's unknown-name refusal now OFFERS
creation with the draft restored — the ratified
refuse-then-offer pattern completed. The create door refuses an
existing name typed (`ParamExists`; the edit door conversely
refuses a missing one — the two chrome doors partition the API's
create-or-replace, which is untouched). Rider fix for a latent
class: `viewer` now forwards the `interval` feature like every
other workspace crate — found when the lane's first run drew the
interval lane with `-p viewer` scope; any viewer-only diff would
have hit it. Orchestrator-reviewed, no A/B ceremony (maintenance
precedent).

**Post-close maintenance (2026-08-29 — four display/editing tweaks,
Evan-requested in chat):** all four asked for as GUI tweaks; each
landed as values with headless rows, no A/B ceremony (maintenance
precedent, #1108/#1125/#1129).

1. **A 3-vector is one panel row.** `SlotId::component` /
   `VectorSlot` name the vector families in the NODE VOCABULARY
   (an exhaustive match, so a vector-valued slot added later
   cannot reach a consumer as three unrelated scalars), and
   `props::group_rows` folds a node's rows on it. A datum plane is
   two rows instead of six. An incomplete family degrades to
   scalars rather than drawing a vector with a hole in it.

2. **`pi` is a row of the unit table**, quantity Angle, factor π —
   a NOTATION carried as a unit, which is exactly what a stored
   per-literal display unit is for ("here is how I want this
   number written"). `0.5 pi` parses and formats back bit-exactly
   through the existing machinery; nothing downstream
   distinguishes it from `deg`. Evan's own framing, taken as
   stated: it is not a unit and the module docs say so.

3. **The GUI's canonical-units ruling is SUPERSEDED** (GUI-PLAN's
   units row, edited in place with the date). Panels render and
   author in the display unit each literal remembers, with a
   picker per row and per vector; `SessionOp::SetSlotUnit` moves
   the notation and provably not the bits. Everything crossing
   into the session is still canonical. Document PARAMETERS are
   the one asymmetry and it is the storage's — `DocParam` has no
   unit field — recorded rather than papered over.

4. **The drawing marks what the side panel is showing.**
   `pick::focus` turns a selection into the set of drawn patch ids
   it is responsible for: every patch a feature drew; for a node
   that draws nothing itself (a profile, a datum) the geometry
   built from it; for a document parameter every feature it
   drives. Carried as a per-corner `FLAG_FOCUS` (the free-move
   probe's mechanism, second bit), tinted under the existing
   selection/hover marks. **Known gap, stated as a gap:** the
   marking is per NODE, so selecting a profile lights the whole
   body rather than the walls of the one segment being edited. The
   type is already a set of patch ids, so per-segment marking is
   expressible; it wants the profile-step ↔
   `RoleSeg::Lateral(ProfileEdgeRef)` correspondence ESTABLISHED
   rather than guessed, which is its own small unit. Filed as
   issue 1182.

5. **"How far can this field move before something breaks."**
   `viewer::bounds` — step outward, then bisect, against a caller-
   supplied validity oracle; the search evaluates nothing itself,
   so the part that can be wrong is tested against arithmetic
   predicates. `SessionOp::ProbeBounds` drives it inline against
   real evaluations (landed evaluation as memo, so a sample
   re-runs the edited node's cone), with valid = "the failing-node
   set did not grow from where the field is now" — which makes the
   current value valid by construction, in a broken document too.
   Bounded at 44 samples over both directions.

   **Three limits are stated in the module docs, in the rendered
   wording, and in the type**, because a probe that read as a
   derivation would be the confident-wrong-answer class: validity
   is not monotone, so what is found is the nearest boundary the
   sampling could SEE; each side reports a BRACKET (furthest
   valid, nearest invalid) rather than a number; and a side with
   no failure in reach says how far it looked, never "unbounded".

   **Sampling is a stand-in for a proof, and issue 1183 says so**
   (Evan's framing, in the conversation that asked for this): the
   kernel already runs `evaluate::<Interval>` over a whole `Doc`,
   and `Interval::from_bounds` is the subdivision driver's
   constructor, so replaying with the field WIDENED and
   branch-and-bounding the box would give the largest CERTIFIED
   locally-valid interval instead of the nearest boundary sampling
   could see. What is missing is kernel tooling, not viewer code —
   `evaluate` derives its own `ParamEnv` degenerately, a node slot
   has no name to widen (the widening belongs to the QUERY, so the
   answer is a driver-side override rather than an interval-valued
   literal in the recipe), and the verdict contract has to rule
   that an INDETERMINATE interval decision means subdivide rather
   than fail, which is adjacent to the enclosure-lane contract open
   as issue 1143. `BoundsProbe` evaluates nothing itself precisely
   so the oracle can be replaced without the panel noticing.

   **The two remaining open questions for Evan**: (a) inline is a
   hitch of tens of evaluations on a button press — the resumable
   state machine makes moving it behind the eval seam or onto a
   per-frame drip mechanical if the hitch is felt; (b) the seed
   step is one of whatever unit the field is written in, which is a
   guess at the user's scale and the one number in the feature
   nothing derives.

**Post-close maintenance (2026-08-29, PRs #1217, #1230, #1247 — the
Open… freeze, and what was actually under it):** reported as "one of
the recent GUI changes broke the Open tool". It had not. The typed
door was fine and always was; what was wrong is that `sync_scene`
builds the pick index on the UI thread, so opening the tour's
`hollowring` at the application's starting δ meant 25 s of a frozen
window still showing the previous document — indistinguishable, from
the user's seat, from a dialog that did nothing. **Not a regression
either**: the two suspected PRs (#1162, #1184) were measured at
`617d039` and at head, on the same documents, and the Open… → new
document time was 24 s before and 25 s after. The whole investigation
ran the real binary on Xvfb + lavapipe with `xdotool` driving the
toolbar; the recipe is now in `crates/viewer/README.md`, and the A/B
of two app builds in one environment is what made the
not-a-regression claim measurable rather than argued.

Three landed. **#1217** — `bvh::Bvh::build` sorted every range at
every level; the split rule reads only which side of the median an
item falls on, so `select_nth_unstable_by` answers it under the same
total order, O(n log²n) → O(n log n), same tree (same shape, leaf
membership and hulls; only the unread order within a leaf differs, and
both queries sort their own output). 20.5 s → 9.2 s on 4·10⁶ boxes.
**#1230** — `diefillet.pncad` shipped with two product roots, the
composed die and the blank sitting exactly on it, so the file drew one
die-shaped thing with its pips filled in and twice the material. #1162
diagnosed that and its separation resident REPORTS it by design;
`gallery_document` now acts on the finding's own recourse and deletes
the narration body. The exporter prints each document's roots and
findings as it writes, and two gated rows pin the shape of every
gallery document as a table. **#1247** — `scene::TRIANGLE_BUDGET` and
`fit_delta`: the budget chooses the δ a document OPENS at (a default,
explicitly not a cap — the first cut clamped every rebuild and made
`Finer δ` a button that did nothing), by probing once at 8× and
solving the measured `triangles ≈ C/δ` law. Open… on the ring: 25 s →
16 s → 8 s.

Residue owned: **#1259** the index build still on the UI thread (most
of the remaining 8 s; moving it extends the seam §5 ratified, so it
wants a ruling); **#1260** `torus_grid_step` sizing both chart
directions off one step, ~65× the triangles the chord asks for, which
is the order-of-magnitude lever every number above sits under and is
TESS-BUDGET's question, not this program's; **#1261** the heatsink's
fins unioned in the demo's `solidify()` and never in the recipe;
**#1253** the status line cleared by every camera fold through `land`,
which eats the product fault raised on an open and is why the budget's
verdict is a badge. No A/B ceremony per the post-close maintenance
precedent (#1108); reviewed by Evan in conversation, who directed each
merge — including the #1247 rework, which was his correction: the
budget was a clamp until he said he had expected it to set a default.


**Post-close maintenance (2026-09-01 — authored literals remember the
form's unit; schema v20):** reported as "author in mm, the panel shows
metres until you use its picker". True, and the cause was one struct
field: the creation forms already had a unit picker and already used
it (`unit_field` divides the draft by the picked factor for display),
but `SessionOp::AddExtrude { distance: f64 }` carried only canonical
metres, so `DocSession::add_extrude` minted `Expr::literal` and the
notation died one field before it could be stored. Not GUI-only — the
demos author with `Expr::literal` too, so the tour's 10 mm ring opened
in the panel as `0.01 m`.

Three rulings, all Evan's in the conversation that asked for it.

1. **The creation ops carry `Expr`.** The first cut carried a
   value-plus-notation carrier; Evan took the bigger version instead —
   "sounds like we'll need it eventually" — which is one vocabulary
   break rather than two, and is what an expression-driven creation
   form will want. `AddExtrude`, `AddRevolve`, `AddFillet`,
   `AddChamfer`, `AddTransform`, `AddPattern`'s rule and `AddDatum`'s
   spec all carry `Expr` now; `AddProfile` carries `Vec<LoopProgram>`,
   the document's OWN loop vocabulary, which was already Expr-bearing.
   `ProfileShape`/`PathStep` stay f64 and stay the FORM's template
   vocabulary — a template is a thing a person edits in a dialog, and
   `sketch::loop_program` is where it becomes a document, now taking
   the notation it is authored in (`sketch::Notation`).

   Two consequences worth stating. The session stops minting literals
   entirely, so a wrong-dimension expression refuses at the EDIT door
   (`SlotDimensionMismatch`), one rule for authored and hand-written
   programs. And a non-finite field can no longer reach an op at all —
   an `Expr` cannot hold one — so the refusal moved from the session
   to the literal door at the form, and the rows that pinned it were
   re-pointed rather than deleted.

2. **A dimensionless row, and the display unit stops being
   optional.** The `Option` was defended on the grounds that `None`
   and `Some(rad)` differ; Evan's answer was that an `Option` with an
   unclear interpretation is worse than a special case, and he was
   right for a reason better than the one offered: **two readers of
   this repository already disagreed about what `None` meant.**
   `expr::write_literal` resolved an unmarked angle to `rad`;
   `props::written_unit` resolved the same stored literal to
   `pi rad`. One value, two renderings, decided by which door reached
   it. `quantity::UNITS` gains [`ONE`] — quantity `Scalar`, symbol the
   EMPTY string, factor 1.0 — so a `Scalar` literal names its notation
   (writing no suffix) instead of declining to name one, and
   `Lit.display_unit` is a plain `UnitSym`. `Expr::literal(v, dim)`
   keeps its signature and stores the canonical row, so no call site
   moved. `props::written_unit` is gone; `props::rendering_unit`
   replaces it with a NARROWER job — the unit to render a value nobody
   wrote, i.e. a slot driven by an expression — and it chooses
   canonical, so the panel and `unparse` now agree by construction.

   The picker offers nothing for `Scalar`, which is the one place the
   dimensionless row is special-cased, and it is a chrome rule rather
   than a storage one — Evan's own framing: not displaying an option
   when only one unit is possible beats an `Option` at every use site.

3. **Document parameters join.** `DocParam::Continuous` carries a
   display unit beside its dimension and its distribution, closing the
   asymmetry the units ruling recorded rather than papered over: every
   literal could remember its notation and no parameter could, so the
   one value a recipe shares across features was the one that forgot
   how it was written. The pairing (unit measures the declared
   dimension) is a document invariant, checked in the shared save/load
   validator (`PersistError::DisplayUnit`) because the payload is
   `pub` and the dimension is data; the typed authoring doors
   (`DocParam::written_length` / `written_angle`) cannot build a
   mismatched one.

**Schema v20** carries all of it, and its ledger entry states the
break honestly: there is no degenerate carry this time — a v19
document and its v20 regeneration differ on nearly every literal.
Evan: "it's fine to have a break that means old documents don't load
anymore". Every committed document was regenerated
(`gallery_ring.v20.pncad`, `plate_param.v20.pncad`, the die and
assembly corpora, `v20_golden.cad`); v19's golden stays on disk, as
every version's does.

**The demos carry the exhibit**, because a demo demonstrates REAL
usage and "author in millimetres" is the usage in question. The four
gallery documents now span the space deliberately: `ring` is written
in millimetres and half-turns throughout (`300 mm`, and a full turn as
`2 pi rad` rather than `6.283185307179586 rad`); `diefillet` is MIXED
— millimetres for its lengths, degrees for the pip table's quarter and
half turns, dimensionless for the rotation axes — and it exercises the
other authoring door, `WrittenLength::canonical_in`, because its
lengths are DERIVED from the die's geometry and only their notation is
being chosen, which is exactly a GUI form's shape; `checks` and
`heatsink` stay canonical, and say in a comment that they are the
control. `ring`'s constants moved to millimetres with the canonical
metres derived through `quantity::MILLI`, so the analytic oracle and
the recipe cannot drift.

Degrees are inexact by nature (the unit table says so), so the die's
pip rotations moved by an ulp — `180 deg` is `180 · fl(π/180)`, not
`PI`. That is the honest consequence of writing an angle in degrees,
it is orders below every tolerance the scene asserts, and the tour
runs clean.

`WrittenLength`/`WrittenAngle` joined the prelude for `Length`'s
reason: they are what an authored quantity IS at the D6 boundary. The
prefix data (`MILLI`, `ONE`, `UNITS`) stayed one hop away at
`pncad::quantity`, per the corpus-measured prelude rule.

**A rider bug, found on the way and fixed by the change rather than
beside it:** the creation forms' `unit_field` read the draft's raw
unit while the `unit_picker` beside it resolved through
`props::written_unit`, so the revolve form's default full turn
rendered `6.28318…` next to a picker reading `pi rad`. It was the same
root cause — a `None` two readers interpreted differently — and it
dissolved when the drafts became typed and non-optional
(`length_unit: LengthUnit`, `angle_unit: AngleUnit`, defaulting to `m`
and `pi rad`, which preserves exactly what the editor renders today).
