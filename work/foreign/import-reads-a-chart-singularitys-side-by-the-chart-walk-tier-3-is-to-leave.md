---
id: import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-is-to-leave
kind: issue
title: Import reads a chart singularity's side by walk_loop, which tier 3 is to leave for the doors
status: open
opened: 2026-10-31
priority: P2
cost: M
refs: [the-sphere-door-has-no-two-leg-path-so-a-pole-beside-a-slim-lune-reads-in-band, a-loop-the-chart-singularity-walk-cannot-certify-is-exempt]
---

## What

DESIGN `[topology.tiers]` gives a point's place in a face one reading,
the point-in-face doors'. Tier 3's chart-singularity arm reads a sphere
pole's or cone apex's side through them: the face's door at the point,
every surface cycle kept (`boolean::cycle_side`), after
`geom_brep::chart_singularity::place_on_boundary` places it off the
boundary
(`work/restcurve/the-sphere-door-has-no-two-leg-path-so-a-pole-beside-a-slim-lune-reads-in-band.md`,
which retired tier 3's use of `locate`'s side reading and of
`walk_loop`). Import shares only `place_on_boundary` with it now.

Step-import's normalization cannot follow tier 3 there
(`crates/step-import/src/normalize/singularity.rs`: `scan` calls
`home::locate`, `holds` calls `home::holds`, `nappe` calls
`home::walk_loop`). It runs on `FaceSpec`s before any `Body` exists, and
the topo doors read keys. So import and tier 3 read one point's side
by two instruments. Where the walk's certified area bound
straddles and a door decides, import can leave a pole the doors place in
the face as no vertex, and tier 3 then refuses the imported body.

## Owed

Import reads the side by the doors' reading: a key-free entry to them
over the spec's loops, or the normalization run after a body is built.
Then `walk_loop`, `holds`, `nappe`'s walk and `Sides` retire from
`geom_brep::chart_singularity`, which closes
`work/restcurve/a-loop-the-chart-singularity-walk-cannot-certify-is-exempt.md`
by retirement. It lands with or after the RESTCURVE row above, never
before it. Filed by RESTCURVE's region-arm lane.
