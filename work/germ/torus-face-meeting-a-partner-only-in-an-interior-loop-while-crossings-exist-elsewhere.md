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

**Measured, and wrong on main until the stopgap below** (it was
recorded here as unmeasured when filed): a half donut and a bracket
whose foot cuts an oval off the outer equator came back as a valid ∪
body with the lens counted twice, and a sphere cap cut the same way
came back wrong under every op.

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

