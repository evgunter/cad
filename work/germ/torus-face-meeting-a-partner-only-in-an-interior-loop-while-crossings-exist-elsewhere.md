---
id: torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere
kind: issue
title: A torus face that meets a partner face only in an interior loop, while crossings exist elsewhere, is classified by face-region propagation that cannot see the loop
status: open
opened: 2026-09-26
refs: [torus-operand-gate-admission]
priority: P0
cost: H
---

## What

PR #3265's `torus_extent_gate` (`crates/topo/src/boolean/ops.rs`)
closes the no-crossings case. There, a torus face that may meet a
partner face in a closed loop interior to both refuses typed, instead
of reaching `classify_shells`' vertex probe.

The same loop can exist when the operation DOES have crossings
somewhere else, for example a bracket whose plate grazes a donut's
outer equator in an oval while its leg pierces the tube. The fallback
is not taken then. The torus face that holds the oval is classified
through the joined regions and `finish.rs`'s uncut-component probe,
and neither sees a loop no edge crosses. The shape is kind-generic:
cylinder wall×wall pairs have it too, and `cylinder_extent_gate`
guards only the no-crossings fallback.

**Measured, and reached by a union on main** (GERM torus-ops PR,
2026-09-28). The fixture is in `crates/sweep/tests/germ_torus_doors.rs`
(`half_donut`, `bracket`): the donut's profile revolved by `π` about
`y` (`R = 2`, `r = 0.5`, the half with `z ≤ 0`), and a `0.6`-thick
C-shaped bracket whose pin crosses the `x > 0` cap (`x ∈ [1.95, 2.05]`,
down to `z = −0.1`, inside the tube) and whose foot's top face
`z = −2.45` cuts an oval off the outer equator's crown. The pin's four
edges pierce the planar cap and nothing else; every other edge of
either body misses the other body's faces. So the op has crossings,
the no-crossings extent gate never runs, and the oval is seen by
nothing.

- `topo::union(half_donut, bracket)` returns `Ok`, kind `Seamed`,
  valid at tiers 1–3, with volume `6.404802…` =
  `vol(H) + vol(C) − 0.006`: only the pin's overlap is removed, and the
  oval's lens is counted twice. The body carries the whole outer torus
  face and the whole foot face, which cross in the oval.
- With `Torus` admitted to `revert_arm_exists` (a scratch change, not
  landed), `topo::intersect` returned a valid body of volume `0.006`
  (the pin's piece alone), and `point_in_solid` read
  `(0, 0, −2.47)` — `In` both operands — as `Out` of it; `subtract`
  both ways missed the lens the same way.

This is a confident wrong answer on `union` today, which is why the
roster admission (`torus-onto-the-subtract-and-intersect-roster`)
stopped: admitting the kind extends it to ∖ and ∩. The row
`subtract_and_intersect_refuse_an_oval_their_crossings_cannot_see`
pins that ∖ and ∩ refuse this shape at the roster; ∪ has no row,
because the only true answer a row could assert today is the
refusal no door gives.

**The fork.** Two shapes of fix, and choosing is a posture question:

- a reach gate on the CROSSINGS path, the no-crossings
  `torus_extent_gate`'s argument carried over: a torus face whose box
  overlaps an undeclared face of the other operand refuses. Sound, and
  cheap, but it refuses every undeclared torus op whose boxes meet
  (the pin alone, which answers correctly today, among them), which
  gives back most of what the union's torus doors admitted;
- a certificate: for each (torus face, partner face) pair whose boxes
  overlap, the section (`geom_brep::plane_torus_section` for a plane
  partner) is enumerated and every closed component interior to both
  faces refuses, or is cut in. Sound and narrow, and it is the H.

The shape is kind-generic, as stated above; a sphere face with a small
cap cut clear of its seams, while crossings exist elsewhere, is the
same question, and is unmeasured.

## Home

GERM, beside the torus doors. Found by the PR #3265 fix pass's
roster re-sweep.
