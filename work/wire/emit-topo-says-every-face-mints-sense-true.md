---
id: emit-topo-says-every-face-mints-sense-true
kind: issue
title: emit_topo::face_plane's doc calls its sense fold the identity because 'every face this build mints has sense: true', which no longer holds
status: closed
opened: 2026-09-29
priority: P3
cost: E
closed: 2026-10-01
pr: 3629
---

## Finding

`crates/editor-core/src/names/emit_topo.rs`, `carrier_plane`'s doc
(it was `face_plane`'s until PR 3542 moved the paragraph) says the sense fold is the identity and no name moves, because
"every face this build mints has `sense: true`". That has not held
since M5 S11: extrude states `false` on a concave arc wall, and
revolve on its inward walls (`crates/sweep/tests/m5_s11_concave_sense.rs`).
Since PR 3467 every construction states or derives the bit through
`FaceSurface` (D1).

**What would close it.** State what the fold does on a `false` face
(the doc's own N4 argument says it must negate), and pin a row where
a reversed planar face is named, or drop the "no name moves" claim.

Found by PR 3467's fix pass (TOPO), sweeping for present-tense prose
that states a default `sense`.

## Closed (2026-10-01, PR 3629)

`carrier_plane` and its doc are gone: no naming rule reads a plane
since edge pieces are named by their ends and split faces by the edges
they keep (`edge-pieces-are-named-by-their-ends`).
