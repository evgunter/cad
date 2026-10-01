---
id: a-sweep-diff-gates-no-eps-row-of-sweep
kind: issue
title: a diff touching only sweep's tests runs none of sweep's 1e-6/1e-12 rows at the PR gate, though today's eps-only red sat there
status: open
opened: 2026-10-01
priority: P2
cost: E
---

Filed by the reach-eps lane (REACH, PR 3636).

**What.** `.github/workflows/ci.yml`'s change filter (the "EXTRA EPS
ROWS" block) adds a crate's 1e-6 and 1e-12 rows only when the diff seeds
one of `step-import geom-brep profile topo`, or touches a path matching
`crates/<c>/.*(probe|golden)`. `sweep` is in neither. PR 3636 changes
`crates/sweep/tests/reach_volume_backstop.rs`, and its run 36823485092
logs `eps_extra=package(topo)`, which comes from its doc edit in topo.
So none of the rows it fixes run at 1e-6 or 1e-12 at the gate.

Those rows' eps-only red reached main through PR 3611 the same way
(`work/reach/reach-volume-backstop-fails-off-the-default-eps`). It
surfaced only on a diff that ran `all()` (EDIT's PR 3625), and then it
blocked unrelated PRs (#2468). The filter's own comment names the rule
for joining the list ("the eps-only reds of 2026-07..09 sat in these
crates"), and sweep now qualifies.

A `workflow_dispatch` with `scope=all` would cover it by hand. From an
agent session's integration token that dispatch is refused with
`403 Resource not accessible by integration`.

**The question for CIW** is the cost of adding `sweep` to the list
against the latency cut, or a narrower route such as the eps rows of
the changed test files only.
