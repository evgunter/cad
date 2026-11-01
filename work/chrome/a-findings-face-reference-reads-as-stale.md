---
id: a-findings-face-reference-reads-as-stale
kind: issue
title: clicking a face a check finding names says it no longer resolves
status: open
opened: 2026-10-31
priority: P1
cost: M
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

## Traced (2026-11-01)

**Extrude 1's payload does not change.** "payload differs" is the no-prior fallback, `Diagnosis::cause_not_in_evidence`, which names the minting node as a site and not as an edit. The viewer's resolve (`tree::pick_resolution`) is single-run, so it has no prior to diff. The sentence's falsehood is REFS' row `d-prime3-fallback-diagnosis-blames-the-sources-minting-node-for-a-re-pointed-copy`, with evidence added.

**What changed is the copy the face was picked on.** The pick is `(Place, Extrude1·Cap(End))`. Sketch-on-face's `repoint` sets that `Place`'s source to the union, and the union carries the same face as `From { read: <Extrude 1's body var>, of: Extrude1·Cap(End) }`. The held name is looked up in the copy's new table and misses. The add-profile form's held face is stranded identically.

**What a held selection does across this edit is decided on DOORS' ground.** In `work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md` `## Decided`, the takeover writes a re-seat through each name's unique carrier in one action, and "the same rebind map re-seats the viewer's held picks". Following descent automatically at resolve time is excluded (`[identity]`). Dropping the selection is excluded by the ratified state rule (`session::select::Standing`; SELECT-DESIGN §4). That row is P0 and its blocker, PR 5636, has merged. Evidence has been added there. This row closes when that one lands, plus a viewer row asserting both held picks stay resolved.

**Finding links (still open).** The bracket's finding links selected cleanly, and every staleness found here came from a takeover re-pointing the copy a pick reads. A finding's references are minted by the check run against the landed evaluation. A link should only go stale if the link outlives a landing after such an edit. The original report's `Assemble … payload differs` is the same no-prior fallback naming the Assemble as the minting node, so it says nothing about the cause either. That case was not reproduced.
