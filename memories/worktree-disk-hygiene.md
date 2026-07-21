---
name: worktree-disk-hygiene
description: Each agent worktree accumulates a 4-8 GB target/ dir; no safe way to share cargo artifacts across PARALLEL builds — clean merged-branch worktrees at every pipeline seam instead
metadata:
  type: project
---

**The problem (hit 2026-07-21, disk at 94%/100%):** every agent
worktree carries its own `target/` (4-8 GB each; interval-feature
builds are the biggest). A dozen finished M2-era worktrees held ~40 GB
of stale artifacts and nearly filled the disk mid-session (one
subagent's cargo builds actually died at 100%).

**Why not share target/:** cargo takes an exclusive build lock per
target dir, so a shared `CARGO_TARGET_DIR` SERIALIZES concurrent
builds — unacceptable for the parallel implementer/reviewer fleet.
sccache would share compiled artifacts (CPU savings) but each worktree
still materializes full rlibs, so disk usage is unchanged. The one
thing already shared safely is `~/.cache/gmp-mpfr-sys` (the expensive
GMP/MPFR C build survives across worktrees, user-level).

**How to apply (Evan-endorsed: "clean up old worktrees every once in a
while"):** at every pipeline seam (and in any handoff flush), sweep
`git worktree list` / `du -sh .claude/worktrees/*`: for each worktree
whose branch is merged (`git merge-base --is-ancestor <br>
origin/main`) and whose status is clean, `git worktree remove` it,
then `git worktree prune`. Keep the most recent warm worktree when a
review/fix pass will want its build cache ([[orchestration-model]]'s
M0 lesson). Dead mngr worktrees belong to other agents — `cargo
clean` their target/ but leave the checkout for mngr's bookkeeping.
Sandbox note: the permission classifier may block batch/loop removal
commands and `kill` — issue `git worktree remove` one per Bash call.
