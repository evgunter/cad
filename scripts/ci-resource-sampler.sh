#!/usr/bin/env bash
# ci-resource-sampler.sh — one line of a hosted runner's memory, disk and
# heaviest processes every INTERVAL seconds, to stdout, until killed.
#
#   scripts/ci-resource-sampler.sh [INTERVAL] &
#
# It writes to the step's own log rather than to a file, because a runner
# that is shut down mid-step runs no later step, not even an `if: always()`
# upload. That log survives a shutdown that ends the step with exit 143 (the
# 11-16 and 11-19 kills), up to its last line. It does not survive a runner
# that loses communication with the server (the 11-18 and 11-22 kills): those
# jobs have no log at all, and nothing written on the runner reaches GitHub.
#
# Each line: UTC time, MemAvailable, swap in use, free disk on / and on the
# workspace's filesystem, the 1-minute load, then every process over 300 MB
# RSS as `name:MB`. A rustc is named by its --crate-name, marked `[test]`
# when it builds a test harness, and prefixed with its package (cargo's
# CARGO_PKG_NAME in its environment) when that differs from the crate name,
# so each `tests/all.rs` reads as `editor-core/all[test]`.
set -uo pipefail
interval=${1:-30}
echo "[sampler] $(nproc) cpus; $(awk '/MemTotal/{printf "%d MB", $2/1024}' /proc/meminfo) memory; $(free -m | awk '/Swap/{print $2}') MB swap"
df -m / "${GITHUB_WORKSPACE:-.}" | sed 's/^/[sampler] /'
while :; do
  mem=$(awk '/MemAvailable/{a=$2} /SwapTotal/{t=$2} /SwapFree/{f=$2}
             END{printf "avail=%dM swap=%dM", a/1024, (t-f)/1024}' /proc/meminfo)
  root=$(df -m --output=avail / | tail -1 | tr -d ' ')
  work=$(df -m --output=avail "${GITHUB_WORKSPACE:-.}" | tail -1 | tr -d ' ')
  heavy=
  while read -r pid rss args; do
    [ "$rss" -gt $((300 * 1024)) ] || break
    read -ra argv <<< "$args"
    name=${argv[0]##*/}; test=; pkg=
    for i in "${!argv[@]}"; do
      case ${argv[i]} in
        --crate-name) name=${argv[i + 1]:-?} ;;
        --test) test='[test]' ;;
      esac
    done
    pkg=$(tr '\0' '\n' < "/proc/$pid/environ" 2>/dev/null | sed -n 's/^CARGO_PKG_NAME=//p')
    [ -z "$pkg" ] || [ "${pkg//-/_}" = "$name" ] || name="$pkg/$name"
    heavy+="$name$test:$((rss / 1024)) "
  done < <(ps -eo pid=,rss=,args= --sort=-rss)
  echo "[sample] $(date -u +%H:%M:%S) $mem disk/=${root}M disk-work=${work}M load=$(cut -d' ' -f1 /proc/loadavg) $heavy"
  sleep "$interval"
done
