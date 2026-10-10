---
id: mfkrh-leaves-an-orphan-surface-under-the-per-op-scalpel
kind: issue
title: topo: mfkrh leaves an orphan surface, so ten sweep rows panic under topo/per-op-postcondition
status: open
opened: 2026-10-09
priority: P2
cost: M
---



(Filed by the ENCL orchestrator from the full review of PR 4433. It is pre-existing: it reproduces on `origin/main` `9409cb5e43`, and PR 4433 does not cause it.)

## What

With `topo`'s `per-op-postcondition` feature on (the scalpel), ten sweep rows panic in about 0.2 s:

```
crates/topo/src/surgery.rs:357:9: mfkrh postcondition: result is not tier-1 valid (kernel bug)
  left: Err([OrphanGeometry { geometry: Surface(SurfaceKey(1v3)) }])
```

The rows:
- `a_plane_across_a_one_face_wall::*` (7);
- `germ_coplanar_conic::every_op_refuses_or_answers_its_closed_form`;
- `one_segment_loop::a_boolean_on_an_extruded_seam_wall_builds_along_and_across_it`;
- `run_walls_built::fully_revolved_arc_runs_build_one_wall_each`.

Reproduce: `cargo nextest run -p topo -p sweep -p editor-core --all-features -E 'test(a_plane_across_a_one_face_wall) | test(germ_coplanar_conic) | test(one_segment_loop) | test(run_walls_built)'`. With `-p sweep --all-features` alone the rows pass, because feature unification does not turn on topo's scalpel. It is deterministic, not load.

So `mfkrh` leaves a surface no face references, inside a surgery scope, on the paths these sweep rows drive. It is a tier-1 invariant break that the per-op scalpel catches and the end-of-op check does not.

## Related

- `work/ciw/no-ci-row-runs-topo-under-the-per-op-postcondition-scalpel.md`: no CI row runs the scalpel, which is why this is invisible to CI.
- The closed `tests-fail-under-the-per-op-postcondition-feature` covered different rows.

## Repair shape

Find the `mfkrh` call on these paths that ends with the face's surface unreferenced. Either the surface should be removed in the same op, or the op's precondition should forbid the state. Then the rows pass under the scalpel; the ciw row would keep them there.
