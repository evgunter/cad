---
id: select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate
kind: issue
title: SelectRefusal is #[non_exhaustive], so its variant coverage cannot be enforced from editor-core/tests
status: open
opened: 2026-09-15
---


Found by S-TINT's TINT-1 (`work/tint/assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums`),
whose spec names this as a residue for WIRE and forbids the lane from
touching `geompred.rs`.

## The finding

TINT-1 made the compiler the census for `assert_f6`'s ban lists in
`crates/editor-core/tests/display_contract.rs`: each error enum gets a
wildcard-free `match` from a value to its variant identifier, so a
variant added to the enum leaves that `match` non-exhaustive and the
test crate stops compiling. Six of the seven enums are ordinary and get
that guarantee.

**`SelectRefusal` does not.** It is declared `#[non_exhaustive]`
(`crates/editor-core/src/names/geompred.rs`, `pub enum SelectRefusal`,
around `:197-199`), and `editor-core/tests/` is a separate crate, so a
`match` there is REQUIRED to carry a wildcard arm and rustc checks
nothing about the arms above it. `select_refusal_variant` in
`display_contract.rs` therefore ends on

    other => panic!("`SelectRefusal` grew a variant with no arm here: {other:?}"),

and that arm is not a guard: it fires only if some case in the same file
happens to construct the new variant, which is exactly the vacuity
TINT-1 exists to close. The comment at the site says so plainly rather
than papering over it; this row is the repair.

## Why it lands on WIRE

`crates/editor-core/src/names/geompred.rs` is WIRE's territory
(`python3 scripts/work.py territory --files -` answers
"owned by wire"). The fix is a unit test BESIDE the enum, inside
`editor-core`, where `#[non_exhaustive]` does not apply to a local
`match` and rustc does enforce exhaustiveness:

- a `#[cfg(test)] mod` in `geompred.rs` with a wildcard-free `match`
  over `SelectRefusal` (a variant added to the enum then fails the
  crate's own build), and
- an assertion that the identifier set it names is the set
  `display_contract.rs`'s `SELECT_REFUSAL_VARIANTS` holds — or, more
  cheaply, a `pub(crate)`/`#[doc(hidden)]`-free in-crate census whose
  identifiers the contract test can compare itself against.

Either shape moves the census to the one place the attribute cannot
defeat it. Do NOT weaken the other six to match the exception, and do
not drop `#[non_exhaustive]` from the enum to make the test crate's
`match` exhaustive — the attribute is a deliberate API decision and this
is a test-siting problem, not an API one.

## Where it came from

`docs/TINT-1-SPEC.md` §*`SelectRefusal` is `#[non_exhaustive]` — a
stated exception, not a gap*. That spec is deleted at merge; this file
is the record.
