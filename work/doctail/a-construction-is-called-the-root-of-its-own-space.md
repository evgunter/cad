---
id: a-construction-is-called-the-root-of-its-own-space
kind: issue
title: Code and tests call a construction the root of its own space, a sense of root D10 names no word for
status: open
opened: 2026-10-28
priority: P4
cost: E
---


Left by the roots rename (PR 5919, `the-world-still-speaks-of-roots`) as a second sense of "root" with no single D10 word. Sites the lane listed: `crates/editor-core/src/mate/member.rs`, the tests `intent_s3_d_placements`, `name_words_corpus`, `instance_authoring` and `assembly_display` (`gesture_on_a_non_root_copy`), and `on=root` locals in `test_north_star` and the `emit_*` tests.

DESIGN `[variables.kinds]` / `[placement.spaces]` say each construction's body "is born in a space of its own (`a.space`)", minted by the construction. So the relation is "the construction whose own space it is" (`Doc::space_of`'s `space_definer`). A word for it is a naming call: settle it from the space clauses, or ask, before renaming. Test-local names may simply say what they hold.
