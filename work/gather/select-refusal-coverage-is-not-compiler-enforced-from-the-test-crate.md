---
id: select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate
kind: issue
title: SelectRefusal is #[non_exhaustive], so its variant coverage cannot be enforced from editor-core/tests
status: open
opened: 2026-09-15
priority: P3
cost: D
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
`match` and rustc does enforce exhaustiveness.

It has to be SELF-CONTAINED. The identifier roster in
`editor-core/tests/display_contract.rs` is a private item in a separate
integration-test binary and is invisible from `src/`, so nothing in
`geompred.rs` can compare itself against it. What the in-crate test owes
instead is the same two halves, both local:

- a `#[cfg(test)] mod` in `geompred.rs` holding a wildcard-free `match`
  over `SelectRefusal`. Inside the crate rustc DOES check it, so a
  variant added to the enum fails the crate's own build — the guarantee
  the test crate cannot have.
- one sample value per variant, and an assertion that the identifiers
  read off their `Debug` (the first token, as
  `test_utils::f6::variant_identifier` does it) are pairwise distinct
  and as many as the `match` has arms. That makes the sample list a
  roster the compile error sends an author to, rather than a list
  nothing checks.

That moves the census to the one place the attribute cannot defeat it.
The contract test in `display_contract.rs` stays as it is — it is about
RENDERINGS, and its own roster is welded to its cases by a set
difference, so the two do not need to see each other.

Do NOT weaken the other six enums in `display_contract.rs` to match this
exception, and do not drop `#[non_exhaustive]` from the enum to make the
test crate's `match` exhaustive — the attribute is a deliberate API
decision and this is a test-siting problem, not an API one.

## Where it came from

`docs/TINT-1-SPEC.md` §*`SelectRefusal` is `#[non_exhaustive]` — a
stated exception, not a gap*. That spec is deleted at merge; this file
is the record.
