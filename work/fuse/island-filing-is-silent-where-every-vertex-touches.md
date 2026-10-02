---
id: island-filing-is-silent-where-every-vertex-touches
kind: issue
title: Island filing leaves a solid as the graft filed it when some shell has no vertex clear of the solid's other shells
status: open
opened: 2026-10-02
priority: P3
cost: E
---

`crates/topo/src/boolean/islands.rs`, `enclosers_of`: a shell's
nesting is read at its first vertex that lies on no other shell of the
solid. A shell every vertex of which touches another shell (an island
inscribed in its cavity, touching it at declared contacts at every
corner) reads `None`, and `islands_of` then files nothing for that
whole solid: the island stays under the wall's solid, the shape this
filing exists to remove, and tier 3 admits it (check 10 is silent on
the same shell for the same reason). No row reaches it.

The repair is a witness that is not a vertex: an interior point of a
face (`crates/topo/src/boolean/shell_witness.rs` already mints one for
planar faces, `face_interior_point`), tried after the vertices run
out. Until then the silence is the false-refusal direction, documented
in the module docs.
