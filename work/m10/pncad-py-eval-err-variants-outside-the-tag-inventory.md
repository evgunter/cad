---
id: pncad-py-eval-err-variants-outside-the-tag-inventory
kind: issue
title: TAG_INVENTORY cannot see a refusal variant minted at an eval_err call site, and measure_unavailable is pinned nowhere
status: open
opened: 2026-09-04
---


Found while repairing the code-tier red in
`work/m10/pncad-py-tag-inventory-misses-two-measure-tags.md`. Not that
item's defect and not repaired with it: that one is two inventory
lines, this is a question about the gate's reach.

**The gate reads one file.** `the_whole_tag_table_matches_its_committed_inventory`
lexes `crates/pncad-py/src/tags.rs` and nothing else
(`crates/pncad-py/src/tests.rs:24`, "the shared Rust-source lexer:
`src/tags.rs` is READ by the tag-table…", and every panic in the reader
is spelled `tags.rs: …`). But a Python-visible refusal variant does not
have to come from `tags.rs`: `eval_err` takes the variant as a `&str`,
and eight call sites under `crates/pncad-py/src/py/` pass a STRING
LITERAL instead of a `tags::` function's answer. They mint four words:
`wrong_kind` (five sites, e.g. `crates/pncad-py/src/py/value.rs:586`,
`:622`, `:691`), `empty_boolean` (`value.rs:580`), `unknown_node`, and
`measure_unavailable` (`value.rs:721`). Those words reach Python as
`.variant` exactly like a `tags.rs` word does, and the inventory cannot
see any of them.

**Three of the four are covered by accident, and the fourth is not.**
`wrong_kind`, `empty_boolean` and `unknown_node` are each pinned in
`crates/pncad-py/src/tests.rs` anyway (3, 2 and 7 occurrences) and each
is named in `crates/pncad-py/pncad.pyi`. **`measure_unavailable` is named
in exactly one place in the repository** — the call site that mints it,
`crates/pncad-py/src/py/value.rs:721` — and in no test, no `.pyi`, no
Python test and no doc. It was added by M10-6 (PR 1685, commit
`7cb46c6ba`, MINOR-4) as the read door's own arm, in the same PR that
added the two `node_error_tag` values the sibling item is about; those
two the gate caught, this one it structurally cannot.

**Why it is probably not a bug today, and why it is still worth
recording.** The door that mints it — `Value.measure` on a
`min_clearance` — is unreachable from Python until measure AUTHORING
ships, which `crates/pncad-py/tests/test_binding_census.py` records as
the `B-MEASURES` census gap by name (`MeasureUnavailableAt`,
`MinClearanceRefusal`). So there is nothing a Python test could observe
yet, and the absence is the same "correct surface" answer the sibling
item got. What is NOT settled is the gate's reach: the next literal
variant added at an `eval_err` call site will be public Python
vocabulary that no inventory looks at, and the sibling item's whole
lesson is that a word which reaches Python with no gate over it is a
red waiting for an unrelated branch to find.

**Two ways to close it**, either of which is a small change:

1. widen the reader to lex the literal-variant `eval_err` call sites
   under `crates/pncad-py/src/py/` into their own inventory row, so
   every Python-visible variant is in exactly one table; or
2. rule that a literal at a call site is *deliberately* out of scope,
   say so where the reader's `tags.rs`-only scope is stated
   (`crates/pncad-py/src/tests.rs:24`), and pin `measure_unavailable`
   somewhere so the one uncovered word stops being uncovered.

Territory note: `crates/pncad-py/*` is LIB's fence and the gate is
LIB's (`434964dfa`), but LIB is not active and the uncovered word is
M10-6's, so this is filed to M10 on the same reasoning Ev gave for
re-homing the sibling item (2026-09-04).
