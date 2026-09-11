---
id: subdivided-profile-side-coplanar-walls-gate
kind: issue
title: "sweep/topo: a subdivided profile side lowers to two coplanar walls — one surface key, one GeomSource, or does gate_maximal_faces refuse it?"
status: open
opened: 2026-09-02
github: 1568
refs: [1508, 1520, 433, BOOL-12]
---

## From GitHub issue 1568

Opened 2026-09-02; 0 comments.

**Forward observation from BOOL-12's pre-build loop-start reading (issues 727/433 lineage; filed by the S-BOOL orchestrator as a durable home — UNMEASURED).**

Since BOOL-8, a profile side may carry two authored vertices on one carrier (`line(len)` off a directed point: the declared straight continuation). Extruded or revolved, such a side lowers to TWO walls on one plane/cylinder. The boolean's `gate_maximal_faces` (`crates/topo/src/boolean/reduce.rs` ~:552) is edge-keyed: two distinct parent faces on one edge with the same surface KEY and planar → `NonMaximalFaces`; otherwise `oriented_plane_eq` with `declared: false`, and any non-Distinct verdict refuses. The merge ladder (`crates/topo/src/merge_faces.rs` ~:550/:617) merges only on a structural rung (same surface key), a declared rung (same `GeomSource`), or a per-call declared pair — never by value (the numeric rung is retired).

**The question:** does the sweep lowering give a subdivided side's two walls one surface key or one `GeomSource` (so the ladder merges them and the operand is maximal-faced), or do they arrive as two distinct-keyed coplanar faces (so `gate_maximal_faces` refuses the operand, and a subdivided profile cannot be a boolean operand at all)? Neither branch is wrong by construction, but one of them is a wall an author will hit, and nothing in tree states which holds.

Seam-independent: BOOL-12's declared arrival at the seam adds the same adjacency at the index pair the ring wraps at, nothing new in kind; BOOL-12 reads germ matching and the merge ladders as seam-blind (its PR carries the reading). Ground: `sweep`'s lowering and `topo`'s boolean gate.

**What the taker owes:** one measured row — a subdivided square extruded, then unioned with a box crossing the subdivided wall — reporting which branch holds; then either the lowering assigns one key/`GeomSource` per authored carrier (the structural rung fires) or the refusal names the recourse.

Refs BOOL-8 (#1508), BOOL-11 (#1520), BOOL-12, issue 433.

## Home

`work/bool/` — the gate is `crates/topo/src/boolean/reduce.rs`, an S-BOOL territory glob, and the question is BOOL-12's own forward observation on the PATHS lattice ground S-BOOL charters.
