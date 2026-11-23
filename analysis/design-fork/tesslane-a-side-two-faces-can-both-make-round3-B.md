## For Ev

**Bottom line: no promise in the tree is broken by two private vertices at one position, or by a double layer.** So the split point is a preference about the mesh, not a contract. I would put it to you as a choice between two consistent pairings. I lean to the first: a straight-midpoint split, with one patch keeping the side. Confidence: likely, and the two are close.

### What the tree promises a consumer that has no vertex ids (from reading the code, plus one measurement)
- **`check_mesh`** (`mesh::validate`) checks vertex ids only. Its doc says "distinct indices, coincident points … pass". The coincident-edges row (`two-coincident-edges-…`) also lists "one id per sheet, coincident positions" as an admissible mesh. (sure)
- **The STL writer** (`crates/stl/src/lib.rs`) promises:
  - no snapping, no merging of vertices, no reordering;
  - byte-identical output for the same mesh;
  - facet normals taken from the f64 winding;
  - a refusal only when a cross product is exactly zero.
  It promises nothing about what a reader gets by merging vertices at the same position. (sure)
- **The nightly admesh gate** (`scripts/check_admesh.sh`, `nightly.yml`) is the only check by position. It merges vertices by position in f32, accepts no repair, and checks a fixed list of bodies.
  - I measured what it sees. Two closed tetrahedra sharing one edge, so that edge has four triangles, report **0 disconnected facets, 0 degenerate facets**. The only change was "2 parts".
  - So the gate does not see an edge of four triangles. It would not see one recreated by merging two coincident split points either. (sure, by that measurement)
- **`check_volume`** reads the signed volume. In a double layer the two triangles face opposite ways, so they cancel. Its bound uses the body's area, not the mesh's. Nothing breaks. (sure)
- **`[mesh.winding]`** is a per-triangle promise. Both kinds of split produce triangles that are checked under it, and a thin one refuses with a typed error. Nothing in the tree promises a mesh free of self-contact: a body thinner than δ can already produce crossing sheets. (sure)
- **`[determinism.equivariance]`** is about rigid motions and reflections. Moving or mirroring a body does not reorder its face arena, so a tie-break by arena order commutes with both. The clause only says to *prefer* intrinsic quantities over enumeration order, and to document a fallback. It is also marked UNAUDITED. A tie-break by arena order therefore breaks nothing as written. It is only less preferred. (likely)

### So the pairings
- **(1) Straight midpoint; one patch keeps the side; every other inside user splits.** This is my lean.
  - The new triangles are pieces of triangles that were already checked, so they need no certificate.
  - The join needs no chart coordinates, and no lane is run again.
  - Merging by position still gives every edge exactly two triangles: the new point sits on the keeper's edge as a T-junction, never as a shared vertex.
  - Keeper rule: a face whose own boundary carries the side keeps it. Failing that, the first face in arena order keeps it, and the code documents this as the fallback.
- **(2) A point on the face's own surface; every inside user splits.**
  - The two sheets separate, and no tie-break is needed.
  - The join needs each patch's chart coordinates, or each lane exposes a split hook. Each half needs a certificate that can refuse, and so far that is measured only on ruled faces.
- **Do not mix them.** A straight midpoint with every user splitting puts two vertices at one position, which recreates the edge of four triangles by position. (sure)

Choose (2) only if you want the mesh to separate sheets that touch. Nothing in the tree asks for that today. If you do want it, it should become a stated promise: in `[mesh.winding]`'s neighbourhood, or a new clause beside `check_mesh`'s "combinatorial only".

### What moved me this round, and what did not
- **Moved.** Designer C's point stands for one pairing: a straight midpoint with every user splitting is wrong by position. That is why pairing (1) has to keep the side in one patch. Designer A has also come round to the straight midpoint with "all inside users split". I think that combination is the one to avoid.
- **Not moved.**
  - C's argument that a straight midpoint "only renames a vertex" over a double layer. A double layer breaks no promise. It also exists today wherever δ is larger than the body is thick.
  - C's argument that equivariance forbids the arena-order tie-break. The clause says "prefer", and placement does not change arena order.

## For the orchestrator

- **The measurement I took.** I wrote two ASCII STLs with a Python script in `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork3/B/scratch/adm/` and ran the system `admesh` on each (no cargo):
  - `control.stl` is one tetrahedron: clean, 1 part.
  - `fouruse.stl` is two tetrahedra sharing one edge: 0 disconnected facets, 0 degenerate, 0 backwards, 2 parts.
  - So the nightly gate catches an edge of four triangles only if it happens to split the parts count. That deserves its own row: the gate is weaker than its header ("watertight/manifold") says.
- **Before this goes to Ev:** nothing further is needed for pairing (1); designer C's probe already ran it suite-wide.
- **What can wait.** The other designer's request (the surface-point split run on the triangulating arms and on sphere and torus faces, counting certificate and fold refusals) prices pairing (2). Run it before Ev only if you expect Ev to lean (2). Otherwise it belongs to the build unit, if (2) is chosen.
- **Unchecked: the viewer's pick memo.** It caches by `StoredPatchId`, and a patch split at the join is no longer the stored patch alone. Under (1) that costs one id for each patch that splits. Someone should check what `editor_core::PickMemo` does with a patch carrying a split list.
- **Recommendation format:** a two-option `[ev]` decision with a stated lean, not a single recommendation, because the deciding question (should the mesh promise sheet separation by position?) is a new promise and Ev's call.
- **Still standing from earlier rounds:**
  - the chord-pass fix, so that two edges between one vertex pair never share one segment;
  - the fold fix;
  - the stale comment above `joined()`;
  - that the walked lane's memo key omits the neighbour's surface (moot if the recommendation is taken).
