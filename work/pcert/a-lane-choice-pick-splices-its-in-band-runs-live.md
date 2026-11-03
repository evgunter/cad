---
id: a-lane-choice-pick-splices-its-in-band-runs-live
kind: issue
title: A lane-choice pick climbs Answering::First, so its in-band candidates' runs splice live into the verdict log a parameter drive classifies
status: open
opened: 2026-11-03
priority: P2
cost: M
---

Filed by PCERT unit E5 (PR 6484, review S6). E5 made the mint-side
picks' consumers read an in-band pick as "not shown" and take the lane
a definite reading takes, so the answer no longer rests on the in-band
reading (D3, `docs/DESIGN.md` `[verdicts.consumers]`: a lane choice
records nothing). The verdict log still records it as if it did.

## What

- `crates/geom-brep/src/edge_nurbs.rs` `first_within` (near :1374) climbs
  `crate::ladder::climb(Answering::First, …)`. Under `Answering::First`,
  a kept in-band reading ahead of the answer is spliced **live**
  (`crates/geom-brep/src/ladder.rs`, the `Answering` doc near :197–204:
  "the answer rests on it and its run is spliced live").
- `probe_pick` and `pick_arc_rim` read through `first_within`. So a
  candidate in band (`pcurve_iso_arc_row_direction`,
  `pcurve_iso_wall_seam`, `_line_rim`, `_seam_column`,
  `pcurve_shared_side`) leaves a live escalation run in the log. This is
  true even where a later candidate answers, and even where the consumer
  goes on to the general lane and the run decides nothing.
- `editor-core`'s parameter drive reads that log (`drive.rs`
  `classify` / `classify_replay`, near :1529), so a leaf whose mint
  passed a pick in band is classified as resting on an undecided
  reading. That leaf in fact took the general lane.
- The class is shared with `pcurve_cache`'s `derive_class`
  (`crates/geom-brep/src/pcurve_cache.rs` near :9618), whose class
  readings are the analytic charts' lane choices.

`first_within`'s other caller, `topo::pcurves::near_pole_gap_closes`
(`pcurve_loop_pole_gap`), is a joint's branch read, which is
structural. There, live is right.

## Fix

A lane-choice read splices its in-band runs as superseded, or does not
log them at all: the answer is the lane, not the reading. That needs a
way for `climb`'s caller to say "a lane choice", distinct from
`Answering::First` (which candidate answers chooses the answer) and
`Answering::Any`. One row should drive a document through a mint whose
pick reads in band and assert the leaf is not classified undecided.
`pcurve_loop_pole_gap` keeps its live splice.
