---
id: the-cfg-test-rule-is-spelled-by-eight-text-readers
kind: issue
title: Eight text readers spell the #[cfg(test)] rule themselves
status: open
opened: 2026-11-10
---

Filed from PR 6694's delta review (NOTE 6). Eight guards that read
Rust as text each decide what a test item is by matching the literal
`"#[cfg(test)]"`, and each cuts the item out in its own way:

- `crates/topo/src/tier3_tests.rs:2359`
- `crates/topo/src/boolean/offer_rows.rs:2393` (`without_test_modules`)
- `crates/mesh/tests/all.rs:434`
- `crates/profile/tests/recourse_roster.rs:164` (`without_tests`, on
  `test_utils::source::attributed_extent`)
- `crates/editor-core/tests/node_standing.rs:607`
- `crates/editor-core/tests/msolve7_member_residue.rs:53`
- `crates/geom-brep/tests/kstats_escalation_channel.rs:342`
  (`cfg_test_bodies`)
- `tools/k-lint/tests/predicate_roster.rs:277` (`test_modules`)

The rule they share is "an item whose attributes include `#[cfg(test)]`,
spelled exactly so", and today it has eight spellings. Each one has its
own answer for a brace-less item (`mod tests;`), an attribute after the
`cfg`, and a compound `cfg(all(test, …))`. At the parse level the rule
has a single home: `source_scope`'s `is_cfg_test`, behind
`FnItem::in_test`. A text reader cannot call that.

**To decide.** Where the text-level helper lives. The candidate is
`test_utils::source`: a `test_items(code) -> Vec<Range<usize>>` built on
`attributed_extent`, which `recourse_roster.rs` already calls. Each
reader would then move onto it. The other option is to move the readers
onto `source-scope` where they already need a parse. Then pin the
spelling once, for example with a row in
`crates/test-utils/tests/reader_census.rs`.
