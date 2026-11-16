---
id: main-red-rustdoc-borders-links-to-obstacles-split
kind: issue
title: main red: rustdoc rejects editor-core names/borders.rs's link to Obstacles::split, which is now split_corner
status: closed
pr: 6708
closed: 2026-11-10
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `rustdoc (gate, every root)`: `unresolved link to Obstacles::split` at `crates/editor-core/src/names/borders.rs:460` (`/// ([`Obstacles::split`]), so a wall is a face across an edge at the`). `Obstacles`' method is `split_corner` (`borders.rs:465`).

It is new since the 11-07 nightly (run 42342466395). Not BOXES': no BOXES merge touches `crates/editor-core/src/`.

(BOXES nightly-reds triage, 2026-11-09)

## Closed

2026-11-10. Already fixed on main by ONECOPY's PR 6708
(`onecopy/doc-gate-clean`, commit `94feb337b4`, "main passes
doc-gate's rustdoc lints"). That PR re-pointed every intra-doc link
the workspace rustdoc pass rejected, and this one with them:
`origin/main`'s `crates/editor-core/src/names/borders.rs` no longer
links `Obstacles::split`, and its method is `split_corner`
(`borders.rs:465`). Closed from the BANDREACH/SEAL/REFS session's
survey of the main-red rows, with no code change.
