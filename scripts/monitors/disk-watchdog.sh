#!/usr/bin/env bash
# Disk-space watchdog (arm via Monitor; persistent). Two disk-full
# WSL crashes on 2026-07-24 motivated this: each agent lane grows a
# 5-8G target/, the gate cache alone runs ~30G.
#
# On WARN/CRITICAL: delete finished lanes' clones and idle lanes'
# target/ dirs; NEVER touch the gate runner's target while a gate is
# running; leave ~/.cache/gmp-mpfr-sys alone. After any disk-full
# crash, ELF-magic-scan recent executables in surviving targets for
# torn writes, and treat crash-window test results as ENOSPC-suspect.
set -u
WARN_GB="${CAD_DISK_WARN_GB:-15}"
CRIT_GB="${CAD_DISK_CRIT_GB:-8}"
while true; do
  avail_kb=$(df --output=avail / | tail -1 | tr -d ' ')
  avail_gb=$((avail_kb / 1048576))
  if [ "$avail_gb" -lt "$CRIT_GB" ]; then
    echo "DISK CRITICAL: ${avail_gb}G free — clean build caches NOW. Biggest: $(du -sh "$HOME"/.local/share/cad-work/*/target "$HOME"/.local/share/cad-gate/repo/target 2>/dev/null | sort -rh | head -3 | tr '\n' '; ')"
    sleep 600
  elif [ "$avail_gb" -lt "$WARN_GB" ]; then
    echo "DISK WARNING: ${avail_gb}G free — clean finished lanes' target/ dirs"
    sleep 1800
  else
    sleep 300
  fi
done
