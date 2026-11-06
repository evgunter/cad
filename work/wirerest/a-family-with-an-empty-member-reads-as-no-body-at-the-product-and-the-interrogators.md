---
id: a-family-with-an-empty-member-reads-as-no-body-at-the-product-and-the-interrogators
kind: issue
title: A family of bodies with an empty member reads as no body at the product gather, the at-rest checks and the interrogators
status: open
opened: 2026-11-06
priority: P2
cost: M
---



Found by the REFS lane that closed `a-family-of-boolean-results-cannot-be-read-whole-or-by-index` (2026-11-06), from reading; not pinned by a test.

A family whose members are each a body or a boolean's empty result (`F[k] − F[1]` over a row: member 1 is empty) evaluates to `ValuePayload::Family`, and since that fix it carries the family's name table over its bodies, each at its member's place (`eval/members.rs` `family_value`). Inside the evaluator every reader reads it through `NodeValue::member_bodies` (`eval/mod.rs`). Two readers outside it match `ValuePayload::Instances` alone and read such a family as no body:

- `product::sources_of` (`crates/editor-core/src/product.rs`) answers `None` for `Family`. So a product that is such a family refuses `ProductError::Read(NodeStanding::NotEvaluated)` in `product.rs` (the `sources_of(value) else` arm), which is wrong: the family was evaluated. And the at-rest checks over placements (`checks.rs`, the `for placement in doc.placements()` loops) `continue` past it, so its members are never checked and nothing says so. The fix is the `Split` arm's rule: read `member_bodies`, give each present body its member's index, and skip an absent one without shifting the rest. `resolve/pick.rs` calls `sources_of` too.
- `names::interrogate::output_body` (`crates/editor-core/src/names/interrogate.rs`) takes the payload alone and answers `NoBodies { payload: "family" }`. Its callers (`clearance.rs`, and the measure door's entity reads in `eval/wire.rs`) can now resolve a member's name against the family's table and then find no body there. It needs the value, or the members, so that it can read `member_bodies`.

Pin each with a document whose per-`k` cut has an empty member (`crates/editor-core/tests/g_family_of_booleans.rs` builds one).
