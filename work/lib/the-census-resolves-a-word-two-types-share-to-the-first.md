---
id: the-census-resolves-a-word-two-types-share-to-the-first
kind: issue
title: The binding census resolves a curated word two types in one crate share to the first one declared, so Placed is checked as coincide::Placed and mate::pin::Placed's fields go unchecked
status: closed
opened: 2026-10-20
priority: P3
cost: E
closed: 2026-10-20
---


Found by INTENT stage 3 D (2026-10-20), when `pncad::document` came to re-export `editor_core::Placed` (the placement gesture's answer, `mate::pin::Placed { mate, values, by }`).

`crates/pncad-py/tests/test_binding_census.py`, `curated_declarations`, takes `decls[0]` of the declarations the resolver (`scripts/payload-rung-sweep.py`) answers for a word in the carrying crate. `editor-core` declares two `pub` types named `Placed`: `coincide::Placed` (an enum, `Instance`/`World`: which copy a coincidence row's cell is on) and `mate::pin::Placed` (a struct). The resolver is crate-aware but not path-aware, so the census reads the curated `Placed` as the enum: its member rule demanded `Placed::Instance` and `Placed::World` of Python's `Placed` (which mirrors the struct), and never checks the struct's fields. D lists the two arms in `MEMBERS_NOT_BOUND` with that reason; the gap is the resolution, which would treat a renamed or removed field of `mate::pin::Placed` the same way.

**Done when** the census resolves a curated name through the `pub use` path that exports it (so `pncad::document::Placed` is `mate::pin::Placed`), the two `Placed::*` entries leave `MEMBERS_NOT_BOUND`, and a row pins a same-word pair in one crate resolving to the exported one.

## Closed

INTENT's stage 3 D lane closed it (2026-10-20). `scripts/payload-rung-sweep.py`'s `Resolver` follows a curated name's `pub use` chain to the declaration it names (crate root, `self`/`super`, child modules shadowing a same-named crate, renames, globs), and `resolve_curated` is what both the sweep and the census read. The census now checks `pncad::document::Placed` as `mate::pin::Placed` (`mate`, `values`, `by`), the two `Placed::*` entries are gone from `MEMBERS_NOT_BOUND`, and the sweep's selftest pins a same-word pair in one crate resolving to the exported one. A payload's curation is asked of its declaration, not its word, which surfaced two rungs the shared word had hidden: `coincide::Recourse` (argued, `document::coincide`) and `topo::coincidence::Relation` (moved from cross-list to uncurated, still filed on `payload-rungs-behind-struct-payloads`).
