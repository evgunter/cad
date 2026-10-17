---
id: tour-certified-chaintol-three-links-refuses-at-a-transform-not-dihedral-arm
kind: issue
title: demo-tour certified panics at chaintol: at three links the symbolic lane's first refusal is a transform's re-certification, not dihedral_arm (red on main)
status: open
opened: 2026-10-17
---


`demo-tour certified` exits 101 on main as of merge `4760a784b2`. INTENT
stage 3 B's branch fails it identically, at the same node.

- The panic is at `demos/tour/src/chaintol.rs:502`: "the table says the
  first refusal at 3 link(s) over the whole study is `dihedral_arm`".
- What refuses first instead is `node caf4a2e4ca41 — the transform op
  refused: an edge the map moved failed re-certification: the surfaces'
  tangent planes at sample 4 are undefined: a surface's gradient is zero
  or undefined somewhere over the enclosure`. This is the `Sym<Interval>`
  row at 3 links. The `Interval` row at 3 links refuses at `node
  58f0bde5734b` on the start-endpoint residual.
- The study's header table and the `(Lane::Symbolic, _)` arm of its
  check name `dihedral_arm` at three links, so either the transform's
  re-certification regressed or the table is stale.
- The row is `demos/tour/tests/eps_regression.rs`
  (`certified_cells_run_green_at_default_eps`), which runs the binary.
- The chain-tolerance study is the last certified cell. The plate's
  tolerance study before it runs green on both sides.
