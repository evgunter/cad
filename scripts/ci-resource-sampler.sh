#!/usr/bin/env bash
# ci-resource-sampler.sh — one line of a hosted runner's memory, disk and
# heaviest processes every INTERVAL seconds, to stdout, until killed.
#
#   scripts/ci-resource-sampler.sh [INTERVAL] &
#
# It writes to the step's own log rather than to a file, because a runner
# that is shut down mid-step runs no later step: the log is the one record
# that survives, up to the last line before the kill.
#
# Each line: UTC time, MemAvailable, swap in use, free disk on / and on the
# workspace's filesystem, the 1-minute load, then every process over 300 MB
# RSS as `name:MB`, where a rustc is named by its --crate-name and marked
# `[test]` when it builds a test harness.
set -uo pipefail
interval=${1:-30}
echo "[sampler] $(nproc) cpus; $(awk '/MemTotal/{printf "%d MB", $2/1024}' /proc/meminfo) memory; $(free -m | awk '/Swap/{print $2}') MB swap"
df -m / "${GITHUB_WORKSPACE:-.}" | sed 's/^/[sampler] /'
while :; do
  mem=$(awk '/MemAvailable/{a=$2} /SwapTotal/{t=$2} /SwapFree/{f=$2}
             END{printf "avail=%dM swap=%dM", a/1024, (t-f)/1024}' /proc/meminfo)
  root=$(df -m --output=avail / | tail -1 | tr -d ' ')
  work=$(df -m --output=avail "${GITHUB_WORKSPACE:-.}" | tail -1 | tr -d ' ')
  heavy=$(ps -eo rss=,args= --sort=-rss | awk '$1 > 300*1024 {
      name = $2; sub(".*/", "", name); test = ""
      for (i = 2; i <= NF; i++) {
        if ($i == "--crate-name") name = $(i + 1)
        if ($i == "--test") test = "[test]"
      }
      printf "%s%s:%d ", name, test, $1 / 1024 }')
  echo "[sample] $(date -u +%H:%M:%S) $mem disk/=${root}M disk-work=${work}M load=$(cut -d' ' -f1 /proc/loadavg) $heavy"
  sleep "$interval"
done
