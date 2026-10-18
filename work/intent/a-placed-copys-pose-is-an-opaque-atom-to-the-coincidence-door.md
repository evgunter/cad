---
id: a-placed-copys-pose-is-an-opaque-atom-to-the-coincidence-door
kind: issue
title: A placed copy's pose is an opaque atom to the coincidence door
status: open
opened: 2026-10-18
priority: P2
cost: H
parent: transform-retires-into-a-placement
---

The coincidence door replays a recorded row with every variable a symbol and asks whether the margin is the zero polynomial (`crates/editor-core/src/coincide/replay.rs`, `discharge_or_why`). A row it cannot close gets a recourse:
- `OneConstruction` when the residual is a function of the variables alone;
- `Assert` when it keeps a value computed apart from them (`Undischarged::KeepsAnOpaqueAtom`).

Before D, a second construction sat on a datum frame written in numbers, and the replay read those numbers as symbols. In D (D10), that construction is a copy placed by a mate. Its pose comes out of the mate solve as numbers, and the replay keeps them as an opaque atom. So a row between two constructions now gets `Assert` where it used to get `OneConstruction`, and the residual's words say `placed: [World(..)]`. The rows are:
- `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`;
- `glue_on_zero_rows::a_flush_rest_glues_and_records_one_unproven_row`.

Both were restated to `Assert` in D.

The other direction moved as well: a lone construction no longer reads a frame, so its own rows close. A filleted disc's tangent joints now prove `Structural(MarginIdentity)` (`coincidence_door::a_filleted_disc_records_its_coaxial_supports_and_tangent_joints`), and `reshaped_rod`'s support row no longer reports.

**Done when** the replay reads a placed copy's pose as a function of its mate's variables, where the bundle closes in form. Examples are a plane mate with slide and spin values, or an axis mate. A cross-construction row whose residual is then a function of the variables alone gets the one-construction recourse back. A pose the solve only reaches numerically stays an atom.
