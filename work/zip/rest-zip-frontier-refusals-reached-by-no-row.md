---
id: rest-zip-frontier-refusals-reached-by-no-row
kind: issue
title: "zip: fourteen of the rest zip's fifteen typed sub-frontier refusals are reached by no row, so which are gates and which are dead is unmeasured"
status: open
opened: 2026-10-01
priority: P3
cost: M
---

## Finding

`topo::RestZipFrontier` (`crates/topo/src/boolean/refusal_routes.rs`)
names fifteen sub-frontiers the declared-REST zip
(`crates/topo/src/boolean/rest.rs`) refuses at. One is reached by an
end-to-end fixture: `ChordBetweenIsolatedPierces`
(`crates/topo/tests/seat3_flush_detector.rs`, and
`crates/sweep/tests/m9_3_zip.rs`'s
`a_tangent_curved_sector_on_a_face_lumps_whole`). The other fourteen are
constructed only as values, by the refusal-text rows in
`boolean/mod.rs` (`every_rest_zip_frontier_ends_in_its_own_lever_and_no_declaration`
and the display table), which assert their wording and reach none of
their sites:

- seam realization (`realize_seam`, `fan_edge_between`, `mint_chord`):
  `ParallelSeamEdges`, `ChordMefRefused`, `ChordMekrRefused`,
  `PierceRingMekrRefused`, `ChordEndpointAbsent`,
  `ChordEndpointRevisited`;
- patch pairing (`pair_patches`): `PatchVertexUnmatched`,
  `PatchCyclesIncongruent`;
- rings (`glue_pair`): `HoleCyclesIncongruent`;
- the slit zip (`slit_zip`, `zip_folded`): `SlitFaceHoles`,
  `WholeBoundaryShared`, `RunVertexBranches`, `BandRunOffLoops`
  (also `work/zip/slit-zip-band-run-across-two-loops-is-reached-by-no-row.md`),
  `FoldVertexFused`.

The TANG m9-3 residues unit measured one family of these: the ring-count
refusal (`HoleCountsDiffer`) was shadowed by `ChordBetweenIsolatedPierces`
on the natural mismatch (a hole on one side only leaves isolated pierce
points on the other), and the ring-vertex refusal
(`HoleVertexUnmatched`) could not fire at all — the patch flood stops
only at seam edges, so every ring vertex is a segment end or a paired
patch face's outer vertex. The first folded into `HoleCyclesIncongruent`
and the second became a lane desync. The rest have not been looked at
that way.

Some are plausibly reachable (`PatchVertexUnmatched`: two operands whose
contact faces are split by internal edges at different interior
vertices, equal patch counts); some may be dead by an invariant the way
`HoleVertexUnmatched` was; the Euler-operator ones wrap refusals the
operators may never give on a well-formed seam
(`work/zip/rest-zip-drops-the-euler-operators-refusal.md`).

**What would close it.** Per variant: a fixture that reaches it,
asserted end to end; or the invariant that makes it unreachable, stated
at the site, with the refusal turned into a desync; or, where an
earlier refusal shadows it on the natural fixture, its text folded into
the one a user actually meets.

Filed from the TANG m9-3 residues unit.
