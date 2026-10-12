---
id: a-curved-notch-at-a-corner-pinch-refuses-join-desync-on-a-section-vertexs-null-edge-copies
kind: issue
title: A curved notch at a corner pinch refuses JoinDesync: a section vertex's null-edge copies have not exactly one kept end (15 lines, 5 poses)
status: closed
opened: 2026-10-11
priority: P2
cost: M
refs: [4249]
branch: flush/pinch-residue
pr: 4777
closed: 2026-10-12
---



## What

Found re-measuring PR 4249 review r1's curved-notch probe
(`a-curved-face-at-a-shared-pinch-vertex-builds-bodies-the-census-and-the-reuse-check-refuse`).
The probe is now `sweep` `curved_notch_pinch::curved_notch_pinch`
(ignored, release): `notch343` with its face `v → (2, 1.15)` bowed
into an arc (bulge tan 2.5°), against the two-cube corner pinch of
`pinch_runs_battery`, every op in both orders, plain booleans.

On main `6e3985cb7`, 15 of its 1 512 lines refuse
`JoinDesync { what: "a section vertex's null-edge copies have not
exactly one kept end" }`, raised in `crates/topo/src/boolean/finish.rs`
`discarded` (the several-copies arm: "a vertex several runs of one
pierce cut"):

| pose (i j k) | lines |
|---|---|
| 0 3 2 | ab U, ab S, ba U, ba S |
| 1 6 0 | ab U, ab S, ba U, ba S |
| 3 1 0 | ab U, ab S, ba U, ba S |
| 6 1 0 | ab I, ba U, ba S |

On main `875e049a` the same probe gave 11 of these; pose `3 1 0` was
`ClassificationInvariant` there, and PR 4249's hang moved it on to
this refusal. Replay one with
`CURVED_POSE="0 3 2 ab U" cargo test -p sweep --release --test all
curved_notch_pinch_one -- --ignored --nocapture`.

## Open

Not root-caused. Whether the twin edge joins no pair of the vertex's
copies, or more than one, was not read. The planar batteries'
pinches do not reach this arm; the curved wall at `v` does.

## Not the pinch weld's ground (2026-10-11, branch `flush/pinch-cluster`)

The pinch cluster priced this row against its fix and left it. That
fix mints the pierces of one face at one point as one ring
(`vtxfac::mint_ring`) and retires `finish::weld_pinches`. The
`curved_notch_pinch` probe is byte-identical across it (1 513 lines,
release; the base side is the branch built with a switch that restores
the per-pierce rings and the weld), and so are its 15 `JoinDesync`
lines. This probe's pinch is
the two cubes' vertex-vertex pinch at the notch's corner. It has no
coincident pierces, and `discarded`'s several-copies arm reads the
null-edge copies the vertex-vertex insertion leaves, which the ring
does not reach.

## Measured (2026-10-12, main `300f7cefe`)

Reproduces: 15 of `curved_notch_pinch`'s lines refuse `JoinDesync`,
the four poses of the table above (the row's "5 poses" counts one
twice).

Read at `0 3 2 ab U`, in `finish::discarded`'s `kept_ends`. The A
section face across the failing stretch runs
`17 → 31 → 19 → 11 → 20 → 11 → 18`: it passes the pinch vertex `11`
(at `v`) **twice**, once by a line `11 → 20` and back by the bowed
wall's ellipse `20 → 11`. The stretch is the ellipse. Its twin runs the
same polygon the other way on copies, `… 26 → 39 → 28 → 37 → 27 …`:
`39 → 28` is the ellipse's copy and `28 → 37` the line's. `kept_ends`
took the kept ends as the copies one twin edge joins, and two twin
edges join a copy of `11` to `20`'s one copy. The read was ambiguous
because it named an edge by its two ends on a face that passes one
vertex twice; it was not a wrong copy set.

## Fix (branch `flush/pinch-residue`)

`discard_row` hands `kept_ends` the half-edge across the stretch, and
`finish::twin_copy` reads its copy off its place: the twin's cycle is
the section face's mirrored, each vertex a null-edge copy of the
other's, so the mirror that aligns the two cycles maps each half-edge
to its own copy. Where no single mirror aligns them it refuses as
before.

Release battery diff, base `300f7cefe` against the fix: the 15 lines
build with the oracle's volume, tier 2, the certificate, one vertex per
cone and the reuse union. Eight are clean. Seven carry the signatures
that already have rows: `t3p=CensusUndecidable` on five
(`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`)
and `mesh=Triangulation(Plane rings=0)` at δ 0.05 on four
(`work/tess/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md`),
both with the evidence added there. Every other line of every pinch
battery is byte-identical. Pinned by two poses added to
`curved_notch_pinch::a_curved_notch_at_a_corner_pinch_builds_bodies_the_census_cannot_read_yet`
and by `finish::twin_copy_rows`.

`3 1 0 ab S` meshes at δ 0.05 and panics the mesher at 0.02 on a
`debug_assert!` its comment calls vacuous: filed as
`work/tess/a-pinched-curved-wall-reaches-the-trimmed-meshs-vacuous-one-column-assert.md`.

## Fix pass (PR 4777's dual review)

- `twin_copy` aligns loop by loop: a section face with a ring reads each
  half-edge's copy on the one twin loop that mirrors its own loop
  (`finish::section_loops`), where reading every loop as one cycle could
  find a unique, wrong mirror. `twin_copy_rows` now builds each twin by
  running each loop backwards on fresh copies, so the expected copy
  comes from the build, with a ring row.
- The discard still hands the copy on as two end vertices: 9 of the 15
  bodies have 4 bordered stretches each on an end pair more than one
  result edge joins. Filed on REFS's slate as
  `a-discard-names-a-bordered-stretch-by-its-two-end-vertices`, pinned at
  `3 1 0 ba S`.
- `CertificateExceeded` at δ 0.005 and 0.001 on 11 of the 15 bodies is
  the probe's finer-δ sweep, and poses that build on the base show it
  too (`work/tess/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md`).
