---
id: MSOLVE-14
kind: unit
title: The mate solve runs at the evaluation's own scalar: frames, coset fold and solved poses generic over T; the structure read at the nominal; Unpinned loses its producer
status: dispatched
opened: 2026-10-03
priority: P1
cost: H
branch: msolve/14-solve-at-the-run-scalar
---


Spec: `docs/MSOLVE-14-SPEC.md`. Plan item 19, approved by Ev on `[ev]`
PR 3679 (A11 (5) states it in place). The solve goes generic over the
evaluation's scalar: frames, placer maps, the coset fold, `check_offsets`
and the solved poses run at `T`, while the structure, the per-reference
counts and indices, and the reach stay nominal. The memo key carries
the pose at `T`. `Unpinned` loses its producer. Review tier: dual. The
unit changes every analysis run's reading of an assembly, the memo
key's inputs and a public payload. Dispatches after MSOLVE-13 merges.
