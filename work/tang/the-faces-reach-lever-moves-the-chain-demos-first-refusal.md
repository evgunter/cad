---
id: the-faces-reach-lever-moves-the-chain-demos-first-refusal
kind: issue
title: The faces'-reach lever (#5204) moved the chain demo's first refusal at three links from dihedral_arm to dihedral_wedge, and demos/tour's certified cell is red on main
status: open
opened: 2026-10-16
priority: P1
cost: E
---

`demos/tour`'s `tests/eps_regression.rs::certified_cells_run_green_at_default_eps` is red on main (`9ae7e2e959`) and was red from the merge of #5204 (`75cf96347d`, `tang/u-lever-face-extent`). Bisected over first-parent merges: it passes at the first parent `b5d7d750a0` and fails at `75cf96347d`. `demos/tour` is its own cargo root, which the PR gate tests only when the diff touches it.

## The diff

`demo-tour certified` panics at `demos/tour/src/chaintol.rs:502`:

```
the table says the first refusal at 3 link(s) over the whole study is `dihedral_arm`; it was: node caf4a2e4ca41 — the transform op refused: an edge the map moved failed re-certification: the surfaces' tangent planes at sample 4 are undefined: a surface's gradient is zero or undefined somewhere over the enclosure of the point there, ... [Transform(Certify { edge: EdgeKey(1v1), source: Escalated { check: TangentPlanes, sample: 4, cause: Indeterminate { margin: MarginDiag(Invalid), terminal_sliver: false, band: Band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("dihedral_wedge") } } })]
```

At 2 links the first refusal is unchanged: `dihedral_wedge`, the same edge, sample 4. At 3 links it is now the same, and so it no longer matches the header table's `dihedral_arm`, `[0, 7.34e-3]` row (`demos/tour/src/chaintol.rs:38`). The 4-link row is not reached because the cell panics at 3 links, but it claims the same `dihedral_arm`.

## Judgement: correct, re-bless

The table's `dihedral_arm` straddle came from the lever arm `min(curvature arms, edge_extent)`, with the moved edge's own extent enclosed over the whole angle box (`work/sym/a-chain-of-three-joints-straddles-dihedral-arm.md`). #5204 levers the wedge at the faces' reach (`geom_brep::dihedral::folded_lever_arm(.., reach)`, `topo::wedge_reach`), so that arm is no longer the edge's straddling extent and the gate passes. The first refusal is now the one the cell's own `the_wall_is_the_wedge_not_the_arm` row names as what bounds the certifiable box: the wedge's poisoned tangent-plane margin on `EdgeKey(1v1)` at sample 4. The chain still refuses at three and four links, as the table says, and only which refusal comes first has moved. That is the change Ev approved for #5204 ("a short edge between large faces now reads its faces").

**Fix:**
- Restate the 3- and 4-link rows of the header table as `dihedral_wedge`: no tangent plane, re-measuring the 4-link row and the costs.
- Make the `(Lane::Symbolic, _)` arm at `chaintol.rs:489-493` expect `dihedral_wedge` at every link count of 2 or more.
- Re-word the paragraph after the table that says the arm's straddle comes first at three and four links.
- Check whether `work/sym/a-chain-of-three-joints-straddles-dihedral-arm.md` is now closed in substance. If the straddle is gone at every link count, close it with this evidence; otherwise say where it still shows.
