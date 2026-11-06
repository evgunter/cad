---
id: a-family-of-bodies-has-two-payloads
kind: issue
title: A family of bodies has two payloads: Instances, and Family with an absent member
status: closed
opened: 2026-11-06
priority: P1
cost: H
closed: 2026-11-06
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

## Closed

**One representation.** A family of bodies is `ValuePayload::Instances(Vec<Option<Framed>>)`: one place per member in index order, `None` at a member that is a boolean's empty result. `ValuePayload::Family` is a family of anything else, and it has no output. `family_value` is the one constructor of both, and every family it builds keeps its members; a placement over a family is evaluated once per member and never meets the family whole.

**Why this final state.** The three were weighed as landed code.

- **A third variant**, a gapped family beside `Instances`, would keep two spellings of one concept. Every reader would match both, and `kind_name` would need two words for one thing.
- **The bridge**, with `member_bodies` as the only public door, leaves the bad state representable. `Instances` is a public variant, so a reader that matches it alone still compiles and is still wrong. A `Family` value would also mean two things, told apart only by what its members hold, and a payload-only reader cannot see that: `output_body`, Python `Value`, the viewer.
- **`Instances(Vec<Option<_>>)`** puts the empty member in the type. A reader of a family of bodies matches one arm and has to handle `None`. No second payload is kept in step with the first.

**What changed.**

- `NodeValue::member_bodies` is deleted. Its readers match the payload: `whole_bodies`, the indexed read in `reads_projected`, and `own_bodies`.
- `ValuePayload::member_body` is now the public `one_body`, the one "is this one body" rule. The viewer's `one_body` copy is deleted, and the ladder copies in `pncad` export, the Python binding, `distribute`, the viewer's `body_reach` and test support, and the `diefillet` demo read it instead.
- `kind_name` says `"instances"` for every family of bodies. `"family"` now names only a family of non-bodies.
- `mate/corner.rs` `read_of` reads the first member that is a body. The corner walk only needs the names a per-member reader reads at each member, and any member spells them. The pose it writes is refused at the empty member, where it is evaluated.
- The tree readout question is filed on CHROME (`work/chrome/the-tree-row-of-a-family-with-an-empty-member-says-nothing.md`), as a fork about what the row shows.

**Pins.** `crates/editor-core/tests/g_family_of_booleans.rs`, for the payload's shape, the product, the interrogators and the corner walk, and `crates/pncad-py/tests/test_pattern.py`, for Python `Value`. Rows closed with it: `a-family-with-an-empty-member-reads-as-no-body-at-the-product-and-the-interrogators` and `python-value-bodies-drops-a-family-with-an-empty-member`. `a-selection-of-a-member-of-a-family-with-an-empty-member-refuses` stays open for its pin, which belongs with PR 6555.
