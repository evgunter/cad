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

**How to apply:** Arm via `bash ~/.local/share/cad-work/monitors/disk-watchdog.sh` (install once per machine: `cp scripts/monitors/*.sh ~/.local/share/cad-work/monitors/` from any up-to-date checkout — the repo's `scripts/monitors/` is canonical). On WARN/CRITICAL: delete finished lanes' whole
clones (reviews that reported) and idle lanes' `target/` dirs (work
pushed = clone is cheap to rebuild); NEVER touch
`~/.local/share/cad-gate/repo/target` while a gate is running, and
leave `~/.cache/gmp-mpfr-sys` alone. After any disk-full crash,
purge torn binaries in every surviving target that was building
(ELF-magic scan on recent executables — see M4-LOG 2026-07-24) and
treat test results from the pressure window as suspect (ENOSPC can
kill processes silently). See [[worktree-disk-hygiene]],
[[hourly-agent-checkins]].

**RAM contention corollary (2026-07-24)**: this WSL instance has
only ~5G RAM. Running the gate concurrently with agent test
batteries can OOM-kill a gate test process mid-suite — observed as
a bare "Terminated" in one ε row (4s FAIL) while every other row
passed; the identical suites passed on rerun with a quiet machine.
Before diagnosing a fast single-row gate FAIL as a code bug: check
for "Terminated", check what else was running, and rerun quiet.
Prefer sequencing agent batteries away from gate runs when
possible.
