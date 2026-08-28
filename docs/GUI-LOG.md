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
