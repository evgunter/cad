---
id: recl-unbuilt-contact-line-refusals-are-classification-invariants
kind: issue
title: Three reachable recl.rs contact-line refusals are ClassificationInvariant (unbuilt) with no recourse
status: closed
opened: 2026-10-13
priority: P3
cost: E
refs: [an-edge-lying-in-a-face-at-a-pierce-refuses-ring-meets-ring]
closed: 2026-10-26
---

Filed by the TANG lane on PR 4967, from its review. That review moved
the same class out of `vtxfac::faces_holding_pierce` into a typed
variant, `BooleanError::HeldPierceUnbuilt`.

## What

`recl.rs`, in the on-ray group of the contact-line reclassification
(the `a_ms.len() > 1 || b_ms.len() > 1` arm), refuses three reachable
inputs as `BooleanError::ClassificationInvariant`:

- "a solid's coincident edges on one ray lie in a face of the other
  (unbuilt)" and "a contact line's ray holds a subdivision bisector
  (unbuilt)": both are now built, not refused
  (`a-contact-line-whose-ray-holds-a-subdivision-bisector-refuses-unbuilt`);
- "an edge crosses two wedges about a contact line on one ray
  (unbuilt)", reached by
  `a_third_prism_crossing_the_wedges_about_the_line_builds_or_refuses_typed`
  (`TWO_GERMS`).

`ClassificationInvariant` reads as a kernel bug, and its sentence names
no recourse. These inputs are valid and reachable; they are just not
built yet.

## The shape to give

Give them a typed unbuilt variant, as `HeldPierceUnbuilt` and
`RimCuspArmUnbuilt` are. It should carry the operand and the cells,
and its `Display` should end on a recourse a user can act on: fold the
members in another order, or move the solid off the contact line. The
two rows that assert the strings then match the variant instead.


## Closed (2026-10-26)

Fixed: the row records two of its three refusals as built, and PR 4987 (`6c42d047f1`, TANG) typed the third as `CrossingsAlongAContactLine`. None of the three `ClassificationInvariant` texts remains in `crates/topo/src/boolean/recl.rs`. (CLEAVE slate triage)
