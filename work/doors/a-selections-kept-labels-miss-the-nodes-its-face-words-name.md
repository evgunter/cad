---
id: a-selections-kept-labels-miss-the-nodes-its-face-words-name
kind: issue
title: A selection keeps three nodes' labels, not every node and profile step its face's words name
status: open
opened: 2026-11-01
priority: P3
cost: E
---


## Finding

Found while routing the Properties header and the add-profile form's
held face through `tree::PickedName` (`doors/face-words`). Both say the
pick with a speaker `.or_held(DocSession::selection_said())`, so a node
the landed document no longer holds is said by the label it had. That
snapshot is `DocSession::spoken_now` (`crates/viewer/src/session.rs`),
which keeps `Selection::nodes` — the name's outermost node, the minting
feature and the drawn node — and nothing else.

A face's words name more than those three: each join's node ("…,
joined at Union d1aa from Place 3218"), each node a cited name ends at,
and the facts a step is said by (`loop 0 step 2`, a feature's sole
profile: `HeldNodes`' `facts`). Once a later edit deletes one of those,
the header says it by its tag (`node <tag>`, `the profile step <tag>`)
where the tools' kept labels — `MateTool` and `MirrorTool` keep
`held_by(&PickedName::…, doc)` — would still say it.

The fix is the one the tools use: `spoken_now` keeps
`held_by(&PickedName::face(face), doc)` for an entity selection (with
the landed fallback it applies per node today), so the selection keeps
what its sentence names rather than a list beside it.
