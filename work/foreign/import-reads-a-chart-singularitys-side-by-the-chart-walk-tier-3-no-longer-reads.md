---
id: import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-no-longer-reads
kind: issue
title: Import reads a chart singularity's side by walk_loop, which tier 3 no longer reads
status: open
opened: 2026-10-31
priority: P2
cost: M
refs: [validate-tier3-curved-boundary-containment, a-loop-the-chart-singularity-walk-cannot-certify-is-exempt]
---

## What

Tier 3's chart-singularity arm (`crates/topo/src/validate.rs`, check 6)
now places a sphere pole or cone apex on the face's boundary with
`geom_brep::chart_singularity::place_on_boundary` and reads its side,
off the boundary, through the face's point-in-face door
(`boolean::cycle_side` with every surface cycle kept), as DESIGN
`[topology.tiers]` says ("whether a point lies in a face has one
reading, the point-in-face doors'"). It no longer calls `locate`'s side
reading or `walk_loop`.

Step-import's normalization still does
(`crates/step-import/src/normalize/singularity.rs`: `scan` calls
`home::locate`, `holds` calls `home::holds`, `nappe` calls
`home::walk_loop`). It runs on `FaceSpec`s before any `Body` exists, so
it cannot call the topo doors, which read keys. So import and tier 3 read
one point's side by two instruments. They agree wherever both decide (the
step-import fixtures pass at tier 3 at ε 1e-6, the default and 1e-12), but
where the walk's certified area bound straddles a door can decide, and the
other way round: import can leave a pole the doors place in the face as
no vertex, and tier 3 then refuses the imported body.

## Owed

Import reads the side by the doors' reading: a key-free entry to them
over the spec's loops, or the normalization run after a body is built.
Then `walk_loop`, `holds`, `nappe`'s walk and `Sides` retire from
`geom_brep::chart_singularity`, which closes
`work/restcurve/a-loop-the-chart-singularity-walk-cannot-certify-is-exempt.md`
by retirement. Filed by RESTCURVE's region-arm lane.
