---
id: offset-axial-re-authored-arc-sweep-is-an-endpoint-atan2
kind: issue
title: offset_axial re-authors a moved sketch arc's sweep as atan2 of its endpoints, folding past a half turn
status: open
opened: 2026-10-02
---


Found in PATHS 5b's hit list (#3774, `store-constructed-carriers`).

`crates/topo/src/offset_axial.rs`, in the declaring pushforward's
re-authoring of a `PlacedSegment` arc (the `SketchSegment::Arc` arm), sets
`sweep: u.perp_dot(v).atan2(u.dot(v))` with `u = a − centre`,
`v = b − centre`.

Two problems follow from that spelling:
- `atan2` returns (−π, π], so a source arc turning more than a half turn
  is re-authored as its complement the other way round. Nothing reads the
  source's own sweep or turn.
- D1 says downstream re-inspection of arc geometry uses the stored carrier
  data, never endpoint `atan2`. The sweep here is re-derived from the moved
  endpoints rather than carried from the source segment. A rigid move keeps
  the signed sweep, so `arc.sweep` of the source is available as it is.

The `radius` is the moved circle's, the centre is its centre flattened,
and the endpoints are moved. The fix that suggests itself is to carry the
source arc's `sweep` (negated only if the move reverses orientation).
Whether the move can reverse orientation is the owner's call.
