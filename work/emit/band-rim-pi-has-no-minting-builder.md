---
id: band-rim-pi-has-no-minting-builder
kind: issue
title: the minting builders spell band, band_pi and band_rim but not band_rim_pi, so a whole rim on an axis-touching revolve is hand-spelled
status: open
opened: 2026-10-02
priority: P3
cost: E
---


## Finding

A full revolve of a profile that touches the axis mints each latitude
rim as two half-arcs, `RoleSeg::BandRim` (`[0, π)`) and
`RoleSeg::BandRimPi` (`[π, 2π)`). A fillet over the rim must name both:
one arc alone ends at a seam vertex and refuses
`UnsupportedCorner { corner: SeamVertex }` (measured on the teapot lid,
each of its three rims, at radii 2/256, 1/256 and 1/512).

The minting direction of the vocabulary (`editor-core/src/names/role.rs`,
re-exported through `pncad::select` — its module doc lists `band`,
`band_pi`, `band_rim`, `meridian_vertex`, `carried`) has `band_pi` for
the face twin but no `band_rim_pi` for the edge twin. A consumer writing
the natural selection therefore hand-spells
`StableName { kind: EntityKind::Edge, node, path: vec![RoleSeg::BandRimPi(v)] }`,
which is exactly the field-by-field spelling the builders exist to
retire (role.rs, `band`'s doc: "the field a hand-spelled name gets
wrong silently until emission refuses it").

Met at `demos/tour/src/teapot.rs`, `rim_arcs` (show
`teapot-lid-unbored`). The same hand spelling already lives in
`crates/editor-core/tests/blend5_rim_support.rs` (the base-rim
selection).

## Fix shape

A `band_rim_pi(node, vertex) -> StableName` beside `band_rim`, exported
through `pncad::select` and the python `select` module. Perhaps also a
`band_rim_whole(node, vertex) -> Vec<StableName>` answering one name on
an annular profile and two on an axis-touching one, though that needs
the profile's shape and so is a door rather than a builder.

The viewer half of the same gap (a click picks half a rim) is
`work/vseam/a-picked-half-rim-blends-half-and-refuses.md`.
