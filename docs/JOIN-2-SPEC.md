# JOIN-2 — the declared-REST zip reads the join's segments

Binds the implementer of `work/join/JOIN-2.md`. Deleted at merge.

## The defect

The declared-REST zip (`crates/topo/src/boolean/rest.rs`) enumerates
section segments for itself. `enumerate_segments` pairs germs by its
own straight-chord facing test, which the join stopped using when it
became locus-aware. It identifies a segment by its VERTEX PAIR, and
`realize_seam` → `fan_edge_between` then looks the seam edge up by
that pair, so two distinct edges between one vertex pair refuse
`RestZipUnsupported { ParallelSeamEdges }`. That is one fact,
"which section segments exist and what each one is", decided twice,
with the zip's copy the weaker
(`work/join/rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity.md`).

JOIN-1 (PR 3790) gave every germ a per-operand locus
(`boolean::Locus { InFace(FaceKey), OnEdge(EdgeKey) }`). It made the
join's matching (`join::partners`, `find_match`) the one place that
pairs germs, and it routed segments that are an edge in both solids
through `JoinLane::AlongEdge`. This unit finishes the design the two
designers converged on (`work/join/log.md`, 2026-10-02):
**one enumeration of segments, read by both join and zip.**

## The design

1. The join's matching yields the matched segments as data: the two
   ends (records and germ slots), the per-operand loci, and the lane.
   It is the one producer. Extract it as a function both the join's
   surgery and the zip call, so the zip never re-pairs germs.
2. `rest.rs` reads those segments:
   - `realize_seam` takes `OnEdge(E)` as "the segment already IS
     operand edge E", which is record data rather than a lookup.
   - It takes `InFace(F)` as `mint_chord`'s host face, replacing the
     "unique common face" search.
   - `enumerate_segments`, its facing test and `fan_edge_between` go.
   - So does the `RestZipFrontier::ParallelSeamEdges` arm, unless some
     other path still reaches it. Prove it unreachable or keep it with
     a row that reaches it.
3. The zip's patch discovery and glue steps (its own steps 3 to 6) stay
   (CONTACT-DESIGN C7). This unit changes only where the segments come
   from.

## Acceptance

- Every declared-REST suite stays green, built through the zip where
  main builds it through the zip:
  - `m9_3_zip`, `r1_probes_m9_3`, `curved_mergedoor`, `mate7a_torus_rest`;
  - TANG's arc-matching rows (PR 3823);
  - `reach_continuation`;
  - the tour's two-peg mate.
- A declared-REST pose with two arcs between one vertex pair (the
  dumbbell's joint circle, split into two semicircles at the same
  vertices) zips by key. Pin one that reaches the zip itself, not the
  join, if one exists. If none does, say so.
- No SOUND→refusal and no BAD on R1's declared battery
  (`join1_r1_probes.rs`, ignored), main vs head. Report the table.
- `rest.rs`'s module docs state the new source of segments.

## Not this unit

- The segment's chord curve: JOIN-3.
- The closed single-site loop's surgery.
