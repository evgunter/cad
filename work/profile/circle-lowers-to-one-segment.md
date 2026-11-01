---
id: circle-lowers-to-one-segment
kind: unit
title: circle and circle_split(n ≥ 1) lower to one full-turn segment per arc; lily migrates; re-baseline
status: open
opened: 2026-09-25
priority: P1
cost: H
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
---


Unit 4 of the #3218 lowering. `circle(c, r)` lowers through `circle_split`'s kernel with n = 1, phase 0, and stays its own verb in the program (Ev, #3218 q3). `CircleSplitCount` refuses n = 0. The anchor's n = 2 paragraph goes. `lily::foot` drops its `circle_split(3)` workaround; bossplate and twopeg keep theirs, which are deliberate. Goldens, censuses, names and Python pins re-baseline. EMIT's `Piece(0)/Piece(1)` circle names become one `Carrier` (settled on #3202: second names break). Loft over a circle waits on TESS's `lofted-circle-sections-are-unmeshable…`.

## From unit 3 (2026-10-06)

- Revolve and loft refuse a one-segment loop (`OneSegmentLoop`) until
  `one-segment-loop-revolves-and-lofts-to-one-wall` builds the wrap
  edge (#4175); this unit makes every revolve or loft of a `circle`
  take it.
- Validate's `ByConstruction` full-turn arm (`seg::build_loop_seg`)
  decides only the turn's reach and sense: no door constructs a full
  turn yet, so the arm trusts the construction to write its start on
  its carrier and |Δθ| = 2π. This unit's `circle_split` kernel is that
  construction, and owns showing it.
- An extruded one-segment cylinder cut ACROSS its wall by a plane (a
  slab, a pocket floor) refuses `Join(SingleSiteSectionLoop)`
  (`work/sect/closed-in-face-section-loop-has-one-site.md`), where
  today's two-arc cylinder builds. That row is parked on D10.
- `lift::lift_seamed` still refuses fewer than two vertices
  (`LiftRefusal::TooFewVertices`, "the chain vocabulary spells a loop
  of at least two vertices"):
  a one-segment loop lifts to `circle` once `circle` lowers to one.

## Evidence from JOIN (PR 4345, branch `join/wrap-edge-section-loop`)

- The 2026-10-06 note above says the cylinder cut across its wall by a
  slab or pocket floor refuses `Join(SingleSiteSectionLoop)`. With
  JOIN's wrap-edge arm (`a-plane-across-a-one-face-wall-meets-its-wrap-edge-once`)
  that cut builds. In every op and both orders it reaches its closed
  form, tiers 2 and 3′ and the certificate:
  - `crates/sweep/tests/a_plane_across_a_one_face_wall.rs`;
  - `one_segment_loop.rs`
    `a_boolean_on_an_extruded_seam_wall_builds_along_and_across_it`.
- A washer, an annular one-segment tube through a plate, still refuses
  when its two wrap edges sit at different azimuths. That is filed as
  `annular-one-segment-tube-through-a-plate-refuses-section-loop-undecided` (JOIN, closed by PR 4397).

## Its blocker has closed (2026-10-14, branch `paths/in-face-section-self-loop`)

`closed-in-face-section-loop-has-one-site` is closed: a one-segment
circle lying in the partner's face joins as a one-site section loop
(`boolean/join.rs` `one_site_segments`). The rows that measured the
class build in every op and both orders, at their closed forms
(`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`): a boss
on a plate (the parked unit's witness), the coaxial equal-cylinder
union, a rim inside a ball face, a puck and its rounding ring. Still
refused typed: a one-segment rim lying in a cone, cylinder or torus
face, at the crossing layer (`CurvedPierceUnsupported`; SECT's
`a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm`).
The four other regression groups the 2026-10-14 log entry names
still need attributing when the unit resumes.

**A likely second blocker** (CLEAVE, P0):
`work/cleave/a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells.md`.
A one-segment boss over a hole or pocket tangent inside its rim, both
vertices at the tangent point, unites as two shells sharing the
crescent, silently (tier 3′ fails; production does not run it). Today
that needs both circles phased alike by hand; once `circle()` lowers
to one segment every circle's vertex sits at azimuth 0, so a boss over
a pocket tangent inside it on the `+x` side builds the wrong body.
That union refuses typed `TangentCrescentUnsupported` since PR 5687, in
both forms. What this unit still turns from a build into a refusal is
the one-segment tangent forms whose two-arc forms build:
`work/cleave/one-segment-tangent-rims-refuse-where-their-two-arc-forms-build.md`.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

The four regression groups PATHS's log named on 2026-10-14, still to be attributed when the unit resumes: `CurvedPierceUnsupported`, `CarryRefused{Found, Unread}`, the containment verdict `None`, and `verbs_germarms`. The log goes with PATHS's directory; its ledger note gives the SHA to read it at.
