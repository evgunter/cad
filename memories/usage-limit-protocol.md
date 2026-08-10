---
name: usage-limit-protocol
description: Session-usage limit protocol (Evan, 2026-08-10) — usage-watch.sh monitors per-account statusline usage; orchestrators act on WARN/STOP-SOON/STOP-NOW/RESET/CACHE-PING events; mngr usage conflates accounts and mngr CLI hangs from agent worktrees
metadata:
  type: project
---

**Why this exists (Evan, #348 comment, 2026-08-10):** hitting the
subscription session limit opens Claude Code's usage-credits dialog,
which KILLS that session for the rest of the day — it does not
recover when the window resets (unlike the fable-specific API-error
limit, which is retry-through-able). So agents must be stopped
BEFORE the window fills.

**`mngr usage` is not usable for this**: statusline events carry no
account identity and `mngr_usage/api.py::_combine_agent_walks`
reduces freshest-wins across ALL agents — with Evan's four accounts
it shows whichever account rendered last, unlabeled, hiding the
others. (Bonus finding, unfixed: the `mngr` CLI hangs indefinitely
when run from an agent worktree — likely host-lock contention;
verdict was derived from source + raw logs.)

**The monitor**: `scripts/monitors/usage-watch.sh` (install to
`~/.local/share/cad-work/monitors/`, arm persistent at session
start WITH the other three — monitors deliberately stay SEPARATE
scripts; consolidation was weighed and rejected: independent
failure domains, differing cadences, per-stream disarm). It joins
each agent's `events/claude/usage/events.jsonl` to its account via
the agent's `.claude.json` oauthAccount, and emits per-account
CROSSING events.

**Orchestrator protocol on its events**:
- `USAGE WARN` (≥90%): wind down that account's lanes — finish the
  current unit, start nothing new that won't land quickly.
- `USAGE STOP-SOON` (≥95%): land in-flight work, start nothing.
- `USAGE STOP-NOW` (≥99%): pause that account's agents immediately
  (better a cold cache than the dialog).
- `USAGE CACHE-PING` (every 55m while in [90,99)): ping paused
  agents with a do-nothing message to keep prompt caches warm.
- `USAGE RESET`: the window rolled over — resume/restart paused
  agents (from transcript if dead) and tell them to continue.

The hourly check-in stays hourly — the 55m cadence is baked into
usage-watch itself, no re-cadencing needed. See
[[agent-lane-operations]], [[orchestration-model]].
