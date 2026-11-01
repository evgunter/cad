---
id: the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane
kind: unit
title: The split decides a vertex's distance from the section it will end (delta / sin psi), not its offset from the plane
status: closed
opened: 2026-10-12
priority: P2
cost: M
parent: a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join
branch: cleave/vertex-against-section
closed: 2026-10-14
---

Part 4 of the design on
`a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join.md`.
It is separable from parts 1–3.

## What

`split_vertex_side` decides δ, the vertex's offset from the plane. The
edge certificate reads the vertex's distance from the section it ends,
which is δ/sin ψ, where ψ is the dihedral between the plane and the
wall at the vertex. Measured in a replica: δ = 0.2ε at ψ = atan 0.1
reads 2.01ε (review m2 on PR 4810; the brief's "0.5ε" pose is likely
0.2ε). A vertex admitted Zero at 0.99ε with ψ = 0.05 misses its
section by 20ε: a definite miss ending "kernel defect" for an input
the kernel accepted. That is two decisions inferred from each other
(Q1).

## Owed

Decide the vertex against the section it will end (δ/sin ψ over its
curved incident faces) at the split. Do the same at the Boolean's
germ-plane classification. Pin the m2 family (normal (cos φ, sin φ,
0.1), φ ∈ {0.9, 2.2}) at every ε. Once this lands, a definite endpoint
miss at the join is a defect, and the certificate's
`EndpointStart → Defect` row is true.

## Built (branch cleave/vertex-against-section)

`topo::vertex_section::decide_vertex_on_plane` is the one rule both
doors decide a vertex on a plane by: the split's vertex sweep
(`splitting::classify::classify_vertices_margined`) and the Boolean's
vertex against a planar face of the other solid (the three
`bool_vertex_face_side` sites of `boolean::reduce::sweep_direction`,
through `vertex_face_side`). The plane offset is decided as before,
under the door's own name. A Zero offset is then read against the
faces a section enters from the vertex, `δ / sin ψ` at the shallowest,
under `vertex_section_side`: Zero is ON, in band escalates typed
(`SliverVertex` at the split, `Escalated { Coincidence(VertexOnFace) }`
at the Boolean, each with its lever), and definite reads the vertex off
on its offset's side.

- Which faces a section enters is the neighbourhood walk's own entry
  reading (`splitting::entry_side`, factored out of the walk, and
  `bisector_side` for a wide sector): a face's sector holds the trace
  where its bounds leave on opposite sides. A bound in the plane, a
  supporting plane, a planar face and a ruling the C5 table reads add
  no reading.
- The reading's own rounding is charged through the division
  (`Rounded`, with `Rounded::quotient` now in `geom_core::running`): the
  door's offset as it evaluated it (`sector_shape::plane_offset_rounded`,
  bit-identical to `plane_offset`), plus the skew between the normal it
  evaluated with and the table's unit normal. It is the input's, with
  its lever, only where `K·ρ` reads Zero (`vertex_section_resolution`).
  Past that, a definite side the rounding cannot move stands
  (`vertex_section_clear`), and otherwise the offset's own verdict does,
  as on main: Ev's decision on PR 4812, under which nothing that builds
  starts refusing.

Measured on `origin/main` first, the unit cylinder turned φ ∈ {0.9,
2.2}, plane through its seam vertex `+ δ·n`, `n = cos ψ·w + sin ψ·ẑ`
(`crates/sweep/examples/vertex_against_section_probe.rs`), at ε = 1e-9,
1e-6, 1e-12, both doors alike:

- ψ = atan 0.1, δ = ±0.2ε: `Join(Euler(Certification { Escalated {
  EndpointStart } }))` reading 2.0099744e-9 at ε = 1e-9 (the replica's
  2.01ε), a kernel-defect ending. On the branch: `SliverVertex` /
  `VertexOnFace` reading `vertex_section_side` 2.0099752e-9.
- ψ = atan 0.05, δ ∈ ±{0.5, 0.9, 0.99}ε, and ψ = 1e-3, δ ∈ ±{0.1, …,
  0.99}ε: `ResidualExceeded { EndpointStart }` at the join, the
  definite miss. On the branch the vertex reads off at the split; the
  split builds the closed-form halves on the side the plane leaves the
  cap, and the other side refuses `UnpairedLooseEnds`
  (`a-rim-grazing-the-plane-at-a-vertex-read-off-by-its-section-leaves-two-ends-unpaired`).
- ψ ∈ {0, 90°}, every δ: unchanged.

Pinned in `crates/sweep/tests/a_vertex_on_a_grazing_plane.rs` at every
ε row:
- the ψ ladder (90°, 30°, atan 0.1, 3°), both doors: ON rows build to
  the `δ = 0` halves, in-band rows refuse typed at the vertex with the
  closed-form reading and the lever, the lever builds, and each definite
  row ends exactly as measured;
- the plane exactly through the seam vertex with its origin moved
  kilometres in-plane (ψ ∈ {3e-4, 1e-4, 1e-5}): no row refuses at the
  vertex, every body returned validates, and ψ = 3e-4 builds at the
  default band, as main does; at ψ = `0.1·K·u/ε` no row offers the
  section reading's lever;
- a supporting plane through a rim vertex builds at every offset within
  the zero band, at the split and as a slab the Boolean subtracts;
- two cone walls both entered at a vertex refuse on the shallower.
