---
name: hourly-agent-checkins
description: Standing instruction (Evan, 2026-07-24) — check in on every running subagent at least once per hour; arm an hourly heartbeat Monitor at session start
metadata:
  type: feedback
---

Check in on every running subagent lane at least once per hour.

**Why:** Evan asked for it (2026-07-24, #88) after the appearance and
issue-86 agents idled with lost wake-on-completion events and the
stall was only discovered hours later when he noticed a quiet
branch: "that way their cache doesn't get broken if they stop and it
only gets discovered hours later." Subagents that park waiting on
long test runs sometimes never receive the completion wake-up; the
work is done and pushed but unreported, blocking the pipeline.

**How to apply:** At session start (with the other monitors), arm a
persistent hourly heartbeat Monitor:
`while true; do sleep 3600; echo "HOURLY AGENT CHECK-IN TICK ..."; done`
On each tick: list the lanes believed running, check each for real
activity (branch pushes, cargo processes in its clone, task output
mtime), and SendMessage-nudge any agent that appears idle without a
final report. See [[subagent-death-recovery]] for the recovery
ladder when a nudge reveals a dead agent.
