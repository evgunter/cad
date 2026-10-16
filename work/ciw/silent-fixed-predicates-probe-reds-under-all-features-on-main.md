---
id: silent-fixed-predicates-probe-reds-under-all-features-on-main
kind: issue
title: rim_dim_review_probes' silent-fixed-predicates probe is red on main under --all-features: an undeclared flush subtract no longer refuses
status: closed
closed: 2026-10-15
opened: 2026-10-10
priority: P1
cost: E
---


## What

`topo::rim_dim_review_probes::silent_fixed_predicates_scale_linearly`
fails on `origin/main` when the suite is built with `--all-features`
(the file is `#![cfg(feature = "probe")]`; without it the row does not
compile in, so the PR gate never runs it):

```text
panicked at crates/topo/tests/rim_dim_review_probes.rs:168:9:
undeclared flush must refuse
```

The probe harvests `bool_plane_orient` margins from a flush-face
subtract it expects to refuse typed at `bool_plane_offset`
(`rim_dim_review_probes.rs:162-170`), and from an oblique split. The
subtract now returns `Ok`, so the probe stops at its premise and never
reaches the linearity check it exists for.

Seen twice, independently: on PR 4553's head under
`nextest --profile ci -p topo --all-features`, then on a pristine
`origin/main` worktree (`f43f16b268`), and confirmed by PR 4553's
sequential review. Not this PR's: nothing in 4553 touches `topo`'s
boolean lane.

## Lead (unverified)

PR 4496 (`intent/s4-e-glue-on-zero`, "booleans glue on Zero") changed
what an undeclared flush contact does, which is this probe's premise.
Not bisected.

## Owed

- Decide what the flush case is now (glued, or refused somewhere
  else), and re-pose the probe's harvest on a subtract that still
  fires `bool_plane_orient`, or retire the flush half with the reason.
- The class this row is an instance of (probe suites the PR gate
  never runs) is `work/ciw/tests-red-under-all-features-never-run-by-ci.md`.

## 2026-10-15 — moved from TINT to CIW (CIW orchestrator, at Ev's direction)

TINT has no orchestrator. This red sits in a nightly job that CIW owns, and its fix is the test's own (a stale premise in the test, not a kernel defect), so CIW takes it. If the fix turns out to need kernel code, it is routed to the crate's owner.

## Closed

The premise was stale, not the kernel. #4496 (INTENT stage 4 E) made
rung 3 of the plane ladder glue an undeclared one-plane contact on its
decided Zero (`plane_eq.rs`, `bool_plane_offset`'s `Sign::Zero` arm)
instead of refusing, which INTENT-STAGE4-SPEC §6 asks for ("refused
ones build"). The flush subtract therefore builds, and
`bool_plane_orient` still fires on the way, since the orientation rung
decides before the offset. The probe's `is_err()` premise was flipped
to `is_ok()` in 60b58da54b, which reached main with #4641. At 1e-9 the
probe is green: `bool_plane_orient` 10 decisive samples,
`split_join_frame_arm` 1, `split_section_area` 1, worst relative
deviation 4.4e-16. This row's PR makes those three sites required, so
a silence there reds the probe rather than passing a vacuous linearity
check. (CIW lane)
