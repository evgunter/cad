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
#
# THE TWO THRESHOLDS BELOW ARE DERIVED FROM THE READING ABOVE, so this is
# an extraction and not a decoration: 15G warns because one lane rebuilding
# can take 5-8G at a stroke, and 8G is critical because at that point a
# single lane can fill the disk mid-write, which is what the 2026-07-24
# crashes were. Nothing re-takes the per-lane figure and nothing can from
# in here — it is a property of the developer's tree, it moves with the
# workspace's size and with the debug setting `setup-build-env.sh`
# applies, and this script sees only free space. Two consequences, stated
# rather than left to be discovered: the figure has ALREADY drifted from a
# second copy of it elsewhere in the repo, and if per-lane target/ ever
# grows past ~8G the CRITICAL threshold stops being a margin at all. The
# thresholds are environment-overridable (`CAD_DISK_*_GB`) precisely
# because the reading behind them is not durable; re-take with `du -sh` on
# a live lane before trusting either number, and prefer raising the floor
# to arguing from this comment.
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
