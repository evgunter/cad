---
id: main-red-inert-deny-on-the-qualifier-enums
kind: issue
title: Main red: the deny_unknown_fields census finds the two Qualifier enums inert since 'nothing is ranked'
status: open
opened: 2026-10-26
---


Found by PR 5742's slow-set run (2026-10-26), on a head carrying main `d67ac3cbde`; neither file is in that PR's diff.

`test-utils` `deny_unknown_fields_census::every_deny_unknown_fields_attribute_has_a_named_field_to_deny` (`crates/test-utils/tests/deny_unknown_fields_census.rs:369`) is red. It names two sites:

- `crates/editor-core/src/names/nest_reference.rs:114`, `enum Qualifier`;
- `crates/editor-core/src/names/role.rs:1112`, `enum Qualifier`.

Each still carries `#[serde(deny_unknown_fields)]`, but neither declaration has a named field any more. 353f5163b6 ("names: an orientation is read, never chosen; nothing is ranked") removed their one named variant, `OrderAlong { rank, of }`. So the attribute denies nothing.

**Done when** both attributes are removed, any doc that reasons from them says what actually refuses, and the census row is green on main.
