---
id: MSOLVE-14
kind: unit
title: The mate solve runs at the evaluation's own scalar: frames, coset fold and solved poses generic over T; the structure read at the nominal; Unpinned loses its producer
status: closed
opened: 2026-10-03
priority: P1
cost: H
branch: msolve/14-solve-at-the-run-scalar
pr: 3986
closed: 2026-10-04
---


Spec: `docs/MSOLVE-14-SPEC.md`. Plan item 19, approved by Ev on `[ev]`
PR 3679 (A11 (5) states it in place). The solve goes generic over the
evaluation's scalar: frames, placer maps, the coset fold, `check_offsets`
and the solved poses run at `T`, while the structure, the per-reference
counts and indices, and the reach stay nominal. The memo key carries
the pose at `T`. `Unpinned` loses its producer. Review tier: dual. The
unit changes every analysis run's reading of an assembly, the memo
key's inputs and a public payload. Dispatches after MSOLVE-13 merges.

Closed at merge on PR 3986, after a dual review (both arms on frozen
head `3053f4254`, under the late-trigger fallback) and a fix pass of
eleven rulings. What landed, and where the build departed from the
spec, is the 2026-10-04 MERGED entry in `work/msolve/log.md`.
