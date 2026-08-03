---
name: clone-placement
description: Never put working git clones in the /tmp session scratchpad — use ~/.local/share/cad-work/<purpose>/ or .claude/worktrees/; scratchpad is for disposable artifacts only
metadata:
  type: feedback
---

Working git clones/worktrees (anything that will hold commits or
in-flight edits) must NOT live in the session scratchpad under
/tmp. Put them in a persistent location: `~/.local/share/cad-work/
<purpose>/` (same reasoning as the gate runner's
`~/.local/share/cad-gate/repo`) or the main checkout's
`.claude/worktrees/` (where isolation-worktree subagents already
go). The /tmp scratchpad is only for genuinely disposable
artifacts: PR body drafts, STL exports, probe scripts, notes.

**Why:** Evan flagged it (2026-07-23, after the PR 3 casualty
scare): "why use /tmp? seems risky for no benefit." Correct — the
scratchpad path is session-derived and treated as disposable by
the tooling (a previous orchestrator's scratchpad was found reaped
to empty), so uncommitted work there can vanish silently. The only
upside is prompt-free writes, which doesn't justify the exposure.
/tmp being on the rootfs (not tmpfs) on this WSL2 box softens but
does not remove the risk.

**How to apply:** Before creating any clone/worktree, pick the
persistent path. Push-after-every-commit still applies
([[subagent-death-recovery]]); persistent placement protects the
window between edits and the next push. Disk hygiene still
applies — remove merged-branch clones at pipeline seams
([[worktree-disk-hygiene]]).

**Lane creation is now scripted (2026-08-03, Evan-directed
enforcement):** `scripts/new-lane.sh <lane> [branch]` is the
standard clone path — it sets `core.hooksPath scripts/hooks`,
activating the committed pre-push hook (`fmt-all.sh --check`,
every workspace, ~9s). Briefs say "create your clone with
scripts/new-lane.sh" instead of a raw git clone; a hand-rolled
clone silently lacks the hooks (git never auto-activates
committed hooks), which is exactly the gap this closes.
