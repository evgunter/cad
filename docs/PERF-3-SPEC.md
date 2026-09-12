# PERF-3 — per-face patches in local ids; bases assigned by an arena-order fold

**Status: ratified at dispatch (PERF orchestrator, 2026-09-10).** Binds
the implementer of unit `PERF-3`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first.

## 0. What this unit is, and what it is not

`mesh::tessellate` (`crates/mesh/src/tessellate.rs:43`) mints mesh
vertex ids in one running counter: topology vertices in arena order,
then every edge's chord points (`chords::compute_chords`), then — face
by face, in face-arena order — each face's interior grid points, each
lane minting an id as `positions.len()` at the moment it pushes
(`curved.rs:301`, `trimmed.rs:460,686`, the planar lane reads only).
Every lane therefore takes `&mut positions`, and the per-face loop is
serial by construction even though nothing a face does depends on
another face's interior.

This unit makes each face's patch a value computed from read-only
inputs: a lane returns its **interior positions and its triangles in
local ids**, and `tessellate` assigns every face's base offset in a
sequential fold over the face arena and rebases the triangles. The
mesh that comes out is **bit-identical** to today's — same positions
in the same order, same triangle indices, same patches, same
boundaries — because today's interior ids are already contiguous per
face in face-arena order, so an arena-order fold reproduces them
exactly. That identity is the whole verification of this unit.

It is NOT: a memo, a parallel map, a change to any sizing rule, a
change to the chord pass, or a change to the walk. It is the
prerequisite those need (`work/perf/plan.md` §5: the per-face patch
memo keyed on face content, and D9 idiom 1 over faces), landed alone
so its diff is reviewable as a pure refactor.

## 1. The shape

- A lane's output type carries `interior: Vec<Point3<f64>>` and
  `triangles` whose vertex references distinguish **shared** ids (a
  topology vertex or a chord point — every id below the mark
  `tessellate.rs` already computes as `shared_below`) from **local**
  ids (an index into `interior`). Choose a representation that makes
  the two unconfusable by type or by a documented encoding, and say
  which; a bare `u32` that means one thing below a threshold and
  another above it is the shape this crate's own D9 conventions
  forbid (`DESIGN.md` D9 engineering convention 1, sentinel-free
  tagged encodings).
- The three lanes (`tessellate_planar`, `tessellate_curved`,
  `tessellate_trimmed`) take `&[Point3<f64>]` for the shared
  positions (they already read chord and vertex positions) and return
  the patch value. The planar lane mints no interior points and
  returns an empty `interior`.
- `tessellate` folds: `base = positions.len()` at the face's turn,
  `positions.extend(interior)`, rebase every local id by `base`,
  push the `FacePatch`. Face-arena order, sequential — exactly the
  order the lanes mint in today.
- The certification each lane performs on its own triangles
  (`curved.rs:319` onward, the `cert::*` calls) reads positions the
  lane holds — shared ones from the slice, local ones from its own
  `interior` — and is unchanged in what it certifies.
- Whatever reads `positions.len()` for a purpose other than minting
  (the `shared_below` census at the end of `tessellate`, the
  `trimmed` lane's staging at `:460-686`) keeps its meaning; state in
  the PR what each such read became.

## 2. The pin

Bit identity, mechanically:

1. A differential row in `crates/mesh/tests/` that tessellates every
   body the crate's tests already build plus the corpus documents
   (`crates/editor-core/tests/corpus/`, through the public
   evaluate → gather → `tessellate` doors, at two δ) on the merge base
   and on this branch is not possible in one binary — so the pin is
   **goldens**: before changing anything, write a row that hashes
   `Mesh` (positions bytes, patches, boundaries) for a named set of
   bodies and commit the hashes; the same row on the new code must
   produce the same hashes. Keep the row (it is the D9 mesh contract
   made checkable) and say in the PR which bodies it covers and which
   lanes each exercises — planar, curved (every `ChartKind`), trimmed
   (NURBS and a trim carrier on an analytic surface), poles.
2. The tour's render lane and the tess-budget baseline must not move
   (`docs/TESS-BUDGET.md`; `tools/tess-lint`'s censuses go red if a
   count moves, and a moved count here is a defect, never a re-cut).
3. `benches/` rows `tessellate/washer/1e-4` and `/1e-6` within the
   lane's ~10 % noise (no speedup is claimed by this unit; a slowdown
   above the floor is a finding to explain).

## 3. Out of fence

`crates/mesh/src/sizing.rs`, `chords.rs`, `walk.rs` and every rule in
`curved.rs` / `trimmed.rs` / `planar.rs` that decides WHICH points and
triangles exist. PERF-1 edits `sizing.rs` and `chords.rs` concurrently
on its own branch; this unit does not touch them.

## 4. Report

Per `implementer-discipline.md` §1: ≤150 lines. Name the local/shared
representation chosen and why, every `positions.len()` read and what
it became, the golden set with its lane coverage, and the bench
readings before and after on your box (release, debug assertions off,
`benches/`'s own profile), with the spread.
