---
id: SHELL-3
kind: unit
title: the clearance engine's body-level half moves into topo behind interval — one engine, two consumers
status: open
opened: 2026-09-04
refs: [shell-curved-clearance-consumer, M10-7, 1737]
---


Ruled B on `[ev]` #1737 (Ev, 2026-09-04). The inner subdivision of
`editor_core::clearance` — `window_of`, `Cell`, `Sweep`, `split`,
`separation`, `verify_witness`, `min_separation` and its selection and
config types, everything that reads only `Body<Interval>`, `bvh` and
`geom-core` — moves into `topo` behind the `interval` feature;
`editor-core` keeps the leaf/param-box outer half (`clearance_over`,
`LeafFold`, `facet_restrict`, the `Doc`/`Selection` resolution) and
calls down. No behaviour change: the M10-5/M10-6 suites and goldens
are the differential. Joint with M10 (the file is M10's; M10-7's PR
#1725 does not touch it, measured); spec to follow, dispatched after
SHELL-1 and SHELL-2 return.
