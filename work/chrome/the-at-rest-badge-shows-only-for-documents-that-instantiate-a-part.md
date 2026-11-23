---
id: the-at-rest-badge-shows-only-for-documents-that-instantiate-a-part
kind: issue
title: the viewer shows the at-rest badge only for a document that instantiates a part; a ⊔ of the document's own bodies gets none though its census runs
status: open
opened: 2026-11-02
priority: P1
cost: M
---


Found by the PLACING sweep Ev asked for on PR 6281 (2026-11-02): "can you see if anything similar also got missed?" by the INTENT refactor. Read at `3bf15bef9c`; traced in code, not run unless said. Filed on CHROME's slate, because the at-rest badge is CHROME's ground. (PLACING orchestrator)

**Ruled.** `[assertions]`: "The census is each `⊔`'s door… a `⊔` the product does not read is checked all the same". `[one-way]`: "an assembly is a document; the kernel has no assembly type". Stage 5 retired A5's at-rest gate between instances.

**Survives** (sure).
- `assembly_shaped` / `puts_an_instance` (`crates/viewer/src/session.rs`) gate the `AtRestBadge` on a document that instantiates a part.
- So a `p ⊔ bolt` built from the document's own bodies gets no badge, though its census runs and reports interference.
- Pinned by `crates/viewer/tests/landing_gathers.rs` ("a part has no at-rest badge", three rows) and `an_assembly_shaped_document_lands_on_one_gather`.

**Owed.** The badge reads each `⊔`'s census, whatever the document instantiates. Re-baseline the pinning rows.
