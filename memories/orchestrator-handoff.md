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

The `mngr` CLI was broken for a stretch of M2 (azure plugin
ImportError; workaround was reading the usage events.jsonl directly —
see [[orchestration-model]]'s checklist); verified working again
2026-07-20. Check `mngr --help` before relying on it.
