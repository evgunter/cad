---
id: SHELL-3
kind: unit
title: the clearance engine's body-level half moves into topo — one engine, two consumers
status: closed
opened: 2026-09-04
refs: [shell-curved-clearance-consumer, 1725, 1737]
priority: P0
cost: H
branch: clear/shell-3-engine-into-topo
closed: 2026-10-28
pr: 5909
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

## Shape (CLEAR orchestrator, 2026-10-27)

Ev's ruling on #1737 ("i think B makes sense?") placed the engine. It did
not fix the detail above, which was agent prose. Two corrections, both
from the symbolic-tier designers (`symbolic-tier-and-clearance-engine`,
`## Decided`):

- **There is no `interval` feature.** RING-4 (#3154) removed it, and
  certified code compiles in every build. The engine moves into
  `topo::clearance` unconditionally.
- **`verify_witness` does not move.** It rebuilds the `Doc` at `f64` at
  the leaf's midpoint. It stays in `editor-core` with `clearance_over`,
  `LeafFold`, `facet_restrict` and the selection resolution. The `topo`
  walk takes a verifier or returns an unverified candidate: the spec
  picks one, and SHELL-4 needs its answer for a body with no document.

What moves, in shape:
- `Side { body: &Body<T>, faces }`, carrying no node address and no frame.
  INTENT E's `Framed::read_in` reads both operands into one frame first.
- `min_separation` and the bound walk, generic over `T: Decide +
  CertifiedBounds`.
- The door reads through `Interval::from_certified`, as `QuadLane` does.
- `cell_region` and `station_region` become generic the same way.
- `ClearanceLane<T>` is a door value on `AtRestPolicy`. At this step it
  answers `Some` only at `Interval`; the symbolic row turns `Sym` on.

At `Interval` the M10-5/M10-6 suites and goldens are the differential.
Dispatch waits for INTENT's E (#5811), which edits these sites.

## Closed (2026-10-28, #5909)

- **What moved.** The engine's body-level half is `topo::clearance`, in
  every build. Its door is `Side<T>`, generic over
  `Decide + CertifiedBounds`, with interval arithmetic inside.
  `cell_region` and `station_region` are generic too.
- **What stayed.** Witness verification stays in `editor-core`, reached
  through a verifier callback. `ClearanceRefusal` is split:
  `topo::clearance::EngineRefusal` holds the engine's arms, wrapped by
  `editor-core`'s `ClearanceRefusal`.
- **No behaviour change.** The M10-5/M10-6 goldens and
  `m10_6_certifying_keys.txt` held.
- **Review.** A single FULL review read APPROVE-WITH-FIXES, and its fix
  pass was read and accepted by the orchestrator. Its class findings
  are on CLEAR's and CLEARTAIL's slates.
