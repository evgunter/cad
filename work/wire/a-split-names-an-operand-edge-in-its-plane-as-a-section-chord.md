---
id: a-split-names-an-operand-edge-in-its-plane-as-a-section-chord
kind: issue
title: A split names an operand edge lying in its plane, and the copies of it, as section chords: the edge's own name dies, and a face piece's Keeps cannot cite it
status: closed
opened: 2026-10-18
priority: P2
cost: M
design: true
closed: 2026-10-19
---


## Finding

Measured by the `wire-keeps` lane (2026-10-18, building the Split vertex
`Keeps` rule): the `NOTCHED` prism of `crates/topo/tests/m3_pr3_split.rs`,
extruded 1 and split at `y = 1`. The notch tip's lateral edge
`LateralEdge(tip)` lies in the tool plane. Its operand key survives in the
lower half, and the upper half holds one new edge along it per piece
touching the tip. `crates/editor-core/src/names/emit_topo.rs`
`name_split_edges_vertices` names all three by its chord pass
(`chord_faces`, which lists every boundary edge of a section face, operand
edges included), each `SectionEdge { face }` for the operand face across it:
`SectionEdge { Lateral(tip) }` below, `SectionEdge { Lateral(left rim) }`
and `SectionEdge { Lateral(right rim) }` above. Nothing in either half's
table is named `LateralEdge(tip)`, so a reference to that edge through the
split resolves in neither half, although the lower half holds it whole.

(The FORK-PINCHNAME designers expected these edge copies to be carried as
one name twice and refuse `Duplicate`. At this tree they do not: the chord
pass names them for different faces.)

A second consequence: the face `Keeps` lane (`name_split_faces`, "Same-side
multiplicity") reads a piece's boundary edges only by descent
(`chase_split_edge_to_table`), so a face piece bounded by a chord along a
parent boundary edge that lies in the plane does not cite that edge. Two
pieces of one face in one half that differ only there would tie. The
vertex `Keeps` rule (N2, PR 5258) cites such an edge ("an edge lying in the
tool plane included"), and its build reads the chord along it
(`chord_within`); the face rule does not say either way.

## Question

Is an operand edge lying in the tool plane a pass-through (its own name in
the half that holds its key) and its copies `SplitFragment { parent }` +
`Ends`, as N1/N2 would read an intact edge and its pieces, or a section
chord, as today? N2 names neither case. If it changes, N2's face `Keeps`
sentence should say whether an in-plane boundary edge is cited.

Signed (wire-keeps lane).

## A second form: the closed in-plane edge (2026-10-18)

A revolved apex on the tool plane sweeps a closed circle lying in the plane
(`split_pinch_keeps.rs`
`a_split_through_a_revolved_apex_keeps_the_circle_it_sweeps`). In the
pinched half, its copies are section chords from each corner copy round to
itself, and so they are read the same way. The vertex `Keeps` credits the
circle through the chord along it. Whether the circle itself keeps its own
name is this row's question.

Signed (wire-keeps lane).

## Weighed (2026-10-18, FORK-INPLANE, five rounds)

Two designers, briefed with the problem only, converged after three
crossovers:

- **The premise is corrected.** No half holds the in-plane edge whole
  between its own faces. In the half that keeps the operand's key, the
  edge lies between two coplanar section faces. It is named by
  `frag_rows.root` of a section face, that is, by the operand face the
  section's `mef` ran in: build order. The coplanar floor's rims are a
  second shape of the same defect. The `-y` revolved refusal is likely
  the same lookup.
- **Measured.** A split half that carries the leftover is refused
  `NonMaximalFaces` as the input to any later boolean, and no editor
  door offers the merge
  (`work/cleave/a-split-exactly-on-a-face-leaves-a-half-no-boolean-accepts.md`).
  `merge_coplanar_faces` on the lower halves gives the box the plane just
  below the tip gives. With the vertex `Keeps` chord arm switched off, no
  fixture ties.
- **Recommendation.** The split finishes each half with maximal faces.
  An edge along an in-plane operand edge is that edge, or a piece of it,
  only when both its faces descend from the edge's two faces. A copy
  beside a section face is a `SectionEdge` chord, as today. `Keeps` reads
  descent only, in one builder for faces and corners, and N2's "an edge
  lying in the tool plane included" is struck. Every merged group is
  `Merged` of its constituents, section faces included, and no section is
  renumbered.

The build closes, or verifies closed, `split-keeps-has-two-builders-that-disagree`,
`a-revolved-pinch-split-refuses-with-its-normal-flipped`,
`a-section-faces-division-root-is-the-operand-face-its-cut-ran-in` and
CLEAVE's split-half row. It also re-baselines `split_pinch_keeps.rs`'s
four expected sets from `{rim, in-plane edge}` to `{rim}`.

## Decided (2026-10-19)

Ev approved the recommendation on PR 5306 ("sure, sounds good"): N2/N3 as
the PR edits them. Design-fork log row 163. The build is a WIRE unit:
the naming side plus `topo/src/splitting/finish.rs` (CLEAVE's
`a-split-exactly-on-a-face-leaves-a-half-no-boolean-accepts`).

## Resolution (2026-10-19)

Closed by PR 5334: a split finishes each half with maximal faces (recording a curved skip), names in-plane edges and merged groups per N2/N3 as Ev approved on PR 5306, and reads `Keeps` by descent in one builder. Pinned in `split_pinch_keeps.rs`, `m3_pr3_split.rs` and `a_lower_half_on_the_plane_takes_a_box_hole`.
