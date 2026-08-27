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

Next actions: liveness check-ins on both lanes; at each PR-open,
freeze head, claim ordinal (recorded here, pushed), dispatch the
v6 dual.
