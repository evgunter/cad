---
id: torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere
kind: issue
title: A curved face (torus, sphere; cylinder×cylinder unmeasured) that meets a partner face only in an interior loop, while crossings exist elsewhere, is classified by face-region propagation that cannot see the loop
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

**Measured, and reached by a union on main until the stopgap below**
(GERM torus-ops PR, 2026-09-28; recorded here as unmeasured when filed). The fixture is in `crates/sweep/tests/germ_torus_doors.rs`
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
pins that ∖ and ∩ refuse this shape at the roster. ∪ refuses it at
the interior-loop guard since the stopgap below.

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

The shape is kind-generic, as stated above. A sphere face with a small
cap cut clear of its seams, while crossings exist elsewhere, is the
same question, and it was measured wrong on main under every op (the
Stopgap section below).

## Home

GERM, beside the torus doors. Found by the PR #3265 fix pass's
roster re-sweep.

## Stopgap (GERM interior-oval stopgap PR, 2026-09-28)

Measured on main: `union(half_donut, bracket)` answered a valid
`Seamed` body with the oval's lens counted twice, and the sphere
analogue (`dome`, `dome_bracket`: a cap cut off a hemisphere's side,
clear of its seams, while the bracket's top crosses the crown)
answered every op wrong: `dome ∩ bracket` came back as the crown
alone, missing the side cap. Fixtures and rows are in
`crates/sweep/tests/germ_interior_oval.rs`.

`ops::interior_loop_verdict` now guards the crossings path. It is
decided on the reduction and raised only where a body would be
returned, as `CurvedPairUnsupported { op: Some(op) }`:

- **torus face:** refused on reach. Any undeclared face of the other
  operand whose box overlaps it refuses, unless the two carriers are
  certified apart (`ops::carriers_apart`: a plane beyond the torus's
  support, a ball clear of the tube). This refuses correct answers
  too: the pin-only bracket (pinned as
  `the_pin_alone_is_the_torus_guards_conservative_refusal`).
- **sphere face:** refused per pair.
  - A pair with an edge event passes when the partner is a plane, a
    sphere or a cylinder. Their section with a ball is one closed
    curve, or, for a cylinder, components that each cross the wall's
    seams.
  - A pair with no event passes only on a certificate: the carriers
    apart, or, against a plane, one point of the section circle outside
    either face (`ops::circle_misses_a_face`; with no event the circle
    lies in both faces wholly or not at all).
  - Everything else refuses.

**A second instance, the corner bar**, found in PR 3330's dual review
(`the_corner_bar_never_comes_back_a_body`): a bar across the hole with
all eight corners on the inner face, whose end-square edges run inside
the tube. Its only events are the corners' contacts.

- It never reaches the guard today. On main it refuses at the chord
  rule (`CurvedPierceUnsupported`). With PR 3330's chord relaxation it
  refuses at the sagitta charge (`CurvedSectorSideUnsupported`),
  measured on a scratch merge.
- The torus half covers it anyway, because it never consults events:
  any undeclared overlapping pair refuses. A per-pair rule that cleared
  pairs WITH events would not have covered it.

**Still open, and why this row stays open:**

- **(b)** is not built. It would enumerate each overlapping torus
  pair's section, so that a torus face whose locus stands apart from
  the partner, or whose loop is cut in, answers instead of refusing.
- **The rest of the kind-generic class is unguarded and unmeasured:**
  - cylinder × cylinder;
  - cone faces (these refuse at the operand gate anyway);
  - NURBS × plane or NURBS × sphere loops. A sphere face's NURBS
    partner refuses on reach.

