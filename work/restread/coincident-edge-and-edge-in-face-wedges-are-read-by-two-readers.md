---
id: coincident-edge-and-edge-in-face-wedges-are-read-by-two-readers
kind: issue
title: Tier 3′ reads wedge overlap on a line by two readers: line_wedges for two edges, rest::judge for an edge in a face
status: open
opened: 2026-10-21
priority: P4
cost: M
---

Filed by `tier-3-prime-reads-no-side-between-coincident-edges-on-one-line`'s
lane (branch `restread/coincident-wedges`), from that row's evidence
"two readers of wedge overlap on a line".

## What

The census asks one question in two places: whether two materials
meeting along one line overlap there.

- Two coincident line edges: `census.rs` `ee_overlap_sides`, through
  `census::line_wedges`, which reads each edge's wedge from its two
  face normals at one point and tests membership through tier 3's
  pinch-corner reading (`validate::wedge_holds_named`).
- An edge lying in a face of its own solid: `census.rs`
  `edge_in_own_face`, through the touch analysis (`Link::on_edge`,
  `Link::in_face`, `rest::judge`).

Tier 3's own pinch-corner check and the cross-solid touch analysis are
the other two spellings. Two readers of one question can disagree in
band, and each carries its own predicate names into the K stream.

## Owed

One reader for both. Either `rest::judge` over two `Link::on_edge`
links at the shared span's middle, or `line_wedges` taking a face's two
sheets about the line as a wedge. The Boolean gate
(`boolean/ops.rs` `coincident_wedges_disjoint`) moves with whichever
is chosen. Witnesses that must not move:
`crates/topo/tests/a_doubled_line_does_not_revert.rs` and
`crates/topo/tests/a_face_inside_an_edges_wedge_refuses_at_tier_3_prime.rs`.
