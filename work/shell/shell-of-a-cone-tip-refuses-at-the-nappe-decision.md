---
id: shell-of-a-cone-tip-refuses-at-the-nappe-decision
kind: issue
title: shell of a body with a cone tip refuses NappeStraddles: a cone face reaching its apex has no nappe to turn the offset by
status: open
opened: 2026-10-06
priority: P2
cost: M
refs: [shell-open-refuses-a-curved-designated-face]
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
