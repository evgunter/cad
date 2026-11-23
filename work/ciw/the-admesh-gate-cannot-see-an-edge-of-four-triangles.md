---
id: the-admesh-gate-cannot-see-an-edge-of-four-triangles
kind: issue
title: scripts/check_admesh.sh passes an STL with an edge of four triangles: admesh pairs facets greedily, so the gate its header calls watertight/manifold checks gaps and orientation only
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found by the designers weighing TESSLANE's shared-side question (`work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md`), 2026-11-23. Filed here because `scripts/check_admesh.sh` is CIW's.

## What

The nightly's export gate (`.github/workflows/nightly.yml` → `scripts/check_admesh.sh`) is the one check in the tree that joins a mesh's vertices by position, as a reader of an STL must. Its header says "External watertight/manifold verification of exported STLs", and `crates/stl`'s tests call what it guards a "watertight STL". It accepts no repair: any disconnected, degenerate, removed, added or reversed facet, any fixed edge or normal, or a part count other than one fails.

It does not see an edge used by four triangles. Measured with the system `admesh` (0.98.5), two ways:

- Two closed tetrahedra sharing one edge, written as ASCII STL: 0 disconnected facets, 0 degenerate, 0 reversed. The only thing that differs from one tetrahedron is "2 parts".
- A ball of radius 1.05 bored at radius 1 meshed with every shared-side rule off, whose rim-to-rim chord is a side of four triangles (`mesh::check_mesh` refuses it): admesh reads it clean, one part.

admesh pairs facets across an edge greedily, so an edge of four facets is two satisfied pairs. What the gate does enforce is that there are no gaps and that the orientation is consistent and outward: a closed mesh of zero or negative volume has every facet reversed, and fails.

## Why it matters

`mesh::check_mesh` counts edge uses by vertex id, so a mesh in which two separate vertices sit at one position passes it. An STL has no ids. "No edge of more than two triangles once vertices are joined by position" is therefore promised by the gate's words and checked by nothing.

## What closing it takes

Either the gate gains the check it names (a count of edge uses after joining vertices by position, in the script or as a Rust check beside `check_mesh` that welds by position), or its header and the tests' wording say what it checks. The acceptance bodies are few, so the count is cheap.
