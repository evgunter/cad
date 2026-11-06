---
id: a-family-with-an-empty-member-reads-as-no-body-at-the-product-and-the-interrogators
kind: issue
title: A family of bodies with an empty member reads as no body at the product gather, the at-rest checks and the interrogators
status: open
opened: 2026-11-06
priority: P2
cost: M
---



Found by the REFS lane that closed `a-family-of-boolean-results-cannot-be-read-whole-or-by-index` (2026-11-06), from reading; not pinned by a test. Both readers below now classify one body through `ValuePayload::member_body`, but neither reads a family's members. The unification row `work/refs/a-family-of-bodies-has-two-payloads.md` would retire this row, as would fixing these two readers.

A family whose members are each a body or a boolean's empty result (`F[k] − F[1]` over a row: member 1 is empty) evaluates to `ValuePayload::Family`, and since that fix it carries the family's name table over its bodies, each at its member's place (`eval/members.rs` `family_value`). Inside the evaluator every reader reads it through `NodeValue::member_bodies` (`eval/mod.rs`). Two readers outside it match `ValuePayload::Instances` alone and read such a family as no body:

- `product::sources_of` (`crates/editor-core/src/product.rs`) answers `None` for `Family`. So a product that is such a family refuses `ProductError::Read(NodeStanding::NotEvaluated)` in `product.rs` (the `sources_of(value) else` arm), which is wrong: the family was evaluated. And the at-rest checks over placements (`checks.rs`, the `for placement in doc.placements()` loops) `continue` past it, so its members are never checked and nothing says so. The fix is the `Split` arm's rule: read `member_bodies`, give each present body its member's index, and skip an absent one without shifting the rest. `resolve/pick.rs` calls `sources_of` too.
- `names::interrogate::output_body` (`crates/editor-core/src/names/interrogate.rs`) takes the payload alone and answers `NoBodies { payload: "family" }`. Its callers can now resolve a member's name against the family's table and then find no body there. It needs the value, or the members, so that it can read `member_bodies`. Its callers, directly or through `Evaluation::read_body` (`eval/mod.rs`, which hands a non-split payload to `output_body`):
  - `clearance.rs` (its two `output_body` calls);
  - `eval/wire.rs`: the entity read in `body_at`, and the f64 rebuild lane's `output_body(&value.payload, index)`;
  - `names/select.rs` (the `.read_body(var, ent.body)` site);
  - `names/geompred.rs` (`ev.read_body(var, at.body)`);
  - `names/interrogate.rs` (`ev.read_body(var, index)??`);
  - `coincide/replay.rs` (`ev.read_body(*input, *body).ok()?.ok()?`). That one discards the refusal silently: a replay over such a family drops its rows without a word.
  - viewer: `combine.rs` (`value.payload.output_body(u32::from(port))`) and `session.rs` (`.payload.output_body(body).ok()?`). These are on viewer ground, but they read through this door.

Pin each with a document whose per-`k` cut has an empty member (`crates/editor-core/tests/g_family_of_booleans.rs` builds one).
