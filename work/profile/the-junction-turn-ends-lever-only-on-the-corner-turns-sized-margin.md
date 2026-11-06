---
id: the-junction-turn-ends-lever-only-on-the-corner-turns-sized-margin
kind: issue
title: profile: path_junction_turn ends lever-only on the same sin φ·arm margin the two corner turns end Sized on
status: open
opened: 2026-11-06
---


(ROUND implementer, from `round/escalation-carries-its-gate`.)

## Finding

Three gates classify the same levered turn margin, `sin φ · arm`
against the run's linear band, and two of them end differently from
the third (`crates/profile/src/gate.rs`, the `gates!` table):

| gate | margin | ending |
|---|---|---|
| `fillet_corner_turn` (`sugar::arc_fillet_corner`) | sin φ · the shorter leg's extent | `Sized(FILLET_TURN_INBAND_RECOURSE)` |
| `path_corner_turn` (`path.rs`, `resolve_pending_ray_arrival`) | sin φ · the ray-origin-to-anchor separation | `Sized(CORNER_ANCHOR_SEPARATION_LEVER)` |
| `path_junction_turn` (`seg::junction_turn`, `path.rs` `junction_check`, validation's `seg::joint_tangency`) | sin φ · the shorter leg's arm | `Lever(JUNCTION_TANGENT_LEVER)` |
| `path_seam_arrival_turn` (`path.rs`, the declared seam arrival) | sin φ · the arrival's arm | `Lever(SEAM_ARRIVAL_LEVER)` |
| `path_seam_arrival_side` (the same arrival's cusp test) | cos φ · the arrival's arm | `Lever(SEAM_ARRIVAL_LEVER)` |
| `path_fillet_joint_turn` (`path.rs` `stored_joint`, the stored-form re-read) | sin φ · the shorter leg's arm | `Lever(FILLET_STORED_FORM_INBAND_RECOURSE)` |

The two corner turns were ruled `Sized` by the ROUND orchestrator for
that unit: the margin is levered by a length, so a smaller ε decides it,
and a real angle on a short arm is a size the user may intend (D4 ¶1 (i),
`docs/DESIGN.md` `[tolerance.recourse]`). `path_junction_turn` has the
same shape and ends in its lever alone, with no valued tighten, at both
the path door and validation. So do the seam arrival's two gates and the
stored form's joint turn, which read the same levered turn (or, for the
side test, its cosine) at other sites.

## Provenance of the `Lever` kind

`git log --all -S'kind: EndingKind::Lever(JUNCTION_TANGENT_LEVER)'` finds
`4e7fe80e3e` ("profile: one ending table with a closed kind;
full_turn_reach sized; the junction's own lever", ENCL, 2026-10-12),
merged by an agent in PR 4780. No `[ev]` ratification of the kind turned
up, so it is an implementer's choice, not a ruling.

## The question

Should `path_junction_turn`, and with it the three siblings above, be
`Sized` like the two corner turns? (`path_seam_arrival_side` is a cusp
test and passes only at one sign, so it may well stay `Lever` on its
own grounds.) Its
definite sibling `PathError::JunctionTangent` tells the lever story
alone. Making it `Sized` means its undecided ending gains
"or, if this size is intended, tighten the tolerance below X". The
definite arm would not change, since a reading decided Zero names no
size to tighten below.
