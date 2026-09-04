---
id: unit-admits-non-finite-direction-norm
kind: issue
title: unit() admits the non-finite-norm class SEAT-DV closed at the datum door: a 1e200 Pattern direction silently mints coincident instances
status: review
opened: 2026-09-02
github: 1572
refs: [1564, 1570, 1372, direction-underflow-reports-zero-length, is-finite-length-homed-in-the-query-seat]
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
second wording.

Three rows, each its own `#[test]` so one failure never hides another:
a linear `Node::Pattern` with a `1e200` direction (which minted three
coincident instances at the merge base), a `Node::Transform` with a
`1e200` rotation axis (which the rigidity check refused downstream —
accidental coverage, now held at the door), and the same pattern
document at the ENCLOSURE scalar, which pins that nothing coincident
is minted there either.

Carried by the same PR, from its style review:

- the two refusal sentences no longer stutter — the roles are complete
  noun phrases, so the template names the role and stops
  (`"the pattern direction direction has no finite length"` was what a
  user read);
- the mate road names the direction it actually normalized, so a
  circular rule's refusal says `"datum axis direction"` and not
  `"pattern direction"`;
- `placement.rs`'s sentence about which refusal a non-finite axis
  yields is no longer stale, and says that the bit-identity claim
  above it is between the two MAPS and not the two refusals;
- `is_finite_length`'s own doc claims only what is true (every
  direction door in `topo` plus the evaluation layer's, not "every
  direction door"), and `query.rs`'s header names it as the second
  non-selection thing in the seat.

**What is NOT closed by this PR, deliberately.** On the mate road
(`mate/solve.rs`), the direction door's refusal does not survive the
translation into `MateFault`: a catch-all arm reports every
non-escalation refusal as `DanglingHead`, so a pattern direction with
no finite length is announced as a dangling head for a head that
resolves perfectly well. This is not new here — the door's own
`# Errors` block lists a degenerate direction among its `DanglingHead`
causes, so the mislabel predates this unit and already applied to the
decided-zero case; the non-finite refusal joined an existing bucket
through the catch-all arm.

What this unit measured is that the door's stated MITIGATION does not
hold: it defended the catch-all with "the pattern node's own
evaluation names the underlying cause in its own voice", and a mate
fault poisons the document, so that node evaluates to `Poisoned` and
the length is named nowhere. That rationale is corrected here rather
than extended — a disproved mitigation left standing tells the next
reader the compensation exists.

The fix itself is not taken here (FIX orchestrator's ruling, at the
review): one variant carrying the evaluation layer's typed refusal
verbatim and closing the catch-all is the proposed end state, but it
reverses a documented S-MATE decision on S-MATE's charter ground, and
a one-PR item needing another program's design assent is an item cut
wrong. Filed to S-MATE with this unit's measurement as the argument.

**What was swept for.** The class — a caller-supplied direction
normalized, or its length decided, without the finiteness question
asked first — grepped by SHAPE over `crates/editor-core/src/` in three
passes (`\.normalize\(\)`; `Margin::norm3|norm_squared|\.norm\(\)`;
`rotation_about|UnitVec3::new`). The full hit list with a disposition
per hit is in PR #1738's body. `Frame::rotate_then_translate`
(`placement.rs:76`) normalizes a caller's raw axis and its refusal was
verified by execution rather than read off its doc — a `1e200` axis
yields an all-NaN frame that `SetPlacement` (`edit.rs:1671`) and the
persist check (`persist/check.rs:726`) each refuse typed — so it is
not a live hole, but its refusal is spelled as a NaN sweep of a matrix
rather than as the length question, which is 1570's family.

**What the sweep could not match.** Only `editor-core`'s `src/`, only
at this merge base, and only by three syntactic shapes: it is blind to
the same hole in any other crate; to a direction normalized inside a
helper, since the call site shows neither `normalize` nor `norm`
(`Affine3::rotation_about_axis` at `eval/wire.rs:2349` is that shape
and was dispositioned by reading, not by matching); to sites reached
only through a trait method or macro expansion; and to anything merged
to main after this base. It also matched only the OVERFLOW end of the
range: the underflow twin is filed as
`direction-underflow-reports-zero-length`.

Two findings filed rather than taken:
`direction-underflow-reports-zero-length` (a direction under ~1e-162
is refused as "zero length", which it is not) and
`is-finite-length-homed-in-the-query-seat` (a question for SEAT about
where the predicate belongs).
