---
id: a-family-with-an-empty-member-reads-as-no-body-at-the-product-and-the-interrogators
kind: issue
title: A family of bodies with an empty member reads as no body at the product gather, the at-rest checks and the interrogators
status: closed
opened: 2026-11-06
priority: P2
cost: M
parent: a-family-of-bodies-has-two-payloads
closed: 2026-11-06
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

## Closed

**Closed by the unification row** (`a-family-of-bodies-has-two-payloads`). A family whose members are each a body or a boolean's empty result is now `ValuePayload::Instances`, `None` at the empty member, so neither reader below needs the members any more.

- `product::sources_of` reads the payload. Each present body is tagged with its member's place, and an empty member contributes nothing and shifts no later place (the `Split` arm's rule). The product's `gather` reads each body's carry (its census, its `censused` flag) at that place, not at its position in the list of present bodies; read at the position, the body after an empty member took the empty member's carry and the product reported a false unrecorded contact. So the product gathers such a family with each member's own records, and the at-rest loops over placements reach its members.
- `names::interrogate::output_body` answers a present member's body at its place, and `NoSuchBody` at an empty member's place (an empty half's answer). Every caller above reads through it, directly or through `Evaluation::read_body`.

**Pinned** by `g_family_of_booleans::a_family_with_an_empty_member_is_its_bodies_at_the_product_and_the_interrogators`, and by `g_family_of_booleans::an_empty_member_shifts_no_carry_at_the_product` for the carry at each place. It covers the product of the family's world copies (two solids), `read_body` at each of the three places, and `face_carrier_kind` on member 2's face by the family's name for it. The callers listed above are not pinned one by one: clearance, `select`, `geompred`, coincide replay, and the viewer's `combine` and `session`. Each reaches the body through the one door the row pins.
