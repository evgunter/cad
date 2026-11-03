---
id: face-to-surface-walks-outside-topo-predate-body-face-surface
kind: issue
title: Crates outside topo walk a face to its surface by hand, and mesh and sweep keep private twins
status: open
opened: 2026-11-02
priority: P2
cost: M
---

Found by the sweep of `boolean-walks-face-to-surface-inline-at-two-dozen-sites`,
which gave `topo` one public walk from a face key to its surface,
`Body::face_surface` (`crates/topo/src/body.rs`): `None` exactly where
the face key is stale, a panic where a live face's surface link does
not resolve (D2 row 4).

## What

Outside `crates/topo`, consumers still spell the walk by hand —
`body.get_surface(body.get_face(f).unwrap().surface)`,
`get_face(f).and_then(|d| body.get_surface(d.surface))`, or a record
read then a key lookup — each with its own reading of a miss. At the
merge base of that PR: 93 `get_surface(` calls in `src/` of other crates
and roots (`editor-core`, `mesh`, `step-export`, `step-import`,
`sweep`, `demos/tour`, `tools/tess-meter`), and 843 in their tests and
examples (400 files). Not every one is a face walk: some resolve a
surface key that is not a face's (an edge description's chart).

Two private twins of the walk, each with its own refusal:

- `crates/mesh/src/tessellate.rs` `face_surface`;
- `crates/sweep/src/swept.rs` `face_surface_key` (the key half of it).

## Direction

Repoint each face walk at `Body::face_surface` (or, holding the face
record, read its fields beside it), deciding at each site whether a
stale face key is a desync or an honest no; delete the twins. The
tests and demos are mechanical; the `src/` sites are each a decision.

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.
