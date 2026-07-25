#!/usr/bin/env bash
# Hourly agent check-in heartbeat (arm via Monitor; persistent).
# Evan's standing request (2026-07-24): sweep every running subagent
# lane at least hourly — check branches for pushes, clones for live
# cargo processes, task outputs for staleness — and nudge (resume via
# SendMessage) any agent that idled without delivering its report.
# Lost wake-on-completion events are common; the work is usually done
# and pushed but unreported, silently blocking the pipeline.
set -u
while true; do
  sleep 3600
  echo "HOURLY AGENT CHECK-IN TICK $(date -u +%Y-%m-%dT%H:%MZ) — sweep every running subagent lane; nudge any that idled without reporting"
done
