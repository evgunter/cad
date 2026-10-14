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
for a null edge with no certified carrier or a lone-vertex cycle; and
`Degenerate` — "{at}'s stored carrier is degenerate under the run's band"
(`degenerate`), for the face-box read in `reach.rs` `band_reach` that a
cylinder with no decided axis or radius length refuses. Every site that
met a null edge or a lone-vertex cycle and rendered `UnsupportedGeometry`
with the shape recourse (`surgery.rs`, `open/end_face.rs`, `battery.rs`
`screened_loop`) now refuses `BodyNotIntact`/`Scaffolding` with no
recourse; the reads that folded an unresolved curve row or a null edge
into one `None` (`stored_piece`, `rim_carrier`, `split_param_in_span`,
`edge_midpoint`, `reach.rs` `station`, and the battery's four
`ChainNotConnected` folds, now `battery::link_carrier`) refuse each cause
as itself. Rows: `body_not_intact_renders_each_breach_as_its_own_cause`
pins the texts; `scaffolding_and_a_degenerate_carrier_refuse_as_their_own_breach`
reaches one real refusal per new kind. The sweep's out-of-fence residue
is `blend-two-geometry-refusals-meet-no-shape`.
