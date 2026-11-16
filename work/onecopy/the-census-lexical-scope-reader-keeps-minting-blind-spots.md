---
id: the-census-lexical-scope-reader-keeps-minting-blind-spots
kind: issue
title: the census's lexical scope reader keeps minting blind spots; read scopes with a parser
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-09
pr: 6694
---

Filed from PR 6640's FULL review (`onecopy/census-rosters-2`), as the class behind its FIX.

**The class.** `geom-core/tests/flagged_census.rs` reads Rust scopes lexically: `use` trees (`use_items`), `fn` heads and their `impl`/`#[cfg(test)]` membership (`fn_heads`), closure heads (`closure_parameter`), `let`/`fn`/parameter bindings and their blocks (`spellings`, `enclosing_block`, `is_impl_or_trait_body`). Three review passes in a row (6603, 6627, 6640) closed one blind spot each and opened another:
- 6627 made the "other meaning" exemption file-wide, and a field silenced a call;
- 6640's first version read a closure by a comma and an odd count of `|`, so a match or-pattern or a bitwise or silenced a call; a method in an `impl` scoped its whole block; a closure parameter scoped to its block's end.

Each fix is right; the reader stays a list of shapes it knows.

**Options.**
1. **Read scopes with a parser.** `syn` 2 is already in `Cargo.lock` through dependencies. As a dev-dependency of `geom-core` (or of a test-support crate), it gives the census items, `use` trees, closures, blocks and patterns as they parse, and the census keeps only its question. The cost is a dev-dependency and a rewrite of the census's readers (and the least-band gate's, if it moves to Rust).
2. **Keep the lexical reader, and make its header's blind-spot list the contract.** Every shape it does not know is named there and pinned by a fixture, and a new blind spot is a header line plus a fixture. That is cheap, but it is the treadmill this row is filed against.

Not decided: the choice is the orchestrator's (and the dependency's, Ev's, if it binds future work). Do not do the parser move in 6640.
