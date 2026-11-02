---
id: recl-edge-edge-composes-the-other-solids-material-beside-the-cone
kind: issue
title: recl::resolve_edge_edge composes the other solid's material at a vertex from its wedge and the faces holding the line, a second gathered reading beside the boolean's cone
status: closed
opened: 2026-11-01
priority: P1
cost: H
branch: oneread/recl-line-cone
pr: 6282
closed: 2026-11-02
---

Filed by `the-boolean-reads-the-other-operand-through-one-cone`'s sweep
(branch `oneread/boolean-one-cone`), which made the boolean's published
vertex-vertex rows read each own edge through one cone of the other
operand gathered at the point (`topo::link::Cone::gathered`,
`boolean::vtxfac::cone_classes`).

## What

`crates/topo/src/boolean/recl.rs` `resolve_edge_edge` decides, for two
edges of the two solids lying on one line through a shared vertex,
whether a flanker's representative direction lies in the other solid's
material (`membership`): it composes that solid's wedge about the line
(`in_wedge_of`, and `wedge_is_reflex` where the two half-planes
disagree) with each of its faces holding the line beside the wedge, by
the side of the face the wedge's bounds read (`link_of`, through
`vtxfac::PierceDatum::side`). That is the material of the other solid at
the point gathered across keys (its vertex's sectors and the faces
through the point), read by convexity and composition rather than by
the cone's crossing parity: the same fact the gathered cone answers.

It is not this unit's: the spec keeps the boolean's construction data
(sector chains, reclassification), and this reading decides which germ
a reclassified record carries, not a published row. It is also one of
the consumers of TANGRIM's bound-side reading (`link_of` reads a
partner's bounds against a face through `PierceDatum::side`).

## Owed

Weigh whether `membership` can ask the gathered cone for the
representative's material (the wedge's faces and the held faces as one
`Cone`), keeping `link_of`'s construction checks; or say why its
composition reads something the cone does not.

## Decided (2026-11-01, ONEREAD orchestrator, on a converged two-designer weighing)

Decided without Ev: the two designers converged on the final state, and no ratified text changes. The draw and the reports are on `analysis/design-fork/oneread-recl-line`.

Premise correction (both): the row's "the same fact the gathered cone answers" is false. `membership` asks for the other solid's material about the common line (its cross-section, the vertex cone's tangent cone at the axis ray), and the gathered vertex cone answers for directions from the point. A lane's witness shows the two part: at a thin pyramid's apex, the composition reads In and the vertex cone reads Out, both correctly. But the duplication is real one level over. The kernel already reads material about a line by parity (`Link::on_edge`; `Cone::gathered` at a sweep-minted vertex inside an edge), and recl's convexity-and-composition reading (`Half`, `in_wedge_of`, `wedge_is_reflex`, `compose`, `Part::bounding`) is another reader of that fact. Both designers worked out by hand that parity reproduces the composition on every physical arrangement.

The final state:

1. **A line cone, built sign-free in `boolean::sectors` beside `fans`**: the other solid's faces holding the line, each as what it is about the line. An edge's flankers are half-planes (fans along +axis, the flank's bound point, −axis, in the orbit's structural order). A face the line runs inside (a bisector-parted or `in_sector` wedge, a `Folded` face, a `HeldFace`) is a full plane. That is either a closed fan carrying the axis rays, which gives the reader wedge references, or a whole plane whose side is given from `PierceDatum::side` through `Cone::reads_given`; the lane picks and states why. The doors test takes the builder as a needle.
2. **`membership` reads that cone through the one reader**: `In`/`Out` answer directly; `On(k)` hands sector k's face to recl's existing coplanar ladder (`require_same`/`eq15_3_lump`/`tangent_lump`), which stays; `Boundary` (two sheets of one solid on one half-plane) refuses typed; a reading in band refuses.
3. **The reader's refusal carries its cause**: where the `Cone` reads nothing it names the sector it was in band of and the escalation, so recl keeps its declaration offers (`Coincide::FlankSense`, `offer_rows`) and `[tolerance.recourse]` holds; `cone_classes`' unread rows gain the reason too.
4. **Deleted**: `Half`, `in_wedge_of`'s half-space combination, `wedge_is_reflex`, `link_of`'s composition and `compose`, `Part::bounding`, `HeldFace::under`, `flank_rep`'s projection, the K names `bool_wedge_reflex`, `bool_flank_offset` and recl's `bool_dir_same`. Kept as construction: `HeldFace::holds`, `Folded::of`, `read_twice`, the two-faces invariant, the one-side check (renamed for what it checks), and the germ placing (`crossings`, `pick`, `onto_inside`, the slit rules).

Not now: folding `resolve_edge_sector` into the same reading, and one line cone across every wedge on a contact line. Both are named by the designers and owed a row if the lane confirms them.

## Closed (2026-11-02)

Merged as PR 6282: `membership` reads the other solid's line cone (`boolean::sectors::line_fans`) through the one `Cone` reader; the composition is gone; the `Cone`'s refusal carries a typed cause, so recl refuses typed with recourse (`LineMaterialUnread`). Dual review, adjudicated; record `docs/dual-review-log/rows/PR-6282.md`. Filed by the lane: `resolve-edge-sector-reads-the-other-solids-face-about-the-line-by-table-ii` (P1), `recl-reads-one-wedge-pair-at-a-time-on-a-contact-line` (P2), `the-split-rule-b-reads-an-in-plane-edges-material-by-convexity` (P3), and ATREST's `line-wedges-reads-material-about-a-line-by-the-wedges-turn-beside-the-line-cone`.
