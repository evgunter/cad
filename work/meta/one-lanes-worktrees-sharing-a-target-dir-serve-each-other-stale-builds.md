---
id: one-lanes-worktrees-sharing-a-target-dir-serve-each-other-stale-builds
kind: issue
title: One lane's worktrees sharing a CARGO_TARGET_DIR serve each other's builds, because cargo hashes a workspace member the same in every checkout
status: closed
closed: 2026-10-12
pr: 4758
opened: 2026-10-11
---

## Measured

NURBS build A (PR 4551, 2026-10-11) ran one `CARGO_TARGET_DIR` across its own branch worktree and a main worktree.
- Cargo gives workspace members the same metadata hash in every checkout of this repo. `geom-core` builds as `metadata=22386fc276c9eb8a` in both `/home/user/cad` and the main worktree.
- So a build in one tree reuses the other tree's artifacts whenever mtimes look fresh.
- It surfaced when `-p sweep` failed to compile on the branch, with geom-brep linked against main's geom-core.

What it had already produced:
- a false "this branch reads PR 4684's digest bit for bit" (an isolated build moves `arc_loft_1e9eps`);
- a cost table with cells that may be another tree's code;
- full-suite passes that followed foreign builds.

NURBS build B (PR 4684, same day) hit it independently: one probe was void because cargo reused a binary across worktrees, and it was caught only from "0 crates compiled".

## The gap

`docs/prompts/implementer-discipline.md` §2 says to use your own `CARGO_TARGET_DIR` and **never one shared with another lane**. That bullet's own examples are cross-lane, so a lane that keeps one target dir across its *own* worktrees (branch, main, bisect points) reads itself as compliant. The hazard is identical. Comparing a branch against main in two worktrees is routine, and it is exactly where a silent reuse produces a false "no change".

## Wanted

The rule is one target dir per worktree, not per lane, or at least an equivalent guard. Touching sources per worktree is what build B fell back to, and it is fragile. A change to `docs/prompts/` waits for Ev, so the wording goes to an `[ev]` PR.

— (NURBS orchestrator)

## Closed (PR 4758, Ev's sign-off)

`docs/prompts/implementer-discipline.md` §2 now reads "Use one `CARGO_TARGET_DIR` per worktree, never one shared with another lane or with another of your own worktrees", with the mechanism stated: cargo hashes a workspace member the same in every checkout. A script that mints a target dir per worktree remains possible later. — (NURBS orchestrator)
