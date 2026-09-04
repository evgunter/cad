#!/usr/bin/env bash
# Hourly agent check-in heartbeat (arm via Monitor; persistent).
# Ev's standing request (2026-07-24): sweep every running subagent
# lane at least hourly — check branches for pushes, clones for live
# cargo processes, task outputs for staleness — and nudge (resume via
# SendMessage) any agent that idled without delivering its report.
# Lost wake-on-completion events are common; the work is usually done
# and pushed but unreported, silently blocking the pipeline.
#
# Second purpose (Ev, 2026-08-22): the tick itself re-invokes the
# orchestrator, keeping its prompt cache warm through otherwise-idle
# stretches — so an hourly cadence (within the cache TTL) is worth
# keeping armed even when no lanes currently need sweeping.
#
# The tick is anchored to WALL CLOCK, not to sleep durations: a bare
# `sleep 3600` fires early whenever the sleep is interrupted (WSL2
# clock skew under load, host suspend/resume), which showers the
# orchestrator with spurious ticks. Short polls + an elapsed-time
# check make interrupted sleeps harmless.
set -u
last=$(date +%s)
while true; do
  sleep 300
  now=$(date +%s)
  if [ $((now - last)) -ge 3300 ]; then
    echo "HOURLY AGENT CHECK-IN TICK $(date -u +%Y-%m-%dT%H:%MZ) — sweep every running subagent lane; nudge any that idled without reporting (tick also keeps the orchestrator's prompt cache warm — no lanes running is still a valid tick)"
    last=$now
  fi
done
