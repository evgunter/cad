---
id: a-plane-poses-foot-outgrows-the-coincidence-doors-budget
kind: issue
title: A split's crossing vertices on its tool plane are no first-rung construction, and the Sym replay cannot prove them
status: closed
opened: 2026-10-23
closed: 2026-10-23
priority: P1
cost: M
refs: [a-plane-poses-in-plane-origin-enters-the-mate-coset]
pr: 5624
---

Found by the implementer of `a-plane-poses-in-plane-origin-enters-the-mate-coset` (PR 5624), measured.

When a plane pose's value became `{ normal, offset }`, a split began reading its point on the plane as the foot, `offset · normal`. After that, `coincidence_door::a_reunited_splits_section_caps_are_one_construction` (`crates/editor-core/tests/coincidence_door.rs`) went red: the reunion's four crossing-vertex `OnCarrier` rows read `Unproven { recourse: Budget }`.

The halves' copies of one crossing are two minted names, `CrossingVertex { edge, sense }`, each with its sense against its own half. So rung 1 left them to the `Sym` replay. There, the plane's normal is the document's, and the tier cannot reduce it:
- The tier does fold `|n|² = 1` for a unit vector built from plain symbols.
- But it is the carrier's normal, a Newell sum normalised again, that is opaque (`work/sym/a-plane-poses-normal-read-off-a-face-is-opaque-to-the-unit-relation-at-sym.md`).

So the problem is not budget alone. The reunion's margins discharge at 2048 terms but not at 512. On designer A's probe (below), they fail even at 16384.

## Decided (2026-10-23, designer pair; INTENT orchestrator)

A blinded designer pair weighed this; the record is on `analysis/design-fork/msolve-plane-foot-sym-budget`. Both recommend the same final state ("likely"). No ratified text changes.

**Rung 1 learns the split's own constructions.** The plane value, the door's budget and the tier are unchanged.
- **(a)** A crossing vertex's construction is the split, the crossed edge, and the sense read against one fixed half, the tool normal's `Above` side. So the `Above` half's `Enters` copy of one crossing and the `Below` half's `Leaves` copy are one construction, and for an arc crossed twice `(Above, Enters) ≡ (Below, Leaves)` holds per crossing. Names are untouched: selection keeps the per-half sense.
- **(b)** A crossing vertex and a section edge lie on their split's tool plane, and so on that plane's construction; a plane read off a face is that face's construction (PR 5616's `plane()`). A section edge also lies on the operand face it runs across.
- **Hazard:** an operand vertex the tool happens to pass through (`OnToolVertex`) stays a value coincidence.

## Closed

Built on PR 5624 in `crates/editor-core/src/coincide.rs` (COINCIDE's ground).

`construction` reads a split's `CrossingVertex` against the `Above` half (`crossing`). It takes the half from the port the walk entered the split by (`split_half`), and keeps the ranking qualifiers. Crossings of one sense are ranked along the crossed edge as the target holds it, which both halves share, so ranks align per crossing.

`lies_on` gains `on_its_tool`: a crossing vertex or a section edge lies on its split's tool plane, and a section edge on the face it runs across. Rung 1 now reads an `OnCarrier` row through `lies_on` (`on_by_construction`). An `OnToolVertex` is carried (`CarriedAs::ToolCopy`), so its construction is the operand corner and never this arm.

Rows, all in `crates/editor-core/tests/coincidence_door.rs`:
- the reunion, every row `Structural(SameConstruction)` with no replay;
- `a_split_on_a_faces_plane_lays_its_crossings_on_that_face` (designer A's probe): box X split on box Y's top and unioned with Y, its four crossings proved at rung 1;
- `a_corner_the_tool_passes_through_is_not_on_it_by_construction`: a turned cube levelled so a corner lies on the slab's top by value; that row stays `Unproven` and the crossings beside it prove.

Filed beside it:
- `work/lathe/an-extrudes-cap-carrier-is-minted-by-newell-not-by-its-frame.md` (P2);
- `work/sym/a-plane-poses-normal-read-off-a-face-is-opaque-to-the-unit-relation-at-sym.md` (P2);
- `work/msolve/split-and-mirror-take-a-point-the-plane-kind-forgets.md` (P3).
