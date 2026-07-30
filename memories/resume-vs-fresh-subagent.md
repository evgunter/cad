---
name: resume-vs-fresh-subagent
description: When a stopped subagent's remaining work is small and fully specifiable, dispatch a fresh agent instead of resuming the transcript
metadata:
  type: feedback
---

When an agent has been stopped for over an hour and its task is
almost done — the orchestrator knows exactly what remains (rerun
battery rows, commit+push, emit a report derivable from commits) —
dispatch a **fresh subagent with a precise prompt** instead of
resuming the stopped agent's transcript. Resume **only when the
stopped agent's accumulated context is genuinely useful**: mid-design
state, unreported findings, judgment calls made but not yet written
down.

**Why:** a transcript resume replays the agent's entire context
(often 300–400k tokens) to do work a 1k-token fresh prompt covers —
pure token waste when the context adds nothing. (Evan, 2026-07-29,
after spend-limit outage #5.)

**How to apply:** at each stalled-lane recovery, ask "would a new
agent with only my prompt + the pushed commits do this correctly?"
If yes → fresh agent (include clone path, cwd guard, exact remaining
steps, report format). If no → SendMessage resume as before. Related:
[[cad-working-style]].
