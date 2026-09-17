---
id: pick-closed-acceptance-loses-a-graze-to-rounding
kind: issue
title: the exact ray/triangle test loses a vertex or edge graze to rounding in u and v on near-tangent hits
status: closed
opened: 2026-09-16
closed: 2026-09-17
---


## The finding

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) documents
its closed boundaries — `u ∈ [0, 1]`, `v ≥ 0`, `u + v ≤ 1` — as the
reason "watertight meshes never lose a graze to an open boundary".
The comparisons are on rounded values, and a graze whose test is
ill-conditioned (a near-tangent hit, determinant ~1e-5 on a
unit-scale mesh) rounds its `u` or `v` a few 1e-14 across the
boundary on EVERY triangle sharing the point, so the graze is lost
to all of them and the ray falls through to whatever is behind, or
to a miss.

Measured at the guard unit (`pick-grazing-ray-answer-depends-on-candidate-order`,
its PR), over every landing of `viewer`'s `index_memo` corpus and the
gallery ring: of the tie-break row's aimed rays (`tie_rays_for`, each
aimed at a mesh vertex or an edge midpoint at distance `reach` along
an axis), 273 answer BEYOND the aimed point or miss — identically
before and after the box-entry guard, so this is the closed
acceptance's own class, not the guard's. One diagnosed: the gallery
ring at open, tie rays 32 and 33 (a +y/−y pair through the midpoint
of a boundary segment shared by patch 2 tri 72 and patch 3 tri 88):
both incident triangles compute `det ≈ ±2.66e-5`, `u = 0.5`,
`v = −1.03e-14`, `t = 1.48` — the aimed point exactly, refused by
`v ≥ 0` on both — and the pick answers the typed miss through a ring
the ray touches.

The tie-break row does not see this: it counts ties where they occur
and asserts nothing about a ray that misses its aimed point. A row
that would (every aimed ray answers at or before `reach`) is red on
`main` for these 273, which is why the guard unit did not add it.

The shape of a fix is a question for the row's owner: a rounding-aware
acceptance (a derived bound on the error of `u` and `v` in terms of
the operands and machine epsilon, never a tuned constant), or a
vertex/edge-aware fallback in the loop. Either changes the answers
the `index_memo` differential pins, and its tie floor.

## Parked on the `t` ruling, with EDIT-PICK2's measurement (2026-09-16)

This row does NOT close with
`pick-accepts-uncertified-barycentrics-on-a-certified-determinant`,
and its `rides_with` is cleared. EDIT-PICK2 ruled, built and measured
the acceptance that would have closed it — MEET, admitting a
barycentric whose interval reaches back into `[0, 1]` — and MEET
trades order independence: it admits a value outside the range, the
hit point `a + u·e1 + v·e2` then leaves the closed triangle, and the
caller's early-out stops before a candidate that would have won
(`work/edit/pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle`,
measured at 2 rays of 19 296 on the tie-break aim; MEET's gain was 20
of 149 aimed rays recovered). The unit landed the half that does not
trade it — the closed comparison ∧ INFORM — and this row waits on the
same ruling as
`pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`:
what the door answers for `t`, and with what width, once a candidate
is admitted at all.

### What EDIT-PICK2 did to this row's class

The landed acceptance (the closed comparison ∧ INFORM) moved this
class SLIGHTLY IN THE REFUSING DIRECTION, which is the honest reading
of the tally's `+12`. Measured over the wide aim's 441 126 rays
(`crates/viewer/tests/review_pick2_r1.rs`, `--nocapture`):

| | |
| --- | --- |
| answers that moved against `main` | 133, every one FARTHER |
| aimed vertices lost (main answered `t = reach`, the tree does not) | 3 |
| aimed vertices gained | 15 |
| net, the tally's third column | `141 094` → `141 106` |

The three losses are one ray class: `cut_cylinder` at open, `+z`
through `(-0.4842915805643155, 0.12434494358242767, 0.0595152840731647)`
at each of the three reaches, answering `0.5356375566584823` further
than the aimed vertex. They are corner grazes on a candidate at the
certification's floor, refused at INFORM because the barycentric bound
does not vanish where the barycentric does — filed with its own
asymmetry as
`pick-a-corner-graze-verdict-depends-on-the-corner-labelling`, and
stated at `ray_triangle`'s door as a cost rather than claimed to be
free.

Over the TIE-BREAK aim the count is unchanged at 149, and the review
lane checked that it is the same SET, not just the same number: no
aimed ray swapped sides (`newly_beyond_or_miss: 0`, `newly_aimed: 0`).

## Measured (EDIT-PICK3, 2026-09-16): stays

The `t` ruling this row waited on landed, and with it the clamp that
made MEET affordable in the column that killed it. EDIT-PICK3 ran the
three-rule table and the wide aim for BOTH acceptances under the
interval order (`crates/viewer/tests/pick3_acceptance.rs`, over every
landing of the corpus and the gallery ring):

| | closed ∧ INFORM | MEET ∧ INFORM |
| --- | --- | --- |
| tie-break aim, beyond-or-miss (19 296 rays) | 149 | 129 |
| tie-break aim, `Pruned ≠ Every` | 0 | **0** |
| tie-break aim, winners with a bound ≥ 1 | 0 | 0 |
| wide aim, aimed vertices lost against `main` (441 126 rays) | 0 | **513** |
| wide aim, aimed vertices gained | 3 | 0 |

**The clamp did what it was for**: MEET's 2 order-dependent rays are
gone, because an admitted candidate's hit point is now a point OF its
triangle and so inside its own box. **And MEET still cannot be taken**,
for a reason EDIT-PICK2 could not see past the first column: the clamp
places an out-of-range candidate's answer on the NEAREST point of its
triangle, which is a real point of the mesh at a smaller `t` than the
vertex the ray was aimed at — so the ray answers a face it passes
BESIDE instead of the one it passes through. 513 aimed vertices of the
wide aim go that way, against the 20 grazes MEET recovers.

The ruling's rule was "MEET only at `0 Pruned ≠ Every` and no lost
aim". The second half fails, so the closed comparison stays and these
149 grazes remain the stated class at `ray_triangle`'s door. This row is
measured, not fixed; it closes with EDIT-PICK3's merge.

## Unparked (2026-09-17, EDIT orchestrator)

The trigger fired: `what-t-the-pick-door-answers-and-with-what-width`
was ruled by Ev on `[ev]` PR #2764 and built by EDIT-PICK3, which
measured this row (its body's last paragraphs) and closes it.

## Closed (2026-09-17, EDIT orchestrator)

Measured under EDIT-PICK3's retraction and closed at its merge (PR
#2786): closed ∧ INFORM stays, because MEET reaches `0 Pruned ≠ Every`
under the retraction but loses 513 aimed vertices of the wide aim's
441 126 against the 20 grazes it recovers, failing the ruling's second
rule. The 149 grazes are the stated class at `ray_triangle`'s door,
recorded on `pick-refuses-a-crossing-within-rounding-of-a-plane`.
