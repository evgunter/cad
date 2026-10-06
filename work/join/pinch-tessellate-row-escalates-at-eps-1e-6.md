---
id: pinch-tessellate-row-escalates-at-eps-1e-6
kind: issue
title: pinch_faces_tessellate's notch307 fib117 row escalates the boolean at ε = 1e-6: main is red on that row
status: open
opened: 2026-10-06
priority: P0
refs: [a-new-test-file-outside-the-eps-crates-never-runs-at-the-extra-eps-rows-before-merge, a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id]
---


`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all -E
'test(pinch_faces_tessellate)'` fails on bare `origin/main` at
b879a7cb: `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`
panics at its "the op builds no body" arm on the row
`notch307 fib117 edge psi=1.9 cp S`:

```
Err(Escalated { decision: Coincidence(Join, Moot), diag: Indeterminate {
  margin: MarginDiag(Value(-5.196042020649827e-6, None)), terminal_sliver: false,
  band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 },
  predicate: Some("bool_join_nearest") } })
```

The row's join margin (about −5.2e-6) sits inside the ε = 1e-6 row's
ambiguity band, so the boolean escalates instead of building. The test
arrived with the pinch-wedge work (PR 4074, item
`a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id`).
That PR did touch sweep. The row still never ran at 1e-6 before merge,
because of `.github/workflows/ci.yml`'s `eps_extra` rule (the "EXTRA EPS
ROWS" block of the `change filter` job): a crate gets the extra ε rows
only if it is one of `step-import geom-brep profile topo`, or if the diff
touches a `crates/<c>/…(probe|golden)` path. PR 4111 reached sweep's
1e-6 row only because it edits `*_probes.rs`. It saw the failure in run
37421965598, in the sweep `eps=1e-6` group. That gate gap is
`a-new-test-file-outside-the-eps-crates-never-runs-at-the-extra-eps-rows-before-merge`
(ciw).

Owed: either the row's fixture is moved clear of the band at every ε
row (a fixture whose margin is ε-relative), or the row names the
escalation it takes at 1e-6 as its expected outcome there.
`crates/sweep/tests/pinch_faces_tessellate.rs` is tcost/tint ground
by territory; the row is JOIN's.

## Note (FUSE, PR 3953, 2026-10-06)

Where the refusal comes from: `join::nearer` (`crates/topo/src/boolean/join.rs`),
which only orders two pairs whose partners `nearer_along` has already
chosen, by chord length in metres. Two chords within the band tie in
that order; the geometry itself is not coincident. A proposed kernel-side
fix, not measured: read an indeterminate `bool_join_nearest` as that
tie (keep `best`) instead of escalating, if no join order can matter
there. Same absolute-metre chord rank as
`a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale`.
