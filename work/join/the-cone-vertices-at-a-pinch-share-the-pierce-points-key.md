---
id: the-cone-vertices-at-a-pinch-share-the-pierce-points-key
kind: issue
title: A pinch's cone vertices that descend from one pierce point sit on two point keys: tier 3′ refuses VertexVertex, and the output-stage join kills one
status: dispatched
opened: 2026-10-06
priority: P1
cost: M
branch: join/pinch-cones-share-a-point-key
refs: [a-pinch-no-kept-face-can-cross-refuses, the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone]
---

## What

Found re-merging PR 4139 (`join/pinch-one-vertex-per-cone-build`) onto
main after #4140's output-stage join. A pinch is one vertex per cone,
"several vertices on one point key" (Ev, PR 4057). Where one cone's
vertex descends from one operand's vertex and the other's from the other
operand's pierce copy, the two sit at one position on two point keys:
- tier 3′ refuses `UndeclaredContact { VertexVertex }` at the pinch (the
  census's same-point rung reads keys);
- `boolean::edge_join`'s guard, which leaves a vertex that shares its
  point key with another unjoined, does not see the pinch, so the join
  kills a cone vertex that is valence 2 between collinear edges. The
  joined edge then runs through the other cone's vertex.

## Measured

PR 4139's review r1's set `dbl` (two `Ltop`/`asym` corners touching only
at `v`, against a cube whose near face holds `v`), main `52cbc833` vs
PR head `aa349909`, release. On PR 4139's head before the re-merge
(`9d5f2a2f`) each line held 2 vertices for 2 cones and meshed; all were
`BAD` on tier 3′ only (`VertexVertex`) there and on main.

| line | now (2 cones) |
|---|---|
| `dbl Ltop asym seed=2296 fib21 xy U` | 1 vertex; mesh refuses `Triangulation` |
| `dbl Ltop asym seed=2296 fib21 yx U` | 1 vertex; mesh refuses `Triangulation` |
| `dbl Ltop asym seed=2296 fib3 xy U` | 1 vertex; the mesher panics |
| `dbl Ltop asym seed=2296 fib3 yx U` | 1 vertex; the mesher panics |
| `dbl Ltop asym seed=2296 fib8 xy U` | 1 vertex; meshes |
| `dbl Ltop asym seed=2296 fib8 yx U` | 1 vertex; meshes |
| `dbl Ltop asym seed=2959 fib3 xy U` | 1 vertex; meshes |
| `dbl asym asym seed=15 fib6 xy U` | 1 vertex; meshes |
| `dbl asym asym seed=225 fib6 xy U` | 1 vertex; meshes |

Volumes are the oracle's on every line. Repro: `R1_PICK="dbl Ltop asym
seed=2296 fib21" cargo run -p sweep --release --example r1_4139_probes
dbl` (branch `join/pinch-one-vertex-per-cone-review-r1`).

## The shape to give

Every cone vertex at a pinch that descends from one pierce point sits on
that point's one key: after the zips, the vertices the pierce's
correspondence reaches take its key, by structure, not by comparing
positions. Tier 3′'s same-point rung then clears the pinch, and the
`edge_join` key guard covers these lines with no change to the join (a
join decided by structure alone; a position comparison there was ruled
out). Rows: the nine lines above, each with one vertex per cone on one
key, `SOUND`, meshing.

## Traced

Both keys come from the pinched operand, not from the cube's pierce
copy. On every line the operand (an earlier union of two corners
touching at `v`) already held its pinch as two vertices on two keys;
the cube's copies of `v` sit on a third key, which the zips fuse away.
The seam correspondence (`SeamCorrespondence`, the null-pair records)
pairs each operand vertex at `v` with the cube's copy there, so the
two operand keys are linked through the cube's key: one class, by the
records alone. No position is read to find it.

## Built (branch `join/pinch-cones-share-a-point-key`)

- `zip::point_classes` reads the correspondence before the cone split
  and unions the point keys each pair names; the classes of two or
  more keys are the points the seams say are one.
- `zip::share_points`, after the zips, rebinds the live vertices of a
  class that still span more than one key onto the class's smallest
  key, through the new `Body::share_point`, which writes no
  coordinate. A class whose survivors already share a key is left
  alone, so a body with no pinch is untouched (the editor-core digests
  hold).
- Row: `join_pierce_runs_sweep::a_pinchs_cones_share_one_point_key`,
  the six distinct poses of the nine lines, each union in both orders:
  `SOUND`, one vertex per cone on one key, meshing. Red with the
  rebinding removed.

## Measured (main `3e9d1a96` vs head, release)

- The nine lines: `BAD` → `SOUND`, 2 vertices for 2 cones, all meshing
  (main: 1 vertex; 2 refuse `Triangulation`, 2 panic the mesher).
- r2's pinched-operand battery: 75 lines `BAD` → `SOUND`. Every
  tier-3′ `VertexVertex` at the pinch clears (236 on main, 0 on
  head). The 120 lines still `BAD` fail only on
  `StaleContactDeclaration { VertexOnFace }` (176 records), which main
  reports on the same lines, line for line: FUSE's
  `a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms`.
- Everything else is byte-identical: r1's other families (`nt`,
  `stair3`, `islnotch`, `cyl`, `multi`, `pair`, `x4`), the rest of
  `dbl`, r2's tri-cone battery, and the pierce, pinch, corner-pair,
  both reflex and `rc_wide` ×84 batteries. No line goes `SOUND` →
  refusal.

## Fix pass (FULL review, PR 4207)

- Docs carry the traced cause: the row's doc, `share_points`' doc
  (which no longer names a ring-copy source no line has), and `zip.rs`'s
  module header, which now names `share_points`.
- One union-find (`zip::Roots`) serves `point_classes` and
  `split_cones`.
- `share_point`'s premise is structural, and its doc says so: its one
  caller passes keys the seam pairs tie, and the zips certify those
  pairs coincident. A runtime check is not expressible over `T: Real`:
  `Real` has no equality, a `Bounds` read is a ratified seam, and the
  bit channel (`geom_core::bit_identity`) is fenced. The premise is
  pinned in test builds instead (`sweep-testing`,
  `topo::take_shared_points`). The pinch rows assert that every class
  rebound held one point, bit for bit. A rebind that takes in a foreign
  key goes red there. `share_point` and `move_vertices` name each other
  as sibling doors.
- `point_key_finding` refuses a point with fewer than two vertices.
  `shared_point_finding` is the reading for a point that may hold one.
- Residue filed: `a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys`
  (`touch`, `dbl3`; parked on D10).
- No row for a vertex whose second correspondent alone links a class.
  `dbl` and the pinch battery hold 489 such vertices, but a
  first-correspondent-only `point_classes` leaves every line
  byte-identical: the zips already leave those vertices on one key.

Re-measured after merging main, main `7e4e2c96` vs head `02f7a182`,
release:
- `dbl`: 9 lines `BAD` → `SOUND`; the rest of the 4 320 byte-identical.
- r2's pinched-operand battery: 75 `BAD` → `SOUND`. The review's exact
  replay (`r2p`): the same 75, plus 73 still-`BAD` lines (stale
  declarations) whose pinch goes from two keys to one.
- `two`: 1 line `BAD` → `SOUND` of 1 728.
- Byte-identical: `dbl3` (216), `touch` (84), the pinch (3 029) and
  pierce (4 541) batteries.
- The pinch rows (26) pass.
- No line goes `SOUND` → refusal.
