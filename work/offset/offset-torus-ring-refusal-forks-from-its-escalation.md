---
id: offset-torus-ring-refusal-forks-from-its-escalation
kind: issue
title: offset: offset_surface's torus-ring refusal carries a stage label and no recourse, and its in-band sibling a generic escalation
status: open
opened: 2026-09-30
---


(TOPO, the §5 second pass of PR 3506: the other doors that read the
ring-torus convention, checked for D4 ¶1 (iv).)

## What

`geom_brep::offset::offset_surface`'s torus arm
(`crates/geom-brep/src/offset.rs`, near :404) decides
`geom::ring_torus(R, r + d)` on the realized minor radius:

- decided `Zero | Negative`: `OffsetError::TorusRing`, rendered
  "offset_surface: the offset torus leaves the ring convention R > r —
  … so the mint would self-intersect": a stage label, no recourse, and
  no tolerance on its Zero half;
- in band: `OffsetError::Escalated`, rendered "offset_surface escalated:
  {source}", the whole `Indeterminate` (`COINCIDENCE_RECOURSE`), with the
  same label.

The decision is the ring convention's, whose one lever and pass set now
live on `geom_brep::TorusConvention::Ring.sized()` (PR 3506, in this
crate: `crates/geom-brep/src/torus_convention.rs`); here the
lever the user holds is the offset distance, as the offset meters'
(PR 3347) are.

## Repair shape

End both arms of the realized ring from one `SizedDecision`, the
convention's `TorusConvention::Ring` (its subject and `refused()` facts)
with the offset's lever in place of the radii's ("use a smaller offset distance, or offset to the other
side"), valued on the band-decided arms (`decide_reported`), and drop the
`offset_surface` label.
