---
id: are-the-upstream-flip-and-recipe-edit-rungs-reachable-from-emitted-names
kind: issue
title: Are the Upstream flip and RecipeEdit rungs still reachable from names the emitters mint, now that edge pieces cite their cutters?
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [edge-pieces-are-named-by-their-ends]
---

## What

N5's upstream scope answers `Upstream { node, cause }` for a cause at an
ancestor of the minting node that is not on the name's derivation path.
Edge pieces now cite their end vertices (`Ends`), which cite the faces
that cut them, so the cutter is on a piece's path, and face pieces
already cited their divider walls (`Borders`). The real-document rows of
`crates/editor-core/tests/resolve_upstream_scope.rs` that reached the
flip and recipe-edit lanes through ranked edge pieces now reach them
only on hand-built tables (names that mention no partner); the
structural-parameter lane still answers on a real document (a `Part` is
not mentioned).

## The question

Is there a name the emitters mint, that can vanish, whose cause sits at
an ancestor off its path — and if none, whether those two lanes are
still worth their rungs. Measure over the corpus before deciding.
