---
name: orchestrator-handoff
description: How to hand off to a successor orchestrator — timing (drain the pipeline first), state flush, and the mngr create/capture/start mechanics Evan specified
metadata:
  type: feedback
---

Evan's procedure for switching to a fresh orchestrator (given
2026-07-20, during M2):

**When**: at a pipeline seam with NOTHING in flight — background
subagents are resumable only from the session that spawned them, so
drain reviews/fix passes first. Prefer a seam before a self-contained
work unit (e.g. M2's PR 7: STL + K report + exit sweep), which also
benefits from a fresh context reading the committed logs cold.

**Before creating** — the state flush (see [[orchestration-model]]'s
stopping rule): commit + merge the milestone log (per-PR sections
current, updated state snapshot naming the successor's first moves),
memories updates, and any in-flight branch status. The snapshot is the
successor's resumption contract; assume it gets only memories + logs +
its initial prompt.

**Mechanics** (Evan, verbatim intent):
1. `cd ~/projects/cad` with **latest main checked out**.
2. `mngr create <successor-name>`.
3. `mngr capture <successor-name>` — confirm from the screenshot that
   the model is **Fable** before sending anything.
4. Start it with `--message-file /path/to/initial-prompt` (write the
   handoff prompt to a file first; same shape as the M2 handoff:
   read-order = memories/MEMORY.md → plan → log snapshot, current
   state, first moves, standing process, monitors to arm).
5. Wait ~30 s, `mngr capture <successor-name>` again — confirm it has
   actually started before considering the handoff done.
6. **Stop the predecessor's tmux session (Evan, 2026-07-21)**: after
   the capture confirms the successor is running, the LAST step is
   shutting down the old orchestrator's tmux session. Most sensible
   form: the old orchestrator's handoff prompt instructs the
   successor to stop the predecessor once the successor confirms it
   has everything it needs to get to work; if something is missing,
   the successor instead uses tmux `send-keys` to the predecessor's
   session to ask for guidance before stopping it.

The `mngr` CLI was broken for a stretch of M2 (azure plugin
ImportError; workaround was reading the usage events.jsonl directly —
see [[orchestration-model]]'s checklist); verified working again
2026-07-20. Check `mngr --help` before relying on it.

**Login caveat (learned 2026-07-20, first execution):** a freshly
`mngr create`d agent starts NOT logged in (per-agent interactive
OAuth — an orchestrator cannot clear it). Fallback that worked, per
Evan: start `claude` in a SEPARATE detached tmux session (not a
window of your own session — `mngr stop <you>` would kill it),
same cwd, with your own `CLAUDE_CONFIG_DIR` env exported — the
login rides the config dir. Consequence: the successor's auth
lives in the PREDECESSOR's agent state dir — do not clean that
dir while the successor runs. Confirm the model from the banner
capture ("Fable 5 · Claude Max") before sending the prompt; a
send-keys message may need a second Enter to submit.

**Streamlined path when Evan pre-creates (2026-07-27, M4-close
handoff)**: Evan created cad-implement-5 himself (logged in,
Fable-verified, worktree fresh from main) — the orchestrator's
job reduces to: (1) reach the drained seam; (2) `mngr message
cad-implement-5 --message-file <handoff-prompt>`; (3) `mngr
capture cad-implement-5` to confirm receipt + started working;
(4) successor stops the predecessor once it confirms it has
everything. The prompt should tell the successor to MERGE MAIN
first (its worktree is a creation-time snapshot). The old
create/start/login-fallback dance is unnecessary on this path.
