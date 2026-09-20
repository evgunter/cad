---
id: quantity-fmt-error-display-row-is-a-verbatim-copy-of-assert-f6
kind: issue
title: quantity's FmtQuantityError display row is a verbatim copy of assert_f6's body, panic wording included
status: open
opened: 2026-09-19
priority: P4
cost: E
---

Found by S-DUP's F6 fold of `crates/viewer/tests/panel_edits.rs`, whose
census re-derivation reached this file. **It sits on S-FIX's slate
rather than S-DUP's because S-DUP claims no paths** (`work/dup/plan.md`,
"Territory — none"); `python3 scripts/work.py territory` names S-FIX as
this path's sole owner.

## The finding

`crates/quantity/src/tests.rs`,
`fmt_quantity_error_display_names_its_content_not_its_struct`, spells
all four F6 clauses inline — **and spells them with
`test_utils::f6::assert_f6`'s own panic wording, character for
character**:

    "{err:?} renders as {shown:?}, missing {want:?}"
    "{err:?} renders as {shown:?} — that is the variant name, i.e. a struct dump"
    "{err:?} renders as {shown:?} — that is Debug punctuation, not a sentence"
    assert_ne!(shown, format!("{err:?}"));

That is not a paraphrase of the shared door; it is the door's body
transcribed. Every existing row in this class is about a **ban list**
that mirrors an enum; this is the whole predicate, one clause at a
time, and no sweep for `dumps`, `assert_f6`, `guts` or `contains('{')`
in a `tests/` directory reaches it, because it is inline mid-file in a
`src/` unit-test module.

## The drift it already carries

`dumps` here is a one-element list: `!shown.contains("NonFinite")`.
`FmtQuantityError`'s other arms' identifiers are not banned, so a
rendering that leaked a **sibling** arm's name would pass — the same
approximation S-DUP folded out of `panel_edits.rs` and proved mattered
by planting a sibling leak (green under the per-arm form, red under the
roster form). Unlike `viewer`'s, this list also has no compiler behind
it: a variant added to `FmtQuantityError` leaves the row asserting
about one identifier and saying nothing about the new one.

## What makes it awkward, and why it is a decision rather than a fold

`crates/quantity/Cargo.toml` has an **empty `[dependencies]`** with a
comment stating that as the design: *"the one home that serves both is
a leaf crate with no dependencies."* Folding onto
`test_utils::f6::assert_f6` adds `test-utils` to `[dev-dependencies]`
(where `proptest` already sits), so the stated leaf property is not
touched — but it is the kind of manifest edge that wants saying out
loud rather than assuming, and it is S-FIX's to decide, not S-DUP's.

Worth checking in the same breath: `crates/pncad-py`'s wheel closure is
the **non-dev** dependency closure (`docs/prompts/implementer-discipline.md`),
so a dev-dependency edge from `quantity` does not move which crates buy
the python suite. That is the reasoning, not a measurement — re-take it
at the merge base.

## Instrument that found it, and its blind spot

`git grep -nE '(contains|find|matches|starts_with)\s*\(\s*["'"'"']\s*\{'`
over **every tracked file, no path argument** — the brace literal in any
spelling, which is the one clause every full copy of F6 must contain.
Its blind spot: a copy that bans only identifiers and never the brace.
A name-shaped sweep (`assert_f6`, `dumps`, `guts`) finds nothing here.
