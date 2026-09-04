---
id: unit-admits-non-finite-direction-norm
kind: issue
title: unit() admits the non-finite-norm class SEAT-DV closed at the datum door: a 1e200 Pattern direction silently mints coincident instances
status: review
opened: 2026-09-02
github: 1572
refs: [1564, 1570, 1372]
branch: fix/unit-finite-norm
pr: 1738
---

## From GitHub issue 1572

Opened 2026-09-02; 0 comments.

(SEAT orchestrator) Live defect found by SEAT-DV's fix pass (PR #1564) while probing the sibling door — reported there as a fork not taken (different door, explicit fix list), reproduced, and filed here.

`editor-core`'s `unit()` (`eval/wire.rs`) has the same hole `UnitVec3::new` had before SEAT-DV's fix: a direction with components ≳1e154 makes `norm_squared` overflow to +∞, the ∞ margin reads as maximally definite under `sign_within`, and `normalize` collapses the vector to zero. Measured end-to-end at PR #1564's fix head:

- `Node::Transform` with a `1e200` rotation axis IS refused — but downstream by the rigidity check, not the direction door (accidental coverage).
- `Node::Pattern` (linear) with a `1e200` direction is **NOT refused**: it evaluates to three instances at offsets `[0.0, 0.0, 0.0]` — silently coincident copies out of a decided path, in a fail-loud codebase.

The fix is one line from closed: `unit()` gates on the same value-channel finiteness question SEAT-DV shipped (`is_finite_length` via the poison self-difference — no bracket, no threshold, no `Bounds`), refusing typed before deciding. Red-first row: the linear-pattern reproduction above. The wider direction-family unification (two funnel doors, three direction spellings) is issue #1372's sibling #1570 and stays there; this issue is only the live hole.

## Home

`work/seat/` — SEAT-DV's own fork, the same parameter-identity/direction channel §3 of `docs/VERB-SEAT-DESIGN.md` charters and the sibling of the `UnitVec3` door SEAT closed.

## Closed

PR #1738 (`fix/unit-finite-norm`).

**What landed.** `editor-core`'s `unit()`
(`crates/editor-core/src/eval/wire.rs`) asks the finiteness question
before deciding the length, through the kernel's own predicate rather
than a second spelling of it: `topo::query::is_finite_length` — the
value-channel self-difference SEAT-DV shipped for `UnitVec3::new`
(#1564), no bracket, no threshold, no `Bounds` — becomes `pub` in
`topo::query` and both direction doors call it. A non-finite length
raises the refusal that already existed for the datum door,
`NodeErrorKind::NonFiniteDirection { role }`; no new variant and no
second wording. Red-first row
`m4_pr2_wire::non_finite_direction_refuses_at_the_direction_door` pins
both roles this layer owns: a linear `Node::Pattern` with a `1e200`
direction (which minted three coincident instances at the merge base)
and a `Node::Transform` with a `1e200` rotation axis (which the
rigidity check refused downstream — accidental coverage, now held at
the door), and pins that the pattern refusal's sentence names the
direction.

**What was swept for.** The class — a caller-supplied direction
normalized, or its length decided, without the finiteness question
asked first — grepped by SHAPE over `crates/editor-core/src/` in three
passes (`\.normalize\(\)`; `Margin::norm3|norm_squared|\.norm\(\)`;
`rotation_about|UnitVec3::new`). The full hit list with a disposition
per hit is in PR #1738's body. No second live hole of this shape was
found: `Frame::rotate_then_translate` (`placement.rs:76`) normalizes a
caller's raw axis, and its refusal was verified empirically rather than
read off its doc — a `1e200` axis yields an all-NaN frame that
`SetPlacement` (`edit.rs:1671`) and the persist check
(`persist/check.rs:726`) each refuse typed.

**What the sweep could not match.** Only `editor-core`'s `src/`, only
at this merge base, and only by three syntactic shapes: it is blind to
the same hole in any other crate; to a direction normalized inside a
helper, since the call site shows neither `normalize` nor `norm`
(`Affine3::rotation_about_axis` at `eval/wire.rs:2349` is that shape
and was dispositioned by reading, not by matching); to sites reached
only through a trait method or macro expansion; and to anything merged
to main after this base.

The wider direction-family unification stays #1570 under #1372;
`clearance.rs`'s `chart_frame` finiteness door is a third spelling of
the question and is noted there, not taken here.
