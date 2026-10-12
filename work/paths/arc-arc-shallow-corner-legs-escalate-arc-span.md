---
id: arc-arc-shallow-corner-legs-escalate-arc-span
kind: issue
title: A fillet's own legs escalate a span predicate against each other at small turns, so the loop cannot validate whatever the fillet does
status: closed
opened: 2026-09-13
priority: P0
cost: H
closed: 2026-10-07
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

## The class, widened (BLEND-10 fix pass)

The arc × arc witness above is one instance of a class, not a fact about
`arc_span`. The general statement: **a fillet at a small turn leaves its
own two legs close together, and validation's pair pass has to separate
them at a margin that vanishes with the turn.** Which predicate lands in
the band depends on what the legs are — a chordal defect between two
near-cocircular arcs (`arc_span`), or an arc-length parameter between a
straight leg and its neighbour (`line_span`) — and the fillet is fine in
every one of them.

Two more instances, both at `CAD_TOLERANCE_EPS=1e-12`, both loops the
path door BUILDS at BLEND-10's head and `Profile::validate` then refuses
between **adjacent** segments 1 and 2:

- `line x arc`, turn `32·√ε`, radius 0.5 — `line_span` indeterminate;
- `line x arc`, turn `128·√ε`, radius 0.2 — `line_span` indeterminate.

The fixtures are `crates/profile/tests/fillet_stored_tangency.rs`'s
`line_arc` at those turns and radii, and
`crates/profile/tests/review_fillet_stored_tangency_r2_probes.rs`'s
`corpus_loops_built_at_the_head_that_validation_refuses_elsewhere`
records both with the refusal read off the error.

What this bounds, as before: a fillet-side row can promise that the door
never mints a declaration validation contradicts, and cannot promise
that every loop with a small-turn fillet in it validates — the legs
around the fillet are the other half of that, and they are this item's
subject.

## Outcome

**Re-measured on main (`19f9037063`), after #4264 and #4276.** The
witness still escalated. The `arc_arc` fixture was swept over 10⁻⁸ to
10⁻¹ rad, at radii 0.02, 0.05, 0.2, 0.5 and 1, and at ε = 1e-6, 1e-9
and 1e-12. At 1e-9 and 1e-12 every corner the door builds validates;
the door refuses the shallow turns first, at the offset-lever gate. At
1e-6, 19 of 331 built corners escalated in validation: 15 between legs
0 and 2 (`arc_span`) and 4 between leg 0 and the fillet. The line × arc
instances in "The class, widened" already validate on main at every ε.

**Root cause: not honest.** An independent distance between the legs
(closed form: endpoints, common normals, crossings) put every
escalating leg pair 1.114 to 2.108 Kε apart, so definitely apart (the
1.00 to 1.06 Kε values belong to the adjacent corners). The separation
is the fillet's chord, r × the fillet's sweep, not the O(R·t²) carrier gap
the item supposed, so it moves with the fillet radius. The pair pass
read the carriers' crossing, which stands about half that chord past
each leg's end, in band on both spans. A crossing at angle φ stands for
a stretch ε/sin φ long, so at a shallow turn its position is not a
question the segments need answered (the #4276 argument).

**Change (`crates/profile/src/seg.rs`).** A candidate that a span reads
in band is settled by the segments' ends, as after a definite miss,
when, for each segment that reads it in band, the end nearest the
candidate (`nearer_end`) reads on the other carrier and, where the
carriers have a second candidate, definitely nearer this one than that
(`own_candidate`). Otherwise the escalation stands. Along a carrier the
distance to the other rises from each candidate to one peak midway to
the next, so an end nearer its candidate than the rival, and within ε,
has the carrier within ε all the way back to the candidate: the end
read stands only for the candidate's own stretch, whatever the span's
reach. After the change, 0 of the 15 leg-pair escalations remain (0 of
45 at thirty turns a decade). The 4 adjacent ones remain, at the
smallest buildable turn per radius: the fillet's far end is 1.0 Kε from
its leg along a quarter turn, which is inside `arc_span`'s documented
reach.

**Rows.**
- `seg::pair_contact_tests::a_crossing_read_in_band_settles_on_the_ends_inside_its_stretch`
  (f64 and Interval). On the unfixed code, 8 of its rows are wrong at
  every ε. The no-guard mutant reds its square row, and the swapped-end
  mutant reds 8 rows.
- `rejections::legs_stopping_short_either_side_of_a_shallow_crossing_validate`
  (every ε).
- `fillet_stored_tangency::a_shallow_arc_x_arc_corner_validates_wherever_its_legs_stand_apart`
  (the witness, and 36 corners within 10 Kε at 1e-6).
- `seg_reach_fuzz` gains a shallow-crossing family. It finds 0 misses
  at effort 10 at all three ε, against 122 to 171 for the no-guard
  mutant.

**Moved.**
- The sym11 past-the-ceiling receipts for the bracket and the pad now
  replay past validation: bracket `[1390,58,1008,1865]`, pad
  `[1248,69,1159,2914]` on the merged head. The SYM drift row
  attributes the values main's merges moved in the stages past
  validation (#4304, #4189, #4640).
- `fillet_stored_tangency`'s 1e-6 golden: `arc x arc c=0.3 r=0.05` now
  validates, and nothing else in the dump moves.
- `review_m2_pr5`'s near-tangent join now validates: its joint is
  derived tangent and recorded (D10 stage 4 PR G), and the pair pass
  settles the shallow crossing on the ends. On the axis the revolve
  then refuses at the arc's ring-torus clearance
  (`SliverAxisClearance`): at 40–50 digits the carrier crosses the axis
  by about 6ε at every ε. The row now stands 3 off the axis and reaches
  its subject, `SliverJoin`; a second row pins the on-axis refusal.
- At 1e-12, one shallow-arc grid cell now refuses at the extrude's
  attachment gate instead of at validation. This closes
`the-past-the-ceiling-row-replays-only-validation-on-the-bracket-and-pad`
(SYM).

**After merging `main` (`8192224a78`).** Main's D10 stage 4 PR G
(`5484404bc2`, `ed553c418f`: tangent joints derived, not stored)
reworked the joint pass and left the pair pass's `candidates`
untouched, so the change re-applies as it was. On main alone the
witness still escalates (`arc_span`, segments 0 and 2, at 1e-6; the
door refuses it at 1e-9 and 1e-12). The sweep at ten turns a decade
reads 19 escalations of 116 built corners at 1e-6 (15 leg pairs, 4
adjacent), and none at 1e-9 (116 built) or 1e-12 (4 built); at thirty
a decade, 51 of 340 (45 leg pairs, 6 adjacent). On the merged head
the leg pairs read 0 and the adjacent ones stand (4 and 6). The
earlier count of 331 built corners is not reproduced on this grid.
D10's hold did not bind: no tangent joint's derivation, recording or
verification changed.

**Fix pass (review of #4290).** The first guard asked only that the
nearer end read on the other carrier. An end beside the carriers'
OTHER crossing passes that, and `arc_span`'s in-band zone, hundreds of
Kε on an arc past about 5.6 rad or of a radius of a few Kε, can reach
both: a figure-eight whose arcs cross 2.7e-3 inside a 2π − 0.011 rad
arc validated at every ε, at `f64` and `Interval`
(`rejections::a_crossing_read_in_band_is_not_settled_by_an_end_beside_the_other_crossing`).
`own_candidate` closes it; the witness escalates as on main. The leg
pairs stay fixed (0 of 15 at ten turns a decade, 0 of 45 at thirty, at
1e-6; nothing escalates at 1e-9 or 1e-12). A fuzz family whose two
crossings stand 0.5ε to 10³ε apart
(`seg_reach_fuzz::a_crossing_near_its_rival_reads_no_contact_only_past_eps`)
finds 4 misses per ε at effort 20 against the first guard and none
against this one or main. It also found main's `f64` secant arm
misplacing crossings at unequal radii, filed as
`arc-arc-secant-arm-misses-crossings-at-unequal-radii` (P0).

