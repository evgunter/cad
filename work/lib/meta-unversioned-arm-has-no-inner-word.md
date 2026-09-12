---
id: meta-unversioned-arm-has-no-inner-word
kind: issue
title: EditError's meta_unversioned arm carries a shape refusal with no Python word
status: closed
opened: 2026-09-08
refs: [LIB-DOORS-1]
closed: 2026-09-08
---



Found by LIB-DOORS-1, which projects every `EditError` arm's payload
as attributes and pins 57 of the 58 arms by construction.

`EditError::MetaUnversioned { name, key, error }`
(`crates/editor-core/src/edit.rs`) is the one arm the pin cannot
build. Its `error: MetaVersionError` is a three-arm shape refusal —
`NotAMap`, `MissingVersion`, `VersionNotInt`
(`crates/editor-core/src/meta/mod.rs:150`) — and:

- **The arm has no inner word.** `edit_inner_variant_tag`
  (`crates/pncad-py/src/tags.rs`) answers `None` for it, so a Python
  caller who reads `variant == "meta_unversioned"` learns that a D7
  producer convention was broken and not WHICH of the three ways.
  That is the arm half's question, ruled and executed by LIB-ARMS,
  and this arm was left where it was.
- **The façade does not carry the type.** `MetaVersionError` is in
  `NOT_CARRIED` (`crates/pncad/tests/all.rs`), so `pncad-py` — which
  depends on `pncad` alone — cannot name it to write a tag map, and
  cannot construct one to build the arm for a test either. LIB-DOORS-1
  carried `AttrKind` and `ExprPath` out of that list under the
  façade's payload rule; it did not carry this one, because the rule
  moves a payload a projection needs and this projection does not
  need it — `name` and `key` are the arm's leaf fields and both
  cross.

The payload half is therefore CLOSED for this arm and the arm half is
not: `name` and `key` are attributes like every other arm's, and the
shape refusal is in the message only.

What closes it: carry `MetaVersionError` at
`crates/pncad/src/document.rs` under the payload rule, add a
three-literal `meta_version_error_tag` to `crates/pncad-py/src/tags.rs`
with its `TAG_INVENTORY` row, point `edit_inner_variant_tag`'s
`MetaUnversioned` arm at it, and add the arm to
`every_edit_arm_projects_the_payload_it_carries` — which then covers
58 of 58. A census row moves with it (`MetaVersionError` becomes
`EditError.inner_variant`, the `ProgramRefusal` precedent). There is
no Python door for `SetAppearanceMeta`, so the row is a Rust
construction pin either way.

## Closed (2026-09-08, LIB-CUR7)

Every clause of "what closes it" executed, and nothing had to be
re-argued: the payload rule already said carry it, and what had held
it back was that the arm had no word to mint the type FOR. It has one.

`MetaVersionError` is carried at `crates/pncad/src/document.rs`
(`NOT_CARRIED` 85 -> 84, and the family entry that held it is replaced
by the paragraph saying what moved). `meta_version_error_tag` speaks
`not_a_map` / `missing_version` / `version_not_int` — the kernel's own
three arms — with its `TAG_INVENTORY` row and the delegate row on
`edit_inner_variant_tag`, whose `MetaUnversioned` arm now answers it.
`every_edit_arm_projects_the_payload_it_carries` covers 58 of 58.
Census row: `MetaVersionError` -> `EditError.inner_variant`, the
`ProgramRefusal` precedent.

The measurement the row asked for: `meta_unversioned` said that a D7
producer convention was broken and now names which of the three ways,
so a caller repairing a stored value knows whether to wrap it in a
map, add the version, or make the version an integer. No shipped
`variant` or `inner_variant` value moved — the arm's inner word was
`None`. There is still no Python door that mints a
`SetAppearanceMeta`, so the reach is the Rust construction pin, which
is what the row predicted.
