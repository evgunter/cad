---
id: rustdoc-d-warnings-breakages-outside-the-doc-gate
kind: issue
title: Pre-existing rustdoc -D warnings breakages the doc gate does not render
status: open
opened: 2026-09-04
---

Found by DOCM-1 (PR #1829) running `RUSTDOCFLAGS="-D warnings" cargo doc
--no-deps` on `topo` and `editor-core` — a pass `scripts/doc-gate.sh
--pr` does NOT make (it renders `--workspace` with its own flags and is
green), so none of these gate today and none is DOCM-1's:

- `crates/topo/src/boolean/mod.rs:31` — unresolved link to
  `SweepStrategy::Idealized` (the variant is gone or renamed).
- `crates/topo/src/boolean/contain.rs:70`, `:532`, `:536`, `:555` —
  public docs link to private items (`boundary_pre_pass`,
  `super::solid_contain::point_on_wall_in_face`,
  `curved_boundary_containment`, `super::solid_contain::cylinder_chart_trim`).
- `crates/topo/src/boolean/rest.rs:492` — public docs link to the
  private `face_plane`.
- `crates/editor-core/src/eval/mod.rs:243` — `contacts` links to the
  private `crate::eval::wire::OpOut`; `:4071` — unresolved link to
  `crate::report`.
- `crates/editor-core/src/node.rs` `payload_names` — links to the
  private macro `name_free_node`.

Either the gate should render the private-item links too (the
`--document-private-items` question the doc-gate header may already
answer) or these eight sites want their links rewritten; whichever, a
reader following any of them today lands nowhere.
