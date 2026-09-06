---
name: orchestrator-switch-runbook
description: RUNBOOK — read only when handing a session to a successor orchestrator. The handoff contract plus this host's tmux mechanics.
metadata:
  type: project
---

**Read this only when performing a switch.** Nothing here is needed
during normal operation.

The outgoing orchestrator performs the switch itself (Ev, PR #285).

**The contract — this is the part that must hold whatever the harness
looks like:**

1. Finalize the handoff file (`cad-work/handoff-prompt-*.md`) with the
   LIVE resting state, including per-lane resume instructions for any
   in-flight agents: agents die with the session, lanes survive if
   pushed.
2. Push every lane's local commits. If a lane's own agent may still be
   alive and holding git state, note that in the handoff rather than
   fighting the lock.
3. Commit crucial state (logs, memories, branch status).
4. Hand off in a way that PRESERVES THE LOGIN, and verify the successor
   is actually working before you exit.

**This host's mechanics (tmux), which is only one way to satisfy step 4:**
`tmux split-window` a new pane in the SAME tmux session and the same
`CLAUDE_CONFIG_DIR` (that is what keeps the login), launch `claude`
(model fable), `send-keys` the handoff kickoff, confirm via
`capture-pane`, then kill **only your own pane** — never the session
(the successor lives in it), and never another orchestrator's session.
