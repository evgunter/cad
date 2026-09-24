---
id: payload-attribute-names-are-spelled-twice-and-held-equal-by-nothing
kind: issue
title: 31 payload attribute names are spelled in two files and nothing holds the two equal
status: open
opened: 2026-09-15
priority: P3
cost: E
---

Found by CENSUS-ERRORS-ARRIVAL's sweep (2026-09-15), which asked a
question no earlier sweep on this crate asked: **which files under
`crates/pncad-py/src/` spell string literals at all, and which of them
is read by an instrument.** Three are —
`crates/pncad-py/src/tags.rs` (`TAG_INVENTORY`),
`crates/pncad-py/src/node_kind.rs` (`NODE_KIND_ROSTER`) and, from that
unit, `crates/pncad-py/src/errors.rs` (`ERRORS_MINTING_ITEMS`). The
four payload modules are not, and what they spell is Python-visible.

## The measurement

Every arm's payload record in `crates/pncad-py/src/edit_payload.rs`,
`check_payload.rs`, `mate_payload.rs` and `pick_payload.rs` carries a
`presence()` map built from an exhaustive destructure, whose rows are
`("word", field.is_some())`:

- **67 such rows across the four files** — 31 in `mate_payload.rs`, 21
  in `edit_payload.rs`, 12 in `check_payload.rs`, 3 in
  `pick_payload.rs` — naming **64 distinct words**.
- **All 67 currently pair the word with the identically named field**,
  so there is no live defect. What holds them equal is that someone
  wrote them that way: the destructure is exhaustive, so a NEW field
  is an `E0027` and arrival is the compiler's, but the WORD beside it
  is a free literal and `("nodes", node.is_some())` compiles.
- **31 of the 64 are spelled a second time** at a raise site under
  `crates/pncad-py/src/py/`, as the key of a payload tuple —
  `("referenced_by", node(payload.referenced_by))` in
  `crates/pncad-py/src/py/doc.rs`'s edit-boundary raise is one. Each of
  the 31 was checked at a real payload-tuple site, not sampled: the
  list is `count`, `escalate`, `expected`, `field`, `first`, `found`,
  `from_kind`, `index`, `inner_variant`, `input`, `instance`, `key`,
  `kind`, `margin`, `margin_high`, `margin_low`, `name`, `node`,
  `param`, `patch`, `path`, `pin`, `predicate`, `reason`,
  `referenced_by`, `slot`, `to_kind`, `triangle`, `value`,
  `value_path`, `zero`.

So an attribute name has **three spellings** — the Rust field, the
`presence()` literal, and the raise-site key — and nothing in the tree
holds any two of them equal.

## Why it costs something

The raise-site key is the word Python sees. The `presence()` literal
is what `present()` answers, and `present()` is read only by
`crates/pncad-py/src/tests.rs`, at five call sites in the four
`every_*_arm_projects_the_payload_it_carries` tests, which assert
which attributes an arm carries. So a drift between the two does not change
what Python receives — it makes those assertions pin a name Python
never sees, which is a pin that reports agreement about the wrong
word. That is this program's charter shape rather than a wire bug, and
it is why this is filed rather than fixed: the repair is a decision
about where the one spelling lives, not an edit.

`crate::py::typed_err` already asserts a raise's payload keys against
`ErrorClass::class_discriminant`'s attribute set for the two classes
that carry a discriminant (CENSUS-PY-RAISE-LITERALS' fix pass). That
device is the shape a fix would extend; what it does not reach is the
payload records, whose words are not a class's discriminant
vocabulary.

## What the sweep could not match

The `py/` half was found with a `\("([a-z_0-9]+)",\s` pattern over
`src/py/*.rs`, which **over-matches** — any two-tuple opening with a
lowercase literal — so the 106 distinct words it saw are an upper
bound and only the 31-word intersection is claimed. It **under-matches**
a payload key written across lines, or one passed as a `const` rather
than a literal, so 31 is a floor on the overlap rather than the whole
of it.

Territory: `crates/pncad-py/*` is LIB's fence and CENSUS's `keep_out`
announces its pncad-py rows there.
