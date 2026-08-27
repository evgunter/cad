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
