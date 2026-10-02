---
id: selector-has-no-sharp-edge-atom-so-a-kind-pair-takes-tangent-seams
kind: issue
title: "select: no convexity atom (GS-Q2), so a kind-pair description cannot tell a crease from a tangent seam — the GS-Q2 Convex/Reflex demand row"
status: open
opened: 2026-10-02
priority: P3
cost: M
design: true
---


## Finding

The demand row for the reserved `GeomPred::Convex` / `Reflex` slot
(`crates/editor-core/src/names/geompred.rs`, "Reserved, unbuilt:
convexity (GS-Q2)"; `docs/SELECT-DESIGN.md` GS-Q2). GS-Q2 deferred it
because "the demand evidence is a COMMENT, not a call site". There are
now two call sites, and both want the same atom: a convexity reading
that answers Zero on a tangent edge, so "the convex ones" leaves a
tangent seam out.

1. **The rocker's keyhole** (`demos/tour/src/rocker.rs`,
   `crease_narration`). The plate's outline is filleted in the profile
   and its keyhole on the solid. The keyhole's creases are, by
   description, "the lines between a cylinder and a plane", and on a
   plate with profile fillets that description also matches every
   vertical seam where a fillet arc meets a straight side: 8 edges, 6 of
   them tangent. `fillet_edges` on the 8 refuses `TangentialEdge`, as
   it should. Neither seat can say "the sharp ones":
   - the body seat (`topo::query`) has kind and adjacent-kind
     predicates only, so the scene scopes the description to the
     keyhole loop's struts (`Extruded::walls`);
   - the document seat (`select_where`) localises with
     `datum_distance` to an axis datum at the keyhole's centre
     (`crates/pncad-py/tests/test_north_star.py`, `TestRocker`).
   Both say WHERE the creases are rather than WHAT they are.
2. **The snowman's waist selection** (`demos/tour/src/snowman.rs`,
   PR 3787): an adjacent-kind pair cannot tell the crease it wants from
   a co-surface seam between faces of the same kinds — the finding
   PR 3787 files as `adjacent-kinds-cannot-tell-a-crease-from-a-co-surface-seam`
   (on that PR's branch; a `refs` entry is added when both are on main).

## What the taker owes

The GS-Q2 question re-opened with these call sites: a decided dihedral
atom (the blend battery's `fillet3_convexity_sign` margin is the
existing comparand, `sweep::blend::battery::convexity_at`), its answer
on a tangent edge (neither convex nor reflex — the `Zero` the battery
already refuses), and its body-seat twin in `topo::query`. It is a
design fork: it is weighed first by one Opus and one Fable designer
(`docs/prompts/designer.md`, procedure in
`memories/orchestration-model.md`) until the recommendations are
clear, and goes to Ev only if it is Ev's fork — a change to the
ratified GS-Q2 decision is.
