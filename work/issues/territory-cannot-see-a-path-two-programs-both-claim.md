---
id: territory-cannot-see-a-path-two-programs-both-claim
kind: issue
title: scripts/work.py territory is silent on a path two open programs both claim — it flags only paths the branch's own program does not claim
status: open
opened: 2026-09-04
---


Found by FIX's `transform-rigid-refuses-described-nurbs` lane
(PR 1742) while running the check that was supposed to catch exactly
the collision it missed. Filed here rather than on FIX's or SHELL's
slate: the tool serves every program.

## The measurement

`scripts/work.py territory --base main` reads a branch's prefix and
its diff and names every path **another** program owns. The
implementation asks whether a changed path falls outside the branch's
own program's `paths` globs — so a path the branch's program **does**
claim is never reported, whatever else claims it too.

At that lane's merge base, `crates/topo/src/transform.rs` appeared in
the `paths` list of BOTH `work/fix/program.md` and
`work/shell/program.md`. SHELL opened 2026-09-03, after FIX's charter
had read the file as unowned. The lane ran `territory`, which was
silent on that path — the one fence that mattered — and reported the
others correctly. The collision surfaced only because the orchestrator
noticed SHELL's `paths` while merging main for an unrelated reason.

## Why this is the failure that matters

`work/README.md` says territory "warns; it does not block", and that
is the right posture. But the warning it does not give is precisely
the contested case: a path exactly one program claims is a clean
handoff a lane can announce, while a path two programs claim is a
live conflict neither orchestrator may know about. The check is
strongest on the easy case and blind on the hard one.

Nothing detects the double claim today. Lint enforces that every glob
matches at least one tracked path; it does not ask whether two
programs' globs match the same path.

## The two candidate fixes, not decided here

1. **A lint rule**: two open programs' `paths` globs matching one
   tracked path is an error, or a warning printed by `status`. This
   catches the state at rest, in CI, for every program at once, and it
   fires on the day the second program opens rather than on the day a
   lane happens to run `territory`. It needs a decision about
   deliberate overlaps — whether any exist, and whether `keep_out`
   prose is the sanctioned way to record one.
2. **A `territory` change**: report a changed path claimed by another
   program EVEN IF the branch's own program also claims it, with
   wording that distinguishes "X owns this" from "X also claims this".
   Narrower, and it only warns the lane that happens to touch the path.

They are not exclusive and (1) is the one that would have caught this
instance early. FIX has since dropped `transform.rs` from its glob and
recorded the crossing in `keep_out`, so the instance is closed; the
blind spot is not.

## Home

`work/issues/` — `scripts/work.py` is tracker tooling in no open
program's territory, and the finding is about the tool rather than
about either program that collided in it.
