---
id: the-boolean-reads-the-other-operand-through-one-cone
kind: unit
title: The boolean reads each own edge at a point through one Cone of the other operand gathered across keys, and its cone reader and vtxfac's layering go
status: dispatched
opened: 2026-11-01
priority: P1
cost: H
parent: the-booleans-cone-reader-is-a-second-link
branch: oneread/boolean-one-cone
---


Items 3 and 4 of `the-booleans-cone-reader-is-a-second-link`'s `## Decided`, which is the spec: the sign-free gathering builder of a `Cone` of the other operand at a point across keys (partner vertices, a touched face, an edge the vertex rests on), with curved faces as tangent sectors; the orbit walk (`sector_face`/`sector_shape`) moved to a home neutral between `boolean` and `link`; the boolean reading each own edge through it; the deletions; `cone_fuzz` moved under `link`; `census_touch_*` K names renamed `link_*`.

Verification owed: a before/after diff of every published `EdgePieceClass` and `UnreadPartner` row across the topo, editor-core and sweep suites, each changed row explained. The bore-rim `NoBisector` poses (`crates/sweep/tests/full_turn_bore_mate.rs`, `crates/editor-core/tests/a_mate_on_a_full_turn_bore_names.rs`) should flip from unread to classed, asserted against hand-derived classes. Absorbs `cone-side-is-a-second-reader-of-a-directions-side-of-a-polygon-cone` and `in-sector-is-a-second-spelling-of-within`.

Review tier: dual (it moves every published vertex-vertex row; logic broad and hard to check by reading).
