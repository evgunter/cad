---
id: the-containment-check-has-no-sphere-lane
kind: issue
title: The quieting rule's containment check has no sphere lane, so a holding Gap on a ball in its socket turns the finding unlocalized instead of quieting it
status: open
opened: 2026-10-27
priority: P2
cost: M
---

Filed from stage 5 C's delta review (PR 5810, probe d3).

`crates/editor-core/src/checks/at_rest.rs` `beyond` builds the region
past an asserted face's carrier, away from its copy's material, as a
solid: a box past a plane, the solid cylinder inside a bore, the tube
outside a pin. A sphere (or a cone) has no arm, and refuses "the
containment check has no lane for this carrier". So `quiet_verdicts`
turns a component a holding `Gap ≤ −δ` bounds into
`Overlap::Unlocalized(Unlocalized::Containment)` rather than quieting
it, and `could_quiet` finds no candidate for it.

Witness: `crates/editor-core/tests/intent_s5_c_quietable.rs`
`a_ball_in_a_concentric_socket_is_not_quietable`, a ball of 0.52 pressed
into a concentric spherical socket of 0.5. `Gap(socket, ball)` reads the
pair (`g = R − r − ‖Δc‖`), `opposed` answers it (concentric spheres pair
alike at every point), and every holding `Gap` demotes the finding. It
is not quietable and refuses at no severity, so nothing refuses wrongly;
a person simply cannot quiet a ball press fit.

Done when `beyond` has a sphere arm (the solid ball inside a socket, the
shell outside a ball, sized by the result's reach as the cylinder arm
is) and that row reads quietable, quiet under its `Gap`.
