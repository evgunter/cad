---
id: a-lens-candidate-dropped-in-band-refuses-unreadable
kind: issue
title: second_order::pair drops a candidate whose rays read in band, and the point refuses Unreadable, not InBand
status: open
opened: 2026-11-12
priority: P3
cost: E
---


Found by the remainder-refuses lane (PR 6854), sweeping for band
readings that end in `Unreadable`. Priority P3: only the refusal's
stated cause, and so its recourse, is wrong. The point refuses either
way.

## What

`second_order::pair` (`crates/topo/src/link/second_order.rs:622`) takes
`[i, j]` as a candidate only where `j`'s rays read on `i`'s by
`super::on_ray` (`:643`). A ray in band of the other's reads "not on
it", so the pair is dropped as no candidate (`None`), rather than
refused in band.

The two sheets then stay on each other. `one_material` notes them
coincident (`crates/topo/src/link.rs:5013`) and refuses
`Curved(Unreadable)` (`:5023`), telling the user the shape is not read
yet when the true recourse is the tolerance.

## The fix

Make `pair`'s candidacy three-valued. Where `on_ray` or `wedge_side`
reads in band, refuse `TouchUnread::InBand`, as `kind` now does for its
own signs. Pin it with a lens whose second ray is turned off the first
by a hair between the band's zero and its escalation.
