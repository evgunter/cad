---
id: stl-writes-a-solid-for-a-mesh-with-no-triangles
kind: issue
title: stl: the writers emit a valid empty STL for a mesh with no triangles
status: open
opened: 2026-09-22
---



Found by TESS-4's sweep (the vacuous-on-empty class), by reading; not
measured through the door.

`stl::facets` (`crates/stl/src/lib.rs`) folds `for patch in
&mesh.patches { for tri in &patch.triangles { … } }`, so a `Mesh` with
no triangles yields `Ok(vec![])` and both writers then emit a
syntactically valid file with nothing in it — binary an 84-byte header
plus `count = 0` (`crates/stl/src/binary.rs`), ASCII a bare
`solid`/`endsolid` pair (`crates/stl/src/ascii.rs`). No caller of the
writers on the shipped path validates first: `pncad_py`'s
`Mesh.to_stl_ascii` / `to_stl_binary` (`crates/pncad-py/src/py/mesh.rs`)
call `stl::write_ascii` / `write_binary` straight, and `pncad`'s prelude
re-exports the same two functions. Only the acceptance example
(`crates/stl/examples/export_acceptance.rs`) runs `check_mesh` in front.

Why it is a question now: as of TESS-4 `mesh::validate::check_mesh`
refuses a mesh with no triangles by name (`MeshError::NoTriangles`,
`EmptyPatch`), on the stated contract that such a mesh is not the mesh
of a solid — and the format's own keyword is `solid`. The writers are
the one public door that turns a `Mesh` into a file making that claim,
and they do not consult the contract. The producers are real:
`tessellate(&Body::new(), …)` answers `Ok` with an empty mesh (the
empty body is tier-1 and tier-2 valid), and a curved face that walks to
a degenerate domain leaves an empty patch with debug assertions off
(`work/tess/rim-free-loop-on-a-poleless-chart-meshes-as-a-hole.md`).

The call is `stl`'s, not tess's, and both answers are defensible: the
crate's docs are explicit that the writers refuse only per-triangle
degeneracy and I/O, not validity, so "an empty triangle list writes an
empty file" may be the right posture with a sentence saying so. What is
not defensible is the silence — today neither the crate docs nor the
`StlError` arms mention the state at all.
