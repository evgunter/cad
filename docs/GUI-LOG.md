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

Next actions: dispatch GUI-0 and GUI-1 per the block GUI-B1 draw;
reviews per protocol v6 (cross-model duals, banded ordinals from
400) at each PR.

## GUI-0 merged (2026-08-27, PR #1094, sample #28 / ordinal 400)

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
