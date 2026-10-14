---
id: lineage-lists-are-five-newtypes-over-one-core
kind: issue
title: The checked lineage lists are five newtypes re-forwarding one core, tagged through their key type, with unchecked (new, from) edge rows left outside the Boolean
status: open
opened: 2026-10-13
priority: P3
cost: M
refs: [split-rows-are-chased-with-no-cycle-check, null-edge-copy-rows-are-checked-for-a-cycle-by-their-reader]
---

Found by the review of PR 5022 (`fuse/lineage-rows-refuse-at-append`),
which added the fourth and fifth lists over `crate::lineage`'s private
`Minted<K>` core.

**Five newtypes, one core.** `FaceDivisions`, `VertexCopies`,
`EdgeDivisions` and `ConeCopies` (`crates/topo/src/lineage.rs`) each
wrap `Minted<K>` and re-forward `push` and `root`, some of them `rows`,
`from` and `is_empty` too, each with its own copy of the same doc prose
(the freshness invariant, the `# Errors` block). Each new list repeats
the forwarding. One generic list over `K` and a list tag would hold the
methods and the prose once.

**The tag belongs to the list, not the key.** `Substitutions<K>` names
its list in a refusal through `LineageKey::ROWS`, a const on the key
type, so `Substitutions<VertexKey>` says `Fusions` and every
`Substitutions<FaceKey>` says `Substitutions`. A list's tag is a fact
about the list (what it records), which the key cannot know: the
`Minted` lists already pass the tag at the push, and PR 5022's
per-operand lists carry it as a field. One spelling would do.

**`EdgeSplits` stores each `(child, parent)` twice**
(`crates/topo/src/boolean/mod.rs`): once in its `rows`
(`EdgeSplit { operand, parent, vertex, child }`) and once in the
operand's `EdgeDivisions`. The rows are kept for the minted vertex
(`ops::input_cell`) and the forward walk (`ops::split_pieces`); a
`Minted` over edges whose rows carry the vertex would hold both.

**Unchecked `(new, from)` edge rows outside the Boolean.** The split
op's `SplitNaming::edge_pieces` (`crates/topo/src/splitting/finish.rs`,
read through `roots` and `Descent::of`) and the shell op's
`RimNaming::seam_pieces` (`crates/topo/src/shell.rs`, the same) are
division rows written as plain `Vec`s. Their readers are bounded
(`Descent` keeps a visited set), so a bad row cannot hang them, but it
can name a piece twice or under two parents, and nothing refuses it
where it is written. PR 5022's sweep looked for unbounded chases, so it
could not see these. They are the `EdgeDivisions` shape.

**One cycle, two pncad-py tags.** A `SplitLineageCycle` reaches Python
as `split_lineage` from the Boolean (`BooleanErrorKind::SplitLineage`,
`crates/pncad-py/src/tags.rs`) and as `split_lineage_cycle` from naming
(`NamingError::SplitLineage`). One value, one tag.

**`LineageRows`' arm names.** The arms mix list names (`Fusions`,
`Substitutions`, `FaceDivisions`, `EdgeDivisions`) with what a row
records (`NullEdgeCopies`, `ConeCopies`), and `word()` mixes the two
again ("vertex fusion", "copy-per-cone"). One rule for both.
