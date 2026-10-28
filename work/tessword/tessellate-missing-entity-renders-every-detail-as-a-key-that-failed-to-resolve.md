---
id: tessellate-missing-entity-renders-every-detail-as-a-key-that-failed-to-resolve
kind: issue
title: mesh: TessellateError::MissingEntity says a referenced key failed to resolve for a degenerate polygon, a router defect, a memo miss and a broken cycle
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## Finding

`TessellateError::MissingEntity`'s `Display` (`crates/mesh/src/types.rs`) renders every `what` as "a key the body's own topology references failed to resolve ({what}) — ... the input is corrupt". Its raise sites cover other causes: a polygon of fewer than three points (`curved.rs` "degenerate curved boundary", `trimmed.rs` "degenerate trimmed boundary"), router defects that are kernel bugs (`walk.rs` "non-iso trim carrier reached the iso-rectangle walk (router defect)", `trimmed.rs` "trimmed router fired without a trim carrier"), a chart that does not exist (`curved.rs` "curved chart"), memo/cache misses (`memo.rs`, `tessellate.rs` "edge chords", `nurbs_cert.rs`), and `LoopEdgesError::Corrupt` forwarded from `topo::props::loop_edges` ("empty loop", "broken half-edge cycle" — walks that did not close).

Same shape as band's `BodyNotIntact` (fixed by PR 5021 with a typed `Breach`): give the variant a typed cause and render by it, or split the sites to the variants that state their cause. Found by that PR's §5 sweep.

## Moved to TESSWORD (tessellation refusal words, mesh docs and contract hygiene) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
