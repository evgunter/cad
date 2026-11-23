---
id: a-cylinder-reads-its-symmetry-by-how-it-was-made
kind: issue
title: A traced tangency reads a cylinder's symmetry by the construction that made it, so a revolved cylinder is not an extruded one
status: open
opened: 2026-10-30
priority: P3
cost: M
design: true
---


Ev, on [ev] #6064 (2026-10-30), approving its wording "the symmetry the surface's construction states": "this at least makes sense to start? it seems like we probably should treat cylinders the same way regardless of how they were made eventually but it doesn't have to be now".

## Today

`[coincidence.structural]`'s trace statement holds when the traced face's construction slides the wall along its axis. Stage 6 D (#6060) reads that as a role: the face is an extrude's `Lateral` (`crates/editor-core/src/coincide.rs` `slides_along_its_generators`). So a cylinder made another way is a different case:
- a revolve's band swept from a segment parallel to the axis;
- a boolean's or a fillet's face on a cylindrical carrier.

A boss traced on such a face, extruded along its axis, stays `Unproven` even though it is tangent for every value. D pins that as a control: `crates/editor-core/tests/intent_s6_d_tangency.rs` `a_boss_on_a_revolved_band_is_unproven` uses a band revolved from a quadrilateral leaning by `t`, a cylinder only at `t = 0`.

## Where the eventual reading has to decide something

"Treat cylinders the same regardless of how they were made" reads the face's kind, which today is a carrier verdict over values. A revolved band is a cylinder only where its profile segment is parallel to the axis, and that can hold at one value (D's `t = 0`) or by construction (a segment typed parallel by a director or a rung-one equality).

The open question is which of these a statement of "this face is a cylinder" may rest on:
- the carrier kind at the current values, which is rung two/three territory and is how the kernel classifies;
- a constructed parallel, which keeps rung one construction-only.

It is a design fork for the designer pair and then Ev, not a lane's call. It may also want to settle `a-revolve-traced-tangent-has-no-axis-read-off-the-face`'s step 3 (F Spherical by construction) the same way.

## Done when

A boss traced on a revolved cylinder, and on a cylindrical face that a boolean leaves, proves `Structural` wherever the ruling says it should. D's leaning-band control then moves or stays as the ruling says.
