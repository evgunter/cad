---
id: a-findings-face-reference-reads-as-stale
kind: issue
title: clicking a face a check finding names says it no longer resolves
status: open
opened: 2026-10-31
priority: P1
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

## Reproduced (2026-10-31; Ev: P1)

The message reproduces through a **held selection**, not a finding link: a face of a plate picked before adding a boss on it stays selected, and after the boss lands Properties reads "this face no longer resolves in this evaluation: the recorded reference disagrees with the recipe as it stands on the derivation path (Extrude "Extrude 1" (12b76f99b7dd)'s payload differs). Recourse: rebind it". In the same document, clicking the face links of the bracket's `unproven-coincidence` findings selected each face cleanly. So the selection that outlives an edit is the case to fix first; whether a finding link ever goes stale on its own is still open.

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`:

```text
click "New…"
click "document name"
type "bracket"
click "Create" 0
click "Add feature"
click "rectangle"
click "0.0100" 0
type "0.06"
key Enter
click "0.0100" 0
type "0.04"
key Enter
click "Add profile"
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
click "on the picked face"
click "circle"
click "0.0000" 1
type "-0.012"
key Enter
click "0.0100" 0
type "0.003"
key Enter
click "Add profile"
tree
dump
```
