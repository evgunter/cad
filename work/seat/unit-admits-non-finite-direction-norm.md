---
id: unit-admits-non-finite-direction-norm
kind: issue
title: unit() admits the non-finite-norm class SEAT-DV closed at the datum door: a 1e200 Pattern direction silently mints coincident instances
status: open
opened: 2026-09-02
github: 1572
refs: [1564, 1570, 1372]
---

## From GitHub issue 1572

opened 2026-09-02, 0 comments.

(SEAT orchestrator) Live defect found by SEAT-DV's fix pass (PR #1564) while probing the sibling door — reported there as a fork not taken (different door, explicit fix list), reproduced, and filed here.

`editor-core`'s `unit()` (`eval/wire.rs`) has the same hole `UnitVec3::new` had before SEAT-DV's fix: a direction with components ≳1e154 makes `norm_squared` overflow to +∞, the ∞ margin reads as maximally definite under `sign_within`, and `normalize` collapses the vector to zero. Measured end-to-end at PR #1564's fix head:

- `Node::Transform` with a `1e200` rotation axis IS refused — but downstream by the rigidity check, not the direction door (accidental coverage).
- `Node::Pattern` (linear) with a `1e200` direction is **NOT refused**: it evaluates to three instances at offsets `[0.0, 0.0, 0.0]` — silently coincident copies out of a decided path, in a fail-loud codebase.

The fix is one line from closed: `unit()` gates on the same value-channel finiteness question SEAT-DV shipped (`is_finite_length` via the poison self-difference — no bracket, no threshold, no `Bounds`), refusing typed before deciding. Red-first row: the linear-pattern reproduction above. The wider direction-family unification (two funnel doors, three direction spellings) is issue #1372's sibling #1570 and stays there; this issue is only the live hole.

## Home

`work/seat/` — SEAT-DV's own fork, the same parameter-identity/direction channel §3 of `docs/VERB-SEAT-DESIGN.md` charters and the sibling of the `UnitVec3` door SEAT closed.
