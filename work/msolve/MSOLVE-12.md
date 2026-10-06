---
id: MSOLVE-12
kind: unit
title: Two decidably non-parallel planes solve or refuse in their own words, a mate's Band is reached the way a state reaches it, and the cluster maintenance leaves the solve's file
status: closed
opened: 2026-10-01
priority: P0
cost: H
branch: msolve/12-honest-translation
pr: 3698
closed: 2026-10-03
---


Spec: `docs/MSOLVE-12-SPEC.md`. Plan item 23 gathers three rows that
were routed here on 2026-09-20 and left untriaged until 2026-10-01:
- `near-parallel-planes-refuse-under-a-false-predicate` (P0);
- `mate-band-fault-unreachable-on-a-mate` (P0);
- `mate-solve-carries-the-cluster-maintenance-half` (P1).

The work: the translation stage at the parallel edge solves in the
line's own frame, or refuses in its own words; a mate's `Band` is
reached by a loaded snapshot; the D-3 maintenance moves to
`mate/maintain.rs`.

Review tier: single, full. Dispatched 2026-10-01 on Opus.

## Closed

Merged on PR 3698 (2026-10-03).
- **Review:** single, full, on `fc5ea8dca`. APPROVE-WITH-FIXES, no MAJOR,
  C1–C5 held. The closed forms were checked against exact rationals
  over about 31,000 poses.
- **Fix pass:** R1–R9.
- **Merge of main:** after #3676 deleted the cluster maintenance, §3's
  move went with the code. The merge also guarded main's new offset
  check (`trivial_member`) with R1's lever floor, and gave an
  unmeasurable offset its own arm (`OffsetCheck::OutOfRange`).
- **Rows:** in the PR body.

