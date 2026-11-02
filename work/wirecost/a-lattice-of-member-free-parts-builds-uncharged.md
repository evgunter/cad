---
id: a-lattice-of-member-free-parts-builds-uncharged
kind: issue
title: A lattice of member-free parts is charged nothing and builds up to 2^depth times
status: open
priority: P2
cost: M
design: true
opened: 2026-11-02
---

Found on PR 6305 (`wirehold/a-family-has-no-member-ceiling`), whose member budget charges a part at every build its descent makes.

**The shape.** Levels of two part documents, each instancing both documents of the level below, over documents that list no member (no family, so no index value is bound). The build shares a row within one document and not across two (`crates/editor-core/src/eval/parts.rs:850`, `resolve_and_evaluate` at `:853`, the per-document table `Entered::reached` at `:1179`, `next_unreached` at `:1230`), so a document `d` levels down is built once per path to it: 2^d times, and a lattice `d` deep builds `2^(d+1) − 2` documents. The only bound is `MAX_DEPTH = 1024` (`parts.rs:101`): up to about 2^1024 builds. The member budget charges listing steps (`docs/DESIGN.md:2214`, "the index values bound while families are listed"), and these documents list none, so the charge is 0 at every build (`PartStore::charge`, `parts.rs:713`) and nothing stops the run but cancelation. This was already unbounded on main; PR 6305 does not widen it. A lattice whose documents do list members is charged its builds and refuses (`member_budget::a_lattice_is_charged_its_builds`).

**Why the build is not shared run-wide.** Per-parent sharing is deliberate: commit `2871d662e4` ("depth: a nested cache never descends; one census of the asks…", branch `edit/part-depth-bound`) states that each row is "decided against that document's own chain", and `part_depth_bound::below_the_top_a_documents_rows_are_the_ones_its_own_evaluation_produces` (`crates/editor-core/tests/part_depth_bound.rs:391`) pins it. A row depends on its path, not on its `DocRef` alone: a cycle refusal (`Entered::enter`) names the loop from where the path first reached it, and `DepthExceeded` depends on the depth it is reached at. No ratified text records the rule.

**Candidate fixes** (design, so designers first):
1. *A step per build*: charge each build one step besides its listing (built and withdrawn on PR 6305's review). It bounds the lattice exponentially and refuses honestly, but it revises DESIGN's "the budget counts the index values bound while families are listed", so it needs Ev.
2. *Path-independent run-wide sharing*: share a nested row across parents when its subtree is path-independent, meaning its descent met no cycle refusal and its depth plus the subtree's height stays under `MAX_DEPTH`. Such a row is what its own evaluation produces wherever it is reached. Builds and charge then fall to once per distinct `DocRef`, which DESIGN's wording already covers, with path-dependent subtrees still built per parent and bounded by their refusals. This relaxes the per-parent mechanism behind the pinned invariant, and a dense lattice under depth 1023 still needs the height check.
