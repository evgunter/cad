---
id: select-all-edges-picks-edges-a-fillet-refuses
kind: issue
title: A blend refuses a requested smooth edge (a bore's strut), so select all edges then fillet fails on any part with a round hole
status: closed
opened: 2026-10-31
priority: P0
cost: M
pr: 6344
closed: 2026-11-02
branch: reads/smooth-edge-identity
---


## Finding

Found driving the viewer (2026-10-31; Ev: P0). A bracket whose upright boss is exactly as wide as its base plate, so their side walls are coplanar: the union keeps those walls as separate faces (the at-rest check reports each pair as an `unproven-coincidence`), and the edges between them are tangent. The blend tool's **Select all edges** (`editor_core::all_edges`, `crates/editor-core/src/names/mod.rs`, through `BlendTool::load_all_edges`) loads those edges too, so a 1 mm fillet of all edges fails: "the fillet op refused: an edge's supports meet tangentially, so its dihedral has no definite wedge side (margin 0e0 m (fillet3_convexity_sign decided zero …))". An ordinary part cannot be filleted all round in one click.

The design question: should **all edges** leave out edges whose supports meet tangentially (they have no wedge to blend), or should the union merge coplanar faces so the edge does not exist? The second runs into the coincidence door, which leaves these coincidences unproven when the two widths are two typed numbers (`work/chrome/an-unproven-coincidence-is-hard-to-resolve-by-a-variable.md`).

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`:

```text
click "New…"
click "document name"
type "bracket"
click "Create document"
click "Add feature"
click "Add profile…"
click "rectangle"
click "0.0100" 0
type "0.06"
key Enter
click "0.0100" 0
type "0.04"
key Enter
click "Add profile"
click "Add profile…"
click "Profile 1 Profile bf935d4a4c62"
click "0.0100" 0
type "0.005"
key Enter
click "Extrude Profile \"Profile 1\" (bf935d4a4c62)"
vclick 0.5 0.5
click "on the picked face"
click "rectangle"
click "0.0400" 0
type "0.005"
key Enter
click "0.0100" 0
type "0.03"
key Enter
click "Add profile"
click "Add profile…"
click "on the picked face"
click "circle"
click "0.0000" 1
type "-0.012"
key Enter
click "0.0100" 0
type "0.003"
key Enter
click "Add profile"
click "Add profile…"
scroll 1000 200 -600
click "Profile 3 Profile c05c7d9b118f"
click "0.0050" 0
type "0.02"
key Enter
click "Extrude Profile \"Profile 3\" (c05c7d9b118f)"
click "Add feature"
click "Combine bodies"
click "Boolean tool…"
click "subtract"
scroll 1000 200 -600
click "Profile 2 Union d76139d3ff7f »"
click "Extrude 3 Extrude b869bfd8c6d6 »"
click "Commit boolean"
click "Combine bodies"
click "Blend edges"
vclick 0.5 0.5
click "Blend tool…"
click "Select all edges"
click "Commit blend"
tree
click "View"
dump
```

## Diagnosis and decision (2026-11-02, REFS orchestrator)

Two designers weighed this (fork `select-all-edges-tangent`) and agreed after one reconciliation round.

**The premise was wrong (measured).** The union does merge the flush walls. Three flush configurations were measured, and so was the GUI-shaped body plate ∪ boss − cylinder. The six `unproven-coincidence` rows are the merge's own record, as `crates/topo/README.md` ratifies; they do not show separate faces. The exactly-zero refusal comes from the **bore**. `profile::circle` lowers to two arcs, and C12.5 keeps the extrude's authored cuts, so the bore is two half-cylinder faces with two straight cut edges ("struts") between them at dihedral exactly 0. That gives the GUI's text verbatim, and `m5_pr12_refusals` already pins the case. So one-click fillet fails on any part with a round hole or boss, a slot, a revolve or an earlier fillet. Wedge-less edges are legitimate by ratified design: wrap edges (D1), C12.5 cuts and STEP imports.

**Decision: fix it at the blend verb, through one kernel door.**

- **The door.** One kernel door classifies an edge's dihedral, read where the blend already reads it (`fillet3_convexity_sign`), with the material side added (`geom_brep::classify_material_pairing`, or the normals' alignment, which is exact when the cross product decides Zero):
  - *Convex* or *Concave*;
  - *Smooth*: the sign decides Zero and the normals are aligned, a wedge of π;
  - *Knife*: the sign decides Zero and the normals are opposed, a cusp or slit;
  - in band.

  The validator's `ContactMark`/`DihedralClass` answers the same question. Make it read the door, or say in the PR why it cannot; do not add a third copy.
- **The verb (fillet and chamfer, which share the battery).**
  - A requested *Smooth* link is the blend's identity: the ball rests on both supports at the edge. The verb leaves it as it is, and `Blended` names it in a `smooth` row.
  - A request whose every edge is smooth returns the body unchanged, with that row.
  - A *Knife* link refuses, typed and named honestly (today's `TangentialEdge` with its recourse narrowed to a cusp or slit).
  - In band escalates as now.
- **Naming.** A left-alone edge carries through like any untouched edge, with no band row. editor-core's node badge says which edges were left alone, by `StableName`, and a refusal names its edge by `StableName` too. (Today `TangentialEdge`'s `EdgeKey` never becomes a name, which is why the filer guessed the wrong edge.)
- **Not in this unit.** `all_edges` keeps meaning every edge, and the button is unchanged. The crease selection atom (GS-Q2) stays on its P3 rows (`selector-has-no-sharp-edge-atom-so-a-kind-pair-takes-tangent-seams`, `adjacent-kinds-cannot-tell-a-crease-from-a-co-surface-seam`) as the door's second consumer, later.
- **Ratified text corrected with the change.** These are facts the change makes false, not new decisions:
  - names README N3's "Numeric coplanarity never merges" contradicts DESIGN `[topology.conventions]` and the topo README; re-word it to the ratified rule;
  - SELECT-DESIGN GS-Q2's "no 3D edge-convexity predicate exists" is now false;
  - KERNEL-VERBS' fillet run-out note, "no consumer has ever wanted either", now has a consumer:
    `work/bandrule/a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`.

**Done when:**

- a bored block with a two-arc bore, with all of its edges requested, builds and reports its two struts as smooth;
- a declared cusp body (`wedge_end_doors`) still refuses, as *Knife*;
- a sphere or torus "fillet all edges" returns the body unchanged with its seams reported;
- the GUI replay script in this item is re-run against the fix, and its log shows that the struts were the refused edges and what the next refusal is (expected: `UnsupportedCorner::MixedConvexity` where the boss meets the plate).

**What is still between this and a one-click bracket** (measured; filed on BANDRULE):

- `a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds`;
- `a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses`.

Review tier: **dual** (it changes a verb's contract on every blend request, and the material-side gate is easy to get subtly wrong). Class M, drawn byte 43 → **sequential** arm.

## Closed (2026-11-02, PR 6344)

A requested smooth edge (decided Zero with aligned normals, a wedge of π) is a silent no-op of the fillet and the chamfer, per Ev's ruling. A knife edge refuses as `KnifeEdge`, an unbuilt case. Your bracket's Select all edges now gets past the bore's struts, and stops at the next wall. That wall and the one after it are on BANDRULE at P0, confirmed by Ev:

- `a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds`;
- `a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses`.

Also filed: the knife blend (`a-knife-edge-blend-is-unbuilt-though-its-rolling-ball-is-defined`, P1), the chain-end corner fold (BANDRULE), and the split finish's composition (ONEDOOR).
