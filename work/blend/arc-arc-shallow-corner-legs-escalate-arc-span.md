---
id: arc-arc-shallow-corner-legs-escalate-arc-span
kind: issue
title: A shallow arc x arc corner's near-cocircular leg arcs escalate arc_span in the pair pass, so the loop cannot validate whatever the fillet does
status: open
opened: 2026-09-13
---


## The witness

Two radius-2 circles about `(-t, 0)` and `(t, 0)` cross at
`(0, sqrt(4 - t^2))`, where their tangents are exactly `t` apart — the
arc x arc fixtures' vesica (`crates/profile/tests/arc_fillet.rs`,
`arc_arc_internal`) with its corner opened out to a shallow turn. The
construction is `crates/profile/tests/fillet_stored_tangency.rs`,
`arc_arc`.

At `CAD_TOLERANCE_EPS=1e-6` and `t = 5e-5` the path door builds the
loop and both fillet joints classify `Tangent` — the fillet is fine —
but `Profile::validate` refuses:

    validation escalated between loop 0 segment 0 and loop 0 segment 2:
    predicate 'arc_span' indeterminate: margin -5.132675275909548e-6
    lies inside the ambiguity band (zero = 1e-6, escalate = 1e-5)

Segments 0 and 2 are the two LEG arcs, not the fillet.

## Why it is structural, not a fixture accident

A shallow crossing of two circles is a near-tangency: for two circles of
radius `R` whose tangents at the crossing differ by `t`, the centres are
`O(R*t)` apart and the carriers are within `O(R*t^2)` of each other over
the whole neighbourhood of the crossing. So EVERY arc x arc corner at a
tiny turn has leg carriers the simplicity pair pass has to separate at a
margin that vanishes with the turn — `arc_span`
(`crates/profile/src/seg.rs`, the `arc_span` predicate, "the chordal
defect from the apex") lands in the band, and the loop escalates. The
turn at which that happens is a property of the legs, not of the fillet
between them, and no fillet radius moves it.

Two consequences worth deciding:

- a caller authoring a shallow arc x arc corner gets an escalation whose
  sentence is about a segment pair, with no recourse naming the turn
  that would fix it;
- it bounds what any fillet-side row can claim about arc x arc corners
  at small turns: the loop is unvalidatable below some turn whatever the
  fillet door does. BLEND unit 10's rows say so explicitly
  (`crates/profile/tests/fillet_stored_tangency.rs`,
  `a_fillet_the_stored_form_carries_builds_and_validates` reads the
  arc x arc corner at an absolute turn for exactly this reason).

## Found by

BLEND unit 10, while sweeping three corner kinds x four decades of turn
x three eps rows for the fillet stored-form measurement. Not in that
unit's fence: the escalation is in the simplicity pair pass, between two
authored legs, and has nothing to do with the tangency a fillet
declares.
