---
name: usage-limit-protocol
description: Session-usage limit protocol (Evan, 2026-08-10) — usage-watch.sh monitors per-account statusline usage; orchestrators act on WARN/STOP-SOON/STOP-NOW/RESET/CACHE-PING events; mngr usage conflates accounts and mngr CLI hangs from agent worktrees
metadata:
  type: project
---

**Account identifiers stay OFF GitHub (Evan, #355, 2026-08-10,
anticipating the repo going public; restored after the #359/#360
dedupe crossing deleted both copies):** never post account email
addresses — or any personal identifier beyond the commit-signing
identity (`evgunter`) — in issues, PRs, comments, commits, or
committed files (this file included: name accounts by role, keep
concrete addresses in local cad-work logs only). Existing comments
naming addresses were edited on request — do not reintroduce them.

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

**The monitor**: `local-scripts/monitors/usage-watch.sh`. Session-start
convention (Evan's arming-burden point, #349): `cp
local-scripts/monitors/*.sh ~/.local/share/cad-work/monitors/` then arm
EVERY script in the installed dir as a persistent Monitor — glob
the directory, do not maintain a named list, so new monitors get
armed without instruction edits. Monitors stay separate SCRIPTS
(independent failure domains — a stalled gh call must not stall
the disk watchdog; per-stream disarm); if arming burden grows, the
sanctioned merge is local-only pollers (disk/hourly/usage) into
one, keeping the network-bound away-channel separate. It joins
each agent's `events/claude/usage/events.jsonl` to its account via
the agent's `.claude.json` oauthAccount, and emits per-account
CROSSING events.

**Per-account discipline (Evan, #349): act ONLY on alerts naming
YOUR OWN account.** Resolve it at session start from your own
agent dir: `agent-<id>/plugin/claude/anthropic/.claude.json →
oauthAccount.emailAddress` (the id is in your memory-directory
path). Other accounts' alerts are informational — do not pause
your lanes for them, and do not act on them: cross-account
revival was tried and dropped (2026-08-18). Judging another
session dead from outside is unreliable, and the recovery action
was keystroke injection into someone else's session.

**RESET events can name the 7-DAY window while the 5-hour one is
still full** (2026-08-28, learned by resuming and being STOP-NOW'd
seconds later). The event text says only "window rolled over", and the
percentage quoted is the 7d one. **Do not infer the 5h state from a
RESET.** Read the ground truth instead — the last line of the agent's
own `events/claude/usage/events.jsonl` carries
`rate_limits.five_hour.used_percentage` and `.seven_day...` in one
`cost_snapshot`:

```
tail -1 <agent-dir>/events/claude/usage/events.jsonl | python3 -c \
  "import sys,json; print(json.load(sys.stdin)['rate_limits'])"
```

One read settles it and costs nothing. Resuming on a misread costs a
cold cache each time, and resuming wrongly at 99% risks the dialog that
kills the session for the day.

**Orchestrator protocol on its events** (scoped to your account):
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
