---
name: disk-watchdog
description: Standing (Evan, 2026-07-24, after two disk-full WSL crashes) — arm a disk-space watchdog Monitor at session start; parallel agent lanes grow ~5-8G of target/ each
metadata:
  type: feedback
---

Arm a disk-space watchdog Monitor at session start (with the
away-channel and hourly-check-in monitors): poll `df /` every 5 min;
WARN below 15G free, CRITICAL below 8G (echo the top target/ dirs by
size in the alert).

**Why:** Two WSL crashes in one day (2026-07-24) were caused by disk
exhaustion — each parallel agent lane grows a 5-8G `target/`, the
gate runner's warm cache alone reached 30G, and five concurrent
lanes filled a 251G disk to 100%, crashing WSL and killing every
monitor, gate run, and agent session. Evan asked for the watchdog
explicitly.

**How to apply:** On WARN/CRITICAL: delete finished lanes' whole
clones (reviews that reported) and idle lanes' `target/` dirs (work
pushed = clone is cheap to rebuild); NEVER touch
`~/.local/share/cad-gate/repo/target` while a gate is running, and
leave `~/.cache/gmp-mpfr-sys` alone. After any disk-full crash,
purge torn binaries in every surviving target that was building
(ELF-magic scan on recent executables — see M4-LOG 2026-07-24) and
treat test results from the pressure window as suspect (ENOSPC can
kill processes silently). See [[worktree-disk-hygiene]],
[[hourly-agent-checkins]].
