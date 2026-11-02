---
id: step-round-trip-parts-shared-points-so-a-pinch-half-refuses-at-import
kind: issue
title: STEP export writes one CARTESIAN_POINT per vertex, so a pinch half whose copies share a point re-imports on separate points and refuses at the import gate
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [3856]
---


## What

Since PR 3856, an op's copies of one vertex share its `PointKey` (D1
tier 3′: "structural sharing (same surface or point key)"). That is
how a split pinch half holds its touch without records. STEP export
does not carry the sharing:
- `step-export`'s writer mints one `VERTEX_POINT` and one
  `CARTESIAN_POINT` per `VertexKey` (`writer.rs`, `vertex_point`,
  keyed on `shared.vertex_points`).
- So two vertices on one point are written as two coordinate-equal
  points, and import adopts them as two points.

Measured on `tquery/split-pinch-shared-point` with a scratch test, not
committed. The notched block (`m3_pr3_split.rs` `NOTCHED`, height 1)
split at `y = 1`, taking the above half:
- Before export, `validate_pseudomanifold` with no records passes.
- `step_string` exports it; the document has 67 `CARTESIAN_POINT`s.
- `import_step` refuses at its shared at-rest gate (`TierInvalid`)
  with three verdicts: `UndeclaredContact` `VertexVertex` at
  `(4, 1, 0)` and at `(4, 1, 1)`, and `EdgeEdgeOverlap` at
  `(4, 1, 0.5)`.

So a valid kernel body does not round-trip. The fix is for the writer
to key `VERTEX_POINT`'s `CARTESIAN_POINT` on the `PointKey`, so one
point is written once and referenced by every vertex on it (valid
STEP). Import then has to adopt one `CARTESIAN_POINT` referenced by
several `VERTEX_POINT`s as one point. It must not adopt coordinate-equal
points as one: that would be the inference the ladder forbids.
Whether a third-party reader keeps the sharing is a separate question.

## `iso`'s canonical form (TOPO's `crates/topo/src/iso.rs`)

`canonical_form` counts points in its header but does not encode which
vertices share one. It is `pub(crate)` and only the Euler kill/make
round-trip rows use it (`euler_kill.rs`). There, a `mev_null` ∘ `kev`
round trip leaves the count unchanged either way, so nothing reads the
partition today, and I have not filed a row for it. If an op ever needs
to show that it kept or parted a shared point, `iso` would have to
encode the vertex-to-point partition.

