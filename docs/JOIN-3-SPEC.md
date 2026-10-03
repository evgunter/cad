# JOIN-3 — a matched segment carries its chord curve

Binds the implementer of `work/join/JOIN-3.md`. Deleted at merge.

## The defect

The join decides a segment's chord geometry in one place and assumes it
in another:

- the joiner mints each chord with the lane's curve (`chord_spec`, or
  `along_edge_spec` for `JoinLane::AlongEdge`), which may be a conic;
- role resolution's ring lane (`choose_roles` → `ring_run_ccw` →
  `Body::planar_run_winding_decided`, `Closing::Chord`) closes the run
  it winds with a STRAIGHT chord.

On a planar face whose section 2-gon has a straight side and an arc
side, the run closed by the straight chord has zero area in both role
orders, and the join refuses `JoinDesync "ring-run winding is
degenerate"`. That is the blind D pocket's top pose
(`work/join/blind-d-pocket-subtract-refuses-with-join-internal-words.md`,
`## Diagnosed`, defect 2).

The pocket's defect 1 is a different one: the chord core's value-reading
adjacency skip took a straight edge for the section segment, so the arc
side was never minted. JOIN-1 replaced that skip with the locus test, so
defect 1 may already be gone. Re-measure the pose first.

## The design

1. A matched segment carries its chord curve, computed ONCE when the
   segment is matched. It is the curve the joiner will mint.
2. The `mef`, `choose_roles` and `ring_run_ccw` all read it. The ring
   lane's closing term is that curve's own winding contribution. A conic
   closing term goes through `loop_winding::conic_segment_term`, so
   `Closing::Chord` grows a closing curve, not a second spelling of the
   sum.
3. Nothing else recomputes a chord's geometry from the face pair after
   matching.

## Acceptance

- The blind D top pose builds at its exact volume,
  3.6632128205514776 = 4 − 0.5·A_D, passes tiers 2, 3′ and the
  certificate, and is a legal operand (`sweep::test_support::assert_legal_operand`).
  The bottom pose still builds.
- A row goes red if `ring_run_ccw` closes with a straight chord again.
  Demonstrate it by the mutation.
- No SOUND→refusal and no BAD on R1's and R2's batteries
  (`join1_r1_probes.rs`, `join1_r2_rand.rs`, ignored), main vs head.
  Report the table.

## Not this unit

- Where the zip gets its segments: JOIN-2.
