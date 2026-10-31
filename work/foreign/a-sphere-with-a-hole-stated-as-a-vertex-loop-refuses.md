---
id: a-sphere-with-a-hole-stated-as-a-vertex-loop-refuses
kind: issue
title: Open CASCADE's sphere with a hole, a VERTEX_LOOP beside the cut circle, refuses at import
status: open
opened: 2026-10-29
priority: P3
cost: M
---


Found by `import-normalizes-the-rim-only-cap`'s lane (FOREIGN implementer, 2026-10-29).

Open CASCADE (7.8.1, written through OCP with FreeCAD's writer settings:
`crates/step-import/tests/fixtures/occ/gen_occ.py`) states a R = 10 mm
ball with the cap `y > 8` cut away — a cut that misses OCC's seam — as
ONE sphere `ADVANCED_FACE` with two bounds: a `VERTEX_LOOP` (OCC's whole
sphere, its seam and pole edges dropped) and the cut circle. Import
refuses it at `entities::edge_free_face`: "an ADVANCED_FACE mixing a
VERTEX_LOOP with other bounds — a vertex loop bounds a CLOSED face,
which has no room for rings" (`side_cut_ring.step`, pinned `Refused` in
`tier_gate.rs`'s corpus table).

The face is the sphere less a disc, both poles inside it. The pieces to
import it exist: the edge-free sphere mints the pole form's two
meridians from `geom::pole_form`, and the chart-singularity pass
(`normalize/singularity.rs`) cuts a face holding both poles through a
boundary vertex's azimuth. A re-mint would drop the vertex loop, keep
the cut circle as the face's one loop, and let the singularity pass
make both poles vertices. The same cut through OCC's seam
(`side_cut.step`) already imports, as two strut tips.

Interop on an everyday shape (a ball with a hole, from FreeCAD): P3 by
the band definitions, though it is the commonest way this state arrives.
