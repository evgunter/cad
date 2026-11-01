---
id: blend-refusals-whose-sentence-does-not-fit-their-site
kind: issue
title: blend: a face that cannot be boxed refuses BodyNotIntact, and a missing certified carrier refuses UnsupportedGeometry with a shape recourse
status: closed
opened: 2026-10-13
priority: P3
cost: E
pr: 5116
branch: band/refusal-sentences-fit-their-site
closed: 2026-10-14
---


## Finding

Two blend sites whose refusal sentence does not fit what they met, left by PR 5021's classification:

- `reach.rs`, after `FaceBoxes::of(body, band)`: the box builder refuses a cylinder carrier whose axis has no decided length, and the site renders it `BodyNotIntact`/`Breach::Disagrees` ("a face's certified box"). A degenerate stored surface is neither a key, a walk nor a structural disagreement; it is an undecidable geometry read, closer to `UnsupportedGeometry` or a certification refusal.
- `UnsupportedGeometry` appends `FILLET3_GEOMETRY_RECOURSE` (plane, cylinder, cone, sphere supports with line and circle edges) to sites that met an edge with NO certified carrier (`surgery.rs` `stored_piece`/`metered_piece` reads, `open/end_face.rs` "an end face's rim carries no certified carrier"): the recourse does not address a missing certificate.

## Closed

`Breach` carries two more kinds (`crates/sweep/src/blend/mod.rs`), each
with its own constructor in `surgery.rs`: `Scaffolding` — "{at} is
construction scaffolding a finished body does not hold" (`scaffolding`),
for a null edge with no certified carrier or a lone-vertex cycle, ending
in the recourse the at-rest gate gives the same state
(`topo::Unfinished::SCAFFOLDING_RECOURSE`); and `Degenerate` — "{at}'s
stored carrier is degenerate under the run's band" (`degenerate`), for the
face-box read in `reach.rs` `band_reach`. Every site that met a null edge
or a lone-vertex cycle and rendered `UnsupportedGeometry` with the shape
recourse (`surgery.rs`, `open/end_face.rs`, `battery.rs` `screened_loop`)
now refuses `BodyNotIntact`/`Scaffolding`. Every read of an edge's
carrier goes through `surgery::certified_curve`, which refuses an edge or
curve row that does not resolve as `unresolved` and a null edge as
`scaffolding`, where reads used to fold those into one `None` or into
`ChainNotConnected`. Rows: `body_not_intact_renders_each_breach_as_its_own_cause`
pins the texts; `the_raw_battery_refuses_a_null_strut_as_scaffolding_with_the_doors_advice`
reaches `Scaffolding` through the public `run_battery_for`, and
`a_degenerate_carrier_refuses_as_its_own_breach` reaches `Degenerate`
crate-internally (no door admits its body). The residue is
`blend-two-geometry-refusals-meet-no-shape` and
`blend-reach-may-refuse-a-valid-thin-cylinder-as-degenerate`.
