---
id: a-family-of-bodies-has-two-payloads
kind: issue
title: A family of bodies has two payloads: Instances, and Family with an absent member
status: open
opened: 2026-11-06
priority: P1
cost: H
---



Found by the review of PR 6557, which closed `a-family-of-boolean-results-cannot-be-read-whole-or-by-index` (2026-11-06).

**The state.** A family of bodies is held two ways. When every member is a body, it is `ValuePayload::Instances(Vec<Framed>)`. When at least one member is a boolean's empty result, it is `ValuePayload::Family`: the bodies stay in `NodeValue::members`, and the one output names each body at its member's place. `ValuePayload::Family` also means a family of anything else (profiles, measures, splits, families), which has no output.

PR 6557 bridged the two inside `editor-core` with one door, `NodeValue::member_bodies`. Every evaluator reader of a family of bodies goes through it: the whole reads in `wire_combine` and `wire_assemble` (`whole_bodies`), the indexed read in `reads_projected`, and `own_bodies`. It also gave the per-body ladder one rule, `ValuePayload::member_body`.

Every reader that sees only the payload is still outside the bridge.

**The final state to weigh** is one representation. One candidate is `Instances(Vec<Option<Framed>>)`, `None` at an absent member, with `Family` kept for families of non-bodies. `member_bodies` then reads the payload, and the payload-only readers below take one shape. `ValuePayload` is public, so this reaches `viewer`, `pncad-py` and about 20 test files that match `Instances(..)`. Two alternatives to weigh against it: a third payload variant, or keeping the bridge and making `member_bodies` the only public way to read a family's bodies.

**Hit list** (the reviewer's, at `3bf9231062` plus the fix pass):

- **Payload-only readers that see an absent-member family as no bodies.**
  - `names/interrogate.rs` `output_body`, and every caller through `Evaluation::read_body`. Its own row: `work/wirerest/a-family-with-an-empty-member-reads-as-no-body-at-the-product-and-the-interrogators.md`.
  - `product.rs` `sources_of` (same row).
  - `ValuePayload::kind_name` (`eval/mod.rs`) spells one concept two ways in user-facing refusals: `found: "family"` for a family of bodies with an empty member, `"instances"` for one without.
  - viewer `session/refuse.rs` `one_body`.
  - viewer `tree.rs`'s readout gives no readout for a family with an empty member. Its `Family` arm answers only a clearance assertion, and no arm says a member is empty.
  - Python `Value`: `work/bind/python-value-bodies-drops-a-family-with-an-empty-member.md`.
- **Copies of the `Body | Boolean::Body` / `Empty` ladder.**
  - In `editor-core`, the fix pass routes these through `member_body`: `placeable_operand`, `read_body` (through `placeable_operand`), `payload_reach`, `wire_place`, `wire_assemble`, `own_bodies`'s first arm, `sources_of`, and `output_body`.
  - The viewer's `one_body` (`session/refuse.rs`) is a copy outside the crate, and cannot reach the crate-private rule.
- **`mate/corner.rs` `read_of`** reads a family through its first member and refuses `PinRefusal::NotABody` when member 0 is empty and the others are bodies. Under one representation, it would read the first present member, or refuse naming the empty one.

Pin with the empty-member document in `crates/editor-core/tests/g_family_of_booleans.rs`.
