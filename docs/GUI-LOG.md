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
