---
id: borders-and-keeps-spell-a-lone-piece-bare
kind: issue
title: A face piece alone on its side is spelled bare, so it gains Borders or Keeps when a sibling appears
status: open
opened: 2026-10-06
priority: P3
parent: edge-pieces-are-named-by-their-ends
design: true
---


Found by the designer pair on PR 4134 (fork-log row 73).

**The flaw.** An edge piece alone on its side of a cut used to be spelled
bare, and it gained `Ends{..}` when a sibling appeared on that side. That
renamed it although nothing touched it. The run is in the PR 4134
reconciliation: on the pinned fixture, the untouched Below piece goes
from `SplitFragment{Below, rim}` to `+ Ends{..}`. Ev's ruling on PR 4134
gives every edge piece its `Ends`.

**The same rule is open on faces.** `Borders` (a boolean or union face
piece) and `Keeps` (a Split's same-side pieces) are added only when a
parent holds several pieces (`names/README.md` N2: "Pieces with equal
sets are N4's tie"). So a lone face piece is spelled bare, and a cut
elsewhere on its parent that adds a sibling renames it. The minting
sites are in `names/emit_topo.rs`, where the pieces are qualified; check
the lone shortcut there, beside `name_edge_pieces`.

**The question.** Should every face piece of a divided parent carry its
`Borders` or `Keeps`, as edge pieces now carry `Ends`? This is a design
question, because it renames every lone face piece in the goldens.
