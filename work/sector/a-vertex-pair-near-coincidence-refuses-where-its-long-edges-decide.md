---
id: a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide
kind: issue
title: A vertex pair whose faces agree at the shorter arm refuses as a coincidence where the long edges' far vertices decide the pose
status: open
opened: 2026-09-29
priority: P3
cost: M
---


Filed by CONTACT-9 (review MAJOR-1).

**The witness.** Block `[0,20]²×[-20,0]`, and a parallelepiped on its
corner `(0,0,0)` with edges:
- `a = (10, 1, 500·ε)`;
- `b = (-1e-3, -0.2e-3, 0)`;
- `c = (1e-4, 1e-4, -1e-3)`.

It is pinned in `crates/topo/tests/contact9_side_codes.rs`,
`a_near_coincident_pair_at_a_corner_refuses_typed`. `∩`, `−` and `∪`
refuse `UndeclaredCoincidence`. At ε = 1e-6, where the 1 mm edges are a
thousand bands, an in-band edge contact escalates first. The same pose
with `b` and `c` ×1000 answers, and the row checks it.

**Why it refuses.** The tool's top face and the block's top read
parallel at the 1 mm arm. `pair_search` then records the pair as a
coincidence with every code On, and the carrier ladder refuses it
undeclared. The pair is not one carrier: `a`'s far vertex stands
500 bands above the top.

**Why reading the bounds is not enough.** CONTACT-9's first fix read
the bounds instead, `b` On and `a` Out. That turned the pair into half
a crossing. `within` then admitted the tool top against the block's
`y = 0` face as a 1e-10 graze at the arm, and the vertex's germs came
out odd: `ClassificationInvariant`, "odd number of surviving crossing
records at a vertex pair". At the 1 mm scale, the tool top and the
block's `x` edge coincide to the band. They part only beyond it.

**The fix's shape.** The vertex-pair classification decides at the
shorter arm, and this pose is a coincidence there and a crossing
beyond. An answer needs the classification to read the pair at its
long reaches: the germ tests (`within`) and the edge-sector event's
keys, not only the side codes. It is the pair-lane sibling of the pierce
lane, which answers the same pose placed on the face (CONTACT-9's
pierce rows).

**Checked against CONTACTHOLD's flush quarter round**
(`work/contacthold/a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex.md`).
The two rows share the refusal text but not the cause. That row's odd
count came from `recl::resolve_edge_edge`'s tangent short-circuit,
which marked a germ whatever the op. This row's comes from
`pair_search` and `within` reading at the shorter arm. That fix leaves
`contact9_side_codes` as it was.

## Evidence: the pose's refusal, and a span lever (2026-11-10, SEAL PR 6693)

Fork `unglued-corner-ending` settled this pose's refusal.
- **What it does now.** An unglued plane pair that `fan::meeting` reads parallel at the corner's arm refuses there: `Coincidence(CornerCrossing)`, the tilt `θ·arm` rejected by the Positive-only gate, offering "tighten below θ·arm/K".
- **The pins.** `crates/topo/tests/a_corner_reads_only_its_own_sectors.rs` pins it on `test_support::tool_on_a_block_corner`, this row's witness with its top tilted by θ.
- **Following the offer.**
  - The generic pose (ψ = 0) builds in all six runs just below the offer.
  - The pose hinged along `b` meets `bool_contact_edge` there instead.
  - `contact9_side_codes::a_near_coincident_pair_at_a_corner_refuses_typed` is that hinged pose; it is repinned to the new row.

Both designers measured what levering `fan::meeting` at the far span instead of the arm does:
- **The generic pose under a span lever is a ready witness.** The designers measured the odd germ count. The lane's mutant levering `bool_faces_parallel` at the larger span (PR 6693) measured the generic tool pose ending `ClassificationInvariant` ("germ record without a clean In/Out transition after rewrite") in every op, at ε 1e-9 and 1e-12.
- **The hinged pose still refuses.** Subdivision bisectors (`BoundSupport::Bisector`) are levered at the arm, a second short lever this row's fix has to reach.
