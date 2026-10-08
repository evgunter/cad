---
id: shell-of-a-cone-tip-refuses-at-the-nappe-decision
kind: issue
title: shell of a body with a cone tip refuses NappeStraddles: a cone face reaching its apex has no nappe to turn the offset by
status: closed
opened: 2026-10-06
priority: P2
cost: M
refs: [shell-open-refuses-a-curved-designated-face]
pr: 4356
closed: 2026-10-08
---


`topo::shell` (sealed, and so every `shell_open`) of a vessel under a
cone tip, a cylinder `r = 0.5`, `h = 0.6` capped by a cone of height
`0.4` whose apex is on the axis (`common::shell_operands::cone_tipped_vessel`),
refuses at the cavity's own door at `t = 0.05`, `Tol::witness()`:

    Face { error: NappeStraddles { station_min: -0.4, station_max: 0.0,
      what: "a cone face whose own corners reach its apex, so it stands on neither nappe alone" } }

`topo::offset_nappe::face_nappe` decides the nappe from the face's
corner stations, and a face whose corner IS the apex decides `Zero` on
that station, so the two extremes never agree. A face that reaches its
apex from one side lies on that side's nappe; the open question is
whether the doors' apex-window gate and the axial corner solve then
take the apex vertex (on the axis, so its station is determined by the
moved apex) or need their own arm.

Pinned in `sweep`'s `shell_curved_mouth::a_cone_tip_and_a_tangent_dome_refuse_in_the_sealed_arm`.
It blocks two fixtures that `shell-open-refuses-a-curved-designated-face`
owed: the opened cone tip, and step-import's apex cone of two half-faces
(the same topology). Whether the rim stage then takes the cone is
unmeasured. The seamed band (`shell::seamed_band`) is built through a
pole, and its struts run along each seam's own carrier, which is a
ruling on a cone where it is a meridian on a sphere; its only measured
pole so far is a sphere's.

## The lift's cone arm is unreached (PR 4191's review, folded here)

`shell::lift_to` turns the inverse offset distance by the counterpart
group's nappe (`offset_nappe::group_nappe`) on a cone, and no buildable
body reaches that arm today. Both reviewers of PR 4191 measured it:

- a full revolve's cone band (a roof, a skirt) cannot be designated
  without disconnecting the shell (`OpenFacesDisconnect`);
- a partial-revolve frustum refuses in the sealed door
  (`Face { TogetherEdgeDisagreement }`);
- the cone tip refuses here (`NappeStraddles`).

The day this item lands, the cone tip's opened row is the first body
through the cone arm. It owes an assertion that the lifted cone lands
on the designated one, on a mirror-nappe face as well as an opening
one.

## Closed

`topo::offset_nappe::face_nappe` reads a corner whose station decides
Zero as the apex, which is on both nappes and decides neither: a face
that reaches its apex lies on the nappe its other corners stand on
(`(Zero, Positive)` is `Opening`, `(Negative, Zero)` is `Mirror`), and
only a face with corners strictly on both sides, or every corner at the
apex, refuses `NappeStraddles`. The axial door takes the apex vertex
with no arm of its own: it is an axis pole, and the pole arm puts it on
the moved apex. Measured on the cone tip (mirror nappe) and on its
mirror image, `common::shell_operands::funnel_vessel` (opening nappe):
both shell sealed, opened through the cone, and opened through the flat
cap, tier 3, at the closed-form volume.

- **The lift's cone arm is reached.** Opening the cone lifts the
  cavity's counterpart cone through `shell::lift_to`'s nappe turn; the
  band wears the designated cone (apex unmoved), and the cavity wall's
  corner lands on it at radius `r − t`, on both nappes.
- **The seamed band takes a cone pole.** Both half-faces survive under
  their keys, ring-free, genus 0, the apex dies, and the band meshes.
- **The per-chart door's apex-window gate does not take it**: it reads
  a window whose near end is the apex as not cleared and refuses
  `ApexWindow`. No `shell` path reaches that door on a cone; filed as
  `per-chart-door-refuses-a-cone-window-reaching-its-apex`.
- Step-import's apex cone of two half-faces was not measured here.

Pinned in `sweep`'s
`shell_curved_mouth::a_cone_tip_shells_through_its_apex_on_either_nappe`.
