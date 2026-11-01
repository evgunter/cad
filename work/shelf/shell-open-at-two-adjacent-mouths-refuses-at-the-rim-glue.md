---
id: shell-open-at-two-adjacent-mouths-refuses-at-the-rim-glue
kind: issue
title: shell_open at two designated faces that share an edge refuses OpenFaceRimNotExpressible, on a box as on the sphere lune
status: open
opened: 2026-10-06
priority: P3
---


## What

`shell_open` with two designated faces that share an edge refuses at
the rim glue with `ShellError::OpenFaceRimNotExpressible`. Each
cavity counterpart's boundary meets its designated face's boundary
along the shared edge, instead of sitting strictly inside it. The
glue's only output shape is one region per face, with an outer loop
and disjoint rings (`crates/topo/src/shell.rs`, the variant's docs),
so it refuses rather than mint a ring that stands on its own outer
loop.

It is the glue's limit, not a lift's. Two cases reach it:

- **A box opened at two adjacent faces.** This is the planar
  instance.
- **The quarter-turn sphere lune opened at both meridian caps.**
  PR 4151 made both rim lifts succeed here; the second one moves the
  rim corner the two cavity caps met at onto the axis. The refusal
  then comes from the glue.
  `sweep::torax_axial::torax_the_sphere_lune_lifts_its_rims_onto_the_caps`
  pins it (`expect_err`, matched on the variant).

## Owed

A rim surgery whose output can be a mouth that runs across a shared
edge into its neighbour. That means one region spanning two faces'
rims, or a face whose rim ring meets its outer loop along a run. The
lune row flips when it lands.
