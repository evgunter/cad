---
id: a-findings-face-reference-reads-as-stale
kind: issue
title: clicking a face a check finding names says it no longer resolves
status: open
opened: 2026-10-31
priority: P0
---


## Finding

Ev (in chat, 2026-10-31): clicking on the things a check finding names
"seems to always say"

> this face no longer resolves in this evaluation: the recorded
> reference disagrees with the recipe as it stands on the derivation
> path (Assemble 2fb2165d1430's payload differs). Recourse: rebind it

"or similar". The finding was the unproven-coincidence one quoted in
`work/coincide/a-check-findings-text-restates-every-path.md`, whose faces
are read `through read <tag> at Assemble 2fb2165d1430`. The selection
is offered a reference whose derivation path runs through the
`Assemble`, and the resolver reports that node's payload as changed
(`ResolveError` diagnosis `NodeChanged`, worded in
`crates/editor-core/src/resolve/mod.rs`; drawn by the Properties pane's
verdict, `crates/viewer/src/pane/properties.rs`). Nothing was edited
between the finding and the click, so a reference recorded by the check
run and resolved against the same document should resolve.

**Not yet reproduced.** With `examples/drive.rs`, two extruded
rectangles at the origin with the second placement added to the product
raise an at-rest overlap finding; its one link (`Extrude a59299af4a92`)
selects the node correctly, and the faces it names are inside one text
block with no separate link, so no face reference could be clicked.
The next step is Ev's document, or a script that reaches an
unproven-coincidence finding.

**Hypothesis to check first**: the reference is minted against the
`Assemble` as the check run saw it (the landed evaluation) and resolved
against another version of it — for example a product slot rewritten
by "Add to the product" — so its payload digest differs. If so, the
fault is in which document the finding's references are recorded
against, owned by `doctail` (`crates/editor-core/src/resolve/`), and
this row moves there.
